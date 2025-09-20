//! Adaptive Performance Optimization Engine
//!
//! Runtime performance monitoring and adaptive optimization engine
//! that automatically tunes system parameters for optimal performance.

// Re-export everything from the adaptive_optimization module for backward compatibility
pub use self::adaptive_optimization::*;

/// Adaptive optimization submodule containing the refactored implementation
pub mod adaptive_optimization; 