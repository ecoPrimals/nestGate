//! Native ZFS Backend - Modular Implementation
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

pub mod configuration;
pub mod core;
pub mod dataset_operations;
pub mod metrics;
pub mod parsing;
pub mod pool_operations;
pub mod snapshot_operations;

// Re-export the main service for backwards compatibility
pub use core::NativeZfsService;
