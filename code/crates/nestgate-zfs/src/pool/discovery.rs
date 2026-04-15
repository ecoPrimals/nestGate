// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Pool discovery, parsing `zpool list` output, and cache-backed queries.

use std::collections::HashMap;

use tokio::process::Command as TokioCommand;
use tracing::{debug, info};

use crate::error::Result;
use crate::pool_helpers::parse_size_with_units;
use nestgate_core::error::NestGateError;

use super::manager::ZfsPoolManager;
use super::types::{PoolCapacity, PoolHealth, PoolInfo, PoolState};

impl ZfsPoolManager {
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

        // Store discovered pools in cache (parse without holding the write lock)
        let mut to_insert: Vec<PoolInfo> = Vec::new();
        for line in stdout.lines() {
            if let Some(pool_info) = self.parse_pool_line(line)? {
                to_insert.push(pool_info);
            }
        }
        self.discovered_pools.write().await.extend(
            to_insert
                .into_iter()
                .map(|pool_info| (pool_info.name.clone(), pool_info)),
        );
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
            let name = pool_name.to_string();
            {
                let mut pools = self.discovered_pools.write().await;
                pools.insert(name.clone(), pool_info);
            }
            debug!("Updated pool info for {} in discovered pools cache", name);
        }
        Ok(())
    }

    /// Discover a single pool by name
    pub(crate) async fn discover_single_pool(&self, pool_name: &str) -> Result<Option<PoolInfo>> {
        use crate::error::{ZfsOperation, create_zfs_error};

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

#[cfg(test)]
pub(super) fn sample_pool_info(
    name: &str,
    health: super::types::PoolHealth,
    state: super::types::PoolState,
    total_bytes: u64,
    used_bytes: u64,
    available_bytes: u64,
    utilization_percent: f64,
) -> super::types::PoolInfo {
    use std::collections::HashMap;

    use super::types::{PoolCapacity, PoolInfo};

    PoolInfo {
        name: name.to_string(),
        state,
        health,
        capacity: PoolCapacity {
            total: total_bytes,
            total_bytes,
            used: used_bytes,
            used_bytes,
            available: available_bytes,
            available_bytes,
            utilization_percent,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
        },
        devices: Vec::new(),
        properties: HashMap::new(),
    }
}

#[cfg(test)]
#[expect(
    clippy::float_cmp,
    reason = "parse tests compare sentinel 0.0 from invalid percent fallback and epsilon for parsed values"
)]
mod tests {
    use crate::pool::types::{PoolHealth, PoolState};

    use super::super::manager::ZfsPoolManager;
    use super::sample_pool_info;

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
        assert!((info.capacity.utilization_percent - 45.0).abs() < f64::EPSILON);
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
    fn parse_pool_line_exactly_five_fields_yields_none() {
        let m = ZfsPoolManager::new_for_testing();
        let out = m
            .parse_pool_line("a\tb\tc\td\te")
            .expect("test: parse five fields");
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

    #[test]
    fn parse_pool_line_unavail_health_and_offline_state() {
        let m = ZfsPoolManager::new_for_testing();
        let unavail = m
            .parse_pool_line("z\t1G\t0\t1G\t0%\tUNAVAIL")
            .expect("test: parse unavail")
            .expect("test: unavail row");
        assert_eq!(unavail.health, PoolHealth::Critical);
        assert_eq!(unavail.state, PoolState::Unknown);

        let offline = m
            .parse_pool_line("o\t1G\t0\t1G\t0%\tOFFLINE")
            .expect("test: parse offline")
            .expect("test: offline row");
        assert_eq!(offline.health, PoolHealth::Unknown);
        assert_eq!(offline.state, PoolState::Offline);
    }

    #[test]
    fn parse_pool_line_unknown_health_and_state() {
        let m = ZfsPoolManager::new_for_testing();
        let row = m
            .parse_pool_line("weird\t1G\t0\t1G\t0%\tSUSPENDED")
            .expect("test: parse weird")
            .expect("test: weird row");
        assert_eq!(row.health, PoolHealth::Unknown);
        assert_eq!(row.state, PoolState::Unknown);
    }

    #[test]
    fn parse_pool_line_invalid_capacity_percent_defaults_to_zero() {
        let m = ZfsPoolManager::new_for_testing();
        let row = m
            .parse_pool_line("x\t1G\t0\t1G\tnot-a-number%\tONLINE")
            .expect("test: parse bad cap")
            .expect("test: bad cap row");
        assert_eq!(row.capacity.utilization_percent, 0.0);
    }

    #[test]
    fn parse_pool_line_unparseable_sizes_default_to_zero() {
        let m = ZfsPoolManager::new_for_testing();
        let row = m
            .parse_pool_line("y\t???\t???\t???\t0%\tONLINE")
            .expect("test: parse bad sizes")
            .expect("test: bad sizes row");
        assert_eq!(row.capacity.total_bytes, 0);
        assert_eq!(row.capacity.used_bytes, 0);
        assert_eq!(row.capacity.available_bytes, 0);
    }

    #[test]
    fn parse_pool_line_dash_sizes_use_zero() {
        let m = ZfsPoolManager::new_for_testing();
        let row = m
            .parse_pool_line("dash\t-\t-\t-\t0%\tONLINE")
            .expect("test: parse dash")
            .expect("test: dash row");
        assert_eq!(row.capacity.total_bytes, 0);
    }

    #[test]
    fn parse_pool_line_devices_and_properties_empty() {
        let m = ZfsPoolManager::new_for_testing();
        let row = m
            .parse_pool_line("t\t10G\t5G\t5G\t0%\tONLINE")
            .expect("test: parse")
            .expect("test: row");
        assert!(row.devices.is_empty());
        assert!(row.properties.is_empty());
    }

    #[tokio::test]
    async fn list_pools_empty_without_discovery() {
        let m = ZfsPoolManager::new_for_testing();
        let pools = m.list_pools().await.expect("test: list_pools");
        assert!(pools.is_empty());
    }

    #[tokio::test]
    async fn list_pools_and_get_pool_info_from_cache() {
        let m = ZfsPoolManager::new_for_testing();
        let info = sample_pool_info(
            "cached",
            PoolHealth::Healthy,
            PoolState::Online,
            4096,
            1024,
            3072,
            25.0,
        );
        m.insert_pool_for_testing(info.clone()).await;

        let pools = m.list_pools().await.expect("test: list_pools");
        assert_eq!(pools.len(), 1);
        assert_eq!(pools[0].name, "cached");

        let got = m
            .get_pool_info("cached")
            .await
            .expect("test: get_pool_info");
        assert_eq!(got.name, info.name);
        assert_eq!(got.capacity.total_bytes, 4096);
    }

    #[tokio::test]
    async fn get_pool_info_cache_miss_returns_err() {
        let m = ZfsPoolManager::new_for_testing();
        m.get_pool_info("missing_pool_xyz")
            .await
            .expect_err("test: expected pool lookup failure");
    }

    #[tokio::test]
    #[ignore = "Requires real ZFS"]
    async fn discover_pools_runs_zpool_list() {
        let m = ZfsPoolManager::new_for_testing();
        let _ = m.discover_pools().await;
    }

    #[tokio::test]
    async fn refresh_pool_info_is_ok_when_pool_not_discoverable() {
        let m = ZfsPoolManager::new_for_testing();
        m.refresh_pool_info("nonexistent_pool_for_refresh_test")
            .await
            .expect("test: refresh should not error when discovery yields none");
    }
}
