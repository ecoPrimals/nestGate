// Unified final configuration module
//! Module definitions and exports.
// This module provides the final unified configuration system.

pub mod domain_configs;
pub mod extensions;
pub mod implementation;
pub mod supporting_types;

// Re-export commonly used types
pub use domain_configs::*;
pub use extensions::*;
pub use supporting_types::*;
