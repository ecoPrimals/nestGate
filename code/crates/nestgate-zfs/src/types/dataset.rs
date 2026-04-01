// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS dataset-related types
//!
//! Domain: Dataset information, properties, quotas, mountpoints

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

// Re-export core storage tier
pub use nestgate_core::canonical_types::StorageTier;

/// ZFS dataset information and properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    /// Short dataset name
    pub name: String,
    /// Full qualified dataset name (pool/path)
    pub full_name: String,
    /// Parent pool name
    pub pool: String,
    /// Dataset size in bytes
    pub size: u64,
    /// Space used by this dataset in bytes
    pub used: u64,
    /// Space available to this dataset in bytes
    pub available: u64,
    /// Mount point for the dataset
    pub mountpoint: Option<PathBuf>,
    /// Mount point (alternate field name for compatibility)
    pub mount_point: Option<PathBuf>,
    /// Dataset type (filesystem, volume, snapshot)
    pub dataset_type: String,
    /// Compression algorithm in use
    pub compression: String,
    /// Checksum algorithm in use
    pub checksum: String,
    /// Referenced data size in bytes
    pub referenced: u64,
    /// Compression ratio achieved
    pub compression_ratio: f64,
    /// Storage tier assignment
    pub tier: StorageTier,
    /// All ZFS properties for this dataset
    pub properties: HashMap<String, String>,
    /// Dataset creation time
    pub created_at: SystemTime,
}

impl Default for DatasetInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            full_name: String::new(),
            pool: String::new(),
            size: 0,
            used: 0,
            available: 0,
            mountpoint: None,
            mount_point: None,
            dataset_type: "filesystem".to_string(),
            compression: "lz4".to_string(),
            checksum: "sha256".to_string(),
            referenced: 0,
            compression_ratio: 1.0,
            tier: StorageTier::Warm,
            properties: HashMap::new(),
            created_at: SystemTime::now(),
        }
    }
}

/// Dataset properties for configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetProperties {
    /// Compression algorithm (lz4, gzip, zstd, etc.)
    pub compression: Option<String>,
    /// Quota limit in bytes
    pub quota: Option<u64>,
    /// Reservation in bytes
    pub reservation: Option<u64>,
    /// Record size in bytes
    pub recordsize: Option<u64>,
    /// Mount point path
    pub mountpoint: Option<PathBuf>,
    /// Whether dataset is read-only
    pub readonly: bool,
    /// Deduplication setting
    pub dedup: Option<String>,
    /// Additional custom properties
    pub custom: HashMap<String, String>,
}

/// Dataset quota configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetQuota {
    /// Dataset name
    pub dataset: String,
    /// Quota limit in bytes (None = no quota)
    pub quota: Option<u64>,
    /// Reservation in bytes (None = no reservation)
    pub reservation: Option<u64>,
    /// Current usage in bytes
    pub used: u64,
    /// Available space considering quota
    pub available: u64,
}

/// Dataset snapshot schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotSchedule {
    /// Dataset to snapshot
    pub dataset: String,
    /// Snapshot frequency (hourly, daily, weekly, monthly)
    pub frequency: String,
    /// Number of snapshots to retain
    pub retention_count: u32,
    /// Whether schedule is enabled
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== DatasetInfo Tests ====================

    #[test]
    fn test_dataset_info_default() {
        let info = DatasetInfo::default();

        assert_eq!(info.name, "");
        assert_eq!(info.full_name, "");
        assert_eq!(info.pool, "");
        assert_eq!(info.size, 0);
        assert_eq!(info.used, 0);
        assert_eq!(info.available, 0);
        assert_eq!(info.mountpoint, None);
        assert_eq!(info.mount_point, None);
        assert_eq!(info.dataset_type, "filesystem");
        assert_eq!(info.compression, "lz4");
        assert_eq!(info.checksum, "sha256");
        assert_eq!(info.referenced, 0);
        assert_eq!(info.compression_ratio, 1.0);
        assert!(matches!(info.tier, StorageTier::Warm));
        assert!(info.properties.is_empty());
    }

    #[test]
    fn test_dataset_info_creation() {
        let mut properties = HashMap::new();
        properties.insert("compression".to_string(), "zstd".to_string());

        let info = DatasetInfo {
            name: "data".to_string(),
            full_name: "tank/data".to_string(),
            pool: "tank".to_string(),
            size: 1024 * 1024 * 1024,
            used: 512 * 1024 * 1024,
            available: 512 * 1024 * 1024,
            mountpoint: Some(PathBuf::from("/mnt/tank/data")),
            mount_point: Some(PathBuf::from("/mnt/tank/data")),
            dataset_type: "filesystem".to_string(),
            compression: "zstd".to_string(),
            checksum: "sha512".to_string(),
            referenced: 500 * 1024 * 1024,
            compression_ratio: 1.5,
            tier: StorageTier::Hot,
            properties,
            created_at: SystemTime::now(),
        };

        assert_eq!(info.name, "data");
        assert_eq!(info.full_name, "tank/data");
        assert_eq!(info.pool, "tank");
        assert_eq!(info.size, 1024 * 1024 * 1024);
        assert_eq!(info.compression, "zstd");
        assert_eq!(info.compression_ratio, 1.5);
        assert!(matches!(info.tier, StorageTier::Hot));
    }

    #[test]
    fn test_dataset_info_serialization() {
        let info = DatasetInfo::default();

        let json = serde_json::to_string(&info).expect("Failed to serialize");
        let deserialized: DatasetInfo = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(info.name, deserialized.name);
        assert_eq!(info.dataset_type, deserialized.dataset_type);
        assert_eq!(info.compression, deserialized.compression);
    }

    #[test]
    fn test_dataset_info_clone() {
        let info = DatasetInfo {
            name: "test".to_string(),
            full_name: "tank/test".to_string(),
            pool: "tank".to_string(),
            ..Default::default()
        };

        let cloned = info.clone();
        assert_eq!(info.name, cloned.name);
        assert_eq!(info.full_name, cloned.full_name);
        assert_eq!(info.pool, cloned.pool);
    }

    #[test]
    fn test_dataset_info_all_storage_tiers() {
        use nestgate_core::canonical_types::StorageTier as CoreStorageTier;

        for tier in [
            CoreStorageTier::Hot,
            CoreStorageTier::Warm,
            CoreStorageTier::Cold,
            CoreStorageTier::Cache,
            CoreStorageTier::Archive,
        ] {
            let info = DatasetInfo {
                tier: tier.clone(),
                ..Default::default()
            };

            // Verify tier is stored correctly
            match tier {
                CoreStorageTier::Hot => assert!(matches!(info.tier, CoreStorageTier::Hot)),
                CoreStorageTier::Warm => assert!(matches!(info.tier, CoreStorageTier::Warm)),
                CoreStorageTier::Cold => assert!(matches!(info.tier, CoreStorageTier::Cold)),
                CoreStorageTier::Cache => assert!(matches!(info.tier, CoreStorageTier::Cache)),
                CoreStorageTier::Archive => assert!(matches!(info.tier, CoreStorageTier::Archive)),
            }
        }
    }

    #[test]
    fn test_dataset_info_with_properties() {
        let mut properties = HashMap::new();
        properties.insert("custom:priority".to_string(), "high".to_string());
        properties.insert("custom:owner".to_string(), "admin".to_string());

        let info = DatasetInfo {
            properties: properties.clone(),
            ..Default::default()
        };

        assert_eq!(info.properties.len(), 2);
        assert_eq!(
            info.properties.get("custom:priority"),
            Some(&"high".to_string())
        );
        assert_eq!(
            info.properties.get("custom:owner"),
            Some(&"admin".to_string())
        );
    }

    // ==================== DatasetProperties Tests ====================

    #[test]
    fn test_dataset_properties_basic() {
        let props = DatasetProperties {
            compression: Some("lz4".to_string()),
            quota: Some(1024 * 1024 * 1024),
            reservation: Some(512 * 1024 * 1024),
            recordsize: Some(128 * 1024),
            mountpoint: Some(PathBuf::from("/mnt/data")),
            readonly: false,
            dedup: None,
            custom: HashMap::new(),
        };

        assert_eq!(props.compression, Some("lz4".to_string()));
        assert_eq!(props.quota, Some(1024 * 1024 * 1024));
        assert_eq!(props.readonly, false);
    }

    #[test]
    fn test_dataset_properties_readonly() {
        let props = DatasetProperties {
            compression: None,
            quota: None,
            reservation: None,
            recordsize: None,
            mountpoint: None,
            readonly: true,
            dedup: None,
            custom: HashMap::new(),
        };

        assert!(props.readonly);
    }

    #[test]
    fn test_dataset_properties_with_dedup() {
        let props = DatasetProperties {
            compression: None,
            quota: None,
            reservation: None,
            recordsize: None,
            mountpoint: None,
            readonly: false,
            dedup: Some("sha256".to_string()),
            custom: HashMap::new(),
        };

        assert_eq!(props.dedup, Some("sha256".to_string()));
    }

    #[test]
    fn test_dataset_properties_custom() {
        let mut custom = HashMap::new();
        custom.insert("backup:enabled".to_string(), "true".to_string());
        custom.insert("backup:schedule".to_string(), "daily".to_string());

        let props = DatasetProperties {
            compression: None,
            quota: None,
            reservation: None,
            recordsize: None,
            mountpoint: None,
            readonly: false,
            dedup: None,
            custom: custom.clone(),
        };

        assert_eq!(props.custom.len(), 2);
        assert_eq!(
            props.custom.get("backup:enabled"),
            Some(&"true".to_string())
        );
    }

    #[test]
    fn test_dataset_properties_serialization() {
        let props = DatasetProperties {
            compression: Some("zstd".to_string()),
            quota: Some(1024 * 1024 * 1024),
            reservation: None,
            recordsize: Some(1024 * 1024),
            mountpoint: Some(PathBuf::from("/mnt/test")),
            readonly: false,
            dedup: None,
            custom: HashMap::new(),
        };

        let json = serde_json::to_string(&props).expect("Failed to serialize");
        let deserialized: DatasetProperties =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(props.compression, deserialized.compression);
        assert_eq!(props.quota, deserialized.quota);
        assert_eq!(props.readonly, deserialized.readonly);
    }

    // ==================== DatasetQuota Tests ====================

    #[test]
    fn test_dataset_quota_with_limits() {
        let quota = DatasetQuota {
            dataset: "tank/data".to_string(),
            quota: Some(1024 * 1024 * 1024),
            reservation: Some(256 * 1024 * 1024),
            used: 512 * 1024 * 1024,
            available: 512 * 1024 * 1024,
        };

        assert_eq!(quota.dataset, "tank/data");
        assert_eq!(quota.quota, Some(1024 * 1024 * 1024));
        assert_eq!(quota.used, 512 * 1024 * 1024);
        assert_eq!(quota.available, 512 * 1024 * 1024);
    }

    #[test]
    fn test_dataset_quota_no_limits() {
        let quota = DatasetQuota {
            dataset: "tank/unlimited".to_string(),
            quota: None,
            reservation: None,
            used: 100 * 1024 * 1024,
            available: 1024 * 1024 * 1024 * 1024,
        };

        assert_eq!(quota.quota, None);
        assert_eq!(quota.reservation, None);
    }

    #[test]
    fn test_dataset_quota_serialization() {
        let quota = DatasetQuota {
            dataset: "tank/test".to_string(),
            quota: Some(1024),
            reservation: Some(512),
            used: 256,
            available: 768,
        };

        let json = serde_json::to_string(&quota).expect("Failed to serialize");
        let deserialized: DatasetQuota =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(quota.dataset, deserialized.dataset);
        assert_eq!(quota.quota, deserialized.quota);
        assert_eq!(quota.used, deserialized.used);
    }

    #[test]
    fn test_dataset_quota_at_limit() {
        let quota = DatasetQuota {
            dataset: "tank/full".to_string(),
            quota: Some(1024 * 1024 * 1024),
            reservation: None,
            used: 1024 * 1024 * 1024,
            available: 0,
        };

        assert_eq!(quota.used, quota.quota.unwrap());
        assert_eq!(quota.available, 0);
    }

    // ==================== SnapshotSchedule Tests ====================

    #[test]
    fn test_snapshot_schedule_hourly() {
        let schedule = SnapshotSchedule {
            dataset: "tank/data".to_string(),
            frequency: "hourly".to_string(),
            retention_count: 24,
            enabled: true,
        };

        assert_eq!(schedule.dataset, "tank/data");
        assert_eq!(schedule.frequency, "hourly");
        assert_eq!(schedule.retention_count, 24);
        assert!(schedule.enabled);
    }

    #[test]
    fn test_snapshot_schedule_daily() {
        let schedule = SnapshotSchedule {
            dataset: "tank/important".to_string(),
            frequency: "daily".to_string(),
            retention_count: 7,
            enabled: true,
        };

        assert_eq!(schedule.frequency, "daily");
        assert_eq!(schedule.retention_count, 7);
    }

    #[test]
    fn test_snapshot_schedule_weekly() {
        let schedule = SnapshotSchedule {
            dataset: "tank/archive".to_string(),
            frequency: "weekly".to_string(),
            retention_count: 4,
            enabled: true,
        };

        assert_eq!(schedule.frequency, "weekly");
        assert_eq!(schedule.retention_count, 4);
    }

    #[test]
    fn test_snapshot_schedule_monthly() {
        let schedule = SnapshotSchedule {
            dataset: "tank/backups".to_string(),
            frequency: "monthly".to_string(),
            retention_count: 12,
            enabled: true,
        };

        assert_eq!(schedule.frequency, "monthly");
        assert_eq!(schedule.retention_count, 12);
    }

    #[test]
    fn test_snapshot_schedule_disabled() {
        let schedule = SnapshotSchedule {
            dataset: "tank/temp".to_string(),
            frequency: "daily".to_string(),
            retention_count: 1,
            enabled: false,
        };

        assert!(!schedule.enabled);
    }

    #[test]
    fn test_snapshot_schedule_serialization() {
        let schedule = SnapshotSchedule {
            dataset: "tank/test".to_string(),
            frequency: "hourly".to_string(),
            retention_count: 48,
            enabled: true,
        };

        let json = serde_json::to_string(&schedule).expect("Failed to serialize");
        let deserialized: SnapshotSchedule =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(schedule.dataset, deserialized.dataset);
        assert_eq!(schedule.frequency, deserialized.frequency);
        assert_eq!(schedule.retention_count, deserialized.retention_count);
        assert_eq!(schedule.enabled, deserialized.enabled);
    }

    #[test]
    fn test_snapshot_schedule_zero_retention() {
        let schedule = SnapshotSchedule {
            dataset: "tank/minimal".to_string(),
            frequency: "daily".to_string(),
            retention_count: 0,
            enabled: true,
        };

        assert_eq!(schedule.retention_count, 0);
    }

    #[test]
    fn test_snapshot_schedule_large_retention() {
        let schedule = SnapshotSchedule {
            dataset: "tank/forever".to_string(),
            frequency: "monthly".to_string(),
            retention_count: 120, // 10 years
            enabled: true,
        };

        assert_eq!(schedule.retention_count, 120);
    }

    #[test]
    fn test_snapshot_schedule_clone() {
        let schedule = SnapshotSchedule {
            dataset: "tank/data".to_string(),
            frequency: "daily".to_string(),
            retention_count: 7,
            enabled: true,
        };

        let cloned = schedule.clone();
        assert_eq!(schedule.dataset, cloned.dataset);
        assert_eq!(schedule.frequency, cloned.frequency);
        assert_eq!(schedule.retention_count, cloned.retention_count);
        assert_eq!(schedule.enabled, cloned.enabled);
    }
}
