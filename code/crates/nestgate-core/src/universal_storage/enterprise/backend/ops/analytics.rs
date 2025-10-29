//! Enterprise Storage Analytics Operations
//!
//! This module provides comprehensive analytics functionality for enterprise storage,
//! including performance monitoring, usage analysis, trend prediction, and optimization insights.

// Re-export everything from the analytics module for backward compatibility
pub use self::analytics::*;

/// Analytics submodule containing the refactored implementation
pub mod analytics; 