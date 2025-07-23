//! ZFS Manager Dataset Operations - ZFS dataset management operations
//!
//! Contains all dataset-related operations including creation, destruction,
//! and snapshot management.

use crate::error::{Result, ZfsError};
use nestgate_core::StorageTier;
// Removed unused tracing import

use super::ZfsManager;
use tracing::info;

impl ZfsManager {
    /// Create a new dataset
    pub async fn create_dataset(
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
            .map_err(|e| ZfsError::Internal {
                message: format!("Failed to create dataset: {e}"),
            })?;

        Ok(result)
    }

    /// Destroy a dataset
    pub async fn destroy_dataset(&self, name: &str) -> Result<()> {
        info!("Destroying dataset: {}", name);

        self.dataset_manager
            .destroy_dataset(name)
            .await
            .map_err(|e| ZfsError::Internal {
                message: format!("Failed to destroy dataset: {e}"),
            })?;

        Ok(())
    }

    /// List snapshots for a dataset
    pub async fn list_snapshots(
        &self,
        dataset: &str,
    ) -> Result<Vec<crate::snapshot::SnapshotInfo>> {
        self.snapshot_manager
            .list_snapshots(dataset)
            .await
            .map_err(|e| ZfsError::Internal {
                message: format!("Failed to list snapshots: {e}"),
            })
    }
}
