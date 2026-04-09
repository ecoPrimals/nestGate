// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS Pool Management Module
//!
//! This module provides types and functionality for managing ZFS storage pools.
//!
//! # Organization
//!
//! - `types` - Type definitions for pools, states, health, and capacity
//! - `manager` - `ZfsPoolManager` type and constructors
//! - `discovery` - Pool discovery, parsing, cache-backed listing and info
//! - `status` - Aggregate status and `zpool status` output
//! - `operations` - Create, destroy, scrub
//! - `tests` - Comprehensive test suite
//!
//! # Example
//!
//! ```no_run
//! use nestgate_zfs::pool::{ZfsPoolManager, PoolInfo};
//! use nestgate_zfs::config::ZfsConfig;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = ZfsConfig::default();
//! let manager = ZfsPoolManager::new(&config).await?;
//! manager.discover_pools().await?;
//! # Ok(())
//! # }
//! ```

mod discovery;
pub mod manager;
mod operations;
mod status;
pub mod types;

#[cfg(test)]
mod operations_tests;
#[cfg(test)]
mod tests;

// Re-export commonly used types
pub use manager::ZfsPoolManager;
pub use types::{PoolCapacity, PoolHealth, PoolInfo, PoolState};
