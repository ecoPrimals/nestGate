// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Contains all dataset-related operations including creation, destruction,
// and snapshot management.

//! Dataset Operations module

use crate::error::{ZfsOperation, create_zfs_error};
use crate::types::StorageTier;
use nestgate_core::Result;
// Removed unused tracing import

use super::ZfsManager;
use tracing::info;

/// Dataset analyzer for ZFS operations
#[derive(Debug, Clone)]
/// Datasetanalyzer
pub struct DatasetAnalyzer {
    /// Configuration for
    pub config: std::collections::HashMap<String, String>,
}
impl DatasetAnalyzer {
    /// Creates a new dataset analyzer with empty configuration.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: std::collections::HashMap::new(),
        }
    }
}

impl Default for DatasetAnalyzer {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset_analyzer_new() {
        let analyzer = DatasetAnalyzer::new();
        assert!(analyzer.config.is_empty());
    }

    #[test]
    fn test_dataset_analyzer_default() {
        let analyzer = DatasetAnalyzer::default();
        assert!(analyzer.config.is_empty());
    }

    #[test]
    fn test_dataset_analyzer_with_config() {
        let mut analyzer = DatasetAnalyzer::new();
        analyzer
            .config
            .insert("key1".to_string(), "value1".to_string());
        analyzer
            .config
            .insert("key2".to_string(), "value2".to_string());

        assert_eq!(analyzer.config.len(), 2);
        assert_eq!(analyzer.config.get("key1"), Some(&"value1".to_string()));
        assert_eq!(analyzer.config.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_dataset_analyzer_clone() {
        let mut analyzer = DatasetAnalyzer::new();
        analyzer
            .config
            .insert("test".to_string(), "value".to_string());

        let cloned = analyzer.clone();
        assert_eq!(analyzer.config, cloned.config);
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
            .map_err(|_e| {
                create_zfs_error(
                    "Failed to create dataset: error details".to_string(),
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
    pub async fn destroy_dataset(&self, name: &str) -> Result<()> {
        info!("Destroying dataset: {}", name);

        self.dataset_manager
            .destroy_dataset(name)
            .await
            .map_err(|_e| {
                create_zfs_error(
                    "Failed to destroy dataset: error details".to_string(),
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
                    "Failed to list snapshots: error details".to_string(),
                    ZfsOperation::DatasetCreate,
                )
            })
    }
}
