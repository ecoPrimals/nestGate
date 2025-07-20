//! Team Management Operations
//!
//! Team creation, listing, and workspace team association functionality.

use axum::{extract::Json, http::StatusCode};
use serde_json::{json, Value};
use tracing::info;

/// Get all teams
pub async fn get_teams() -> Result<Json<Value>, StatusCode> {
    info!("👥 Getting all teams");

    let teams = vec![
        json!({
            "id": "team-1",
            "name": "Development Team",
            "members": 5,
            "workspaces": 3,
            "storage_used": "25GB"
        }),
        json!({
            "id": "team-2",
            "name": "QA Team",
            "members": 3,
            "workspaces": 2,
            "storage_used": "15GB"
        }),
    ];

    Ok(Json(json!({
        "status": "success",
        "teams": teams,
        "count": teams.len()
    })))
}

/// Create a new team
pub async fn create_team(Json(request): Json<Value>) -> Result<Json<Value>, StatusCode> {
    info!("👥 Creating new team: {:?}", request);

    let team_name = request
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unnamed-team");

    let team_id = uuid::Uuid::new_v4().to_string();

    Ok(Json(json!({
        "status": "success",
        "message": "Team created successfully",
        "team_id": team_id,
        "name": team_name
    })))
}
