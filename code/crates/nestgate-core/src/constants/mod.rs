//! **NESTGATE CONSTANTS**
//!
//! Centralized constants to reduce hardcoding throughout the codebase.

pub mod canonical_defaults;
pub mod magic_numbers_replacement;
pub mod network;
pub mod system;
pub mod testing;

pub use network::*;
pub use system::*;
pub use testing::*;
