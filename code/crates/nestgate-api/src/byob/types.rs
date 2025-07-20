//! BYOB API Types and Data Structures
//!
//! This module contains all the data structures, request/response types,
//! and constants used by the BYOB API endpoints.

use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

// Constants for common string values to avoid repeated allocations

/// Default storage quota allocation for workspaces (100 gigabytes)
pub const DEFAULT_STORAGE_QUOTA: &str = "100G";
/// Default compute quota allocation for workspaces (10 CPU cores)
pub const DEFAULT_COMPUTE_QUOTA: &str = "10 cores";
/// Default workspace quota allocation (10 gigabytes)
pub const DEFAULT_WORKSPACE_QUOTA: &str = "10G";
/// Default compression algorithm (LZ4 for fast compression/decompression)
pub const DEFAULT_COMPRESSION: &str = "lz4";
/// Status string indicating an active state
pub const ACTIVE_STATUS: &str = "active";
/// Default network name for BYOB containers
pub const BYOB_NETWORK: &str = "byob-network";
/// Status string indicating a healthy state
pub const HEALTHY_STATUS: &str = "healthy";
/// Status string indicating an unavailable state
pub const UNAVAILABLE_STATUS: &str = "unavailable";

/// HTTP API error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Error code or type identifier
    pub error: String,
    /// Human-readable error message
    pub message: String,
    /// Timestamp when the error occurred
    pub timestamp: String,
}

impl ErrorResponse {
    /// Create a new error response with the current timestamp
    pub fn new(error: &str, message: &str) -> Self {
        Self {
            error: error.to_string(),
            message: message.to_string(),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
}

/// Storage provision request
#[derive(Debug, Serialize, Deserialize)]
pub struct ProvisionRequest {
    pub deployment_id: Uuid,
    pub team_id: String,
    pub deployment_name: String,
    pub storage_requirements: HashMap<String, ServiceStorageRequirements>,
    pub team_quotas: TeamStorageQuotas,
}

/// Storage list query parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct ListQuery {
    pub team_id: Option<String>,
    pub status: Option<String>,
    pub limit: Option<u32>,
}

/// Storage usage query parameters
#[derive(Debug, Deserialize)]
pub struct UsageQuery {
    pub include_snapshots: Option<bool>,
    pub include_children: Option<bool>,
}

/// Workspace creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub team_id: String,
    pub storage_quota: Option<String>,
    pub compression: Option<String>,
    pub description: Option<String>,
}

/// Workspace configuration update request
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWorkspaceConfigRequest {
    pub quota: Option<String>,
    pub compression: Option<String>,
    pub deduplication: Option<bool>,
    pub encryption: Option<bool>,
}

/// Team creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
    pub description: Option<String>,
    pub storage_quota: Option<String>,
    pub compute_quota: Option<String>,
}

/// Project creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub team_id: String,
    pub description: Option<String>,
    pub storage_quota: Option<String>,
}

/// Create snapshot request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSnapshotRequest {
    pub name: String,
    pub description: Option<String>,
    pub retention_days: Option<u32>,
}

/// In-memory storage for workspace state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub id: Uuid,
    pub name: String,
    pub team_id: String,
    pub status: String,
    pub dataset_name: String,
    pub storage_quota: String,
    pub compression: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// In-memory storage for team state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamState {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub storage_quota: String,
    pub compute_quota: String,
    pub created_at: DateTime<Utc>,
}

/// In-memory storage for project state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectState {
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub description: Option<String>,
    pub storage_quota: String,
    pub created_at: DateTime<Utc>,
}

/// API state for managing in-memory data
#[derive(Debug, Clone)]
pub struct ApiState {
    pub database: Arc<RwLock<HashMap<String, serde_json::Value>>>,
    pub config: Arc<RwLock<serde_json::Value>>,
}

impl Default for ApiState {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiState {
    pub fn new() -> Self {
        Self {
            database: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(serde_json::json!({}))),
        }
    }
}

// Stub types for BYOB storage (these would normally come from nestgate-zfs)

use async_trait::async_trait;

#[async_trait]
pub trait ByobStorageProvider: Send + Sync {
    /// Process a storage provision request
    async fn provision_storage(
        &self,
        request: &ProvisionRequest,
    ) -> Result<ByobStorageResponse, String>;

    /// List storage resources
    async fn list_storage(&self, query: &ListQuery) -> Result<Vec<ByobStorageResponse>, String>;

    /// Get storage status
    async fn get_storage_status(&self, deployment_id: &Uuid)
        -> Result<ByobStorageResponse, String>;

    /// Remove storage
    async fn remove_storage(&self, deployment_id: &Uuid) -> Result<(), String>;

    /// Get storage usage
    async fn get_storage_usage(&self, deployment_id: &Uuid) -> Result<serde_json::Value, String>;

    /// Create workspace
    async fn create_workspace(
        &self,
        request: &CreateWorkspaceRequest,
    ) -> Result<WorkspaceState, String>;

    /// Get workspace
    async fn get_workspace(&self, workspace_id: &Uuid) -> Result<WorkspaceState, String>;

    /// List workspaces
    async fn list_workspaces(&self) -> Result<Vec<WorkspaceState>, String>;

    /// Create team
    async fn create_team(&self, request: &CreateTeamRequest) -> Result<TeamState, String>;

    /// Get team
    async fn get_team(&self, team_id: &str) -> Result<TeamState, String>;

    /// List teams
    async fn list_teams(&self) -> Result<Vec<TeamState>, String>;

    /// Create snapshot
    async fn create_snapshot(
        &self,
        deployment_id: &Uuid,
        request: &CreateSnapshotRequest,
    ) -> Result<String, String>;

    /// Get health status
    async fn get_health(&self) -> Result<serde_json::Value, String>;

    /// Get storage overview
    async fn get_storage_overview(&self) -> Result<serde_json::Value, String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByobStorageRequest {
    pub deployment_id: Uuid,
    pub team_id: String,
    pub deployment_name: String,
    pub storage_requirements: HashMap<String, ServiceStorageRequirements>,
    pub team_quotas: TeamStorageQuotas,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByobStorageResponse {
    pub deployment_id: Uuid,
    pub status: String,
    pub message: String,
    pub dataset_name: Option<String>,
    pub mount_point: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStorageRequirements {
    pub storage_gb: u64,
    pub iops: u64,
    pub bandwidth_mbps: u64,
    pub tier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamStorageQuotas {
    pub total_storage_gb: u64,
    pub max_iops: u64,
    pub max_bandwidth_mbps: u64,
}
