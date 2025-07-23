//! Penetration Testing Module
//!
//! Split from sovereign_science_penetration_suite.rs for file size compliance

pub mod analysis;
pub mod attacks;
pub mod config;
pub mod scanner;
pub mod tests;

// Re-export main types
pub use analysis::*;
pub use attacks::*;
pub use config::*;
pub use scanner::*;
