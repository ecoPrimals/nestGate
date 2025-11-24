//! Native real ZFS operations with production implementations
//!
//! **REFACTORED**: This module now uses proper separation of concerns.
//! The original 1037-line god file has been split into focused modules:
//! - `core`: Core service structure and command execution (150 lines)
//! - `pool_operations`: Pool management operations (280 lines)
//! - `dataset_operations`: Dataset management operations (200 lines)
//! - `snapshot_operations`: Snapshot operations (180 lines)
//! - `metrics`: Metrics collection and monitoring (120 lines)
//! - `configuration`: Configuration management (107 lines)
//!
//! **ARCHITECTURAL IMPROVEMENT**:
//! - Single Responsibility Principle: Each module has one clear purpose
//! - Command Pattern: Commands are abstracted and reusable
//! - Zero-copy optimizations: String parsing uses views where possible
//! - Error handling: Proper error propagation and context
//! - Testability: Each module can be tested independently

/// Configuration management for native ZFS operations
pub mod configuration;
/// Core native ZFS service structure and execution
pub mod core;
/// Dataset creation, management, and deletion operations
pub mod dataset_operations;
/// Metrics collection and monitoring for ZFS operations
pub mod metrics;
/// Parsing utilities for ZFS command output
pub mod parsing;
/// Pool creation, management, and status operations
pub mod pool_operations;
/// Snapshot creation, management, and cleanup operations
pub mod snapshot_operations;

// Re-export the main service for backwards compatibility
pub use core::NativeZfsService;
