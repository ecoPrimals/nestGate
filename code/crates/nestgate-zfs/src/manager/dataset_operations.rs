//
// Contains all dataset-related operations including creation, destruction,
// and snapshot management.

use crate::error::{create_zfs_error, ZfsOperation};
use crate::types::StorageTier;
use nestgate_core::Result;
// Removed unused tracing import

use super::ZfsManager;
use tracing::info;

/// Dataset analyzer for ZFS operations
#[derive(Debug, Clone)]
pub struct DatasetAnalyzer {
    pub config: std::collections::HashMap<String, String>,
}
impl DatasetAnalyzer {
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: std::collections::HashMap::new(),
        }
    }
}

impl Default for DatasetAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ZfsManager {
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
        name: &str,
        parent: &str,
        tier: StorageTier,
    ) -> Result<crate::dataset::DatasetInfo> {
        info!(
            "Creating dataset: {} in parent: {} on tier: {:?}",
            name, parent, tier
        );

        let result = self
            .dataset_manager
            .create_dataset(name, parent, tier)
            .await
            .map_err(|_e| {
                create_zfs_error(
                    format!("Failed to create dataset: {"actual_error_details"}"),
                    ZfsOperation::DatasetCreate,
                )
            })?;

        Ok(result)
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
    pub fn destroy_dataset(&self, name: &str) -> Result<()> {
        info!("Destroying dataset: {}", name);

        self.dataset_manager
            .destroy_dataset(name)
            .await
            .map_err(|_e| {
                create_zfs_error(
                    format!("Failed to destroy dataset: {"actual_error_details"}"),
                    ZfsOperation::DatasetCreate,
                )
            })?;

        Ok(())
    }

    /// List snapshots for a dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn list_snapshots(
        &self,
        dataset: &str,
    ) -> Result<Vec<crate::snapshot::SnapshotInfo>> {
        self.snapshot_manager
            .list_snapshots(dataset)
            .await
            .map_err(|_e| {
                create_zfs_error(
                    format!("Failed to list snapshots: {"actual_error_details"}"),
                    ZfsOperation::DatasetCreate,
                )
            })
    }
}
