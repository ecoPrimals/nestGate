// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **ZFS production HTTP handlers (no `dev-stubs`)**
//!
//! Read-only routes call [`nestgate_zfs::command::ZfsOperations`] or [`nestgate_zfs::native`]
//! helpers when `zpool` / `zfs` userland is available. When tools are missing, handlers return
//! `503` with a structured `zfs_unavailable` body. Routes that intentionally omit HTTP wiring
//! return `501` with `not_implemented` and a message pointing to the CLI or lower-level APIs.
//!
//! The types [`ZfsConfig`], [`ProductionZfsManager`], [`ZeroCostZfsOperations`], and
//! [`ZfsHandlerImpl`] exist for API compatibility with other build configurations; they are not
//! used by the async handler functions in this module, which call into `nestgate_zfs` directly.

use axum::{extract::Path, http::StatusCode, response::Json};
use nestgate_zfs::command::ZfsOperations;
use nestgate_zfs::native::{get_zfs_version, is_zfs_available, is_zpool_available};
use serde_json::json;
use std::collections::HashMap;

/// Compatibility-only ZFS config (unused by handlers here; kept for type parity with stub builds).
#[derive(Debug, Clone, Default)]
pub struct ZfsConfig;

/// Compatibility-only manager (unused by handlers here; kept for type parity with stub builds).
#[derive(Debug, Clone)]
pub struct ProductionZfsManager;

impl ProductionZfsManager {
    /// Create a new placeholder manager (matches stub API)
    #[must_use]
    pub const fn new(_config: ZfsConfig) -> Self {
        Self
    }
}

/// Alias for [`ProductionZfsManager`] (compatibility with alternate handler modules).
pub type ZfsManager = ProductionZfsManager;

/// Compatibility-only zero-cost operations type (handlers use [`ZfsOperations`] instead).
#[derive(Debug, Clone, Default)]
pub struct ZeroCostZfsOperations;

impl ZeroCostZfsOperations {
    /// Create a new placeholder zero-cost operations
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

/// Compatibility-only handler struct (HTTP entry points in this module are free functions that
/// call `nestgate_zfs` directly).
#[derive(Debug, Clone)]
pub struct ZfsHandlerImpl;

impl ZfsHandlerImpl {
    /// Create a new placeholder handler
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for ZfsHandlerImpl {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

fn zfs_unavailable(message: impl std::fmt::Display) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({
            "status": "error",
            "error": "zfs_unavailable",
            "message": message.to_string(),
        })),
    )
}

fn zfs_operation_failed(message: impl std::fmt::Display) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "status": "error",
            "error": "zfs_operation_failed",
            "message": message.to_string(),
        })),
    )
}

fn not_implemented_http(message: impl std::fmt::Display) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "status": "error",
            "error": "not_implemented",
            "message": message.to_string(),
        })),
    )
}

/// Lists pools via [`ZfsOperations::list_pools`].
pub async fn list_universal_pools() -> (StatusCode, Json<serde_json::Value>) {
    if !is_zpool_available().await {
        return zfs_unavailable(
            "zpool is not available on this system; install ZFS userland tools to list pools.",
        );
    }
    let ops = ZfsOperations::new();
    match ops.list_pools().await {
        Ok(pools) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "pools": pools,
            })),
        ),
        Err(e) => zfs_operation_failed(e),
    }
}

/// Pool creation is not implemented over HTTP (returns `501`).
// TODO: Wire to a validated `zpool create` flow or an internal automation API when product requirements allow.
pub async fn create_pool(
    _body: Json<HashMap<String, serde_json::Value>>,
) -> (StatusCode, Json<serde_json::Value>) {
    not_implemented_http(
        "HTTP pool creation is not implemented; use the CLI or nestgate-zfs pool setup APIs.",
    )
}

/// Pool status via [`ZfsOperations::pool_status`].
pub async fn get_universal_pool(path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    let pool_name = path.0;
    if !is_zpool_available().await {
        return zfs_unavailable(
            "zpool is not available on this system; install ZFS userland tools to query pool status.",
        );
    }
    let ops = ZfsOperations::new();
    match ops.pool_status(&pool_name).await {
        Ok(status) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "pool": pool_name,
                "pool_status": status,
            })),
        ),
        Err(e) => zfs_operation_failed(e),
    }
}

/// Pool destruction is not implemented over HTTP (returns `501`).
// TODO: Optional future: gated destroy behind strong auth and confirmation tokens.
pub async fn delete_pool(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    not_implemented_http(
        "HTTP pool destruction is not implemented for safety; use `zpool destroy` or automation APIs.",
    )
}

/// Storage optimization trigger is not implemented over HTTP (returns `501`).
// TODO: Map to scrub/resilver/trim or pool maintenance workflows when defined.
pub async fn trigger_optimization(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    not_implemented_http("Storage optimization trigger is not implemented over HTTP yet.")
}

/// Lists datasets via [`ZfsOperations::list_datasets`].
pub async fn list_datasets() -> (StatusCode, Json<serde_json::Value>) {
    if !is_zfs_available().await {
        return zfs_unavailable(
            "zfs is not available on this system; install ZFS userland tools to list datasets.",
        );
    }
    let ops = ZfsOperations::new();
    match ops.list_datasets(None).await {
        Ok(datasets) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "datasets": datasets,
            })),
        ),
        Err(e) => zfs_operation_failed(e),
    }
}

/// Dataset creation is not implemented over HTTP (returns `501`).
// TODO: Wire to `zfs create` with validated properties when HTTP contract is specified.
pub async fn create_dataset(
    _body: Json<HashMap<String, serde_json::Value>>,
) -> (StatusCode, Json<serde_json::Value>) {
    not_implemented_http(
        "HTTP dataset creation is not implemented; use nestgate-zfs or `zfs create` directly.",
    )
}

/// Resolves one dataset by scanning [`ZfsOperations::list_datasets`] (returns `404` if missing).
pub async fn get_dataset(path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    let name = path.0;
    if !is_zfs_available().await {
        return zfs_unavailable(
            "zfs is not available on this system; install ZFS userland tools to query datasets.",
        );
    }
    let ops = ZfsOperations::new();
    match ops.list_datasets(None).await {
        Ok(datasets) => {
            if let Some(ds) = datasets.into_iter().find(|d| d.name == name) {
                (
                    StatusCode::OK,
                    Json(json!({
                        "status": "success",
                        "dataset": ds,
                    })),
                )
            } else {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "status": "error",
                        "error": "dataset_not_found",
                        "message": format!("No dataset named {name:?}"),
                    })),
                )
            }
        }
        Err(e) => zfs_operation_failed(e),
    }
}

/// Dataset destruction is not implemented over HTTP (returns `501`).
// TODO: Wire to `zfs destroy` with safety checks when HTTP contract is specified.
pub async fn delete_dataset(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    not_implemented_http(
        "HTTP dataset destruction is not implemented; use `zfs destroy` or nestgate-zfs dataset APIs.",
    )
}

/// Dataset `zfs get` listing is not implemented over HTTP (returns `501`).
// TODO: Expose `zfs get -H` / parsed properties via nestgate-zfs.
pub async fn get_dataset_properties(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    not_implemented_http(
        "HTTP dataset property listing is not implemented; use `zfs get` or extend nestgate-zfs::command.",
    )
}

/// Dataset property updates are not implemented over HTTP (returns `501`).
// TODO: Expose validated `zfs set` when HTTP contract is specified.
pub async fn set_dataset_properties(
    _path: Path<String>,
    _body: Json<HashMap<String, serde_json::Value>>,
) -> (StatusCode, Json<serde_json::Value>) {
    not_implemented_http(
        "HTTP dataset property updates are not implemented; use `zfs set` or nestgate-zfs.",
    )
}

/// Lists snapshots via [`ZfsOperations::list_snapshots`].
pub async fn list_snapshots() -> (StatusCode, Json<serde_json::Value>) {
    if !is_zfs_available().await {
        return zfs_unavailable(
            "zfs is not available on this system; install ZFS userland tools to list snapshots.",
        );
    }
    let ops = ZfsOperations::new();
    match ops.list_snapshots(None).await {
        Ok(snapshots) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "snapshots": snapshots,
            })),
        ),
        Err(e) => zfs_operation_failed(e),
    }
}

/// Snapshot creation is not implemented over HTTP (returns `501`).
// TODO: Delegate to [`nestgate_zfs::command::ZfsOperations::create_snapshot`] with validation.
pub async fn create_snapshot(
    _body: Json<HashMap<String, serde_json::Value>>,
) -> (StatusCode, Json<serde_json::Value>) {
    not_implemented_http(
        "HTTP snapshot creation is not implemented; use `zfs snapshot` or nestgate-zfs::command::ZfsOperations::create_snapshot.",
    )
}

/// Snapshot deletion is not implemented over HTTP (returns `501`).
// TODO: Wire to `zfs destroy` for snapshots when HTTP contract is specified.
pub async fn delete_snapshot(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    not_implemented_http(
        "HTTP snapshot deletion is not implemented; use `zfs destroy` for snapshots.",
    )
}

/// Pool health summary from [`ZfsOperations::list_pools`] (flags pools whose health is not online-like).
pub async fn get_universal_storage_health() -> (StatusCode, Json<serde_json::Value>) {
    if !is_zpool_available().await {
        return zfs_unavailable(
            "zpool is not available on this system; cannot assess storage health.",
        );
    }
    let ops = ZfsOperations::new();
    match ops.list_pools().await {
        Ok(pools) => {
            let unhealthy: Vec<&str> = pools
                .iter()
                .filter(|p| {
                    let h = p.health.to_ascii_lowercase();
                    !(h.contains("online") || h == "ok" || h == "healthy")
                })
                .map(|p| p.name.as_str())
                .collect();
            (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "pool_count": pools.len(),
                    "pools_unhealthy": unhealthy,
                    "pools": pools,
                })),
            )
        }
        Err(e) => zfs_operation_failed(e),
    }
}

/// Same as [`get_universal_pool`].
pub async fn get_pool_status(path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    get_universal_pool(path).await
}

/// Performance analytics are not implemented over HTTP (returns `501`).
// TODO: Surface arcstat / pool IO stats or nestgate performance engine metrics when available.
pub async fn get_performance_analytics() -> (StatusCode, Json<serde_json::Value>) {
    not_implemented_http("ZFS performance analytics over HTTP is not implemented yet.")
}

/// Tier prediction is not implemented over HTTP (returns `501`).
// TODO: Integrate placement / tier policy engine when available.
pub async fn predict_tier(
    _body: Json<HashMap<String, serde_json::Value>>,
) -> (StatusCode, Json<serde_json::Value>) {
    not_implemented_http("Storage tier prediction over HTTP is not implemented yet.")
}

/// Reports `zfs` userland availability via [`get_zfs_version`].
pub async fn get_zfs_health() -> (StatusCode, Json<serde_json::Value>) {
    if !is_zfs_available().await {
        return zfs_unavailable("zfs is not available on this system");
    }
    match get_zfs_version().await {
        Ok(version) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "zfs_version": version,
            })),
        ),
        Err(e) => zfs_unavailable(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Path;
    use nestgate_zfs::native::is_zpool_available;

    #[tokio::test]
    async fn list_universal_pools_delegates_or_reports_unavailable() {
        let (code, Json(v)) = list_universal_pools().await;
        if is_zpool_available().await {
            assert_eq!(code, StatusCode::OK);
            assert_eq!(v["status"], "success");
            assert!(v.get("pools").is_some());
        } else {
            assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
            assert_eq!(v["error"], "zfs_unavailable");
        }
    }

    #[tokio::test]
    async fn create_pool_not_implemented() {
        let (code, Json(v)) = create_pool(Json(HashMap::new())).await;
        assert_eq!(code, StatusCode::NOT_IMPLEMENTED);
        assert_eq!(v["error"], "not_implemented");
    }

    #[tokio::test]
    async fn get_universal_pool_delegates_or_unavailable() {
        let (code, Json(v)) = get_universal_pool(Path("nonexistent_pool_xyz".to_string())).await;
        if is_zpool_available().await {
            assert_ne!(code, StatusCode::SERVICE_UNAVAILABLE);
            if code == StatusCode::OK {
                assert_eq!(v["status"], "success");
            } else {
                assert_eq!(v["error"], "zfs_operation_failed");
            }
        } else {
            assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
            assert_eq!(v["error"], "zfs_unavailable");
        }
    }

    #[tokio::test]
    async fn predict_tier_not_implemented() {
        let (code, Json(v)) = predict_tier(Json(HashMap::new())).await;
        assert_eq!(code, StatusCode::NOT_IMPLEMENTED);
        assert_eq!(v["error"], "not_implemented");
    }

    #[tokio::test]
    async fn placeholder_types_construct() {
        let _ = ZfsConfig;
        let _ = ProductionZfsManager::new(ZfsConfig);
        let _ = ZeroCostZfsOperations::new();
        let _ = ZfsHandlerImpl;
    }
}
