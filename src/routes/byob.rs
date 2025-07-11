use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post, delete, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::byob::ApiState;
use crate::routes::AppState;
use crate::models::*;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/storage", get(get_storage_overview))
        .route("/storage/health", get(get_storage_health))
        .route("/workspaces", get(get_workspaces))
        .route("/workspaces", post(create_workspace))
        .route("/workspaces/:id", get(get_workspace))
        .route("/deployments", post(create_deployment))
        .route("/deployments/:id", get(get_deployment))
        .route("/deployments/:id", delete(delete_deployment))
        .route("/teams", get(get_teams))
        .route("/teams", post(create_team))
        .route("/teams/:id", get(get_team))
        .route("/teams/:id", delete(delete_team))
        .route("/teams/:id/quota", get(get_team_quota))
        .route("/teams/:id/quota", put(update_team_quota))
        .route("/teams/:id/projects", get(get_team_projects))
        .route("/projects", post(create_project))
        .route("/projects/:id", get(get_project))
        .route("/projects/:id", delete(delete_project))
        .route("/projects/:id/datasets", get(get_project_datasets))
        .route("/datasets", post(create_dataset))
        .route("/datasets/:id", get(get_dataset))
        .route("/datasets/:id", delete(delete_dataset))
        .route("/snapshots", get(get_snapshots))
        .route("/snapshots/:id", get(get_snapshot))
}

// Storage endpoints
pub async fn get_storage_overview(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "total_capacity": "1TB",
        "used_capacity": "500GB",
        "available_capacity": "500GB",
        "pool_health": "healthy",
        "pools": [
            {
                "name": "main",
                "health": "healthy",
                "capacity": "1TB",
                "used": "500GB"
            }
        ]
    }))
}

pub async fn get_storage_health(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "status": "healthy",
        "degraded_count": 0,
        "errors": [],
        "warnings": []
    }))
}

// Deployment endpoints
pub async fn create_deployment(
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "id": Uuid::new_v4(),
        "name": request.get("name").unwrap_or(&serde_json::Value::String("default".to_string())),
        "status": "creating",
        "created_at": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn get_deployment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "id": id,
        "name": "deployment_1",
        "status": "running",
        "created_at": "2023-01-01T00:00:00Z"
    }))
}

pub async fn delete_deployment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "id": id,
        "message": "Deployment deleted successfully"
    }))
}

// Team endpoints
pub async fn get_teams(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "teams": [
            {
                "id": Uuid::new_v4(),
                "name": "Team 1",
                "created_at": "2023-01-01T00:00:00Z"
            }
        ]
    }))
}

pub async fn create_team(
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "id": Uuid::new_v4(),
        "name": request.get("name").unwrap_or(&serde_json::Value::String("default".to_string())),
        "created_at": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn get_team(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "id": id,
        "name": "Team 1",
        "created_at": "2023-01-01T00:00:00Z"
    }))
}

pub async fn delete_team(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "id": id,
        "message": "Team deleted successfully"
    }))
}

pub async fn get_team_quota(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "team_id": id,
        "quota": "1TB",
        "used": "500GB"
    }))
}

pub async fn update_team_quota(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "team_id": id,
        "quota": request.get("quota").unwrap_or(&serde_json::Value::String("1TB".to_string())),
        "updated_at": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn get_team_projects(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "team_id": id,
        "projects": [
            {
                "id": Uuid::new_v4(),
                "name": "Project 1",
                "created_at": "2023-01-01T00:00:00Z"
            }
        ]
    }))
}

// Project endpoints
pub async fn create_project(
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "id": Uuid::new_v4(),
        "name": request.get("name").unwrap_or(&serde_json::Value::String("default".to_string())),
        "created_at": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "id": id,
        "name": "Project 1",
        "created_at": "2023-1-01T00:00:00Z"
    }))
}

pub async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "id": id,
        "message": "Project deleted successfully"
    }))
}

pub async fn get_project_datasets(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "project_id": id,
        "datasets": [
            {
                "id": Uuid::new_v4(),
                "name": "Dataset 1",
                "created_at": "2023-01-01T00:00:00Z"
            }
        ]
    }))
}

// Dataset endpoints
pub async fn create_dataset(
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "id": Uuid::new_v4(),
        "name": request.get("name").unwrap_or(&serde_json::Value::String("default".to_string())),
        "created_at": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn get_dataset(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "id": id,
        "name": "Dataset 1",
        "created_at": "2023-01-01T00:00:00Z"
    }))
}

pub async fn delete_dataset(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "id": id,
        "message": "Dataset deleted successfully"
    }))
}

// Snapshot endpoints
pub async fn get_snapshots(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "snapshots": [
            {
                "id": Uuid::new_v4(),
                "name": "Snapshot 1",
                "created_at": "2023-01-01T00:00:00Z"
            }
        ]
    }))
}

pub async fn get_snapshot(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "id": id,
        "name": "Snapshot 1",
        "created_at": "2023-01-01T00:00:00Z"
    }))
}

// Workspace endpoints
pub async fn get_workspaces(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "workspaces": [
            {
                "id": Uuid::new_v4(),
                "name": "Workspace 1",
                "created_at": "2023-01-01T00:00:00Z"
            }
        ]
    }))
}

pub async fn create_workspace(
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let api_state = &state.api_state;
    Json(serde_json::json!({
        "id": Uuid::new_v4(),
        "name": request.get("name").unwrap_or(&serde_json::Value::String("default".to_string())),
        "created_at": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn get_workspace(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let api_1 = &state.api_state;
    Json(serde_json::json!({
        "id": id,
        "name": "Workspace 1",
        "created_at": "2023-01-01T00:00:00Z"
    }))
}