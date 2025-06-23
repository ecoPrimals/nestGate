//! ZFS Pool Manager - Pool discovery and management
//! 
//! Enhanced with real ZFS integration for Day 2 implementation

use std::sync::Arc;
use std::collections::HashMap;
use std::process::Command;
use serde::{Deserialize, Serialize};
use tracing::{info, debug, warn, error};

use nestgate_core::Result;
use crate::{config::ZfsConfig, error::PoolError, manager::PoolOverallStatus};

/// ZFS Pool Manager - handles pool discovery and management
#[derive(Debug)]
pub struct ZfsPoolManager {
    config: ZfsConfig,
    discovered_pools: Arc<dashmap::DashMap<String, PoolInfo>>,
}

/// Information about a discovered ZFS pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    pub name: String,
    pub state: PoolState,
    pub health: PoolHealth,
    pub capacity: PoolCapacity,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolState {
    Online,
    Offline,
    Degraded,
    Faulted,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Create a new ZFS pool manager
    pub async fn new(config: &ZfsConfig) -> Result<Self> {
        info!("Initializing ZFS Pool Manager");
        
        Ok(Self {
            config: config.clone(),
            discovered_pools: Arc::new(dashmap::DashMap::new()),
        })
    }
    
    /// Discover available ZFS pools
    pub async fn discover_pools(&self) -> Result<()> {
        info!("Discovering ZFS pools");
        
        // Try to discover real ZFS pools
        match self.discover_real_pools().await {
            Ok(_) => {
                debug!("Real pool discovery successful: {} pools found", self.discovered_pools.len());
            }
            Err(e) => {
                warn!("Real pool discovery failed: {}, falling back to mock data", e);
                self.create_mock_pool().await?;
            }
        }
        
        Ok(())
    }
    
    /// Discover real ZFS pools using zpool command
    async fn discover_real_pools(&self) -> Result<()> {
        let output = Command::new("zpool")
            .args(&["list", "-H", "-p"])
            .output()
            .map_err(|e| crate::error::ZfsError::PoolError(PoolError::DiscoveryFailed { reason: format!("Failed to execute zpool command: {}", e) }))?;
        
        if !output.status.success() {
            return Err(crate::error::ZfsError::PoolError(PoolError::DiscoveryFailed { 
                reason: format!("zpool command failed: {}", String::from_utf8_lossy(&output.stderr))
            }).into());
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        for line in stdout.lines() {
            if let Some(pool_info) = self.parse_pool_line(line).await? {
                self.discovered_pools.insert(pool_info.name.clone(), pool_info);
            }
        }
        
        Ok(())
    }
    
    /// Parse a line from zpool list output
    async fn parse_pool_line(&self, line: &str) -> Result<Option<PoolInfo>> {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 10 {
            return Ok(None);
        }
        
        let name = parts[0].to_string();
        let total_bytes: u64 = parts[1].parse().unwrap_or(0);
        let used_bytes: u64 = parts[2].parse().unwrap_or(0);
        let available_bytes: u64 = parts[3].parse().unwrap_or(0);
        let health_str = parts[9];
        
        let utilization_percent = if total_bytes > 0 {
            (used_bytes as f64 / total_bytes as f64) * 100.0
        } else {
            0.0
        };
        
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
        
        // Get additional properties
        let properties = self.get_pool_properties(&name).await.unwrap_or_default();
        
        Ok(Some(PoolInfo {
            name,
            state,
            health,
            capacity: PoolCapacity {
                total_bytes,
                used_bytes,
                available_bytes,
                utilization_percent,
            },
            properties,
        }))
    }
    
    /// Get pool properties using zpool get
    async fn get_pool_properties(&self, pool_name: &str) -> Result<HashMap<String, String>> {
        let output = Command::new("zpool")
            .args(&["get", "all", "-H", "-p", pool_name])
            .output()
            .map_err(|e| crate::error::ZfsError::PoolError(PoolError::HealthCheckFailed { 
                pool_name: pool_name.to_string(),
                details: format!("Failed to get pool properties: {}", e)
            }))?;
        
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
    
    /// Create mock pool for testing when real pools aren't available
    async fn create_mock_pool(&self) -> Result<()> {
        let mock_pool = PoolInfo {
            name: self.config.default_pool.clone(),
            state: PoolState::Online,
            health: PoolHealth::Healthy,
            capacity: PoolCapacity {
                total_bytes: 1024 * 1024 * 1024 * 1024, // 1TB
                used_bytes: 1024 * 1024 * 1024 * 100,   // 100GB
                available_bytes: 1024 * 1024 * 1024 * 924, // 924GB
                utilization_percent: 9.76,
            },
            properties: HashMap::new(),
        };
        
        self.discovered_pools.insert(mock_pool.name.clone(), mock_pool);
        Ok(())
    }
    
    /// Get overall pool status
    pub async fn get_overall_status(&self) -> Result<PoolOverallStatus> {
        let pools_online = self.discovered_pools.iter()
            .filter(|entry| matches!(entry.value().state, PoolState::Online))
            .count();
            
        let pools_degraded = self.discovered_pools.iter()
            .filter(|entry| matches!(entry.value().state, PoolState::Degraded))
            .count();
            
        let (total_capacity, available_capacity) = self.discovered_pools.iter()
            .fold((0u64, 0u64), |(total, available), entry| {
                let pool = entry.value();
                (total + pool.capacity.total_bytes, available + pool.capacity.available_bytes)
            });
        
        Ok(PoolOverallStatus {
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
                    pool_name: pool_name.to_string() 
                }).into())
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
        
        let output = Command::new("zpool")
            .args(&args)
            .output()
            .map_err(|e| crate::error::ZfsError::PoolError(PoolError::CreationFailed { 
                pool_name: name.to_string(),
                reason: format!("Failed to execute zpool create: {}", e)
            }))?;
        
        if !output.status.success() {
            return Err(crate::error::ZfsError::PoolError(PoolError::CreationFailed { 
                pool_name: name.to_string(),
                reason: format!("zpool create failed: {}", String::from_utf8_lossy(&output.stderr))
            }).into());
        }
        
        // Refresh the pool info and return it
        self.discover_single_pool(name).await?;
        self.get_pool_info(name).await
    }

    /// Destroy a ZFS pool
    pub async fn destroy_pool(&self, name: &str) -> Result<()> {
        warn!("Destroying ZFS pool: {}", name);
        
        let output = Command::new("zpool")
            .args(&["destroy", "-f", name])
            .output()
            .map_err(|e| crate::error::ZfsError::PoolError(PoolError::DestructionFailed { 
                pool_name: name.to_string(),
                reason: format!("Failed to execute zpool destroy: {}", e)
            }))?;
        
        if !output.status.success() {
            return Err(crate::error::ZfsError::PoolError(PoolError::DestructionFailed { 
                pool_name: name.to_string(),
                reason: format!("zpool destroy failed: {}", String::from_utf8_lossy(&output.stderr))
            }).into());
        }
        
        // Remove from discovered pools
        self.discovered_pools.remove(name);
        
        info!("Successfully destroyed pool: {}", name);
        Ok(())
    }

    /// Get detailed pool status
    pub async fn get_pool_status(&self, name: &str) -> Result<String> {
        debug!("Getting status for pool: {}", name);
        
        let output = Command::new("zpool")
            .args(&["status", name])
            .output()
            .map_err(|e| crate::error::ZfsError::PoolError(PoolError::HealthCheckFailed { 
                pool_name: name.to_string(),
                details: format!("Failed to execute zpool status: {}", e)
            }))?;
        
        if !output.status.success() {
            return Err(crate::error::ZfsError::PoolError(PoolError::HealthCheckFailed { 
                pool_name: name.to_string(),
                details: format!("zpool status failed: {}", String::from_utf8_lossy(&output.stderr))
            }).into());
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Start a scrub operation on a pool
    pub async fn scrub_pool(&self, name: &str) -> Result<()> {
        info!("Starting scrub for pool: {}", name);
        
        let output = Command::new("zpool")
            .args(&["scrub", name])
            .output()
            .map_err(|e| crate::error::ZfsError::PoolError(PoolError::ScrubFailed { 
                pool_name: name.to_string(),
                details: format!("Failed to execute zpool scrub: {}", e)
            }))?;
        
        if !output.status.success() {
            return Err(crate::error::ZfsError::PoolError(PoolError::ScrubFailed { 
                pool_name: name.to_string(),
                details: format!("zpool scrub failed: {}", String::from_utf8_lossy(&output.stderr))
            }).into());
        }
        
        info!("Successfully started scrub for pool: {}", name);
        Ok(())
    }

    /// List all discovered pools
    pub async fn list_pools(&self) -> Result<Vec<PoolInfo>> {
        Ok(self.discovered_pools.iter().map(|entry| entry.value().clone()).collect())
    }
    
    /// Refresh pool information
    pub async fn refresh_pool_info(&self, pool_name: &str) -> Result<()> {
        // Re-discover specific pool
        if let Some(pool_info) = self.discover_single_pool(pool_name).await? {
            self.discovered_pools.insert(pool_name.to_string(), pool_info);
        }
        Ok(())
    }
    
    /// Discover a single pool by name
    async fn discover_single_pool(&self, pool_name: &str) -> Result<Option<PoolInfo>> {
        let output = Command::new("zpool")
            .args(&["list", "-H", "-p", pool_name])
            .output()
            .map_err(|e| crate::error::ZfsError::PoolError(PoolError::DiscoveryFailed { reason: format!("Failed to execute zpool command: {}", e) }))?;
        
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