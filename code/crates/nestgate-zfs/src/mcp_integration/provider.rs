// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! ZFS-backed MCP storage provider implementation.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;

use crate::manager::ZfsManager;
use nestgate_core::Result;
use tracing::{error, info, warn};

use super::types::{
    McpMountRequest, McpVolumeRequest, MountStatus, VolumeStatus, ZfsMcpConfig, ZfsMountInfo,
    ZfsVolumeInfo,
};

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
    pub async fn create_mount(&self, request: McpMountRequest) -> Result<ZfsMountInfo> {
        info!("Creating ZFS mount for MCP: {}", request.mount_id);

        let tier = request.tier.clone();
        let tier_clone = tier.clone(); // Clone before move
        let dataset_name = format!("nestpool/mcp/mounts/{}", request.mount_id);
        let mount_path = format!("/mcp/mounts/{}", request.mount_id);

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
    pub async fn remove_mount(&self, mount_id: &str) -> Result<()> {
        info!("Removing ZFS mount: {}", mount_id);

        let mount_info = {
            let mut mounts = self.active_mounts.write().await;
            mounts.remove(mount_id)
        };
        if let Some(mount_info) = mount_info {
            match self
                .zfs_manager
                .dataset_manager
                .delete_dataset(&mount_info.dataset_path)
            {
                Ok(()) => {
                    info!("Successfully removed ZFS mount: {}", mount_id);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to destroy dataset for mount {}: {}", mount_id, e);
                    Err(nestgate_core::NestGateError::internal_error(
                        format!(
                            "MCP Integration: Failed to destroy dataset for mount {mount_id}: {e}"
                        ),
                        "mcp-integration",
                    ))
                }
            }
        } else {
            warn!("Mount not found: {}", mount_id);
            Err(nestgate_core::NestGateError::internal_error(
                format!("Mount not found: {mount_id}"),
                "mcp-integration",
            ))
        }
    }

    /// Create volume
    pub async fn create_volume(&self, request: McpVolumeRequest) -> Result<ZfsVolumeInfo> {
        info!("Creating ZFS volume for MCP: {}", request.volume_id);

        let tier = request.tier.clone();
        let tier_clone = tier.clone(); // Clone before move
        let dataset_name = format!("nestpool/mcp/volumes/{}", request.volume_id);

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
    pub async fn remove_volume(&self, volume_id: &str) -> Result<()> {
        info!("Removing ZFS volume: {}", volume_id);

        let volume_info = {
            let mut volumes = self.active_volumes.write().await;
            volumes.remove(volume_id)
        };
        if let Some(volume_info) = volume_info {
            match self
                .zfs_manager
                .dataset_manager
                .delete_dataset(&volume_info.dataset_path)
            {
                Ok(()) => {
                    info!("Successfully removed ZFS volume: {}", volume_id);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to destroy dataset for volume {}: {}", volume_id, e);
                    Err(nestgate_core::NestGateError::internal_error(
                        format!(
                            "MCP Integration: Failed to create snapshot for volume_id: {volume_id}"
                        ),
                        "mcp-integration",
                    ))
                }
            }
        } else {
            warn!("Volume not found: {}", volume_id);
            Err(nestgate_core::NestGateError::internal_error(
                format!("Volume not found: {volume_id}"),
                "mcp-integration",
            ))
        }
    }

    /// List all volumes
    pub async fn list_volumes(&self) -> Result<Vec<ZfsVolumeInfo>> {
        let volumes = self.active_volumes.read().await;
        Ok(volumes.values().cloned().collect())
    }

    /// Trigger AI optimization
    ///
    /// # Errors
    ///
    /// Returns error when AI optimization is disabled or not yet wired.
    pub fn trigger_ai_optimization(&self) -> Result<()> {
        if !self.config.enable_ai_optimization {
            return Err(nestgate_core::NestGateError::internal_error(
                "MCP integration - AI optimization is disabled".to_string(),
                "mcp-integration",
            ));
        }

        info!("AI optimization requested for MCP resources");
        Err(nestgate_core::NestGateError::not_implemented(
            "AI-driven tier optimization is not yet wired — enable_ai_optimization accepted but no optimization engine is available",
        ))
    }
}
