// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Contains all pool-related operations including creation, destruction,
// status checking, and maintenance operations like scrubbing.

//! Pool Operations module

use crate::error::{create_zfs_error, ZfsOperation};
use nestgate_core::Result;
// Removed unused tracing import

use super::ZfsManager;
use tracing::info;

impl ZfsManager {
    /// Create a new ZFS pool
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_pool(
        &self,
        name: &str,
        devices: &[String],
    ) -> Result<crate::pool::PoolInfo> {
        info!("Creating ZFS pool: {}", name);

        let result = self
            .pool_manager
            .create_pool(name, devices)
            .await
            .map_err(|_e| {
                create_zfs_error(
                    "Failed to create pool: error details".to_string(),
                    ZfsOperation::PoolCreate,
                )
            })?;

        Ok(result)
    }

    /// Destroy a ZFS pool
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn destroy_pool(&self, name: &str) -> Result<()> {
        info!("Destroying ZFS pool: {}", name);

        self.pool_manager.destroy_pool(name).await.map_err(|_e| {
            create_zfs_error(
                "Failed to destroy pool: error details".to_string(),
                ZfsOperation::PoolCreate,
            )
        })?;

        Ok(())
    }

    /// Get pool status information
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_pool_status(&self, name: &str) -> Result<String> {
        self.pool_manager.get_pool_status(name).await.map_err(|_e| {
            create_zfs_error(
                "Failed to get pool status: error details".to_string(),
                ZfsOperation::PoolCreate,
            )
        })
    }

    /// Initiate pool scrub
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn scrub_pool(&self, name: &str) -> Result<()> {
        info!("Starting scrub for pool: {}", name);

        self.pool_manager.scrub_pool(name).await.map_err(|_e| {
            create_zfs_error(
                "Failed to scrub pool: error details".to_string(),
                ZfsOperation::PoolCreate,
            )
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_pool_error_path() {
        let manager = ZfsManager::mock();
        let result = manager.create_pool("nonexistent", &[]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_destroy_pool_error_path() {
        let manager = ZfsManager::mock();
        let result = manager.destroy_pool("nonexistent-pool").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_pool_status_error_path() {
        let manager = ZfsManager::mock();
        let result = manager.get_pool_status("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_scrub_pool_error_path() {
        let manager = ZfsManager::mock();
        let result = manager.scrub_pool("nonexistent").await;
        assert!(result.is_err());
    }
}
