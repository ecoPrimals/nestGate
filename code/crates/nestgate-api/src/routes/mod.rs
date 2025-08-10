use axum::{
    extract::State,
    response::Json,
    routing::{delete, get, patch, post, put},
    Router,
};
use serde_json::json;
use std::sync::Arc;

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
            create_team, create_workspace, delete_workspace, get_workspace, get_workspaces,
            update_workspace_config,
        },
        zfs::{
            create_dataset_snapshot, create_zfs_dataset, create_zfs_pool, create_zfs_snapshot,
            delete_dataset_snapshot, delete_zfs_dataset, delete_zfs_pool, delete_zfs_snapshot,
            get_dataset_properties, get_zfs_dataset, get_zfs_datasets, get_zfs_health,
            get_zfs_optimization_analytics, get_zfs_pool, get_zfs_pools, get_zfs_snapshots,
            get_zfs_status, list_dataset_snapshots, predict_zfs_tier, scrub_zfs_pool,
            set_dataset_properties, trigger_zfs_optimization,
        },
    },
    mcp_streaming::McpStreamingManager,
};

// Optional ZFS imports - graceful degradation if not available
use nestgate_zfs::ZfsManager;

#[cfg(feature = "streaming-rpc")]
use crate::{
    sse::{sse_events, sse_health, sse_storage, SseManager},
    websocket::WebSocketManager,
};

/// Enhanced application state for the API with all communication layers
#[derive(Clone)]
pub struct AppState {
    #[cfg(feature = "streaming-rpc")]
    /// WebSocket connection manager for real-time communication
    pub websocket_manager: WebSocketManager,
    /// MCP streaming manager for protocol communication
    pub mcp_streaming_manager: McpStreamingManager,
    /// Event coordination service for system-wide events
    pub event_coordinator: EventCoordinator,
    #[cfg(feature = "streaming-rpc")]
    /// Server-Sent Events manager for streaming updates
    pub sse_manager: Arc<SseManager>,
    // BYOB API state integration planned for future release
    /// Hardware tuning service adapter
    pub hardware_tuning_service: Arc<crate::handlers::hardware_tuning::HardwareTuningAdapter>,
    /// Optional ZFS manager for universal storage operations
    pub zfs_manager: Option<Arc<ZfsManager>>,
    /// Universal Storage Bridge for storage-agnostic operations
    pub universal_storage_bridge: Option<Arc<crate::handlers::zfs::UniversalStorageBridge>>,
}

impl Default for AppState {
    fn default() -> Self {
        #[cfg(feature = "streaming-rpc")]
        {
            Self::new_with_streaming(
                WebSocketManager::new(),
                McpStreamingManager::new(),
                EventCoordinator::new(),
            )
        }
        #[cfg(not(feature = "streaming-rpc"))]
        {
            Self {
                mcp_streaming_manager: McpStreamingManager::new(),
                event_coordinator: EventCoordinator::new(),
                // BYOB API state integration planned for future release
                hardware_tuning_service: Arc::new(
                    crate::handlers::hardware_tuning::HardwareTuningHandler::new(),
                ),
                zfs_manager: None, // Will be initialized if ZFS is available
                universal_storage_bridge: None, // Will be initialized for storage-agnostic operations
            }
        }
    }
}

impl AppState {
    #[cfg(feature = "streaming-rpc")]
    /// Create new application state with streaming services enabled
    pub fn new_with_streaming(
        websocket_manager: WebSocketManager,
        mcp_streaming_manager: McpStreamingManager,
        event_coordinator: EventCoordinator,
    ) -> Self {
        Self {
            websocket_manager,
            mcp_streaming_manager,
            event_coordinator,
            sse_manager: Arc::new(SseManager::new()),
            // BYOB API state integration planned for future release
            hardware_tuning_service: Arc::new(
                crate::handlers::hardware_tuning::HardwareTuningAdapter::new(
                    Arc::new(nestgate_core::ecosystem_integration::universal_adapter::adapter::UniversalAdapter::new(
                        nestgate_core::ecosystem_integration::universal_adapter::config::AdapterConfig::default()
                    )),
                    "hardware-tuning".to_string()
                ),
            ),
            zfs_manager: None, // Will be initialized if ZFS is available
            universal_storage_bridge: None, // Will be initialized for storage-agnostic operations
        }
    }

    /// Initialize storage systems - ZFS manager and Universal Storage Bridge
    pub async fn with_zfs_manager(mut self) -> Self {
        // Try to initialize ZFS manager, but continue gracefully if it fails
        match self.try_init_zfs_manager().await {
            Ok(Some(zfs_manager)) => {
                tracing::info!("ZFS manager initialized successfully");
                self.zfs_manager = Some(Arc::new(zfs_manager));
            }
            Ok(None) => {
                tracing::info!("ZFS not available, continuing without ZFS manager");
            }
            Err(e) => {
                tracing::warn!("Failed to initialize ZFS manager: {}", e);
                // Continue without ZFS manager - graceful degradation
            }
        }

        // Initialize Universal Storage Bridge for storage-agnostic operations
        match crate::handlers::zfs::UniversalStorageBridge::new().await {
            Ok(mut bridge) => {
                // Detect and configure the best available storage backend
                match bridge.detect_best_backend().await {
                    Ok(backend) => {
                        tracing::info!(
                            "✅ Universal Storage Bridge initialized with backend: {}",
                            backend
                        );
                        self.universal_storage_bridge = Some(Arc::new(bridge));
                    }
                    Err(e) => {
                        tracing::warn!(
                            "⚠️ Universal Storage Bridge backend detection failed: {}",
                            e
                        );
                        // Still initialize the bridge - it can use fallback modes
                        self.universal_storage_bridge = Some(Arc::new(bridge));
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Failed to initialize Universal Storage Bridge: {}", e);
                // Continue without bridge - graceful degradation
            }
        }

        self
    }

    /// Try to initialize ZFS manager
    async fn try_init_zfs_manager(&self) -> Result<Option<ZfsManager>, Box<dyn std::error::Error>> {
        // Check if ZFS is available first
        if !nestgate_zfs::is_zfs_available().await {
            return Ok(None);
        }

        // Try to create ZFS config
        let config = nestgate_zfs::config::ZfsConfig::default();

        // Try to create ZFS manager
        match ZfsManager::new(config).await {
            Ok(manager) => Ok(Some(manager)),
            Err(e) => Err(Box::new(e)),
        }
    }

    /// Get ZFS manager or None if not available
    pub fn get_zfs_manager(&self) -> Option<Arc<ZfsManager>> {
        self.zfs_manager.clone()
    }

    /// Create new AppState (for backward compatibility)
    pub fn new() -> Self {
        Self::default()
    }
}

/// Create a new router with default application state
pub fn create_router() -> Router<AppState> {
    // This is a backward compatibility function that uses default state
    // In practice, you should use create_router_with_state() for proper initialization

    let router = Router::new()
        .route("/health", get(health_check))

        // Hardware tuning routes
        .route("/hardware/tune", post(|| async {
            Json(serde_json::json!({
                "status": "success",
                "message": "Hardware tuning not implemented yet"
            }))
        }))
        .route("/hardware/config", get(|| async {
            Json(serde_json::json!({
                "status": "success",
                "config": {},
                "message": "Hardware config not implemented yet"
            }))
        }))

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

        // ZFS routes (now universal storage-agnostic)
        .route("/api/v1/zfs/pools", get(crate::handlers::zfs::list_universal_pools))
        .route("/api/v1/zfs/pools", post(create_zfs_pool))
        .route("/api/v1/zfs/pools/:pool_name", get(crate::handlers::zfs::get_universal_pool))
        .route("/api/v1/zfs/pools/:pool_name", delete(delete_zfs_pool))
        .route("/api/v1/zfs/datasets", get(get_zfs_dataset))
        .route("/api/v1/zfs/datasets", post(create_zfs_dataset))
        .route("/api/v1/zfs/datasets/:dataset_name", get(get_zfs_datasets))
        .route("/api/v1/zfs/datasets/:dataset_name", delete(delete_zfs_dataset))
        .route("/api/v1/zfs/snapshots", get(get_zfs_snapshots))
        .route("/api/v1/zfs/snapshots", post(create_zfs_snapshot))
        .route("/api/v1/zfs/snapshots/:snapshot_name", delete(delete_zfs_snapshot))
        .route("/api/v1/zfs/properties/:dataset_name", get(get_dataset_properties))
        .route("/api/v1/zfs/health", get(get_zfs_health))
        .route("/api/v1/zfs/pools/:pool_name", get(get_zfs_pool))
        .route("/api/v1/zfs/pools", get(get_zfs_pools))
        .route("/api/v1/zfs/optimization/analytics", get(get_zfs_optimization_analytics))

        // Universal Primal Integration routes (commented out until module is available)
        // .route("/api/v1/universal-primal/connect", post(universal_primal::connect_to_ecosystem))
        // .route("/api/v1/universal-primal/status", get(universal_primal::get_ecosystem_status))

        // BYOB routes (commented out until create_router is available)
        // .nest("/api/v1/byob", crate::byob::create_router())

        // Workspace management routes
        .route("/api/v1/workspaces", post(create_workspace))
        .route("/api/v1/workspaces", get(get_workspaces))
        .route("/api/v1/workspaces/:workspace_id", get(get_workspace))
        .route("/api/v1/workspaces/:workspace_id", patch(update_workspace_config))
        .route("/api/v1/workspaces/:workspace_id", delete(delete_workspace))

        // Team management routes
        .route("/api/v1/teams", post(create_team));

    // Add streaming routes conditionally
    #[cfg(feature = "streaming-rpc")]
    let router = router
        .route("/api/v1/communication/websocket", get(websocket_handler))
        .route("/api/v1/sse/events", get(sse_events))
        .route("/api/v1/sse/storage", get(sse_storage))
        .route("/api/v1/sse/health", get(sse_health));

    router
}

/// Create a router with initialized application state
pub async fn create_router_with_state() -> Router {
    let app_state = {
        #[cfg(feature = "streaming-rpc")]
        {
            AppState::new_with_streaming(
                WebSocketManager::new(),
                McpStreamingManager::new(),
                EventCoordinator::new(),
            )
            .with_zfs_manager()
            .await
        }
        #[cfg(not(feature = "streaming-rpc"))]
        {
            AppState::new().with_zfs_manager().await
        }
    };

    create_router_with_initialized_state(app_state)
}

fn create_router_with_initialized_state(app_state: AppState) -> Router {
    let router = Router::new()
        .route("/health", get(health_check))

        // Hardware tuning routes
        .route("/hardware/tune", post(|| async {
            Json(serde_json::json!({
                "status": "success",
                "message": "Hardware tuning not implemented yet"
            }))
        }))
        .route("/hardware/config", get(|| async {
            Json(serde_json::json!({
                "status": "success",
                "config": {},
                "message": "Hardware config not implemented yet"
            }))
        }))

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

        // ZFS routes (now universal storage-agnostic)
        .route("/api/v1/zfs/pools", get(crate::handlers::zfs::list_universal_pools))
        .route("/api/v1/zfs/pools", post(create_zfs_pool))
        .route("/api/v1/zfs/pools/:pool_name", get(crate::handlers::zfs::get_universal_pool))
        .route("/api/v1/zfs/pools/:pool_name", delete(delete_zfs_pool))
        .route("/api/v1/zfs/pools/:pool_name/scrub", post(scrub_zfs_pool))
        .route("/api/v1/zfs/datasets", get(get_zfs_datasets))
        .route("/api/v1/zfs/datasets", post(create_zfs_dataset))
        .route("/api/v1/zfs/datasets/:dataset_name", get(get_zfs_dataset))
        .route("/api/v1/zfs/datasets/:dataset_name", delete(delete_zfs_dataset))
        .route("/api/v1/zfs/datasets/:dataset_name/properties", get(get_dataset_properties))
        .route("/api/v1/zfs/datasets/:dataset_name/properties", put(set_dataset_properties))
        .route("/api/v1/zfs/datasets/:dataset_name/snapshots", get(list_dataset_snapshots))
        .route("/api/v1/zfs/datasets/:dataset_name/snapshots", post(create_dataset_snapshot))
        .route("/api/v1/zfs/datasets/:dataset_name/snapshots/:snapshot_name", delete(delete_dataset_snapshot))
        .route("/api/v1/zfs/snapshots", get(get_zfs_snapshots))
        .route("/api/v1/zfs/snapshots", post(create_zfs_snapshot))
        .route("/api/v1/zfs/snapshots/:snapshot_name", delete(delete_zfs_snapshot))
        .route("/api/v1/zfs/health", get(crate::handlers::zfs::get_universal_storage_health))
        .route("/api/v1/zfs/status", get(get_zfs_status))
        .route("/api/v1/zfs/optimization/analytics", get(get_zfs_optimization_analytics))
        .route("/api/v1/zfs/optimization/trigger", post(trigger_zfs_optimization))
        .route("/api/v1/zfs/ai/tier-prediction", post(predict_zfs_tier))

        // Universal Primal Integration routes (commented out until module is available)
        // .route("/api/v1/universal-primal/connect", post(universal_primal::connect_to_ecosystem))
        // .route("/api/v1/universal-primal/status", get(universal_primal::get_ecosystem_status))

        // BYOB routes (commented out until create_router is available)
        // .nest("/api/v1/byob", crate::byob::create_router())

        // Workspace management routes
        .route("/api/v1/workspaces", get(get_workspaces))
        .route("/api/v1/workspaces", post(create_workspace))
        .route("/api/v1/workspaces/:workspace_id", get(get_workspace))
        .route("/api/v1/workspaces/:workspace_id", patch(update_workspace_config))
        .route("/api/v1/workspaces/:workspace_id", delete(delete_workspace))

        // Team management routes
        .route("/api/v1/teams", post(create_team));

    // Add streaming routes conditionally
    #[cfg(feature = "streaming-rpc")]
    let router = router
        .route("/api/v1/communication/websocket", get(websocket_handler))
        .route("/api/v1/sse/events", get(sse_events))
        .route("/api/v1/sse/storage", get(sse_storage))
        .route("/api/v1/sse/health", get(sse_health));

    router.with_state(app_state)
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
    let websocket_stats = state.websocket_manager.get_stats().await;
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
            state.websocket_manager.get_stats().await.active_connections
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
            state.websocket_manager.get_stats().await.messages_sent
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
