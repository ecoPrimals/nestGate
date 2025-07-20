//! Team Management Handler Functions
//!
//! This module contains all the HTTP handlers for team-related operations
//! including creating, reading, updating, and deleting teams.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use std::sync::OnceLock;
use tracing::{error, info};

use super::super::types::{ByobStorageProvider, CreateTeamRequest};
use super::super::ZfsStorageProvider;
use crate::routes::AppState;

// Global storage provider instance
static STORAGE_PROVIDER: OnceLock<ZfsStorageProvider> = OnceLock::new();

fn get_storage_provider() -> &'static ZfsStorageProvider {
    STORAGE_PROVIDER.get_or_init(ZfsStorageProvider::new)
}

/// Get all teams
pub async fn get_teams(State(_state): State<AppState>) -> impl IntoResponse {
    info!("📋 Getting all teams");

    let provider = get_storage_provider();
    match provider.list_teams().await {
        Ok(teams) => {
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
        Err(e) => {
            error!("❌ Failed to get teams: {}", e);
            Json(json!({
                "error": "Failed to get teams",
                "message": e,
                "teams": [],
                "count": 0,
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}

/// Create a new team
pub async fn create_team(
    State(_state): State<AppState>,
    Json(request): Json<CreateTeamRequest>,
) -> impl IntoResponse {
    info!("👥 Creating new team: {}", request.name);

    let provider = get_storage_provider();
    match provider.create_team(&request).await {
        Ok(team_state) => {
            info!(
                "✅ Successfully created team: {} ({})",
                request.name, team_state.id
            );

            (
                StatusCode::CREATED,
                Json(json!({
                    "status": "success",
                    "team_id": team_state.id,
                    "name": team_state.name,
                    "description": team_state.description,
                    "storage_quota": team_state.storage_quota,
                    "compute_quota": team_state.compute_quota,
                    "created_at": team_state.created_at,
                    "message": "Team created successfully"
                })),
            )
        }
        Err(e) => {
            error!("❌ Failed to create team: {}", e);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "error": "TEAM_CREATION_FAILED",
                    "message": format!("Failed to create team: {}", e),
                    "timestamp": chrono::Utc::now()
                })),
            )
        }
    }
}

/// Get a specific team
pub async fn get_team(
    State(_state): State<AppState>,
    Path(team_id): Path<String>,
) -> impl IntoResponse {
    info!("📖 Getting team details: {}", team_id);

    let provider = get_storage_provider();
    match provider.get_team(&team_id).await {
        Ok(team) => Json(json!({
            "status": "success",
            "team": {
                "id": team.id,
                "name": team.name,
                "description": team.description,
                "storage_quota": team.storage_quota,
                "compute_quota": team.compute_quota,
                "created_at": team.created_at
            },
            "timestamp": chrono::Utc::now()
        })),
        Err(e) => {
            error!("❌ Failed to get team: {}", e);
            Json(json!({
                "status": "error",
                "error": "TEAM_NOT_FOUND",
                "message": format!("Team not found: {}", e),
                "team_id": team_id,
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}

/// Delete a team
pub async fn delete_team(
    State(_state): State<AppState>,
    Path(team_id): Path<String>,
) -> impl IntoResponse {
    info!("🗑️ Deleting team: {}", team_id);

    // For now, this is a placeholder - in a real implementation,
    // we would need to check for dependencies and clean up resources
    Json(json!({
        "team_id": team_id,
        "status": "deleted",
        "message": "Team deletion requested",
        "timestamp": chrono::Utc::now()
    }))
}

/// Get team quota information
pub async fn get_team_quota(
    State(_state): State<AppState>,
    Path(team_id): Path<String>,
) -> impl IntoResponse {
    info!("📊 Getting quota for team: {}", team_id);

    let provider = get_storage_provider();
    match provider.get_team(&team_id).await {
        Ok(team) => Json(json!({
            "team_id": team_id,
            "quota": {
                "storage": team.storage_quota,
                "compute": team.compute_quota
            },
            "timestamp": chrono::Utc::now()
        })),
        Err(e) => {
            error!("❌ Failed to get team quota: {}", e);
            Json(json!({
                "error": "Failed to get team quota",
                "message": e,
                "team_id": team_id,
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}

/// Update team quota
pub async fn update_team_quota(
    State(_state): State<AppState>,
    Path(team_id): Path<String>,
    Json(_request): Json<serde_json::Value>,
) -> impl IntoResponse {
    info!("🔄 Updating quota for team: {}", team_id);

    // For now, this is a placeholder - in a real implementation,
    // we would update the team's quota in the storage provider
    Json(json!({
        "team_id": team_id,
        "status": "updated",
        "message": "Team quota update requested",
        "timestamp": chrono::Utc::now()
    }))
}

/// Get projects for a team
pub async fn get_team_projects(
    State(_state): State<AppState>,
    Path(team_id): Path<String>,
) -> impl IntoResponse {
    info!("📁 Getting projects for team: {}", team_id);

    // For now, this is a placeholder - in a real implementation,
    // we would query the storage provider for team projects
    Json(json!({
        "team_id": team_id,
        "projects": [],
        "count": 0,
        "timestamp": chrono::Utc::now()
    }))
}
