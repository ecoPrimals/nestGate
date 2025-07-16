use axum::{
    extract::State,
    response::Json,
    routing::{delete, get, patch, post},
    Router,
};
use serde_json::json;
use std::sync::Arc;

pub mod hardware_tuning;

use crate::{
    event_coordination::EventCoordinator,
    handlers::{
        load_testing::{
            get_load_test_history, get_load_test_results, get_performance_baselines,
            start_load_test,
        },
        performance_analytics::{
            get_performance_alerts, get_performance_metrics, get_performance_recommendations,
        },
        storage::{
            get_storage_datasets, get_storage_metrics, get_storage_pools, get_storage_snapshots,
        },
        workspace_management::{
            create_team, create_workspace, delete_workspace, get_teams, get_workspace,
            get_workspaces, update_workspace_config,
        },
        zfs::{
            create_zfs_snapshot, get_zfs_dataset, get_zfs_datasets, get_zfs_pool, get_zfs_pools,
            get_zfs_snapshots,
        },
    },
    mcp_streaming::McpStreamingManager,
};

#[cfg(feature = "streaming-rpc")]
use crate::{
    sse::{sse_events, sse_health, sse_storage, SseManager},
    websocket::WebSocketManager,
};

/// Enhanced application state for the API with all communication layers
#[derive(Clone)]
pub struct AppState {
    #[cfg(feature = "streaming-rpc")]
    pub websocket_manager: WebSocketManager,
    pub mcp_streaming_manager: McpStreamingManager,
    pub event_coordinator: EventCoordinator,
    #[cfg(feature = "streaming-rpc")]
    pub sse_manager: Arc<SseManager>,
    pub api_state: Arc<crate::byob::ApiState>,
    pub hardware_tuning_service: Arc<crate::handlers::hardware_tuning::HardwareTuningHandler>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    #[cfg(feature = "streaming-rpc")]
    pub fn new(
        websocket_manager: WebSocketManager,
        mcp_streaming_manager: McpStreamingManager,
        event_coordinator: EventCoordinator,
    ) -> Self {
        Self {
            websocket_manager,
            mcp_streaming_manager,
            event_coordinator,
            sse_manager: Arc::new(SseManager::new()),
            api_state: Arc::new(crate::byob::ApiState::new()),
            hardware_tuning_service: Arc::new(
                crate::handlers::hardware_tuning::HardwareTuningHandler::new(),
            ),
        }
    }

    #[cfg(not(feature = "streaming-rpc"))]
    pub fn new() -> Self {
        Self {
            mcp_streaming_manager: McpStreamingManager::new(),
            event_coordinator: EventCoordinator::new(),
            api_state: Arc::new(crate::byob::ApiState::new()),
            hardware_tuning_service: Arc::new(
                crate::handlers::hardware_tuning::HardwareTuningHandler::new(),
            ),
        }
    }

    /// Enhanced constructor with SSE manager
    #[cfg(feature = "streaming-rpc")]
    pub fn with_sse_manager(
        websocket_manager: WebSocketManager,
        mcp_streaming_manager: McpStreamingManager,
        event_coordinator: EventCoordinator,
        sse_manager: SseManager,
    ) -> Self {
        Self {
            websocket_manager,
            mcp_streaming_manager,
            event_coordinator,
            sse_manager: Arc::new(sse_manager),
            api_state: Arc::new(crate::byob::ApiState::new()),
            hardware_tuning_service: Arc::new(
                crate::handlers::hardware_tuning::HardwareTuningHandler::new(),
            ),
        }
    }
}

pub fn create_router() -> Router {
    let router = Router::new()
        .route("/health", get(health_check))

        // Hardware tuning routes
        .route("/api/v1/hardware/tune", post(hardware_tuning::auto_tune))
        .route("/api/v1/hardware/config", get(hardware_tuning::get_config))

        // Communication routes
        .route("/api/v1/communication/stats", get(get_communication_stats))
        .route("/api/v1/events", get(get_events))

        // Performance analytics routes
        .route("/api/v1/analytics/performance", get(get_performance_metrics))
        .route("/api/v1/analytics/alerts", get(get_performance_alerts))
        .route("/api/v1/analytics/recommendations", get(get_performance_recommendations))

        // Load testing routes
        .route("/api/v1/load-testing/start", post(start_load_test))
        .route("/api/v1/load-testing/results", get(get_load_test_results))
        .route("/api/v1/load-testing/history", get(get_load_test_history))
        .route("/api/v1/load-testing/baselines", get(get_performance_baselines))

        // Storage routes
        .route("/api/v1/storage/pools", get(get_storage_pools))
        .route("/api/v1/storage/datasets", get(get_storage_datasets))
        .route("/api/v1/storage/snapshots", get(get_storage_snapshots))
        .route("/api/v1/storage/metrics", get(get_storage_metrics))

        // ZFS routes
        .route("/api/v1/zfs/pools", get(get_zfs_pools))
        .route("/api/v1/zfs/pools/:pool_name", get(get_zfs_pool))
        .route("/api/v1/zfs/datasets", get(get_zfs_datasets))
        .route("/api/v1/zfs/datasets/:dataset_name", get(get_zfs_dataset))
        .route("/api/v1/zfs/snapshots", get(get_zfs_snapshots))
        .route("/api/v1/zfs/snapshots", post(create_zfs_snapshot))

        // Workspace management routes
        .route("/api/v1/workspaces", get(get_workspaces))
        .route("/api/v1/workspaces", post(create_workspace))
        .route("/api/v1/workspaces/:workspace_id", get(get_workspace))
        .route("/api/v1/workspaces/:workspace_id", delete(delete_workspace))
        .route("/api/v1/workspaces/:workspace_id/config", patch(update_workspace_config))

        // Team management routes
        .route("/api/v1/teams", get(get_teams))
        .route("/api/v1/teams", post(create_team));

    // Add streaming routes conditionally
    #[cfg(feature = "streaming-rpc")]
    let router = router
        .route("/api/v1/communication/websocket", get(websocket_handler))
        .route("/api/v1/sse/events", get(sse_events))
        .route("/api/v1/sse/storage", get(sse_storage))
        .route("/api/v1/sse/health", get(sse_health));

    router.with_state({
        #[cfg(feature = "streaming-rpc")]
        {
            AppState::new(
                WebSocketManager::new(),
                McpStreamingManager::new(),
                EventCoordinator::new(),
            )
        }
        #[cfg(not(feature = "streaming-rpc"))]
        {
            AppState::new()
        }
    })
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
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
async fn get_communication_stats(State(state): State<AppState>) -> Json<serde_json::Value> {
    #[cfg(feature = "streaming-rpc")]
    let websocket_stats = state.websocket_manager.get_stats();
    #[cfg(not(feature = "streaming-rpc"))]
    let websocket_stats = serde_json::json!({
        "active_connections": 0,
        "messages_sent": 0,
        "status": "disabled"
    });

    let stream_stats = state.mcp_streaming_manager.get_stats();
    let event_stats = state.event_coordinator.get_stats().await;

    #[cfg(feature = "streaming-rpc")]
    let sse_stats = state.sse_manager.get_stats().await;
    #[cfg(not(feature = "streaming-rpc"))]
    let sse_stats = serde_json::json!({
        "active_connections": 0,
        "events_sent": 0,
        "status": "disabled"
    });

    // Calculate totals based on feature flags
    let total_active_connections = {
        #[cfg(feature = "streaming-rpc")]
        {
            state.websocket_manager.get_stats().active_connections
                + state.sse_manager.get_stats().await.active_connections
        }
        #[cfg(not(feature = "streaming-rpc"))]
        {
            0
        }
    };

    let total_messages_sent = {
        #[cfg(feature = "streaming-rpc")]
        {
            state.websocket_manager.get_stats().messages_sent
                + state.sse_manager.get_stats().await.events_sent
        }
        #[cfg(not(feature = "streaming-rpc"))]
        {
            0
        }
    };

    Json(json!({
        "websocket": websocket_stats,
        "mcp_streaming": stream_stats,
        "event_coordination": event_stats,
        "sse": sse_stats,
        "total_active_connections": total_active_connections,
        "total_messages_sent": total_messages_sent
    }))
}

// Events endpoint
async fn get_events(State(state): State<AppState>) -> Json<serde_json::Value> {
    let handlers = state.event_coordinator.list_handlers().await;
    let handler_count = state.event_coordinator.get_handler_count().await;
    let event_count = state.event_coordinator.get_event_count().await;

    Json(json!({
        "handlers": handlers,
        "handler_count": handler_count,
        "event_count": event_count
    }))
}

// WebSocket handler
#[cfg(feature = "streaming-rpc")]
async fn websocket_handler(
    State(state): State<AppState>,
    ws: axum::extract::WebSocketUpgrade,
    query: axum::extract::Query<crate::websocket::ConnectionParams>,
) -> axum::response::Response {
    // Use the WebSocket manager to handle the upgrade
    state
        .websocket_manager
        .handle_websocket_upgrade(ws, query.0)
        .await
}
