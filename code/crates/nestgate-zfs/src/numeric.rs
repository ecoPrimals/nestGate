// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Saturating float/integer conversions for ZFS size parsing and metrics.

/// Converts a finite non-negative `f64` to `u64`, saturating at `u64::MAX`.
/// NaN, infinity, and negative values yield `0`.
#[must_use]
pub fn f64_to_u64_saturating(x: f64) -> u64 {
    if !x.is_finite() || x <= 0.0 {
        return 0;
    }
    if x >= u64::MAX as f64 {
        return u64::MAX;
    }
    #[expect(
        clippy::cast_possible_truncation,
        reason = "value clamped to 0..=u64::MAX; truncates fractional sub-byte product"
    )]
    #[expect(
        clippy::cast_sign_loss,
        reason = "non-finite and non-positive values handled above"
    )]
    let v: u64 = x as u64;
    v
}

/// `usize` to `f64` for counts and indices in averages (not byte sizes).
#[must_use]
pub const fn usize_to_f64_lossy(n: usize) -> f64 {
    #[expect(
        clippy::cast_precision_loss,
        reason = "pool/dataset counts and indices; always << 2^53 in practice"
    )]
    let v: f64 = n as f64;
    v
}

/// `u64` to `f64` for approximate metrics (very large byte totals may exceed IEEE integer precision).
#[must_use]
pub const fn u64_to_f64_approximate(x: u64) -> f64 {
    #[expect(
        clippy::cast_precision_loss,
        reason = "gauge-style metrics; approximate for multi-PB aggregates"
    )]
    let v: f64 = x as f64;
    v
}
