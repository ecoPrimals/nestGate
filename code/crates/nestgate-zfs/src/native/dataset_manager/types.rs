// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Dataset creation options and related types for [`super::NativeZfsDatasetManager`].

use nestgate_core::canonical_modernization::canonical_constants::storage;
use nestgate_core::canonical_types::StorageTier;
use serde::{Deserialize, Serialize};

/// Dataset creation options
///
/// Configuration options for creating a new ZFS dataset with specific
/// properties including compression, encryption, and storage quotas.
///
/// # Examples
///
/// ```no_run
/// use nestgate_zfs::native::dataset_manager::DatasetCreateOptions;
/// use nestgate_core::canonical_types::StorageTier;
///
/// let options = DatasetCreateOptions {
///     compression: Some("lz4".to_string()),
///     deduplication: Some(false),
///     encryption: Some("aes-256-gcm".to_string()),
///     mount_point: Some("/mnt/data".to_string()),
///     quota: Some(1_000_000_000),
///     reservation: Some(500_000_000),
///     record_size: Some("128K".to_string()),
///     storage_tier: Some(StorageTier::Hot),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Datasetcreateoptions
pub struct DatasetCreateOptions {
    /// Compression algorithm (e.g., "lz4", "gzip", "zstd")
    pub compression: Option<String>,
    /// Enable or disable deduplication
    pub deduplication: Option<bool>,
    /// Encryption algorithm and key (e.g., "aes-256-gcm")
    pub encryption: Option<String>,
    /// Custom mount point for the dataset
    pub mount_point: Option<String>,
    /// Maximum space quota in bytes
    pub quota: Option<u64>,
    /// Minimum space reservation in bytes
    pub reservation: Option<u64>,
    /// ZFS record size (e.g., "128K", "1M")
    pub record_size: Option<String>,
    /// Storage tier classification (Hot, Warm, Cold, Archive)
    pub storage_tier: Option<StorageTier>,
}

impl Default for DatasetCreateOptions {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            compression: Some(storage::COMPRESSION_LZ4.into()),
            deduplication: Some(false),
            encryption: None,
            mount_point: None,
            quota: None,
            reservation: None,
            record_size: Some(crate::constants::RECORDSIZE_128K.into()),
            storage_tier: Some(StorageTier::Hot),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset_create_options_default() {
        let options = DatasetCreateOptions::default();

        assert!(options.compression.is_some());
        assert_eq!(
            options
                .compression
                .as_ref()
                .expect("Test: compression should be Some"),
            "lz4"
        );
        assert!(options.deduplication.is_some());
        assert!(
            !options
                .deduplication
                .expect("Test: deduplication should be Some")
        );
        assert!(options.record_size.is_some());
        assert!(options.storage_tier.is_some());
    }

    #[test]
    fn test_dataset_create_options_custom() {
        let options = DatasetCreateOptions {
            compression: Some("zstd".to_string()),
            deduplication: Some(true),
            encryption: Some("aes-256-gcm".to_string()),
            mount_point: Some("/mnt/data".to_string()),
            quota: Some(1_073_741_824),     // 1GB
            reservation: Some(536_870_912), // 512MB
            record_size: Some("256K".to_string()),
            storage_tier: Some(StorageTier::Cold),
        };

        assert_eq!(
            options
                .compression
                .as_ref()
                .expect("Test: compression should be Some"),
            "zstd"
        );
        assert!(
            options
                .deduplication
                .expect("Test: deduplication should be Some")
        );
        assert_eq!(
            options
                .encryption
                .as_ref()
                .expect("Test: encryption should be Some"),
            "aes-256-gcm"
        );
        assert_eq!(
            options
                .mount_point
                .as_ref()
                .expect("Test: mount_point should be Some"),
            "/mnt/data"
        );
        assert_eq!(
            options.quota.expect("Test: quota should be Some"),
            1_073_741_824
        );
        assert_eq!(
            options
                .reservation
                .expect("Test: reservation should be Some"),
            536_870_912
        );
    }

    #[test]
    fn test_dataset_create_options_cloning() {
        let options = DatasetCreateOptions::default();
        let cloned = options.clone();

        assert_eq!(options.compression, cloned.compression);
        assert_eq!(options.deduplication, cloned.deduplication);
        assert_eq!(options.record_size, cloned.record_size);
    }

    #[test]
    fn test_dataset_create_options_serialization() {
        let options = DatasetCreateOptions {
            compression: Some("lz4".to_string()),
            deduplication: Some(false),
            encryption: None,
            mount_point: Some("/data".to_string()),
            quota: Some(2_147_483_648),
            reservation: None,
            record_size: Some("128K".to_string()),
            storage_tier: Some(StorageTier::Warm),
        };

        let json = serde_json::to_string(&options);
        assert!(json.is_ok());

        let json_str = json.expect("Test: serde_json should serialize successfully");
        let deserialized: DatasetCreateOptions = serde_json::from_str(&json_str)
            .expect("Test: serde_json should deserialize successfully");

        assert_eq!(deserialized.compression, options.compression);
        assert_eq!(deserialized.quota, options.quota);
    }

    #[test]
    fn test_dataset_create_options_none_values() {
        let options = DatasetCreateOptions {
            compression: None,
            deduplication: None,
            encryption: None,
            mount_point: None,
            quota: None,
            reservation: None,
            record_size: None,
            storage_tier: None,
        };

        assert!(options.compression.is_none());
        assert!(options.deduplication.is_none());
        assert!(options.encryption.is_none());
        assert!(options.mount_point.is_none());
        assert!(options.quota.is_none());
        assert!(options.reservation.is_none());
        assert!(options.record_size.is_none());
        assert!(options.storage_tier.is_none());
    }

    #[test]
    fn test_dataset_create_options_storage_tiers() {
        let tiers = vec![StorageTier::Hot, StorageTier::Warm, StorageTier::Cold];

        for tier in tiers {
            let options = DatasetCreateOptions {
                storage_tier: Some(tier.clone()),
                ..Default::default()
            };

            assert_eq!(
                options
                    .storage_tier
                    .expect("Test: storage_tier should be Some"),
                tier
            );
        }
    }

    #[test]
    fn test_dataset_create_options_compression_types() {
        let compressions = vec!["lz4", "zstd", "gzip", "off"];

        for comp in compressions {
            let options = DatasetCreateOptions {
                compression: Some(comp.to_string()),
                ..Default::default()
            };

            assert_eq!(
                options
                    .compression
                    .as_ref()
                    .expect("Test: compression should be Some"),
                comp
            );
        }
    }

    #[test]
    fn test_dataset_create_options_encryption_types() {
        let encryptions = vec![
            "aes-128-ccm",
            "aes-192-ccm",
            "aes-256-ccm",
            "aes-128-gcm",
            "aes-192-gcm",
            "aes-256-gcm",
        ];

        for enc in encryptions {
            let options = DatasetCreateOptions {
                encryption: Some(enc.to_string()),
                ..Default::default()
            };

            assert_eq!(
                options
                    .encryption
                    .as_ref()
                    .expect("Test: encryption should be Some"),
                enc
            );
        }
    }

    #[test]
    fn test_dataset_create_options_quota_ranges() {
        let quotas = vec![
            1_073_741_824,     // 1GB
            10_737_418_240,    // 10GB
            107_374_182_400,   // 100GB
            1_099_511_627_776, // 1TB
        ];

        for quota in quotas {
            let options = DatasetCreateOptions {
                quota: Some(quota),
                ..Default::default()
            };

            assert_eq!(options.quota.expect("Test: quota should be Some"), quota);
        }
    }

    #[test]
    fn test_dataset_create_options_record_sizes() {
        let record_sizes = vec![
            "4K", "8K", "16K", "32K", "64K", "128K", "256K", "512K", "1M",
        ];

        for size in record_sizes {
            let options = DatasetCreateOptions {
                record_size: Some(size.to_string()),
                ..Default::default()
            };

            assert_eq!(
                options
                    .record_size
                    .as_ref()
                    .expect("Test: record_size should be Some"),
                size
            );
        }
    }

    #[test]
    fn test_dataset_create_options_mount_points() {
        let mount_points = vec!["/mnt/data", "/var/lib/data", "/opt/storage", "/data/zfs"];

        for mp in mount_points {
            let options = DatasetCreateOptions {
                mount_point: Some(mp.to_string()),
                ..Default::default()
            };

            assert_eq!(
                options
                    .mount_point
                    .as_ref()
                    .expect("Test: mount_point should be Some"),
                mp
            );
        }
    }

    #[test]
    fn test_dataset_create_options_with_all_fields() {
        let options = DatasetCreateOptions {
            compression: Some("zstd".to_string()),
            deduplication: Some(true),
            encryption: Some("aes-256-gcm".to_string()),
            mount_point: Some("/mnt/secure".to_string()),
            quota: Some(5_368_709_120),       // 5GB
            reservation: Some(1_073_741_824), // 1GB
            record_size: Some("256K".to_string()),
            storage_tier: Some(StorageTier::Hot),
        };

        // Verify all fields are set
        assert!(options.compression.is_some());
        assert!(options.deduplication.is_some());
        assert!(options.encryption.is_some());
        assert!(options.mount_point.is_some());
        assert!(options.quota.is_some());
        assert!(options.reservation.is_some());
        assert!(options.record_size.is_some());
        assert!(options.storage_tier.is_some());

        // Verify specific values
        assert_eq!(
            options
                .compression
                .expect("Test: compression should be Some"),
            "zstd"
        );
        assert!(
            options
                .deduplication
                .expect("Test: deduplication should be Some")
        );
    }

    #[test]
    fn test_dataset_create_options_partial_configuration() {
        let options = DatasetCreateOptions {
            compression: Some("lz4".to_string()),
            quota: Some(2_147_483_648),
            storage_tier: Some(StorageTier::Warm),
            ..Default::default()
        };

        // Custom fields
        assert_eq!(
            options
                .compression
                .as_ref()
                .expect("Test: compression should be Some"),
            "lz4"
        );
        assert_eq!(
            options.quota.expect("Test: quota should be Some"),
            2_147_483_648
        );
        assert_eq!(
            options
                .storage_tier
                .expect("Test: storage_tier should be Some"),
            StorageTier::Warm
        );

        // Default fields
        assert!(
            !options
                .deduplication
                .expect("Test: deduplication should be Some")
        );
        assert!(options.record_size.is_some());
    }

    #[test]
    fn test_dataset_create_options_deduplication_toggle() {
        let with_dedup = DatasetCreateOptions {
            deduplication: Some(true),
            ..Default::default()
        };

        let without_dedup = DatasetCreateOptions {
            deduplication: Some(false),
            ..Default::default()
        };

        assert!(
            with_dedup
                .deduplication
                .expect("Test: deduplication should be Some")
        );
        assert!(
            !without_dedup
                .deduplication
                .expect("Test: deduplication should be Some")
        );
    }
}
