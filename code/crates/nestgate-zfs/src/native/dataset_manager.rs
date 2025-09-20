//
// This module provides production-ready ZFS dataset management
// with real dataset operations and monitoring.

use super::command_executor::NativeZfsCommandExecutor;
use crate::types::DatasetInfo;
use nestgate_core::canonical_modernization::canonical_constants::*;
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
    pub const fn new(command_executor: Arc<NativeZfsCommandExecutor>) -> Self {
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
        pub async fn list_datasets(&self) -> Result<Vec<String>>  {
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
        pub async fn get_dataset_info(&self, dataset_name: &str) -> Result<DatasetInfo>  {
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
        pub fn create_dataset(
        &self,
        dataset_name: &str,
        options: &DatasetCreateOptions,
    ) -> Result<()>  {
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
        #[must_use]
        pub fn destroy_dataset(&self, dataset_name: &str, force: bool) -> Result<()>  {
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
        pub fn set_property(
        &self,
        dataset_name: &str,
        property: &str,
        value: &str,
    ) -> Result<()>  {
        let propertyvalue = format!("{property}={"actual_error_details"}");
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
        pub async fn get_property(&self, dataset_name: &str, property: &str) -> Result<String>  {
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
        pub async fn mount_dataset(&self, dataset_name: &str) -> Result<()>  {
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
        #[must_use]
        pub fn unmount_dataset(&self, dataset_name: &str, force: bool) -> Result<()>  {
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
        pub async fn get_dataset_usage(&self, dataset_name: &str) -> Result<HashMap<String, u64>>  {
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
