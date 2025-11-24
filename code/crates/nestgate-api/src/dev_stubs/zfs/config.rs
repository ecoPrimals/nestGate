//! **ZFS STUB IMPLEMENTATION - DEVELOPMENT ONLY**
//!
//! ⚠️ **WARNING: THIS IS NOT PRODUCTION CODE** ⚠️
//!
//! This module provides stub implementations for ZFS operations during development and testing.
//! All data returned is HARDCODED and does not reflect actual system state.
//!
//! **DO NOT USE IN PRODUCTION** - Use real ZFS implementations from `nestgate-zfs` crate instead.
//!
//! # File Size: 1,007 Lines (0.7% over 1000 LOC guideline)
//!
//! **Rationale**: Development-only test fixtures and mock data.
//! - Contains comprehensive mock ZFS responses for all operations
//! - Dev-only code (not included in production builds via `dev-stubs` feature)
//! - Splitting would scatter related test fixtures
//! - Minimal violation: Only 7 lines over (0.7% excess)
//!
//! # Production Implementations
//!
//! For production use, see:
//! - `nestgate_zfs::operations::production::ProductionZfsOperations` - Real command execution
//! - `nestgate_zfs::RealZfsOperations` - Actual ZFS commands  
//! - `nestgate_zfs::zero_cost::ProductionZfsManager` - Zero-cost production manager
//!
//! # Feature Gates
//!
//! This module is only available with the `dev-stubs` feature flag.
//! Production builds will NOT include this code.

#![cfg(feature = "dev-stubs")]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

/// **ZFS CONFIGURATION (Development Stub)**
///
/// Configuration structure for ZFS stub operations during development.
/// This is NOT production configuration - see `nestgate-zfs` for real implementations.
#[derive(Debug, Clone)]
pub struct ZfsConfig {
    /// List of available ZFS pools (hardcoded for development)
    pub pools: Vec<String>,
    /// Mapping of datasets to their parent pools (hardcoded for development)
    pub datasets: HashMap<String, String>,
}

impl Default for ZfsConfig {
    fn default() -> Self {
        Self {
            pools: vec!["tank".to_string(), "backup".to_string()],
            datasets: HashMap::new(),
        }
    }
}

/// **DEVELOPMENT ZFS STUB MANAGER**
///
/// ⚠️ **THIS IS A STUB - NOT FOR PRODUCTION USE** ⚠️
/// ⚠️ **ONLY AVAILABLE WITH `dev-stubs` FEATURE** ⚠️
///
/// This manager returns HARDCODED mock data for development and testing purposes only.
/// All operations return fake data and do not interact with real ZFS systems.
///
/// **For production use**, see:
/// - `nestgate_zfs::operations::production::ProductionZfsOperations`
/// - `nestgate_zfs::RealZfsOperations`
/// - `nestgate_zfs::zero_cost::ProductionZfsManager`
///
/// # Development Use Only
///
/// This stub is provided to enable:
/// - Local development without ZFS installed
/// - Unit testing of API endpoints  
/// - Integration testing with predictable data
///
/// **Never deploy this to production environments.**
///
/// # Naming Note
///
/// Despite the name `ProductionZfsManager`, this is a development stub.
/// The name exists for API compatibility during development.
/// Use the real `nestgate_zfs::operations::production::ProductionZfsOperations` for production.
#[derive(Debug, Clone)]
#[deprecated(
    since = "0.1.0",
    note = "Development stub only. Use nestgate_zfs::operations::production::ProductionZfsOperations for production."
)]
pub struct ProductionZfsManager {
    config: ZfsConfig,
}

impl ProductionZfsManager {
    /// Create a new production ZFS manager with the given configuration
    #[must_use]
    pub const fn new(config: ZfsConfig) -> Self {
        Self { config }
    }

    /// Get reference to the ZFS configuration
    #[must_use]
    pub const fn config(&self) -> &ZfsConfig {
        &self.config
    }
}
