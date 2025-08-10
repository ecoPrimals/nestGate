//! ZFS Pool Manager - Pool discovery and management
//!
//! Enhanced with real ZFS integration for Day 2 implementation

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::process::Command as TokioCommand;
// Removed unused tracing import

use crate::{config::ZfsConfig, error::PoolError};
use nestgate_core::{NestGateError, Result};
use tracing::debug;
use tracing::info;
use tracing::warn;

/// ZFS Pool Manager - handles pool discovery and management
#[derive(Debug)]
pub struct ZfsPoolManager {
    #[allow(dead_code)]
    config: Arc<ZfsConfig>,
    discovered_pools: Arc<DashMap<String, PoolInfo>>,
}

/// Information about a discovered ZFS pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    pub name: String,
    pub state: PoolState,
    pub health: PoolHealth,
    pub capacity: PoolCapacity,
    pub devices: Vec<String>,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PoolState {
    Online,
    Offline,
    Degraded,
    Faulted,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PoolHealth {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolCapacity {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub utilization_percent: f64,
}

impl ZfsPoolManager {
    /// Create a new ZFS pool manager (async)
    pub async fn new(config: &ZfsConfig) -> Result<Self> {
        info!("Initializing ZFS pool manager");

        let manager = Self {
            config: Arc::new(config.clone()),
            discovered_pools: Arc::new(DashMap::new()),
        };

        // Test ZFS availability
        if !crate::is_zfs_available().await {
            warn!("ZFS not available, running in mock mode");
        }

        Ok(manager)
    }

    /// Create a new ZFS pool manager with owned config (zero-copy)
    pub async fn with_owned_config(config: ZfsConfig) -> Result<Self> {
        info!("Initializing ZFS pool manager with owned config");

        let manager = Self {
            config: Arc::new(config),
            discovered_pools: Arc::new(DashMap::new()),
        };

        // Test ZFS availability
        if !crate::is_zfs_available().await {
            warn!("ZFS not available, running in mock mode");
        }

        Ok(manager)
    }

    /// Create a pool manager for testing and development (now uses real ZFS commands)
    pub fn new_for_testing() -> Self {
        Self {
            config: Arc::new(ZfsConfig::default()),
            discovered_pools: Arc::new(DashMap::new()),
        }
    }

    /// Create instance for real production use
    pub fn new_production(config: ZfsConfig) -> Self {
        Self {
            config: Arc::new(config),
            discovered_pools: Arc::new(DashMap::new()),
        }
    }

    /// Discover all available ZFS pools
    pub async fn discover_pools(&self) -> Result<()> {
        info!("Discovering ZFS pools");

        let output = TokioCommand::new("zpool")
            .args(["list", "-H", "-o", "name,size,alloc,free,cap,health"])
            .output()
            .await
            .map_err(|e| NestGateError::Internal {
                message: format!("Failed to execute zpool list: {}", e),
                location: Some(format!("{}:{}", file!(), line!())),
                debug_info: None,
                is_bug: false,
            })?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(NestGateError::Internal {
                message: format!("zpool list failed: {}", error_msg),
                location: Some(format!("{}:{}", file!(), line!())),
                debug_info: None,
                is_bug: false,
            });
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut pools = Vec::new();

        for line in stdout.lines() {
            if let Some(pool_info) = self.parse_pool_line(line).await? {
                pools.push(pool_info);
            }
        }

        info!("Discovered {} ZFS pools", pools.len());
        Ok(())
    }

    /// Discover real ZFS pools on the system
    #[allow(dead_code)]
    async fn discover_real_pools(&self) -> Result<()> {
        let output = TokioCommand::new("zpool")
            .args(["list", "-H", "-p"])
            .output()
            .await
            .map_err(|e| {
                crate::error::ZfsError::PoolError(PoolError::DiscoveryFailed {
                    reason: format!("Failed to execute zpool command: {e}"),
                })
            })?;

        if !output.status.success() {
            return Err(
                crate::error::ZfsError::PoolError(PoolError::DiscoveryFailed {
                    reason: format!(
                        "zpool command failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ),
                })
                .into(),
            );
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            if let Some(pool_info) = self.parse_pool_line(line).await? {
                self.discovered_pools
                    .insert(pool_info.name.clone(), pool_info);
            }
        }
        Ok(())
    }

    /// Parse a single line from zpool list output
    async fn parse_pool_line(&self, line: &str) -> Result<Option<PoolInfo>> {
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
        let total_bytes = self.parse_size_with_units(size_str).unwrap_or(0);
        let used_bytes = self.parse_size_with_units(alloc_str).unwrap_or(0);
        let available_bytes = self.parse_size_with_units(free_str).unwrap_or(0);

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
                total_bytes,
                used_bytes,
                available_bytes,
                utilization_percent: cap_str.trim_end_matches('%').parse().unwrap_or(0.0),
            },
            devices: Vec::new(), // Would be populated by separate command
            properties: HashMap::new(),
        }))
    }

    /// Parse size string with units (simplified implementation)
    fn parse_size_with_units(&self, size_str: &str) -> Option<u64> {
        if size_str == "-" {
            return Some(0);
        }

        let size_str = size_str.trim();
        let (number_part, unit) = if let Some(last_char) = size_str.chars().last() {
            if last_char.is_alphabetic() {
                let unit_start = size_str.len() - 1;
                (&size_str[..unit_start], &size_str[unit_start..])
            } else {
                (size_str, "")
            }
        } else {
            (size_str, "")
        };

        let number: f64 = number_part.parse().ok()?;

        let multiplier = match unit.to_uppercase().as_str() {
            "" | "B" => 1,
            "K" => 1024,
            "M" => 1024 * 1024,
            "G" => 1024 * 1024 * 1024,
            "T" => 1024_u64 * 1024 * 1024 * 1024,
            "P" => 1024_u64 * 1024 * 1024 * 1024 * 1024,
            _ => return None,
        };

        Some((number * multiplier as f64) as u64)
    }

    /// Get pool properties using zpool command
    #[allow(dead_code)]
    async fn get_pool_properties(&self, pool_name: &str) -> Result<HashMap<String, String>> {
        let output = TokioCommand::new("zpool")
            .args(["get", "all", "-H", "-p", pool_name])
            .output()
            .await
            .map_err(|e| {
                crate::error::ZfsError::PoolError(PoolError::HealthCheckFailed {
                    pool_name: pool_name.to_string(),
                    details: format!("Failed to get pool properties: {e}"),
                })
            })?;

        if !output.status.success() {
            return Ok(HashMap::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut properties = HashMap::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 {
                properties.insert(parts[1].to_string(), parts[2].to_string());
            }
        }

        Ok(properties)
    }

    /// Ensure default pool exists for testing/development
    #[allow(dead_code)]
    async fn ensure_default_pool(&self) -> Result<()> {
        if self.discovered_pools.is_empty() {
            info!("No pools discovered, attempting to create default pool");
            // Real pool creation logic would go here
            self.discover_pools().await?;
        }
        Ok(())
    }

    /// Get overall pool status
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
    pub async fn get_pool_info(&self, pool_name: &str) -> Result<PoolInfo> {
        if let Some(pool) = self.discovered_pools.get(pool_name) {
            Ok(pool.clone())
        } else {
            // Try to refresh this specific pool
            self.discover_single_pool(pool_name).await?;

            if let Some(pool) = self.discovered_pools.get(pool_name) {
                Ok(pool.clone())
            } else {
                Err(crate::error::ZfsError::PoolError(PoolError::NotFound {
                    pool_name: pool_name.to_string(),
                })
                .into())
            }
        }
    }

    /// Create a new ZFS pool
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
            .map_err(|e| {
                crate::error::ZfsError::PoolError(PoolError::CreationFailed {
                    pool_name: name.to_string(),
                    reason: format!("Failed to execute zpool create: {e}"),
                })
            })?;

        if !output.status.success() {
            return Err(
                crate::error::ZfsError::PoolError(PoolError::CreationFailed {
                    pool_name: name.to_string(),
                    reason: format!(
                        "zpool create failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ),
                })
                .into(),
            );
        }

        // Refresh the pool info and return it
        self.discover_single_pool(name).await?;
        self.get_pool_info(name).await
    }

    /// Destroy a ZFS pool
    pub async fn destroy_pool(&self, name: &str) -> Result<()> {
        warn!("Destroying ZFS pool: {}", name);

        let output = TokioCommand::new("zpool")
            .args(["destroy", "-f", name])
            .output()
            .await
            .map_err(|e| {
                crate::error::ZfsError::PoolError(PoolError::DestructionFailed {
                    pool_name: name.to_string(),
                    reason: format!("Failed to execute zpool destroy: {e}"),
                })
            })?;

        if !output.status.success() {
            return Err(
                crate::error::ZfsError::PoolError(PoolError::DestructionFailed {
                    pool_name: name.to_string(),
                    reason: format!(
                        "zpool destroy failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ),
                })
                .into(),
            );
        }

        // Remove from discovered pools
        self.discovered_pools.remove(name);

        info!("Successfully destroyed pool: {}", name);
        Ok(())
    }

    /// Get detailed pool status
    pub async fn get_pool_status(&self, name: &str) -> Result<String> {
        debug!("Getting status for pool: {}", name);

        let output = TokioCommand::new("zpool")
            .args(["status", name])
            .output()
            .await
            .map_err(|e| {
                crate::error::ZfsError::PoolError(PoolError::HealthCheckFailed {
                    pool_name: name.to_string(),
                    details: format!("Failed to execute zpool status: {e}"),
                })
            })?;

        if !output.status.success() {
            return Err(
                crate::error::ZfsError::PoolError(PoolError::HealthCheckFailed {
                    pool_name: name.to_string(),
                    details: format!(
                        "zpool status failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ),
                })
                .into(),
            );
        }

        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// Start a scrub operation on a pool
    pub async fn scrub_pool(&self, name: &str) -> Result<()> {
        info!("Starting scrub for pool: {}", name);

        let output = TokioCommand::new("zpool")
            .args(["scrub", name])
            .output()
            .await
            .map_err(|e| {
                crate::error::ZfsError::PoolError(PoolError::ScrubFailed {
                    pool_name: name.to_string(),
                    details: format!("Failed to execute zpool scrub: {e}"),
                })
            })?;

        if !output.status.success() {
            return Err(crate::error::ZfsError::PoolError(PoolError::ScrubFailed {
                pool_name: name.to_string(),
                details: format!(
                    "zpool scrub failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            })
            .into());
        }

        info!("Successfully started scrub for pool: {}", name);
        Ok(())
    }

    /// List all discovered pools
    pub async fn list_pools(&self) -> Result<Vec<PoolInfo>> {
        Ok(self
            .discovered_pools
            .iter()
            .map(|entry| entry.value().clone())
            .collect())
    }

    /// Refresh pool information
    pub async fn refresh_pool_info(&self, pool_name: &str) -> Result<()> {
        // Re-discover specific pool
        if let Some(pool_info) = self.discover_single_pool(pool_name).await? {
            self.discovered_pools
                .insert(pool_name.to_string(), pool_info);
        }
        Ok(())
    }

    /// Discover a single pool by name
    async fn discover_single_pool(&self, pool_name: &str) -> Result<Option<PoolInfo>> {
        let output = TokioCommand::new("zpool")
            .args(["list", "-H", "-p", pool_name])
            .output()
            .await
            .map_err(|e| {
                crate::error::ZfsError::PoolError(PoolError::DiscoveryFailed {
                    reason: format!("Failed to execute zpool command: {e}"),
                })
            })?;

        if !output.status.success() {
            return Ok(None);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().next() {
            return self.parse_pool_line(line).await;
        }

        Ok(None)
    }
}
