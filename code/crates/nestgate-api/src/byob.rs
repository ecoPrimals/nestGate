//! # NestGate BYOB HTTP API
//!
//! REST API endpoints for BYOB storage operations.
//! Handles storage requests from Songbird coordination layer.

use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use uuid::Uuid;

use nestgate_zfs::{
    byob::{
        ByobStorageProvider, ByobStorageRequest, ByobStorageResponse, ServiceStorageRequirements,
        StorageUsage, TeamStorageQuotas,
    },
    ZfsManager,
};

/// HTTP API error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub timestamp: String,
}

impl ErrorResponse {
    pub fn new(error: &str, message: &str) -> Self {
        Self {
            error: error.to_string(),
            message: message.to_string(),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
}

/// Storage provision request
#[derive(Debug, Deserialize)]
pub struct ProvisionRequest {
    pub deployment_id: Uuid,
    pub team_id: String,
    pub deployment_name: String,
    pub storage_requirements: std::collections::HashMap<String, ServiceStorageRequirements>,
    pub team_quotas: TeamStorageQuotas,
}

/// Storage list query parameters
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub team_id: Option<String>,
    pub status: Option<String>,
    pub limit: Option<u32>,
}

/// Storage usage query parameters
#[derive(Debug, Deserialize)]
pub struct UsageQuery {
    pub include_details: Option<bool>,
}

/// Snapshot creation request
#[derive(Debug, Deserialize)]
pub struct CreateSnapshotRequest {
    pub snapshot_name: String,
    pub description: Option<String>,
}

/// Snapshot restore request
#[derive(Debug, Deserialize)]
pub struct RestoreSnapshotRequest {
    pub snapshot_name: String,
}

/// API state
pub struct ApiState {
    pub storage_provider: Arc<dyn ByobStorageProvider>,
}

/// Create the BYOB API router
pub fn create_byob_router(storage_provider: Arc<dyn ByobStorageProvider>) -> Router {
    let state = ApiState { storage_provider };

    Router::new()
        .route("/health", get(health))
        .route("/storage", post(provision_storage))
        .route("/storage", get(list_storage))
        .route("/storage/:deployment_id", get(get_storage_status))
        .route("/storage/:deployment_id", post(remove_storage))
        .route("/storage/:deployment_id/usage", get(get_storage_usage))
        .route("/storage/:deployment_id/snapshots", post(create_snapshot))
        .route("/storage/:deployment_id/snapshots/:snapshot_name", post(restore_snapshot))
        .with_state(Arc::new(state))
}

/// Health check endpoint
pub async fn health() -> impl IntoResponse {
    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Json(response)
}

/// Provision storage for a team deployment
pub async fn provision_storage(
    State(state): State<Arc<ApiState>>,
    Json(request): Json<ProvisionRequest>,
) -> impl IntoResponse {
    info!("Provisioning storage for deployment: {}", request.deployment_id);

    // Create storage request
    let team_id = request.team_id.clone(); // Clone before moving
    let storage_request = ByobStorageRequest {
        deployment_id: request.deployment_id,
        team_id: request.team_id,
        deployment_name: request.deployment_name,
        storage_requirements: request.storage_requirements,
        team_quotas: request.team_quotas,
        network_config: nestgate_zfs::byob::StorageNetworkConfig {
            network_name: "byob-network".to_string(),
            nfs_config: Some(nestgate_zfs::byob::NfsExportConfig {
                export_path: format!("/nestpool/teams/{}", team_id),
                allowed_hosts: vec!["*".to_string()],
                options: std::collections::HashMap::new(),
            }),
            smb_config: None,
        },
        created_at: Utc::now(),
    };

    match state.storage_provider.provision_storage(storage_request).await {
        Ok(response) => {
            info!("Storage provisioned successfully: {}", response.deployment_id);
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => {
            error!("Failed to provision storage: {}", e);
            ErrorResponse::new("STORAGE_PROVISION_FAILED", &e.to_string()).into_response()
        }
    }
}

/// List storage deployments
pub async fn list_storage(
    State(state): State<Arc<ApiState>>,
    Query(query): Query<ListQuery>,
) -> impl IntoResponse {
    if let Some(team_id) = query.team_id {
        match state.storage_provider.list_team_storage(&team_id).await {
            Ok(deployments) => {
                let limited_deployments = if let Some(limit) = query.limit {
                    deployments.into_iter().take(limit as usize).collect()
                } else {
                    deployments
                };
                Json(limited_deployments).into_response()
            }
            Err(e) => {
                error!("Failed to list team storage: {}", e);
                ErrorResponse::new("STORAGE_LIST_FAILED", &e.to_string()).into_response()
            }
        }
    } else {
        // If no team_id provided, we can't list all storage (for security)
        ErrorResponse::new("MISSING_TEAM_ID", "team_id parameter is required").into_response()
    }
}

/// Get storage status for a deployment
pub async fn get_storage_status(
    State(state): State<Arc<ApiState>>,
    Path(deployment_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.storage_provider.get_storage_status(deployment_id).await {
        Ok(response) => Json(response).into_response(),
        Err(e) => {
            error!("Failed to get storage status: {}", e);
            ErrorResponse::new("STORAGE_STATUS_FAILED", &e.to_string()).into_response()
        }
    }
}

/// Remove storage for a deployment
pub async fn remove_storage(
    State(state): State<Arc<ApiState>>,
    Path(deployment_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.storage_provider.remove_storage(deployment_id).await {
        Ok(()) => {
            info!("Storage removed successfully: {}", deployment_id);
            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => {
            error!("Failed to remove storage: {}", e);
            ErrorResponse::new("STORAGE_REMOVE_FAILED", &e.to_string()).into_response()
        }
    }
}

/// Get storage usage for a deployment
pub async fn get_storage_usage(
    State(state): State<Arc<ApiState>>,
    Path(deployment_id): Path<Uuid>,
    Query(query): Query<UsageQuery>,
) -> impl IntoResponse {
    match state.storage_provider.get_storage_usage(deployment_id).await {
        Ok(usage) => Json(usage).into_response(),
        Err(e) => {
            error!("Failed to get storage usage: {}", e);
            ErrorResponse::new("STORAGE_USAGE_FAILED", &e.to_string()).into_response()
        }
    }
}

/// Create a snapshot for a deployment
pub async fn create_snapshot(
    State(state): State<Arc<ApiState>>,
    Path(deployment_id): Path<Uuid>,
    Json(request): Json<CreateSnapshotRequest>,
) -> impl IntoResponse {
    match state.storage_provider.create_snapshot(deployment_id, request.snapshot_name).await {
        Ok(snapshot_id) => {
            let response = serde_json::json!({
                "snapshot_id": snapshot_id,
                "deployment_id": deployment_id,
                "created_at": Utc::now().to_rfc3339()
            });
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => {
            error!("Failed to create snapshot: {}", e);
            ErrorResponse::new("SNAPSHOT_CREATE_FAILED", &e.to_string()).into_response()
        }
    }
}

/// Restore from a snapshot
pub async fn restore_snapshot(
    State(state): State<Arc<ApiState>>,
    Path((deployment_id, snapshot_name)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    match state.storage_provider.restore_snapshot(deployment_id, snapshot_name).await {
        Ok(()) => {
            info!("Snapshot restored successfully: {}", deployment_id);
            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => {
            error!("Failed to restore snapshot: {}", e);
            ErrorResponse::new("SNAPSHOT_RESTORE_FAILED", &e.to_string()).into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use nestgate_zfs::byob::create_zfs_storage_provider;
    use nestgate_zfs::config::ZfsConfig;
    use nestgate_zfs::ZfsManager;
    use std::sync::Arc;

    // Mock storage provider for testing
    struct MockStorageProvider;

    #[async_trait::async_trait]
    impl ByobStorageProvider for MockStorageProvider {
        async fn provision_storage(
            &self,
            _request: ByobStorageRequest,
        ) -> nestgate_core::Result<ByobStorageResponse> {
            // Mock implementation
            Ok(ByobStorageResponse {
                deployment_id: Uuid::new_v4(),
                status: nestgate_zfs::byob::StorageStatus::Ready,
                datasets: std::collections::HashMap::new(),
                mounts: std::collections::HashMap::new(),
                usage: StorageUsage {
                    total_allocated: 0,
                    total_used: 0,
                    usage_per_tier: std::collections::HashMap::new(),
                    usage_per_dataset: std::collections::HashMap::new(),
                },
                endpoints: std::collections::HashMap::new(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn get_storage_status(
            &self,
            _deployment_id: Uuid,
        ) -> nestgate_core::Result<ByobStorageResponse> {
            // Mock implementation
            Ok(ByobStorageResponse {
                deployment_id: Uuid::new_v4(),
                status: nestgate_zfs::byob::StorageStatus::Ready,
                datasets: std::collections::HashMap::new(),
                mounts: std::collections::HashMap::new(),
                usage: StorageUsage {
                    total_allocated: 0,
                    total_used: 0,
                    usage_per_tier: std::collections::HashMap::new(),
                    usage_per_dataset: std::collections::HashMap::new(),
                },
                endpoints: std::collections::HashMap::new(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn remove_storage(&self, _deployment_id: Uuid) -> nestgate_core::Result<()> {
            Ok(())
        }

        async fn list_team_storage(&self, _team_id: &str) -> nestgate_core::Result<Vec<ByobStorageResponse>> {
            Ok(vec![])
        }

        async fn get_storage_usage(&self, _deployment_id: Uuid) -> nestgate_core::Result<StorageUsage> {
            Ok(StorageUsage {
                total_allocated: 0,
                total_used: 0,
                usage_per_tier: std::collections::HashMap::new(),
                usage_per_dataset: std::collections::HashMap::new(),
            })
        }

        async fn create_snapshot(
            &self,
            _deployment_id: Uuid,
            _snapshot_name: String,
        ) -> nestgate_core::Result<String> {
            Ok("mock-snapshot-id".to_string())
        }

        async fn restore_snapshot(
            &self,
            _deployment_id: Uuid,
            _snapshot_name: String,
        ) -> nestgate_core::Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let storage_provider = Arc::new(MockStorageProvider) as Arc<dyn ByobStorageProvider>;
        let app = create_byob_router(storage_provider);
        let server = TestServer::new(app).unwrap();

        let response = server.get("/health").await;
        assert_eq!(response.status_code(), StatusCode::OK);

        let health: HealthResponse = response.json();
        assert_eq!(health.status, "healthy");
    }

    #[tokio::test]
    async fn test_provision_storage() {
        let storage_provider = Arc::new(MockStorageProvider) as Arc<dyn ByobStorageProvider>;
        let app = create_byob_router(storage_provider);
        let server = TestServer::new(app).unwrap();

        let request = ProvisionRequest {
            deployment_id: Uuid::new_v4(),
            team_id: "test-team".to_string(),
            deployment_name: "test-deployment".to_string(),
            storage_requirements: std::collections::HashMap::new(),
            team_quotas: TeamStorageQuotas {
                max_total_storage: 1000000000,
                max_per_tier: std::collections::HashMap::new(),
                max_datasets: 10,
                max_snapshots: 100,
                max_backup_retention_days: 30,
            },
        };

        let response = server.post("/storage").json(&request).await;
        assert_eq!(response.status_code(), StatusCode::CREATED);
    }
} 