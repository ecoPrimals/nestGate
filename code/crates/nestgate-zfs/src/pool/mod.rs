// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! ZFS Pool Management Module
//!
//! This module provides types and functionality for managing ZFS storage pools.
//!
//! # Organization
//!
//! - `types` - Type definitions for pools, states, health, and capacity
//! - `manager` - ZfsPoolManager implementation for pool operations
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

pub mod manager;
pub mod types;

#[cfg(test)]
mod tests;

// Re-export commonly used types
pub use manager::ZfsPoolManager;
pub use types::{PoolCapacity, PoolHealth, PoolInfo, PoolState};
