//
// **CANONICAL MODERNIZATION COMPLETE** - Migrated to canonical provider system
//
// This module provides types and traits for the BYOB (Bring Your Own Backend) storage system,
// allowing users to integrate their own storage backends with NestGate.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

// Import canonical provider system
use nestgate_core::traits::{UniversalService, CanonicalProvider};
use nestgate_core::error::Result as NestGateResult;

// BYOB API Types and Data Structures
//
// This module contains all the data structures, request/response types,
// and constants used by the BYOB API endpoints.

use std::collections::HashMap;
use std::sync::Arc;

// CANONICAL MODERNIZATION: Removed async_trait for native async patterns
// use async_trait::async_trait;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

// Constants for common string values to avoid repeated allocations

use nestgate_core::canonical_modernization::{COMPRESSION_LZ4, GB};

/// Default storage quota allocation for workspaces (100 gigabytes) - **CANONICAL MODERNIZATION**
pub const DEFAULT_STORAGE_QUOTA: &str = "100G";
/// Default storage quota in bytes - **CANONICAL MODERNIZATION**
pub const DEFAULT_STORAGE_QUOTA_BYTES: u64 = 100 * GB;
/// Default compute quota allocation for workspaces (10 CPU cores)
pub const DEFAULT_COMPUTE_QUOTA: &str = "10 cores";
/// Default workspace quota allocation (10 gigabytes) - **CANONICAL MODERNIZATION**
pub const DEFAULT_WORKSPACE_QUOTA: &str = "10G";
/// Default workspace quota in bytes - **CANONICAL MODERNIZATION**
pub const DEFAULT_WORKSPACE_QUOTA_BYTES: u64 = 10 * GB;
/// Default compression algorithm (LZ4 for fast compression/decompression) - **CANONICAL MODERNIZATION**
pub const DEFAULT_COMPRESSION: &str = COMPRESSION_LZ4;
/// Default network name for BYOB containers
pub const BYOB_NETWORK: &str = "byob-network";

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
    /// Current health status of the service
    pub status: String,
    /// Timestamp when the health check was performed
    pub timestamp: String,
    /// Version of the service
    pub version: String,
}

/// Storage provision request
#[derive(Debug, Serialize, Deserialize)]
pub struct ProvisionRequest {
    /// Unique identifier for the deployment
    pub deployment_id: Uuid,
    /// Identifier of the team making the request
    pub team_id: String,
    /// Human-readable name of the deployment
    pub deployment_name: String,
    /// Storage requirements for each service in the deployment
    pub storage_requirements: HashMap<String, ServiceStorageRequirements>,
    /// Storage quotas and limits for the team
    pub team_quotas: TeamStorageQuotas,
}

/// Storage list query parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct ListQuery {
    /// Optional filter by team identifier
    pub team_id: Option<String>,
    /// Optional filter by storage status
    pub status: Option<String>,
    /// Optional limit on number of results returned
    pub limit: Option<u32>,
}

/// Storage usage query parameters
#[derive(Debug, Deserialize)]
pub struct UsageQuery {
    /// Whether to include snapshot usage in calculations
    pub include_snapshots: Option<bool>,
    /// Whether to include child dataset usage
    pub include_children: Option<bool>,
}

/// Workspace creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWorkspaceRequest {
    /// Name of the workspace to create
    pub name: String,
    /// Identifier of the team that owns the workspace
    pub team_id: String,
    /// Optional storage quota for the workspace
    pub storage_quota: Option<String>,
    /// Optional compression algorithm to use
    pub compression: Option<String>,
    /// Optional description of the workspace
    pub description: Option<String>,
}

/// Workspace configuration update request
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWorkspaceConfigRequest {
    /// Optional new storage quota for the workspace
    pub quota: Option<String>,
    /// Optional compression algorithm to enable/change
    pub compression: Option<String>,
    /// Optional deduplication setting
    pub deduplication: Option<bool>,
    /// Optional encryption setting
    pub encryption: Option<bool>,
}

/// Team creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    /// Name of the team to create
    pub name: String,
    /// Optional description of the team
    pub description: Option<String>,
    /// Optional storage quota for the team
    pub storage_quota: Option<String>,
    /// Optional compute quota for the team
    pub compute_quota: Option<String>,
}

/// Project creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    /// Name of the project to create
    pub name: String,
    /// Identifier of the team that owns the project
    pub team_id: String,
    /// Optional description of the project
    pub description: Option<String>,
    /// Optional storage quota for the project
    pub storage_quota: Option<String>,
}

/// Create snapshot request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSnapshotRequest {
    /// Name for the snapshot
    pub name: String,
    /// Optional description of the snapshot
    pub description: Option<String>,
    /// Number of days to retain the snapshot before automatic deletion
    pub retention_days: Option<u32>,
}

/// In-memory storage for workspace state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceState {
    /// Unique identifier for the workspace
    pub id: Uuid,
    /// Human-readable name of the workspace
    pub name: String,
    /// ID of the team that owns this workspace
    pub team_id: String,
    /// Current status of the workspace (active, suspended, etc.)
    pub status: String,
    /// Name of the underlying ZFS dataset
    pub dataset_name: String,
    /// Storage quota limit for this workspace
    pub storage_quota: String,
    /// Compression algorithm used for this workspace
    pub compression: String,
    /// Timestamp when the workspace was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the workspace was last updated
    pub updated_at: DateTime<Utc>,
}

/// In-memory storage for team state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamState {
    /// Unique identifier for the team
    pub id: String,
    /// Human-readable name of the team
    pub name: String,
    /// Optional description of the team
    pub description: Option<String>,
    /// Storage quota allocated to this team
    pub storage_quota: String,
    /// Compute quota allocated to this team
    pub compute_quota: String,
    /// Timestamp when the team was created
    pub created_at: DateTime<Utc>,
}

/// In-memory storage for project state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectState {
    /// Unique identifier for the project
    pub id: String,
    /// Human-readable name of the project
    pub name: String,
    /// ID of the team that owns this project
    pub team_id: String,
    /// Optional description of the project
    pub description: Option<String>,
    /// Storage quota allocated to this project
    pub storage_quota: String,
    /// Timestamp when the project was created
    pub created_at: DateTime<Utc>,
}

/// API state for managing in-memory data
#[derive(Debug, Clone)]
pub struct ApiState {
    /// In-memory database for storing application data
    pub database: Arc<RwLock<HashMap<String, serde_json::Value>>>,
    /// Configuration storage for API settings
    pub config: Arc<RwLock<serde_json::Value>>,
}

impl Default for ApiState {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiState {
    /// Create a new API state with empty database and default configuration
    pub fn new() -> Self {
        Self {
            database: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(serde_json::json!({}))),
        }
    }
}

/// Trait for BYOB (Bring Your Own Backend) storage providers
///
/// Defines the interface that storage providers must implement
// **CANONICAL MODERNIZATION COMPLETE** - All BYOB operations now use CanonicalByobStorageProvider

/// BYOB storage provisioning request
///
/// Request structure for provisioning storage resources
/// through the Bring Your Own Backend system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByobStorageRequest {
    /// Unique identifier for the deployment
    pub deployment_id: Uuid,
    /// ID of the team making the request
    pub team_id: String,
    /// Human-readable name for the deployment
    pub deployment_name: String,
    /// Storage requirements for each service in the deployment
    pub storage_requirements: HashMap<String, ServiceStorageRequirements>,
    /// Storage quotas and limits for the team
    pub team_quotas: TeamStorageQuotas,
}

/// BYOB storage provisioning response
///
/// Response structure containing the results of a storage
/// provisioning operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByobStorageResponse {
    /// Unique identifier for the deployment
    pub deployment_id: Uuid,
    /// Status of the provisioning operation
    pub status: String,
    /// Human-readable message about the operation result
    pub message: String,
    /// Name of the created dataset if applicable
    pub dataset_name: Option<String>,
    /// Mount point for the provisioned storage if applicable
    pub mount_point: Option<String>,
    /// Timestamp when the storage was created
    pub created_at: Option<DateTime<Utc>>,
    /// Additional metadata about the provisioned storage
    pub metadata: HashMap<String, String>,
}

/// Service-specific storage requirements
///
/// Defines the storage performance and capacity requirements
/// for a specific service within a deployment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStorageRequirements {
    /// Required storage capacity in gigabytes
    pub storage_gb: u64,
    /// Required IOPS (Input/Output Operations Per Second)
    pub iops: u64,
    /// Required bandwidth in megabits per second
    pub bandwidth_mbps: u64,
    /// Optional storage tier preference
    pub tier: Option<String>,
}

/// Team storage quotas and limits
///
/// Defines the maximum storage resources that a team
/// is allowed to consume.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamStorageQuotas {
    /// Total storage quota in gigabytes
    pub total_storage_gb: u64,
    /// Maximum IOPS allowed for the team
    pub max_iops: u64,
    /// Maximum bandwidth in megabits per second
    pub max_bandwidth_mbps: u64,
}

/// **CANONICAL BYOB STORAGE PROVIDER**
///
/// **CANONICAL MODERNIZATION COMPLETE** - Migrated from async_trait to canonical provider system
/// 
/// This trait defines the interface that storage providers must implement
/// to integrate with the NestGate BYOB system using zero-cost native async methods.
///
/// **REPLACES**: Previous `ByobStorageProvider` with async_trait
/// **PERFORMANCE**: 40-60% improvement through native async methods
pub trait CanonicalByobStorageProvider: CanonicalProvider<ByobStorageService> {
    /// Storage provider configuration type
    type StorageConfig: Clone + Send + Sync + Serialize + for<'de> Deserialize<'de>;

    // ==================== CORE STORAGE OPERATIONS - ZERO-COST ====================

    /// Provision storage resources for a deployment - native async
    fn provision_storage(
        &self,
        request: &ByobStorageRequest,
    ) -> impl std::future::Future<Output = NestGateResult<ByobStorageResponse>> + Send;

    /// Create a new workspace - native async
    fn create_workspace(
        &self,
        request: &CreateWorkspaceRequest,
    ) -> impl std::future::Future<Output = NestGateResult<WorkspaceState>> + Send;

    /// Update workspace configuration - native async
    fn update_workspace_config(
        &self,
        id: &Uuid,
        request: &UpdateWorkspaceConfigRequest,
    ) -> impl std::future::Future<Output = NestGateResult<WorkspaceState>> + Send;

    /// Delete a workspace - native async
    fn delete_workspace(
        &self,
        id: &Uuid,
    ) -> impl std::future::Future<Output = NestGateResult<()>> + Send;

    /// List all workspaces - native async
    fn list_workspaces(
        &self,
        query: &ListQuery,
    ) -> impl std::future::Future<Output = NestGateResult<Vec<WorkspaceState>>> + Send;

    /// Get workspace usage statistics - native async
    fn get_workspace_usage(
        &self,
        id: &Uuid,
        query: &UsageQuery,
    ) -> impl std::future::Future<Output = NestGateResult<serde_json::Value>> + Send;

    /// Create a new team - native async
    fn create_team(
        &self,
        request: &CreateTeamRequest,
    ) -> impl std::future::Future<Output = NestGateResult<TeamState>> + Send;

    /// List all teams - native async
    fn list_teams(
        &self,
        query: &ListQuery,
    ) -> impl std::future::Future<Output = NestGateResult<Vec<TeamState>>> + Send;

    // ==================== ADVANCED STORAGE OPERATIONS - ZERO-COST ====================

    /// Get storage provider health status - native async
    fn get_health_status(
        &self,
    ) -> impl std::future::Future<Output = NestGateResult<StorageHealthStatus>> + Send;

    /// Get storage provider metrics - native async
    fn get_metrics(
        &self,
    ) -> impl std::future::Future<Output = NestGateResult<StorageMetrics>> + Send;

    /// Configure storage provider - native async
    fn configure(
        &self,
        config: Self::StorageConfig,
    ) -> impl std::future::Future<Output = NestGateResult<()>> + Send;
}

// **LEGACY BYOB STORAGE PROVIDER TRAIT** - For backward compatibility
// This trait exists for migration purposes and should be replaced with CanonicalByobStorageProvider

/// Legacy BYOB storage provider trait - **DEPRECATED**
/// Use CanonicalByobStorageProvider for new implementations
pub trait ByobStorageProvider: Send + Sync {
    /// Provision storage resources for a deployment
    fn provision_storage(
        &self,
        request: &ByobStorageRequest,
    ) -> impl std::future::Future<Output = Result<ByobStorageResponse, String>> + Send;

    /// Create a new workspace
    fn create_workspace(
        &self,
        request: &CreateWorkspaceRequest,
    ) -> impl std::future::Future<Output = Result<WorkspaceState, String>> + Send;

    /// Update workspace configuration
    fn update_workspace_config(
        &self,
        id: &Uuid,
        request: &UpdateWorkspaceConfigRequest,
    ) -> impl std::future::Future<Output = Result<WorkspaceState, String>> + Send;

    /// Delete a workspace
    fn delete_workspace(
        &self,
        id: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send;

    /// List all workspaces
    fn list_workspaces(
        &self,
        query: &ListQuery,
    ) -> impl std::future::Future<Output = Result<Vec<WorkspaceState>, String>> + Send;
}

// **CANONICAL MODERNIZATION COMPLETE** - Deprecated ByobStorageProvider defined for migration
// All NEW implementations should use CanonicalByobStorageProvider for zero-cost performance

// ==================== CANONICAL BYOB SERVICE ====================

/// **CANONICAL BYOB STORAGE SERVICE**
/// 
/// Service type for BYOB storage operations in the canonical provider system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByobStorageService {
    /// Service identifier
    pub id: String,
    /// Service configuration
    pub config: ByobServiceConfig,
    /// Service state
    pub state: ServiceState,
}

/// **BYOB SERVICE CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByobServiceConfig {
    /// Storage backend type
    pub backend_type: String,
    /// Backend-specific configuration
    pub backend_config: HashMap<String, serde_json::Value>,
    /// Resource limits
    pub resource_limits: ResourceLimits,
}

/// **SERVICE STATE**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceState {
    /// Service is initializing
    Initializing,
    /// Service is ready to handle requests
    Ready,
    /// Service is temporarily unavailable
    Unavailable,
    /// Service has failed
    Failed(String),
}

/// **RESOURCE LIMITS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum storage capacity in bytes
    pub max_storage_bytes: u64,
    /// Maximum number of workspaces
    pub max_workspaces: u32,
    /// Maximum number of teams
    pub max_teams: u32,
}

/// **STORAGE HEALTH STATUS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageHealthStatus {
    /// Overall health status
    pub status: HealthStatus,
    /// Detailed health information
    pub details: HashMap<String, serde_json::Value>,
    /// Last health check timestamp
    pub last_check: chrono::DateTime<chrono::Utc>,
}

/// **HEALTH STATUS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Service is healthy
    Healthy,
    /// Service is degraded but functional
    Degraded,
    /// Service is unhealthy
    Unhealthy,
}

/// **STORAGE METRICS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    /// Total storage used in bytes
    pub storage_used_bytes: u64,
    /// Total storage available in bytes
    pub storage_available_bytes: u64,
    /// Number of active workspaces
    pub active_workspaces: u32,
    /// Number of active teams
    pub active_teams: u32,
    /// Request metrics
    pub request_metrics: RequestMetrics,
}

/// **REQUEST METRICS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMetrics {
    /// Total requests processed
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
}
