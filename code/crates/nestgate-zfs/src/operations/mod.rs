// ZFS Operations Module
//
// This module provides comprehensive ZFS operations functionality including
// real command execution, caching, metrics, and health monitoring.

// Production ZFS operations (PHASE 2 MOCK REPLACEMENT)
//! Operations module

pub mod production;

// Re-export the main types for convenience
pub use production::{
    CommandResult, ProductionZfsOperations, SystemCapabilities, ZfsMetrics, ZfsOperationsConfig};
