//! Arctypon OS Core Types
//!
//! This crate is 100% `no_std` and `alloc-free` by default.
//! It defines the fundamental building blocks for the Actor model and deterministic simulation.

#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::panic)]

pub mod error;
pub mod id;
pub mod math;

// Re-export core traits for ergonomic use
pub use error::ArctyponError;
pub use id::{ActorId, CellId};
pub use math::Fixed;
