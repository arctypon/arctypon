// SPDX-License-Identifier: MIT OR Apache-2.0
//! Deterministic fixed-point math (replaces f32/f64 per contract §2.3).

use fixed::types::extra::U16;

/// 32-bit signed fixed-point with 16 fractional bits (Fixed<16,16>).
pub type Fixed = fixed::FixedI32<U16>;

/// π in Fixed<16,16> (≈ 3.14158).
pub const PI: Fixed = Fixed::from_bits(205_887);

/// e in Fixed<16,16> (≈ 2.71827).
pub const E: Fixed = Fixed::from_bits(178_145);
