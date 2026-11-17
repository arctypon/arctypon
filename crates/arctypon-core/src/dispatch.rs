// SPDX-License-Identifier: MIT OR Apache-2.0
//! Transport-agnostic message dispatch (atomr pattern, see HANDOFF appendix E).
//!
//! `ActorRef::tell_bytes` hands an already-encoded payload to a [`Dispatcher`].
//! The same ref can target an in-kernel mailbox today and a host shared-memory
//! ring or an accelerator queue tomorrow, with identical backpressure and
//! placement semantics. Typed `tell()` (postcard into a stack buffer) lands in
//! Phase 2; this module stays serialization-agnostic and dependency-free.

use arctypon_types::{ActorId, ArctyponError};
use core::marker::PhantomData;

/// Where the runtime placed an actor's execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    /// Pinned to a specific CPU core.
    Cpu(u32),
    /// Executed by the host layer (Phase 3, via `arctypon-abi`).
    Host,
    /// Offloaded to accelerator device `u8` (future, OSTD DMA).
    Accel(u8),
}

/// How a full mailbox reacts to a new message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackpressurePolicy {
    /// Reject the message with [`ArctyponError::MailboxFull`] (default).
    Reject,
    /// Drop the oldest queued message to make room.
    DropOldest,
    /// Park the sender until space frees (kernel-only, OSTD waitqueues).
    Park,
}

/// A transport backend for actor messages.
///
/// Implementations must be cheap to call from interrupt-free kernel context
/// and must never allocate.
pub trait Dispatcher: Send + Sync {
    /// Enqueue an encoded payload; applies the backpressure policy on fullness.
    fn enqueue(&self, payload: &[u8]) -> Result<(), ArctyponError>;

    /// Execution placement this dispatcher serves.
    fn placement(&self) -> Placement;

    /// Backpressure policy applied when the target queue is full.
    fn backpressure(&self) -> BackpressurePolicy {
        BackpressurePolicy::Reject
    }
}

/// A handle for sending messages to one actor, independent of transport.
pub struct ActorRef<'a, M> {
    id: ActorId,
    dispatcher: &'a dyn Dispatcher,
    _msg: PhantomData<fn(M)>,
}

impl<'a, M> ActorRef<'a, M> {
    /// Create a ref bound to `dispatcher` for actor `id`.
    pub fn new(id: ActorId, dispatcher: &'a dyn Dispatcher) -> Self {
        Self {
            id,
            dispatcher,
            _msg: PhantomData,
        }
    }

    /// Target actor's identifier.
    pub fn id(&self) -> ActorId {
        self.id
    }

    /// Placement of the target actor (runtime-explicit, atomr principle).
    pub fn placement(&self) -> Placement {
        self.dispatcher.placement()
    }

    /// Send an encoded payload through the underlying transport.
    pub fn tell_bytes(&self, payload: &[u8]) -> Result<(), ArctyponError> {
        self.dispatcher.enqueue(payload)
    }
}

impl<M> Clone for ActorRef<'_, M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for ActorRef<'_, M> {}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use std::sync::Mutex;

    struct MockDispatcher {
        placement: Placement,
        fail: bool,
        last_len: Mutex<Option<usize>>,
    }

    impl Dispatcher for MockDispatcher {
        fn enqueue(&self, payload: &[u8]) -> Result<(), ArctyponError> {
            if self.fail {
                return Err(ArctyponError::MailboxFull);
            }
            *self.last_len.lock().unwrap() = Some(payload.len());
            Ok(())
        }

        fn placement(&self) -> Placement {
            self.placement
        }
    }

    #[test]
    fn tell_routes_payload_and_placement() {
        let d = MockDispatcher {
            placement: Placement::Cpu(2),
            fail: false,
            last_len: Mutex::new(None),
        };
        let r: ActorRef<'_, u32> = ActorRef::new(ActorId(7), &d);
        assert_eq!(r.placement(), Placement::Cpu(2));
        assert_eq!(r.id(), ActorId(7));
        assert!(r.tell_bytes(&[1, 2, 3]).is_ok());
        assert_eq!(*d.last_len.lock().unwrap(), Some(3));
    }

    #[test]
    fn backpressure_error_propagates() {
        let d = MockDispatcher {
            placement: Placement::Host,
            fail: true,
            last_len: Mutex::new(None),
        };
        let r: ActorRef<'_, u32> = ActorRef::new(ActorId(1), &d);
        assert_eq!(r.tell_bytes(&[0]), Err(ArctyponError::MailboxFull));
        assert_eq!(d.backpressure(), BackpressurePolicy::Reject);
    }
}
