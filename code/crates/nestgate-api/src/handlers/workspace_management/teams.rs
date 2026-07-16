// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Team creation, listing, and workspace team association functionality.

use axum::{http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};

/// **TEAM CREATION REQUEST**
///
/// Request structure for creating a new team.
#[derive(Debug, Deserialize)]
/// Request parameters for `CreateTeam` operation
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
        .map_or_else(
            |_| {
                tracing::warn!("System time before UNIX_EPOCH, using 0");
                0
            },
            |d| d.as_secs(),
        );

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
#[expect(
    clippy::unnecessary_wraps,
    reason = "Stub API; Result preserved for future error handling"
)]
pub fn get_teams() -> Result<Json<Vec<TeamInfo>>, StatusCode> {
    let teams = vec![
        TeamInfo {
            id: "team_001".into(),
            name: "Development Team".into(),
            description: Some("Core development team".into()),
            members: vec!["alice".into(), "bob".into()],
            created_at: std::time::SystemTime::now(),
        },
        TeamInfo {
            id: "team_002".into(),
            name: "Operations Team".into(),
            description: Some("Infrastructure and operations".into()),
            members: vec!["charlie".into(), "diana".into()],
            created_at: std::time::SystemTime::now(),
        },
    ];

    Ok(Json(teams))
}
