// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! JSON-RPC 2.0 handler that wraps the tarpc service for HTTP access.
//!
//! Orchestration and other capability peers can use this for initial
//! discovery before escalating to tarpc.  Supports both legacy method
//! names (`list_pools`) and ecosystem-standard semantic names
//! (`zfs.pool.list`) per `SEMANTIC_METHOD_NAMING_STANDARD.md`.

use nestgate_zfs::command::ZfsOperations;
use nestgate_zfs::native::{is_zfs_available, is_zpool_available};
use serde_json::json;
use tarpc::context::Context;

use super::NestGateRpc;
use super::tarpc_server::NestGateRpcServer;

/// JSON-RPC handler that delegates to the tarpc server implementation
pub struct NestGateJsonRpcHandler {
    server: NestGateRpcServer,
}

impl NestGateJsonRpcHandler {
    /// Create new JSON-RPC handler
    #[must_use]
    pub fn new() -> Self {
        Self {
            server: NestGateRpcServer::default(),
        }
    }

    /// Route a JSON-RPC method call to the appropriate tarpc handler.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the method is unknown or serialisation fails.
    pub async fn handle(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        match method {
            "list_pools" => {
                let ctx = Context::current();
                let result = self.server.clone().list_pools(ctx).await;
                serde_json::to_value(result)
                    .map_err(|e| format!("Failed to serialize list_pools result: {e}"))
            }
            "list_datasets" => {
                let pool: String = serde_json::from_value(params).map_err(|e| e.to_string())?;
                let ctx = Context::current();
                let result = self.server.clone().list_datasets(ctx, pool).await;
                serde_json::to_value(result)
                    .map_err(|e| format!("Failed to serialize list_datasets result: {e}"))
            }
            "get_metrics" => {
                let ctx = Context::current();
                let result = self.server.clone().get_metrics(ctx).await;
                serde_json::to_value(result)
                    .map_err(|e| format!("Failed to serialize get_metrics result: {e}"))
            }
            "health" | "health.liveness" | "health.check" => {
                let ctx = Context::current();
                let result = self.server.clone().health(ctx).await;
                serde_json::to_value(result)
                    .map_err(|e| format!("Failed to serialize health result: {e}"))
            }
            "health.readiness" => {
                let ctx = Context::current();
                let h = self.server.clone().health(ctx).await;
                let ready = h.status != "unhealthy";
                serde_json::to_value(serde_json::json!({
                    "ready": ready,
                    "status": h.status,
                    "version": h.version,
                    "uptime_seconds": h.uptime_seconds,
                    "pools_healthy": h.pools_healthy,
                    "pools_total": h.pools_total,
                }))
                .map_err(|e| format!("Failed to serialize health.readiness result: {e}"))
            }
            "version" => {
                let ctx = Context::current();
                let result = self.server.clone().version(ctx).await;
                serde_json::to_value(result)
                    .map_err(|e| format!("Failed to serialize version result: {e}"))
            }
            "capabilities" | "capabilities.list" => {
                let ctx = Context::current();
                let result = self.server.clone().capabilities(ctx).await;
                serde_json::to_value(result)
                    .map_err(|e| format!("Failed to serialize capabilities result: {e}"))
            }
            m if m.starts_with("zfs.") => handle_zfs_method(m, &params).await,
            _ => Err(format!("Unknown method: {method}")),
        }
    }
}

impl Default for NestGateJsonRpcHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Ecosystem-standard `zfs.*` semantic methods (GAP-MATRIX-04 alignment).
///
/// Uses [`ZfsOperations`] directly rather than tarpc so the response schema
/// matches the raw ZFS output exposed on the UDS surface.
///
/// # Errors
///
/// Returns `Err` for unknown methods, unavailable tools, or ZFS failures.
async fn handle_zfs_method(
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    match method {
        "zfs.pool.list" => {
            require_zpool().await?;
            let ops = ZfsOperations::new();
            ops.list_pools()
                .await
                .map(|pools| json!({"status": "success", "pools": pools}))
                .map_err(|e| format!("zfs.pool.list failed: {e}"))
        }
        "zfs.pool.get" => {
            let pool = str_param(params, "pool")?;
            require_zpool().await?;
            let ops = ZfsOperations::new();
            ops.pool_status(pool)
                .await
                .map(|s| {
                    json!({
                        "status": "success", "pool": pool,
                        "state": s.state, "scan": s.scan, "errors": s.errors,
                    })
                })
                .map_err(|e| format!("zfs.pool.get failed: {e}"))
        }
        "zfs.pool.health" => {
            require_zpool().await?;
            let ops = ZfsOperations::new();
            ops.list_pools()
                .await
                .map(|pools| {
                    let bad: Vec<&str> = pools
                        .iter()
                        .filter(|p| {
                            let h = p.health.to_ascii_lowercase();
                            !(h.contains("online") || h == "ok" || h == "healthy")
                        })
                        .map(|p| p.name.as_str())
                        .collect();
                    json!({"status":"success","pool_count":pools.len(),"pools_unhealthy":bad})
                })
                .map_err(|e| format!("zfs.pool.health failed: {e}"))
        }
        "zfs.dataset.list" => {
            require_zfs().await?;
            let pool = params.get("pool").and_then(serde_json::Value::as_str);
            let ops = ZfsOperations::new();
            ops.list_datasets(pool)
                .await
                .map(|ds| json!({"status": "success", "datasets": ds}))
                .map_err(|e| format!("zfs.dataset.list failed: {e}"))
        }
        "zfs.dataset.get" => {
            let name = str_param(params, "dataset")?;
            require_zfs().await?;
            let ops = ZfsOperations::new();
            ops.list_datasets(Some(name))
                .await
                .map_err(|e| format!("zfs.dataset.get failed: {e}"))
                .and_then(|ds| {
                    ds.into_iter()
                        .find(|d| d.name == name)
                        .map(|d| json!({"status":"success","dataset":d}))
                        .ok_or_else(|| format!("dataset {name:?} not found"))
                })
        }
        "zfs.snapshot.list" => {
            require_zfs().await?;
            let ds = params.get("dataset").and_then(serde_json::Value::as_str);
            let ops = ZfsOperations::new();
            ops.list_snapshots(ds)
                .await
                .map(|snaps| json!({"status": "success", "snapshots": snaps}))
                .map_err(|e| format!("zfs.snapshot.list failed: {e}"))
        }
        "zfs.health" => {
            let zpool_ok = is_zpool_available().await;
            let zfs_ok = is_zfs_available().await;
            Ok(json!({"status":"success","zpool_available":zpool_ok,"zfs_available":zfs_ok}))
        }
        _ => Err(format!("Unknown zfs method: {method}")),
    }
}

fn str_param<'a>(params: &'a serde_json::Value, key: &str) -> Result<&'a str, String> {
    params
        .get(key)
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| format!("missing '{key}' parameter"))
}

async fn require_zpool() -> Result<(), String> {
    if is_zpool_available().await {
        Ok(())
    } else {
        Err("zpool is not available on this system".to_string())
    }
}

async fn require_zfs() -> Result<(), String> {
    if is_zfs_available().await {
        Ok(())
    } else {
        Err("zfs is not available on this system".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn handle_unknown_method_returns_error_string() {
        let h = NestGateJsonRpcHandler::new();
        let err = h
            .handle("not_a_real_method", serde_json::json!({}))
            .await
            .expect_err("test: unknown method");
        assert!(err.contains("Unknown method"));
    }

    #[tokio::test]
    async fn handle_list_datasets_bad_params() {
        let h = NestGateJsonRpcHandler::new();
        let err = h
            .handle("list_datasets", serde_json::json!({"pool": "t"}))
            .await
            .expect_err("test: bad params");
        assert!(!err.is_empty());
    }

    #[tokio::test]
    async fn handle_health_readiness_includes_ready_flag() {
        let h = NestGateJsonRpcHandler::new();
        let v = h
            .handle("health.readiness", serde_json::json!({}))
            .await
            .expect("test: readiness");
        assert!(v.get("ready").is_some());
        assert!(v.get("status").is_some());
    }

    #[tokio::test]
    async fn handle_capabilities_alias() {
        let h = NestGateJsonRpcHandler::new();
        let v = h
            .handle("capabilities.list", serde_json::json!({}))
            .await
            .expect("test: caps");
        assert!(v.is_array() || v.as_array().is_some());
    }

    #[tokio::test]
    async fn handle_zfs_health_always_success_json() {
        let v = handle_zfs_method("zfs.health", &serde_json::json!({}))
            .await
            .expect("test: zfs.health");
        assert_eq!(v["status"], "success");
        assert!(v.get("zpool_available").is_some());
        assert!(v.get("zfs_available").is_some());
    }

    #[tokio::test]
    async fn handle_zfs_unknown_semantic_method() {
        let err = handle_zfs_method("zfs.not_implemented_xyz", &serde_json::json!({}))
            .await
            .expect_err("test: unknown zfs");
        assert!(err.contains("Unknown zfs method"));
    }

    #[tokio::test]
    async fn handle_zfs_pool_get_missing_pool_param() {
        let err = handle_zfs_method("zfs.pool.get", &serde_json::json!({}))
            .await
            .expect_err("test: missing pool");
        assert!(err.contains("missing"));
    }

    #[tokio::test]
    async fn handle_zfs_dataset_get_missing_dataset_param() {
        let err = handle_zfs_method("zfs.dataset.get", &serde_json::json!({}))
            .await
            .expect_err("test: missing dataset");
        assert!(err.contains("missing"));
    }

    #[tokio::test]
    async fn handle_list_pools_returns_json_array() {
        let h = NestGateJsonRpcHandler::new();
        let v = h
            .handle("list_pools", serde_json::json!(null))
            .await
            .expect("list_pools");
        assert!(v.is_array());
    }

    #[tokio::test]
    async fn handle_list_datasets_accepts_pool_string_param() {
        let h = NestGateJsonRpcHandler::new();
        let v = h
            .handle("list_datasets", serde_json::json!("test-pool"))
            .await
            .expect("list_datasets");
        assert!(v.is_array());
    }

    #[tokio::test]
    async fn handle_get_metrics_returns_object() {
        let h = NestGateJsonRpcHandler::new();
        let v = h
            .handle("get_metrics", serde_json::json!({}))
            .await
            .expect("get_metrics");
        assert!(v.is_object());
    }

    #[tokio::test]
    async fn handle_health_returns_status() {
        let h = NestGateJsonRpcHandler::new();
        let v = h
            .handle("health", serde_json::json!({}))
            .await
            .expect("health");
        assert!(v.get("status").is_some());
    }

    #[tokio::test]
    async fn handle_health_liveness_returns_status() {
        let h = NestGateJsonRpcHandler::new();
        let v = h
            .handle("health.liveness", serde_json::json!({}))
            .await
            .expect("health.liveness");
        assert!(v.get("status").is_some());
    }

    #[tokio::test]
    async fn handle_health_check_alias_returns_status() {
        let h = NestGateJsonRpcHandler::new();
        let v = h
            .handle("health.check", serde_json::json!({}))
            .await
            .expect("health.check");
        assert!(v.get("status").is_some());
    }

    #[tokio::test]
    async fn handle_version_returns_version_info() {
        let h = NestGateJsonRpcHandler::new();
        let v = h
            .handle("version", serde_json::json!({}))
            .await
            .expect("version");
        assert!(v.get("version").is_some() || v.is_object());
    }

    #[tokio::test]
    async fn handle_capabilities_returns_array() {
        let h = NestGateJsonRpcHandler::new();
        let v = h
            .handle("capabilities", serde_json::json!({}))
            .await
            .expect("capabilities");
        assert!(v.is_array());
    }

    #[tokio::test]
    async fn handle_zfs_pool_list_follows_zpool_availability() {
        let r = handle_zfs_method("zfs.pool.list", &serde_json::json!({})).await;
        match r {
            Ok(v) => assert_eq!(v["status"], "success"),
            Err(e) => assert!(
                e.contains("zpool") || e.contains("zfs.pool.list"),
                "unexpected: {e}"
            ),
        }
    }

    #[tokio::test]
    async fn handle_zfs_snapshot_list_missing_zfs_errors_cleanly() {
        let r = handle_zfs_method(
            "zfs.snapshot.list",
            &serde_json::json!({"dataset": "tank/fs"}),
        )
        .await;
        match r {
            Ok(v) => assert_eq!(v["status"], "success"),
            Err(e) => assert!(!e.is_empty()),
        }
    }
}
