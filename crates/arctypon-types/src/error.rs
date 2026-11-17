// SPDX-License-Identifier: MIT OR Apache-2.0
//! Unified error codes for the arctypon series.

use core::fmt;

/// Core error codes; `repr(u32)` keeps the layout stable for the future ABI layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ArctyponError {
    /// Mailbox is full; the message was rejected (backpressure).
    MailboxFull = 1,
    /// Target actor does not exist.
    InvalidActorId = 2,
    /// Operation timed out.
    Timeout = 3,
    /// Payload could not be serialized or deserialized.
    InvalidPayload = 4,
    /// Fixed-point arithmetic overflow.
    MathOverflow = 5,
}

impl ArctyponError {
    /// Raw discriminant, reserved for the future `arctypon-abi` wire format.
    pub const fn as_raw(self) -> u32 {
        self as u32
    }

    /// Parse a raw code; unknown codes yield `None` (open-enum semantics,
    /// to be layered in `arctypon-abi`).
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            1 => Some(Self::MailboxFull),
            2 => Some(Self::InvalidActorId),
            3 => Some(Self::Timeout),
            4 => Some(Self::InvalidPayload),
            5 => Some(Self::MathOverflow),
            _ => None,
        }
    }
}

impl fmt::Display for ArctyponError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::MailboxFull => "MailboxFull",
            Self::InvalidActorId => "InvalidActorId",
            Self::Timeout => "Timeout",
            Self::InvalidPayload => "InvalidPayload",
            Self::MathOverflow => "MathOverflow",
        };
        write!(f, "ArctyponError::{}({})", name, self.as_raw())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::ArctyponError;

    #[test]
    fn raw_round_trip() {
        let all = [
            ArctyponError::MailboxFull,
            ArctyponError::InvalidActorId,
            ArctyponError::Timeout,
            ArctyponError::InvalidPayload,
            ArctyponError::MathOverflow,
        ];
        for e in all {
            assert_eq!(ArctyponError::from_raw(e.as_raw()), Some(e));
        }
        assert_eq!(ArctyponError::from_raw(999), None);
    }
}
