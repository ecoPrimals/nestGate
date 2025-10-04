//! Canonical Types for `NestGate`,
//!
//! Unified type system that consolidates all data structures across
//! the `NestGate` ecosystem into consistent, canonical forms.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Canonical Service Type Classification,
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnifiedServiceType {
    Storage,
    Network,
    Compute,
    Security,
    Intelligence,
    Orchestration,
}
impl std::fmt::Display for UnifiedServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Storage => write!(f, "Storage"),
            Self::Network => write!(f, "Network"),
            Self::Compute => write!(f, "Compute"),
            Self::Security => write!(f, "Security"),
            Self::Intelligence => write!(f, "Intelligence"),
            Self::Orchestration => write!(f, "Orchestration"),
        }
    }
}

/// Canonical Capability Identifier,
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CapabilityId {
    pub domain: String,
    pub capability: String,
    pub version: String,
}
impl CapabilityId {
    /// Create a new capability ID with pedantic validation
    #[must_use]
    pub fn new(domain: String, capability: String, version: String) -> Self {
        Self {
            domain,
            capability,
            version,
        }
    }

    /// Get the capability domain
    #[must_use]
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Get the capability name
    #[must_use]
    pub fn capability(&self) -> &str {
        &self.capability
    }

    /// Get the capability version
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
}

/// Canonical Storage Tier Classification,
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageTier {
    Hot,
    Warm,
    Cold,
    Archive,
    Cache,
}
/// Canonical File Analysis Result,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub path: String,
    pub size: u64,
    pub access_pattern: AccessPattern,
    pub recommended_tier: StorageTier,
    pub compression_ratio: f64,
    pub last_accessed: SystemTime,
    pub metadata: HashMap<String, String>,
}
/// Canonical Access Pattern Classification,
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessPattern {
    Sequential,
    Random,
    WriteOnce,
    ReadHeavy,
    WriteHeavy,
    Streaming,
}
/// Canonical Service Health Status,
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub healthy: bool,
    pub message: String,
    pub details: HashMap<String, String>,
}
impl Default for ServiceHealth {
    fn default() -> Self {
        Self {
            healthy: true,
            message: "Service operational".to_string(),
            details: HashMap::new(),
        }
    }
}

impl ServiceHealth {
    /// Create a healthy service status
    #[must_use]
    pub fn healthy() -> Self {
        Self {
            healthy: true,
            message: "Service is healthy".to_string(),
            details: HashMap::new(),
        }
    }

    /// Create a shutdown service status
    #[must_use]
    pub fn shutdown() -> Self {
        Self {
            healthy: false,
            message: "Service is shutting down".to_string(),
            details: HashMap::new(),
        }
    }
}

/// Canonical Service Metrics,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub network_io: u64,
    pub disk_io: u64,
    pub request_count: u64,
    pub error_count: u64,
    pub response_time_ms: u64,
    pub uptime_seconds: u64,
}
impl Default for ServiceMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            network_io: 0,
            disk_io: 0,
            request_count: 0,
            error_count: 0,
            response_time_ms: 0,
            uptime_seconds: 0,
        }
    }
}

/// Canonical Configuration Base,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalConfig {
    pub service_name: String,
    pub version: String,
    pub environment: String,
    pub debug_mode: bool,
    pub log_level: String,
    pub network: NetworkConfig,
    pub storage: StorageConfig,
    pub security: SecurityConfig,
    pub performance: PerformanceConfig,
}
/// Canonical Network Configuration,
/// **⚠️ DEPRECATED**: Use `CanonicalNetworkConfig` from `nestgate_core::config::canonical_master::domains::network`
#[deprecated(
    since = "0.9.0",
    note = "Use nestgate_core::config::canonical_master::domains::network::CanonicalNetworkConfig instead"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub bind_endpoint: String,
    pub port: u16,
    pub timeout_seconds: u64,
    pub max_connections: u32,
    pub enable_tls: bool,
    pub websocket_port: Option<u16>,
}
/// Canonical Storage Configuration,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub backend_type: String,
    pub data_directory: String,
    pub cache_size_mb: u64,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
    pub backup_enabled: bool,
}
/// Canonical Security Configuration,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub authentication_enabled: bool,
    pub authorization_enabled: bool,
    pub session_timeout_minutes: u64,
    pub max_login_attempts: u32,
    pub password_policy: PasswordPolicy,
}
/// Canonical Password Policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    pub min_length: u32,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_special: bool,
}
/// Canonical Performance Configuration,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub thread_pool_size: u32,
    pub buffer_size_kb: u32,
    pub batch_size: u32,
    pub enable_metrics: bool,
    pub metrics_interval_seconds: u64,
}
/// Canonical Request/Response Types,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalRequest {
    pub id: String,
    pub service_type: UnifiedServiceType,
    pub capability: CapabilityId,
    pub payload: HashMap<String, serde_json::Value>,
    pub timeout: Option<Duration>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalResponse {
    pub request_id: String,
    pub success: bool,
    pub payload: Option<serde_json::Value>,
    pub error: Option<String>,
    pub metrics: Option<ServiceMetrics>,
}
impl Default for CanonicalResponse {
    fn default() -> Self {
        Self {
            request_id: "unknown".to_string(),
            success: false,
            payload: None,
            error: None,
            metrics: None,
        }
    }
}

impl Default for CanonicalConfig {
    fn default() -> Self {
        Self {
            service_name: "nestgate".to_string(),
            version: "2.0.0".to_string(),
            environment: "production".to_string(),
            debug_mode: false,
            log_level: "info".to_string(),
            network: NetworkConfig::default(),
            storage: StorageConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            bind_endpoint: std::env::var("NESTGATE_BIND_ADDRESS")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("NESTGATE_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            timeout_seconds: 30,
            max_connections: 1000,
            enable_tls: false,
            websocket_port: None,
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            backend_type: "local".to_string(),
            data_directory: std::env::var("NESTGATE_DATA_DIR")
                .unwrap_or_else(|_| "./data".to_string()),
            cache_size_mb: 512,
            compression_enabled: true,
            encryption_enabled: false,
            backup_enabled: true,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            authentication_enabled: true,
            authorization_enabled: true,
            session_timeout_minutes: 60,
            max_login_attempts: 5,
            password_policy: PasswordPolicy::default(),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            thread_pool_size: num_cpus::get() as u32,
            buffer_size_kb: 1024,
            batch_size: 100,
            enable_metrics: true,
            metrics_interval_seconds: 60,
        }
    }
}

/// Authentication configuration for canonical modernization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// JWT secret key
    pub jwt_secret: String,
    /// Token expiry in hours
    pub token_expiry_hours: u64,
    /// Enable authorization
    pub authorization_enabled: bool,
    /// Maximum login attempts
    pub max_login_attempts: u32,
    /// Password policy
    pub password_policy: PasswordPolicy,
    /// Session timeout in minutes
    pub session_timeout_minutes: u64,
}
impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 12,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special: false,
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "default-secret".to_string(),
            token_expiry_hours: 24,
            authorization_enabled: true,
            max_login_attempts: 5,
            password_policy: PasswordPolicy::default(),
            session_timeout_minutes: 60,
        }
    }
}
