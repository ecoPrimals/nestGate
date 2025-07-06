//! Songbird Orchestrator Integration
//!
//! This module provides integration with Songbird orchestrator for
//! service discovery and registration.

use std::collections::HashMap;
use std::time::SystemTime;
// use reqwest::Client;  // Commented out until reqwest is properly configured
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};
use uuid::Uuid;
// use nestgate::songbird_integration::{NestGateServiceInfo, NestGateHealth};  // Commented out until available

use crate::{error::ZfsError, Result};
type ZfsResult<T> = Result<T>;

/// ZFS service for Songbird integration
#[derive(Debug)]
pub struct NestGateZfsService {
    service_id: String,
    config: ZfsServiceConfig,
    node_id: String,
    last_health_check: Option<SystemTime>,
    registered_with_orchestrator: bool,
    // client: Client,  // Commented out until reqwest is available
}

/// Configuration for ZFS service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsServiceConfig {
    pub service_name: String,
    pub service_version: String,
    pub capabilities: Vec<String>,
    pub health_check_interval: u64,
}

impl Default for ZfsServiceConfig {
    fn default() -> Self {
        Self {
            service_name: "nestgate-zfs".to_string(),
            service_version: "0.1.0".to_string(),
            capabilities: vec![
                "zfs-pool-management".to_string(),
                "zfs-dataset-management".to_string(),
                "zfs-snapshot-management".to_string(),
                "tier-management".to_string(),
            ],
            health_check_interval: 30,
        }
    }
}

/// Health status for ZFS service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsHealthStatus {
    pub service_id: String,
    pub status: String,
    pub pools_healthy: u32,
    pub pools_degraded: u32,
    pub total_capacity: u64,
    pub available_capacity: u64,
    pub last_check: u64,
}

impl NestGateZfsService {
    /// Create a new ZFS service instance
    pub fn new(service_id: String, config: ZfsServiceConfig) -> Self {
        let node_id = Uuid::new_v4().to_string();
        Self {
            service_id,
            config,
            node_id,
            last_health_check: None,
            registered_with_orchestrator: false,
            // client: Client::new(),  // Commented out until reqwest is available
        }
    }

    /// Get service information for Songbird registration
    pub fn get_service_info(&self) -> ServiceInfo {
        ServiceInfo {
            service_id: self.service_id.clone(),
            service_type: "nestgate-zfs".to_string(),
            version: self.config.service_version.clone(),
            capabilities: self.config.capabilities.clone(),
            metadata: HashMap::new(),
        }
    }

    /// Get current health status
    pub async fn get_health_status(&mut self) -> Result<ZfsHealthStatus> {
        // Perform real ZFS health checks
        let pool_health = self.check_pool_health().await?;
        let dataset_health = self.check_dataset_health().await?;
        let system_health = self.check_system_health().await?;

        let overall_healthy = pool_health && dataset_health && system_health;

        // Update last health check timestamp
        self.last_health_check = Some(SystemTime::now());

        Ok(ZfsHealthStatus {
            service_id: self.service_id.clone(),
            status: if overall_healthy {
                "healthy"
            } else {
                "degraded"
            }
            .to_string(),
            pools_healthy: if pool_health { 1 } else { 0 },
            pools_degraded: if pool_health { 0 } else { 1 },
            total_capacity: 1000000000000,    // 1TB placeholder
            available_capacity: 500000000000, // 500GB placeholder
            last_check: SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_else(|_| {
                    std::time::Duration::from_secs(
                        std::env::var("NESTGATE_ZFS_DEFAULT_TIMEOUT_SECS")
                            .ok()
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0), // 0 seconds default (immediate)
                    )
                })
                .as_secs(),
        })
    }

    /// Register with Songbird orchestrator
    pub async fn register_with_songbird(&mut self, _songbird_url: &str) -> Result<()> {
        // Implement ZFS service registration with orchestrator
        info!("🔗 Registering ZFS service with orchestrator");

        let registration_data = serde_json::json!({
            "service_name": "nestgate-zfs",
            "service_type": "storage",
            "capabilities": ["pool_management", "dataset_operations", "tiered_storage"],
            "health_endpoint": "/health",
            "version": env!("CARGO_PKG_VERSION"),
            "node_id": self.node_id,
            "supported_features": ["raidz", "compression", "deduplication", "snapshots"]
        });

        // For now, log the registration (actual HTTP call would go here when reqwest is available)
        info!("📝 Service registration payload: {}", registration_data);
        info!("✅ ZFS service registration prepared");

        // Store registration state
        self.registered_with_orchestrator = true;
        // let registration = ServiceRegistration {
        //     service_id: self.service_id.clone(),
        //     service_type: "nestgate-zfs".to_string(),
        //     capabilities: self.config.capabilities.clone(),
        //     endpoints: vec![format!("http://localhost:8080/zfs")],
        //     metadata: HashMap::new(),
        // };
        //
        // let response = self.client
        //     .post(&format!("{}/register", songbird_url))
        //     .json(&registration)
        //     .send()
        //     .await?;
        //
        // if response.status().is_success() {
        //     Ok(())
        // } else {
        //     Err(crate::error::ZfsError::Internal(
        //         format!("Registration failed: {}", response.status())
        //     ))
        // }

        Ok(()) // Placeholder implementation
    }

    /// Check ZFS pool health
    async fn check_pool_health(&self) -> ZfsResult<bool> {
        debug!("🔍 Checking ZFS pool health");

        let output = tokio::process::Command::new("zpool")
            .args(["status", "-x"])
            .output()
            .await
            .map_err(|e| ZfsError::Internal {
                message: format!("Failed to check pool status: {}", e),
            })?;

        if !output.status.success() {
            warn!("⚠️ Pool status check failed");
            return Ok(false);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // If all pools are healthy, zpool status -x returns "all pools are healthy"
        if stdout.contains("all pools are healthy") {
            info!("✅ All ZFS pools are healthy");
            Ok(true)
        } else {
            warn!("⚠️ ZFS pool health issues detected: {}", stdout);
            Ok(false)
        }
    }

    /// Check ZFS dataset health
    async fn check_dataset_health(&self) -> ZfsResult<bool> {
        debug!("🔍 Checking ZFS dataset health");

        let output = tokio::process::Command::new("zfs")
            .args(["list", "-H", "-o", "name,available,used"])
            .output()
            .await
            .map_err(|e| ZfsError::Internal {
                message: format!("Failed to list datasets: {}", e),
            })?;

        if !output.status.success() {
            warn!("⚠️ Dataset listing failed");
            return Ok(false);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Basic check: ensure we have datasets and they're accessible
        if stdout.trim().is_empty() {
            warn!("⚠️ No datasets found");
            Ok(false)
        } else {
            info!("✅ ZFS datasets accessible");
            Ok(true)
        }
    }

    /// Check system-level health for ZFS operations
    async fn check_system_health(&self) -> ZfsResult<bool> {
        debug!("🔍 Checking system health for ZFS");

        // Check available memory (ZFS is memory-intensive)
        let memory_ok = self.check_memory_health().await?;

        // Check ZFS kernel module
        let zfs_module_ok = self.check_zfs_module().await?;

        Ok(memory_ok && zfs_module_ok)
    }

    /// Check available memory for ZFS operations
    async fn check_memory_health(&self) -> ZfsResult<bool> {
        let memory_info = tokio::fs::read_to_string("/proc/meminfo")
            .await
            .map_err(|e| ZfsError::Internal {
                message: format!("Failed to read memory info: {}", e),
            })?;

        let mut total_memory = 0u64;
        let mut available_memory = 0u64;

        for line in memory_info.lines() {
            if line.starts_with("MemTotal:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    total_memory = value.parse().unwrap_or(0);
                }
            } else if line.starts_with("MemAvailable:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    available_memory = value.parse().unwrap_or(0);
                }
            }
        }

        if total_memory == 0 {
            warn!("⚠️ Could not determine total memory");
            return Ok(false);
        }

        let memory_usage_percent =
            ((total_memory - available_memory) as f64 / total_memory as f64) * 100.0;

        if memory_usage_percent > 90.0 {
            warn!("⚠️ High memory usage: {:.1}%", memory_usage_percent);
            Ok(false)
        } else {
            debug!("✅ Memory usage healthy: {:.1}%", memory_usage_percent);
            Ok(true)
        }
    }

    /// Check ZFS kernel module availability
    async fn check_zfs_module(&self) -> ZfsResult<bool> {
        let modules_info = tokio::fs::read_to_string("/proc/modules")
            .await
            .map_err(|e| ZfsError::Internal {
                message: format!("Failed to read modules info: {}", e),
            })?;

        if modules_info.contains("zfs ") {
            debug!("✅ ZFS kernel module loaded");
            Ok(true)
        } else {
            warn!("⚠️ ZFS kernel module not found");
            Ok(false)
        }
    }
}

/// Service information for registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub service_id: String,
    pub service_type: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub metadata: HashMap<String, String>,
}
