    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamRequest {
    pub name: String,
    pub workspace_id: String,
}

/// Get all workspaces
pub async fn get_workspaces(State(_state): State<crate::routes::AppState>) -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "workspaces": [
            {
                "id": "workspace_1",
                "name": "Default Workspace",
                "description": "Default workspace for development",
                "created_at": "2024-01-01T00:00:00Z"
            }
        ]
    }))
}

/// Create a new workspace
pub async fn create_workspace(
    State(_state): State<crate::routes::AppState>,
    Json(request): Json<WorkspaceRequest>,
) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(WorkspaceResponse {
            id: "new_workspace_id".to_string(),
            name: request.name,
            description: request.description,
            created_at: chrono::Utc::now().to_rfc3339(),
        }),
    )
}

/// Get a specific workspace
pub async fn get_workspace(
    State(_state): State<crate::routes::AppState>,
    axum::extract::Path(workspace_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    Json(WorkspaceResponse {
        id: workspace_id,
        name: "Sample Workspace".to_string(),
        description: Some("A sample workspace".to_string()),
        created_at: "2024-01-01T00:00:00Z".to_string(),
    })
}

/// Update workspace configuration
pub async fn update_workspace_config(
    State(_state): State<crate::routes::AppState>,
    axum::extract::Path(workspace_id): axum::extract::Path<String>,
    Json(request): Json<WorkspaceRequest>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "message": format!("Workspace {} updated successfully", workspace_id),
        "workspace": {
            "id": workspace_id,
            "name": request.name,
            "description": request.description
        }
    }))
}

/// Delete a workspace
pub async fn delete_workspace(
    State(_state): State<crate::routes::AppState>,
    axum::extract::Path(workspace_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "message": format!("Workspace {} deleted successfully", workspace_id)
        })),
    )
}

/// Create a new team
pub async fn create_team(
    State(_state): State<crate::routes::AppState>,
    Json(request): Json<TeamRequest>,
) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "status": "success",
            "team": {
                "id": "new_team_id",
                "name": request.name,
                "workspace_id": request.workspace_id,
                "created_at": chrono::Utc::now().to_rfc3339()
            }
        })),
    )
}
