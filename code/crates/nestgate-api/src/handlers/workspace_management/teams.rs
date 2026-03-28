// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Team creation, listing, and workspace team association functionality.

use axum::{http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};

/// **TEAM CREATION REQUEST**
///
/// Request structure for creating a new team.
#[derive(Debug, Deserialize)]
/// Request parameters for CreateTeam operation
pub struct CreateTeamRequest {
    /// Team name
    pub name: String,
    /// Team description
    pub description: Option<String>,
    /// Initial team members
    pub members: Vec<String>,
}

/// **TEAM INFO**
///
/// Information about a team.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Teaminfo
pub struct TeamInfo {
    /// Team identifier
    pub id: String,
    /// Team name
    pub name: String,
    /// Team description
    pub description: Option<String>,
    /// Team members
    pub members: Vec<String>,
    /// Team creation timestamp
    pub created_at: std::time::SystemTime,
}

/// **CREATE TEAM HANDLER**
///
/// Create a new team with the specified configuration.
pub async fn create_team(
    Json(request): Json<CreateTeamRequest>,
) -> Result<Json<TeamInfo>, StatusCode> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_else(|_| {
            tracing::warn!("System time before UNIX_EPOCH, using 0");
            0
        });

    let team = TeamInfo {
        id: format!("team_{timestamp}"),
        name: request.name,
        description: request.description,
        members: request.members,
        created_at: std::time::SystemTime::now(),
    };

    Ok(Json(team))
}

/// **GET TEAMS HANDLER**
///
/// Retrieve all teams.
pub fn get_teams() -> Result<Json<Vec<TeamInfo>>, StatusCode> {
    let teams = vec![
        TeamInfo {
            id: "team_001".to_string(),
            name: "Development Team".to_string(),
            description: Some("Core development team".to_string()),
            members: vec!["alice".to_string(), "bob".to_string()],
            created_at: std::time::SystemTime::now(),
        },
        TeamInfo {
            id: "team_002".to_string(),
            name: "Operations Team".to_string(),
            description: Some("Infrastructure and operations".to_string()),
            members: vec!["charlie".to_string(), "diana".to_string()],
            created_at: std::time::SystemTime::now(),
        },
    ];

    Ok(Json(teams))
}
