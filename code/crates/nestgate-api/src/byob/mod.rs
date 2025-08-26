//
// This module provides REST API endpoints for BYOB storage operations.
// It handles storage requests from orchestration coordination layer.
// **CANONICAL MODERNIZATION**: Migrated from async_trait to native async patterns

// CANONICAL MODERNIZATION: Removed async_trait for native async patterns
// use async_trait::async_trait;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};
use uuid::Uuid;

use crate::routes::AppState;
use nestgate_zfs::types::SnapshotInfo;
pub mod handlers;
pub mod types;

use types::{
    ByobStorageProvider, ByobStorageRequest, ByobStorageResponse, CreateTeamRequest,
    CreateWorkspaceRequest, ListQuery, TeamState, UpdateWorkspaceConfigRequest, UsageQuery,
    WorkspaceState,
};

/// Health check endpoint
pub async fn health() -> axum::response::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "healthy",
        "service": "byob-api",
        "timestamp": Utc::now()
    }))
}

/// ZFS-backed implementation of ByobStorageProvider
pub struct ZfsStorageProvider {
    /// In-memory state for tracking workspaces, teams, and deployments
    state: Arc<RwLock<ByobState>>,
}

/// Internal state for BYOB operations
#[derive(Debug, Clone)]
struct ByobState {
    workspaces: HashMap<Uuid, WorkspaceState>,
    teams: HashMap<String, TeamState>,
    deployments: HashMap<Uuid, DeploymentState>,
}

/// Deployment state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeploymentState {
    deployment_id: Uuid,
    team_id: String,
    deployment_name: String,
    dataset_name: String,
    mount_point: String,
    status: String,
    created_at: DateTime<Utc>,
    metadata: HashMap<String, String>,
}

// CANONICAL MODERNIZATION: Native async implementation without async_trait overhead
impl ByobStorageProvider for ZfsStorageProvider {
    fn provision_storage(
        &self,
        request: &ByobStorageRequest,
    ) -> impl std::future::Future<Output = Result<ByobStorageResponse, String>> + Send {
        async move {
        info!("🚀 Provisioning storage for team: {}", request.team_id);

        let deployment_id = request.deployment_id;
        let dataset_name = format!("nestgate/{}/{}", request.team_id, deployment_id);
        let mount_point = format!("/mnt/nestgate/{}/{}", request.team_id, deployment_id);

        // Get storage requirements from the first requirement
        let (storage_gb, tier) = request
            .storage_requirements
            .values()
            .next()
            .map(|req| (req.storage_gb, req.tier.as_deref().unwrap_or("warm")))
            .unwrap_or((10, "warm"));

        // Create ZFS dataset
        self.create_zfs_dataset(&dataset_name, &mount_point, storage_gb, tier)
            .await?;

        let deployment = DeploymentState {
            deployment_id,
            team_id: request.team_id.clone(),
            deployment_name: request.deployment_name.clone(),
            dataset_name: dataset_name.clone(),
            mount_point: mount_point.clone(),
            status: "active".to_string(),
            created_at: Utc::now(),
            metadata: HashMap::new(),
        };

        // Store deployment state
        {
            let mut state = self.state.write().await;
            state.deployments.insert(deployment_id, deployment.clone());
        }

        Ok(ByobStorageResponse {
            deployment_id,
            dataset_name: Some(dataset_name),
            mount_point: Some(mount_point),
            status: "provisioned".to_string(),
            message: "Storage provisioned successfully".to_string(),
            created_at: Some(Utc::now()),
            metadata: HashMap::new(),
        })
        } // Close async move block
    }

    fn create_workspace(
        &self,
        request: &CreateWorkspaceRequest,
    ) -> impl std::future::Future<Output = Result<WorkspaceState, String>> + Send {
        async move {
        info!("🏗️ Creating workspace: {}", request.name);

        let workspace_id = Uuid::new_v4();
        let workspace = WorkspaceState {
            id: workspace_id,
            name: request.name.clone(),
            team_id: request.team_id.clone(),
            dataset_name: format!("nestpool/workspaces/{}", workspace_id),
            storage_quota: request
                .storage_quota
                .clone()
                .unwrap_or_else(|| "10GB".to_string()),
            compression: request
                .compression
                .clone()
                .unwrap_or_else(|| "lz4".to_string()),
            status: "active".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Store workspace state
        {
            let mut state = self.state.write().await;
            state.workspaces.insert(workspace_id, workspace.clone());
        }

        Ok(workspace)
        } // Close async move block
    }

    fn list_workspaces(&self, _query: &ListQuery) -> impl std::future::Future<Output = Result<Vec<WorkspaceState>, String>> + Send {
        async move {
        let state = self.state.read().await;
        Ok(state.workspaces.values().cloned().collect())
        } // Close async move block
    }

    fn update_workspace_config(
        &self,
        id: &Uuid,
        _request: &UpdateWorkspaceConfigRequest,
    ) -> impl std::future::Future<Output = Result<WorkspaceState, String>> + Send {
        async move {
        let mut state = self.state.write().await;
        if let Some(workspace) = state.workspaces.get_mut(id) {
            workspace.updated_at = Utc::now();
            Ok(workspace.clone())
        } else {
            Err("Workspace not found".to_string())
        }
        } // Close async move block
    }

    fn delete_workspace(&self, id: &Uuid) -> impl std::future::Future<Output = Result<(), String>> + Send {
        async move {
        let mut state = self.state.write().await;
        if state.workspaces.remove(id).is_some() {
            Ok(())
        } else {
            Err("Workspace not found".to_string())
        }
        } // Close async move block
    }

    async fn get_workspace_usage(
        &self,
        _id: &Uuid,
        _query: &UsageQuery,
    ) -> Result<serde_json::Value, String> {
        Ok(json!({
            "storage_used": "1.2GB",
            "storage_quota": "10GB",
            "compute_used": "2 cores",
            "compute_quota": "10 cores"
        }))
    }

    async fn create_team(&self, request: &CreateTeamRequest) -> Result<TeamState, String> {
        info!("👥 Creating team: {}", request.name);

        let team = TeamState {
            id: request.name.clone(),
            name: request.name.clone(),
            description: Some(
                request
                    .description
                    .clone()
                    .unwrap_or_else(|| format!("Team {}", request.name)),
            ),
            storage_quota: request
                .storage_quota
                .clone()
                .unwrap_or_else(|| "100GB".to_string()),
            compute_quota: request
                .compute_quota
                .clone()
                .unwrap_or_else(|| "10 cores".to_string()),
            created_at: Utc::now(),
        };

        // Store team state
        {
            let mut state = self.state.write().await;
            state.teams.insert(request.name.clone(), team.clone());
        }

        Ok(team)
    }

    async fn list_teams(&self) -> Result<Vec<TeamState>, String> {
        let state = self.state.read().await;
        Ok(state.teams.values().cloned().collect())
    }

    async fn create_snapshot(
        &self,
        deployment_id: &Uuid,
        request: &types::CreateSnapshotRequest,
    ) -> Result<String, String> {
        info!(
            "📸 Creating snapshot {} for {}",
            request.name, deployment_id
        );

        // Create the snapshot using ZFS commands (simplified)
        let snapshot_id = format!("{}@{}", deployment_id, request.name);

        // In a real implementation, this would call ZFS snapshot creation
        // For now, we return a success message
        Ok(snapshot_id)
    }

    async fn get_health(&self) -> Result<serde_json::Value, String> {
        Ok(json!({
            "status": "healthy",
            "service": "byob-storage",
            "backend": "zfs",
            "timestamp": Utc::now()
        }))
    }
}

impl ZfsStorageProvider {
    /// Create a new ZFS storage provider
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(ByobState {
                workspaces: HashMap::new(),
                teams: HashMap::new(),
                deployments: HashMap::new(),
            })),
        }
    }

    /// Create a ZFS dataset for storage
    async fn create_zfs_dataset(
        &self,
        dataset_name: &str,
        mount_point: &str,
        storage_gb: u64,
        tier: &str,
    ) -> Result<(), String> {
        let quota_bytes = storage_gb * 1024 * 1024 * 1024;

        // Set compression based on tier
        let compression = match tier {
            "hot" => "lz4",
            "warm" => "zstd",
            "cold" => "gzip",
            _ => "lz4",
        };

        let mut cmd = tokio::process::Command::new("zfs");
        cmd.args([
            "create",
            "-o",
            &format!("compression={compression}"),
            "-o",
            &format!("quota={quota_bytes}"),
            "-o",
            &format!("mountpoint={mount_point}"),
            "-o",
            &format!("nestgate:tier={tier}"),
            dataset_name,
        ]);

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Failed to execute zfs create: {e}"))?;

        if !output.status.success() {
            return Err(format!(
                "ZFS create failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    /// Create a ZFS dataset for workspace
    async fn create_workspace_dataset(
        &self,
        dataset_name: &str,
        mount_point: &str,
        quota: &str,
        compression: &str,
    ) -> Result<(), String> {
        let mut cmd = tokio::process::Command::new("zfs");
        cmd.args([
            "create",
            "-o",
            &format!("compression={compression}"),
            "-o",
            &format!("quota={quota}"),
            "-o",
            &format!("mountpoint={mount_point}"),
            "-o",
            "nestgate:type=workspace",
            dataset_name,
        ]);

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Failed to execute zfs create: {e}"))?;

        if !output.status.success() {
            return Err(format!(
                "ZFS create failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    /// Destroy a ZFS dataset
    async fn destroy_zfs_dataset(&self, dataset_name: &str) -> Result<(), String> {
        let mut cmd = tokio::process::Command::new("zfs");
        cmd.args(["destroy", "-r", dataset_name]);

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Failed to execute zfs destroy: {e}"))?;

        if !output.status.success() {
            return Err(format!(
                "ZFS destroy failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    /// Create a ZFS snapshot
    async fn create_zfs_snapshot(&self, snapshot_name: &str) -> Result<(), String> {
        let mut cmd = tokio::process::Command::new("zfs");
        cmd.args(["snapshot", snapshot_name]);

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Failed to execute zfs snapshot: {e}"))?;

        if !output.status.success() {
            return Err(format!(
                "ZFS snapshot failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    /// Get dataset usage information
    async fn get_dataset_usage(&self, dataset_name: &str) -> Result<serde_json::Value, String> {
        let mut cmd = tokio::process::Command::new("zfs");
        cmd.args([
            "get",
            "-H",
            "-o",
            "value",
            "used,avail,quota,compressratio",
            dataset_name,
        ]);

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Failed to execute zfs get: {e}"))?;

        if !output.status.success() {
            return Err(format!(
                "ZFS get failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();

        if lines.len() >= 4 {
            Ok(json!({
                "used": lines[0].trim(),
                "available": lines[1].trim(),
                "quota": lines[2].trim(),
                "compression_ratio": lines[3].trim(),
            }))
        } else {
            Err("Invalid ZFS output format".to_string())
        }
    }

    /// Get pool information
    async fn get_pool_info(&self) -> Result<serde_json::Value, String> {
        let mut cmd = tokio::process::Command::new("zpool");
        cmd.args(["list", "-H", "-o", "size,alloc,free,health", "nestpool"]);

        let output = cmd
            .output()
            .await
            .map_err(|e| format!("Failed to execute zpool list: {e}"))?;

        if !output.status.success() {
            return Err(format!(
                "ZPool list failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = stdout.trim().split('\t').collect();

        if parts.len() >= 4 {
            Ok(json!({
                "total_size": parts[0].trim(),
                "used_size": parts[1].trim(),
                "available_size": parts[2].trim(),
                "health": parts[3].trim(),
            }))
        } else {
            Err("Invalid ZPool output format".to_string())
        }
    }
}

impl Default for ZfsStorageProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// Create BYOB router with all endpoints
pub fn create_byob_router() -> Router<AppState> {
    Router::new().route("/health", get(health))
    // Note: Teams and snapshots handlers need to be implemented
    // .route("/teams", get(teams::get_teams))
    // .route("/teams", post(teams::create_team))
    // .route("/snapshots", get(snapshots::get_snapshots))
    // .route("/snapshots", post(snapshots::create_snapshot))
}

/// Create BYOB service with ZFS storage provider
pub fn create_byob_service() -> Router<AppState> {
    create_byob_router()
}

// Re-export types for convenience
pub use types::*;
