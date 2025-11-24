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
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::SecurityConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::SecurityConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct SecurityConfig {
    pub authentication_required: bool,
    pub encryption_required: bool,
    pub allowed_primals: Option<Vec<String>>,
    pub security_level: SecurityLevel,
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type SecurityConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using SecurityConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.
