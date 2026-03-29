// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! ZFS Types Module - Domain-Driven Organization
//!
//! This module provides a clean, domain-driven organization of ZFS-related types.
//! Types are organized by their domain purpose rather than arbitrary grouping.
//!
//! ## Architecture
//!
//! The types are organized into cohesive domain modules:
//! - `capacity`: Monitoring, bottlenecks, retention policies
//! - `errors`: Error types and result aliases
//! - `pool`: Pool information, health, capacity
//! - `dataset`: Dataset information, properties, quotas
//! - `snapshot`: Snapshot information and options
//! - `performance`: Zero-cost optimized types
//! - `command`: Command execution and results
//! - `config`: Configuration and settings
//!
//! ## Usage
//!
//! All types are re-exported from this module for backward compatibility:
//!
//! ```rust
//! use nestgate_zfs::types::{PoolInfo, DatasetInfo, SnapshotInfo};
//! ```
//!
//! ## Migration
//!
//! This structure replaces the previous monolithic `types.rs` (1,104 lines)
//! with a clean domain-driven architecture. All public APIs remain the same.

// Domain modules
pub mod capacity;
pub mod command;
pub mod config;
pub mod dataset;
pub mod errors;
pub mod parsers; // ZFS output parsers
pub mod performance;
pub mod pool;
pub mod snapshot;

// Re-export all public types for backward compatibility
pub use capacity::*;
pub use command::*;
pub use config::*;
pub use dataset::*;
pub use errors::*;
pub use parsers::*; // Export parser functions
pub use performance::*;
pub use pool::*;
pub use snapshot::*;

// Re-export canonical error handling from nestgate-core
pub use crate::error::{ZfsOperation, create_zfs_error};
