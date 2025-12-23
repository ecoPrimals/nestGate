//
// This module contains configuration types and structures for primal configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Primal configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::config::PrimalConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::PrimalConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for Primal
pub struct PrimalConfig {
    /// Unique instance identifier
    pub instance_id: String,
    /// Biome this primal belongs to
    pub biome_id: Option<String>,
    /// Network configuration
    pub network: NetworkConfig,
    /// Resource allocation
    pub resources: ResourceAllocation,
    /// Security settings
    pub security: SecuritySettings,
    /// Logging configuration  
    pub logging: super::types::LoggingConfig,
    /// Custom primal-specific configuration
    pub custom: HashMap<String, serde_json::Value>,
    /// Environment variables
    pub environment: HashMap<String, String>,
}
/// Network configuration
/// **⚠️ DEPRECATED**: Use `CanonicalNetworkConfig` from `nestgate_core::config::canonical_primary::domains::network`
#[deprecated(since = "0.9.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Network
pub struct NetworkConfig {
    /// Bind address
    pub bind_endpoint: String,
    /// Primary port
    pub port: u16,
    /// Additional ports
    pub additional_ports: HashMap<String, u16>,
    /// Enable TLS
    pub tls_enabled: bool,
    /// Certificate configuration
    pub certificates: Option<CertificateConfig>,
    /// Network policies
    pub policies: Vec<NetworkPolicy>,
}
/// Certificate configuration  
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::config::CertificateConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::CertificateConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for Certificate
pub struct CertificateConfig {
    /// Certificate file path
    pub cert_file: String,
    /// Private key file path
    pub key_file: String,
    /// CA certificate file path
    pub ca_file: Option<String>,
    /// Certificate source
    pub source: CertificateSource,
}
/// Certificate sources
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Certificatesource
pub enum CertificateSource {
    /// File-based certificates
    File,
    /// Environment variables
    Environment,
    /// Kubernetes secrets
    K8sSecret(String),
    /// External certificate manager
    External(String),
}
/// Network policy
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkpolicy
pub struct NetworkPolicy {
    /// Policy name
    pub name: String,
    /// Source CIDR blocks
    pub source_cidrs: Vec<String>,
    /// Destination ports
    pub destination_ports: Vec<u16>,
    /// Policy action
    pub action: PolicyAction,
}
/// Policy actions
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Policyaction
pub enum PolicyAction {
    /// Allow the connection
    Allow,
    /// Deny the connection
    Deny,
    /// Log and allow
    LogAllow,
    /// Log and deny
    LogDeny,
}
/// Resource allocation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Resourceallocation
pub struct ResourceAllocation {
    /// Resource limits
    pub limits: ResourceLimits,
    /// Resource requests
    pub requests: ResourceRequests,
}
/// Resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Resourcelimits
pub struct ResourceLimits {
    /// CPU limit in cores
    pub cpu_cores: Option<f64>,
    /// Memory limit in bytes
    pub memory_bytes: Option<u64>,
    /// Disk limit in bytes
    pub disk_bytes: Option<u64>,
    /// Network bandwidth limit in bytes per second
    pub network_bps: Option<u64>,
}
/// Resource requests
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Resourcerequests
pub struct ResourceRequests {
    /// CPU request in cores
    pub cpu_cores: Option<f64>,
    /// Memory request in bytes
    pub memory_bytes: Option<u64>,
    /// Disk request in bytes
    pub disk_bytes: Option<u64>,
    /// Network bandwidth request in bytes per second
    pub network_bps: Option<u64>,
}
/// Security settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Securitysettings
pub struct SecuritySettings {
    /// Authentication policy
    pub auth_policy: AuthPolicy,
    /// Encryption settings
    pub encryption: EncryptionSettings,
    /// Audit settings
    pub audit: AuditSettings,
}
/// Authentication policy
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authpolicy
pub struct AuthPolicy {
    /// Authentication methods
    pub methods: Vec<AuthMethod>,
    /// Policy scope
    pub scope: PolicyScope,
    /// Session timeout in seconds
    pub session_timeout: Option<u64>,
}
/// Authentication methods
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authmethod
pub enum AuthMethod {
    /// No authentication
    None,
    /// Basic authentication
    Basic,
    /// Bearer token
    Bearer,
    /// JWT tokens
    Jwt,
    /// OAuth2
    OAuth2,
    /// Custom authentication
    Custom(String),
}
/// Policy scope
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Policyscope
pub enum PolicyScope {
    /// Global policy
    Global,
    /// Per-biome policy
    Biome,
    /// Per-primal policy
    Primal,
}
/// Encryption settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Encryptionsettings
pub struct EncryptionSettings {
    /// Enable encryption at rest
    pub at_rest: bool,
    /// Enable encryption in transit
    pub in_transit: bool,
    /// Key management settings
    pub key_management: KeyManagementSettings,
}
/// Key management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Keymanagementsettings
pub struct KeyManagementSettings {
    /// Key management type
    pub key_type: KeyManagementType,
    /// Key rotation interval in days
    pub rotation_interval_days: Option<u32>,
}
/// Key management types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of KeyManagement
pub enum KeyManagementType {
    /// Internal key management
    Internal,
    /// External key management service
    External(String),
    /// Hardware security module
    Hsm(String),
}
/// Audit settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Auditsettings
pub struct AuditSettings {
    /// Enable audit logging
    pub enabled: bool,
    /// Audit level
    pub level: AuditLevel,
    /// Audit destinations
    pub destinations: Vec<AuditDestination>,
}
/// Audit levels
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Auditlevel
pub enum AuditLevel {
    /// Minimal auditing
    Minimal,
    /// Standard auditing
    Standard,
    /// Detailed auditing
    Detailed,
    /// Full auditing
    Full,
}
/// Audit destinations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Auditdestination
pub enum AuditDestination {
    /// Local file
    File(String),
    /// Syslog
    Syslog,
    /// External service
    External(String),
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Primalconfigcanonical
pub type PrimalConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using PrimalConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Certificateconfigcanonical
pub type CertificateConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using CertificateConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

