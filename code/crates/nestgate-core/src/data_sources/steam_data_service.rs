// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **STEAM DATA SERVICE**
//!
//! Steam gaming data integration for `NestGate` data service.
//! Handles game library storage, save data federation, and asset caching.

use crate::error::NestGateError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Steam App ID type
pub type SteamAppId = u32;

/// Steam game metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Gamemetadata
pub struct GameMetadata {
    /// Steam App ID
    pub app_id: SteamAppId,
    /// Game name
    pub name: String,
    /// Game developer
    pub developer: String,
    /// Game publisher
    pub publisher: String,
    /// Release date
    pub release_date: chrono::DateTime<chrono::Utc>,
    /// Game genres
    pub genres: Vec<String>,
    /// Game tags
    pub tags: Vec<String>,
    /// Installation size in bytes
    pub size_bytes: u64,
    /// Last played timestamp
    pub last_played: Option<chrono::DateTime<chrono::Utc>>,
}

/// Play statistics for a game
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Playstats
pub struct PlayStats {
    /// Total playtime in minutes
    pub total_playtime_minutes: u32,
    /// Playtime in last 2 weeks in minutes
    pub recent_playtime_minutes: u32,
    /// Last session start time
    pub last_session_start: Option<chrono::DateTime<chrono::Utc>>,
    /// Session count
    pub session_count: u32,
    /// Average session length in minutes
    pub avg_session_length_minutes: f32,
}

/// Achievement data for a game
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Achievementdata
pub struct AchievementData {
    /// Total achievements available
    pub total_achievements: u32,
    /// Unlocked achievements count
    pub unlocked_achievements: u32,
    /// Achievement completion percentage
    pub completion_percentage: f32,
    /// Recent achievements (last 30 days)
    pub recent_achievements: Vec<Achievement>,
}

/// Individual achievement
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Achievement
pub struct Achievement {
    /// Achievement ID
    pub id: String,
    /// Achievement name
    pub name: String,
    /// Achievement description
    pub description: String,
    /// Unlock timestamp
    pub unlocked_at: chrono::DateTime<chrono::Utc>,
    /// Rarity percentage (0.0 to 100.0)
    pub rarity_percent: f32,
}

/// Game library storage component
#[derive(Debug)]
/// Gamelibrarystorage
pub struct GameLibraryStorage {
    /// Game metadata indexed by App ID
    game_metadata: Arc<RwLock<HashMap<SteamAppId, GameMetadata>>>,
    /// Installation paths indexed by App ID
    installation_paths: Arc<RwLock<HashMap<SteamAppId, PathBuf>>>,
    /// Play statistics indexed by App ID
    play_statistics: Arc<RwLock<HashMap<SteamAppId, PlayStats>>>,
    /// Achievement data indexed by App ID
    achievement_data: Arc<RwLock<HashMap<SteamAppId, AchievementData>>>,
}

impl Default for GameLibraryStorage {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl GameLibraryStorage {
    /// Create a new game library storage
    #[must_use]
    pub fn new() -> Self {
        Self {
            game_metadata: Arc::new(RwLock::new(HashMap::new())),
            installation_paths: Arc::new(RwLock::new(HashMap::new())),
            play_statistics: Arc::new(RwLock::new(HashMap::new())),
            achievement_data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add or update game metadata
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn update_game_metadata(&self, metadata: GameMetadata) -> Result<(), NestGateError> {
        let mut games = self.game_metadata.write().await;
        info!(
            "Updating metadata for game: {} ({})",
            metadata.name, metadata.app_id
        );
        games.insert(metadata.app_id, metadata);
        Ok(())
    }

    /// Get game metadata by App ID
    pub async fn get_game_metadata(&self, app_id: SteamAppId) -> Option<GameMetadata> {
        let games = self.game_metadata.read().await;
        games.get(&app_id).cloned()
    }

    /// Get all games in library
    pub async fn get_all_games(&self) -> Vec<GameMetadata> {
        let games = self.game_metadata.read().await;
        games.values().cloned().collect()
    }

    /// Update installation path for a game
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn update_installation_path(
        &self,
        app_id: SteamAppId,
        path: PathBuf,
    ) -> Result<(), NestGateError> {
        let mut paths = self.installation_paths.write().await;
        debug!("Updating installation path for {}: {:?}", app_id, path);
        paths.insert(app_id, path);
        Ok(())
    }

    /// Get installation path for a game
    pub async fn get_installation_path(&self, app_id: SteamAppId) -> Option<PathBuf> {
        let paths = self.installation_paths.read().await;
        paths.get(&app_id).cloned()
    }

    /// Update play statistics for a game
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn update_play_stats(
        &self,
        app_id: SteamAppId,
        stats: PlayStats,
    ) -> Result<(), NestGateError> {
        let mut play_stats = self.play_statistics.write().await;
        debug!(
            "Updating play stats for {}: {} minutes total",
            app_id, stats.total_playtime_minutes
        );
        play_stats.insert(app_id, stats);
        Ok(())
    }

    /// Get play statistics for a game
    pub async fn get_play_stats(&self, app_id: SteamAppId) -> Option<PlayStats> {
        let play_stats = self.play_statistics.read().await;
        play_stats.get(&app_id).cloned()
    }

    /// Update achievement data for a game
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn update_achievement_data(
        &self,
        app_id: SteamAppId,
        achievements: AchievementData,
    ) -> Result<(), NestGateError> {
        let mut achievement_map = self.achievement_data.write().await;
        debug!(
            "Updating achievements for {}: {}/{} unlocked",
            app_id, achievements.unlocked_achievements, achievements.total_achievements
        );
        achievement_map.insert(app_id, achievements);
        Ok(())
    }

    /// Get achievement data for a game
    pub async fn get_achievement_data(&self, app_id: SteamAppId) -> Option<AchievementData> {
        let achievement_map = self.achievement_data.read().await;
        achievement_map.get(&app_id).cloned()
    }

    /// Get library statistics
    pub async fn get_library_stats(&self) -> LibraryStats {
        let games = self.game_metadata.read().await;
        let play_stats = self.play_statistics.read().await;

        let total_games = games.len();
        let total_playtime: u32 = play_stats
            .values()
            .map(|stats| stats.total_playtime_minutes)
            .sum();

        let total_size: u64 = games.values().map(|game| game.size_bytes).sum();

        LibraryStats {
            total_games,
            total_playtime_minutes: total_playtime,
            total_size_bytes: total_size,
            avg_playtime_per_game: if total_games > 0 {
                total_playtime as f32 / total_games as f32
            } else {
                0.0
            },
        }
    }
}

/// Library statistics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Librarystats
pub struct LibraryStats {
    /// Total number of games in library
    pub total_games: usize,
    /// Total playtime across all games in minutes
    pub total_playtime_minutes: u32,
    /// Total installation size in bytes
    pub total_size_bytes: u64,
    /// Average playtime per game in minutes
    pub avg_playtime_per_game: f32,
}

/// Save data federation node
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Federationnode
pub struct FederationNode {
    /// Node identifier
    pub id: String,
    /// Node endpoint URL
    pub endpoint: String,
    /// Node priority (higher = preferred)
    pub priority: u32,
    /// Last sync timestamp
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
    /// Node health status
    pub healthy: bool,
}

/// Save data conflict resolution strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Conflictresolution
pub enum ConflictResolution {
    /// Use the most recent save
    MostRecent,
    /// Use the save with most playtime
    MostPlaytime,
    /// Manual resolution required
    Manual,
    /// Keep both saves with timestamps
    KeepBoth,
}

/// Save data federation component
#[derive(Debug)]
/// Savedatafederation
pub struct SaveDataFederation {
    /// Federation nodes
    sync_targets: Arc<RwLock<Vec<FederationNode>>>,
    /// Conflict resolution strategy
    #[allow(dead_code)]
    conflict_resolution: ConflictResolution,
    /// Encryption configuration (placeholder)
    _encryption_config: (),
}

impl SaveDataFederation {
    /// Create a new save data federation
    #[must_use]
    pub fn new(conflict_resolution: ConflictResolution) -> Self {
        Self {
            sync_targets: Arc::new(RwLock::new(Vec::new())),
            conflict_resolution,
            _encryption_config: (),
        }
    }

    /// Add a federation node
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn add_federation_node(&self, node: FederationNode) -> Result<(), NestGateError> {
        let mut nodes = self.sync_targets.write().await;
        info!("Adding federation node: {} at {}", node.id, node.endpoint);
        nodes.push(node);
        Ok(())
    }

    /// Sync save data for a specific game
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn sync_save_data(
        &self,
        app_id: SteamAppId,
        _save_data: &[u8],
    ) -> Result<(), NestGateError> {
        let nodes = self.sync_targets.read().await;

        info!(
            "Syncing save data for game {} to {} nodes",
            app_id,
            nodes.len()
        );

        // In a real implementation, this would:
        // 1. Encrypt the save data
        // 2. Send to all healthy federation nodes
        // 3. Handle conflicts using the resolution strategy
        // 4. Return success/failure status

        for node in nodes.iter().filter(|n| n.healthy) {
            debug!("Syncing to node: {} ({})", node.id, node.endpoint);
            // Placeholder for actual sync implementation
            // This would involve HTTP requests to federation nodes
        }

        Ok(())
    }

    /// Get federation status
    pub async fn get_federation_status(&self) -> FederationStatus {
        let nodes = self.sync_targets.read().await;
        let healthy_nodes = nodes.iter().filter(|n| n.healthy).count();
        let total_nodes = nodes.len();

        FederationStatus {
            total_nodes,
            healthy_nodes,
            sync_enabled: healthy_nodes > 0,
            last_global_sync: None, // Would track actual last sync
        }
    }
}

/// Federation status summary
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Federationstatus
pub struct FederationStatus {
    /// Total number of federation nodes
    pub total_nodes: usize,
    /// Number of healthy nodes
    pub healthy_nodes: usize,
    /// Whether sync is currently enabled
    pub sync_enabled: bool,
    /// Last successful global sync
    pub last_global_sync: Option<chrono::DateTime<chrono::Utc>>,
}

/// Main Steam data service
#[derive(Debug)]
/// Service implementation for SteamData
pub struct SteamDataService {
    /// Game library storage
    pub game_library_storage: GameLibraryStorage,
    /// Save data federation
    pub save_data_federation: SaveDataFederation,
    /// Asset cache management (placeholder)
    pub asset_cache_management: (),
    /// Steam API data sync (placeholder)
    pub steam_api_data_sync: (),
}

impl SteamDataService {
    /// Create a new Steam data service
    #[must_use]
    pub fn new() -> Self {
        Self {
            game_library_storage: GameLibraryStorage::new(),
            save_data_federation: SaveDataFederation::new(ConflictResolution::MostRecent),
            asset_cache_management: (),
            steam_api_data_sync: (),
        }
    }

    /// Initialize the service with federation nodes
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn initialize(
        &self,
        federation_nodes: Vec<FederationNode>,
    ) -> Result<(), NestGateError> {
        info!(
            "Initializing Steam data service with {} federation nodes",
            federation_nodes.len()
        );

        for node in federation_nodes {
            self.save_data_federation.add_federation_node(node).await?;
        }

        Ok(())
    }

    /// Get service health status
    pub async fn health_check(&self) -> ServiceHealth {
        let federation_status = self.save_data_federation.get_federation_status().await;
        let library_stats = self.game_library_storage.get_library_stats().await;

        ServiceHealth {
            service_name: "steam_data_service".to_string(),
            healthy: federation_status.sync_enabled,
            games_tracked: library_stats.total_games,
            federation_nodes: federation_status.total_nodes,
            last_check: chrono::Utc::now(),
        }
    }
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicehealth
pub struct ServiceHealth {
    /// Service identifier
    pub service_name: String,
    /// Overall health status
    pub healthy: bool,
    /// Number of games being tracked
    pub games_tracked: usize,
    /// Number of federation nodes
    pub federation_nodes: usize,
    /// Last health check timestamp
    pub last_check: chrono::DateTime<chrono::Utc>,
}

impl Default for SteamDataService {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// Data source provider trait implementation for Steam - native async for zero-cost abstractions
pub trait SteamDataProvider: Send + Sync {
    /// Fetch game data from Steam API - native async, no boxing
    fn fetch_game_data(
        &self,
        app_id: SteamAppId,
    ) -> impl Future<Output = Result<GameMetadata, NestGateError>> + Send;

    /// Fetch user's game library - native async, no boxing
    fn fetch_user_library(
        &self,
        steam_id: u64,
    ) -> impl Future<Output = Result<Vec<GameMetadata>, NestGateError>> + Send;

    /// Fetch achievement data for a game - native async, no boxing
    fn fetch_achievements(
        &self,
        app_id: SteamAppId,
        steam_id: u64,
    ) -> impl Future<Output = Result<AchievementData, NestGateError>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_game_library_storage() {
        let storage = GameLibraryStorage::new();

        let game = GameMetadata {
            app_id: 12345,
            name: "Test Game".to_string(),
            developer: "Test Dev".to_string(),
            publisher: "Test Pub".to_string(),
            release_date: chrono::Utc::now(),
            genres: vec!["Action".to_string()],
            tags: vec!["Singleplayer".to_string()],
            size_bytes: 1_000_000_000,
            last_played: None,
        };

        storage
            .update_game_metadata(game.clone())
            .await
            .expect("Operation failed");

        let retrieved = storage.get_game_metadata(12345).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.expect("Operation failed").name, "Test Game");
    }

    #[tokio::test]
    async fn test_save_data_federation() {
        let federation = SaveDataFederation::new(ConflictResolution::MostRecent);

        let node = FederationNode {
            id: "node1".to_string(),
            endpoint: "http://gaming-rig-2:8080".to_string(),
            priority: 100,
            last_sync: None,
            healthy: true,
        };

        federation
            .add_federation_node(node)
            .await
            .expect("Operation failed");

        let status = federation.get_federation_status().await;
        assert_eq!(status.total_nodes, 1);
        assert_eq!(status.healthy_nodes, 1);
        assert!(status.sync_enabled);
    }

    #[tokio::test]
    async fn test_steam_data_service() {
        let service = SteamDataService::new();

        let nodes = vec![FederationNode {
            id: "primary".to_string(),
            endpoint: "http://gaming-rig-1:8080".to_string(),
            priority: 100,
            last_sync: None,
            healthy: true,
        }];

        service.initialize(nodes).await.expect("Operation failed");

        let health = service.health_check().await;
        assert_eq!(health.service_name, "steam_data_service");
        assert_eq!(health.federation_nodes, 1);
    }
}
