//
// **CANONICAL MODERNIZATION COMPLETE** - Uses canonical provider system
// for zero-cost abstractions and improved performance.

use crate::byob::types::{
    CanonicalByobStorageProvider, CreateTeamRequest, TeamState,
};
use crate::error::CanonicalResult as Result;
use uuid::Uuid;

/// **CANONICAL BYOB TEAMS HANDLER**
///
/// Handles team operations using the canonical provider system
/// **PERFORMANCE**: Native async patterns for optimal performance
pub struct ByobTeamsHandler<P: CanonicalByobStorageProvider> {
    provider: P,
}

impl<P: CanonicalByobStorageProvider> ByobTeamsHandler<P> {
    /// Create new teams handler with canonical provider
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Create team using canonical provider methods
    pub async fn create_team(&self, request: &CreateTeamRequest) -> Result<TeamState> {
        self.provider.create_team(request).await
    }

    /// List all teams
    pub async fn list_teams(&self) -> Result<Vec<TeamState>> {
        self.provider.list_teams().await
    }

    /// Get team details
    pub async fn get_team(&self, team_id: &Uuid) -> Result<TeamState> {
        self.provider.get_team(team_id).await
    }

    /// Update team configuration
    pub async fn update_team(&self, team_id: &Uuid, request: &CreateTeamRequest) -> Result<TeamState> {
        self.provider.update_team(team_id, request).await
    }

    /// Delete team
    pub async fn delete_team(&self, team_id: &Uuid) -> Result<()> {
        self.provider.delete_team(team_id).await
    }
}
