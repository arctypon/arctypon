// SPDX-License-Identifier: MIT OR Apache-2.0
//! Core actor abstractions for the arctypon series (`no_std`, host-testable).
#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]

extern crate alloc;

mod mailbox;
pub use mailbox::Mailbox;

/// An actor processes one message at a time from its mailbox.
pub trait Actor {
    /// The message type this actor accepts.
    type Message;
    /// Handle a single message. Must not block.
    fn handle(&mut self, msg: Self::Message);
}