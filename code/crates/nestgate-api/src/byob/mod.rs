//! BYOB (Bring Your Own Build) API Module
//!
//! This module provides REST API endpoints for BYOB storage operations.
//! It handles storage requests from orchestration coordination layer.

use async_trait::async_trait;
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

pub mod handlers;
pub mod types;

use handlers::*;

/// Health check endpoint
pub async fn health() -> axum::response::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "healthy",
        "service": "byob-api",
        "timestamp": chrono::Utc::now()
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

#[async_trait]
impl ByobStorageProvider for ZfsStorageProvider {
    async fn provision_storage(
        &self,
        request: &ProvisionRequest,
    ) -> Result<ByobStorageResponse, String> {
        info!(
            "🏗️ Provisioning storage for deployment: {}",
            request.deployment_id
        );

        // Calculate total storage requirements
        let total_storage_gb: u64 = request
            .storage_requirements
            .values()
            .map(|req| req.storage_gb)
            .sum();

        // Determine storage tier (default to warm if not specified)
        let tier = request
            .storage_requirements
            .values()
            .find_map(|req| req.tier.as_ref())
            .cloned()
            .unwrap_or_else(|| "warm".to_string());

        // Create ZFS dataset for the deployment
        let dataset_name = format!("nestpool/deployments/{}", request.deployment_id);
        let mount_point = format!("/mnt/deployments/{}", request.deployment_id);

        // Execute ZFS create command
        let result = self
            .create_zfs_dataset(&dataset_name, &mount_point, total_storage_gb, &tier)
            .await;

        match result {
            Ok(()) => {
                // Store deployment state
                let deployment_state = DeploymentState {
                    deployment_id: request.deployment_id,
                    team_id: request.team_id.clone(),
                    deployment_name: request.deployment_name.clone(),
                    dataset_name: dataset_name.clone(),
                    mount_point: mount_point.clone(),
                    status: "provisioned".to_string(),
                    created_at: Utc::now(),
                    metadata: HashMap::new(),
                };

                {
                    let mut state = self.state.write().await;
                    state
                        .deployments
                        .insert(request.deployment_id, deployment_state);
                }

                info!(
                    "✅ Successfully provisioned storage for deployment: {}",
                    request.deployment_id
                );

                Ok(ByobStorageResponse {
                    deployment_id: request.deployment_id,
                    status: "provisioned".to_string(),
                    message: "Storage provisioned successfully".to_string(),
                    dataset_name: Some(dataset_name),
                    mount_point: Some(mount_point),
                    created_at: Some(Utc::now()),
                    metadata: HashMap::new(),
                })
            }
            Err(e) => {
                error!("❌ Failed to provision storage: {}", e);
                Err(format!("Storage provisioning failed: {e}"))
            }
        }
    }

    async fn list_storage(&self, query: &ListQuery) -> Result<Vec<ByobStorageResponse>, String> {
        let state = self.state.read().await;

        let mut results = Vec::new();
        for deployment in state.deployments.values() {
            // Apply filters
            if let Some(ref team_id) = query.team_id {
                if deployment.team_id != *team_id {
                    continue;
                }
            }

            if let Some(ref status) = query.status {
                if deployment.status != *status {
                    continue;
                }
            }

            results.push(ByobStorageResponse {
                deployment_id: deployment.deployment_id,
                status: deployment.status.clone(),
                message: "Storage active".to_string(),
                dataset_name: Some(deployment.dataset_name.clone()),
                mount_point: Some(deployment.mount_point.clone()),
                created_at: Some(deployment.created_at),
                metadata: deployment.metadata.clone(),
            });
        }

        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit as usize);
        }

        Ok(results)
    }

    async fn get_storage_status(
        &self,
        deployment_id: &Uuid,
    ) -> Result<ByobStorageResponse, String> {
        let state = self.state.read().await;

        if let Some(deployment) = state.deployments.get(deployment_id) {
            Ok(ByobStorageResponse {
                deployment_id: *deployment_id,
                status: deployment.status.clone(),
                message: "Storage active".to_string(),
                dataset_name: Some(deployment.dataset_name.clone()),
                mount_point: Some(deployment.mount_point.clone()),
                created_at: Some(deployment.created_at),
                metadata: deployment.metadata.clone(),
            })
        } else {
            Err("Deployment not found".to_string())
        }
    }

    async fn remove_storage(&self, deployment_id: &Uuid) -> Result<(), String> {
        info!("🗑️ Removing storage for deployment: {}", deployment_id);

        let deployment = {
            let state = self.state.read().await;
            state.deployments.get(deployment_id).cloned()
        };

        if let Some(deployment) = deployment {
            // Remove ZFS dataset
            let result = self.destroy_zfs_dataset(&deployment.dataset_name).await;

            match result {
                Ok(()) => {
                    // Remove from state
                    let mut state = self.state.write().await;
                    state.deployments.remove(deployment_id);

                    info!(
                        "✅ Successfully removed storage for deployment: {}",
                        deployment_id
                    );
                    Ok(())
                }
                Err(e) => {
                    error!("❌ Failed to remove storage: {}", e);
                    Err(format!("Storage removal failed: {e}"))
                }
            }
        } else {
            Err("Deployment not found".to_string())
        }
    }

    async fn get_storage_usage(&self, deployment_id: &Uuid) -> Result<serde_json::Value, String> {
        let state = self.state.read().await;

        if let Some(deployment) = state.deployments.get(deployment_id) {
            // Get real ZFS usage data
            let usage_data = self.get_dataset_usage(&deployment.dataset_name).await?;

            Ok(json!({
                "deployment_id": deployment_id,
                "dataset_name": deployment.dataset_name,
                "usage": usage_data,
                "timestamp": Utc::now()
            }))
        } else {
            Err("Deployment not found".to_string())
        }
    }

    async fn create_workspace(
        &self,
        request: &CreateWorkspaceRequest,
    ) -> Result<WorkspaceState, String> {
        info!("🏗️ Creating workspace: {}", request.name);

        let workspace_id = Uuid::new_v4();
        let dataset_name = format!("nestpool/workspaces/{workspace_id}");
        let mount_point = format!("/mnt/workspaces/{workspace_id}");

        let storage_quota = request
            .storage_quota
            .as_ref()
            .cloned()
            .unwrap_or_else(|| DEFAULT_WORKSPACE_QUOTA.to_string());
        let compression = request
            .compression
            .as_ref()
            .cloned()
            .unwrap_or_else(|| DEFAULT_COMPRESSION.to_string());

        // Create ZFS dataset
        let result = self
            .create_workspace_dataset(&dataset_name, &mount_point, &storage_quota, &compression)
            .await;

        match result {
            Ok(()) => {
                let workspace_state = WorkspaceState {
                    id: workspace_id,
                    name: request.name.clone(),
                    team_id: request.team_id.clone(),
                    status: ACTIVE_STATUS.to_string(),
                    dataset_name: dataset_name.clone(),
                    storage_quota: storage_quota.clone(),
                    compression: compression.clone(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };

                {
                    let mut state = self.state.write().await;
                    state
                        .workspaces
                        .insert(workspace_id, workspace_state.clone());
                }

                info!(
                    "✅ Successfully created workspace: {} ({})",
                    request.name, workspace_id
                );
                Ok(workspace_state)
            }
            Err(e) => {
                error!("❌ Failed to create workspace: {}", e);
                Err(format!("Workspace creation failed: {e}"))
            }
        }
    }

    async fn get_workspace(&self, workspace_id: &Uuid) -> Result<WorkspaceState, String> {
        let state = self.state.read().await;

        state
            .workspaces
            .get(workspace_id)
            .cloned()
            .ok_or_else(|| "Workspace not found".to_string())
    }

    async fn list_workspaces(&self) -> Result<Vec<WorkspaceState>, String> {
        let state = self.state.read().await;
        Ok(state.workspaces.values().cloned().collect())
    }

    async fn create_team(&self, request: &CreateTeamRequest) -> Result<TeamState, String> {
        info!("👥 Creating team: {}", request.name);

        let team_id = Uuid::new_v4().to_string();
        let storage_quota = request
            .storage_quota
            .as_ref()
            .cloned()
            .unwrap_or_else(|| DEFAULT_STORAGE_QUOTA.to_string());
        let compute_quota = request
            .compute_quota
            .as_ref()
            .cloned()
            .unwrap_or_else(|| DEFAULT_COMPUTE_QUOTA.to_string());

        let team_state = TeamState {
            id: team_id.clone(),
            name: request.name.clone(),
            description: request.description.clone(),
            storage_quota: storage_quota.clone(),
            compute_quota: compute_quota.clone(),
            created_at: Utc::now(),
        };

        {
            let mut state = self.state.write().await;
            state.teams.insert(team_id.clone(), team_state.clone());
        }

        info!(
            "✅ Successfully created team: {} ({})",
            request.name, team_id
        );
        Ok(team_state)
    }

    async fn get_team(&self, team_id: &str) -> Result<TeamState, String> {
        let state = self.state.read().await;

        state
            .teams
            .get(team_id)
            .cloned()
            .ok_or_else(|| "Team not found".to_string())
    }

    async fn list_teams(&self) -> Result<Vec<TeamState>, String> {
        let state = self.state.read().await;
        Ok(state.teams.values().cloned().collect())
    }

    async fn create_snapshot(
        &self,
        deployment_id: &Uuid,
        request: &CreateSnapshotRequest,
    ) -> Result<String, String> {
        info!(
            "📸 Creating snapshot: {} for deployment: {}",
            request.name, deployment_id
        );

        let state = self.state.read().await;
        if let Some(deployment) = state.deployments.get(deployment_id) {
            let snapshot_name = format!("{}@{}", deployment.dataset_name, request.name);

            // Create ZFS snapshot
            let result = self.create_zfs_snapshot(&snapshot_name).await;

            match result {
                Ok(()) => {
                    info!("✅ Successfully created snapshot: {}", snapshot_name);
                    Ok(snapshot_name)
                }
                Err(e) => {
                    error!("❌ Failed to create snapshot: {}", e);
                    Err(format!("Snapshot creation failed: {e}"))
                }
            }
        } else {
            Err("Deployment not found".to_string())
        }
    }

    async fn get_health(&self) -> Result<serde_json::Value, String> {
        Ok(json!({
            "status": "healthy",
            "service": "byob-storage",
            "backend": "zfs",
            "timestamp": Utc::now()
        }))
    }

    async fn get_storage_overview(&self) -> Result<serde_json::Value, String> {
        // Get real ZFS pool information
        let pool_info = self.get_pool_info().await?;

        Ok(json!({
            "total_storage": pool_info.get("total_size").unwrap_or(&json!("Unknown")),
            "used_storage": pool_info.get("used_size").unwrap_or(&json!("Unknown")),
            "available_storage": pool_info.get("available_size").unwrap_or(&json!("Unknown")),
            "pool_health": pool_info.get("health").unwrap_or(&json!("Unknown")),
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

/// Create the BYOB router with ZFS storage provider
pub fn create_byob_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/storage", post(provision_storage))
        .route("/storage", get(list_storage))
        .route("/storage/:deployment_id", get(get_storage_status))
        .route("/storage/:deployment_id", post(remove_storage))
        .route("/storage/:deployment_id/usage", get(get_storage_usage))
        .route("/storage/:deployment_id/snapshots", post(create_snapshot))
        .route("/storage/:deployment_id/snapshots/:snapshot_name", post(restore_snapshot))
        .route("/storage/overview", get(get_storage_overview))
        .route("/storage/health", get(get_storage_health))

        // Team management routes
        .route("/teams", get(get_teams))
        .route("/teams", post(create_team))
        .route("/teams/:team_id", get(get_team))
        .route("/teams/:team_id", delete(delete_team))
        .route("/teams/:team_id/quota", get(get_team_quota))
        .route("/teams/:team_id/quota", put(update_team_quota))
        .route("/teams/:team_id/projects", get(get_team_projects))

        // Project management routes
        .route("/projects", post(create_project))
        .route("/projects/:project_id", get(get_project))
        .route("/projects/:project_id", delete(delete_project))
        .route("/projects/:project_id/datasets", get(get_project_datasets))

        // Dataset management routes
        .route("/datasets", post(create_dataset))
        .route("/datasets/:dataset_id", get(get_dataset))
        .route("/datasets/:dataset_id", delete(delete_dataset))

        // Snapshot management routes
        .route("/snapshots", get(get_snapshots))
        .route("/snapshots/:snapshot_id", get(get_snapshot))

        // Workspace management routes
        .route("/workspaces", get(get_workspaces))
        .route("/workspaces", post(create_workspace))
        .route("/workspaces/:workspace_id", get(get_workspace))

        // Workspace volume operations
        .route("/workspaces/:workspace_id/volumes/:volume_id/properties", put(set_workspace_volume_properties))
        .route("/workspaces/:workspace_id/volumes/:volume_id/inherit", post(inherit_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/userspace", post(userspace_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/groupspace", post(groupspace_workspace_volume))
        .route("/workspaces/:workspace_id/volumes/:volume_id/projectspace", post(projectspace_workspace_volume))
}

/// Create BYOB service with ZFS storage provider
pub fn create_byob_service() -> Router<AppState> {
    create_byob_router()
}

// Re-export types for convenience
pub use types::*;
