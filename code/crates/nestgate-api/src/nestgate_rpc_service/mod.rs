// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! NestGate RPC Service — tarpc + JSON-RPC interface.
//!
//! Exposes NestGate storage capabilities via high-performance RPC protocols
//! for inter-primal communication.
//!
//! ## Protocol escalation (same pattern as Songbird)
//!
//! - **JSON-RPC** for universal access (HTTP-based)
//! - **tarpc** for high-performance binary RPC (native Rust)

pub mod json_rpc_handler;
pub mod parsing;
pub mod tarpc_server;
pub mod types;

pub use json_rpc_handler::NestGateJsonRpcHandler;
pub use tarpc_server::NestGateRpcServer;
pub use types::*;

/// Capability labels advertised by this primal.
pub(crate) const NESTGATE_CAPABILITY_LABELS: &[&str] = &[
    "storage",
    "zfs",
    "snapshots",
    "replication",
    "compression",
    "deduplication",
];

/// Build the capability vector used by RPC and HTTP protocol advertisement.
#[must_use]
pub(crate) fn nestgate_capabilities_vec() -> Vec<String> {
    NESTGATE_CAPABILITY_LABELS
        .iter()
        .copied()
        .map(String::from)
        .collect()
}

// ==================== TARPC SERVICE TRAIT ====================

/// `NestGate` RPC service trait — defines storage operations for inter-primal communication.
///
/// Follows the same pattern as Songbird's `SongbirdRpc` trait.
#[tarpc::service]
pub trait NestGateRpc {
    /// List available storage pools
    async fn list_pools() -> Vec<PoolInfo>;
    /// List datasets in a pool
    async fn list_datasets(pool: String) -> Vec<DatasetInfo>;
    /// Create a new dataset
    async fn create_dataset(request: CreateDatasetRequest) -> OperationResult;
    /// Delete a dataset
    async fn delete_dataset(pool: String, name: String) -> OperationResult;
    /// Create a snapshot
    async fn create_snapshot(
        pool: String,
        dataset: String,
        snapshot_name: String,
    ) -> OperationResult;
    /// List snapshots for a dataset
    async fn list_snapshots(pool: String, dataset: String) -> Vec<SnapshotInfo>;
    /// Get storage metrics
    async fn get_metrics() -> StorageMetrics;
    /// Get health status
    async fn health() -> HealthStatus;
    /// Get version information
    async fn version() -> VersionInfo;
    /// Get available capabilities
    async fn capabilities() -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tarpc::context::Context;

    #[test]
    fn capabilities_vec_matches_labels() {
        let v = nestgate_capabilities_vec();
        assert_eq!(v.len(), NESTGATE_CAPABILITY_LABELS.len());
        for (i, s) in v.iter().enumerate() {
            assert_eq!(s, NESTGATE_CAPABILITY_LABELS[i]);
        }
    }

    #[test]
    fn rpc_types_serde_roundtrip() {
        let pool = PoolInfo {
            name: "p".into(),
            total_capacity_gb: 10,
            used_capacity_gb: 5,
            available_capacity_gb: 5,
            health_status: "ONLINE".into(),
            backend: "zfs".into(),
        };
        let j = serde_json::to_string(&pool).unwrap();
        let back: PoolInfo = serde_json::from_str(&j).unwrap();
        assert_eq!(back.name, pool.name);

        let op = OperationResult {
            success: true,
            message: "ok".into(),
            data: Some(serde_json::json!({"a": 1})),
        };
        let j2 = serde_json::to_string(&op).unwrap();
        let back2: OperationResult = serde_json::from_str(&j2).unwrap();
        assert!(back2.success);

        let m = StorageMetrics {
            total_capacity_gb: 100,
            used_capacity_gb: 40,
            available_capacity_gb: 60,
            compression_ratio: 1.5,
            dedup_ratio: 1.0,
            dataset_count: 3,
            snapshot_count: 10,
        };
        let _: StorageMetrics = serde_json::from_str(&serde_json::to_string(&m).unwrap()).unwrap();
    }

    #[test]
    fn rpc_types_full_serde_roundtrip() {
        let dataset = DatasetInfo {
            name: "p/d".into(),
            pool_name: "p".into(),
            used_space_gb: 3,
            compression_ratio: 1.2,
            dedup_ratio: 1.0,
            created_at: Some("2020-01-01T00:00:00Z".into()),
        };
        let _: DatasetInfo =
            serde_json::from_str(&serde_json::to_string(&dataset).unwrap()).unwrap();

        let snap = SnapshotInfo {
            name: "p/d@s".into(),
            dataset: "p/d".into(),
            created_at: "t".into(),
            size_gb: 1,
        };
        let _: SnapshotInfo = serde_json::from_str(&serde_json::to_string(&snap).unwrap()).unwrap();

        let health = HealthStatus {
            status: "healthy".into(),
            version: "1".into(),
            uptime_seconds: 10,
            pools_healthy: 1,
            pools_total: 1,
        };
        let _: HealthStatus =
            serde_json::from_str(&serde_json::to_string(&health).unwrap()).unwrap();

        let ver = VersionInfo {
            version: "0.1".into(),
            protocol: "tarpc".into(),
            capabilities: vec!["a".into()],
        };
        let _: VersionInfo = serde_json::from_str(&serde_json::to_string(&ver).unwrap()).unwrap();

        let req = CreateDatasetRequest {
            pool: "p".into(),
            name: "n".into(),
            properties: HashMap::from([("compression".into(), "lz4".into())]),
        };
        let _: CreateDatasetRequest =
            serde_json::from_str(&serde_json::to_string(&req).unwrap()).unwrap();
    }

    #[test]
    fn create_dataset_request_tier_property_coverage() {
        let mut props = std::collections::HashMap::new();
        props.insert("tier".into(), "hot".into());
        let _ = CreateDatasetRequest {
            pool: "p".into(),
            name: "d".into(),
            properties: props.clone(),
        };
        props.insert("tier".into(), "bogus".into());
        let _ = CreateDatasetRequest {
            pool: "p".into(),
            name: "d".into(),
            properties: props,
        };
    }

    #[tokio::test]
    async fn tarpc_server_core_paths() {
        let server = NestGateRpcServer::default();
        let ctx = Context::current();
        let _ = server.clone().list_pools(ctx).await;

        let ctx = Context::current();
        let _ = server
            .clone()
            .list_datasets(ctx, "nonexistent_pool".into())
            .await;

        let ctx = Context::current();
        let del = server
            .clone()
            .delete_dataset(ctx, "tank".into(), String::new())
            .await;
        assert!(!del.success);

        let ctx = Context::current();
        let del2 = server
            .clone()
            .delete_dataset(ctx, "tank".into(), "tank".into())
            .await;
        assert!(!del2.success);

        let ctx = Context::current();
        let _ = server
            .clone()
            .create_snapshot(ctx, "p".into(), "d".into(), "snap".into())
            .await;

        let ctx = Context::current();
        let _ = server
            .clone()
            .list_snapshots(ctx, "p".into(), "d".into())
            .await;

        let ctx = Context::current();
        let _ = server.clone().get_metrics(ctx).await;

        let ctx = Context::current();
        let _ = server.clone().health(ctx).await;

        let ctx = Context::current();
        let _ = server.clone().version(ctx).await;

        let ctx = Context::current();
        let caps = server.clone().capabilities(ctx).await;
        assert!(!caps.is_empty());

        let ctx = Context::current();
        let _ = server
            .clone()
            .create_dataset(
                ctx,
                CreateDatasetRequest {
                    pool: "nonexistent_pool_xyz".into(),
                    name: "ds".into(),
                    properties: HashMap::from([
                        ("tier".into(), "cold".into()),
                        ("x".into(), "y".into()),
                    ]),
                },
            )
            .await;
    }

    #[tokio::test]
    async fn json_rpc_handler_routes_methods() {
        let h = NestGateJsonRpcHandler::default();

        assert!(
            h.handle("list_pools", serde_json::json!(null))
                .await
                .is_ok()
        );
        assert!(
            h.handle("list_datasets", serde_json::json!("p"))
                .await
                .is_ok()
        );
        assert!(
            h.handle("get_metrics", serde_json::json!(null))
                .await
                .is_ok()
        );

        for m in ["health", "health.liveness", "health.check"] {
            assert!(h.handle(m, serde_json::json!(null)).await.is_ok());
        }
        assert!(
            h.handle("health.readiness", serde_json::json!(null))
                .await
                .is_ok()
        );
        assert!(h.handle("version", serde_json::json!(null)).await.is_ok());
        assert!(
            h.handle("capabilities", serde_json::json!(null))
                .await
                .is_ok()
        );
        assert!(
            h.handle("capabilities.list", serde_json::json!(null))
                .await
                .is_ok()
        );

        assert!(
            h.handle("not_a_real_method", serde_json::json!(null))
                .await
                .is_err()
        );
    }
}
