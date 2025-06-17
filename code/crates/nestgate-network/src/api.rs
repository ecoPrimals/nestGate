//! Simplified REST API for NestGate Network Layer

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

// Use nestgate_core for error handling
use nestgate_core::{NestGateError, Result};

/// Version constant for the network API
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Service status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

/// Service instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub status: ServiceStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Network API state
type NetworkApiState = Arc<RwLock<HashMap<String, ServiceInstance>>>;

/// Network API
#[derive(Clone)]
pub struct NetworkApi {
    state: NetworkApiState,
}

impl NetworkApi {
    /// Create a new network API
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create the router
    pub fn create_router(&self) -> Router {
        Router::new()
            .route("/api/health", get(health_check))
            .route("/api/version", get(get_version))
            .route("/api/services", get(list_services).post(register_service))
            .route("/api/services/:name", get(get_service).delete(unregister_service))
            .layer(CorsLayer::permissive())
            .with_state(self.state.clone())
    }
}

/// API Response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
}

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }
}

// API Handlers

async fn health_check() -> (StatusCode, Json<ApiResponse<String>>) {
    (StatusCode::OK, Json(ApiResponse::success("OK".to_string())))
}

async fn get_version() -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let version_info = serde_json::json!({
        "version": VERSION.to_string(),
        "build_date": "unknown",
        "git_commit": "unknown"
    });
    
    (StatusCode::OK, Json(ApiResponse::success(version_info)))
}

async fn list_services(
    State(state): State<NetworkApiState>,
) -> (StatusCode, Json<ApiResponse<Vec<ServiceInstance>>>) {
    let services = state.read().await;
    let service_list: Vec<ServiceInstance> = services.values().cloned().collect();
    
    (StatusCode::OK, Json(ApiResponse::success(service_list)))
}

#[derive(Deserialize)]
struct RegisterServiceRequest {
    name: String,
    host: String,
    port: u16,
}

async fn register_service(
    State(state): State<NetworkApiState>,
    Json(request): Json<RegisterServiceRequest>,
) -> (StatusCode, Json<ApiResponse<ServiceInstance>>) {
    let service_instance = ServiceInstance {
        id: uuid::Uuid::new_v4().to_string(),
        name: request.name.clone(),
        host: request.host,
        port: request.port,
        status: ServiceStatus::Running,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    let mut services = state.write().await;
    services.insert(request.name, service_instance.clone());
    
    (StatusCode::CREATED, Json(ApiResponse::success(service_instance)))
}

async fn get_service(
    Path(name): Path<String>,
    State(state): State<NetworkApiState>,
) -> (StatusCode, Json<ApiResponse<Option<ServiceInstance>>>) {
    let services = state.read().await;
    let service = services.get(&name).cloned();
    
    match service {
        Some(service) => (StatusCode::OK, Json(ApiResponse::success(Some(service)))),
        None => (StatusCode::NOT_FOUND, Json(ApiResponse::success(None))),
    }
}

async fn unregister_service(
    Path(name): Path<String>,
    State(state): State<NetworkApiState>,
) -> (StatusCode, Json<ApiResponse<bool>>) {
    let mut services = state.write().await;
    let removed = services.remove(&name).is_some();
    
    (StatusCode::OK, Json(ApiResponse::success(removed)))
} 