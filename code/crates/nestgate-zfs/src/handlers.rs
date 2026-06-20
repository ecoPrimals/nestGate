// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]
//
// Provides standardized request handling for ZFS operations using the unified
// service architecture and error handling patterns.
//
// **CANONICAL MODERNIZATION**: Migrated to zero-cost native async patterns

//! Handlers module

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use nestgate_core::error::Result;
use nestgate_types::{EnvSource, ProcessEnv, env_parsed, env_var_or_default};

use crate::config::ZfsConfig;

/// ZFS service health information
///
/// Contains health status and resource counts for a ZFS service instance.
/// Used for monitoring and service discovery health checks.
///
/// # Fields
///
/// * `status` - Current health status ("healthy", "degraded", "unhealthy")
/// * `pools_count` - Number of ZFS pools managed
/// * `datasets_count` - Total number of datasets across all pools
/// * `snapshots_count` - Total number of snapshots
/// * `last_check` - Timestamp of last health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsHealthInfo {
    /// Current health status
    pub status: String,
    /// Number of ZFS pools managed by this service
    pub pools_count: usize,
    /// Total number of datasets across all pools
    pub datasets_count: usize,
    /// Total number of snapshots
    pub snapshots_count: usize,
    /// Timestamp of last health check
    pub last_check: std::time::SystemTime,
}

/// ZFS service metrics
///
/// Operational metrics for ZFS service performance and reliability monitoring.
///
/// # Fields
///
/// * `requests_processed` - Total number of requests handled
/// * `errors_count` - Total number of errors encountered
/// * `average_response_time_ms` - Average request processing time in milliseconds
/// * `uptime_seconds` - Service uptime in seconds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsMetrics {
    /// Total number of requests processed
    pub requests_processed: u64,
    /// Total number of errors encountered
    pub errors_count: u64,
    /// Average response time in milliseconds
    pub average_response_time_ms: f64,
    /// Service uptime in seconds
    pub uptime_seconds: u64,
}

/// ZFS operation request types
///
/// Enumeration of all supported ZFS operations that can be requested
/// through the service API. Each variant contains the parameters needed
/// for that specific operation.
///
/// # Variants
///
/// - Pool operations: Create, destroy, and query ZFS pools
/// - Dataset operations: Create, destroy, and list datasets
/// - Snapshot operations: Create, destroy, and list snapshots
/// - Health check: Query service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsRequest {
    /// Pool operations
    PoolCreate {
        /// Pool name
        name: String,
        /// Device paths for the pool
        devices: Vec<String>,
    },
    /// Pooldestroy
    PoolDestroy {
        /// Pool name to destroy
        name: String,
    },
    /// Poolstatus
    PoolStatus {
        /// Optional pool name filter
        name: Option<String>,
    },
    /// Dataset operations
    DatasetCreate {
        /// Dataset name
        name: String,
        /// Dataset properties
        properties: HashMap<String, String>,
    },
    /// Destroy a dataset
    DatasetDestroy {
        /// Dataset name to destroy
        name: String,
    },
    /// List datasets
    DatasetList {
        /// Optional pool filter
        pool: Option<String>,
    },

    /// Snapshot operations
    SnapshotCreate {
        /// Source dataset
        dataset: String,
        /// Snapshot name
        name: String,
    },
    /// Destroy a snapshot
    SnapshotDestroy {
        /// Snapshot name to destroy
        name: String,
    },
    /// List snapshots
    SnapshotList {
        /// Optional dataset filter
        dataset: Option<String>,
    },

    /// Health check
    HealthCheck,
}

/// ZFS operation responses
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfsresponse
pub enum ZfsResponse {
    /// Pool status information
    PoolStatus {
        /// List of pools
        pools: Vec<PoolInfo>,
    },
    /// Dataset listing
    DatasetList {
        /// List of datasets
        datasets: Vec<DatasetInfo>,
    },
    /// Snapshot listing
    SnapshotList {
        /// List of snapshots
        snapshots: Vec<SnapshotInfo>,
    },
    /// Operation success
    Success {
        /// Success message
        message: String,
    },
    /// Health status
    Health {
        /// Health status string
        status: String,
        /// Health details
        details: HashMap<String, String>,
    },
}
/// Pool information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Poolinfo
pub struct PoolInfo {
    /// Name
    pub name: String,
    /// State
    pub state: String,
    /// Size
    pub size: String,
    /// Allocated
    pub allocated: String,
    /// Free
    pub free: String,
    /// Devices
    pub devices: Vec<String>,
}
/// Dataset information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Datasetinfo
pub struct DatasetInfo {
    /// Name
    pub name: String,
    /// Used
    pub used: String,
    /// Available
    pub available: String,
    /// Referenced
    pub referenced: String,
    /// Mountpoint
    pub mountpoint: String,
}
/// Snapshot information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Snapshotinfo
pub struct SnapshotInfo {
    /// Name
    pub name: String,
    /// Used
    pub used: String,
    /// Referenced
    pub referenced: String,
    /// Creation
    pub creation: String,
}
/// ZFS Request Handler Service
pub struct ZfsRequestHandler {
    config: ZfsConfig,
}
impl ZfsRequestHandler {
    /// Create a new ZFS request handler
    #[must_use]
    pub const fn new(config: ZfsConfig) -> Self {
        Self { config }
    }

    /// Get the configuration
    #[must_use]
    pub const fn config(&self) -> &ZfsConfig {
        &self.config
    }

    /// Get the configured default pool name
    #[must_use]
    pub fn get_default_pool_name(&self) -> String {
        self.get_default_pool_name_from_env_source(&ProcessEnv)
    }

    /// Like [`Self::get_default_pool_name`], but reads from an injectable [`EnvSource`].
    #[must_use]
    pub fn get_default_pool_name_from_env_source(&self, env: &(impl EnvSource + ?Sized)) -> String {
        env_var_or_default(env, "NESTGATE_DEFAULT_POOL", "tank")
    }

    /// Check if performance monitoring is enabled
    #[must_use]
    pub fn is_performance_monitoring_enabled(&self) -> bool {
        self.is_performance_monitoring_enabled_from_env_source(&ProcessEnv)
    }

    /// Like [`Self::is_performance_monitoring_enabled`], but reads from an injectable [`EnvSource`].
    #[must_use]
    pub fn is_performance_monitoring_enabled_from_env_source(
        &self,
        env: &(impl EnvSource + ?Sized),
    ) -> bool {
        env.get("NESTGATE_PERFORMANCE_MONITORING")
            .is_none_or(|v| v.parse().unwrap_or(true))
    }

    /// Get the configured health check interval
    #[must_use]
    pub fn get_health_check_interval(&self) -> Duration {
        self.get_health_check_interval_from_env_source(&ProcessEnv)
    }

    /// Like [`Self::get_health_check_interval`], but reads from an injectable [`EnvSource`].
    #[must_use]
    pub fn get_health_check_interval_from_env_source(
        &self,
        env: &(impl EnvSource + ?Sized),
    ) -> Duration {
        let seconds = env_parsed(env, "NESTGATE_HEALTH_CHECK_INTERVAL", 300_u64);
        Duration::from_secs(seconds)
    }

    /// Handle ZFS-specific requests
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn handle_zfs_request(&self, request: ZfsRequest) -> Result<ZfsResponse> {
        match request {
            ZfsRequest::PoolStatus { name } => self.handle_pool_status(name).await,
            ZfsRequest::DatasetList { pool } => self.handle_dataset_list(pool).await,
            ZfsRequest::SnapshotList { dataset } => self.handle_snapshot_list(dataset).await,
            ZfsRequest::HealthCheck => self.handle_health_check(),
            ZfsRequest::PoolCreate { name, .. } => Err(anyhow::anyhow!(
                "zfs pool create ({name}) not yet implemented — use `zpool create` directly"
            )
            .into()),
            ZfsRequest::PoolDestroy { name } => Err(anyhow::anyhow!(
                "zfs pool destroy ({name}) not yet implemented — use `zpool destroy` directly"
            )
            .into()),
            ZfsRequest::DatasetCreate { name, .. } => Err(anyhow::anyhow!(
                "zfs dataset create ({name}) not yet implemented — use `zfs create` directly"
            )
            .into()),
            ZfsRequest::DatasetDestroy { name } => Err(anyhow::anyhow!(
                "zfs dataset destroy ({name}) not yet implemented — use `zfs destroy` directly"
            )
            .into()),
            ZfsRequest::SnapshotCreate { dataset, name } => Err(anyhow::anyhow!(
                "zfs snapshot create ({dataset}@{name}) not yet implemented — use `zfs snapshot` directly"
            )
            .into()),
            ZfsRequest::SnapshotDestroy { name } => Err(anyhow::anyhow!(
                "zfs snapshot destroy ({name}) not yet implemented — use `zfs destroy` directly"
            )
            .into()),
        }
    }

    /// Handles  Pool Status
    async fn handle_pool_status(&self, name: Option<String>) -> Result<ZfsResponse> {
        // Use configured pool name or provided name
        let pool_name = name.unwrap_or_else(|| self.get_default_pool_name());

        if crate::real_zfs_operations::RealZfsOperations::is_available().await {
            let real_ops = crate::real_zfs_operations::RealZfsOperations::default();
            real_ops.get_pool_status(Some(pool_name)).await
        } else {
            Err(anyhow::anyhow!(
                "ZFS runtime unavailable — cannot query pool status for `{pool_name}`"
            )
            .into())
        }
    }

    /// Handles  Dataset List
    async fn handle_dataset_list(&self, pool: Option<String>) -> Result<ZfsResponse> {
        // Use configured pool name or provided pool
        let pool_name = pool.unwrap_or_else(|| self.get_default_pool_name());

        if crate::real_zfs_operations::RealZfsOperations::is_available().await {
            let real_ops = crate::real_zfs_operations::RealZfsOperations::default();
            real_ops.get_dataset_list(Some(pool_name)).await
        } else {
            Err(
                anyhow::anyhow!("ZFS runtime unavailable — cannot list datasets for `{pool_name}`")
                    .into(),
            )
        }
    }

    /// Handles  Snapshot List
    async fn handle_snapshot_list(&self, dataset: Option<String>) -> Result<ZfsResponse> {
        if crate::real_zfs_operations::RealZfsOperations::is_available().await {
            let real_ops = crate::real_zfs_operations::RealZfsOperations::default();
            real_ops.get_snapshot_list(dataset).await
        } else {
            Err(anyhow::anyhow!("ZFS runtime unavailable — cannot list snapshots").into())
        }
    }

    /// Reports whether the ZFS subsystem is reachable.
    ///
    /// Returns `zfs_available: true/false` rather than hardcoded counts,
    /// so consumers get an honest signal without simulated data.
    fn handle_health_check(&self) -> Result<ZfsResponse> {
        let zfs_bin = &self.config.zfs_binary;
        let available = std::path::Path::new(zfs_bin).exists()
            || std::process::Command::new(zfs_bin)
                .arg("version")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .is_ok();

        let status = if available { "healthy" } else { "degraded" };

        let mut details = HashMap::new();
        details.insert(String::from("zfs_available"), available.to_string());
        details.insert(String::from("zfs_binary"), zfs_bin.clone());

        Ok(ZfsResponse::Health {
            status: String::from(status),
            details,
        })
    }
}

// REMOVED: UniversalService trait implementation - trait no longer exists
// All service functionality has been migrated to the canonical trait system

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ZfsConfig;

    #[test]
    fn zfs_health_info_round_trips_json() {
        let t = std::time::SystemTime::UNIX_EPOCH;
        let h = ZfsHealthInfo {
            status: String::from("healthy"),
            pools_count: 2,
            datasets_count: 5,
            snapshots_count: 1,
            last_check: t,
        };
        let json = serde_json::to_string(&h).expect("serialize");
        let back: ZfsHealthInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.pools_count, 2);
        assert_eq!(back.status, "healthy");
    }

    #[test]
    fn zfs_request_and_response_variants_serialize() {
        let req = ZfsRequest::PoolCreate {
            name: String::from("p"),
            devices: vec![String::from("/dev/sda")],
        };
        let _ = serde_json::to_string(&req).expect("req json");

        let resp = ZfsResponse::Success {
            message: String::from("ok"),
        };
        let _ = serde_json::to_string(&resp).expect("resp json");
    }

    #[tokio::test]
    async fn handler_returns_health_on_health_check() {
        let handler = ZfsRequestHandler::new(ZfsConfig::default());
        let out = handler
            .handle_zfs_request(ZfsRequest::HealthCheck)
            .await
            .expect("health response");
        match out {
            ZfsResponse::Health { status, details } => {
                assert!(
                    status == "healthy" || status == "degraded",
                    "unexpected status: {status}"
                );
                assert!(details.contains_key("zfs_available"));
                assert!(details.contains_key("zfs_binary"));
            }
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[tokio::test]
    async fn handler_unimplemented_mutations_return_error() {
        let handler = ZfsRequestHandler::new(ZfsConfig::default());
        let result = handler
            .handle_zfs_request(ZfsRequest::PoolDestroy {
                name: String::from("gone"),
            })
            .await;
        assert!(result.is_err(), "unimplemented mutations must return Err");
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("not yet implemented"),
            "error should explain: {msg}"
        );
    }

    #[test]
    fn handler_exposes_config_and_default_pool_name() {
        let cfg = ZfsConfig::default();
        let handler = ZfsRequestHandler::new(cfg.clone());
        assert_eq!(handler.config().zfs_binary, cfg.zfs_binary);
        assert_eq!(handler.config().use_sudo, cfg.use_sudo);
        let name = handler.get_default_pool_name();
        assert!(!name.is_empty());
    }

    #[test]
    fn handler_health_check_interval_is_positive() {
        let handler = ZfsRequestHandler::new(ZfsConfig::default());
        let d = handler.get_health_check_interval();
        assert!(d.as_secs() > 0);
    }
}
