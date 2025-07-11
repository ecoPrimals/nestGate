//! Ecosystem service types for network integration

use super::{DatasetContext, ServiceHealth, StorageContext, TaskPriority};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use nestgate_core::types::StorageTier;

/// Service plan for dynamic task execution
#[derive(Debug, Clone)]
pub enum ServicePlan {
    /// Use Squirrel MCP for data management coordination
    SquirrelMcp {
        squirrel_id: String,
        compute_service_id: String,
    },
    /// Direct connection to external compute service for analysis
    DirectCompute { compute_service_id: String },
    /// Use multiple NestGate peers for distributed processing
    DistributedNestGate { peer_ids: Vec<String> },
    /// Fall back to local processing
    Fallback,
}

/// Ecosystem service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemService {
    pub instance_id: String,
    pub service_type: String,
    pub endpoint: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub health_status: ServiceHealth,
    pub metadata: HashMap<String, String>,
}

/// Service information response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub instance_id: String,
    pub service_type: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// Songbird instance information
#[cfg(feature = "network-integration")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SongbirdInstance {
    pub instance_id: String,
    pub endpoint: String,
    pub version: String,
    pub capabilities: Vec<String>,
    #[serde(with = "system_time_serde")]
    pub last_seen: SystemTime,
    pub is_ephemeral: bool, // True for LAN, false for internet
}

#[cfg(feature = "network-integration")]
mod system_time_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time.duration_since(UNIX_EPOCH).unwrap();
        duration.as_secs().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + std::time::Duration::from_secs(secs))
    }
}

/// Service registration for Songbird
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceRegistration {
    pub service_id: String,
    pub service_type: String,
    pub capabilities: Vec<String>,
    pub endpoints: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// Tier discovery request
#[derive(Debug, Serialize, Deserialize)]
pub struct TierDiscoveryRequest {
    pub dataset_name: String,
    pub parent_pool: String,
    pub context: String,
}

/// Tier discovery response
#[derive(Debug, Serialize, Deserialize)]
pub struct TierDiscoveryResponse {
    pub recommended_tier: StorageTier,
    pub reasoning: String,
}

/// Dataset creation notification
#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetCreatedNotification {
    pub dataset_name: String,
    pub tier: StorageTier,
    pub mount_point: String,
    pub service_id: String,
}

/// MCP tier prediction task
#[derive(Debug, Serialize, Deserialize)]
pub struct McpTierPredictionTask {
    pub file_path: String,
    pub request_id: String,
    pub priority: TaskPriority,
    pub context: StorageContext,
    pub preferred_compute_service: Option<String>,
}

/// MCP optimization task
#[derive(Debug, Serialize, Deserialize)]
pub struct McpOptimizationTask {
    pub dataset_name: String,
    pub current_tier: StorageTier,
    pub request_id: String,
    pub storage_context: DatasetContext,
}

/// MCP task response
#[derive(Debug, Serialize, Deserialize)]
pub struct McpTaskResponse {
    pub task_id: String,
    pub status: String,
}

/// MCP task status
#[derive(Debug, Serialize, Deserialize)]
pub struct McpTaskStatus {
    pub task_id: String,
    pub state: String,
    pub progress: f64,
    pub result: Option<TierDiscoveryResponse>,
    pub error: Option<String>,
}

/// MCP optimization status
#[derive(Debug, Serialize, Deserialize)]
pub struct McpOptimizationStatus {
    pub task_id: String,
    pub state: String,
    pub progress: f64,
    pub result: Option<crate::types::optimization::OptimizationResult>,
    pub error: Option<String>,
}

/// Direct data analysis request to external compute service
#[derive(Debug, Serialize, Deserialize)]
pub struct DirectAnalysisRequest {
    pub task_type: String,
    pub input_data: serde_json::Value,
    pub request_id: String,
}

/// Direct data analysis response from external compute service
#[derive(Debug, Serialize, Deserialize)]
pub struct DirectAnalysisResponse {
    pub request_id: String,
    pub result_data: serde_json::Value,
    pub processing_time_ms: u64,
    pub confidence: f64,
}
