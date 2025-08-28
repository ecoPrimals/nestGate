//
// Storage management for MCP integration

use crate::{
    types::{MountInfo, MountRequest, MountStatus, NfsVersion, StorageProtocol, StorageTier},
    Result,
};
use nestgate_core::biomeos::{BiomeContext, VolumeSpec};
use nestgate_core::error::NestGateError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tracing::debug;
use tracing::info;

/// Storage volume configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeConfig {
    pub name: String,
    pub size_bytes: u64,
    pub tier: StorageTier,
    pub mount_point: String,
    pub filesystem: String,
    pub options: HashMap<String, String>,
}

/// Storage volume information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeInfo {
    pub name: String,
    pub size_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub tier: StorageTier,
    pub mount_point: String,
    pub filesystem: String,
    pub mounted: bool,
    pub health: String,
}

/// biomeOS storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeStorageStats {
    pub biome_id: String,
    pub volume_count: usize,
    pub total_size_bytes: u64,
    pub total_used_bytes: u64,
    pub total_available_bytes: u64,
    pub volumes: Vec<VolumeInfo>,
}

/// MCP Storage Manager
#[derive(Debug, Clone)]
pub struct McpStorageManager {
    volumes: Arc<RwLock<HashMap<String, VolumeInfo>>>,
}

impl Default for McpStorageManager {
    fn default() -> Self {
        Self::new()
    }
}

impl McpStorageManager {
    /// Create a new storage manager
    pub fn new() -> Self {
        info!("Initializing MCP storage manager");

        Self {
            volumes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initialize storage subsystem
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing MCP storage subsystem");

        // Discover existing volumes
        self.discover_volumes().await?;

        info!("MCP storage subsystem initialized");
        Ok(())
    }

    /// Create a new storage volume
    pub async fn create_volume(&self, config: VolumeConfig) -> Result<VolumeInfo> {
        info!("Creating storage volume: {}", config.name);

        // Validate configuration
        if config.name.is_empty() {
            return Err(NestGateError::Validation {
                field: Some("volume_name".to_string()),
                message: "Volume name cannot be empty".to_string(),
                current_value: Some("".to_string()),
                expected: Some("non-empty string".to_string()),
                user_error: true,
                context: None,
            });
        }

        if config.size_bytes == 0 {
            return Err(NestGateError::Validation {
                field: Some("volume_size".to_string()),
                message: "Volume size must be greater than zero".to_string(),
                current_value: Some("0".to_string()),
                expected: Some("> 0".to_string()),
                user_error: true,
                context: None,
            });
        }

        // Check if volume already exists
        let volumes = self.volumes.read().await;
        if volumes.contains_key(&config.name) {
            return Err(nestgate_core::NestGateError::Storage {
                operation: "create_volume".to_string(),
                details: format!("Volume {} already exists", config.name),
            });
        }
        drop(volumes);

        // Create volume info
        let volume_info = VolumeInfo {
            name: config.name.clone(),
            size_bytes: config.size_bytes,
            used_bytes: 0,
            available_bytes: config.size_bytes,
            tier: config.tier,
            mount_point: config.mount_point.clone(),
            filesystem: config.filesystem.clone(),
            mounted: false,
            health: "Healthy".to_string(),
        };

        // Store volume info
        let mut volumes = self.volumes.write().await;
        volumes.insert(config.name.clone(), volume_info.clone());

        info!("Storage volume created successfully: {}", config.name);
        Ok(volume_info)
    }

    /// List all storage volumes
    pub async fn list_volumes(&self) -> Result<Vec<VolumeInfo>> {
        debug!("Listing storage volumes");

        let volumes = self.volumes.read().await;
        let volume_list: Vec<VolumeInfo> = volumes.values().cloned().collect();

        Ok(volume_list)
    }

    /// Get volume information
    pub async fn get_volume(&self, name: &str) -> Result<VolumeInfo> {
        debug!("Getting volume info: {}", name);

        let volumes = self.volumes.read().await;
        volumes
            .get(name)
            .cloned()
            .ok_or_else(|| nestgate_core::NestGateError::Storage {
                operation: "storage_operation".to_string(),
                details: "Storage operation failed".to_string(),
            })
    }

    /// Mount a storage volume
    pub async fn mount_volume(&self, name: &str) -> Result<()> {
        info!("Mounting storage volume: {}", name);

        let mut volumes = self.volumes.write().await;
        let volume =
            volumes
                .get_mut(name)
                .ok_or_else(|| nestgate_core::NestGateError::Storage {
                    operation: "mount_volume".to_string(),
                    details: format!("Volume not found: {name}"),
                })?;

        if volume.mounted {
            return Err(nestgate_core::NestGateError::Storage {
                operation: "storage_operation".to_string(),
                details: "Storage operation failed".to_string(),
            });
        }

        // Simulate mount operation
        volume.mounted = true;

        info!("Storage volume mounted successfully: {}", name);
        Ok(())
    }

    /// Unmount a storage volume
    pub async fn unmount_volume(&self, name: &str) -> Result<()> {
        info!("Unmounting storage volume: {}", name);

        let mut volumes = self.volumes.write().await;
        let volume =
            volumes
                .get_mut(name)
                .ok_or_else(|| nestgate_core::NestGateError::Storage {
                    operation: "unmount_volume".to_string(),
                    details: format!("Volume not found: {name}"),
                })?;

        if !volume.mounted {
            return Err(nestgate_core::NestGateError::Storage {
                operation: "unmount_volume".to_string(),
                details: format!("Volume {name} is not mounted"),
            });
        }

        // Simulate unmount operation
        volume.mounted = false;

        info!("Storage volume unmounted successfully: {}", name);
        Ok(())
    }

    /// Delete a storage volume
    pub async fn delete_volume(&self, name: &str) -> Result<()> {
        info!("Deleting storage volume: {}", name);

        let mut volumes = self.volumes.write().await;
        let volume = volumes
            .get(name)
            .ok_or_else(|| nestgate_core::NestGateError::Storage {
                operation: "delete_volume".to_string(),
                details: format!("Volume not found: {name}"),
            })?;

        if volume.mounted {
            return Err(NestGateError::Io {
                operation: "volume deletion".to_string(),
                error_message: format!("Cannot delete mounted volume: {name}"),
                resource: Some(name.to_string()),
                retryable: false,
                context: None,
            });
        }

        volumes.remove(name);

        info!("Storage volume deleted successfully: {}", name);
        Ok(())
    }

    /// Discover existing volumes
    async fn discover_volumes(&self) -> Result<()> {
        info!("Discovering existing storage volumes");

        // In a real implementation, this would scan for existing volumes
        // For now, we'll start with an empty volume set
        Ok(())
    }

    /// Provision volume from biomeOS manifest
    /// This is the MCP-level implementation of biomeOS volume provisioning
    pub async fn provision_from_biomeos_manifest(
        &self,
        volume_spec: &VolumeSpec,
        biome_context: &BiomeContext,
    ) -> Result<VolumeInfo> {
        info!(
            "Provisioning volume from biome manifest: {} for biome {}",
            volume_spec.name, biome_context.biome_id
        );

        // Parse size string to bytes
        let size_bytes =
            volume_spec
                .size_bytes()
                .map_err(|e| nestgate_core::NestGateError::Validation {
                    field: Some("volume_size".to_string()),
                    message: format!("Invalid volume size: {e}"),
                    current_value: None,
                    expected: Some("Valid size format (e.g., 100GB, 1TB)".to_string()),
                    user_error: true,
                context: None,
                })?;

        // Convert biomeOS tier to MCP tier
        let mcp_tier = match volume_spec.tier.to_lowercase().as_str() {
            "hot" => StorageTier::Hot,
            "warm" => StorageTier::Warm,
            "cold" => StorageTier::Cold,
            "cache" => StorageTier::Hot, // Map cache to hot tier
            _ => {
                return Err(NestGateError::Validation {
                    field: Some("storage_tier".to_string()),
                    message: format!("Unknown storage tier: {}", volume_spec.tier),
                    current_value: Some(volume_spec.tier.clone()),
                    expected: Some("hot, warm, cold, or cache".to_string()),
                    user_error: true,
                context: None,
                });
            }
        };

        // Determine mount point
        let mount_point = volume_spec
            .mount_path
            .clone()
            .unwrap_or_else(|| format!("/biomeos/{}/{}", biome_context.biome_id, volume_spec.name));

        // Create volume configuration
        let config = VolumeConfig {
            name: format!("biomeos-{}-{}", biome_context.biome_id, volume_spec.name),
            size_bytes,
            tier: mcp_tier,
            mount_point,
            filesystem: "zfs".to_string(),
            options: {
                let mut options = HashMap::new();
                options.insert("biome_id".to_string(), biome_context.biome_id.clone());
                options.insert("node_id".to_string(), biome_context.node_id.clone());
                options.insert("provisioner".to_string(), volume_spec.provisioner.clone());
                options.insert("environment".to_string(), biome_context.environment.clone());

                // Add any additional options from volume spec
                if let Some(spec_options) = &volume_spec.options {
                    for (key, value) in spec_options {
                        options.insert(key.clone(), value.clone());
                    }
                }

                options
            },
        };

        // Create the volume using existing infrastructure
        self.create_volume(config).await
    }

    /// List biomeOS volumes only
    pub async fn list_biomeos_volumes(&self) -> Result<Vec<VolumeInfo>> {
        debug!("Listing biomeOS volumes");

        let volumes = self.volumes.read().await;
        let biomeos_volumes: Vec<VolumeInfo> = volumes
            .values()
            .filter(|v| v.name.starts_with("biomeos-"))
            .cloned()
            .collect();

        Ok(biomeos_volumes)
    }

    /// Get biomeOS volume by biome ID and volume name
    pub async fn get_biomeos_volume(
        &self,
        biome_id: &str,
        volume_name: &str,
    ) -> Result<VolumeInfo> {
        let volume_key = format!("biomeos-{biome_id}-{volume_name}");
        self.get_volume(&volume_key).await
    }

    /// Delete biomeOS volume by biome ID and volume name
    pub async fn delete_biomeos_volume(&self, biome_id: &str, volume_name: &str) -> Result<()> {
        let volume_key = format!("biomeos-{biome_id}-{volume_name}");
        self.delete_volume(&volume_key).await
    }

    /// Get volume usage statistics for biome
    pub async fn get_biome_storage_stats(&self, biome_id: &str) -> Result<BiomeStorageStats> {
        debug!("Getting storage stats for biome: {}", biome_id);

        let volumes = self.volumes.read().await;
        let biome_volumes: Vec<&VolumeInfo> = volumes
            .values()
            .filter(|v| v.name.starts_with(&format!("biomeos-{biome_id}-")))
            .collect();

        let total_size = biome_volumes.iter().map(|v| v.size_bytes).sum();
        let total_used = biome_volumes.iter().map(|v| v.used_bytes).sum();
        let total_available = biome_volumes.iter().map(|v| v.available_bytes).sum();

        Ok(BiomeStorageStats {
            biome_id: biome_id.to_string(),
            volume_count: biome_volumes.len(),
            total_size_bytes: total_size,
            total_used_bytes: total_used,
            total_available_bytes: total_available,
            volumes: biome_volumes.into_iter().cloned().collect(),
        })
    }

    /// Resize volume for biomeOS
    pub async fn resize_biomeos_volume(
        &self,
        biome_id: &str,
        volume_name: &str,
        new_size_bytes: u64,
    ) -> Result<VolumeInfo> {
        info!(
            "Resizing biomeOS volume {}/{} to {} bytes",
            biome_id, volume_name, new_size_bytes
        );

        let volume_key = format!("biomeos-{biome_id}-{volume_name}");
        let mut volumes = self.volumes.write().await;

        let volume =
            volumes
                .get_mut(&volume_key)
                .ok_or_else(|| nestgate_core::NestGateError::Storage {
                    operation: "storage_operation".to_string(),
                    details: "Storage operation failed".to_string(),
                })?;

        if new_size_bytes < volume.used_bytes {
            return Err(NestGateError::Io {
                operation: "volume resize".to_string(),
                error_message: format!(
                    "Cannot shrink volume below used space: {} < {}",
                    new_size_bytes, volume.used_bytes
                ),
                resource: Some("volume".to_string()),
                retryable: false,
                context: None,
            });
        }

        volume.size_bytes = new_size_bytes;
        volume.available_bytes = new_size_bytes - volume.used_bytes;

        info!("Successfully resized volume: {}", volume_key);
        Ok(volume.clone())
    }
}

/// Storage adapter for orchestrator integration
#[derive(Debug, Clone)]
pub struct StorageAdapter {
    _manager: McpStorageManager,
}

impl Default for StorageAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl StorageAdapter {
    /// Create a new storage adapter
    pub fn new() -> Self {
        Self {
            _manager: McpStorageManager::new(),
        }
    }

    /// Mount a volume
    pub async fn mount_volume(&self, request: &MountRequest) -> Result<MountInfo> {
        // Implement volume mounting for MCP storage adapter
        tracing::info!(
            "Mounting volume: {} to {}",
            request.volume_id,
            request.mount_path
        );
        Ok(MountInfo {
            volume_id: request.volume_id.clone(),
            mount_path: request.mount_path.clone(),
            protocol: StorageProtocol::Nfs(NfsVersion::V4), // Default protocol
            options: request.options.clone(),
            status: MountStatus::Mounted,
            mounted_at: SystemTime::now(),
        })
    }
}
