//
// Pure data layer REST API with real-time bidirectional RPC capabilities.
// Integrates tarpc (beardog), JSON RPC (songbird), and WebSocket streams.

pub mod handlers;
pub mod models;
pub mod rpc;
pub mod websocket; // ✅ WebSocket module implemented

use axum::{
    routing::{get, post},
    Router,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use uuid::Uuid;

// Re-export commonly used types
pub use handlers::*;
pub use models::*;
pub use rpc::{RpcError, RpcStreamEvent, UnifiedRpcManager, UnifiedRpcRequest, UnifiedRpcResponse};

// Re-export API router function (remove duplicate export)

use nestgate_core::error::Result;
use nestgate_core::universal_storage::{
    canonical_storage::FilesystemBackend, zfs_features::ModernZfsEngine, AutoConfigurator,
    StorageDetector,
};

/// API state with RPC capabilities
#[derive(Clone)]
pub struct ApiState {
    /// ZFS engines for different datasets
    pub zfs_engines: Arc<RwLock<HashMap<String, Arc<ModernZfsEngine<FilesystemBackend>>>>>,
    /// Storage detector for discovering available storage
    pub storage_detector: Arc<Mutex<StorageDetector>>,
    /// Auto-configurator for optimal storage setup
    pub auto_configurator: Arc<Mutex<Option<AutoConfigurator>>>,
    /// Unified RPC manager for beardog/songbird communication
    pub rpc_manager: Arc<Mutex<Option<UnifiedRpcManager>>>,
}

impl ApiState {
    /// Create new API state with RPC capabilities
    pub async fn new() -> Result<Self> {
        let storage_detector = StorageDetector::new();

        Ok(Self {
            zfs_engines: Arc::new(RwLock::new(HashMap::new())),
            storage_detector: Arc::new(Mutex::new(storage_detector)),
            auto_configurator: Arc::new(Mutex::new(None)),
            rpc_manager: Arc::new(Mutex::new(None)),
        })
    }

    /// Initialize RPC connections
    pub async fn init_rpc_connections(&self) -> Result<()> {
        let mut rpc_manager_opt = self.rpc_manager.lock().await;

        if rpc_manager_opt.is_none() {
            let mut rpc_manager = UnifiedRpcManager::new();

            // Initialize beardog connection if available
            if let Ok(beardog_addr) = std::env::var("NESTGATE_BEARDOG_ADDRESS") {
                if let Err(e) = rpc_manager.init_tarpc_service(&beardog_addr).await {
                    tracing::warn!("Failed to connect to beardog at {}: {}", beardog_addr, e);
                }
            } else {
                tracing::info!("NESTGATE_BEARDOG_ADDRESS not set, skipping beardog connection");
            }

            // Initialize songbird connection if available
            if let Ok(songbird_addr) = std::env::var("NESTGATE_SONGBIRD_ADDRESS") {
                if let Err(e) = rpc_manager.init_json_rpc_service(&songbird_addr).await {
                    tracing::warn!("Failed to connect to songbird at {}: {}", songbird_addr, e);
                }
            } else {
                tracing::info!("NESTGATE_SONGBIRD_ADDRESS not set, skipping songbird connection");
            }

            *rpc_manager_opt = Some(rpc_manager);
            tracing::info!("🔗 RPC connections initialized");
        }

        Ok(())
    }

    /// Get RPC manager
    pub async fn get_rpc_manager(&self) -> Option<Arc<Mutex<Option<UnifiedRpcManager>>>> {
        Some(Arc::clone(&self.rpc_manager))
    }
}

/// Unified API response for both REST and RPC
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DataResponse<T> {
    /// Response data
    pub data: T,
    /// Response timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ResponseMeta>,
}

impl<T> DataResponse<T> {
    /// Create new data response
    pub fn new(data: T) -> Self {
        Self {
            data,
            timestamp: chrono::Utc::now(),
            meta: None,
        }
    }

    /// Create paginated data response
    pub fn paginated(data: T, total: u64, page: u64, per_page: u64) -> Self {
        Self {
            data,
            timestamp: chrono::Utc::now(),
            meta: Some(ResponseMeta {
                total,
                page,
                per_page,
                has_more: (page * per_page) < total,
            }),
        }
    }
}

/// Response metadata for pagination
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResponseMeta {
    /// Total number of items
    pub total: u64,
    /// Current page number
    pub page: u64,
    /// Items per page
    pub per_page: u64,
    /// Whether there are more pages
    pub has_more: bool,
}

/// Unified error response for both REST and RPC
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DataError {
    /// Error message
    pub error: String,
    /// Error code for programmatic handling
    pub code: String,
    /// Error timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl DataError {
    /// Create new data error
    pub fn new(error: String, code: String) -> Self {
        Self {
            error,
            code,
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Query parameters for list operations
#[derive(Debug, serde::Deserialize)]
pub struct ListQuery {
    /// Page number (1-based)
    pub page: Option<u64>,
    /// Items per page
    pub per_page: Option<u64>,
    /// Sort field
    pub sort: Option<String>,
    /// Sort order (asc/desc)
    pub order: Option<String>,
    /// Filter string
    pub filter: Option<String>,
}

/// Create the main API router with RPC capabilities
pub fn create_api_router(state: ApiState) -> Router {
    // Health and system routes
    let health_routes = Router::new()
        .route("/health", get(health_check))
        .route("/version", get(version_info))
        .route("/system/status", get(system_status));

    // ZFS data routes
    let zfs_routes = Router::new()
        .route("/zfs/datasets", get(list_datasets).post(create_dataset))
        .route(
            "/zfs/datasets/:name",
            get(get_dataset).put(update_dataset).delete(delete_dataset),
        )
        .route(
            "/zfs/datasets/:name/properties",
            get(get_dataset_properties).put(set_dataset_properties),
        )
        .route("/zfs/datasets/:name/stats", get(get_dataset_stats))
        .route(
            "/zfs/datasets/:name/snapshots",
            get(list_snapshots).post(create_snapshot),
        )
        .route(
            "/zfs/datasets/:name/snapshots/:snap",
            get(get_snapshot).delete(delete_snapshot),
        )
        .route(
            "/zfs/datasets/:name/snapshots/:snap/clone",
            post(clone_snapshot),
        );

    // Storage management routes (unified with ZFS routes)
    let storage_routes = Router::new();

    // Monitoring routes
    let monitoring_routes = Router::new()
        .route("/monitoring/metrics", get(get_metrics))
        .route("/monitoring/metrics/history", get(get_metrics_history))
        .route("/monitoring/alerts", get(get_alerts));

    // WebSocket routes for real-time streams
    let websocket_routes = Router::new()
        .route("/ws/metrics", get(metrics_websocket))
        .route("/ws/logs", get(logs_websocket))
        .route("/ws/events", get(events_websocket));

    // RPC routes for beardog/songbird integration
    let rpc_routes = Router::new()
        .route("/rpc/call", post(handle_rpc_call))
        .route("/rpc/stream", post(handle_rpc_stream))
        .route("/rpc/health", get(handle_rpc_health));

    // Combine all routes
    Router::new()
        .merge(health_routes)
        .nest(
            "/api/v1",
            zfs_routes.merge(storage_routes).merge(monitoring_routes),
        )
        .merge(websocket_routes)
        .nest("/api/v1", rpc_routes)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Handle RPC call requests
async fn handle_rpc_call(
    axum::extract::State(state): axum::extract::State<ApiState>,
    axum::Json(request): axum::Json<UnifiedRpcRequest>,
) -> std::result::Result<axum::Json<DataResponse<UnifiedRpcResponse>>, axum::Json<DataError>> {
    let rpc_manager_opt = state.rpc_manager.lock().await;

    if let Some(rpc_manager) = rpc_manager_opt.as_ref() {
        let target = request.target.clone();
        match rpc_manager.call(&target, request).await {
            Ok(response) => Ok(axum::Json(DataResponse::new(response))),
            Err(e) => Err(axum::Json(DataError::new(
                format!("RPC call failed: {}", e),
                "RPC_CALL_FAILED".to_string(),
            ))),
        }
    } else {
        Err(axum::Json(DataError::new(
            "RPC manager not initialized".to_string(),
            "RPC_NOT_AVAILABLE".to_string(),
        )))
    }
}

/// Handle RPC stream requests
async fn handle_rpc_stream(
    axum::extract::State(state): axum::extract::State<ApiState>,
    axum::Json(request): axum::Json<UnifiedRpcRequest>,
) -> std::result::Result<axum::Json<DataResponse<serde_json::Value>>, axum::Json<DataError>> {
    let rpc_manager_opt = state.rpc_manager.lock().await;

    if let Some(rpc_manager) = rpc_manager_opt.as_ref() {
        match rpc_manager.start_bidirectional_stream(request).await {
            Ok((_tx, mut rx)) => {
                let stream_id = Uuid::new_v4();

                // Spawn task to handle stream events
                tokio::spawn(async move {
                    while let Some(event) = rx.recv().await {
                        tracing::debug!("RPC stream event: {:?}", event);
                        // In a real implementation, this would be sent via WebSocket
                    }
                });

                Ok(axum::Json(DataResponse::new(serde_json::json!({
                    "stream_id": stream_id,
                    "status": "started",
                    "message": "Bidirectional RPC stream initiated"
                }))))
            }
            Err(e) => Err(axum::Json(DataError::new(
                format!("Failed to start RPC stream: {}", e),
                "RPC_STREAM_FAILED".to_string(),
            ))),
        }
    } else {
        Err(axum::Json(DataError::new(
            "RPC manager not initialized".to_string(),
            "RPC_NOT_AVAILABLE".to_string(),
        )))
    }
}

/// Handle RPC health check
async fn handle_rpc_health(
    axum::extract::State(state): axum::extract::State<ApiState>,
) -> std::result::Result<axum::Json<DataResponse<serde_json::Value>>, axum::Json<DataError>> {
    let rpc_manager_opt = state.rpc_manager.lock().await;

    if let Some(rpc_manager) = rpc_manager_opt.as_ref() {
        let health_status = rpc_manager.get_health_status().await;
        Ok(axum::Json(DataResponse::new(serde_json::json!({
            "rpc_connections": health_status,
            "status": "available"
        }))))
    } else {
        Ok(axum::Json(DataResponse::new(serde_json::json!({
            "rpc_connections": {},
            "status": "not_initialized"
        }))))
    }
}
