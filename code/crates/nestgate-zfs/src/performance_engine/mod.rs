//! Performance Engine Module
//!
//! This module provides comprehensive ZFS performance monitoring and optimization
//! capabilities, organized into logical sub-modules for maintainability.

pub mod engine;
pub mod monitoring;
pub mod types;

// Re-export commonly used types
pub use engine::PerformanceOptimizationEngine;
pub use monitoring::RealTimePerformanceMonitor;
pub use types::*;
