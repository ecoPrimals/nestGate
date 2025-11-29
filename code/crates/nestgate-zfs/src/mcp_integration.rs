//
// This module provides integration between ZFS and MCP (Model Coordination Protocol),
// enabling ZFS to act as a storage provider for MCP systems with tiered storage
// capabilities, AI optimization, and performance monitoring.

//! Mcp Integration module

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
/// Request parameters for McpMount operation
pub struct McpMountRequest {
    /// Mount identifier
    pub mount_id: String,
    /// Mount Point
    pub mount_point: String,
    /// Tier
    pub tier: StorageTier,
    /// Size in gigabytes
    pub size_gb: u64,
}
/// MCP volume request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for McpVolume operation
pub struct McpVolumeRequest {
    /// Volume identifier
    pub volume_id: String,
    /// Tier
    pub tier: StorageTier,
    /// Size in gigabytes
    pub size_gb: u64,
}
/// Mount status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Mount
pub enum MountStatus {
    /// Active
    Active,
    /// Inactive
    Inactive,
    Error(String),
}
/// Volume status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Volume
pub enum VolumeStatus {
    /// Active
    Active,
    /// Inactive
    Inactive,
    Error(String),
}
/// ZFS mount information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfsmountinfo
pub struct ZfsMountInfo {
    /// Mount identifier
    pub mount_id: String,
    /// Dataset Path
    pub dataset_path: String,
    /// Mount Point
    pub mount_point: String,
    /// Tier
    pub tier: StorageTier,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Status
    pub status: MountStatus,
}
/// ZFS volume information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfsvolumeinfo
pub struct ZfsVolumeInfo {
    /// Volume identifier
    pub volume_id: String,
    /// Dataset Path
    pub dataset_path: String,
    /// Tier
    pub tier: StorageTier,
    /// Size Bytes
    pub size_bytes: u64,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Status
    pub status: VolumeStatus,
}
/// ZFS MCP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::ZfsMcpConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::ZfsMcpConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for ZfsMcp
pub struct ZfsMcpConfig {
    /// Enable AI optimization
    pub enable_ai_optimization: bool,
    /// Maximum concurrent operations
    pub max_concurrent_operations: u32,
    /// Default tier for new resources
    pub default_tier: StorageTier,
}
impl Default for ZfsMcpConfig {
    /// Returns the default instance
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
    pub fn validate(&self) -> Result<()> {
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
    #[must_use]
    pub fn get_tier_config(&self, tier: &StorageTier) -> TierConfig {
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
/// Configuration for Tier
pub struct TierConfig {
    /// Priority
    pub priority: u8,
    /// Cache Enabled
    pub cache_enabled: bool,
    /// Compression
    pub compression: bool,
    /// Replication
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
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn remove_mount(&self, mount_id: &str) -> Result<()> {
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
                            "MCP Integration: Failed to destroy dataset for mount {mount_id}: {e}"
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
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn remove_volume(&self, volume_id: &str) -> Result<()> {
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
                            "MCP Integration: Failed to create snapshot for volume_id: {volume_id}"
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
    pub async fn list_volumes(&self) -> Result<Vec<ZfsVolumeInfo>> {
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
    pub fn trigger_ai_optimization(&self) -> Result<()> {
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

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Zfsmcpconfigcanonical
pub type ZfsMcpConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ZfsMcpConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_mount_request_creation() {
        let request = McpMountRequest {
            mount_id: "mount-123".to_string(),
            mount_point: "/mcp/data".to_string(),
            tier: StorageTier::Hot,
            size_gb: 100,
        };

        assert_eq!(request.mount_id, "mount-123");
        assert_eq!(request.mount_point, "/mcp/data");
        assert_eq!(request.size_gb, 100);
    }

    #[test]
    fn test_mcp_volume_request_creation() {
        let request = McpVolumeRequest {
            volume_id: "vol-456".to_string(),
            tier: StorageTier::Cold,
            size_gb: 500,
        };

        assert_eq!(request.volume_id, "vol-456");
        assert_eq!(request.size_gb, 500);
    }

    #[test]
    fn test_mount_status_active() {
        let status = MountStatus::Active;
        assert!(matches!(status, MountStatus::Active));
    }

    #[test]
    fn test_mount_status_inactive() {
        let status = MountStatus::Inactive;
        assert!(matches!(status, MountStatus::Inactive));
    }

    #[test]
    fn test_mount_status_error() {
        let status = MountStatus::Error("Mount failed".to_string());
        match status {
            MountStatus::Error(msg) => assert_eq!(msg, "Mount failed"),
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn test_volume_status_variants() {
        let active = VolumeStatus::Active;
        let inactive = VolumeStatus::Inactive;
        let error = VolumeStatus::Error("Volume error".to_string());

        assert!(matches!(active, VolumeStatus::Active));
        assert!(matches!(inactive, VolumeStatus::Inactive));
        assert!(matches!(error, VolumeStatus::Error(_)));
    }

    #[test]
    fn test_zfs_mount_info_creation() {
        let info = ZfsMountInfo {
            mount_id: "mount-789".to_string(),
            dataset_path: "pool/mcp/mount-789".to_string(),
            mount_point: "/mnt/mcp-789".to_string(),
            tier: StorageTier::Warm,
            created_at: SystemTime::now(),
            status: MountStatus::Active,
        };

        assert_eq!(info.mount_id, "mount-789");
        assert_eq!(info.dataset_path, "pool/mcp/mount-789");
        assert!(matches!(info.status, MountStatus::Active));
    }

    #[test]
    fn test_zfs_volume_info_creation() {
        let info = ZfsVolumeInfo {
            volume_id: "vol-abc".to_string(),
            dataset_path: "pool/mcp/vol-abc".to_string(),
            tier: StorageTier::Hot,
            size_bytes: 1073741824, // 1GB
            created_at: SystemTime::now(),
            status: VolumeStatus::Active,
        };

        assert_eq!(info.volume_id, "vol-abc");
        assert_eq!(info.size_bytes, 1073741824);
        assert!(matches!(info.status, VolumeStatus::Active));
    }

    #[test]
    #[allow(deprecated)]
    fn test_zfs_mcp_config_default() {
        let config = ZfsMcpConfig::default();

        assert!(config.enable_ai_optimization);
        assert!(config.max_concurrent_operations > 0);
    }

    #[test]
    #[allow(deprecated)]
    fn test_zfs_mcp_config_validation() {
        let config = ZfsMcpConfig::default();
        let result = config.validate();

        assert!(result.is_ok());
    }

    #[test]
    #[allow(deprecated)]
    fn test_zfs_mcp_config_get_tier_config() {
        let config = ZfsMcpConfig::default();
        let tier_config = config.get_tier_config(&StorageTier::Hot);

        // Verify tier config has valid values
        assert!(tier_config.priority > 0);
        assert!(tier_config.replication > 0);
    }

    #[test]
    fn test_tier_config_structure() {
        let config = TierConfig {
            priority: 10,
            cache_enabled: true,
            compression: true,
            replication: 1,
        };

        assert_eq!(config.priority, 10);
        assert!(config.compression);
        assert!(config.cache_enabled);
        assert_eq!(config.replication, 1);
    }

    #[test]
    fn test_mount_request_cloning() {
        let request = McpMountRequest {
            mount_id: "clone-test".to_string(),
            mount_point: "/mcp/clone".to_string(),
            tier: StorageTier::Hot,
            size_gb: 50,
        };

        let cloned = request.clone();
        assert_eq!(cloned.mount_id, request.mount_id);
        assert_eq!(cloned.size_gb, request.size_gb);
    }

    #[test]
    fn test_volume_request_serialization() {
        let request = McpVolumeRequest {
            volume_id: "vol-serialize".to_string(),
            tier: StorageTier::Cold,
            size_gb: 250,
        };

        let json = serde_json::to_string(&request);
        assert!(json.is_ok());

        let json_str = json.unwrap();
        let deserialized: McpVolumeRequest = serde_json::from_str(&json_str).unwrap();
        assert_eq!(deserialized.volume_id, request.volume_id);
    }

    #[test]
    fn test_mount_status_serialization() {
        let statuses = vec![
            MountStatus::Active,
            MountStatus::Inactive,
            MountStatus::Error("test error".to_string()),
        ];

        for status in statuses {
            let json = serde_json::to_string(&status);
            assert!(json.is_ok());
        }
    }

    #[test]
    fn test_mount_info_with_different_tiers() {
        let tiers = vec![StorageTier::Hot, StorageTier::Warm, StorageTier::Cold];

        for (i, tier) in tiers.iter().enumerate() {
            let info = ZfsMountInfo {
                mount_id: format!("mount-{}", i),
                dataset_path: format!("pool/mount-{}", i),
                mount_point: format!("/mnt/{}", i),
                tier: tier.clone(),
                created_at: SystemTime::now(),
                status: MountStatus::Active,
            };

            assert_eq!(info.tier, *tier);
        }
    }

    #[test]
    fn test_volume_info_size_calculations() {
        let sizes = vec![
            (1, 1_073_741_824),     // 1GB
            (10, 10_737_418_240),   // 10GB
            (100, 107_374_182_400), // 100GB
        ];

        for (gb, bytes) in sizes {
            let info = ZfsVolumeInfo {
                volume_id: format!("vol-{}", gb),
                dataset_path: format!("pool/vol-{}", gb),
                tier: StorageTier::Hot,
                size_bytes: bytes,
                created_at: SystemTime::now(),
                status: VolumeStatus::Active,
            };

            assert_eq!(info.size_bytes, bytes);
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_config_tier_mappings() {
        let config = ZfsMcpConfig::default();

        // Test all tier types exist and return valid configs
        let hot_config = config.get_tier_config(&StorageTier::Hot);
        let warm_config = config.get_tier_config(&StorageTier::Warm);
        let cold_config = config.get_tier_config(&StorageTier::Cold);

        // All should have valid replication factors
        assert!(hot_config.replication > 0);
        assert!(warm_config.replication > 0);
        assert!(cold_config.replication > 0);

        // All should have positive priorities
        assert!(hot_config.priority > 0);
        assert!(warm_config.priority > 0);
        assert!(cold_config.priority > 0);
    }

    #[test]
    fn test_mount_status_error_messages() {
        let errors = vec![
            "Permission denied",
            "Mount point not found",
            "Insufficient space",
        ];

        for error_msg in errors {
            let status = MountStatus::Error(error_msg.to_string());
            match status {
                MountStatus::Error(msg) => assert_eq!(msg, error_msg),
                _ => panic!("Expected Error variant"),
            }
        }
    }

    #[test]
    fn test_volume_status_error_handling() {
        let status = VolumeStatus::Error("Volume creation failed".to_string());

        let json = serde_json::to_string(&status).unwrap();
        let deserialized: VolumeStatus = serde_json::from_str(&json).unwrap();

        match deserialized {
            VolumeStatus::Error(msg) => assert!(msg.contains("failed")),
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_config_customization() {
        let config = ZfsMcpConfig {
            enable_ai_optimization: false,
            max_concurrent_operations: 20,
            ..Default::default()
        };

        assert!(!config.enable_ai_optimization);
        assert_eq!(config.max_concurrent_operations, 20);
    }

    #[test]
    fn test_tier_config_compression_options() {
        let configs = vec![
            TierConfig {
                priority: 10,
                cache_enabled: true,
                compression: true,
                replication: 3,
            },
            TierConfig {
                priority: 5,
                cache_enabled: true,
                compression: true,
                replication: 2,
            },
            TierConfig {
                priority: 1,
                cache_enabled: false,
                compression: false,
                replication: 1,
            },
        ];

        for config in configs {
            assert!(config.priority > 0);
            assert!(config.replication > 0);
        }
    }
}
