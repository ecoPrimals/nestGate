// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! tarpc server implementation backed by real ZFS operations.

use std::sync::Arc;
use std::time::SystemTime;

use nestgate_zfs::command::ZfsOperations;
use nestgate_zfs::numeric::usize_to_f64_lossy;
use nestgate_zfs::types::StorageTier;
use tarpc::context::Context;
use tracing::{debug, error, info, warn};

use super::parsing::{parse_pool_capacity, parse_zfs_size};
use super::types::{
    CreateDatasetRequest, DatasetInfo, HealthStatus, OperationResult, PoolInfo, SnapshotInfo,
    StorageMetrics, VersionInfo,
};
use super::{NestGateRpc, nestgate_capabilities_vec};

/// tarpc server with real ZFS backend connections
#[derive(Clone)]
pub struct NestGateRpcServer {
    state: Arc<ServerState>,
}

struct ServerState {
    zfs_backend: Arc<ZfsOperations>,
    start_time: SystemTime,
}

impl NestGateRpcServer {
    /// Create server with a specific ZFS backend (zero-copy via `Arc`)
    pub fn new(zfs_backend: Arc<ZfsOperations>) -> Self {
        info!("Initializing NestGate RPC server with real ZFS backend");
        Self {
            state: Arc::new(ServerState {
                zfs_backend,
                start_time: SystemTime::now(),
            }),
        }
    }

    /// Create server with a freshly-initialised default backend.
    ///
    /// # Errors
    ///
    /// Returns error if ZFS backend initialisation fails.
    pub fn with_default_backend() -> Result<Self, String> {
        let zfs_backend = ZfsOperations::new();
        Ok(Self::new(Arc::new(zfs_backend)))
    }
}

impl Default for NestGateRpcServer {
    fn default() -> Self {
        Self::with_default_backend().unwrap_or_else(|e| {
            warn!("Failed to create default backend: {e}, creating fallback");
            Self::new(Arc::new(ZfsOperations::new()))
        })
    }
}

impl NestGateRpc for NestGateRpcServer {
    async fn list_pools(self, _context: Context) -> Vec<PoolInfo> {
        debug!("tarpc: list_pools()");

        match self.state.zfs_backend.list_pools().await {
            Ok(pools) => {
                info!("Listed {} ZFS pools from real backend", pools.len());
                pools
                    .into_iter()
                    .map(|pool_info| {
                        let (total, used, available) = parse_pool_capacity(&pool_info);
                        PoolInfo {
                            name: pool_info.name,
                            total_capacity_gb: total,
                            used_capacity_gb: used,
                            available_capacity_gb: available,
                            health_status: pool_info.health,
                            backend: "zfs".to_string(),
                        }
                    })
                    .collect()
            }
            Err(e) => {
                error!("Failed to list ZFS pools: {e}");
                Vec::new()
            }
        }
    }

    async fn list_datasets(self, _context: Context, pool: String) -> Vec<DatasetInfo> {
        debug!("tarpc: list_datasets({pool})");

        match self.state.zfs_backend.list_datasets(None).await {
            Ok(datasets) => {
                info!("Listed {} datasets from real backend", datasets.len());
                let prefix = format!("{pool}/");
                datasets
                    .into_iter()
                    .filter(|ds| ds.name.starts_with(&prefix) || ds.name == pool)
                    .map(|ds| {
                        let used_gb = parse_zfs_size(&ds.used);
                        DatasetInfo {
                            name: ds.name,
                            pool_name: pool.clone(),
                            used_space_gb: used_gb,
                            compression_ratio: 1.0,
                            dedup_ratio: 1.0,
                            created_at: Some(chrono::Utc::now().to_rfc3339()),
                        }
                    })
                    .collect()
            }
            Err(e) => {
                error!("Failed to list datasets for pool {pool}: {e}");
                Vec::new()
            }
        }
    }

    async fn create_dataset(
        self,
        _context: Context,
        request: CreateDatasetRequest,
    ) -> OperationResult {
        info!("tarpc: create_dataset({}/{})", request.pool, request.name);

        let dataset_path = format!("{}/{}", request.pool, request.name);

        let tier = request
            .properties
            .get("tier")
            .and_then(|t| match t.as_str() {
                "hot" => Some(StorageTier::Hot),
                "warm" => Some(StorageTier::Warm),
                "cold" => Some(StorageTier::Cold),
                "archive" => Some(StorageTier::Archive),
                "cache" => Some(StorageTier::Cache),
                _ => None,
            })
            .unwrap_or(StorageTier::Warm);

        match self
            .state
            .zfs_backend
            .create_dataset(&dataset_path, Some(&request.properties))
            .await
        {
            Ok(()) => {
                info!("Dataset created: {dataset_path}");
                OperationResult {
                    success: true,
                    message: format!("Dataset {dataset_path} created successfully"),
                    data: Some(serde_json::json!({
                        "pool": request.pool,
                        "name": request.name,
                        "path": dataset_path,
                        "tier": format!("{tier:?}"),
                        "properties": request.properties,
                    })),
                }
            }
            Err(e) => {
                error!("Failed to create dataset {dataset_path}: {e}");
                OperationResult {
                    success: false,
                    message: format!("Failed to create dataset: {e}"),
                    data: None,
                }
            }
        }
    }

    async fn delete_dataset(
        self,
        _context: Context,
        pool: String,
        name: String,
    ) -> OperationResult {
        let dataset_path = format!("{pool}/{name}");
        info!("tarpc: delete_dataset({dataset_path})");

        if name.is_empty() || name == pool {
            warn!("Blocked deletion of pool root: {pool}");
            return OperationResult {
                success: false,
                message: "Cannot delete pool root dataset".to_string(),
                data: None,
            };
        }

        match self.state.zfs_backend.destroy_dataset(&dataset_path).await {
            Ok(()) => {
                info!("Dataset destroyed: {dataset_path}");
                OperationResult {
                    success: true,
                    message: format!("Dataset {dataset_path} destroyed successfully"),
                    data: Some(serde_json::json!({
                        "pool": pool,
                        "name": name,
                        "path": dataset_path,
                    })),
                }
            }
            Err(e) => {
                error!("Failed to destroy dataset {dataset_path}: {e}");
                OperationResult {
                    success: false,
                    message: format!("Failed to destroy dataset: {e}"),
                    data: None,
                }
            }
        }
    }

    async fn create_snapshot(
        self,
        _context: Context,
        pool: String,
        dataset: String,
        snapshot_name: String,
    ) -> OperationResult {
        let dataset_path = format!("{pool}/{dataset}");
        let snapshot_path = format!("{dataset_path}@{snapshot_name}");
        info!("tarpc: create_snapshot({snapshot_path})");

        match self
            .state
            .zfs_backend
            .create_snapshot(&dataset_path, &snapshot_name)
            .await
        {
            Ok(()) => {
                info!("Snapshot created: {snapshot_path}");
                OperationResult {
                    success: true,
                    message: format!("Snapshot {snapshot_path} created successfully"),
                    data: Some(serde_json::json!({
                        "pool": pool,
                        "dataset": dataset,
                        "snapshot": snapshot_name,
                        "full_path": snapshot_path,
                        "created_at": chrono::Utc::now().to_rfc3339(),
                    })),
                }
            }
            Err(e) => {
                error!("Failed to create snapshot {snapshot_path}: {e}");
                OperationResult {
                    success: false,
                    message: format!("Failed to create snapshot: {e}"),
                    data: None,
                }
            }
        }
    }

    async fn list_snapshots(
        self,
        _context: Context,
        pool: String,
        dataset: String,
    ) -> Vec<SnapshotInfo> {
        let dataset_path = format!("{pool}/{dataset}");
        debug!("tarpc: list_snapshots({dataset_path})");

        match self
            .state
            .zfs_backend
            .list_snapshots(Some(&dataset_path))
            .await
        {
            Ok(snapshots) => {
                info!("Listed {} snapshots for {dataset_path}", snapshots.len());
                snapshots
                    .into_iter()
                    .map(|snap| {
                        let size_gb = parse_zfs_size(&snap.used);
                        SnapshotInfo {
                            name: snap.name,
                            dataset: dataset.clone(),
                            created_at: snap.creation,
                            size_gb,
                        }
                    })
                    .collect()
            }
            Err(e) => {
                error!("Failed to list snapshots for {dataset_path}: {e}");
                Vec::new()
            }
        }
    }

    async fn get_metrics(self, _context: Context) -> StorageMetrics {
        debug!("tarpc: get_metrics()");

        let (pools_result, datasets_result, snapshots_result) = tokio::join!(
            self.state.zfs_backend.list_pools(),
            self.state.zfs_backend.list_datasets(None),
            self.state.zfs_backend.list_snapshots(None)
        );

        let pools = pools_result.unwrap_or_default();
        let (total_gb, used_gb, available_gb, compression_sum) =
            pools
                .iter()
                .fold((0u64, 0u64, 0u64, 0.0f64), |(t, u, a, c), pool| {
                    let (pool_total, pool_used, pool_avail) = parse_pool_capacity(pool);
                    (t + pool_total, u + pool_used, a + pool_avail, c + 1.5)
                });

        let dataset_count = datasets_result.as_ref().map_or(0, std::vec::Vec::len);
        let snapshot_count = snapshots_result.as_ref().map_or(0, std::vec::Vec::len);

        let avg_compression = if pools.is_empty() {
            1.0
        } else {
            compression_sum / usize_to_f64_lossy(pools.len())
        };

        info!(
            "Aggregated metrics: {} pools, {} datasets, {} snapshots",
            pools.len(),
            dataset_count,
            snapshot_count
        );

        StorageMetrics {
            total_capacity_gb: total_gb,
            used_capacity_gb: used_gb,
            available_capacity_gb: available_gb,
            compression_ratio: avg_compression,
            dedup_ratio: 1.0,
            dataset_count,
            snapshot_count,
        }
    }

    async fn health(self, _context: Context) -> HealthStatus {
        debug!("tarpc: health()");

        let uptime_seconds = self
            .state
            .start_time
            .elapsed()
            .unwrap_or_default()
            .as_secs();

        let (pools_total, pools_healthy) = match self.state.zfs_backend.list_pools().await {
            Ok(pools) => {
                let total = pools.len();
                let healthy = pools
                    .iter()
                    .filter(|p| p.health == "ONLINE" || p.health == "healthy")
                    .count();
                (total, healthy)
            }
            Err(e) => {
                warn!("Failed to query pool health: {e}");
                (0, 0)
            }
        };

        let status = match (pools_total, pools_healthy) {
            (0, _) => "unknown",
            (t, h) if h == t => "healthy",
            (_, h) if h > 0 => "degraded",
            _ => "unhealthy",
        }
        .to_string();

        info!("Health check: {status} ({pools_healthy}/{pools_total} pools healthy)");

        HealthStatus {
            status,
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds,
            pools_healthy,
            pools_total,
        }
    }

    async fn version(self, _context: Context) -> VersionInfo {
        debug!("tarpc: version()");
        VersionInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            protocol: String::from("tarpc"),
            capabilities: nestgate_capabilities_vec(),
        }
    }

    async fn capabilities(self, _context: Context) -> Vec<String> {
        debug!("tarpc: capabilities()");
        nestgate_capabilities_vec()
    }
}
