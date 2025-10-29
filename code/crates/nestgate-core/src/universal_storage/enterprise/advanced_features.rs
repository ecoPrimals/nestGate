// **ADVANCED STORAGE FEATURES - ENTRY POINT**
//! Advanced Features functionality and utilities.
// This module provides the main entry point for advanced storage features.
// The actual implementation has been split into focused modules for maintainability.

// Re-export all advanced features from the modular structure
pub use super::features::*;

// Convenience re-exports for backward compatibility
pub use super::features::optimization::*;
pub use super::features::forecasting::*;
pub use super::features::policies::*;
pub use super::features::anomaly_detection::*;
pub use super::features::disaster_recovery::*; 