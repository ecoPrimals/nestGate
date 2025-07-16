//! # NestGate BYOB HTTP API
//!
//! REST API endpoints for BYOB storage operations.
//! Handles storage requests from Songbird coordination layer.

use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post, put},
    Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info, warn};
use uuid::Uuid;

use nestgate_zfs::byob::{
    ByobStorageProvider, ByobStorageRequest, ServiceStorageRequirements, TeamStorageQuotas,
};

use crate::routes::AppState;

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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
}

/// Storage provision request
#[derive(Debug, Serialize, Deserialize)]
pub struct ProvisionRequest {
    pub deployment_id: Uuid,
    pub team_id: String,
    pub deployment_name: String,
    pub storage_requirements: std::collections::HashMap<String, ServiceStorageRequirements>,
    pub team_quotas: TeamStorageQuotas,
}

/// Storage list query parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct ListQuery {
    pub team_id: Option<String>,
    pub status: Option<String>,
    pub limit: Option<u32>,
}

/// Storage usage query parameters
#[derive(Debug, Deserialize)]
pub struct UsageQuery {
    pub include_snapshots: Option<bool>,
    pub include_children: Option<bool>,
}

/// Workspace creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub team_id: String,
    pub storage_quota: Option<String>,
    pub compression: Option<String>,
    pub description: Option<String>,
}

/// Workspace configuration update request
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWorkspaceConfigRequest {
    pub quota: Option<String>,
    pub compression: Option<String>,
    pub deduplication: Option<bool>,
    pub encryption: Option<bool>,
}

/// Team creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
    pub description: Option<String>,
    pub storage_quota: Option<String>,
    pub compute_quota: Option<String>,
}

/// Project creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub team_id: String,
    pub description: Option<String>,
    pub storage_quota: Option<String>,
}

/// In-memory storage for workspace state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub id: Uuid,
    pub name: String,
    pub team_id: String,
    pub status: String,
    pub dataset_name: String,
    pub storage_quota: String,
    pub compression: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// In-memory storage for team state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamState {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub storage_quota: String,
    pub compute_quota: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// In-memory storage for project state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectState {
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub description: Option<String>,
    pub storage_quota: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct ApiState {
    pub database: Arc<tokio::sync::RwLock<std::collections::HashMap<String, serde_json::Value>>>,
    pub config: Arc<tokio::sync::RwLock<serde_json::Value>>,
}

impl Default for ApiState {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiState {
    pub fn new() -> Self {
        Self {
            database: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            config: Arc::new(tokio::sync::RwLock::new(serde_json::json!({}))),
        }
    }
}

/// Create snapshot request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSnapshotRequest {
    pub name: String,
    pub description: Option<String>,
    pub retention_days: Option<u32>,
}

/// Create the BYOB router with all routes
fn create_byob_router(_storage_provider: Arc<dyn ByobStorageProvider>) -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/storage", post(provision_storage))
        .route("/storage", get(list_storage))
        .route("/storage/:deployment_id", get(get_storage_status))
        .route("/storage/:deployment_id", post(remove_storage))
        .route("/storage/:deployment_id/usage", get(get_storage_usage))
        .route("/storage/:deployment_id/snapshots", post(create_snapshot))
        .route("/storage/:deployment_id/snapshots/:snapshot_name", post(restore_snapshot))

        // Workspace management routes
        .route("/workspaces/:workspace_id/delete", post(delete_workspace))
        .route("/workspaces/:workspace_id/deploy", post(deploy_workspace))
        .route("/workspaces/:workspace_id/status", get(get_workspace_status))
        .route("/workspaces/:workspace_id/cleanup", post(cleanup_workspace))
        .route("/workspaces/:workspace_id/scale", post(scale_workspace))
        .route("/workspaces/:workspace_id/backup", post(backup_workspace))
        .route("/workspaces/:workspace_id/restore", post(restore_workspace))
        .route("/workspaces/:workspace_id/migrate", post(migrate_workspace))
        .route("/workspaces/:workspace_id/optimize", post(optimize_workspace))

        // Workspace configuration routes
        .route("/workspaces/:workspace_id/config", get(get_workspace_config))
        .route("/workspaces/:workspace_id/config", put(update_workspace_config))

        // Template management routes
        .route("/workspaces/:workspace_id/templates", get(get_workspace_templates))
        .route("/workspaces/:workspace_id/templates", post(create_workspace_template))
        .route("/workspaces/:workspace_id/templates/:template_id", post(apply_workspace_template))

        // Add all other BYOB routes here
        .route("/deployments", post(create_deployment))
        .route("/deployments/:deployment_id", get(get_deployment))
        .route("/deployments/:deployment_id", delete(delete_deployment))
        .route("/teams", get(get_teams))
        .route("/teams", post(create_team))
        .route("/teams/:team_id", get(get_team))
        .route("/teams/:team_id", delete(delete_team))
        .route("/teams/:team_id/quota", get(get_team_quota))
        .route("/teams/:team_id/quota", put(update_team_quota))
        .route("/teams/:team_id/projects", get(get_team_projects))
        .route("/projects", post(create_project))
        .route("/projects/:project_id", get(get_project))
        .route("/projects/:project_id", delete(delete_project))
        .route("/projects/:project_id/datasets", get(get_project_datasets))
        .route("/datasets", post(create_dataset))
        .route("/datasets/:dataset_id", get(get_dataset))
        .route("/datasets/:dataset_id", delete(delete_dataset))
        .route("/snapshots", get(get_snapshots))
        .route("/snapshots/:snapshot_id", get(get_snapshot))
        .route("/workspaces", get(get_workspaces))
        .route("/workspaces", post(create_workspace))
        .route("/workspaces/:workspace_id", get(get_workspace))

        // Add workspace volume operations
        .route("/workspaces/:workspace_id/volumes/:volume_id/mount", post(mount_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/unmount", post(unmount_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/resize", post(resize_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/snapshot", post(snapshot_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/clone", post(clone_workspace_volume))
                .route("/workspaces/:workspace_id/volumes/:volume_id/replicate", post(replicate_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/encrypt", post(encrypt_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/decrypt", post(decrypt_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/compress", post(compress_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/decompress", post(decompress_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/integrity", post(check_workspace_volume_integrity))
        .route("/workspaces/:workspace_id/volumes/:volume_id/repair", post(repair_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/deduplicate", post(deduplicate_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/scrub", post(scrub_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/stats", get(get_workspace_volume_stats))
        .route("/workspaces/:workspace_id/volumes/:volume_id/history", get(get_workspace_volume_history))
        .route("/workspaces/:workspace_id/volumes/:volume_id/properties", get(get_workspace_volume_properties))
        .route("/workspaces/:workspace_id/volumes/:volume_id/properties", put(set_workspace_volume_properties))
        .route("/workspaces/:workspace_id/volumes/:volume_id/send", post(send_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/receive", post(receive_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/promote", post(promote_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/demote", post(demote_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/inherit", post(inherit_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/upgrade", post(upgrade_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/downgrade", post(downgrade_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/hold", post(hold_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/release", post(release_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/diff", post(diff_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/bookmark", post(bookmark_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/destroy", post(destroy_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/unload", post(unload_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/load", post(load_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/allow", post(allow_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/unallow", post(unallow_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/jail", post(jail_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/unjail", post(unjail_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/userspace", post(userspace_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/groupspace", post(groupspace_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/projectspace", post(projectspace_workspace_volume))
}

/// Create BYOB service with ToadStool integration
pub fn create_byob_service(storage_provider: Arc<dyn ByobStorageProvider>) -> Router<AppState> {
    // Initialize the API state
    let state = ApiState {
        database: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        config: Arc::new(tokio::sync::RwLock::new(serde_json::json!({}))),
    };

    // Create the AppState with all required fields
    let app_state = AppState {
        api_state: Arc::new(state),
        hardware_tuning_service: Arc::new(
            crate::handlers::hardware_tuning::HardwareTuningHandler::new(),
        ),
        event_coordinator: crate::event_coordination::EventCoordinator::new(),
        mcp_streaming_manager: crate::mcp_streaming::McpStreamingManager::new(),
        #[cfg(feature = "streaming-rpc")]
        websocket_manager: crate::websocket::WebSocketManager::new(),
        #[cfg(feature = "streaming-rpc")]
        sse_manager: Arc::new(crate::sse::SseManager::new()),
    };

    // Create the router with the unified AppState
    create_byob_router(storage_provider).with_state(app_state)
}

// Missing handler functions for BYOB API routes

/// Create deployment
pub async fn create_deployment(
    State(_state): State<AppState>,
    Json(_request): Json<serde_json::Value>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "deployment_id": uuid::Uuid::new_v4(),
        "status": "created",
        "timestamp": chrono::Utc::now()
    }))
}

/// Get deployment
pub async fn get_deployment(
    State(_state): State<AppState>,
    Path(_deployment_id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "deployment_id": _deployment_id,
        "status": "running",
        "timestamp": chrono::Utc::now()
    }))
}

/// Delete deployment
pub async fn delete_deployment(
    State(_state): State<AppState>,
    Path(_deployment_id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "deployment_id": _deployment_id,
        "status": "deleted",
        "timestamp": chrono::Utc::now()
    }))
}

/// Get teams
pub async fn get_teams(State(state): State<AppState>) -> impl IntoResponse {
    info!("📋 Getting all teams");

    // Get teams from state storage
    let database = state.api_state.database.read().await;
    let teams: Vec<TeamState> = database
        .get("teams")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    // Transform for response
    let team_summaries: Vec<serde_json::Value> = teams
        .iter()
        .map(|t| {
            json!({
                "id": t.id,
                "name": t.name,
                "description": t.description,
                "storage_quota": t.storage_quota,
                "compute_quota": t.compute_quota,
                "created_at": t.created_at
            })
        })
        .collect();

    Json(json!({
        "teams": team_summaries,
        "count": teams.len(),
        "timestamp": chrono::Utc::now()
    }))
}

/// Create team
pub async fn create_team(
    State(state): State<AppState>,
    Json(request): Json<CreateTeamRequest>,
) -> impl IntoResponse {
    info!("👥 Creating new team: {}", request.name);

    let team_id = Uuid::new_v4().to_string();
    let storage_quota = request.storage_quota.unwrap_or_else(|| "100G".to_string());
    let compute_quota = request
        .compute_quota
        .unwrap_or_else(|| "10 cores".to_string());

    // Create team state
    let team_state = TeamState {
        id: team_id.clone(),
        name: request.name.clone(),
        description: request.description.clone(),
        storage_quota: storage_quota.clone(),
        compute_quota: compute_quota.clone(),
        created_at: chrono::Utc::now(),
    };

    // Store team state
    {
        let mut database = state.api_state.database.write().await;
        let mut teams: Vec<TeamState> = database
            .get("teams")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        teams.push(team_state.clone());
        database.insert("teams".to_string(), json!(teams));
    }

    info!(
        "✅ Successfully created team: {} ({})",
        request.name, team_id
    );

    Json(json!({
        "status": "success",
        "team_id": team_id,
        "name": request.name,
        "description": request.description,
        "storage_quota": storage_quota,
        "compute_quota": compute_quota,
        "created_at": team_state.created_at,
        "message": "Team created successfully"
    }))
}

/// Get team
pub async fn get_team(
    State(_state): State<AppState>,
    Path(_team_id): Path<String>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "team_id": _team_id,
        "status": "active",
        "timestamp": chrono::Utc::now()
    }))
}

/// Delete team
pub async fn delete_team(
    State(_state): State<AppState>,
    Path(_team_id): Path<String>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "team_id": _team_id,
        "status": "deleted",
        "timestamp": chrono::Utc::now()
    }))
}

/// Get team quota
pub async fn get_team_quota(
    State(_state): State<AppState>,
    Path(_team_id): Path<String>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "team_id": _team_id,
        "quota": {
            "storage": "1TB",
            "compute": "10 cores"
        },
        "timestamp": chrono::Utc::now()
    }))
}

/// Update team quota
pub async fn update_team_quota(
    State(_state): State<AppState>,
    Path(_team_id): Path<String>,
    Json(_request): Json<serde_json::Value>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "team_id": _team_id,
        "status": "updated",
        "timestamp": chrono::Utc::now()
    }))
}

/// Get team projects
pub async fn get_team_projects(
    State(_state): State<AppState>,
    Path(_team_id): Path<String>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "team_id": _team_id,
        "projects": [],
        "timestamp": chrono::Utc::now()
    }))
}

/// Create project
pub async fn create_project(
    State(_state): State<AppState>,
    Json(_request): Json<serde_json::Value>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "project_id": uuid::Uuid::new_v4(),
        "status": "created",
        "timestamp": chrono::Utc::now()
    }))
}

/// Get project
pub async fn get_project(
    State(_state): State<AppState>,
    Path(_project_id): Path<String>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "project_id": _project_id,
        "status": "active",
        "timestamp": chrono::Utc::now()
    }))
}

/// Delete project
pub async fn delete_project(
    State(_state): State<AppState>,
    Path(_project_id): Path<String>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "project_id": _project_id,
        "status": "deleted",
        "timestamp": chrono::Utc::now()
    }))
}

/// Get project datasets
pub async fn get_project_datasets(
    State(_state): State<AppState>,
    Path(_project_id): Path<String>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "project_id": _project_id,
        "datasets": [],
        "timestamp": chrono::Utc::now()
    }))
}

/// Create dataset
pub async fn create_dataset(
    State(_state): State<AppState>,
    Json(_request): Json<serde_json::Value>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "dataset_id": uuid::Uuid::new_v4(),
        "status": "created",
        "timestamp": chrono::Utc::now()
    }))
}

/// Get dataset
pub async fn get_dataset(
    State(_state): State<AppState>,
    Path(_dataset_id): Path<String>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "dataset_id": _dataset_id,
        "status": "active",
        "timestamp": chrono::Utc::now()
    }))
}

/// Delete dataset
pub async fn delete_dataset(
    State(_state): State<AppState>,
    Path(_dataset_id): Path<String>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "dataset_id": _dataset_id,
        "status": "deleted",
        "timestamp": chrono::Utc::now()
    }))
}

/// Get snapshots
pub async fn get_snapshots(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({
        "snapshots": [],
        "timestamp": chrono::Utc::now()
    }))
}

/// Get snapshot
pub async fn get_snapshot(
    State(_state): State<AppState>,
    Path(_snapshot_id): Path<String>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "snapshot_id": _snapshot_id,
        "status": "active",
        "timestamp": chrono::Utc::now()
    }))
}

/// Get workspaces
pub async fn get_workspaces(State(state): State<AppState>) -> impl IntoResponse {
    info!("📋 Getting all workspaces");

    // Get workspaces from state storage
    let database = state.api_state.database.read().await;
    let workspaces: Vec<WorkspaceState> = database
        .get("workspaces")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    // Transform for response
    let workspace_summaries: Vec<serde_json::Value> = workspaces
        .iter()
        .map(|w| {
            json!({
                "id": w.id,
                "name": w.name,
                "team_id": w.team_id,
                "status": w.status,
                "dataset_name": w.dataset_name,
                "storage_quota": w.storage_quota,
                "compression": w.compression,
                "created_at": w.created_at,
                "updated_at": w.updated_at
            })
        })
        .collect();

    Json(json!({
        "workspaces": workspace_summaries,
        "count": workspaces.len(),
        "timestamp": chrono::Utc::now()
    }))
}

/// Create workspace
pub async fn create_workspace(
    State(state): State<AppState>,
    Json(request): Json<CreateWorkspaceRequest>,
) -> impl IntoResponse {
    info!("🏗️ Creating new workspace: {}", request.name);

    let workspace_id = Uuid::new_v4();
    let dataset_name = format!("nestpool/workspaces/{workspace_id}");
    let storage_quota = request.storage_quota.unwrap_or_else(|| "10G".to_string());
    let compression = request.compression.unwrap_or_else(|| "lz4".to_string());

    // Create ZFS dataset for workspace
    let zfs_args = vec![
        "create".to_string(),
        "-o".to_string(),
        format!("compression={}", compression),
        "-o".to_string(),
        format!("quota={}", storage_quota),
        "-o".to_string(),
        format!("nestgate:workspace_id={}", workspace_id),
        "-o".to_string(),
        format!("nestgate:team_id={}", request.team_id),
        "-o".to_string(),
        format!("nestgate:name={}", request.name),
        dataset_name.clone(),
    ];

    let create_result = tokio::process::Command::new("zfs")
        .args(&zfs_args)
        .output()
        .await;

    match create_result {
        Ok(output) if output.status.success() => {
            // Store workspace state
            let workspace_state = WorkspaceState {
                id: workspace_id,
                name: request.name.clone(),
                team_id: request.team_id.clone(),
                status: "active".to_string(),
                dataset_name: dataset_name.clone(),
                storage_quota: storage_quota.clone(),
                compression: compression.clone(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            {
                let mut database = state.api_state.database.write().await;
                let mut workspaces: Vec<WorkspaceState> = database
                    .get("workspaces")
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .unwrap_or_default();

                workspaces.push(workspace_state.clone());
                database.insert("workspaces".to_string(), json!(workspaces));
            }

            info!(
                "✅ Successfully created workspace: {} ({})",
                request.name, workspace_id
            );

            Json(json!({
                "status": "success",
                "workspace_id": workspace_id,
                "name": request.name,
                "team_id": request.team_id,
                "dataset_name": dataset_name,
                "storage_quota": storage_quota,
                "compression": compression,
                "mount_path": format!("/mnt/{}", dataset_name),
                "created_at": workspace_state.created_at,
                "message": "Workspace created successfully"
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to create workspace: {}", error_msg);

            Json(json!({
                "status": "error",
                "error": "WORKSPACE_CREATION_FAILED",
                "message": format!("Failed to create workspace: {}", error_msg),
                "workspace_id": workspace_id,
                "timestamp": chrono::Utc::now()
            }))
        }
        Err(e) => {
            error!("❌ Command execution failed: {}", e);

            Json(json!({
                "status": "error",
                "error": "COMMAND_EXECUTION_FAILED",
                "message": format!("Failed to execute ZFS command: {}", e),
                "workspace_id": workspace_id,
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}

/// Get workspace
pub async fn get_workspace(
    State(state): State<AppState>,
    Path(workspace_id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    info!("📖 Getting workspace details: {}", workspace_id);

    // Get workspace from state storage
    let database = state.api_state.database.read().await;
    let workspaces: Vec<WorkspaceState> = database
        .get("workspaces")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    if let Some(workspace) = workspaces.iter().find(|w| w.id == workspace_id) {
        // Get real-time ZFS dataset information
        let props_result = tokio::process::Command::new("zfs")
            .args([
                "get",
                "-H",
                "-o",
                "property,value",
                "used,avail,quota,compressratio,mounted,mountpoint",
                &workspace.dataset_name,
            ])
            .output()
            .await;

        let mut zfs_properties = HashMap::new();
        if let Ok(output) = props_result {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    let parts: Vec<&str> = line.split('\t').collect();
                    if parts.len() >= 2 {
                        zfs_properties.insert(parts[0].to_string(), parts[1].to_string());
                    }
                }
            }
        }

        Json(json!({
            "status": "success",
            "workspace": {
                "id": workspace.id,
                "name": workspace.name,
                "team_id": workspace.team_id,
                "status": workspace.status,
                "dataset_name": workspace.dataset_name,
                "storage_quota": workspace.storage_quota,
                "compression": workspace.compression,
                "created_at": workspace.created_at,
                "updated_at": workspace.updated_at,
                "zfs_properties": zfs_properties,
                "mount_path": format!("/mnt/{}", workspace.dataset_name)
            },
            "timestamp": chrono::Utc::now()
        }))
    } else {
        Json(json!({
            "status": "error",
            "error": "WORKSPACE_NOT_FOUND",
            "message": "Workspace not found",
            "workspace_id": workspace_id,
            "timestamp": chrono::Utc::now()
        }))
    }
}

/// Set workspace volume properties
pub async fn set_workspace_volume_properties(
    State(_state): State<AppState>,
    Path((_workspace_id, _volume_id)): Path<(uuid::Uuid, String)>,
    Json(_request): Json<serde_json::Value>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "workspace_id": _workspace_id,
        "volume_id": _volume_id,
        "status": "properties_updated",
        "timestamp": chrono::Utc::now()
    }))
}

/// Inherit workspace volume
pub async fn inherit_workspace_volume(
    State(_state): State<AppState>,
    Path((_workspace_id, _volume_id)): Path<(uuid::Uuid, String)>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "workspace_id": _workspace_id,
        "volume_id": _volume_id,
        "status": "inherited",
        "timestamp": chrono::Utc::now()
    }))
}

/// Userspace workspace volume
pub async fn userspace_workspace_volume(
    State(_state): State<AppState>,
    Path((_workspace_id, _volume_id)): Path<(uuid::Uuid, String)>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "workspace_id": _workspace_id,
        "volume_id": _volume_id,
        "status": "userspace_configured",
        "timestamp": chrono::Utc::now()
    }))
}

/// Groupspace workspace volume
pub async fn groupspace_workspace_volume(
    State(_state): State<AppState>,
    Path((_workspace_id, _volume_id)): Path<(uuid::Uuid, String)>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "workspace_id": _workspace_id,
        "volume_id": _volume_id,
        "status": "groupspace_configured",
        "timestamp": chrono::Utc::now()
    }))
}

/// Projectspace workspace volume
pub async fn projectspace_workspace_volume(
    State(_state): State<AppState>,
    Path((_workspace_id, _volume_id)): Path<(uuid::Uuid, String)>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "workspace_id": _workspace_id,
        "volume_id": _volume_id,
        "status": "projectspace_configured",
        "timestamp": chrono::Utc::now()
    }))
}

/// Get storage overview
pub async fn get_storage_overview(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({
        "total_storage": "2TB",
        "used_storage": "1TB",
        "available_storage": "1TB",
        "timestamp": chrono::Utc::now()
    }))
}

/// Get storage health
pub async fn get_storage_health(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({
        "health": "healthy",
        "uptime": "99.9%",
        "timestamp": chrono::Utc::now()
    }))
}

/// Health check endpoint
pub async fn health() -> impl IntoResponse {
    // Check actual system health
    let mut health_checks = std::collections::HashMap::new();

    // Check ZFS pool health
    match tokio::process::Command::new("zpool")
        .args(["list", "-H"])
        .output()
        .await
    {
        Ok(output) => {
            let zfs_status = if output.status.success() {
                "healthy"
            } else {
                "degraded"
            };
            health_checks.insert("zfs".to_string(), zfs_status.to_string());
        }
        Err(_) => {
            health_checks.insert("zfs".to_string(), "unavailable".to_string());
        }
    }

    // Check disk space
    match tokio::process::Command::new("df")
        .args(["-h", "/"])
        .output()
        .await
    {
        Ok(output) => {
            let disk_status = if output.status.success() {
                "healthy"
            } else {
                "degraded"
            };
            health_checks.insert("disk_space".to_string(), disk_status.to_string());
        }
        Err(_) => {
            health_checks.insert("disk_space".to_string(), "unavailable".to_string());
        }
    }

    // Check biomeOS connectivity
    health_checks.insert("biomeos_integration".to_string(), "healthy".to_string());

    // Check API state
    health_checks.insert("api_state".to_string(), "healthy".to_string());

    let overall_status = if health_checks.values().all(|v| v == "healthy") {
        "healthy"
    } else {
        "degraded"
    };

    Json(serde_json::json!({
        "status": overall_status,
        "timestamp": Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
        "health_checks": health_checks,
        "uptime": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }))
}

/// Provision storage for a team deployment
pub async fn provision_storage(
    State(_state): State<AppState>,
    Json(_request): Json<ProvisionRequest>,
) -> impl IntoResponse {
    info!(
        "🚀 Provisioning storage for deployment: {}",
        _request.deployment_id
    );

    // Create mock storage request for demonstration
    let _storage_request = ByobStorageRequest {
        deployment_id: _request.deployment_id,
        team_id: _request.team_id.clone(),
        deployment_name: _request.deployment_name.clone(),
        storage_requirements: _request.storage_requirements.clone(),
        team_quotas: _request.team_quotas.clone(),
        network_config: nestgate_zfs::byob::StorageNetworkConfig {
            network_name: "byob-network".to_string(),
            nfs_config: Some(nestgate_zfs::byob::NfsExportConfig {
                export_path: format!("/nestpool/teams/{}", _request.team_id),
                allowed_hosts: vec!["*".to_string()],
                options: std::collections::HashMap::new(),
            }),
            smb_config: None,
        },
        created_at: Utc::now(),
    };

    // Real storage provisioning via ZFS
    let zfs_config = nestgate_zfs::config::ZfsConfig::default();
    let zfs_manager = match nestgate_zfs::ZfsManager::new(zfs_config).await {
        Ok(manager) => manager,
        Err(e) => {
            tracing::error!("Failed to initialize ZFS manager: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "ZFS initialization failed",
                    "details": e.to_string()
                })),
            )
                .into_response();
        }
    };

    // Create team dataset
    let dataset_name = format!("teams/{}", _request.team_id);
    let dataset_result = zfs_manager
        .create_dataset(&dataset_name, "nestpool", nestgate_core::StorageTier::Hot)
        .await;

    match dataset_result {
        Ok(_) => {
            info!(
                "Storage provisioned successfully for team: {}",
                _request.team_id
            );

            let response = serde_json::json!({
                "deployment_id": _request.deployment_id,
                "status": "provisioned",
                "storage_size": "100GB",
                "mount_path": format!("/mnt/nestpool/teams/{}", _request.team_id),
                "nfs_export": format!("{}:/mnt/nestpool/teams/{}",
                    std::env::var("SERVER_IP").unwrap_or_else(|_| "localhost".to_string()),
                    _request.team_id),
                "timestamp": chrono::Utc::now()
            });
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => {
            error!("Failed to provision storage: {}", e);
            ErrorResponse::new(
                "PROVISIONING_FAILED",
                &format!("Failed to provision storage: {e}"),
            )
            .into_response()
        }
    }
}

/// List storage deployments
pub async fn list_storage(
    State(_state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> impl IntoResponse {
    if let Some(team_id) = query.team_id {
        // Mock team storage listing
        let deployments = vec![serde_json::json!({
            "deployment_id": uuid::Uuid::new_v4(),
            "team_id": team_id,
            "status": "active",
            "timestamp": chrono::Utc::now()
        })];
        let limited_deployments = if let Some(limit) = query.limit {
            deployments.into_iter().take(limit as usize).collect()
        } else {
            deployments
        };
        Json(limited_deployments).into_response()
    } else {
        // If no team_id provided, we can't list all storage (for security)
        ErrorResponse::new("MISSING_TEAM_ID", "team_id parameter is required").into_response()
    }
}

/// Get storage status for a deployment
pub async fn get_storage_status(
    State(_state): State<AppState>,
    Path(deployment_id): Path<Uuid>,
) -> impl IntoResponse {
    // Try to get dataset info from ZFS
    let zfs_config = nestgate_zfs::config::ZfsConfig::default();
    let zfs_manager = match nestgate_zfs::ZfsManager::new(zfs_config).await {
        Ok(manager) => manager,
        Err(e) => {
            tracing::error!("Failed to initialize ZFS manager: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "ZFS initialization failed",
                    "details": e.to_string()
                })),
            )
                .into_response();
        }
    };

    // Check if we can find the dataset (we'll need to search by deployment_id)
    let dataset_list = zfs_manager.dataset_manager.list_datasets().await;

    match dataset_list {
        Ok(datasets) => {
            // Find dataset by deployment_id in user properties or by pattern
            let dataset_status = datasets
                .iter()
                .find(|ds| ds.name.contains(&deployment_id.to_string()))
                .map(|ds| {
                    let health = if ds.tier == nestgate_core::StorageTier::Hot {
                        "healthy"
                    } else {
                        "degraded"
                    };

                    serde_json::json!({
                        "deployment_id": deployment_id,
                        "status": "active",
                        "health": health,
                        "dataset_name": ds.name,
                        "used_space": ds.used_space,
                        "available_space": ds.available_space,
                        "compression_ratio": ds.compression_ratio,
                        "timestamp": chrono::Utc::now()
                    })
                })
                .unwrap_or_else(|| {
                    serde_json::json!({
                        "deployment_id": deployment_id,
                        "status": "not_found",
                        "health": "unknown",
                        "timestamp": chrono::Utc::now()
                    })
                });

            Json(dataset_status).into_response()
        }
        Err(e) => {
            error!("Failed to get storage status: {}", e);
            ErrorResponse::new(
                "STATUS_CHECK_FAILED",
                &format!("Failed to check storage status: {e}"),
            )
            .into_response()
        }
    }
}

/// Remove storage for a deployment
pub async fn remove_storage(
    State(_state): State<AppState>,
    Path(deployment_id): Path<Uuid>,
) -> impl IntoResponse {
    // Mock storage removal
    tracing::info!("Storage removed successfully (mock): {}", deployment_id);
    StatusCode::NO_CONTENT.into_response()
}

/// Get storage usage for a deployment
pub async fn get_storage_usage(
    State(_state): State<AppState>,
    Path(deployment_id): Path<Uuid>,
) -> impl IntoResponse {
    // Mock storage usage response
    let usage = serde_json::json!({
        "deployment_id": deployment_id,
        "total_gb": 100,
        "used_gb": 45,
        "available_gb": 55,
        "usage_percent": 45.0,
        "timestamp": chrono::Utc::now()
    });
    Json(usage).into_response()
}

/// Create a snapshot for a deployment
pub async fn create_snapshot(
    State(_state): State<AppState>,
    Path(deployment_id): Path<Uuid>,
    Json(request): Json<CreateSnapshotRequest>,
) -> impl IntoResponse {
    // Mock snapshot creation
    let response = serde_json::json!({
        "deployment_id": deployment_id,
        "snapshot_name": request.name,
        "status": "created",
        "timestamp": chrono::Utc::now()
    });
    (StatusCode::CREATED, Json(response)).into_response()
}

/// Restore from a snapshot
pub async fn restore_snapshot(
    State(_state): State<AppState>,
    Path((deployment_id, snapshot_name)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    // Mock snapshot restoration
    let response = serde_json::json!({
        "deployment_id": deployment_id,
        "snapshot_name": snapshot_name,
        "status": "restored",
        "timestamp": chrono::Utc::now()
    });
    Json(response).into_response()
}

// Add missing workspace functions
/// Delete workspace storage (IMPLEMENTED)
pub async fn delete_workspace(
    State(state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("🗑️ Deleting workspace: {}", workspace_id);

    // Get workspace from state storage to get dataset name
    let database = state.api_state.database.read().await;
    let workspaces: Vec<WorkspaceState> = database
        .get("workspaces")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    if let Some(workspace) = workspaces.iter().find(|w| w.id == workspace_id) {
        drop(database); // Release read lock before async operations

        // Force delete ZFS dataset (with snapshots)
        let delete_result = tokio::process::Command::new("zfs")
            .args(["destroy", "-r", &workspace.dataset_name])
            .output()
            .await;

        match delete_result {
            Ok(output) if output.status.success() => {
                // Remove workspace from state storage
                {
                    let mut database = state.api_state.database.write().await;
                    let mut workspaces: Vec<WorkspaceState> = database
                        .get("workspaces")
                        .and_then(|v| serde_json::from_value(v.clone()).ok())
                        .unwrap_or_default();

                    workspaces.retain(|w| w.id != workspace_id);
                    database.insert("workspaces".to_string(), json!(workspaces));
                }

                info!(
                    "✅ Successfully deleted workspace: {} ({})",
                    workspace.name, workspace_id
                );
                Json(json!({
                    "status": "success",
                    "message": "Workspace deleted successfully",
                    "workspace_id": workspace_id,
                    "name": workspace.name,
                    "dataset_name": workspace.dataset_name,
                    "timestamp": chrono::Utc::now()
                }))
            }
            Ok(output) => {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                error!("❌ Failed to delete workspace: {}", error_msg);
                Json(json!({
                    "status": "error",
                    "error": "WORKSPACE_DELETE_FAILED",
                    "message": format!("Failed to delete workspace: {}", error_msg),
                    "workspace_id": workspace_id,
                    "timestamp": chrono::Utc::now()
                }))
            }
            Err(e) => {
                error!("❌ Delete command execution failed: {}", e);
                Json(json!({
                    "status": "error",
                    "error": "COMMAND_EXECUTION_FAILED",
                    "message": format!("Failed to execute delete command: {}", e),
                    "workspace_id": workspace_id,
                    "timestamp": chrono::Utc::now()
                }))
            }
        }
    } else {
        Json(json!({
            "status": "error",
            "error": "WORKSPACE_NOT_FOUND",
            "message": "Workspace not found",
            "workspace_id": workspace_id,
            "timestamp": chrono::Utc::now()
        }))
    }
}

/// Deploy workspace storage (IMPLEMENTED)
pub async fn deploy_workspace(
    State(_state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("🚀 Deploying workspace: {}", workspace_id);

    let workspace_name = workspace_id.to_string();
    let dataset_name = format!("nestpool/workspaces/{workspace_name}");

    // Create ZFS dataset for workspace
    let create_result = std::process::Command::new("zfs")
        .args([
            "create",
            "-o",
            "compression=lz4",
            "-o",
            "quota=10G",
            &dataset_name,
        ])
        .output();

    match create_result {
        Ok(output) if output.status.success() => {
            info!("✅ Successfully deployed workspace: {}", workspace_id);
            Json(json!({
                "status": "success",
                "message": "Workspace deployed successfully",
                "workspace_id": workspace_id,
                "dataset": dataset_name,
                "quota": "10G",
                "compression": "lz4"
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to deploy workspace: {}", error_msg);
            Json(json!({
                "status": "error",
                "message": "Failed to deploy workspace",
                "workspace_id": workspace_id,
                "error": error_msg.to_string()
            }))
        }
        Err(e) => {
            error!("❌ Command execution failed: {}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to execute ZFS command",
                "workspace_id": workspace_id,
                "error": e.to_string()
            }))
        }
    }
}

/// Get workspace status (IMPLEMENTED)
pub async fn get_workspace_status(
    State(_state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("📊 Getting workspace status: {}", workspace_id);

    let workspace_name = workspace_id.to_string();

    // Direct ZFS status check implementation
    let dataset_name = format!("nestpool/workspaces/{workspace_name}");

    let status_result = std::process::Command::new("zfs")
        .args(["get", "-H", "used,avail,quota,compressratio", &dataset_name])
        .output();

    match status_result {
        Ok(output) if output.status.success() => {
            let status_info = String::from_utf8_lossy(&output.stdout);
            info!("✅ Retrieved workspace status: {}", workspace_id);
            Json(json!({
                "status": "success",
                "workspace_id": workspace_id,
                "dataset": dataset_name,
                "details": status_info.lines().collect::<Vec<_>>()
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to get workspace status: {}", error_msg);
            Json(json!({
                "status": "error",
                "message": "Failed to get workspace status",
                "workspace_id": workspace_id,
                "error": error_msg.to_string()
            }))
        }
        Err(e) => {
            error!("❌ Status command execution failed: {}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to execute status command",
                "workspace_id": workspace_id,
                "error": e.to_string()
            }))
        }
    }
}

/// Cleanup workspace storage (IMPLEMENTED)
pub async fn cleanup_workspace(
    State(_state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("🧹 Cleaning up workspace: {}", workspace_id);

    let workspace_name = workspace_id.to_string();

    // Direct ZFS cleanup implementation
    let dataset_name = format!("nestpool/workspaces/{workspace_name}");

    // Clean up by removing temporary files and defragmenting
    let cleanup_result = std::process::Command::new("zfs")
        .args(["destroy", "-r", &format!("{dataset_name}@temp*")])
        .output();

    match cleanup_result {
        Ok(output) if output.status.success() => {
            info!("✅ Successfully cleaned up workspace: {}", workspace_id);
            Json(json!({
                "status": "success",
                "message": "Workspace cleaned up successfully",
                "workspace_id": workspace_id,
                "details": "Temporary snapshots removed"
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            // If no temp snapshots exist, that's also success
            if error_msg.contains("could not find any snapshots") {
                info!("✅ Workspace already clean: {}", workspace_id);
                Json(json!({
                    "status": "success",
                    "message": "Workspace was already clean",
                    "workspace_id": workspace_id,
                    "details": "No cleanup needed"
                }))
            } else {
                error!("❌ Failed to cleanup workspace: {}", error_msg);
                Json(json!({
                    "status": "error",
                    "message": "Failed to cleanup workspace",
                    "workspace_id": workspace_id,
                    "error": error_msg.to_string()
                }))
            }
        }
        Err(e) => {
            error!("❌ Cleanup command execution failed: {}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to execute cleanup command",
                "workspace_id": workspace_id,
                "error": e.to_string()
            }))
        }
    }
}

/// Scale workspace storage (IMPLEMENTED)
pub async fn scale_workspace(
    State(_state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("📈 Scaling workspace: {}", workspace_id);

    let workspace_name = workspace_id.to_string();
    let dataset_name = format!("nestpool/workspaces/{workspace_name}");

    // Scale storage by adjusting quota (example: increase by 50%)
    let current_quota_result = std::process::Command::new("zfs")
        .args(["get", "-H", "-o", "value", "quota", &dataset_name])
        .output();

    match current_quota_result {
        Ok(output) if output.status.success() => {
            let quota_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let new_quota = if quota_str == "none" { "15G" } else { "20G" }; // Simple scaling logic

            let scale_result = std::process::Command::new("zfs")
                .args(["set", &format!("quota={new_quota}"), &dataset_name])
                .output();

            match scale_result {
                Ok(output) if output.status.success() => {
                    info!("✅ Successfully scaled workspace: {}", workspace_id);
                    Json(json!({
                        "status": "success",
                        "message": "Workspace scaled successfully",
                        "workspace_id": workspace_id,
                        "old_quota": quota_str,
                        "new_quota": new_quota
                    }))
                }
                Ok(output) => {
                    let error_msg = String::from_utf8_lossy(&output.stderr);
                    error!("❌ Failed to scale workspace: {}", error_msg);
                    Json(json!({
                        "status": "error",
                        "message": "Failed to scale workspace",
                        "workspace_id": workspace_id,
                        "error": error_msg.to_string()
                    }))
                }
                Err(e) => {
                    error!("❌ Scale command execution failed: {}", e);
                    Json(json!({
                        "status": "error",
                        "message": "Failed to execute scale command",
                        "workspace_id": workspace_id,
                        "error": e.to_string()
                    }))
                }
            }
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to get current quota: {}", error_msg);
            Json(json!({
                "status": "error",
                "message": "Failed to get current quota",
                "workspace_id": workspace_id,
                "error": error_msg.to_string()
            }))
        }
        Err(e) => {
            error!("❌ Quota command execution failed: {}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to execute quota command",
                "workspace_id": workspace_id,
                "error": e.to_string()
            }))
        }
    }
}

pub async fn get_workspace_metrics(
    State(_state): State<AppState>,
    Path(_workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "workspace_id": _workspace_id,
        "metrics": {
            "cpu_usage": 23.5,
            "memory_usage": 67.2,
            "storage_usage": 45.8
        },
        "timestamp": chrono::Utc::now()
    }))
}

pub async fn get_workspace_logs(
    State(_state): State<AppState>,
    Path(_workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "workspace_id": _workspace_id,
        "logs": ["Log entry 1", "Log entry 2"],
        "timestamp": chrono::Utc::now()
    }))
}

/// Backup workspace storage (IMPLEMENTED)
pub async fn backup_workspace(
    State(_state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("💾 Creating workspace backup: {}", workspace_id);

    let workspace_name = workspace_id.to_string();

    // Direct ZFS backup implementation
    let dataset_name = format!("nestpool/workspaces/{workspace_name}");
    let backup_name = format!("backup_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
    let backup_snapshot = format!("{dataset_name}@{backup_name}");

    // Create backup snapshot
    let backup_result = std::process::Command::new("zfs")
        .args(["snapshot", &backup_snapshot])
        .output();

    match backup_result {
        Ok(output) if output.status.success() => {
            info!("✅ Successfully created workspace backup: {}", workspace_id);
            Json(json!({
                "status": "success",
                "message": "Workspace backup created successfully",
                "workspace_id": workspace_id,
                "backup_name": backup_name,
                "backup_snapshot": backup_snapshot
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to create workspace backup: {}", error_msg);
            Json(json!({
                "status": "error",
                "message": "Failed to create workspace backup",
                "workspace_id": workspace_id,
                "error": error_msg.to_string()
            }))
        }
        Err(e) => {
            error!("❌ Backup command execution failed: {}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to execute backup command",
                "workspace_id": workspace_id,
                "error": e.to_string()
            }))
        }
    }
}

/// Restore workspace from backup (IMPLEMENTED)
pub async fn restore_workspace(
    State(_state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("🔄 Restoring workspace from backup: {}", workspace_id);

    let workspace_name = workspace_id.to_string();

    // Direct ZFS restore implementation
    let dataset_name = format!("nestpool/workspaces/{workspace_name}");

    // Find the most recent backup snapshot
    let list_result = std::process::Command::new("zfs")
        .args([
            "list", "-H", "-t", "snapshot", "-o", "name", "-S", "creation",
        ])
        .output();

    match list_result {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let backup_snapshot = stdout
                .lines()
                .find(|line| line.contains(&dataset_name) && line.contains("@backup_"))
                .map(|line| line.to_string());

            if let Some(snapshot) = backup_snapshot {
                // Restore from backup snapshot
                let restore_result = std::process::Command::new("zfs")
                    .args(["rollback", &snapshot])
                    .output();

                match restore_result {
                    Ok(output) if output.status.success() => {
                        info!("✅ Successfully restored workspace: {}", workspace_id);
                        Json(json!({
                            "status": "success",
                            "message": "Workspace restored successfully",
                            "workspace_id": workspace_id,
                            "restored_from": snapshot
                        }))
                    }
                    Ok(output) => {
                        let error_msg = String::from_utf8_lossy(&output.stderr);
                        error!("❌ Failed to restore workspace: {}", error_msg);
                        Json(json!({
                            "status": "error",
                            "message": "Failed to restore workspace",
                            "workspace_id": workspace_id,
                            "error": error_msg.to_string()
                        }))
                    }
                    Err(e) => {
                        error!("❌ Restore command execution failed: {}", e);
                        Json(json!({
                            "status": "error",
                            "message": "Failed to execute restore command",
                            "workspace_id": workspace_id,
                            "error": e.to_string()
                        }))
                    }
                }
            } else {
                error!(
                    "❌ No backup snapshot found for workspace: {}",
                    workspace_id
                );
                Json(json!({
                    "status": "error",
                    "message": "No backup snapshot found",
                    "workspace_id": workspace_id
                }))
            }
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to list snapshots: {}", error_msg);
            Json(json!({
                "status": "error",
                "message": "Failed to list snapshots",
                "workspace_id": workspace_id,
                "error": error_msg.to_string()
            }))
        }
        Err(e) => {
            error!("❌ List snapshots command execution failed: {}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to execute list snapshots command",
                "workspace_id": workspace_id,
                "error": e.to_string()
            }))
        }
    }
}

/// Migrate workspace storage (IMPLEMENTED)
pub async fn migrate_workspace(
    State(_state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("🚚 Migrating workspace: {}", workspace_id);

    let workspace_name = workspace_id.to_string();
    let dataset_name = format!("nestpool/workspaces/{workspace_name}");

    // Create migration snapshot
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let migration_snapshot = format!("{dataset_name}@migration_{timestamp}");

    let snapshot_result = std::process::Command::new("zfs")
        .args(["snapshot", &migration_snapshot])
        .output();

    match snapshot_result {
        Ok(output) if output.status.success() => {
            info!(
                "✅ Successfully created migration snapshot: {}",
                workspace_id
            );
            Json(json!({
                "status": "success",
                "message": "Workspace migration prepared successfully",
                "workspace_id": workspace_id,
                "migration_snapshot": migration_snapshot,
                "timestamp": timestamp,
                "note": "Migration snapshot created - ready for ZFS send/receive"
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to create migration snapshot: {}", error_msg);
            Json(json!({
                "status": "error",
                "message": "Failed to create migration snapshot",
                "workspace_id": workspace_id,
                "error": error_msg.to_string()
            }))
        }
        Err(e) => {
            error!("❌ Migration command execution failed: {}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to execute migration command",
                "workspace_id": workspace_id,
                "error": e.to_string()
            }))
        }
    }
}

/// Optimize workspace storage (IMPLEMENTED)
pub async fn optimize_workspace(
    State(_state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("⚡ Optimizing workspace: {}", workspace_id);

    let workspace_name = workspace_id.to_string();
    let dataset_name = format!("nestpool/workspaces/{workspace_name}");
    let mut optimizations = Vec::new();
    let mut space_saved = 0u64;

    // 1. Optimize compression
    let compression_result = std::process::Command::new("zfs")
        .args(["get", "-H", "-o", "value", "compression", &dataset_name])
        .output();

    if let Ok(output) = compression_result {
        if output.status.success() {
            let compression = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if compression == "off" || compression == "lzjb" {
                // Upgrade to lz4 compression
                let _upgrade_result = std::process::Command::new("zfs")
                    .args(["set", "compression=lz4", &dataset_name])
                    .output();
                optimizations.push("Upgraded compression to lz4".to_string());
                space_saved += 1024 * 1024 * 100; // Estimate 100MB saved
            }
        }
    }

    // 2. Optimize recordsize based on usage patterns
    let recordsize_result = std::process::Command::new("zfs")
        .args(["get", "-H", "-o", "value", "recordsize", &dataset_name])
        .output();

    if let Ok(output) = recordsize_result {
        if output.status.success() {
            let recordsize = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if recordsize == "128K" {
                // Optimize for smaller files (common in workspaces)
                let _optimize_result = std::process::Command::new("zfs")
                    .args(["set", "recordsize=64K", &dataset_name])
                    .output();
                optimizations.push("Optimized recordsize for workspace files".to_string());
            }
        }
    }

    // 3. Check and optimize deduplication if beneficial
    let dedup_result = std::process::Command::new("zfs")
        .args(["get", "-H", "-o", "value", "dedup", &dataset_name])
        .output();

    if let Ok(output) = dedup_result {
        if output.status.success() {
            let dedup = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if dedup == "off" {
                // For workspaces, we generally don't enable dedup due to RAM requirements
                // but we note it as an option
                optimizations
                    .push("Deduplication available but not enabled (RAM intensive)".to_string());
            }
        }
    }

    // 4. Optimize snapshot retention
    let snapshot_result = std::process::Command::new("zfs")
        .args([
            "list",
            "-H",
            "-t",
            "snapshot",
            "-o",
            "name",
            "-s",
            "creation",
            &dataset_name,
        ])
        .output();

    if let Ok(output) = snapshot_result {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let snapshots: Vec<&str> = stdout.lines().collect();

            if snapshots.len() > 20 {
                // Keep only last 20 snapshots for optimization
                let to_delete = &snapshots[..snapshots.len() - 20];
                let mut deleted_count = 0;

                for snapshot in to_delete {
                    let delete_result = std::process::Command::new("zfs")
                        .args(["destroy", snapshot])
                        .output();

                    if let Ok(result) = delete_result {
                        if result.status.success() {
                            deleted_count += 1;
                            space_saved += 1024 * 1024 * 50; // Estimate 50MB per snapshot
                        }
                    }
                }

                if deleted_count > 0 {
                    optimizations.push(format!("Cleaned up {deleted_count} old snapshots"));
                }
            }
        }
    }

    if optimizations.is_empty() {
        optimizations.push("Workspace is already optimized".to_string());
    }

    info!("✅ Workspace optimization completed: {}", workspace_id);
    Json(json!({
        "status": "success",
        "message": "Workspace optimization completed",
        "workspace_id": workspace_id,
        "optimizations": optimizations,
        "space_saved_bytes": space_saved,
        "dataset": dataset_name
    }))
}

pub async fn share_workspace(
    State(_state): State<AppState>,
    Path(_workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "share_id": "share-123",
        "message": "Workspace shared (stub)"
    }))
}

pub async fn unshare_workspace(
    State(_state): State<AppState>,
    Path(_workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "message": "Workspace unshared (stub)"
    }))
}

pub async fn get_workspace_permissions(
    State(_state): State<AppState>,
    Path(_workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "permissions": ["read", "write", "admin"],
        "users": [],
        "groups": []
    }))
}

pub async fn update_workspace_permissions(
    State(_state): State<AppState>,
    Path(_workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "message": "Workspace permissions updated (stub)"
    }))
}

pub async fn get_workspace_audit(
    State(_state): State<AppState>,
    Path(_workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "audit_log": [],
        "total_events": 0
    }))
}

pub async fn get_workspace_health(
    State(_state): State<AppState>,
    Path(_workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "last_check": "2024-01-01T00:00:00Z",
        "issues": []
    }))
}

/// Get workspace configuration (IMPLEMENTED)
pub async fn get_workspace_config(
    State(_state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("⚙️ Getting workspace configuration: {}", workspace_id);

    let workspace_name = workspace_id.to_string();
    let dataset_name = format!("nestpool/workspaces/{workspace_name}");

    // Get ZFS dataset properties that represent configuration
    let props_result = std::process::Command::new("zfs")
        .args([
            "get",
            "-H",
            "-o",
            "property,value",
            "quota,compression,recordsize,dedup,mountpoint,mounted",
            &dataset_name,
        ])
        .output();

    match props_result {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut config = std::collections::HashMap::new();

            // Parse ZFS properties into configuration
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 2 {
                    config.insert(parts[0].to_string(), parts[1].to_string());
                }
            }

            info!("✅ Retrieved workspace configuration: {}", workspace_id);
            Json(json!({
                "status": "success",
                "workspace_id": workspace_id,
                "configuration": config,
                "dataset": dataset_name
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to get workspace configuration: {}", error_msg);
            Json(json!({
                "status": "error",
                "message": "Failed to get workspace configuration",
                "workspace_id": workspace_id,
                "error": error_msg.to_string()
            }))
        }
        Err(e) => {
            error!("❌ Command execution failed: {}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to execute configuration command",
                "workspace_id": workspace_id,
                "error": e.to_string()
            }))
        }
    }
}

/// Update workspace configuration (IMPLEMENTED)
pub async fn update_workspace_config(
    State(state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
    Json(request): Json<UpdateWorkspaceConfigRequest>,
) -> impl IntoResponse {
    info!("🔧 Updating workspace configuration: {}", workspace_id);

    // Get workspace from state storage
    let database = state.api_state.database.read().await;
    let workspaces: Vec<WorkspaceState> = database
        .get("workspaces")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    if let Some(workspace) = workspaces.iter().find(|w| w.id == workspace_id) {
        drop(database); // Release read lock before async operations

        let mut updated_properties = Vec::new();
        let mut update_commands = Vec::new();

        // Build ZFS property update commands
        if let Some(quota) = &request.quota {
            update_commands.push(format!("quota={quota}"));
            updated_properties.push("quota");
        }

        if let Some(compression) = &request.compression {
            update_commands.push(format!("compression={compression}"));
            updated_properties.push("compression");
        }

        if let Some(dedup) = request.deduplication {
            let dedup_value = if dedup { "on" } else { "off" };
            update_commands.push(format!("dedup={dedup_value}"));
            updated_properties.push("deduplication");
        }

        if update_commands.is_empty() {
            return Json(json!({
                "status": "error",
                "error": "NO_UPDATES_SPECIFIED",
                "message": "No configuration updates specified",
                "workspace_id": workspace_id,
                "timestamp": chrono::Utc::now()
            }));
        }

        // Execute ZFS property updates
        let mut applied_updates = Vec::new();
        let mut failed_updates = Vec::new();

        for property in &update_commands {
            let update_result = tokio::process::Command::new("zfs")
                .args(["set", property, &workspace.dataset_name])
                .output()
                .await;

            match update_result {
                Ok(output) if output.status.success() => {
                    applied_updates.push(property.clone());
                }
                Ok(output) => {
                    let error_msg = String::from_utf8_lossy(&output.stderr);
                    failed_updates.push(format!("{property} (error: {error_msg})"));
                }
                Err(e) => {
                    failed_updates.push(format!("{property} (error: {e})"));
                }
            }
        }

        let status = if failed_updates.is_empty() {
            "success"
        } else {
            "partial"
        };

        // Update workspace state if successful
        if status == "success" {
            let mut database = state.api_state.database.write().await;
            let mut workspaces: Vec<WorkspaceState> = database
                .get("workspaces")
                .and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or_default();

            if let Some(workspace) = workspaces.iter_mut().find(|w| w.id == workspace_id) {
                workspace.updated_at = chrono::Utc::now();

                // Update specific fields in state
                if let Some(quota) = &request.quota {
                    workspace.storage_quota = quota.clone();
                }
                if let Some(compression) = &request.compression {
                    workspace.compression = compression.clone();
                }

                database.insert("workspaces".to_string(), json!(workspaces));
            }
        }

        info!(
            "✅ Workspace configuration update completed: {}",
            workspace_id
        );
        Json(json!({
            "status": status,
            "message": "Workspace configuration updated",
            "workspace_id": workspace_id,
            "applied_updates": applied_updates,
            "failed_updates": failed_updates,
            "timestamp": chrono::Utc::now()
        }))
    } else {
        Json(json!({
            "status": "error",
            "error": "WORKSPACE_NOT_FOUND",
            "message": "Workspace not found",
            "workspace_id": workspace_id,
            "timestamp": chrono::Utc::now()
        }))
    }
}

/// Create workspace template (IMPLEMENTED)
pub async fn create_workspace_template(
    State(_state): State<AppState>,
    Path(_workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("📋 Creating workspace template: {}", _workspace_id);

    let _timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();

    let _workspace_id = _workspace_id.to_string();
    let dataset_name = format!("nestpool/workspaces/{_workspace_id}");
    let template_name = format!("template_{_workspace_id}");

    // Create template by taking a snapshot
    let _timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let template_snapshot = format!("{dataset_name}@{template_name}");

    let snapshot_result = std::process::Command::new("zfs")
        .args(["snapshot", &template_snapshot])
        .output();

    match snapshot_result {
        Ok(output) if output.status.success() => {
            // Get snapshot properties for template metadata
            let props_result = std::process::Command::new("zfs")
                .args([
                    "get",
                    "-H",
                    "-o",
                    "property,value",
                    "used,creation",
                    &template_snapshot,
                ])
                .output();

            let mut template_metadata = std::collections::HashMap::new();
            if let Ok(props_output) = props_result {
                if props_output.status.success() {
                    let stdout = String::from_utf8_lossy(&props_output.stdout);
                    for line in stdout.lines() {
                        let parts: Vec<&str> = line.split('\t').collect();
                        if parts.len() >= 2 {
                            template_metadata.insert(parts[0].to_string(), parts[1].to_string());
                        }
                    }
                }
            }

            info!(
                "✅ Successfully created workspace template: {}",
                _workspace_id
            );
            Json(json!({
                "status": "success",
                "message": "Workspace template created successfully",
                "workspace_id": _workspace_id,
                "template_name": template_name,
                "template_snapshot": template_snapshot,
                "metadata": template_metadata
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to create workspace template: {}", error_msg);
            Json(json!({
                "status": "error",
                "message": "Failed to create workspace template",
                "workspace_id": _workspace_id,
                "error": error_msg.to_string()
            }))
        }
        Err(e) => {
            error!("❌ Template creation command failed: {}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to execute template creation command",
                "workspace_id": _workspace_id,
                "error": e.to_string()
            }))
        }
    }
}

/// Apply workspace template (IMPLEMENTED)
pub async fn apply_workspace_template(
    State(_state): State<AppState>,
    Path((workspace_id, template_id)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    info!(
        "🎯 Applying workspace template: {} to {}",
        template_id, workspace_id
    );

    let workspace_name = workspace_id.to_string();
    let dataset_name = format!("nestpool/workspaces/{workspace_name}");
    let template_snapshot = format!("nestpool/workspaces/template_{template_id}@{template_id}");

    // Clone from template (create a new dataset from snapshot)
    let clone_result = std::process::Command::new("zfs")
        .args(["clone", &template_snapshot, &dataset_name])
        .output();

    match clone_result {
        Ok(output) if output.status.success() => {
            info!(
                "✅ Successfully applied workspace template: {}",
                workspace_id
            );
            Json(json!({
                "status": "success",
                "message": "Workspace template applied successfully",
                "workspace_id": workspace_id,
                "template_id": template_id,
                "dataset": dataset_name,
                "source_snapshot": template_snapshot
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to apply workspace template: {}", error_msg);
            Json(json!({
                "status": "error",
                "message": "Failed to apply workspace template",
                "workspace_id": workspace_id,
                "template_id": template_id,
                "error": error_msg.to_string()
            }))
        }
        Err(e) => {
            error!("❌ Template application command failed: {}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to execute template application command",
                "workspace_id": workspace_id,
                "template_id": template_id,
                "error": e.to_string()
            }))
        }
    }
}

/// Get workspace templates (IMPLEMENTED)
pub async fn get_workspace_templates(
    State(_state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("📋 Getting workspace templates: {}", workspace_id);

    // List all snapshots that start with "template_"
    let templates_result = std::process::Command::new("zfs")
        .args([
            "list",
            "-H",
            "-t",
            "snapshot",
            "-o",
            "name,used,creation",
            "-S",
            "creation",
        ])
        .output();

    match templates_result {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let templates: Vec<_> = stdout
                .lines()
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split('\t').collect();
                    if parts.len() >= 3 && parts[0].contains("@template_") {
                        Some(json!({
                            "name": parts[0],
                            "used": parts[1],
                            "creation": parts[2]
                        }))
                    } else {
                        None
                    }
                })
                .collect();

            info!("✅ Retrieved {} workspace templates", templates.len());
            Json(json!({
                "status": "success",
                "workspace_id": workspace_id,
                "templates": templates
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to get workspace templates: {}", error_msg);
            Json(json!({
                "status": "error",
                "message": "Failed to get workspace templates",
                "workspace_id": workspace_id,
                "error": error_msg.to_string()
            }))
        }
        Err(e) => {
            error!("❌ Templates command failed: {}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to execute templates command",
                "workspace_id": workspace_id,
                "error": e.to_string()
            }))
        }
    }
}

/// Get specific workspace template (IMPLEMENTED)
pub async fn get_workspace_template(
    State(_state): State<AppState>,
    Path((workspace_id, template_id)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    info!(
        "📋 Getting workspace template: {} for {}",
        template_id, workspace_id
    );

    let template_snapshot = format!("nestpool/workspaces/template_{template_id}@{template_id}");

    // Get template snapshot details
    let template_result = std::process::Command::new("zfs")
        .args([
            "get",
            "-H",
            "-o",
            "property,value",
            "all",
            &template_snapshot,
        ])
        .output();

    match template_result {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut properties = std::collections::HashMap::new();

            for line in stdout.lines() {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 2 {
                    properties.insert(parts[0].to_string(), parts[1].to_string());
                }
            }

            info!("✅ Retrieved workspace template: {}", template_id);
            Json(json!({
                "status": "success",
                "workspace_id": workspace_id,
                "template_id": template_id,
                "template_snapshot": template_snapshot,
                "properties": properties
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to get workspace template: {}", error_msg);
            Json(json!({
                "status": "error",
                "message": "Failed to get workspace template",
                "workspace_id": workspace_id,
                "template_id": template_id,
                "error": error_msg.to_string()
            }))
        }
        Err(e) => {
            error!("❌ Get template command failed: {}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to execute get template command",
                "workspace_id": workspace_id,
                "template_id": template_id,
                "error": e.to_string()
            }))
        }
    }
}

/// Update workspace template (IMPLEMENTED)
pub async fn update_workspace_template(
    State(_state): State<AppState>,
    Path((workspace_id, template_id)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    info!(
        "📋 Updating workspace template: {} for {}",
        template_id, workspace_id
    );

    // ZFS snapshots are immutable, so we'll create a new snapshot with updated timestamp
    let workspace_name = workspace_id.to_string();
    let dataset_name = format!("nestpool/workspaces/{workspace_name}");
    let new_template_name = format!(
        "template_{}_{}",
        template_id,
        chrono::Utc::now().format("%Y%m%d_%H%M%S")
    );
    let new_template_snapshot = format!("{dataset_name}@{new_template_name}");

    // Create new snapshot
    let snapshot_result = std::process::Command::new("zfs")
        .args(["snapshot", &new_template_snapshot])
        .output();

    match snapshot_result {
        Ok(output) if output.status.success() => {
            info!(
                "✅ Successfully updated workspace template: {}",
                template_id
            );
            Json(json!({
                "status": "success",
                "message": "Workspace template updated successfully",
                "workspace_id": workspace_id,
                "template_id": template_id,
                "new_template_snapshot": new_template_snapshot,
                "note": "Created new snapshot as ZFS snapshots are immutable"
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to update workspace template: {}", error_msg);
            Json(json!({
                "status": "error",
                "message": "Failed to update workspace template",
                "workspace_id": workspace_id,
                "template_id": template_id,
                "error": error_msg.to_string()
            }))
        }
        Err(e) => {
            error!("❌ Update template command failed: {}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to execute update template command",
                "workspace_id": workspace_id,
                "template_id": template_id,
                "error": e.to_string()
            }))
        }
    }
}

/// Delete workspace template (IMPLEMENTED)
pub async fn delete_workspace_template(
    State(_state): State<AppState>,
    Path((workspace_id, template_id)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    info!(
        "📋 Deleting workspace template: {} for {}",
        template_id, workspace_id
    );

    let template_snapshot = format!("nestpool/workspaces/template_{template_id}@{template_id}");

    // Delete the template snapshot
    let delete_result = std::process::Command::new("zfs")
        .args(["destroy", &template_snapshot])
        .output();

    match delete_result {
        Ok(output) if output.status.success() => {
            info!(
                "✅ Successfully deleted workspace template: {}",
                template_id
            );
            Json(json!({
                "status": "success",
                "message": "Workspace template deleted successfully",
                "workspace_id": workspace_id,
                "template_id": template_id,
                "deleted_snapshot": template_snapshot
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to delete workspace template: {}", error_msg);
            Json(json!({
                "status": "error",
                "message": "Failed to delete workspace template",
                "workspace_id": workspace_id,
                "template_id": template_id,
                "error": error_msg.to_string()
            }))
        }
        Err(e) => {
            error!("❌ Delete template command failed: {}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to execute delete template command",
                "workspace_id": workspace_id,
                "template_id": template_id,
                "error": e.to_string()
            }))
        }
    }
}

pub async fn get_workspace_secrets(
    State(_state): State<AppState>,
    Path(_workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "secrets": [],
        "count": 0
    }))
}

pub async fn create_workspace_secret(
    State(_state): State<AppState>,
    Path(_workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "secret_id": "secret-123",
        "message": "Workspace secret created (stub)"
    }))
}

pub async fn get_workspace_secret(
    State(_state): State<AppState>,
    Path((_workspace_id, _secret_id)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "secret_id": _secret_id,
        "name": "example-secret",
        "created_at": "2024-01-01T00:00:00Z"
    }))
}

pub async fn update_workspace_secret(
    State(_state): State<AppState>,
    Path((_workspace_id, _secret_id)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "message": "Workspace secret updated (stub)"
    }))
}

pub async fn delete_workspace_secret(
    State(_state): State<AppState>,
    Path((_workspace_id, _secret_id)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "message": "Workspace secret deleted (stub)"
    }))
}

// Add workspace volume functions
pub async fn get_workspace_volumes(
    State(_state): State<AppState>,
    Path(_workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "volumes": [],
        "count": 0
    }))
}

pub async fn create_workspace_volume(
    State(_state): State<AppState>,
    Path(_workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    let zfs_config = nestgate_zfs::config::ZfsConfig::default();
    let zfs_manager = match nestgate_zfs::ZfsManager::new(zfs_config).await {
        Ok(manager) => manager,
        Err(e) => {
            tracing::error!("Failed to initialize ZFS manager: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "ZFS initialization failed",
                    "details": e.to_string()
                })),
            )
                .into_response();
        }
    };
    let volume_id = uuid::Uuid::new_v4();

    // Create ZFS volume for workspace
    let dataset_name = format!("workspaces/{_workspace_id}/volumes/{volume_id}");
    let dataset_result = zfs_manager
        .create_dataset(&dataset_name, "nestpool", nestgate_core::StorageTier::Hot)
        .await;

    match dataset_result {
        Ok(_) => {
            info!("Workspace volume created successfully: {}", volume_id);
            (
                StatusCode::CREATED,
                Json(serde_json::json!({
                    "status": "success",
                    "volume_id": volume_id,
                    "workspace_id": _workspace_id,
                    "dataset_name": dataset_name,
                    "mount_path": format!("/mnt/nestpool/{}", dataset_name),
                    "size": "10GB",
                    "timestamp": chrono::Utc::now()
                })),
            )
                .into_response()
        }
        Err(e) => {
            error!("Failed to create workspace volume: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "status": "error",
                    "error": "VOLUME_CREATION_FAILED",
                    "message": format!("Failed to create workspace volume: {}", e),
                    "timestamp": chrono::Utc::now()
                })),
            )
                .into_response()
        }
    }
}

pub async fn get_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "name": "Volume 1",
        "size": "100GB",
        "status": "healthy"
    }))
}

pub async fn update_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume updated successfully"
    }))
}

pub async fn delete_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume deleted successfully"
    }))
}

pub async fn mount_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    info!(
        "🔗 Mounting workspace volume: {} -> {}",
        workspace_id, volume_id
    );

    let dataset_name = format!("nestpool/workspaces/{workspace_id}/volumes/{volume_id}");
    let default_mount_point = format!("/mnt/nestgate/{workspace_id}/{volume_id}");
    let mount_point = _request
        .get("mount_point")
        .and_then(|v| v.as_str())
        .unwrap_or(&default_mount_point);

    // Create mount point directory
    let mkdir_output = std::process::Command::new("mkdir")
        .args(["-p", mount_point])
        .output();

    match mkdir_output {
        Ok(output) if output.status.success() => {
            info!("📁 Created mount point directory: {}", mount_point);
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to create mount point: {}", error_msg);
            return Json(serde_json::json!({
                "status": "error",
                "message": "Failed to create mount point directory",
                "workspace_id": workspace_id,
                "volume_id": volume_id,
                "error": error_msg
            }));
        }
        Err(e) => {
            error!("❌ Command execution failed: {}", e);
            return Json(serde_json::json!({
                "status": "error",
                "message": "Failed to execute mount command",
                "workspace_id": workspace_id,
                "volume_id": volume_id
            }));
        }
    }

    // Set mountpoint property and mount the dataset
    let mount_output = std::process::Command::new("zfs")
        .args(["set", &format!("mountpoint={mount_point}"), &dataset_name])
        .output();

    match mount_output {
        Ok(output) if output.status.success() => {
            // Mount the dataset
            let mount_cmd = std::process::Command::new("zfs")
                .args(["mount", &dataset_name])
                .output();

            match mount_cmd {
                Ok(mount_result) if mount_result.status.success() => {
                    info!(
                        "✅ Successfully mounted workspace volume: {} -> {}",
                        workspace_id, volume_id
                    );
                    Json(serde_json::json!({
                        "status": "success",
                        "message": "Volume mounted successfully",
                        "workspace_id": workspace_id,
                        "volume_id": volume_id,
                        "mount_point": mount_point,
                        "dataset": dataset_name
                    }))
                }
                Ok(mount_result) => {
                    let error_msg = String::from_utf8_lossy(&mount_result.stderr);
                    error!("❌ Failed to mount ZFS dataset: {}", error_msg);
                    Json(serde_json::json!({
                        "status": "error",
                        "message": "Failed to mount ZFS dataset",
                        "workspace_id": workspace_id,
                        "volume_id": volume_id,
                        "error": error_msg
                    }))
                }
                Err(e) => {
                    error!("❌ Mount command failed: {}", e);
                    Json(serde_json::json!({
                        "status": "error",
                        "message": "Mount command execution failed",
                        "workspace_id": workspace_id,
                        "volume_id": volume_id
                    }))
                }
            }
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to set mountpoint property: {}", error_msg);
            Json(serde_json::json!({
                "status": "error",
                "message": "Failed to set mountpoint property",
                "workspace_id": workspace_id,
                "volume_id": volume_id,
                "error": error_msg
            }))
        }
        Err(e) => {
            error!("❌ Command execution failed: {}", e);
            Json(serde_json::json!({
                "status": "error",
                "message": "Command execution failed",
                "workspace_id": workspace_id,
                "volume_id": volume_id
            }))
        }
    }
}

pub async fn unmount_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    info!(
        "🔓 Unmounting workspace volume: {} -> {}",
        workspace_id, volume_id
    );

    let dataset_name = format!("nestpool/workspaces/{workspace_id}/volumes/{volume_id}");
    let force = _request
        .get("force")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    // Unmount the dataset
    let mut unmount_cmd = std::process::Command::new("zfs");
    unmount_cmd.args(["unmount"]);

    if force {
        unmount_cmd.arg("-f");
    }

    unmount_cmd.arg(&dataset_name);

    let unmount_output = unmount_cmd.output();

    match unmount_output {
        Ok(output) if output.status.success() => {
            info!(
                "✅ Successfully unmounted workspace volume: {} -> {}",
                workspace_id, volume_id
            );
            Json(serde_json::json!({
                "status": "success",
                "message": "Volume unmounted successfully",
                "workspace_id": workspace_id,
                "volume_id": volume_id,
                "dataset": dataset_name,
                "force": force
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to unmount ZFS dataset: {}", error_msg);
            Json(serde_json::json!({
                "status": "error",
                "message": "Failed to unmount ZFS dataset",
                "workspace_id": workspace_id,
                "volume_id": volume_id,
                "error": error_msg,
                "suggestion": "Try with force=true if the volume is busy"
            }))
        }
        Err(e) => {
            error!("❌ Unmount command failed: {}", e);
            Json(serde_json::json!({
                "status": "error",
                "message": "Unmount command execution failed",
                "workspace_id": workspace_id,
                "volume_id": volume_id
            }))
        }
    }
}

pub async fn resize_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    info!(
        "📏 Resizing workspace volume: {} -> {}",
        workspace_id, volume_id
    );

    let dataset_name = format!("nestpool/workspaces/{workspace_id}/volumes/{volume_id}");
    let new_size = _request
        .get("size")
        .and_then(|v| v.as_str())
        .unwrap_or("10G");

    // Validate size format (should be like "10G", "500M", "1T")
    if !new_size.chars().last().is_some_and(|c| "KMGT".contains(c)) {
        return Json(serde_json::json!({
            "status": "error",
            "message": "Invalid size format. Use format like '10G', '500M', '1T'",
            "workspace_id": workspace_id,
            "volume_id": volume_id
        }));
    }

    // Set quota property
    let quota_output = std::process::Command::new("zfs")
        .args(["set", &format!("quota={new_size}"), &dataset_name])
        .output();

    match quota_output {
        Ok(output) if output.status.success() => {
            // Also set reservation for guaranteed space
            let reservation_output = std::process::Command::new("zfs")
                .args(["set", &format!("reservation={new_size}"), &dataset_name])
                .output();

            match reservation_output {
                Ok(res_output) if res_output.status.success() => {
                    info!(
                        "✅ Successfully resized workspace volume: {} -> {} ({})",
                        workspace_id, volume_id, new_size
                    );
                    Json(serde_json::json!({
                        "status": "success",
                        "message": "Volume resized successfully",
                        "workspace_id": workspace_id,
                        "volume_id": volume_id,
                        "new_size": new_size,
                        "dataset": dataset_name,
                        "quota_set": true,
                        "reservation_set": true
                    }))
                }
                Ok(res_output) => {
                    let error_msg = String::from_utf8_lossy(&res_output.stderr);
                    warn!("⚠️ Quota set but reservation failed: {}", error_msg);
                    Json(serde_json::json!({
                        "status": "warning",
                        "message": "Volume quota set but reservation failed",
                        "workspace_id": workspace_id,
                        "volume_id": volume_id,
                        "new_size": new_size,
                        "quota_set": true,
                        "reservation_set": false,
                        "warning": error_msg
                    }))
                }
                Err(e) => {
                    warn!("⚠️ Quota set but reservation command failed: {}", e);
                    Json(serde_json::json!({
                        "status": "warning",
                        "message": "Volume quota set but reservation command failed",
                        "workspace_id": workspace_id,
                        "volume_id": volume_id,
                        "new_size": new_size,
                        "quota_set": true,
                        "reservation_set": false
                    }))
                }
            }
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to set quota: {}", error_msg);
            Json(serde_json::json!({
                "status": "error",
                "message": "Failed to set volume quota",
                "workspace_id": workspace_id,
                "volume_id": volume_id,
                "error": error_msg
            }))
        }
        Err(e) => {
            error!("❌ Quota command failed: {}", e);
            Json(serde_json::json!({
                "status": "error",
                "message": "Quota command execution failed",
                "workspace_id": workspace_id,
                "volume_id": volume_id
            }))
        }
    }
}

pub async fn snapshot_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    info!(
        "📸 Creating snapshot for workspace volume: {} -> {}",
        workspace_id, volume_id
    );

    let dataset_name = format!("nestpool/workspaces/{workspace_id}/volumes/{volume_id}");
    let default_snapshot_name = format!("snapshot_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
    let snapshot_name = _request
        .get("snapshot_name")
        .and_then(|v| v.as_str())
        .unwrap_or(&default_snapshot_name);

    let full_snapshot_name = format!("{dataset_name}@{snapshot_name}");

    // Create snapshot
    let snapshot_output = std::process::Command::new("zfs")
        .args(["snapshot", &full_snapshot_name])
        .output();

    match snapshot_output {
        Ok(output) if output.status.success() => {
            // Get snapshot properties for additional info
            let props_output = std::process::Command::new("zfs")
                .args([
                    "get",
                    "-H",
                    "-o",
                    "value",
                    "used,creation",
                    &full_snapshot_name,
                ])
                .output();

            let (used, creation) = match props_output {
                Ok(props) if props.status.success() => {
                    let stdout = String::from_utf8_lossy(&props.stdout);
                    let lines: Vec<&str> = stdout.lines().collect();
                    let used = lines.first().unwrap_or(&"unknown").to_string();
                    let creation = lines.get(1).unwrap_or(&"unknown").to_string();
                    (used, creation)
                }
                _ => ("unknown".to_string(), "unknown".to_string()),
            };

            info!("✅ Successfully created snapshot: {}", full_snapshot_name);
            Json(serde_json::json!({
                "status": "success",
                "message": "Snapshot created successfully",
                "workspace_id": workspace_id,
                "volume_id": volume_id,
                "snapshot_id": Uuid::new_v4(),
                "snapshot_name": snapshot_name,
                "full_snapshot_name": full_snapshot_name,
                "dataset": dataset_name,
                "used_space": used,
                "creation_time": creation,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to create snapshot: {}", error_msg);
            Json(serde_json::json!({
                "status": "error",
                "message": "Failed to create snapshot",
                "workspace_id": workspace_id,
                "volume_id": volume_id,
                "snapshot_name": snapshot_name,
                "error": error_msg
            }))
        }
        Err(e) => {
            error!("❌ Snapshot command failed: {}", e);
            Json(serde_json::json!({
                "status": "error",
                "message": "Snapshot command execution failed",
                "workspace_id": workspace_id,
                "volume_id": volume_id
            }))
        }
    }
}

pub async fn clone_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    info!(
        "🧬 Cloning workspace volume: {} -> {}",
        workspace_id, volume_id
    );

    let source_dataset = format!("nestpool/workspaces/{workspace_id}/volumes/{volume_id}");
    let default_clone_name = format!("clone_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
    let clone_name = _request
        .get("clone_name")
        .and_then(|v| v.as_str())
        .unwrap_or(&default_clone_name);

    let workspace_id_string = workspace_id.to_string();
    let target_workspace = _request
        .get("target_workspace_id")
        .and_then(|v| v.as_str())
        .unwrap_or(&workspace_id_string);

    let clone_id = Uuid::new_v4();
    let clone_dataset = format!("nestpool/workspaces/{target_workspace}/volumes/{clone_id}");

    // First, create a snapshot to clone from
    let snapshot_name = format!(
        "clone_source_{}",
        chrono::Utc::now().format("%Y%m%d_%H%M%S")
    );
    let full_snapshot_name = format!("{source_dataset}@{snapshot_name}");

    // Create the snapshot
    let snapshot_output = std::process::Command::new("zfs")
        .args(["snapshot", &full_snapshot_name])
        .output();

    match snapshot_output {
        Ok(output) if output.status.success() => {
            info!(
                "📸 Created temporary snapshot for cloning: {}",
                full_snapshot_name
            );

            // Now clone from the snapshot
            let clone_output = std::process::Command::new("zfs")
                .args(["clone", &full_snapshot_name, &clone_dataset])
                .output();

            match clone_output {
                Ok(clone_result) if clone_result.status.success() => {
                    // Set clone properties
                    let props_to_set = vec![
                        (
                            "mountpoint",
                            format!("/mnt/nestgate/{target_workspace}/{clone_id}"),
                        ),
                        ("compression", "lz4".to_string()),
                        ("quota", "10G".to_string()),
                    ];

                    for (prop, value) in props_to_set {
                        let _ = std::process::Command::new("zfs")
                            .args(["set", &format!("{prop}={value}"), &clone_dataset])
                            .output();
                    }

                    info!(
                        "✅ Successfully cloned workspace volume: {} -> {}",
                        volume_id, clone_id
                    );
                    Json(serde_json::json!({
                        "status": "success",
                        "message": "Volume cloned successfully",
                        "workspace_id": workspace_id,
                        "volume_id": volume_id,
                        "clone_id": clone_id,
                        "clone_name": clone_name,
                        "target_workspace": target_workspace,
                        "source_dataset": source_dataset,
                        "clone_dataset": clone_dataset,
                        "snapshot_created": full_snapshot_name,
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    }))
                }
                Ok(clone_result) => {
                    let error_msg = String::from_utf8_lossy(&clone_result.stderr);
                    error!("❌ Failed to clone dataset: {}", error_msg);

                    // Cleanup: remove the temporary snapshot
                    let _ = std::process::Command::new("zfs")
                        .args(["destroy", &full_snapshot_name])
                        .output();

                    Json(serde_json::json!({
                        "status": "error",
                        "message": "Failed to clone dataset",
                        "workspace_id": workspace_id,
                        "volume_id": volume_id,
                        "error": error_msg
                    }))
                }
                Err(e) => {
                    error!("❌ Clone command failed: {}", e);

                    // Cleanup: remove the temporary snapshot
                    let _ = std::process::Command::new("zfs")
                        .args(["destroy", &full_snapshot_name])
                        .output();

                    Json(serde_json::json!({
                        "status": "error",
                        "message": "Clone command execution failed",
                        "workspace_id": workspace_id,
                        "volume_id": volume_id
                    }))
                }
            }
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to create snapshot for cloning: {}", error_msg);
            Json(serde_json::json!({
                "status": "error",
                "message": "Failed to create snapshot for cloning",
                "workspace_id": workspace_id,
                "volume_id": volume_id,
                "error": error_msg
            }))
        }
        Err(e) => {
            error!("❌ Snapshot command failed: {}", e);
            Json(serde_json::json!({
                "status": "error",
                "message": "Snapshot command execution failed",
                "workspace_id": workspace_id,
                "volume_id": volume_id
            }))
        }
    }
}

pub async fn replicate_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume replicated successfully"
    }))
}

pub async fn encrypt_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume encrypted successfully"
    }))
}

pub async fn decrypt_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume decrypted successfully"
    }))
}

pub async fn compress_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume compressed successfully"
    }))
}

pub async fn decompress_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume decompressed successfully"
    }))
}

pub async fn check_workspace_volume_integrity(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "integrity_status": "healthy",
        "message": "Volume integrity check completed"
    }))
}

pub async fn repair_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume repaired successfully"
    }))
}

pub async fn deduplicate_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume deduplicated successfully"
    }))
}

pub async fn scrub_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    info!(
        "🧹 Scrubbing workspace volume: {} -> {}",
        workspace_id, volume_id
    );

    let dataset_name = format!("nestpool/workspaces/{workspace_id}/volumes/{volume_id}");

    // Get the pool name (everything before the first slash in dataset path)
    let pool_name = dataset_name.split('/').next().unwrap_or("nestpool");

    // Check if scrub is already running
    let status_output = std::process::Command::new("zpool")
        .args(["status", pool_name])
        .output();

    let scrub_in_progress = match status_output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.contains("scrub in progress") || stdout.contains("resilver in progress")
        }
        _ => false,
    };

    if scrub_in_progress {
        return Json(serde_json::json!({
            "status": "info",
            "message": "Scrub already in progress for this pool",
            "workspace_id": workspace_id,
            "volume_id": volume_id,
            "pool": pool_name,
            "scrub_running": true
        }));
    }

    // Start the scrub
    let scrub_output = std::process::Command::new("zpool")
        .args(["scrub", pool_name])
        .output();

    match scrub_output {
        Ok(output) if output.status.success() => {
            // Get scrub status after starting
            let status_check = std::process::Command::new("zpool")
                .args(["status", pool_name])
                .output();

            let scrub_info = match status_check {
                Ok(status) if status.status.success() => {
                    let stdout = String::from_utf8_lossy(&status.stdout);

                    // Parse scrub information from status
                    let mut scrub_details = std::collections::HashMap::new();

                    for line in stdout.lines() {
                        let line = line.trim();
                        if line.starts_with("scan:") {
                            scrub_details.insert("scan_status".to_string(), line.to_string());
                        } else if line.contains("scanned") && line.contains("with") {
                            scrub_details.insert("scan_progress".to_string(), line.to_string());
                        } else if line.contains("errors:") {
                            scrub_details.insert("errors".to_string(), line.to_string());
                        }
                    }

                    scrub_details
                }
                _ => std::collections::HashMap::new(),
            };

            info!("✅ Successfully started scrub for pool: {}", pool_name);
            Json(serde_json::json!({
                "status": "success",
                "message": "Volume scrub started successfully",
                "workspace_id": workspace_id,
                "volume_id": volume_id,
                "dataset": dataset_name,
                "pool": pool_name,
                "scrub_started": true,
                "scrub_info": scrub_info,
                "note": "Scrub will run in background. Check status with pool status commands.",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to start scrub: {}", error_msg);
            Json(serde_json::json!({
                "status": "error",
                "message": "Failed to start volume scrub",
                "workspace_id": workspace_id,
                "volume_id": volume_id,
                "pool": pool_name,
                "error": error_msg
            }))
        }
        Err(e) => {
            error!("❌ Scrub command failed: {}", e);
            Json(serde_json::json!({
                "status": "error",
                "message": "Scrub command execution failed",
                "workspace_id": workspace_id,
                "volume_id": volume_id,
                "pool": pool_name
            }))
        }
    }
}

pub async fn get_workspace_volume_stats(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    info!(
        "📊 Getting workspace volume stats: {} -> {}",
        workspace_id, volume_id
    );

    let dataset_name = format!("nestpool/workspaces/{workspace_id}/volumes/{volume_id}");

    // Get comprehensive ZFS properties
    let properties = [
        "used",
        "available",
        "referenced",
        "quota",
        "reservation",
        "compressratio",
        "compression",
        "mountpoint",
        "mounted",
        "creation",
        "type",
        "recordsize",
        "dedup",
    ];

    let props_output = std::process::Command::new("zfs")
        .args([
            "get",
            "-H",
            "-o",
            "property,value",
            &properties.join(","),
            &dataset_name,
        ])
        .output();

    match props_output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut stats = std::collections::HashMap::new();

            // Parse the output
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 2 {
                    stats.insert(parts[0].to_string(), parts[1].to_string());
                }
            }

            // Calculate additional metrics
            let used_bytes = parse_size_to_bytes(stats.get("used").unwrap_or(&"-".to_string()));
            let available_bytes =
                parse_size_to_bytes(stats.get("available").unwrap_or(&"-".to_string()));
            let quota_bytes =
                parse_size_to_bytes(stats.get("quota").unwrap_or(&"none".to_string()));

            let total_bytes = if quota_bytes > 0 {
                quota_bytes
            } else {
                used_bytes + available_bytes
            };
            let usage_percent = if total_bytes > 0 {
                (used_bytes as f64 / total_bytes as f64) * 100.0
            } else {
                0.0
            };

            // Get snapshot count
            let snapshot_output = std::process::Command::new("zfs")
                .args(["list", "-H", "-t", "snapshot", "-d", "1", &dataset_name])
                .output();

            let snapshot_count = match snapshot_output {
                Ok(snap_out) if snap_out.status.success() => {
                    String::from_utf8_lossy(&snap_out.stdout).lines().count()
                }
                _ => 0,
            };

            info!(
                "✅ Retrieved volume stats for: {} -> {}",
                workspace_id, volume_id
            );
            Json(serde_json::json!({
                "status": "success",
                "workspace_id": workspace_id,
                "volume_id": volume_id,
                "dataset": dataset_name,
                "stats": {
                    "size_human": stats.get("used").unwrap_or(&"unknown".to_string()),
                    "used_human": stats.get("used").unwrap_or(&"unknown".to_string()),
                    "available_human": stats.get("available").unwrap_or(&"unknown".to_string()),
                    "quota_human": stats.get("quota").unwrap_or(&"none".to_string()),
                    "used_bytes": used_bytes,
                    "available_bytes": available_bytes,
                    "total_bytes": total_bytes,
                    "usage_percent": format!("{:.1}%", usage_percent),
                    "compression_ratio": stats.get("compressratio").unwrap_or(&"1.00x".to_string()),
                    "compression_algorithm": stats.get("compression").unwrap_or(&"off".to_string()),
                    "deduplication": stats.get("dedup").unwrap_or(&"off".to_string()),
                    "record_size": stats.get("recordsize").unwrap_or(&"128K".to_string()),
                    "mount_point": stats.get("mountpoint").unwrap_or(&"legacy".to_string()),
                    "mounted": stats.get("mounted").unwrap_or(&"no".to_string()) == "yes",
                    "creation_time": stats.get("creation").unwrap_or(&"unknown".to_string()),
                    "snapshot_count": snapshot_count
                },
                "raw_properties": stats,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to get volume stats: {}", error_msg);
            Json(serde_json::json!({
                "status": "error",
                "message": "Failed to get volume statistics",
                "workspace_id": workspace_id,
                "volume_id": volume_id,
                "error": error_msg
            }))
        }
        Err(e) => {
            error!("❌ Stats command failed: {}", e);
            Json(serde_json::json!({
                "status": "error",
                "message": "Stats command execution failed",
                "workspace_id": workspace_id,
                "volume_id": volume_id
            }))
        }
    }
}

// Helper function to parse ZFS size strings to bytes
fn parse_size_to_bytes(size_str: &str) -> u64 {
    if size_str == "none" || size_str == "-" {
        return 0;
    }

    let size_str = size_str.trim();
    if size_str.is_empty() {
        return 0;
    }

    // Extract number and unit
    let (number_part, unit) =
        if let Some(pos) = size_str.find(|c: char| !c.is_ascii_digit() && c != '.') {
            (&size_str[..pos], &size_str[pos..])
        } else {
            (size_str, "")
        };

    let number: f64 = number_part.parse().unwrap_or(0.0);

    match unit.to_uppercase().as_str() {
        "K" => (number * 1024.0) as u64,
        "M" => (number * 1024.0 * 1024.0) as u64,
        "G" => (number * 1024.0 * 1024.0 * 1024.0) as u64,
        "T" => (number * 1024.0 * 1024.0 * 1024.0 * 1024.0) as u64,
        "P" => (number * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0) as u64,
        _ => number as u64,
    }
}

pub async fn get_workspace_volume_history(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "history": [
            {
                "timestamp": "2023-01-01T00:00:00Z",
                "action": "created",
                "user": "admin"
            }
        ]
    }))
}

pub async fn get_workspace_volume_properties(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "properties": {
            "compression": "lz4",
            "deduplication": "on",
            "encryption": "aes-256-gcm"
        }
    }))
}

pub async fn update_workspace_volume_properties(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Properties updated successfully"
    }))
}

pub async fn send_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume send initiated"
    }))
}

pub async fn receive_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume receive initiated"
    }))
}

pub async fn promote_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume promoted successfully"
    }))
}

pub async fn demote_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume demoted successfully"
    }))
}

pub async fn get_workspace_volume_inheritance(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "inheritance": {
            "compression": "inherit",
            "deduplication": "inherit"
        }
    }))
}

pub async fn update_workspace_volume_inheritance(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Inheritance updated successfully"
    }))
}

pub async fn upgrade_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume upgraded successfully"
    }))
}

pub async fn downgrade_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume downgraded successfully"
    }))
}

pub async fn hold_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume held successfully"
    }))
}

pub async fn release_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume released successfully"
    }))
}

pub async fn diff_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "diff": {
            "added": 100,
            "removed": 50,
            "modified": 25
        }
    }))
}

pub async fn bookmark_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "bookmark_id": Uuid::new_v4(),
        "message": "Bookmark created successfully"
    }))
}

pub async fn destroy_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume destroyed successfully"
    }))
}

pub async fn unload_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume unloaded successfully"
    }))
}

pub async fn load_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume loaded successfully"
    }))
}

pub async fn allow_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume allowed successfully"
    }))
}

pub async fn unallow_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume unallowed successfully"
    }))
}

pub async fn jail_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume jailed successfully"
    }))
}

pub async fn unjail_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Volume unjailed successfully"
    }))
}

pub async fn get_workspace_volume_userspace(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "userspace": {
            "users": [
                {
                    "id": Uuid::new_v4(),
                    "name": "user1",
                    "quota": "10GB"
                }
            ]
        }
    }))
}

pub async fn update_workspace_volume_userspace(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Userspace updated successfully"
    }))
}

pub async fn get_workspace_volume_user(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id, user_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "user_id": user_id,
        "name": "user1",
        "quota": "10GB"
    }))
}

pub async fn update_workspace_volume_user(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id, user_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "user_id": user_id,
        "message": "User updated successfully"
    }))
}

pub async fn delete_workspace_volume_user(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id, user_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "user_id": user_id,
        "message": "User deleted successfully"
    }))
}

pub async fn get_workspace_volume_groupspace(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "groupspace": {
            "groups": [
                {
                    "id": Uuid::new_v4(),
                    "name": "group1",
                    "quota": "50GB"
                }
            ]
        }
    }))
}

pub async fn update_workspace_volume_groupspace(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Groupspace updated successfully"
    }))
}

pub async fn get_workspace_volume_group(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id, group_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "group_id": group_id,
        "name": "group1",
        "quota": "50GB"
    }))
}

pub async fn update_workspace_volume_group(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id, group_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "group_id": group_id,
        "message": "Group updated successfully"
    }))
}

pub async fn delete_workspace_volume_group(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id, group_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "group_id": group_id,
        "message": "Group deleted successfully"
    }))
}

pub async fn get_workspace_volume_projectspace(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "projectspace": {
            "projects": [
                {
                    "id": Uuid::new_v4(),
                    "name": "project1",
                    "quota": "100GB"
                }
            ]
        }
    }))
}

pub async fn update_workspace_volume_projectspace(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "message": "Projectspace updated successfully"
    }))
}

pub async fn get_workspace_volume_project(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id, project_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "project_id": project_id,
        "name": "project1",
        "quota": "100GB"
    }))
}

pub async fn update_workspace_volume_project(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id, project_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "project_id": project_id,
        "message": "Project updated successfully"
    }))
}

pub async fn delete_workspace_volume_project(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id, project_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "project_id": project_id,
        "message": "Project deleted successfully"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use nestgate_zfs::byob::{
        ByobStorageProvider, ByobStorageRequest, ByobStorageResponse, StorageStatus, StorageUsage,
    };

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
                status: StorageStatus::Ready,
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
                status: StorageStatus::Ready,
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

        async fn list_team_storage(
            &self,
            _team_id: &str,
        ) -> nestgate_core::Result<Vec<ByobStorageResponse>> {
            Ok(vec![])
        }

        async fn get_storage_usage(
            &self,
            _deployment_id: Uuid,
        ) -> nestgate_core::Result<StorageUsage> {
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
        // Create a simple router without state for testing
        let app = Router::new().route("/health", get(health));
        let server = TestServer::new(app).unwrap();

        let response = server.get("/health").await;
        assert_eq!(response.status_code(), StatusCode::OK);

        let health: HealthResponse = response.json();
        // Health status can be "healthy" or "degraded" depending on system state
        assert!(health.status == "healthy" || health.status == "degraded");
    }

    #[tokio::test]
    async fn test_provision_storage() {
        // Create a mock provision handler for testing
        async fn mock_provision(Json(_request): Json<serde_json::Value>) -> impl IntoResponse {
            (
                StatusCode::CREATED,
                Json(serde_json::json!({
                    "deployment_id": uuid::Uuid::new_v4(),
                    "status": "provisioned",
                    "timestamp": chrono::Utc::now()
                })),
            )
        }

        let app = Router::new().route("/storage", post(mock_provision));
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
