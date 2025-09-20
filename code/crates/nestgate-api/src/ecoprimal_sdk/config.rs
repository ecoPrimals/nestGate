//
// This module contains configuration types and structures for primal configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Primal configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct ResourceAllocation {
    /// Resource limits
    pub limits: ResourceLimits,
    /// Resource requests
    pub requests: ResourceRequests,
}
/// Resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct KeyManagementSettings {
    /// Key management type
    pub key_type: KeyManagementType,
    /// Key rotation interval in days
    pub rotation_interval_days: Option<u32>,
}
/// Key management types
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub enum AuditDestination {
    /// Local file
    File(String),
    /// Syslog
    Syslog,
    /// External service
    External(String),
}
