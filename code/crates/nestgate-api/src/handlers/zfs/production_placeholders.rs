// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **ZFS PRODUCTION SHIMS**
//!
//! Handlers for production builds where `dev-stubs` is not enabled: routes compile and return
//! `501 Not Implemented` with a structured body (no fake documentation URLs or placeholder copy).
//!
//! These `501` responses are **intentional**: they are the correct behavior for routes whose
//! backing implementation is not present in this build (delegation pattern), not stubs awaiting
//! implementation. For interactive ZFS over HTTP, enable the `dev-stubs` feature or call
//! `nestgate_zfs` from your integration layer.

use axum::{extract::Path, http::StatusCode, response::Json};
use serde_json::json;
use std::collections::HashMap;

/// Placeholder ZFS config for production builds
#[derive(Debug, Clone, Default)]
/// Configuration for Zfs
pub struct ZfsConfig;

/// Placeholder `ProductionZfsManager` for production builds
#[derive(Debug, Clone)]
/// Manager for `ProductionZfs` operations
pub struct ProductionZfsManager;

impl ProductionZfsManager {
    /// Create a new placeholder manager (matches stub API)
    #[must_use]
    pub const fn new(_config: ZfsConfig) -> Self {
        Self
    }
}

/// Placeholder `ZfsManager` (alias for `ProductionZfsManager`)
pub type ZfsManager = ProductionZfsManager;

/// Placeholder Zero-cost ZFS operations for production builds
#[derive(Debug, Clone, Default)]
/// Zerocostzfsoperations
pub struct ZeroCostZfsOperations;

impl ZeroCostZfsOperations {
    /// Create a new placeholder zero-cost operations
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

/// Placeholder ZFS handler implementation for production builds
///
/// This exists solely to allow compilation. All methods return "not implemented".
#[derive(Debug, Clone)]
/// Zfshandlerimpl
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

/// Response body when the ZFS HTTP surface is not compiled in (production without `dev-stubs`).
fn zfs_endpoint_disabled() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "not_implemented",
            "feature": "zfs_http",
            "details": null,
        })),
    )
}

// Re-export placeholders with same names as dev-stubs handlers

/// List universal storage pools
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
/// Returns `NOT_IMPLEMENTED` with helpful error message directing users to use `nestgate_zfs` crate directly.
pub async fn list_universal_pools() -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Create a new storage pool
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn create_pool(
    _body: Json<HashMap<String, serde_json::Value>>,
) -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Get universal pool information
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn get_universal_pool(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Delete a storage pool
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn delete_pool(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Trigger storage optimization
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn trigger_optimization(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// List storage datasets
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn list_datasets() -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Create a new dataset
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn create_dataset(
    _body: Json<HashMap<String, serde_json::Value>>,
) -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Get dataset information
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn get_dataset(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Delete a dataset
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn delete_dataset(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Get dataset properties
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn get_dataset_properties(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Set dataset properties
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn set_dataset_properties(
    _path: Path<String>,
    _body: Json<HashMap<String, serde_json::Value>>,
) -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// List storage snapshots
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn list_snapshots() -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Create a storage snapshot
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn create_snapshot(
    _body: Json<HashMap<String, serde_json::Value>>,
) -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Delete a snapshot
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn delete_snapshot(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Get universal storage health status
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn get_universal_storage_health() -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Get pool status
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn get_pool_status(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Get performance analytics
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn get_performance_analytics() -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Predict storage tier
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
pub async fn predict_tier(
    _body: Json<HashMap<String, serde_json::Value>>,
) -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

/// Get ZFS health status
///
/// **Note**: ZFS API endpoints are disabled in production builds without `dev-stubs` feature.
#[expect(
    clippy::unused_async,
    reason = "Axum route expects async handler for consistent method routing"
)]
pub async fn get_zfs_health() -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Path;

    fn assert_not_implemented_json(body: &serde_json::Value) {
        assert_eq!(body["error"].as_str(), Some("not_implemented"));
        assert_eq!(body["feature"].as_str(), Some("zfs_http"));
        assert!(body["details"].is_null());
    }

    #[tokio::test]
    async fn list_universal_pools_not_implemented() {
        let (code, Json(v)) = list_universal_pools().await;
        assert_eq!(code, StatusCode::NOT_IMPLEMENTED);
        assert_not_implemented_json(&v);
    }

    #[tokio::test]
    async fn create_pool_not_implemented() {
        let (code, Json(v)) = create_pool(Json(HashMap::new())).await;
        assert_eq!(code, StatusCode::NOT_IMPLEMENTED);
        assert_not_implemented_json(&v);
    }

    #[tokio::test]
    async fn get_universal_pool_not_implemented() {
        let (code, Json(v)) = get_universal_pool(Path("p".to_string())).await;
        assert_eq!(code, StatusCode::NOT_IMPLEMENTED);
        assert_not_implemented_json(&v);
    }

    #[tokio::test]
    async fn predict_tier_not_implemented() {
        let (code, _) = predict_tier(Json(HashMap::new())).await;
        assert_eq!(code, StatusCode::NOT_IMPLEMENTED);
    }

    #[tokio::test]
    async fn placeholder_types_construct() {
        let _ = ZfsConfig;
        let _ = ProductionZfsManager::new(ZfsConfig);
        let _ = ZeroCostZfsOperations::new();
        let _ = ZfsHandlerImpl;
    }
}
