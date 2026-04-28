// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **ZFS STUB IMPLEMENTATION - DEVELOPMENT ONLY**
//!
//! **WARNING: THIS IS NOT PRODUCTION CODE**
//!
//! When the `zfs` / `zpool` CLI is available, [`ZfsConfig::try_detect_system`] reads pool names
//! from `zpool list`. If ZFS is not installed, pools are empty (explicit fallback, not fake names).
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

#![cfg(any(test, feature = "dev-stubs"))]

use std::collections::HashMap;
use std::process::Command;

/// **ZFS CONFIGURATION (Development Stub)**
///
/// Configuration structure for ZFS stub operations during development.
/// This is NOT production configuration - see `nestgate-zfs` for real implementations.
#[derive(Debug, Clone)]
/// Configuration for Zfs
pub struct ZfsConfig {
    /// List of available ZFS pools (from `zpool list` when ZFS is available)
    pub pools: Vec<String>,
    /// Mapping of datasets to their parent pools (populated when detection succeeds)
    pub datasets: HashMap<String, String>,
}

impl ZfsConfig {
    /// Detect whether ZFS kernel support appears present (`/proc/filesystems` lists `zfs`).
    #[must_use]
    pub fn zfs_module_visible_in_proc() -> bool {
        std::fs::read_to_string("/proc/filesystems")
            .map(|s| s.lines().any(|l| l.contains("zfs")))
            .unwrap_or(false)
    }

    /// `true` if `zfs` CLI runs (best-effort).
    #[must_use]
    pub fn zfs_cli_available() -> bool {
        Command::new("zfs")
            .arg("version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Empty configuration used when ZFS cannot be inspected.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            pools: Vec::new(),
            datasets: HashMap::new(),
        }
    }

    /// Populate pool names from `zpool list -H -o name` when possible.
    ///
    /// # Errors
    ///
    /// Returns I/O errors from running `zpool` (missing binary is treated as empty pools, not `Err`).
    pub fn try_detect_system() -> std::io::Result<Self> {
        if !Self::zfs_module_visible_in_proc() && !Self::zfs_cli_available() {
            return Ok(Self::empty());
        }

        let output = Command::new("zpool")
            .args(["list", "-H", "-o", "name"])
            .output();

        let Ok(output) = output else {
            return Ok(Self::empty());
        };

        if !output.status.success() {
            return Ok(Self::empty());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let pools: Vec<String> = stdout
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(std::string::ToString::to_string)
            .collect();

        let mut datasets = HashMap::new();
        if Self::zfs_cli_available()
            && let Ok(ds_out) = Command::new("zfs")
                .args(["list", "-H", "-o", "name"])
                .output()
            && ds_out.status.success()
        {
            let ds_stdout = String::from_utf8_lossy(&ds_out.stdout);
            for line in ds_stdout.lines() {
                let name = line.trim();
                if name.is_empty() {
                    continue;
                }
                if let Some((pool, _rest)) = name.split_once('/') {
                    datasets.insert(name.to_string(), pool.to_string());
                }
            }
        }

        Ok(Self { pools, datasets })
    }
}

impl Default for ZfsConfig {
    /// Returns detected pools when ZFS is available, otherwise an empty config (no hardcoded pools).
    fn default() -> Self {
        Self::try_detect_system().unwrap_or_else(|_| Self::empty())
    }
}

/// **DEVELOPMENT ZFS STUB MANAGER**
///
/// **THIS IS A STUB - NOT FOR PRODUCTION USE**
/// **ONLY AVAILABLE WITH `dev-stubs` FEATURE**
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
/// Manager for `ProductionZfs` operations
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
