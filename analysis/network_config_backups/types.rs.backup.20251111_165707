// **COMMON TYPES - CANONICAL MODERNIZED**
//! Type definitions for universal traits
// Common types and structures shared across universal traits modules.
// Organized for clarity and reusability across the ecosystem.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export types from other modules for convenience
pub use super::compute::{PerformanceMetrics, ResourceAllocation, ResourceSpec};
pub use super::ecosystem::{PrimalContext, PrimalHealth, PrimalInfo, PrimalType};
pub use super::orchestration::{ServiceHealth, ServiceInfo, ServiceRequest, ServiceResponse};
pub use super::security::{AuthToken, Credentials, Signature};

/// Network location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkLocation {
    pub host: String,
    pub port: u16,
    pub protocol: String,
    pub secure: bool,
}
/// Security level enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}
/// Primal capability enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimalCapability {
    Storage,
    Compute,
    Security,
    Orchestration,
    Monitoring,
    Analytics,
    MachineLearning,
    DataProcessing,
    NetworkManagement,
    Custom(String),
}
/// Primal dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimalDependency {
    Required(String),
    Optional(String),
    Preferred(String),
}
/// Primal endpoints configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalEndpoints {
    pub primary: NetworkLocation,
    pub backup: Option<NetworkLocation>,
    pub health_check: Option<NetworkLocation>,
    pub metrics: Option<NetworkLocation>,
}
/// System metrics collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
    pub network_throughput_mbps: f64,
    pub active_connections: u32,
    pub timestamp: std::time::SystemTime,
}
/// Configuration structure for universal traits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalConfig {
    pub primal_id: String,
    pub capabilities: Vec<PrimalCapability>,
    pub endpoints: PrimalEndpoints,
    pub security_level: SecurityLevel,
    pub dependencies: Vec<PrimalDependency>,
    pub metadata: HashMap<String, String>,
}
/// Ecosystem integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemIntegrationConfig {
    pub discovery_endpoint: String,
    pub heartbeat_interval_seconds: u64,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub security_config: SecurityConfig,
}
/// Security configuration for ecosystem integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub authentication_required: bool,
    pub encryption_required: bool,
    pub allowed_primals: Option<Vec<String>>,
    pub security_level: SecurityLevel,
}
