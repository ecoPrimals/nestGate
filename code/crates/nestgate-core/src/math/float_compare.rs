// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Float Comparison Utilities
//!
//! Provides safe floating-point comparison functions that account for
//! floating-point precision limitations.
//!
//! # Why This Module Exists
//!
//! Direct equality comparisons on floating-point numbers are unreliable due to:
//! - Rounding errors in arithmetic operations
//! - Representation limitations (not all decimals can be exactly represented)
//! - Accumulation of small errors in calculations
//!
//! # Usage
//!
//! ```rust
//! use nestgate_core::math::float_compare::{approx_eq_f64, EPSILON_F64};
//!
//! let a = 0.1 + 0.2;
//! let b = 0.3;
//!
//! // ❌ WRONG: May fail due to precision
//! // assert_eq!(a, b);
//!
//! // ✅ CORRECT: Uses epsilon comparison
//! assert!(approx_eq_f64(a, b));
//! ```
//!
//! # Epsilon Values
//!
//! - `EPSILON_F32`: 1e-6 (suitable for most f32 comparisons)
//! - `EPSILON_F64`: 1e-10 (suitable for most f64 comparisons)
//!
//! For specialized cases, use `approx_eq_f64_epsilon` with a custom epsilon.

/// Default epsilon for f32 comparisons
///
/// This value (0.000001) is suitable for most f32 comparisons where
/// you expect values to be "close enough" considering f32 precision.
pub const EPSILON_F32: f32 = 1e-6;

/// Default epsilon for f64 comparisons
///
/// This value (0.0000000001) is suitable for most f64 comparisons where
/// you expect values to be "close enough" considering f64 precision.
pub const EPSILON_F64: f64 = 1e-10;

/// Compare two f32 values for approximate equality
///
/// Returns `true` if the absolute difference between `a` and `b` is less than
/// [`EPSILON_F32`].
///
/// # Examples
///
/// ```
/// use nestgate_core::math::float_compare::approx_eq_f32;
///
/// assert!(approx_eq_f32(0.1 + 0.2, 0.3));
/// assert!(approx_eq_f32(1.0, 1.0000001));
/// assert!(!approx_eq_f32(1.0, 2.0));
/// ```
///
/// # Special Cases
///
/// - Returns `false` if either value is NaN
/// - Returns `true` if both values are the same infinity
/// - Returns `false` if values are different infinities
#[must_use]
#[inline]
pub fn approx_eq_f32(a: f32, b: f32) -> bool {
    // Handle NaN cases
    if a.is_nan() || b.is_nan() {
        return false;
    }

    // Handle infinity cases
    if a.is_infinite() || b.is_infinite() {
        return a == b;
    }

    (a - b).abs() < EPSILON_F32
}

/// Compare two f64 values for approximate equality
///
/// Returns `true` if the absolute difference between `a` and `b` is less than
/// [`EPSILON_F64`].
///
/// # Examples
///
/// ```
/// use nestgate_core::math::float_compare::approx_eq_f64;
///
/// assert!(approx_eq_f64(0.1 + 0.2, 0.3));
/// assert!(approx_eq_f64(1.0, 1.0 + 1e-11));  // within epsilon
/// assert!(!approx_eq_f64(1.0, 2.0));
/// ```
///
/// # Special Cases
///
/// - Returns `false` if either value is NaN
/// - Returns `true` if both values are the same infinity
/// - Returns `false` if values are different infinities
#[must_use]
#[inline]
pub fn approx_eq_f64(a: f64, b: f64) -> bool {
    // Handle NaN cases
    if a.is_nan() || b.is_nan() {
        return false;
    }

    // Handle infinity cases
    if a.is_infinite() || b.is_infinite() {
        return a == b;
    }

    (a - b).abs() < EPSILON_F64
}

/// Compare two f64 values for approximate equality with custom epsilon
///
/// Returns `true` if the absolute difference between `a` and `b` is less than
/// the specified `epsilon`.
///
/// Use this when you need more control over the comparison threshold.
///
/// # Examples
///
/// ```
/// use nestgate_core::math::float_compare::approx_eq_f64_epsilon;
///
/// // Very strict comparison
/// assert!(approx_eq_f64_epsilon(1.0, 1.0 + 1e-8, 1e-7));
///
/// // More lenient comparison
/// assert!(approx_eq_f64_epsilon(1.0, 1.01, 0.02));
///
/// // Custom epsilon for specific domain (0.1% tolerance)
/// let tolerance = 0.1;
/// assert!(approx_eq_f64_epsilon(100.0, 100.05, tolerance));
/// ```
///
/// # Special Cases
///
/// - Returns `false` if either value is NaN
/// - Returns `true` if both values are the same infinity
/// - Returns `false` if values are different infinities
/// - `epsilon` should be positive; negative epsilon will never match
#[must_use]
#[inline]
pub fn approx_eq_f64_epsilon(a: f64, b: f64, epsilon: f64) -> bool {
    // Handle NaN cases
    if a.is_nan() || b.is_nan() {
        return false;
    }

    // Handle infinity cases
    if a.is_infinite() || b.is_infinite() {
        return a == b;
    }

    (a - b).abs() < epsilon
}

/// Compare two f32 values for approximate equality with custom epsilon
///
/// Returns `true` if the absolute difference between `a` and `b` is less than
/// the specified `epsilon`.
///
/// # Examples
///
/// ```
/// use nestgate_core::math::float_compare::approx_eq_f32_epsilon;
///
/// assert!(approx_eq_f32_epsilon(1.0, 1.001, 0.01));
/// assert!(!approx_eq_f32_epsilon(1.0, 1.1, 0.01));
/// ```
#[must_use]
#[inline]
pub fn approx_eq_f32_epsilon(a: f32, b: f32, epsilon: f32) -> bool {
    // Handle NaN cases
    if a.is_nan() || b.is_nan() {
        return false;
    }

    // Handle infinity cases
    if a.is_infinite() || b.is_infinite() {
        return a == b;
    }

    (a - b).abs() < epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== F64 TESTS ====================

    #[test]
    fn test_approx_eq_f64_basic() {
        assert!(approx_eq_f64(1.0, 1.0));
        assert!(approx_eq_f64(0.0, 0.0));
        assert!(approx_eq_f64(-1.0, -1.0));
    }

    #[test]
    fn test_approx_eq_f64_floating_point_arithmetic() {
        // Classic floating point precision issue
        let a = 0.1 + 0.2;
        let b = 0.3;
        assert!(approx_eq_f64(a, b));
    }

    #[test]
    fn test_approx_eq_f64_small_differences() {
        assert!(approx_eq_f64(1.0, 1.0 + EPSILON_F64 / 2.0));
        assert!(!approx_eq_f64(1.0, 1.0 + EPSILON_F64 * 2.0));
    }

    #[test]
    fn test_approx_eq_f64_large_numbers() {
        assert!(approx_eq_f64(1_000_000.0, 1_000_000.0));
        assert!(!approx_eq_f64(1_000_000.0, 1_000_001.0));
    }

    #[test]
    fn test_approx_eq_f64_negative_numbers() {
        assert!(approx_eq_f64(-1.0, -1.0));
        assert!(approx_eq_f64(-0.1 - 0.2, -0.3));
    }

    #[test]
    fn test_approx_eq_f64_nan() {
        assert!(!approx_eq_f64(f64::NAN, f64::NAN));
        assert!(!approx_eq_f64(1.0, f64::NAN));
        assert!(!approx_eq_f64(f64::NAN, 1.0));
    }

    #[test]
    fn test_approx_eq_f64_infinity() {
        assert!(approx_eq_f64(f64::INFINITY, f64::INFINITY));
        assert!(approx_eq_f64(f64::NEG_INFINITY, f64::NEG_INFINITY));
        assert!(!approx_eq_f64(f64::INFINITY, f64::NEG_INFINITY));
        assert!(!approx_eq_f64(f64::INFINITY, 1.0));
    }

    // ==================== F32 TESTS ====================

    #[test]
    fn test_approx_eq_f32_basic() {
        assert!(approx_eq_f32(1.0, 1.0));
        assert!(approx_eq_f32(0.0, 0.0));
        assert!(approx_eq_f32(-1.0, -1.0));
    }

    #[test]
    fn test_approx_eq_f32_floating_point_arithmetic() {
        let a = 0.1_f32 + 0.2_f32;
        let b = 0.3_f32;
        assert!(approx_eq_f32(a, b));
    }

    #[test]
    fn test_approx_eq_f32_nan() {
        assert!(!approx_eq_f32(f32::NAN, f32::NAN));
        assert!(!approx_eq_f32(1.0, f32::NAN));
    }

    #[test]
    fn test_approx_eq_f32_infinity() {
        assert!(approx_eq_f32(f32::INFINITY, f32::INFINITY));
        assert!(approx_eq_f32(f32::NEG_INFINITY, f32::NEG_INFINITY));
        assert!(!approx_eq_f32(f32::INFINITY, f32::NEG_INFINITY));
    }

    // ==================== EPSILON TESTS ====================

    #[test]
    fn test_approx_eq_f64_epsilon_custom() {
        assert!(approx_eq_f64_epsilon(1.0, 1.01, 0.02));
        assert!(!approx_eq_f64_epsilon(1.0, 1.01, 0.005));
    }

    #[test]
    fn test_approx_eq_f64_epsilon_percentage() {
        // 1% tolerance
        let value = 100.0;
        let tolerance = 1.0;
        assert!(approx_eq_f64_epsilon(value, 100.5, tolerance));
        assert!(!approx_eq_f64_epsilon(value, 102.0, tolerance));
    }

    #[test]
    fn test_approx_eq_f32_epsilon_custom() {
        assert!(approx_eq_f32_epsilon(1.0, 1.01, 0.02));
        assert!(!approx_eq_f32_epsilon(1.0, 1.01, 0.005));
    }

    // ==================== EDGE CASES ====================

    #[test]
    fn test_zero_comparisons() {
        assert!(approx_eq_f64(0.0, 0.0));
        assert!(approx_eq_f64(-0.0, 0.0));
        assert!(approx_eq_f32(0.0, 0.0));
        assert!(approx_eq_f32(-0.0, 0.0));
    }

    #[test]
    fn test_very_small_numbers() {
        assert!(approx_eq_f64(1e-15, 1e-15));
        assert!(approx_eq_f32(1e-10, 1e-10));
    }

    #[test]
    fn test_very_large_numbers() {
        assert!(approx_eq_f64(1e15, 1e15));
        assert!(approx_eq_f32(1e10, 1e10));
    }
}
