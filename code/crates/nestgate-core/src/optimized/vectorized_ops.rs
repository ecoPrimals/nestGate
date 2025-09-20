//! **VECTORIZED OPERATIONS**
//!
//! High-performance vectorized operations using compiler auto-vectorization.
//! 
//! This module has been refactored into smaller, focused sub-modules for better
//! maintainability while preserving all public APIs.

pub use self::vectorized::*;

pub mod vectorized;
