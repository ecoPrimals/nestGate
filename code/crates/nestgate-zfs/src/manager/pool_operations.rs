//! ZFS Manager Pool Operations - ZFS pool management operations
//!
//! Contains all pool-related operations including creation, destruction,
//! status checking, and maintenance operations like scrubbing.

use crate::error::{Result, ZfsError};
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
            .map_err(|e| ZfsError::Internal {
                message: format!("Failed to create pool: {e}"),
            })?;

        Ok(result)
    }

    /// Destroy a ZFS pool
    pub async fn destroy_pool(&self, name: &str) -> Result<()> {
        info!("Destroying ZFS pool: {}", name);

        self.pool_manager
            .destroy_pool(name)
            .await
            .map_err(|e| ZfsError::Internal {
                message: format!("Failed to destroy pool: {e}"),
            })?;

        Ok(())
    }

    /// Get pool status information
    pub async fn get_pool_status(&self, name: &str) -> Result<String> {
        self.pool_manager
            .get_pool_status(name)
            .await
            .map_err(|e| ZfsError::Internal {
                message: format!("Failed to get pool status: {e}"),
            })
    }

    /// Initiate pool scrub
    pub async fn scrub_pool(&self, name: &str) -> Result<()> {
        info!("Starting scrub for pool: {}", name);

        self.pool_manager
            .scrub_pool(name)
            .await
            .map_err(|e| ZfsError::Internal {
                message: format!("Failed to scrub pool: {e}"),
            })?;

        Ok(())
    }
}
