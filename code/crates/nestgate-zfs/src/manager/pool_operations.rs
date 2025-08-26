//
// Contains all pool-related operations including creation, destruction,
// status checking, and maintenance operations like scrubbing.

use nestgate_core::error::conversions::create_zfs_error;
use nestgate_core::error::domain_errors::ZfsOperation;
use nestgate_core::Result;
// Removed unused tracing import

use super::ZfsManager;
use tracing::info;

impl ZfsManager {
    /// Create a new ZFS pool
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
            .map_err(|e| {
                create_zfs_error(
                    format!("Failed to create pool: {e}"),
                    ZfsOperation::PoolCreate
                )
            })?;

        Ok(result)
    }

    /// Destroy a ZFS pool
    pub async fn destroy_pool(&self, name: &str) -> Result<()> {
        info!("Destroying ZFS pool: {}", name);

        self.pool_manager.destroy_pool(name).await.map_err(|e| {
            create_zfs_error(
                format!("Failed to destroy pool: {e}"),
                ZfsOperation::PoolCreate
            )
        })?;

        Ok(())
    }

    /// Get pool status information
    pub async fn get_pool_status(&self, name: &str) -> Result<String> {
        self.pool_manager.get_pool_status(name).await.map_err(|e| {
            create_zfs_error(
                format!("Failed to get pool status: {e}"),
                ZfsOperation::PoolCreate
            )
        })
    }

    /// Initiate pool scrub
    pub async fn scrub_pool(&self, name: &str) -> Result<()> {
        info!("Starting scrub for pool: {}", name);

        self.pool_manager.scrub_pool(name).await.map_err(|e| {
            create_zfs_error(
                format!("Failed to scrub pool: {e}"),
                ZfsOperation::PoolCreate
            )
        })?;

        Ok(())
    }
}
