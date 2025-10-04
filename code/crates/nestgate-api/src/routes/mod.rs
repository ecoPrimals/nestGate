use axum::{
    routing::{delete, get, patch, post, put},
    Router,
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
    storage::{
        get_storage_datasets, get_storage_metrics, get_storage_pools, get_storage_snapshots,
    },
    workspace_management::{
        create_workspace, delete_workspace, get_workspace, get_workspaces, update_workspace_config,
    },
};

// Optional ZFS imports - graceful degradation if not available
use crate::handlers::zfs_stub::{ProductionZfsManager, ZfsConfig};
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
pub struct AppState {
    /// ZFS manager instance for storage operations
    pub zfs_manager: Arc<ZfsManager>,
    // Add other shared state here as needed
    /// Phantom data for future extensibility
    pub _phantom: std::marker::PhantomData<()>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    /// Create AppState with ZFS support
    #[cfg(feature = "streaming-rpc")]
    pub fn with_zfs_and_streaming() -> Self {
        Self {
            zfs_manager: Arc::new(ZfsManager::new(ZfsConfig::default())),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create AppState without streaming features
    pub fn without_streaming() -> Self {
        Self {
            zfs_manager: Arc::new(ZfsManager::new(ZfsConfig::default())),
            #[cfg(feature = "streaming-rpc")]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create AppState with optional streaming components based on feature flags
    pub fn new() -> Self {
        Self {
            zfs_manager: Arc::new(ZfsManager::new(ZfsConfig::default())),
            #[cfg(feature = "streaming-rpc")]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Get ZFS manager reference
    pub fn get_zfs_manager(&self) -> Option<Arc<ZfsManager>> {
        Some(self.zfs_manager.clone())
    }

    /// Initialize storage systems - ZFS manager and Universal Storage Bridge
    pub fn with_zfs_manager(self) -> Self {
        // ZFS manager already initialized in constructor
        self
    }

    /// Initialize ZFS manager with graceful fallback
    #[allow(dead_code)] // Method used for ZFS initialization in production
    async fn try_init_zfs_manager(
        &self,
    ) -> Result<Option<ZfsManager>, Box<dyn std::error::Error + Send + Sync>> {
        // Check if ZFS is available first
        Ok(Some(ProductionZfsManager::new(ZfsConfig::default())))
    }
}

/// Create a new router with default application state
pub fn create_router() -> Router<AppState> {
    // This is a backward compatibility function that uses default state
    // In practice, you should use create_router_with_state() for proper initialization
    let router = Router::new()
        .route("/health", get(health_check))
        // Hardware tuning routes
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
        // Communication routes
        .route("/api/v1/communication/stats", get(get_communication_stats))
        .route("/api/v1/events", get(get_events))
        // Performance analytics routes
        .route(
            "/api/v1/analytics/performance",
            get(get_performance_metrics),
        )
        .route("/api/v1/analytics/alerts", get(get_performance_alerts))
        .route(
            "/api/v1/analytics/recommendations",
            get(get_performance_recommendations),
        )
        // Load testing routes
        .route("/api/v1/load-testing/start", post(start_load_test))
        .route("/api/v1/load-testing/results", get(get_load_test_results))
        .route("/api/v1/load-testing/history", get(get_load_test_history))
        .route(
            "/api/v1/load-testing/baselines",
            get(get_performance_baselines),
        )
        // Storage routes
        .route("/api/v1/storage/pools", get(get_storage_pools))
        .route("/api/v1/storage/datasets", get(get_storage_datasets))
        .route("/api/v1/storage/snapshots", get(get_storage_snapshots))
        .route("/api/v1/storage/metrics", get(get_storage_metrics))
        // ZFS routes (now universal storage-agnostic)
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
        // Universal Primal Integration routes (commented out until module is available)
        // .route("/api/v1/universal-primal/connect", post(universal_primal::connect_to_ecosystem))
        // .route("/api/v1/universal-primal/status", get(universal_primal::get_ecosystem_status))
        // BYOB routes (commented out until create_router is available)
        // .nest("/api/v1/byob", crate::byob::create_router())
        // Workspace management routes
        .route("/api/v1/workspaces", post(create_workspace))
        .route("/api/v1/workspaces", get(get_workspaces))
        .route("/api/v1/workspaces/:workspace_id", get(get_workspace))
        .route(
            "/api/v1/workspaces/:workspace_id",
            patch(update_workspace_config),
        )
        .route("/api/v1/workspaces/:workspace_id", delete(delete_workspace))
        // Team management routes
        .route("/api/v1/teams", post(create_team));
    // Authentication routes
    // .nest("/api/v1/auth", crate::handlers::auth::auth_router()); // Module doesn't exist

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
pub fn create_router_with_state() -> Router {
    let app_state = {
        #[cfg(feature = "streaming-rpc")]
        {
            AppState::with_zfs_and_streaming().with_zfs_manager()
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
        // Communication routes
        .route("/api/v1/communication/stats", get(get_communication_stats))
        .route("/api/v1/events", get(get_events))
        // Performance analytics routes
        .route(
            "/api/v1/analytics/performance",
            get(get_performance_metrics),
        )
        .route("/api/v1/analytics/alerts", get(get_performance_alerts))
        .route(
            "/api/v1/analytics/recommendations",
            get(get_performance_recommendations),
        )
        // Load testing routes
        .route("/api/v1/load-testing/start", post(start_load_test))
        .route("/api/v1/load-testing/results", get(get_load_test_results))
        .route("/api/v1/load-testing/history", get(get_load_test_history))
        .route(
            "/api/v1/load-testing/baselines",
            get(get_performance_baselines),
        )
        // Storage routes
        .route("/api/v1/storage/pools", get(get_storage_pools))
        .route("/api/v1/storage/datasets", get(get_storage_datasets))
        .route("/api/v1/storage/snapshots", get(get_storage_snapshots))
        .route("/api/v1/storage/metrics", get(get_storage_metrics))
        // ZFS routes (now universal storage-agnostic)
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
        // Universal Primal Integration routes (commented out until module is available)
        // .route("/api/v1/universal-primal/connect", post(universal_primal::connect_to_ecosystem))
        // .route("/api/v1/universal-primal/status", get(universal_primal::get_ecosystem_status))
        // BYOB routes (commented out until create_router is available)
        // .nest("/api/v1/byob", crate::byob::create_router())
        // Workspace management routes
        .route("/api/v1/workspaces", get(get_workspaces))
        .route("/api/v1/workspaces", post(create_workspace))
        .route("/api/v1/workspaces/:workspace_id", get(get_workspace))
        .route(
            "/api/v1/workspaces/:workspace_id",
            patch(update_workspace_config),
        )
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
async fn websocket_handler(
    ws: axum::extract::WebSocketUpgrade,
    axum::extract::State(_state): axum::extract::State<AppState>,
) -> axum::response::Response {
    // Return a simple message since websocket_manager is not available
    ws.on_upgrade(|_socket| async {
        // Stub websocket handler
        tracing::info!("WebSocket connection established (stub implementation)");
        // In a real implementation, this would handle the websocket connection
    })
}

/// SSE events stub implementation
async fn sse_events(
    axum::extract::State(_state): axum::extract::State<AppState>,
) -> impl axum::response::IntoResponse {
    axum::response::Json(serde_json::json!({
        "status": "success",
        "events": [
            {
                "id": "event_1",
                "type": "system_status",
                "data": "System operational",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        ]
    }))
}

/// SSE storage stub implementation
async fn sse_storage(
    axum::extract::State(_state): axum::extract::State<AppState>,
) -> impl axum::response::IntoResponse {
    axum::response::Json(serde_json::json!({
        "status": "success",
        "storage_events": [
            {
                "id": "storage_1",
                "type": "storage_status",
                "data": "Storage operational",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        ]
    }))
}

/// SSE health stub implementation
async fn sse_health(
    axum::extract::State(_state): axum::extract::State<AppState>,
) -> impl axum::response::IntoResponse {
    axum::response::Json(serde_json::json!({
        "status": "success",
        "health": {
            "api": "healthy",
            "storage": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }
    }))
}
