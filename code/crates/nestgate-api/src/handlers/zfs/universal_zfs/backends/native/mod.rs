// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Native ZFS backend implementation
//!
//! This module contains the native ZFS backend implementation split into logical submodules:
//! - core: Core service structure and utilities
//! - pool_operations: Pool management operations
//! - dataset_operations: Dataset management operations
//! - snapshot_operations: Snapshot operations
//! - optimization: Optimization and analytics
//! - configuration: Configuration and utility methods

mod configuration;
#[cfg(test)]
mod configuration_tests;
/// Core native ZFS service implementation
pub mod core;
#[cfg(test)]
mod core_tests;
mod dataset_operations;
mod optimization;
mod pool_operations;
mod snapshot_operations;

// Re-export the main service
pub use core::NativeZfsService;
