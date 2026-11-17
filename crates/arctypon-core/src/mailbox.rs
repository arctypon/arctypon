// SPDX-License-Identifier: MIT OR Apache-2.0
//! Allocator-generic mailbox: each actor can own an isolated heap/arena.
use alloc::vec::Vec;
use core::alloc::Allocator;

/// Bounded mailbox backed by a caller-supplied allocator.
///
/// Sketch note: LIFO for now; the production design is a lock-free FIFO ring.
pub struct Mailbox<M, A: Allocator> {
    queue: Vec<M, A>,
    capacity: usize,
}

impl<M, A: Allocator> Mailbox<M, A> {
    /// Create an empty mailbox with the given capacity and allocator.
    pub fn new_in(capacity: usize, alloc: A) -> Self {
        Self {
            queue: Vec::new_in(alloc),
            capacity,
        }
    }
    /// Enqueue a message; returns it back if the mailbox is full.
    pub fn try_send(&mut self, msg: M) -> Result<(), M> {
        if self.queue.len() >= self.capacity {
            return Err(msg);
        }
        self.queue.push(msg);
        Ok(())
    }
    /// Dequeue a message, if any.
    pub fn try_recv(&mut self) -> Option<M> {
        self.queue.pop()
    }
    /// Number of queued messages.
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    /// Whether the mailbox is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Mailbox;
    use alloc::alloc::Global;

    #[test]
    fn send_recv_and_backpressure() {
        let mut mb = Mailbox::new_in(2, Global);
        assert!(mb.try_send(1u32).is_ok());
        assert!(mb.try_send(2).is_ok());
        assert_eq!(mb.try_send(3), Err(3)); // 背壓：滿了退回
        assert_eq!(mb.try_recv(), Some(2)); // sketch 為 LIFO
        assert_eq!(mb.try_recv(), Some(1));
        assert_eq!(mb.try_recv(), None);
    }
}
