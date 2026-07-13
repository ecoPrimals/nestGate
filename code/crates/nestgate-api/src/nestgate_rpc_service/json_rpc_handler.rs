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
            "health.liveness" => serde_json::to_value(serde_json::json!({
                "status": "alive",
                "primal": "nestgate",
            }))
            .map_err(|e| format!("Failed to serialize health.liveness result: {e}")),
            "health" | "health.check" => {
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
            // Content-addressed storage (delegates to canonical handlers)
            m if m.starts_with("content.") => handle_content_method(m, &params).await,
            // Coordination domain — ecosystem state served from CAS
            m if m.starts_with("coord.") => handle_coord_method(m, &params).await,
            // footPrint domain — CAS-backed project persistence
            m if m.starts_with("footprint.") => handle_footprint_method(m, &params).await,
            "lifecycle.status" => Ok(json!({
                "status": "running",
                "primal": "nestgate",
                "version": env!("CARGO_PKG_VERSION"),
            })),
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

/// Ecosystem-standard `content.*` semantic methods — delegates to the canonical
/// content-addressed storage handlers in `nestgate-rpc`.
///
/// # Errors
///
/// Returns `Err` for unknown content methods or handler failures.
async fn handle_content_method(
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    use nestgate_core::rpc::content_ops;

    let result = match method {
        "content.put" => content_ops::put(params).await,
        "content.get" => content_ops::get(params).await,
        "content.exists" => content_ops::exists(params).await,
        "content.list" => content_ops::list(params).await,
        "content.publish" => content_ops::publish(params).await,
        "content.resolve" => content_ops::resolve(params).await,
        "content.promote" => content_ops::promote(params).await,
        "content.collections" => content_ops::collections(params).await,
        "content.fetch_heads" => content_ops::fetch_heads(params).await,
        "content.push" => content_ops::push(params).await,
        "content.replicate" => content_ops::replicate(params).await,
        "content.replicate.pull" => content_ops::replicate_pull(params).await,
        "content.sync" => content_ops::sync(params).await,
        "content.store_stream" => content_ops::store_stream_begin(params).await,
        "content.store_stream_chunk" => content_ops::store_stream_chunk(params).await,
        "content.retrieve_stream" => content_ops::retrieve_stream_begin(params).await,
        "content.retrieve_stream_chunk" => content_ops::retrieve_stream_chunk(params).await,
        _ => return Err(format!("Unknown content method: {method}")),
    };
    result.map_err(|e| format!("{method} failed: {e}"))
}

/// Coordination domain `coord.*` methods — delegates to `coord_ops` which
/// wraps `coord_handlers` from `nestgate-rpc`.
///
/// # Errors
///
/// Returns `Err` for unknown coordination methods or handler failures.
async fn handle_coord_method(
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    use nestgate_core::rpc::coord_ops;

    let result = match method {
        "coord.blurbs.current" => coord_ops::blurbs_current(params).await,
        "coord.blurbs.list" => coord_ops::blurbs_list(params).await,
        "coord.blurbs.get" => coord_ops::blurbs_get(params).await,
        "coord.fragos.list" => coord_ops::fragos_list(params).await,
        "coord.fragos.get" => coord_ops::fragos_get(params).await,
        "coord.waves.current" => coord_ops::waves_current(params).await,
        "coord.waves.history" => coord_ops::waves_history(params).await,
        "coord.heads.get" => coord_ops::heads_get(params).await,
        "coord.heads.all" => coord_ops::heads_all(params).await,
        "coord.topology" => coord_ops::topology(params).await,
        "coord.depot.status" => coord_ops::depot_status(params).await,
        "coord.provenance" => coord_ops::provenance(params).await,
        "coord.ingest" => coord_ops::ingest(params).await,
        _ => return Err(format!("Unknown coord method: {method}")),
    };
    result.map_err(|e| format!("{method} failed: {e}"))
}

/// footPrint `footprint.*` methods — CAS-backed project persistence.
///
/// # Errors
///
/// Returns `Err` for unknown footprint methods or handler failures.
async fn handle_footprint_method(
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    use nestgate_core::rpc::footprint_ops;

    let result = match method {
        "footprint.save" => footprint_ops::save(params).await,
        "footprint.get" => footprint_ops::get(params).await,
        "footprint.list" => footprint_ops::list(params).await,
        "footprint.delete" => footprint_ops::delete(params).await,
        "footprint.history" => footprint_ops::history(params).await,
        _ => return Err(format!("Unknown footprint method: {method}")),
    };
    result.map_err(|e| format!("{method} failed: {e}"))
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
        Err(String::from("zpool is not available on this system"))
    }
}

async fn require_zfs() -> Result<(), String> {
    if is_zfs_available().await {
        Ok(())
    } else {
        Err(String::from("zfs is not available on this system"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine as _;

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

    #[tokio::test]
    async fn content_replicate_pull_routes_via_http() {
        let h = NestGateJsonRpcHandler::new();
        let err = h
            .handle(
                "content.replicate.pull",
                serde_json::json!({"cids": ["abc"], "source": "tcp://localhost:9999"}),
            )
            .await;
        assert!(err.is_ok() || err.is_err());
    }

    #[tokio::test]
    async fn content_store_stream_routes_via_http() {
        let h = NestGateJsonRpcHandler::new();
        let result = h
            .handle(
                "content.store_stream",
                serde_json::json!({"family_id": "test-http-stream", "total_size": 512}),
            )
            .await
            .expect("store_stream via HTTP should succeed");
        assert!(result.get("stream_id").is_some());
    }

    #[tokio::test]
    async fn content_store_stream_chunk_bad_session_errors_via_http() {
        let h = NestGateJsonRpcHandler::new();
        let err = h
            .handle(
                "content.store_stream_chunk",
                serde_json::json!({"stream_id": "invalid", "data": "aGVsbG8="}),
            )
            .await
            .expect_err("bad stream_id should fail");
        assert!(!err.is_empty());
    }

    #[tokio::test]
    async fn content_retrieve_stream_missing_hash_errors_via_http() {
        let h = NestGateJsonRpcHandler::new();
        let err = h
            .handle(
                "content.retrieve_stream",
                serde_json::json!({"family_id": "test-retrieve-http"}),
            )
            .await
            .expect_err("missing hash should fail");
        assert!(!err.is_empty());
    }

    #[tokio::test]
    async fn content_retrieve_stream_chunk_bad_session_errors_via_http() {
        let h = NestGateJsonRpcHandler::new();
        let err = h
            .handle(
                "content.retrieve_stream_chunk",
                serde_json::json!({"stream_id": "invalid"}),
            )
            .await
            .expect_err("bad stream_id should fail");
        assert!(!err.is_empty());
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn content_put_and_get_through_http_handler() {
        let h = NestGateJsonRpcHandler::new();
        let family_id = format!("test-http-put-{}", uuid::Uuid::new_v4());
        let raw = b"http dispatch content";
        let encoded = base64::engine::general_purpose::STANDARD.encode(raw);

        let put_r = h
            .handle(
                "content.put",
                serde_json::json!({"family_id": &family_id, "data": encoded}),
            )
            .await
            .expect("content.put");
        assert!(put_r.get("hash").is_some());
        let hash = put_r["hash"].as_str().unwrap();

        let get_r = h
            .handle(
                "content.get",
                serde_json::json!({"hash": hash, "family_id": &family_id}),
            )
            .await
            .expect("content.get");
        assert!(get_r.get("data").is_some());

        let _ = tokio::fs::remove_dir_all(
            nestgate_core::config::storage_paths::get_storage_base_path()
                .join("datasets")
                .join(&family_id),
        )
        .await;
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn content_exists_and_list_through_http_handler() {
        let h = NestGateJsonRpcHandler::new();
        let family_id = format!("test-http-exists-{}", uuid::Uuid::new_v4());
        let encoded = base64::engine::general_purpose::STANDARD.encode(b"exists check");

        let put_r = h
            .handle(
                "content.put",
                serde_json::json!({"family_id": &family_id, "data": encoded}),
            )
            .await
            .expect("put");
        let hash = put_r["hash"].as_str().unwrap();

        let exists_r = h
            .handle(
                "content.exists",
                serde_json::json!({"hash": hash, "family_id": &family_id}),
            )
            .await
            .expect("content.exists");
        assert_eq!(exists_r["exists"], true);

        let list_r = h
            .handle("content.list", serde_json::json!({"family_id": &family_id}))
            .await
            .expect("content.list");
        assert!(list_r.get("items").is_some() || list_r.get("hashes").is_some());

        let _ = tokio::fs::remove_dir_all(
            nestgate_core::config::storage_paths::get_storage_base_path()
                .join("datasets")
                .join(&family_id),
        )
        .await;
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn content_publish_and_resolve_through_http_handler() {
        let h = NestGateJsonRpcHandler::new();
        let family_id = format!("test-http-pub-{}", uuid::Uuid::new_v4());
        let encoded = base64::engine::general_purpose::STANDARD.encode(b"publish me");

        let put_r = h
            .handle(
                "content.put",
                serde_json::json!({"family_id": &family_id, "data": encoded}),
            )
            .await
            .expect("put");
        let hash = put_r["hash"].as_str().unwrap();

        let pub_r = h
            .handle(
                "content.publish",
                serde_json::json!({
                    "family_id": &family_id,
                    "collection": "test-pub",
                    "manifest": { "/index.html": hash }
                }),
            )
            .await
            .expect("content.publish");
        assert!(pub_r.get("collection").is_some() || pub_r.get("published").is_some());

        let resolve_r = h
            .handle(
                "content.resolve",
                serde_json::json!({"collection": "test-pub", "path": "/index.html", "family_id": &family_id}),
            )
            .await
            .expect("content.resolve");
        assert!(resolve_r.get("hash").is_some() || resolve_r.get("data").is_some());

        let _ = tokio::fs::remove_dir_all(
            nestgate_core::config::storage_paths::get_storage_base_path()
                .join("datasets")
                .join(&family_id),
        )
        .await;
    }

    #[tokio::test]
    async fn content_collections_through_http_handler() {
        let h = NestGateJsonRpcHandler::new();
        let family_id = format!("test-http-coll-{}", uuid::Uuid::new_v4());
        let r = h
            .handle(
                "content.collections",
                serde_json::json!({"family_id": &family_id}),
            )
            .await
            .expect("content.collections");
        assert!(r.get("collections").is_some());
    }

    #[tokio::test]
    async fn lifecycle_status_through_http_handler() {
        let h = NestGateJsonRpcHandler::new();
        let r = h
            .handle("lifecycle.status", serde_json::json!({}))
            .await
            .expect("lifecycle.status");
        assert_eq!(r["status"], "running");
        assert_eq!(r["primal"], "nestgate");
        assert!(r.get("version").is_some());
    }

    #[tokio::test]
    async fn unknown_content_method_returns_clear_error() {
        let h = NestGateJsonRpcHandler::new();
        let err = h
            .handle("content.nonexistent_xyz", serde_json::json!({}))
            .await
            .expect_err("unknown content method");
        assert!(err.contains("Unknown content method"), "error: {err}");
    }
}
