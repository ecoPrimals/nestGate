//
// Contains all pool-related operations including creation, destruction,
// status checking, and maintenance operations like scrubbing.

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
        pub fn create_pool(
        &self,
        name: &str,
        devices: &[String],
    ) -> Result<crate::pool::PoolInfo>  {
        info!("Creating ZFS pool: {}", name);

        let result = self
            .pool_manager
            .create_pool(name, devices)
            .await
            .map_err(|_e| {
                create_zfs_error(
                    format!("Failed to create pool: {"actual_error_details"}"),
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
        #[must_use]
        pub fn destroy_pool(&self, name: &str) -> Result<()>  {
        info!("Destroying ZFS pool: {}", name);

        self.pool_manager.destroy_pool(name).await.map_err(|_e| {
            create_zfs_error(
                format!("Failed to destroy pool: {"actual_error_details"}"),
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
        pub async fn get_pool_status(&self, name: &str) -> Result<String>  {
        self.pool_manager.get_pool_status(name).await.map_err(|_e| {
            create_zfs_error(
                format!("Failed to get pool status: {"actual_error_details"}"),
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
        #[must_use]
        pub fn scrub_pool(&self, name: &str) -> Result<()>  {
        info!("Starting scrub for pool: {}", name);

        self.pool_manager.scrub_pool(name).await.map_err(|_e| {
            create_zfs_error(
                format!("Failed to scrub pool: {"actual_error_details"}"),
                ZfsOperation::PoolCreate,
            )
        })?;

        Ok(())
    }
}
