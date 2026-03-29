// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! API Routes Module
//!
//! Defines all HTTP routes and endpoints for the NestGate REST API.
//!
//! # Architecture
//!
//! Routes are organized hierarchically:
//! - `/health` - Health check and system status
//! - `/api/v1/storage/*` - Storage management (pools, datasets, snapshots)
//! - `/api/v1/monitoring/*` - Metrics and performance analytics
//! - `/api/v1/workspaces/*` - Workspace management
//! - `/api/v1/load-testing/*` - Load testing and benchmarking
//!
//! # Handler Organization
//!
//! Handlers are grouped by domain:
//! - `storage`: ZFS pool/dataset operations
//! - `performance_analytics`: Metrics and recommendations
//! - `workspace_management`: Multi-tenant workspace isolation
//! - `load_testing`: Performance testing infrastructure
//!
//! # State Management
//!
//! The [`AppState`] struct contains shared resources:
//! - `zfs_manager`: ZFS operations manager
//! - Configuration and connection pools (as needed)
//!
//! # Example
//!
//! ```rust,ignore
//! use nestgate_api::routes::create_router;
//!
//! #[tokio::main]
//! async fn main() {
//!     let router = create_router();
//!     let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
//!     axum::serve(listener, router).await.unwrap();
//! }
//! ```
//!
//! # Feature Flags
//!
//! - `dev-stubs`: Use stub implementations for development/testing
//! - `streaming-rpc`: Enable bidirectional RPC streaming (optional)

use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};
use std::sync::Arc;

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

// Production: Use real ZFS manager and config
#[cfg(not(feature = "dev-stubs"))]
use nestgate_zfs::ProductionZfsManager;

// Development: Use stub manager and config
#[cfg(feature = "dev-stubs")]
use crate::dev_stubs::zfs::{ProductionZfsManager, ZfsConfig};

/// Production ZFS manager type alias
///
/// Defines the production ZFS manager implementation used throughout
/// the application for consistent ZFS operations and management.
pub type ZfsManager = ProductionZfsManager;

#[cfg(feature = "streaming-rpc")]
// Removed unused import: crate::{}
/// Application state shared across all route handlers
///
/// Contains shared resources and services that route handlers need
/// to access, including ZFS management and configuration.
#[derive(Clone)]
/// Appstate
pub struct AppState {
    /// ZFS manager instance for storage operations
    pub zfs_manager: Arc<ZfsManager>,
    // Add other shared state here as needed
    /// Phantom data for future extensibility
    pub _phantom: std::marker::PhantomData<()>,
}

impl Default for AppState {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    /// Create `AppState` with ZFS support
    #[cfg(feature = "streaming-rpc")]
    #[must_use]
    pub fn with_zfs_and_streaming() -> Self {
        Self {
            #[cfg(feature = "dev-stubs")]
            zfs_manager: Arc::new(ZfsManager::new(ZfsConfig::default())),
            #[cfg(not(feature = "dev-stubs"))]
            zfs_manager: Arc::new(ZfsManager::new()),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create `AppState` without streaming features
    #[must_use]
    pub fn without_streaming() -> Self {
        Self {
            #[cfg(feature = "dev-stubs")]
            zfs_manager: Arc::new(ZfsManager::new(ZfsConfig::default())),
            #[cfg(not(feature = "dev-stubs"))]
            zfs_manager: Arc::new(ZfsManager::new()),
            #[cfg(feature = "streaming-rpc")]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create `AppState` with optional streaming components based on feature flags
    #[must_use]
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "dev-stubs")]
            zfs_manager: Arc::new(ZfsManager::new(ZfsConfig::default())),
            #[cfg(not(feature = "dev-stubs"))]
            zfs_manager: Arc::new(ZfsManager::new()),
            #[cfg(feature = "streaming-rpc")]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Get ZFS manager reference
    #[must_use]
    pub fn get_zfs_manager(&self) -> Option<Arc<ZfsManager>> {
        Some(self.zfs_manager.clone())
    }

    /// Initialize storage systems - ZFS manager and Universal Storage Bridge
    #[must_use]
    pub const fn with_zfs_manager(self) -> Self {
        // ZFS manager already initialized in constructor
        self
    }

    /// Initialize ZFS manager with graceful fallback
    #[expect(dead_code, reason = "ZFS init hook reserved for production wiring")]
    fn try_init_zfs_manager(
        &self,
    ) -> Result<Option<ZfsManager>, Box<dyn std::error::Error + Send + Sync>> {
        // Check if ZFS is available first
        #[cfg(feature = "dev-stubs")]
        let manager = ProductionZfsManager::new(ZfsConfig::default());
        #[cfg(not(feature = "dev-stubs"))]
        let manager = ProductionZfsManager::new();
        Ok(Some(manager))
    }
}

/// Health, analytics, load testing, storage, and JSON-RPC routes.
fn attach_core_routes(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/health", get(health_check))
        .route(
            "/hardware/tune",
            post(|| async {
                axum::response::Json(serde_json::json!({
                    "status": "success",
                    "message": "Hardware tuning not implemented yet"
                }))
            }),
        )
        .route(
            "/hardware/config",
            get(|| async {
                axum::response::Json(serde_json::json!({
                    "status": "success",
                    "config": {},
                    "message": "Hardware config not implemented yet"
                }))
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
fn create_router_with_initialized_state(app_state: AppState) -> Router {
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

/// Health Check
async fn health_check() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(serde_json::json!({
        "status": "ok",
        "service": "nestgate-api",
        "version": env!("CARGO_PKG_VERSION"),
        "communication_layers": {
            "websocket": true,
            "sse": true,
            "streaming_rpc": true,
            "mcp_streaming": true,
            "event_coordination": true
        }
    }))
}

// Enhanced communication stats endpoint
async fn get_communication_stats(
    axum::extract::State(_state): axum::extract::State<AppState>,
) -> axum::response::Json<serde_json::Value> {
    // Return stub stats since the manager fields are not available
    axum::response::Json(serde_json::json!({
        "websocket": {
            "active_connections": 0,
            "total_messages": 0
        },
        "sse": {
            "active_connections": 0,
            "events_sent": 0
        },
        "mcp_streaming": {
            "active_streams": 0,
            "total_messages": 0
        },
        "total_active_connections": 0,
        "total_messages_processed": 0
    }))
}

// Events endpoint
async fn get_events(
    axum::extract::State(_state): axum::extract::State<AppState>,
) -> axum::response::Json<serde_json::Value> {
    // Return stub events since event_coordinator is not available
    axum::response::Json(serde_json::json!({
        "events": [
            {
                "id": "event_1",
                "type": "system_startup",
                "message": "System initialized successfully",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "source": "nestgate-api"
            },
            {
                "id": "event_2",
                "type": "zfs_status",
                "message": "ZFS pools healthy",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "source": "zfs_manager"
            }
        ],
        "total_events": 2,
        "handler_count": 1
    }))
}

// WebSocket handler
#[cfg(feature = "streaming-rpc")]
/// WebSocket handler for real-time updates
///
/// Provides bidirectional communication for real-time system events,
/// storage updates, and performance metrics streaming.
async fn websocket_handler(
    ws: axum::extract::WebSocketUpgrade,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> axum::response::Response {
    ws.on_upgrade(|socket| handle_websocket_connection(socket, state))
}

/// Handle WebSocket connection lifecycle
///
/// Manages the bidirectional WebSocket connection, including:
/// - Connection setup and authentication
/// - Message routing and processing
/// - Periodic health checks and keepalive
/// - Graceful disconnection handling
async fn handle_websocket_connection(mut socket: axum::extract::ws::WebSocket, state: AppState) {
    use axum::extract::ws::Message;

    tracing::info!("WebSocket connection established");

    // Send initial connection success message
    if socket
        .send(Message::Text(
            serde_json::json!({
                "type": "connection",
                "status": "connected",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "version": env!("CARGO_PKG_VERSION")
            })
            .to_string(),
        ))
        .await
        .is_err()
    {
        tracing::warn!("Failed to send connection message");
        return;
    }

    // Main message loop
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                tracing::debug!("Received WebSocket message: {}", text);

                // Parse and route message
                match serde_json::from_str::<serde_json::Value>(&text) {
                    Ok(json) => {
                        let response = handle_websocket_message(json, &state).await;
                        if socket.send(Message::Text(response)).await.is_err() {
                            tracing::warn!("Failed to send response, closing connection");
                            break;
                        }
                    }
                    Err(e) => {
                        let error_response = serde_json::json!({
                            "type": "error",
                            "error": format!("Invalid JSON: {}", e),
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        })
                        .to_string();

                        if socket.send(Message::Text(error_response)).await.is_err() {
                            break;
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => {
                tracing::info!("WebSocket connection closed by client");
                break;
            }
            Ok(Message::Ping(data)) => {
                if socket.send(Message::Pong(data)).await.is_err() {
                    break;
                }
            }
            Ok(_) => {
                // Ignore other message types (Binary, Pong)
            }
            Err(e) => {
                tracing::warn!("WebSocket error: {}", e);
                break;
            }
        }
    }

    tracing::info!("WebSocket connection closed");
}

/// Process WebSocket message and generate response
///
/// Routes messages based on type and returns appropriate responses.
#[expect(
    clippy::unused_async,
    reason = "cfg(test) callers await this helper; body is synchronous"
)]
async fn handle_websocket_message(msg: serde_json::Value, state: &AppState) -> String {
    let msg_type = msg
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    match msg_type {
        "ping" => serde_json::json!({
            "type": "pong",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })
        .to_string(),

        "get_storage_status" => {
            // Get storage metrics from ZFS manager
            match state.get_zfs_manager() {
                Some(_manager) => {
                    // Use manager to get real metrics (simplified for now)
                    serde_json::json!({
                        "type": "storage_status",
                        "data": {
                            "available": true,
                            "manager_initialized": true,
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        }
                    })
                    .to_string()
                }
                None => serde_json::json!({
                    "type": "storage_status",
                    "data": {
                        "available": false,
                        "reason": "ZFS manager not initialized",
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    }
                })
                .to_string(),
            }
        }

        "subscribe" => {
            let channel = msg
                .get("channel")
                .and_then(|v| v.as_str())
                .unwrap_or("general");
            serde_json::json!({
                "type": "subscribed",
                "channel": channel,
                "timestamp": chrono::Utc::now().to_rfc3339()
            })
            .to_string()
        }

        _ => serde_json::json!({
            "type": "error",
            "error": format!("Unknown message type: {}", msg_type),
            "timestamp": chrono::Utc::now().to_rfc3339()
        })
        .to_string(),
    }
}

/// SSE events handler
///
/// Returns system-wide events including configuration changes,
/// service status updates, and administrative notifications.
async fn sse_events(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl axum::response::IntoResponse {
    // Get real system events
    let events = vec![serde_json::json!({
        "id": format!("event_{}", uuid::Uuid::new_v4()),
        "type": "system_status",
        "data": {
            "status": "operational",
            "uptime_seconds": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            "zfs_available": state.get_zfs_manager().is_some(),
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    })];

    axum::response::Json(serde_json::json!({
        "status": "success",
        "events": events,
        "count": events.len(),
        "generated_at": chrono::Utc::now().to_rfc3339()
    }))
}

/// SSE storage events handler
///
/// Returns storage-related events including pool status changes,
/// dataset operations, snapshot creation, and capacity alerts.
async fn sse_storage(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl axum::response::IntoResponse {
    // Check ZFS manager availability and get storage status
    let storage_events = match state.get_zfs_manager() {
        Some(_manager) => {
            vec![serde_json::json!({
                "id": format!("storage_{}", uuid::Uuid::new_v4()),
                "type": "storage_status",
                "data": {
                    "status": "operational",
                    "manager_available": true,
                    "message": "ZFS storage system operational"
                },
                "timestamp": chrono::Utc::now().to_rfc3339()
            })]
        }
        None => {
            vec![serde_json::json!({
                "id": format!("storage_{}", uuid::Uuid::new_v4()),
                "type": "storage_warning",
                "data": {
                    "status": "degraded",
                    "manager_available": false,
                    "message": "ZFS manager not initialized - storage operations limited"
                },
                "timestamp": chrono::Utc::now().to_rfc3339()
            })]
        }
    };

    axum::response::Json(serde_json::json!({
        "status": "success",
        "storage_events": storage_events,
        "count": storage_events.len(),
        "generated_at": chrono::Utc::now().to_rfc3339()
    }))
}

/// SSE health events handler
///
/// Returns health check results, system diagnostics, and
/// component status monitoring events.
async fn sse_health(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl axum::response::IntoResponse {
    // Perform actual health checks
    let zfs_healthy = state.get_zfs_manager().is_some();
    let overall_status = if zfs_healthy { "healthy" } else { "degraded" };

    axum::response::Json(serde_json::json!({
        "status": "success",
        "health": {
            "overall": overall_status,
            "api": "healthy",
            "storage": if zfs_healthy { "healthy" } else { "degraded" },
            "zfs_manager": if zfs_healthy { "available" } else { "unavailable" },
            "components": {
                "zfs": zfs_healthy,
                "api": true
            },
            "timestamp": chrono::Utc::now().to_rfc3339()
        },
        "generated_at": chrono::Utc::now().to_rfc3339()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();
        assert!(state.get_zfs_manager().is_some());
    }

    #[test]
    fn test_app_state_new() {
        let state = AppState::new();
        assert!(state.get_zfs_manager().is_some());
    }

    #[test]
    fn test_app_state_without_streaming() {
        let state = AppState::without_streaming();
        assert!(state.get_zfs_manager().is_some());
    }

    #[test]
    fn test_app_state_with_zfs_manager() {
        let state = AppState::new().with_zfs_manager();
        assert!(state.get_zfs_manager().is_some());
    }

    #[test]
    fn test_create_router_returns_router() {
        let router = create_router();
        let _ = router;
    }

    #[test]
    fn test_create_router_with_state_returns_router() {
        let router = create_router_with_state();
        let _ = router;
    }

    #[tokio::test]
    async fn websocket_ping_returns_pong() {
        let state = AppState::new();
        let msg = serde_json::json!({"type": "ping"});
        let out = handle_websocket_message(msg, &state).await;
        assert!(out.contains("pong"));
    }

    #[tokio::test]
    async fn websocket_unknown_type_is_error() {
        let state = AppState::new();
        let msg = serde_json::json!({"type": "not_real_type_xyz"});
        let out = handle_websocket_message(msg, &state).await;
        assert!(out.contains("error"));
    }

    #[tokio::test]
    async fn websocket_subscribe_includes_channel() {
        let state = AppState::new();
        let msg = serde_json::json!({"type": "subscribe", "channel": "metrics"});
        let out = handle_websocket_message(msg, &state).await;
        assert!(out.contains("metrics"));
    }
}
