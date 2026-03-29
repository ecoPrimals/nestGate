// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! ZFS Pool Manager Implementation
//!
//! Handles ZFS pool operations and management

use std::collections::HashMap;
use tokio::process::Command as TokioCommand;
use tracing::{debug, info, warn};

use crate::error::{ZfsOperation, create_zfs_error};
use crate::pool_helpers::parse_size_with_units;
use crate::{config::ZfsConfig, error::Result};
use nestgate_core::error::NestGateError;

use super::types::{PoolCapacity, PoolHealth, PoolInfo, PoolState};

/// ZFS Pool Manager - handles pool operations and management
#[derive(Debug, Clone)]
/// Manager for `ZfsPool` operations
pub struct ZfsPoolManager {
    #[allow(dead_code)]
    config: ZfsConfig,
    /// In-memory cache of discovered pools with automatic persistence
    discovered_pools:
        std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, PoolInfo>>>,
}

impl ZfsPoolManager {
    /// Create a new ZFS pool manager (async)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn new(config: &ZfsConfig) -> Result<Self> {
        info!("Initializing ZFS pool manager");

        let manager = Self {
            config: config.clone(),
            discovered_pools: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        };

        // Test ZFS availability
        if !crate::native::is_zfs_available().await {
            warn!("ZFS not available, running in mock mode");
        }
        Ok(manager)
    }

    /// Create a new ZFS pool manager with owned config (zero-copy)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn with_owned_config(config: ZfsConfig) -> Result<Self> {
        info!("Initializing ZFS pool manager with owned config");

        let manager = Self {
            config,
            discovered_pools: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        };

        // Test ZFS availability
        if !crate::native::is_zfs_available().await {
            warn!("ZFS not available, running in mock mode");
        }
        Ok(manager)
    }

    /// Create instance for real production use
    ///
    /// This is the primary constructor for production code. Use this instead of
    /// `new()` when you have a synchronous context and need immediate initialization.
    #[must_use]
    pub fn new_production(config: ZfsConfig) -> Self {
        Self {
            config,
            discovered_pools: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        }
    }

    /// Discover all available ZFS pools
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn discover_pools(&self) -> Result<()> {
        info!("Discovering ZFS pools");

        let output = TokioCommand::new("zpool")
            .args(["list", "-H", "-o", "name,size,alloc,free,cap,health"])
            .output()
            .await
            .map_err(|_e| {
                NestGateError::internal_error("Failed to execute zpool list command", "zfs_pool")
            })?;

        if !output.status.success() {
            let _error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(NestGateError::internal_error(
                "ZFS pool list command failed",
                "zfs_pool",
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut pools = Vec::new();

        for line in stdout.lines() {
            if let Some(pool_info) = self.parse_pool_line(line)? {
                pools.push(pool_info);
            }
        }

        info!(
            "Discovered {},
    ZFS pools",
            pools.len()
        );
        Ok(())
    }

    /// Discover real ZFS pools on the system
    #[allow(dead_code)]
    async fn discover_real_pools(&self) -> Result<()> {
        let output = TokioCommand::new("zpool")
            .args(["list", "-H", "-p"])
            .output()
            .await
            .map_err(|_e| {
                NestGateError::storage_error(format!(
                    "Failed to execute zpool command: {}",
                    "actual_error_details"
                ))
            })?;

        if !output.status.success() {
            return Err(NestGateError::storage_error(format!(
                "zpool command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Store discovered pools in cache
        let mut pools = self.discovered_pools.write().await;
        for line in stdout.lines() {
            if let Some(pool_info) = self.parse_pool_line(line)? {
                pools.insert(pool_info.name.clone(), pool_info);
            }
        }
        Ok(())
    }

    /// Parse a single line from zpool list output
    pub(crate) fn parse_pool_line(&self, line: &str) -> Result<Option<PoolInfo>> {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 6 {
            return Ok(None);
        }

        let name = parts[0].to_string();
        let size_str = parts[1];
        let alloc_str = parts[2];
        let free_str = parts[3];
        let cap_str = parts[4];
        let health_str = parts[5];

        // Parse sizes (simplified - real implementation would handle units)
        let total_bytes = parse_size_with_units(size_str).unwrap_or(0);
        let used_bytes = parse_size_with_units(alloc_str).unwrap_or(0);
        let available_bytes = parse_size_with_units(free_str).unwrap_or(0);

        let health = match health_str {
            "ONLINE" => PoolHealth::Healthy,
            "DEGRADED" => PoolHealth::Warning,
            "FAULTED" | "UNAVAIL" => PoolHealth::Critical,
            _ => PoolHealth::Unknown,
        };

        let state = match health_str {
            "ONLINE" => PoolState::Online,
            "OFFLINE" => PoolState::Offline,
            "DEGRADED" => PoolState::Degraded,
            "FAULTED" => PoolState::Faulted,
            _ => PoolState::Unknown,
        };

        Ok(Some(PoolInfo {
            name,
            state,
            health,
            capacity: PoolCapacity {
                total: total_bytes,
                total_bytes,
                used: used_bytes,
                used_bytes,
                available: available_bytes,
                available_bytes,
                utilization_percent: cap_str.trim_end_matches('%').parse().unwrap_or(0.0),
                fragmentation_percent: 0.0,
                deduplication_ratio: 1.0,
            },
            devices: Vec::new(), // Would be populated by separate command
            properties: HashMap::new(),
        }))
    }

    /// Get overall pool status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_overall_status(&self) -> Result<crate::manager::PoolOverallStatus> {
        let pools = self.list_pools().await?;

        let pools_online = pools
            .iter()
            .filter(|p| matches!(p.health, PoolHealth::Healthy))
            .count();

        let pools_degraded = pools
            .iter()
            .filter(|p| matches!(p.health, PoolHealth::Warning | PoolHealth::Critical))
            .count();

        let total_capacity = pools.iter().map(|p| p.capacity.total_bytes).sum();

        let available_capacity = pools.iter().map(|p| p.capacity.available_bytes).sum();

        Ok(crate::manager::PoolOverallStatus {
            pools_online,
            pools_degraded,
            total_capacity,
            available_capacity,
        })
    }

    /// Get information about a specific pool
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_pool_info(&self, pool_name: &str) -> Result<PoolInfo> {
        // Check our cache first
        let pools = self.discovered_pools.read().await;
        if let Some(pool_info) = pools.get(pool_name) {
            return Ok(pool_info.clone());
        }

        // If not in cache, try to discover it
        drop(pools);
        self.discover_pools().await?;

        let pools = self.discovered_pools.read().await;
        pools
            .get(pool_name)
            .cloned()
            .ok_or_else(|| NestGateError::storage_error("Pool not found: error details"))
    }

    /// Create a new ZFS pool
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_pool(&self, name: &str, devices: &[String]) -> Result<PoolInfo> {
        info!("Creating ZFS pool: {} with devices: {:?}", name, devices);

        // Build the zpool create command
        let mut args = vec!["create", name];
        for device in devices {
            args.push(device);
        }

        let output = TokioCommand::new("zpool")
            .args(&args)
            .output()
            .await
            .map_err(|_e| {
                create_zfs_error(
                    "Failed to execute zpool create: error details".to_string(),
                    ZfsOperation::Command,
                )
            })?;

        if !output.status.success() {
            return Err(create_zfs_error(
                format!(
                    "zpool create failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                ZfsOperation::Command,
            ));
        }

        // Refresh the pool info and return it
        self.discover_single_pool(name).await?;
        self.get_pool_info(name).await
    }

    /// Destroy a ZFS pool
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn destroy_pool(&self, name: &str) -> Result<()> {
        warn!("Destroying ZFS pool: {}", name);

        let output = TokioCommand::new("zpool")
            .args(["destroy", "-f", name])
            .output()
            .await
            .map_err(|_e| {
                create_zfs_error(
                    format!(
                        "Failed to execute zpool destroy: {}",
                        "actual_error_details"
                    ),
                    ZfsOperation::Command,
                )
            })?;

        if !output.status.success() {
            return Err(create_zfs_error(
                format!(
                    "zpool destroy failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                ZfsOperation::Command,
            ));
        }

        // Remove from discovered pools storage
        {
            let mut pools = self.discovered_pools.write().await;
            pools.remove(name);
            debug!("Removed pool {} from discovered pools cache", name);
        }

        info!("Successfully destroyed pool: {}", name);
        Ok(())
    }

    /// Get detailed pool status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_pool_status(&self, name: &str) -> Result<String> {
        debug!("Getting status for pool: {}", name);

        let output = TokioCommand::new("zpool")
            .args(["status", name])
            .output()
            .await
            .map_err(|_e| {
                create_zfs_error(
                    "Failed to execute zpool status: error details".to_string(),
                    ZfsOperation::Command,
                )
            })?;

        if !output.status.success() {
            return Err(create_zfs_error(
                format!(
                    "zpool status failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                ZfsOperation::Command,
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// Start a scrub operation on a pool
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn scrub_pool(&self, name: &str) -> Result<()> {
        info!("Starting scrub for pool: {}", name);

        let output = TokioCommand::new("zpool")
            .args(["scrub", name])
            .output()
            .await
            .map_err(|_e| {
                create_zfs_error(
                    "Failed to execute zpool scrub: error details".to_string(),
                    ZfsOperation::Command,
                )
            })?;

        if !output.status.success() {
            return Err(create_zfs_error(
                format!(
                    "zpool scrub failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                ZfsOperation::Command,
            ));
        }

        info!("Successfully started scrub for pool: {}", name);
        Ok(())
    }

    /// List all discovered pools
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn list_pools(&self) -> Result<Vec<PoolInfo>> {
        // Return pools from our cache
        let pools = self.discovered_pools.read().await;
        Ok(pools.values().cloned().collect())
    }

    /// Refresh pool information
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn refresh_pool_info(&self, pool_name: &str) -> Result<()> {
        // Re-discover specific pool
        if let Some(pool_info) = self.discover_single_pool(pool_name).await? {
            // Store pool info in discovered pools cache
            let mut pools = self.discovered_pools.write().await;
            pools.insert(pool_name.to_string(), pool_info);
            debug!(
                "Updated pool info for {} in discovered pools cache",
                pool_name
            );
        }
        Ok(())
    }

    /// Discover a single pool by name
    async fn discover_single_pool(&self, pool_name: &str) -> Result<Option<PoolInfo>> {
        let output = TokioCommand::new("zpool")
            .args(["list", "-H", "-p", pool_name])
            .output()
            .await
            .map_err(|_e| {
                create_zfs_error(
                    format!(
                        "Failed to execute zpool command: {}",
                        "actual_error_details"
                    ),
                    ZfsOperation::Command,
                )
            })?;

        if !output.status.success() {
            return Ok(None);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().next() {
            return self.parse_pool_line(line);
        }

        Ok(None)
    }
}

// ========== TEST-ONLY CONSTRUCTORS ==========
// Isolated from production code to maintain clear boundaries

#[cfg(test)]
impl ZfsPoolManager {
    /// Create instance for testing with default configuration
    ///
    /// **TEST-ONLY**: This constructor is only available in test builds.
    /// Production code must use `ZfsPoolManager::new()` or `new_production()`.
    pub fn new_for_testing() -> Self {
        Self {
            config: ZfsConfig::default(),
            discovered_pools: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pool::types::{PoolHealth, PoolState};

    #[test]
    fn parse_pool_line_valid_tab_row() {
        let m = ZfsPoolManager::new_for_testing();
        let line = "tank\t100G\t50G\t50G\t45%\tONLINE";
        let info = m
            .parse_pool_line(line)
            .expect("test: parse_pool_line")
            .expect("test: parsed pool row");
        assert_eq!(info.name, "tank");
        assert_eq!(info.state, PoolState::Online);
        assert_eq!(info.health, PoolHealth::Healthy);
        assert!(info.capacity.total_bytes > 0);
    }

    #[test]
    fn parse_pool_line_too_few_fields_yields_none() {
        let m = ZfsPoolManager::new_for_testing();
        let out = m
            .parse_pool_line("only\tone")
            .expect("test: parse_pool_line short");
        assert!(out.is_none());
    }

    #[test]
    fn parse_pool_line_maps_degraded_and_faulted() {
        let m = ZfsPoolManager::new_for_testing();
        let degraded = m
            .parse_pool_line("p\t1T\t0\t1T\t0%\tDEGRADED")
            .expect("test: parse degraded")
            .expect("test: degraded row");
        assert_eq!(degraded.state, PoolState::Degraded);
        assert_eq!(degraded.health, PoolHealth::Warning);

        let fault = m
            .parse_pool_line("q\t1T\t0\t1T\t0%\tFAULTED")
            .expect("test: parse faulted")
            .expect("test: faulted row");
        assert_eq!(fault.state, PoolState::Faulted);
        assert_eq!(fault.health, PoolHealth::Critical);
    }

    #[tokio::test]
    async fn get_overall_status_empty_cache() {
        let m = ZfsPoolManager::new_for_testing();
        let st = m
            .get_overall_status()
            .await
            .expect("test: get_overall_status");
        assert_eq!(st.pools_online, 0);
        assert_eq!(st.pools_degraded, 0);
        assert_eq!(st.total_capacity, 0);
        assert_eq!(st.available_capacity, 0);
    }

    #[tokio::test]
    async fn list_pools_empty_without_discovery() {
        let m = ZfsPoolManager::new_for_testing();
        let pools = m.list_pools().await.expect("test: list_pools");
        assert!(pools.is_empty());
    }

    #[tokio::test]
    #[ignore = "Requires real ZFS"]
    async fn discover_pools_runs_zpool_list() {
        let m = ZfsPoolManager::new_for_testing();
        let _ = m.discover_pools().await;
    }
}
