//! Ecosystem service types for network integration

use super::{DatasetContext, ServiceHealth, StorageContext, TaskPriority};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Ecosystem integration configuration (capability-based)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemConfig {
    /// Universal adapter endpoint for capability discovery
    pub adapter_endpoint: Option<String>,
    /// Capability discovery timeout in seconds
    pub discovery_timeout: u64,
    /// Enable automatic capability detection
    pub auto_discovery: bool,
    /// Fallback to standalone mode if no capabilities found
    pub standalone_fallback: bool,
    /// Required capabilities for operation
    pub required_capabilities: Vec<String>,
    /// Optional capabilities that enhance functionality
    pub optional_capabilities: Vec<String>,
}

impl Default for EcosystemConfig {
    fn default() -> Self {
        Self {
            adapter_endpoint: std::env::var("ECOSYSTEM_ADAPTER_URL").ok(),
            discovery_timeout: 30,
            auto_discovery: true,
            standalone_fallback: true,
            required_capabilities: vec!["storage".to_string()],
            optional_capabilities: vec![
                "orchestration".to_string(),
                "security".to_string(),
                "artificial_intelligence".to_string(),
                "compute".to_string(),
            ],
        }
    }
}

/// Capability provider information (discovered dynamically)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityProvider {
    /// Unique service identifier
    pub service_id: Uuid,
    /// Service endpoint
    pub endpoint: String,
    /// Capabilities provided by this service
    pub capabilities: Vec<String>,
    /// Service category
    pub category: ServiceCategory,
    /// Service metadata
    pub metadata: HashMap<String, String>,
    /// Last seen timestamp
    pub last_seen: std::time::SystemTime,
}

/// Service category enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceCategory {
    Storage,
    Orchestration,
    Security,
    ArtificialIntelligence,
    Compute,
    Custom(String),
}

/// Ecosystem integration status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationStatus {
    /// Is ecosystem integration active
    pub active: bool,
    /// Connected capability providers
    pub providers: Vec<CapabilityProvider>,
    /// Available capabilities
    pub capabilities: Vec<String>,
    /// Integration health score (0.0 - 1.0)
    pub health_score: f64,
    /// Last update timestamp
    pub updated_at: std::time::SystemTime,
}

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
    pub last_seen: std::time::SystemTime,
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
        let duration = time
            .duration_since(UNIX_EPOCH)
            .unwrap_or(std::time::Duration::from_secs(0));
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
    pub recommended_tier: crate::types::StorageTier,
    pub reasoning: String,
}

/// Dataset creation notification
#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetCreatedNotification {
    pub dataset_name: String,
    pub tier: crate::types::StorageTier,
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
    pub current_tier: crate::types::StorageTier,
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
