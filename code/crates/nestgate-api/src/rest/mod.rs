//! **REST API MODULE**
//!
//! This module contains the REST API implementation for NestGate, including
//! handlers, models, and WebSocket support for real-time communication.

pub mod handlers;
pub mod models;
pub mod rpc;
/// **WEBSOCKET MODULE**
///
/// WebSocket support for real-time bidirectional communication with clients.
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

// Import handler functions
use crate::rest::handlers::{
    clone_snapshot, create_dataset, create_snapshot, delete_dataset, delete_snapshot,
    events_websocket, get_alerts, get_dataset, get_dataset_properties, get_dataset_stats,
    get_metrics, get_metrics_history, get_snapshot, health_check, list_datasets, list_snapshots,
    logs_websocket, metrics_websocket, set_dataset_properties, system_status, update_dataset,
    version_info,
};

// Re-export commonly used types (removed glob exports to avoid ambiguity)
// pub use handlers::*;  // Commented out to avoid ambiguous re-exports
// pub use models::*;    // Commented out to avoid ambiguous re-exports
pub use rpc::{RpcError, RpcStreamEvent, UnifiedRpcManager, UnifiedRpcRequest, UnifiedRpcResponse};

// Re-export API router function (remove duplicate export)

use nestgate_core::error::Result;
use nestgate_core::universal_storage::{AutoConfigurator, StorageDetector};

/// API state with RPC capabilities
#[derive(Clone)]
/// Apistate
pub struct ApiState {
    /// ZFS engines for different datasets (placeholder)
    pub zfs_engines: Arc<RwLock<HashMap<String, String>>>,
    /// Storage detector for discovering available storage
    pub storage_detector: Arc<Mutex<StorageDetector>>,
    /// Auto-configurator for optimal storage setup
    pub auto_configurator: Arc<Mutex<Option<AutoConfigurator>>>,
    /// Unified RPC manager for security/orchestration communication
    pub rpc_manager: Arc<Mutex<Option<UnifiedRpcManager>>>,
}
impl ApiState {
    /// Create new API state with RPC capabilities
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn new() -> Result<Self> {
        let storage_detector = StorageDetector::new();

        Ok(Self {
            zfs_engines: Arc::new(RwLock::new(HashMap::new())),
            storage_detector: Arc::new(Mutex::new(storage_detector)),
            auto_configurator: Arc::new(Mutex::new(None)),
            rpc_manager: Arc::new(Mutex::new(None)),
        })
    }

    /// Initialize RPC connections
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn init_rpc_connections(&self) -> Result<()> {
        let mut rpc_manager_opt = self.rpc_manager.lock().await;

        if rpc_manager_opt.is_none() {
            let rpc_manager = UnifiedRpcManager::new();

            // ✅ UNIVERSAL ADAPTER PATTERN - Pure capability-based discovery
            // Zero primal knowledge - infant-like discovery of capabilities

            // Initialize security capability discovery
            if let Ok(security_endpoint) = std::env::var("SECURITY_DISCOVERY_ENDPOINT") {
                if let Err(e) = rpc_manager.init_security_capability(&security_endpoint) {
                    tracing::warn!(
                        "Failed to connect to security capability at {}: {}",
                        security_endpoint,
                        e
                    );
                }
            } else {
                tracing::info!("Security capability discovery through universal adapter");
            }

            // ✅ UNIVERSAL ADAPTER INTEGRATION - Modern capability-based RPC
            if std::env::var("UNIVERSAL_ADAPTER_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true)
            {
                tracing::info!(
                    "🔄 Universal adapter enabled - RPC routing will use capability discovery"
                );
                // Future: Initialize universal RPC router here
                // let universal_router = UniversalRpcRouter::new(universal_adapter).await?;
                // rpc_manager.set_universal_router(universal_router);
            }

            *rpc_manager_opt = Some(rpc_manager);
            tracing::info!("🔗 RPC connections initialized");
        }

        Ok(())
    }

    /// Get RPC manager
    #[must_use]
    pub fn get_rpc_manager(&self) -> Option<Arc<Mutex<Option<UnifiedRpcManager>>>> {
        Some(Arc::clone(&self.rpc_manager))
    }
}

/// Unified API response for both REST and RPC
#[derive(Debug, serde::Serialize, serde::Deserialize)]
/// Response data for Data operation
pub struct DataResponse<T> {
    /// Response data
    pub data: T,
    /// Response timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Optional _metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Meta
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

/// Response _metadata for pagination
#[derive(Debug, serde::Serialize, serde::Deserialize)]
/// Responsemeta
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
/// Error type for Data operations
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
    #[must_use]
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
/// Listquery
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

    // RPC routes for security/orchestration integration
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
            Err(_e) => Err(axum::Json(DataError::new(
                "RPC call failed: self.base_url".to_string(),
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
        match rpc_manager.start_bidirectional_stream(request) {
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
            Err(_e) => Err(axum::Json(DataError::new(
                "Failed to start RPC stream: self.base_url".to_string(),
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
        let health_status = rpc_manager.get_health_status();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_response_new() {
        let resp = DataResponse::new(42);
        assert_eq!(resp.data, 42);
        assert!(resp.meta.is_none());
    }

    #[test]
    fn test_data_response_paginated() {
        let resp = DataResponse::<Vec<i32>>::paginated(vec![1, 2, 3], 100, 1, 10);
        assert_eq!(resp.data.len(), 3);
        assert!(resp.meta.is_some());
        let meta = resp.meta.unwrap();
        assert_eq!(meta.total, 100);
        assert_eq!(meta.page, 1);
        assert_eq!(meta.per_page, 10);
        assert!(meta.has_more);
    }

    #[test]
    fn test_data_error_new() {
        let err = DataError::new("test error".to_string(), "ERR_CODE".to_string());
        assert_eq!(err.error, "test error");
        assert_eq!(err.code, "ERR_CODE");
    }

    #[test]
    fn test_response_meta_fields() {
        let meta = ResponseMeta {
            total: 50,
            page: 2,
            per_page: 10,
            has_more: true,
        };
        assert_eq!(meta.total, 50);
        assert_eq!(meta.has_more, true);
    }

    #[test]
    fn test_list_query_deserialization() {
        let json = r#"{"page": 1, "per_page": 20, "sort": "name", "order": "asc"}"#;
        let query: ListQuery = serde_json::from_str(json).unwrap();
        assert_eq!(query.page, Some(1));
        assert_eq!(query.per_page, Some(20));
        assert_eq!(query.sort.as_deref(), Some("name"));
        assert_eq!(query.order.as_deref(), Some("asc"));
    }

    #[tokio::test]
    async fn test_api_state_new() {
        let state = ApiState::new();
        assert!(state.is_ok());
        let state = state.unwrap();
        assert!(state.get_rpc_manager().is_some());
    }
}
