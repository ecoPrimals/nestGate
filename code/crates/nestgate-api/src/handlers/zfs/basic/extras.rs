// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Honest 501 endpoints for ZFS features not yet wired.
//!
//! Each handler returns `501 NOT IMPLEMENTED` with a JSON body describing
//! what it *will* do once the backing ZFS integration is complete.

use crate::routes::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{Value, json};
use tracing::info;

fn not_implemented(feature: &str) -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "status": "not_implemented",
            "feature": feature,
            "message": format!("{feature} is not yet wired to a real ZFS backend"),
        })),
    )
}

/// Performance analytics — not yet backed by real ZFS metrics.
pub async fn get_performance_analytics(
    State(_state): State<AppState>,
) -> (StatusCode, Json<Value>) {
    info!("API: get_performance_analytics — not implemented");
    not_implemented("zfs.performance_analytics")
}

/// Trigger optimization — not yet backed by a real ZFS optimizer.
pub async fn trigger_optimization(State(_state): State<AppState>) -> (StatusCode, Json<Value>) {
    info!("API: trigger_optimization — not implemented");
    not_implemented("zfs.optimization")
}

/// Delete dataset — not yet wired to `zfs destroy`.
pub async fn delete_dataset(
    State(_state): State<AppState>,
    Path(dataset_name): Path<String>,
) -> (StatusCode, Json<Value>) {
    info!("API: delete_dataset({dataset_name}) — not implemented");
    not_implemented("zfs.delete_dataset")
}

/// Get dataset properties — not yet backed by `zfs get`.
pub async fn get_dataset_properties(
    State(_state): State<AppState>,
    Path(dataset_name): Path<String>,
) -> (StatusCode, Json<Value>) {
    info!("API: get_dataset_properties({dataset_name}) — not implemented");
    not_implemented("zfs.dataset_properties.get")
}

/// Set dataset properties — not yet backed by `zfs set`.
pub async fn set_dataset_properties(
    State(_state): State<AppState>,
    Path(dataset_name): Path<String>,
    Json(_properties): Json<std::collections::HashMap<String, String>>,
) -> (StatusCode, Json<Value>) {
    info!("API: set_dataset_properties({dataset_name}) — not implemented");
    not_implemented("zfs.dataset_properties.set")
}

/// Delete snapshot — not yet wired to `zfs destroy`.
pub async fn delete_snapshot(
    State(_state): State<AppState>,
    Path(snapshot_name): Path<String>,
) -> (StatusCode, Json<Value>) {
    info!("API: delete_snapshot({snapshot_name}) — not implemented");
    not_implemented("zfs.delete_snapshot")
}

/// Pool status — not yet backed by `zpool status`.
pub async fn get_pool_status(State(_state): State<AppState>) -> (StatusCode, Json<Value>) {
    info!("API: get_pool_status — not implemented");
    not_implemented("zfs.pool_status")
}

/// Tier prediction — not yet backed by a real ML/heuristic model.
pub async fn predict_tier(
    State(_state): State<AppState>,
    Json(_request): Json<std::collections::HashMap<String, String>>,
) -> (StatusCode, Json<Value>) {
    info!("API: predict_tier — not implemented");
    not_implemented("zfs.predict_tier")
}
