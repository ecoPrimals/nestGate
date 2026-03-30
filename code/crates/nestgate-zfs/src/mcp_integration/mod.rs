// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// This module provides integration between ZFS and MCP (Model Coordination Protocol),
// enabling ZFS to act as a storage provider for MCP systems with tiered storage
// capabilities, AI optimization, and performance monitoring.

//! Mcp Integration module — types, provider, and protocol-facing configuration aliases.

mod provider;
mod types;

pub use provider::ZfsMcpStorageProvider;
pub use types::{
    McpMountRequest, McpVolumeRequest, MountStatus, TierConfig, VolumeStatus, ZfsMcpConfig,
    ZfsMountInfo, ZfsVolumeInfo,
};

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type ZfsMcpConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_mcp_mount_request_creation() {
        let request = McpMountRequest {
            mount_id: "mount-123".to_string(),
            mount_point: "/mcp/data".to_string(),
            tier: crate::types::StorageTier::Hot,
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
            tier: crate::types::StorageTier::Cold,
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
            tier: crate::types::StorageTier::Warm,
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
            tier: crate::types::StorageTier::Hot,
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
        let tier_config = config.get_tier_config(&crate::types::StorageTier::Hot);

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
            tier: crate::types::StorageTier::Hot,
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
            tier: crate::types::StorageTier::Cold,
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
        let tiers = vec![
            crate::types::StorageTier::Hot,
            crate::types::StorageTier::Warm,
            crate::types::StorageTier::Cold,
        ];

        for (i, tier) in tiers.iter().enumerate() {
            let info = ZfsMountInfo {
                mount_id: format!("mount-{i}"),
                dataset_path: format!("pool/mount-{i}"),
                mount_point: format!("/mnt/{i}"),
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
                volume_id: format!("vol-{gb}"),
                dataset_path: format!("pool/vol-{gb}"),
                tier: crate::types::StorageTier::Hot,
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
        let hot_config = config.get_tier_config(&crate::types::StorageTier::Hot);
        let warm_config = config.get_tier_config(&crate::types::StorageTier::Warm);
        let cold_config = config.get_tier_config(&crate::types::StorageTier::Cold);

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

    #[test]
    #[allow(deprecated)]
    fn zfs_mcp_config_validate_rejects_zero_concurrency() {
        let config = ZfsMcpConfig {
            max_concurrent_operations: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    #[allow(deprecated)]
    fn zfs_mcp_config_validate_rejects_too_high_concurrency() {
        let config = ZfsMcpConfig {
            max_concurrent_operations: 1001,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    #[allow(deprecated)]
    fn get_tier_config_all_tiers() {
        let config = ZfsMcpConfig::default();
        for tier in [
            crate::types::StorageTier::Hot,
            crate::types::StorageTier::Warm,
            crate::types::StorageTier::Cold,
            crate::types::StorageTier::Cache,
            crate::types::StorageTier::Archive,
        ] {
            let tc = config.get_tier_config(&tier);
            assert!(tc.replication > 0);
        }
    }
}
