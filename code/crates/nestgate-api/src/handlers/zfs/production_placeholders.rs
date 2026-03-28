// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **ZFS PRODUCTION PLACEHOLDERS**
//!
//! These are placeholder handlers for production builds where `dev-stubs` is not enabled.
//! They return helpful error messages directing users to use `nestgate_zfs` crate directly.
//!
//! **⚠️ IMPORTANT ⚠️**
//!
//! These are NOT functional handlers - they exist solely to allow compilation
//! without `dev-stubs` feature. For production ZFS operations, use `nestgate_zfs` crate.

use axum::{extract::Path, http::StatusCode, response::Json};
use serde_json::json;
use std::collections::HashMap;

/// Placeholder ZFS config for production builds
#[derive(Debug, Clone, Default)]
/// Configuration for Zfs
pub struct ZfsConfig;

/// Placeholder `ProductionZfsManager` for production builds
#[derive(Debug, Clone)]
/// Manager for ProductionZfs operations
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

/// Placeholder response for disabled ZFS API endpoints
fn zfs_endpoint_disabled() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "ZFS API endpoints are disabled in production builds",
            "message": "For production ZFS operations, use nestgate_zfs crate directly",
            "recommendation": "Enable 'dev-stubs' feature for development ZFS API",
            "documentation": "https://docs.nestgate.io/zfs-integration"
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
pub async fn get_zfs_health() -> (StatusCode, Json<serde_json::Value>) {
    zfs_endpoint_disabled()
}
