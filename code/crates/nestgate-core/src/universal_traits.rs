//! Universal Primal Traits
//!
//! This module defines universal traits that any primal can implement,
//! eliminating hardcoded dependencies on specific primal implementations.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

use crate::Result;

/// Universal primal provider trait that any primal can implement
#[async_trait]
pub trait PrimalProvider: Send + Sync {
    /// Unique primal identifier (e.g., "beardog", "nestgate", "squirrel", "toadstool")
    fn primal_id(&self) -> &str;

    /// Instance identifier for multi-instance support
    fn instance_id(&self) -> &str;

    /// User/device context this primal instance serves
    fn context(&self) -> &PrimalContext;

    /// Primal type category
    fn primal_type(&self) -> PrimalType;

    /// Capabilities this primal provides
    fn capabilities(&self) -> Vec<PrimalCapability>;

    /// What this primal needs from other primals
    fn dependencies(&self) -> Vec<PrimalDependency>;

    /// Health check for this primal
    async fn health_check(&self) -> PrimalHealth;

    /// Get primal API endpoints
    fn endpoints(&self) -> PrimalEndpoints;

    /// Handle inter-primal communication
    async fn handle_primal_request(&self, request: PrimalRequest) -> Result<PrimalResponse>;

    /// Initialize the primal with configuration
    async fn initialize(&mut self, config: serde_json::Value) -> Result<()>;

    /// Shutdown the primal gracefully
    async fn shutdown(&mut self) -> Result<()>;

    /// Check if this primal can serve the given context
    fn can_serve_context(&self, context: &PrimalContext) -> bool;
}

/// Universal security primal provider trait
#[async_trait]
pub trait SecurityPrimalProvider: Send + Sync {
    /// Authenticate with provided credentials
    async fn authenticate(&self, credentials: &Credentials) -> Result<AuthToken>;

    /// Encrypt data with specified algorithm
    async fn encrypt(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>>;

    /// Decrypt data with specified algorithm
    async fn decrypt(&self, encrypted: &[u8], algorithm: &str) -> Result<Vec<u8>>;

    /// Sign data cryptographically
    async fn sign_data(&self, data: &[u8]) -> Result<Signature>;

    /// Verify cryptographic signature
    async fn verify_signature(&self, data: &[u8], signature: &Signature) -> Result<bool>;

    /// Get signing key identifier
    async fn get_key_id(&self) -> Result<String>;

    /// Validate security token
    async fn validate_token(&self, token: &str, data: &[u8]) -> Result<bool>;

    /// Generate validation token
    async fn generate_validation_token(&self, data: &[u8]) -> Result<String>;

    /// Check if access crosses security boundary
    async fn evaluate_boundary_access(
        &self,
        source: &str,
        destination: &str,
        operation: &str,
    ) -> Result<SecurityDecision>;
}

/// Security decision for boundary access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityDecision {
    /// Access allowed
    Allow,
    /// Access denied
    Deny,
    /// Require authentication
    RequireAuth,
}

/// Universal AI primal provider trait
#[async_trait]
pub trait AiPrimalProvider: Send + Sync {
    /// Forecast storage capacity needs
    async fn forecast_capacity(&self, metrics: &SystemMetrics) -> Result<CapacityForecast>;

    /// Analyze performance bottlenecks
    async fn analyze_bottlenecks(&self, performance: &PerformanceData) -> Result<BottleneckAnalysis>;

    /// Plan maintenance schedules
    async fn plan_maintenance(&self, health: &HealthData) -> Result<MaintenancePlan>;

    /// Optimize replication strategies
    async fn optimize_replication(&self, replication: &ReplicationData) -> Result<OptimizationPlan>;

    /// Optimize snapshot retention
    async fn optimize_snapshots(&self, snapshot_data: &SnapshotData) -> Result<SnapshotOptimization>;

    /// Predict tier placement
    async fn predict_tier_placement(&self, file_data: &FileAnalysis) -> Result<TierRecommendation>;

    /// Analyze dataset patterns
    async fn analyze_dataset(&self, dataset: &DatasetAnalysis) -> Result<DatasetInsights>;
}

/// Universal orchestration primal provider trait
#[async_trait]
pub trait OrchestrationPrimalProvider: Send + Sync {
    /// Register service with orchestrator
    async fn register_service(&self, service: &ServiceRegistration) -> Result<String>;

    /// Discover available services
    async fn discover_services(&self, service_type: &str) -> Result<Vec<ServiceInstance>>;

    /// Allocate port for service
    async fn allocate_port(&self, service: &str, port_type: &str) -> Result<u16>;

    /// Release allocated port
    async fn release_port(&self, service: &str, port: u16) -> Result<()>;

    /// Route request between services
    async fn route_request(&self, request: &InterPrimalRequest) -> Result<InterPrimalResponse>;

    /// Get service health status
    async fn get_service_health(&self, service: &str) -> Result<ServiceHealth>;

    /// Load balance across service instances
    async fn load_balance(&self, service: &str, request: &ServiceRequest) -> Result<ServiceResponse>;
}

/// Universal compute primal provider trait
#[async_trait]
pub trait ComputePrimalProvider: Send + Sync {
    /// Allocate compute resources
    async fn allocate_resources(&self, spec: &ResourceSpec) -> Result<ResourceAllocation>;

    /// Execute workload
    async fn execute_workload(&self, workload: &WorkloadSpec) -> Result<WorkloadResult>;

    /// Monitor performance metrics
    async fn monitor_performance(&self, allocation: &ResourceAllocation) -> Result<PerformanceMetrics>;

    /// Scale resources up or down
    async fn scale_resources(&self, allocation: &ResourceAllocation, target: &ScalingTarget) -> Result<()>;

    /// Get resource utilization
    async fn get_resource_utilization(&self) -> Result<ResourceUtilization>;

    /// Detect platform capabilities
    async fn detect_platform(&self) -> Result<PlatformCapabilities>;

    /// Optimize resource allocation
    async fn optimize_allocation(&self, current: &ResourceAllocation, metrics: &PerformanceMetrics) -> Result<OptimizationRecommendation>;
}

// Supporting types and structures

/// Context for user/device-specific primal routing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PrimalContext {
    pub user_id: String,
    pub device_id: String,
    pub session_id: String,
    pub network_location: NetworkLocation,
    pub security_level: SecurityLevel,
    pub metadata: HashMap<String, String>,
}

/// Network location information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLocation {
    pub ip_address: String,
    pub subnet: Option<String>,
    pub network_id: Option<String>,
    pub geo_location: Option<String>,
}

impl Default for NetworkLocation {
    fn default() -> Self {
        Self {
            ip_address: "127.0.0.1".to_string(),
            subnet: None,
            network_id: None,
            geo_location: None,
        }
    }
}

/// Security level requirements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityLevel {
    Basic,
    Standard,
    High,
    Maximum,
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self::Basic
    }
}

/// Primal type categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimalType {
    Security,
    Storage,
    Compute,
    AI,
    Network,
    Custom(String),
}

/// Universal capabilities that any primal can provide
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimalCapability {
    // Security capabilities
    Authentication { methods: Vec<String> },
    Encryption { algorithms: Vec<String> },
    KeyManagement { hsm_support: bool },
    ThreatDetection { ml_enabled: bool },
    Compliance { frameworks: Vec<String> },

    // Storage capabilities
    FileSystem { features: Vec<String> },
    ObjectStorage { backends: Vec<String> },
    DataReplication { consistency: String },
    VolumeManagement { protocols: Vec<String> },
    BackupRestore { incremental: bool },

    // Compute capabilities
    ContainerOrchestration { runtime: String },
    ResourceManagement { auto_scaling: bool },
    WorkloadScheduling { algorithms: Vec<String> },
    PerformanceOptimization { ml_enabled: bool },

    // AI capabilities
    ModelInference { models: Vec<String> },
    MachineLearning { training_support: bool },
    NaturalLanguage { languages: Vec<String> },
    PredictiveAnalytics { domains: Vec<String> },

    // Network capabilities
    ServiceDiscovery { protocols: Vec<String> },
    LoadBalancing { algorithms: Vec<String> },
    NetworkRouting { protocols: Vec<String> },
    CircuitBreaking { enabled: bool },
}

/// Dependencies that a primal needs from other primals
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimalDependency {
    RequiresAuthentication { methods: Vec<String> },
    RequiresEncryption { algorithms: Vec<String> },
    RequiresOrchestration { capabilities: Vec<String> },
    RequiresCompute { resources: Vec<String> },
    RequiresAI { capabilities: Vec<String> },
    RequiresStorage { capabilities: Vec<String> },
}

/// Health status for primals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimalHealth {
    Healthy,
    Degraded { reason: String },
    Unhealthy { reason: String },
    Unknown,
}

/// Primal endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalEndpoints {
    pub primary: String,
    pub health: String,
    pub metrics: Option<String>,
    pub admin: Option<String>,
    pub websocket: Option<String>,
    pub custom: HashMap<String, String>,
}

/// Universal request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalRequest {
    pub id: Uuid,
    pub source_primal: String,
    pub target_primal: String,
    pub request_type: PrimalRequestType,
    pub payload: HashMap<String, serde_json::Value>,
    pub timestamp: SystemTime,
    pub security_context: Option<PrimalContext>,
}

/// Universal response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalResponse {
    pub request_id: Uuid,
    pub response_type: PrimalResponseType,
    pub payload: HashMap<String, serde_json::Value>,
    pub timestamp: SystemTime,
    pub success: bool,
    pub error_message: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

/// Request types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimalRequestType {
    HealthCheck,
    Capability,
    Store,
    Retrieve,
    Compute,
    Analyze,
    Authenticate,
    Encrypt,
    Route,
    Custom(String),
}

/// Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimalResponseType {
    Health,
    Capability,
    Storage,
    Compute,
    Analysis,
    Authentication,
    Encryption,
    Routing,
    Custom(String),
}

// Security-related types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub domain: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub expires_at: SystemTime,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub algorithm: String,
    pub signature: String,
    pub key_id: String,
}



// AI-related types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_io: f64,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityForecast {
    pub predicted_usage: f64,
    pub confidence: f64,
    pub time_horizon: u64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceData {
    pub throughput: f64,
    pub latency: f64,
    pub error_rate: f64,
    pub resource_utilization: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckAnalysis {
    pub bottlenecks: Vec<String>,
    pub severity: String,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthData {
    pub overall_health: String,
    pub component_health: HashMap<String, String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenancePlan {
    pub scheduled_tasks: Vec<String>,
    pub priority: String,
    pub estimated_duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationData {
    pub sources: Vec<String>,
    pub targets: Vec<String>,
    pub current_strategy: String,
    pub performance_metrics: PerformanceData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationPlan {
    pub changes: Vec<String>,
    pub expected_improvement: f64,
    pub implementation_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotData {
    pub snapshots: Vec<String>,
    pub retention_policies: Vec<String>,
    pub storage_usage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotOptimization {
    pub retention_recommendations: Vec<String>,
    pub cleanup_candidates: Vec<String>,
    pub space_savings: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub file_path: String,
    pub size: u64,
    pub access_pattern: String,
    pub last_accessed: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierRecommendation {
    pub recommended_tier: String,
    pub confidence: f64,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetAnalysis {
    pub dataset_name: String,
    pub size: u64,
    pub file_count: u64,
    pub access_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInsights {
    pub usage_patterns: Vec<String>,
    pub optimization_opportunities: Vec<String>,
    pub health_score: f64,
}

// Orchestration-related types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    pub name: String,
    pub service_type: String,
    pub version: String,
    pub endpoints: Vec<String>,
    pub capabilities: Vec<String>,
    pub health_endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub status: String,
    pub last_heartbeat: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterPrimalRequest {
    pub id: Uuid,
    pub source: String,
    pub target: String,
    pub operation: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterPrimalResponse {
    pub request_id: Uuid,
    pub success: bool,
    pub payload: serde_json::Value,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub status: String,
    pub uptime: u64,
    pub last_check: SystemTime,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    pub id: Uuid,
    pub operation: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse {
    pub request_id: Uuid,
    pub success: bool,
    pub payload: serde_json::Value,
}

// Compute-related types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    pub cpu_cores: Option<f64>,
    pub memory_mb: Option<u64>,
    pub disk_mb: Option<u64>,
    pub gpu_count: Option<u32>,
    pub network_bandwidth: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub id: String,
    pub allocated_resources: ResourceSpec,
    pub status: String,
    pub created_at: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadSpec {
    pub id: String,
    pub image: String,
    pub command: Vec<String>,
    pub environment: HashMap<String, String>,
    pub resources: ResourceSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadResult {
    pub id: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub execution_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_io: f64,
    pub network_io: f64,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingTarget {
    pub target_cpu: Option<f64>,
    pub target_memory: Option<f64>,
    pub min_instances: Option<u32>,
    pub max_instances: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilization {
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub disk_percent: f64,
    pub network_utilization: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformCapabilities {
    pub architecture: String,
    pub os_type: String,
    pub container_runtime: String,
    pub gpu_support: bool,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    pub recommendations: Vec<String>,
    pub expected_improvement: f64,
    pub confidence: f64,
} 