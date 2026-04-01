// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Mathematical Utilities
//!
//! Provides mathematical utilities for the NestGate system, including
//! safe float comparisons and numerical operations.

pub mod float_compare;

// Re-export commonly used items
pub use float_compare::{
    EPSILON_F32, EPSILON_F64, approx_eq_f32, approx_eq_f64, approx_eq_f64_epsilon,
};
