//! Strongly-typed Identifiers
//!
//! Prevents mixing up Actor IDs and Cell IDs at compile time (Zero-cost abstraction).

use core::fmt;
use serde::{Deserialize, Serialize};

/// Unique identifier for an Actor in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActorId(pub u64);

/// Unique identifier for a Grid Cell in the simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellId(pub u64);

impl fmt::Display for ActorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Actor({})", self.0)
    }
}
