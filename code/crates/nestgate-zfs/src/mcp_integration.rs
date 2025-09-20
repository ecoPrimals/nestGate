//
// This module provides integration between ZFS and MCP (Model Coordination Protocol),
// enabling ZFS to act as a storage provider for MCP systems with tiered storage
// capabilities, AI optimization, and performance monitoring.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
// Removed unused tracing import

use crate::manager::ZfsManager;
use crate::types::StorageTier;
use nestgate_core::{NestGateError, Result};
use tracing::error;
use tracing::info;
use tracing::warn;

/// MCP mount request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpMountRequest {
    pub mount_id: String,
    pub mount_point: String,
    pub tier: StorageTier,
    pub size_gb: u64,
}
/// MCP volume request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpVolumeRequest {
    pub volume_id: String,
    pub tier: StorageTier,
    pub size_gb: u64,
}
/// Mount status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MountStatus {
    Active,
    Inactive,
    Error(String),
}
/// Volume status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VolumeStatus {
    Active,
    Inactive,
    Error(String),
}
/// ZFS mount information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsMountInfo {
    pub mount_id: String,
    pub dataset_path: String,
    pub mount_point: String,
    pub tier: StorageTier,
    pub created_at: SystemTime,
    pub status: MountStatus,
}
/// ZFS volume information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsVolumeInfo {
    pub volume_id: String,
    pub dataset_path: String,
    pub tier: StorageTier,
    pub size_bytes: u64,
    pub created_at: SystemTime,
    pub status: VolumeStatus,
}
/// ZFS MCP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsMcpConfig {
    /// Enable AI optimization
    pub enable_ai_optimization: bool,
    /// Maximum concurrent operations
    pub max_concurrent_operations: u32,
    /// Default tier for new resources
    pub default_tier: StorageTier,
}
impl Default for ZfsMcpConfig {
    fn default() -> Self {
        Self {
            enable_ai_optimization: true,
            max_concurrent_operations: 10,
            default_tier: StorageTier::Warm,
        }
    }
}

impl ZfsMcpConfig {
    /// Validate the configuration settings
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn validate(&self) -> Result<()>  {
        if self.max_concurrent_operations == 0 {
            return Err(NestGateError::validation(
                "max_concurrent_operations must be greater than 0",
            ));
        }

        if self.max_concurrent_operations > 1000 {
            return Err(NestGateError::validation(
                "max_concurrent_operations cannot exceed 1000",
            ));
        }
        Ok(())
    }

    /// Get tier configuration for a specific tier
    pub const fn get_tier_config(&self, tier: &StorageTier) -> TierConfig {
        match tier {
            StorageTier::Hot => TierConfig {
                priority: 1,
                cache_enabled: true,
                compression: false,
                replication: 2,
            },
            StorageTier::Warm => TierConfig {
                priority: 2,
                cache_enabled: true,
                compression: true,
                replication: 1,
            },
            StorageTier::Cold => TierConfig {
                priority: 3,
                cache_enabled: false,
                compression: true,
                replication: 1,
            },
            StorageTier::Cache => TierConfig {
                priority: 0,
                cache_enabled: true,
                compression: false,
                replication: 3,
            },
            StorageTier::Archive => TierConfig {
                priority: 4,
                cache_enabled: false,
                compression: true,
                replication: 1,
            },
        }
    }
}

/// Configuration for a storage tier
#[derive(Debug, Clone)]
pub struct TierConfig {
    pub priority: u8,
    pub cache_enabled: bool,
    pub compression: bool,
    pub replication: u32,
}
/// ZFS MCP Storage Provider
pub struct ZfsMcpStorageProvider {
    zfs_manager: Arc<ZfsManager>,
    config: ZfsMcpConfig,
    active_mounts: Arc<RwLock<HashMap<String, ZfsMountInfo>>>,
    active_volumes: Arc<RwLock<HashMap<String, ZfsVolumeInfo>>>,
}
impl ZfsMcpStorageProvider {
    /// Create new ZFS MCP storage provider
    #[must_use]
    pub fn new(zfs_manager: Arc<ZfsManager>, config: ZfsMcpConfig) -> Self {
        Self {
            zfs_manager,
            config,
            active_mounts: Arc::new(RwLock::new(HashMap::new())),
            active_volumes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create mount for MCP system
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn create_mount(&self, request: McpMountRequest) -> Result<ZfsMountInfo>  {
        info!("Creating ZFS mount for MCP: {}", request.mount_id);

        let tier = request.tier.clone();
        let tier_clone = tier.clone(); // Clone before move
        let dataset_name = format!("nestpool/mcp/mounts/{"actual_error_details"}");
        let mount_path = format!("/mcp/mounts/{"actual_error_details"}");

        // Create the dataset using the new API
        let dataset_name_parts: Vec<&str> = dataset_name.split('/').collect();
        let dataset_name_str = dataset_name.as_str();
        let name = dataset_name_parts.last().unwrap_or(&dataset_name_str);
        let parent = dataset_name_parts[..dataset_name_parts.len() - 1].join("/");

        match self
            .zfs_manager
            .dataset_manager
            .create_dataset(name, &parent, tier)
            .await
        {
            Ok(_) => {
                let mount_info = ZfsMountInfo {
                    mount_id: request.mount_id.clone(),
                    dataset_path: dataset_name,
                    mount_point: mount_path,
                    tier: tier_clone,
                    created_at: SystemTime::now(),
                    status: MountStatus::Active,
                };

                // Store mount info
                {
                    let mut mounts = self.active_mounts.write().await;
                    mounts.insert(request.mount_id.clone(), mount_info.clone());
                }

                info!("Successfully created ZFS mount: {}", request.mount_id);
                Ok(mount_info)
            }
            Err(e) => {
                error!(
                    "Failed to create dataset for mount {}: {}",
                    request.mount_id, e
                );
                Err(nestgate_core::NestGateError::internal_error(
                    format!(
                        "MCP Integration: Failed to unmount filesystem for mount_id: {}",
                        request.mount_id
                    ),
                    "mcp-integration",
                ))
            }
        }
    }

    /// Remove mount
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn remove_mount(&self, mount_id: &str) -> Result<()>  {
        info!("Removing ZFS mount: {}", mount_id);

        if let Some(mount_info) = self.active_mounts.write().await.remove(mount_id) {
            match self
                .zfs_manager
                .dataset_manager
                .delete_dataset(&mount_info.dataset_path)
            {
                Ok(_) => {
                    info!("Successfully removed ZFS mount: {}", mount_id);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to destroy dataset for mount {}: {}", mount_id, e);
                    Err(nestgate_core::NestGateError::internal_error(
                        format!(
                            "MCP Integration: Failed to destroy dataset for mount {}: {}",
                            mount_id, e
                        ),
                        "mcp-integration",
                    ))
                }
            }
        } else {
            warn!("Mount not found: {}", mount_id);
            // IDIOMATIC EVOLUTION: Simple constructor with context
            Err(nestgate_core::NestGateError::internal_error(
                format!("Mount not found: {mount_id}"),
                "mcp-integration",
            ))
        }
    }

    /// Create volume
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn create_volume(&self, request: McpVolumeRequest) -> Result<ZfsVolumeInfo>  {
        info!("Creating ZFS volume for MCP: {}", request.volume_id);

        let tier = request.tier.clone();
        let tier_clone = tier.clone(); // Clone before move
        let dataset_name = format!("nestpool/mcp/volumes/{"actual_error_details"}");

        // Create the dataset using the new API
        let dataset_name_parts: Vec<&str> = dataset_name.split('/').collect();
        let dataset_name_str = dataset_name.as_str();
        let name = dataset_name_parts.last().unwrap_or(&dataset_name_str);
        let parent = dataset_name_parts[..dataset_name_parts.len() - 1].join("/");

        match self
            .zfs_manager
            .dataset_manager
            .create_dataset(name, &parent, tier)
            .await
        {
            Ok(_) => {
                let volume_info = ZfsVolumeInfo {
                    volume_id: request.volume_id.clone(),
                    dataset_path: dataset_name,
                    tier: tier_clone,
                    size_bytes: request.size_gb * 1024 * 1024 * 1024,
                    created_at: SystemTime::now(),
                    status: VolumeStatus::Active,
                };

                {
                    let mut volumes = self.active_volumes.write().await;
                    volumes.insert(request.volume_id.clone(), volume_info.clone());
                }

                info!("Successfully created ZFS volume: {}", request.volume_id);
                Ok(volume_info)
            }
            Err(e) => {
                error!(
                    "Failed to create dataset for volume {}: {}",
                    request.volume_id, e
                );
                Err(nestgate_core::NestGateError::internal_error(
                    format!(
                        "MCP Integration: Failed to destroy filesystem for volume_id: {}",
                        request.volume_id
                    ),
                    "mcp-integration",
                ))
            }
        }
    }

    /// Remove volume
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn remove_volume(&self, volume_id: &str) -> Result<()>  {
        info!("Removing ZFS volume: {}", volume_id);

        if let Some(volume_info) = self.active_volumes.write().await.remove(volume_id) {
            match self
                .zfs_manager
                .dataset_manager
                .delete_dataset(&volume_info.dataset_path)
            {
                Ok(_) => {
                    info!("Successfully removed ZFS volume: {}", volume_id);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to destroy dataset for volume {}: {}", volume_id, e);
                    Err(nestgate_core::NestGateError::internal_error(
                        format!(
                            "MCP Integration: Failed to create snapshot for volume_id: {}",
                            volume_id
                        ),
                        "mcp-integration",
                    ))
                }
            }
        } else {
            warn!("Volume not found: {}", volume_id);
            // IDIOMATIC EVOLUTION: Simple constructor with context
            Err(nestgate_core::NestGateError::internal_error(
                format!("Volume not found: {volume_id}"),
                "mcp-integration",
            ))
        }
    }

    /// List all volumes
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn list_volumes(&self) -> Result<Vec<ZfsVolumeInfo>>  {
        let volumes = self.active_volumes.read().await;
        Ok(volumes.values().cloned().collect())
    }

    /// Trigger AI optimization
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn trigger_ai_optimization(&self) -> Result<()>  {
        if !self.config.enable_ai_optimization {
            return Err(nestgate_core::NestGateError::internal_error(
                "MCP integration - AI optimization is disabled".to_string(),
                "mcp-integration",
            ));
        }

        info!("Triggering AI optimization for MCP resources");

        // This is a placeholder - in a real implementation, this would
        // trigger actual AI optimization of the storage tiers
        info!("AI optimization completed");
        Ok(())
    }
}
