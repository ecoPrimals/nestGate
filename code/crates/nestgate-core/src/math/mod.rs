//! Mathematical Utilities
//!
//! Provides mathematical utilities for the NestGate system, including
//! safe float comparisons and numerical operations.

pub mod float_compare;

// Re-export commonly used items
pub use float_compare::{
    approx_eq_f32, approx_eq_f64, approx_eq_f64_epsilon, EPSILON_F32, EPSILON_F64,
};
