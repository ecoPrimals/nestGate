// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Route registration: core API, ZFS, workspaces, and optional streaming.

use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

use crate::handlers::load_testing::{
    get_load_test_history, get_load_test_results, get_performance_baselines, start_load_test,
};
use crate::handlers::workspace_management::teams::create_team;
use crate::handlers::{
    performance_analytics::{
        get_performance_alerts, get_performance_metrics, get_performance_recommendations,
    },
    rpc_handlers::{get_protocol_capabilities, handle_jsonrpc, rpc_health},
    storage::{
        get_storage_datasets, get_storage_metrics, get_storage_pools, get_storage_snapshots,
    },
    workspace_management::{
        create_workspace, delete_workspace, get_workspace, get_workspaces, update_workspace_config,
    },
};

use super::handlers::{get_communication_stats, get_events, health_check};
use super::state::AppState;

#[cfg(feature = "streaming-rpc")]
use super::streaming::{sse_events, sse_health, sse_storage, websocket_handler};

/// Health, analytics, load testing, storage, and JSON-RPC routes.
fn attach_core_routes(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/health", get(health_check))
        .route(
            "/hardware/tune",
            post(|| async {
                (
                    axum::http::StatusCode::NOT_IMPLEMENTED,
                    axum::response::Json(serde_json::json!({
                        "error": "not_implemented",
                        "message": "Hardware tuning is not yet available"
                    })),
                )
            }),
        )
        .route(
            "/hardware/config",
            get(|| async {
                (
                    axum::http::StatusCode::NOT_IMPLEMENTED,
                    axum::response::Json(serde_json::json!({
                        "error": "not_implemented",
                        "message": "Hardware configuration is not yet available"
                    })),
                )
            }),
        )
        .route("/api/v1/communication/stats", get(get_communication_stats))
        .route("/api/v1/events", get(get_events))
        .route(
            "/api/v1/analytics/performance",
            get(get_performance_metrics),
        )
        .route("/api/v1/analytics/alerts", get(get_performance_alerts))
        .route(
            "/api/v1/analytics/recommendations",
            get(get_performance_recommendations),
        )
        .route("/api/v1/load-testing/start", post(start_load_test))
        .route("/api/v1/load-testing/results", get(get_load_test_results))
        .route("/api/v1/load-testing/history", get(get_load_test_history))
        .route(
            "/api/v1/load-testing/baselines",
            get(get_performance_baselines),
        )
        .route("/api/v1/storage/pools", get(get_storage_pools))
        .route("/api/v1/storage/datasets", get(get_storage_datasets))
        .route("/api/v1/storage/snapshots", get(get_storage_snapshots))
        .route("/api/v1/storage/metrics", get(get_storage_metrics))
        .route("/jsonrpc", post(handle_jsonrpc))
        .route(
            "/api/v1/protocol/capabilities",
            get(get_protocol_capabilities),
        )
        .route("/api/v1/rpc/health", get(rpc_health))
}

/// Universal ZFS / storage API routes.
fn attach_zfs_routes(router: Router<AppState>) -> Router<AppState> {
    router
        .route(
            "/api/v1/zfs/pools",
            get(crate::handlers::zfs::list_universal_pools),
        )
        .route("/api/v1/zfs/pools", post(crate::handlers::zfs::create_pool))
        .route(
            "/api/v1/zfs/pools/:pool_name",
            get(crate::handlers::zfs::get_universal_pool),
        )
        .route(
            "/api/v1/zfs/pools/:pool_name",
            delete(crate::handlers::zfs::delete_pool),
        )
        .route(
            "/api/v1/zfs/pools/:pool_name/scrub",
            post(crate::handlers::zfs::trigger_optimization),
        )
        .route(
            "/api/v1/zfs/datasets",
            get(crate::handlers::zfs::list_datasets),
        )
        .route(
            "/api/v1/zfs/datasets",
            post(crate::handlers::zfs::create_dataset),
        )
        .route(
            "/api/v1/zfs/datasets/:dataset_name",
            get(crate::handlers::zfs::get_dataset),
        )
        .route(
            "/api/v1/zfs/datasets/:dataset_name",
            delete(crate::handlers::zfs::delete_dataset),
        )
        .route(
            "/api/v1/zfs/datasets/:dataset_name/properties",
            get(crate::handlers::zfs::get_dataset_properties),
        )
        .route(
            "/api/v1/zfs/datasets/:dataset_name/properties",
            put(crate::handlers::zfs::set_dataset_properties),
        )
        .route(
            "/api/v1/zfs/datasets/:dataset_name/snapshots",
            get(crate::handlers::zfs::list_snapshots),
        )
        .route(
            "/api/v1/zfs/datasets/:dataset_name/snapshots",
            post(crate::handlers::zfs::create_snapshot),
        )
        .route(
            "/api/v1/zfs/datasets/:dataset_name/snapshots/:snapshot_name",
            delete(crate::handlers::zfs::delete_snapshot),
        )
        .route(
            "/api/v1/zfs/snapshots",
            get(crate::handlers::zfs::list_snapshots),
        )
        .route(
            "/api/v1/zfs/snapshots",
            post(crate::handlers::zfs::create_snapshot),
        )
        .route(
            "/api/v1/zfs/snapshots/:snapshot_name",
            delete(crate::handlers::zfs::delete_snapshot),
        )
        .route(
            "/api/v1/zfs/health",
            get(crate::handlers::zfs::get_universal_storage_health),
        )
        .route(
            "/api/v1/zfs/status",
            get(crate::handlers::zfs::get_pool_status),
        )
        .route(
            "/api/v1/zfs/optimization/analytics",
            get(crate::handlers::zfs::get_performance_analytics),
        )
        .route(
            "/api/v1/zfs/optimization/trigger",
            post(crate::handlers::zfs::trigger_optimization),
        )
        .route(
            "/api/v1/zfs/ai/tier-prediction",
            post(crate::handlers::zfs::predict_tier),
        )
}

/// Workspace and team routes.
fn attach_workspace_routes(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/api/v1/workspaces", post(create_workspace))
        .route("/api/v1/workspaces", get(get_workspaces))
        .route("/api/v1/workspaces/:workspace_id", get(get_workspace))
        .route(
            "/api/v1/workspaces/:workspace_id",
            patch(update_workspace_config),
        )
        .route("/api/v1/workspaces/:workspace_id", delete(delete_workspace))
        .route("/api/v1/teams", post(create_team))
}

/// Standard REST routes shared by [`create_router`] and [`create_router_with_initialized_state`].
fn attach_standard_routes(router: Router<AppState>) -> Router<AppState> {
    let router = attach_core_routes(router);
    let router = attach_zfs_routes(router);
    attach_workspace_routes(router)
}

/// Create a new router with default application state.
///
/// Prefer [`create_router_with_state`] when the process should use an initialized [`AppState`].
pub fn create_router() -> Router<AppState> {
    // This is a backward compatibility function that uses default state
    // In practice, you should use create_router_with_state() for proper initialization
    let router = attach_standard_routes(Router::new());

    // Add streaming routes conditionally
    #[cfg(feature = "streaming-rpc")]
    let router = router
        .route("/api/v1/communication/websocket", get(websocket_handler))
        .route("/api/v1/sse/events", get(sse_events))
        .route("/api/v1/sse/storage", get(sse_storage))
        .route("/api/v1/sse/health", get(sse_health));

    router
}

/// Create a router with initialized application state.
pub fn create_router_with_state() -> Router {
    let app_state = {
        #[cfg(feature = "streaming-rpc")]
        {
            AppState::with_zfs_and_streaming().with_zfs_manager()
        }
        #[cfg(not(feature = "streaming-rpc"))]
        {
            AppState::new().with_zfs_manager()
        }
    };
    create_router_with_initialized_state(app_state)
}

/// Creates router with initialized application state
pub fn create_router_with_initialized_state(app_state: AppState) -> Router {
    let router = attach_standard_routes(Router::new());

    // Add streaming routes conditionally
    #[cfg(feature = "streaming-rpc")]
    let router = router
        .route("/api/v1/communication/websocket", get(websocket_handler))
        .route("/api/v1/sse/events", get(sse_events))
        .route("/api/v1/sse/storage", get(sse_storage))
        .route("/api/v1/sse/health", get(sse_health));

    router.with_state(app_state)
}
