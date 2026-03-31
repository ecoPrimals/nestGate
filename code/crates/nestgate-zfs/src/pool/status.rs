// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Aggregate pool status and raw `zpool status` output.

use tokio::process::Command as TokioCommand;
use tracing::debug;

use crate::error::{Result, ZfsOperation, create_zfs_error};

use super::manager::ZfsPoolManager;
use super::types::PoolHealth;

impl ZfsPoolManager {
    /// Get overall pool status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_overall_status(&self) -> Result<crate::manager::PoolOverallStatus> {
        let pools = self.list_pools().await?;

        let pools_online = pools
            .iter()
            .filter(|p| matches!(p.health, PoolHealth::Healthy))
            .count();

        let pools_degraded = pools
            .iter()
            .filter(|p| matches!(p.health, PoolHealth::Warning | PoolHealth::Critical))
            .count();

        let total_capacity = pools.iter().map(|p| p.capacity.total_bytes).sum();

        let available_capacity = pools.iter().map(|p| p.capacity.available_bytes).sum();

        Ok(crate::manager::PoolOverallStatus {
            pools_online,
            pools_degraded,
            total_capacity,
            available_capacity,
        })
    }

    /// Get detailed pool status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_pool_status(&self, name: &str) -> Result<String> {
        debug!("Getting status for pool: {}", name);

        let output = TokioCommand::new("zpool")
            .args(["status", name])
            .output()
            .await
            .map_err(|_e| {
                create_zfs_error(
                    "Failed to execute zpool status: error details".to_string(),
                    ZfsOperation::Command,
                )
            })?;

        if !output.status.success() {
            return Err(create_zfs_error(
                format!(
                    "zpool status failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                ZfsOperation::Command,
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::pool::types::{PoolHealth, PoolState};

    use super::super::discovery::sample_pool_info;
    use super::super::manager::ZfsPoolManager;

    #[tokio::test]
    async fn get_overall_status_empty_cache() {
        let m = ZfsPoolManager::new_for_testing();
        let st = m
            .get_overall_status()
            .await
            .expect("test: get_overall_status");
        assert_eq!(st.pools_online, 0);
        assert_eq!(st.pools_degraded, 0);
        assert_eq!(st.total_capacity, 0);
        assert_eq!(st.available_capacity, 0);
    }

    #[tokio::test]
    async fn get_overall_status_counts_online_and_degraded() {
        let m = ZfsPoolManager::new_for_testing();
        m.insert_pool_for_testing(sample_pool_info(
            "a",
            PoolHealth::Healthy,
            PoolState::Online,
            1000,
            0,
            1000,
            0.0,
        ))
        .await;
        m.insert_pool_for_testing(sample_pool_info(
            "b",
            PoolHealth::Warning,
            PoolState::Degraded,
            500,
            100,
            400,
            10.0,
        ))
        .await;
        m.insert_pool_for_testing(sample_pool_info(
            "c",
            PoolHealth::Critical,
            PoolState::Faulted,
            200,
            0,
            200,
            0.0,
        ))
        .await;

        let st = m
            .get_overall_status()
            .await
            .expect("test: get_overall_status");
        assert_eq!(st.pools_online, 1);
        assert_eq!(st.pools_degraded, 2);
        assert_eq!(st.total_capacity, 1700);
        assert_eq!(st.available_capacity, 1600);
    }
}
