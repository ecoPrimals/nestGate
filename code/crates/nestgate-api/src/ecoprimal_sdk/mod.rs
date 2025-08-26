//
// This module contains the modularized EcoPrimal SDK implementation,
// broken down into focused, maintainable components.

// Core type definitions and structures
pub mod config;
pub mod errors;
pub mod types;

// Core trait and implementation
pub mod advanced;
pub mod implementation;
pub mod traits;

// Specialized functionality
pub mod community;
pub mod lifecycle;
pub mod metrics;

// Re-export commonly used items for backward compatibility
pub use config::*;
pub use errors::*;
pub use implementation::*;
pub use traits::*;
pub use types::*;
// pub use advanced::*; // Currently minimal implementation
pub use community::*;
pub use lifecycle::*;
pub use metrics::*;
