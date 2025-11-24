//
// This module provides production-ready ZFS dataset management
// with real dataset operations and monitoring.

use super::command_executor::NativeZfsCommandExecutor;
use crate::types::DatasetInfo;
use nestgate_core::canonical_modernization::canonical_constants::storage;
use nestgate_core::canonical_types::StorageTier;
use nestgate_core::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

/// Native ZFS dataset manager
pub struct NativeZfsDatasetManager {
    command_executor: Arc<NativeZfsCommandExecutor>,
}
/// Dataset creation options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetCreateOptions {
    pub compression: Option<String>,
    pub deduplication: Option<bool>,
    pub encryption: Option<String>,
    pub mount_point: Option<String>,
    pub quota: Option<u64>,
    pub reservation: Option<u64>,
    pub record_size: Option<String>,
    pub storage_tier: Option<StorageTier>,
}
impl Default for DatasetCreateOptions {
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

impl NativeZfsDatasetManager {
    /// Create a new dataset manager
    #[must_use]
    pub fn new(command_executor: Arc<NativeZfsCommandExecutor>) -> Self {
        Self { command_executor }
    }

    /// List all datasets
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn list_datasets(&self) -> Result<Vec<String>> {
        let output = self
            .command_executor
            .execute_command_expect_success(&["list", "-H", "-o", "name", "-t", "filesystem"])
            .await?;

        Ok(output
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim().to_string())
            .collect())
    }

    /// Get dataset information
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_dataset_info(&self, dataset_name: &str) -> Result<DatasetInfo> {
        let properties = self.command_executor.get_dataset_info(dataset_name).await?;

        let used_bytes = properties
            .get("used")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let available_bytes = properties
            .get("available")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let mount_point = properties
            .get("mountpoint")
            .cloned()
            .unwrap_or_else(|| "none".into());

        let compression_ratio = properties
            .get("compressratio")
            .and_then(|s| s.trim_end_matches('x').parse::<f64>().ok());

        Ok(DatasetInfo {
            name: dataset_name.to_string(),
            full_name: dataset_name.to_string(),
            pool: dataset_name.split('/').next().unwrap_or("").to_string(),
            size: used_bytes + available_bytes,
            used: used_bytes,
            available: available_bytes,
            compression: compression_ratio.map_or_else(|| "lz4".into(), |r| r.to_string()),
            checksum: "sha256".into(), // Default checksum
            tier: nestgate_core::canonical_types::StorageTier::Warm, // Default tier
            mount_point: if mount_point.is_empty() || mount_point == "none" {
                None
            } else {
                Some(std::path::PathBuf::from(mount_point))
            },
            properties: properties.clone(),
            created_at: std::time::SystemTime::now(),
        })
    }

    /// Create a new dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_dataset(
        &self,
        dataset_name: &str,
        options: &DatasetCreateOptions,
    ) -> Result<()> {
        let mut properties = HashMap::new();

        // Set compression
        if let Some(compression) = &options.compression {
            properties.insert("compression".to_string(), compression.clone());
        }

        // Set deduplication
        if let Some(dedup) = options.deduplication {
            properties.insert(
                "dedup".to_string(),
                if dedup {
                    "on".to_string()
                } else {
                    "off".to_string()
                },
            );
        }

        // Set encryption
        if let Some(encryption) = &options.encryption {
            properties.insert("encryption".to_string(), encryption.clone());
        }

        // Set mount point
        if let Some(mount_point) = &options.mount_point {
            properties.insert("mountpoint".to_string(), mount_point.clone());
        }

        // Set quota
        if let Some(quota) = options.quota {
            properties.insert("quota".to_string(), quota.to_string());
        }

        // Set reservation
        if let Some(reservation) = options.reservation {
            properties.insert("reservation".to_string(), reservation.to_string());
        }

        // Set record size
        if let Some(record_size) = &options.record_size {
            properties.insert("recordsize".to_string(), record_size.clone());
        }

        // Create the dataset
        self.command_executor
            .create_dataset(dataset_name, &properties)
            .await?;

        info!("✅ Created ZFS dataset: {}", dataset_name);
        Ok(())
    }

    /// Destroy a dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn destroy_dataset(&self, dataset_name: &str, force: bool) -> Result<()> {
        let mut args = vec!["destroy"];
        if force {
            args.push("-f");
        }
        args.push(dataset_name);

        self.command_executor
            .execute_command_expect_success(&args)
            .await?;

        info!("✅ Destroyed ZFS dataset: {}", dataset_name);
        Ok(())
    }

    /// Set dataset property
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn set_property(
        &self,
        dataset_name: &str,
        property: &str,
        value: &str,
    ) -> Result<()> {
        let propertyvalue = format!("{property}={value}");
        self.command_executor
            .execute_command_expect_success(&["set", &propertyvalue, dataset_name])
            .await?;

        info!(
            "✅ Set property {}={} on dataset {}",
            property, value, dataset_name
        );
        Ok(())
    }

    /// Get dataset property
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_property(&self, dataset_name: &str, property: &str) -> Result<String> {
        let output = self
            .command_executor
            .execute_command_expect_success(&["get", "-H", "-o", "value", property, dataset_name])
            .await?;

        Ok(output.trim().to_string())
    }

    /// Mount a dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn mount_dataset(&self, dataset_name: &str) -> Result<()> {
        self.command_executor
            .execute_command_expect_success(&["mount", dataset_name])
            .await?;

        info!("✅ Mounted ZFS dataset: {}", dataset_name);
        Ok(())
    }

    /// Unmount a dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn unmount_dataset(&self, dataset_name: &str, force: bool) -> Result<()> {
        let mut args = vec!["unmount"];
        if force {
            args.push("-f");
        }
        args.push(dataset_name);

        self.command_executor
            .execute_command_expect_success(&args)
            .await?;

        info!("✅ Unmounted ZFS dataset: {}", dataset_name);
        Ok(())
    }

    /// Get dataset usage statistics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_dataset_usage(&self, dataset_name: &str) -> Result<HashMap<String, u64>> {
        let properties = self.command_executor.get_dataset_info(dataset_name).await?;

        let mut usage = HashMap::new();

        // Parse numeric properties
        for (key, value) in properties {
            if let Ok(bytes) = value.parse::<u64>() {
                match key.as_str() {
                    "used" | "available" | "referenced" | "compressratio" | "written" => {
                        usage.insert(key, bytes);
                    }
                    _ => {}
                }
            }
        }

        Ok(usage)
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
        assert_eq!(
            options
                .deduplication
                .expect("Test: deduplication should be Some"),
            false
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
        assert_eq!(
            options
                .deduplication
                .expect("Test: deduplication should be Some"),
            true
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
        assert_eq!(
            options
                .deduplication
                .expect("Test: deduplication should be Some"),
            true
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
        assert_eq!(
            options
                .deduplication
                .expect("Test: deduplication should be Some"),
            false
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

        assert_eq!(
            with_dedup
                .deduplication
                .expect("Test: deduplication should be Some"),
            true
        );
        assert_eq!(
            without_dedup
                .deduplication
                .expect("Test: deduplication should be Some"),
            false
        );
    }
}
