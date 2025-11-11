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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[allow(deprecated)] // MIGRATION: Use CanonicalNetworkConfig in next major version
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
/// **⚠️ DEPRECATED**: Use `CanonicalNetworkConfig` from `nestgate_core::config::canonical_primary::domains::network`
#[deprecated(
    since = "0.9.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
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
#[allow(clippy::struct_excessive_bools)] // Policy flags are semantically correct here
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

#[allow(deprecated)] // MIGRATION: Use CanonicalNetworkConfig in next major version
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

#[allow(deprecated)] // MIGRATION: Use CanonicalNetworkConfig in next major version
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
            thread_pool_size: u32::try_from(num_cpus::get()).unwrap_or(4),
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

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== UnifiedServiceType Tests ====================

    #[test]
    fn test_unified_service_type_display() {
        assert_eq!(UnifiedServiceType::Storage.to_string(), "Storage");
        assert_eq!(UnifiedServiceType::Network.to_string(), "Network");
        assert_eq!(UnifiedServiceType::Compute.to_string(), "Compute");
        assert_eq!(UnifiedServiceType::Security.to_string(), "Security");
        assert_eq!(UnifiedServiceType::Intelligence.to_string(), "Intelligence");
        assert_eq!(
            UnifiedServiceType::Orchestration.to_string(),
            "Orchestration"
        );
    }

    #[test]
    fn test_unified_service_type_clone() {
        let service = UnifiedServiceType::Storage;
        let cloned = service.clone();
        assert_eq!(service, cloned);
    }

    #[test]
    fn test_unified_service_type_equality() {
        assert_eq!(UnifiedServiceType::Storage, UnifiedServiceType::Storage);
        assert_ne!(UnifiedServiceType::Storage, UnifiedServiceType::Network);
    }

    #[test]
    fn test_unified_service_type_serialization() {
        let service = UnifiedServiceType::Storage;
        let json = serde_json::to_string(&service).expect("String operation failed");
        assert!(json.contains("Storage"));
    }

    #[test]
    fn test_unified_service_type_deserialization() {
        let json = "\"Storage\"";
        let service: UnifiedServiceType =
            serde_json::from_str(json).expect("Failed to convert from string");
        assert_eq!(service, UnifiedServiceType::Storage);
    }

    // ==================== CapabilityId Tests ====================

    #[test]
    fn test_capability_id_new() {
        let cap_id = CapabilityId::new(
            "storage".to_string(),
            "zfs".to_string(),
            "1.0.0".to_string(),
        );
        assert_eq!(cap_id.domain(), "storage");
        assert_eq!(cap_id.capability(), "zfs");
        assert_eq!(cap_id.version(), "1.0.0");
    }

    #[test]
    fn test_capability_id_getters() {
        let cap_id = CapabilityId {
            domain: "network".to_string(),
            capability: "http".to_string(),
            version: "2.0.0".to_string(),
        };
        assert_eq!(cap_id.domain(), "network");
        assert_eq!(cap_id.capability(), "http");
        assert_eq!(cap_id.version(), "2.0.0");
    }

    #[test]
    fn test_capability_id_clone() {
        let cap_id = CapabilityId::new(
            "compute".to_string(),
            "gpu".to_string(),
            "1.5.0".to_string(),
        );
        let cloned = cap_id.clone();
        assert_eq!(cap_id, cloned);
    }

    #[test]
    fn test_capability_id_equality() {
        let cap1 = CapabilityId::new("a".to_string(), "b".to_string(), "1.0".to_string());
        let cap2 = CapabilityId::new("a".to_string(), "b".to_string(), "1.0".to_string());
        let cap3 = CapabilityId::new("a".to_string(), "b".to_string(), "2.0".to_string());
        assert_eq!(cap1, cap2);
        assert_ne!(cap1, cap3);
    }

    #[test]
    fn test_capability_id_serialization() {
        let cap_id = CapabilityId::new(
            "storage".to_string(),
            "zfs".to_string(),
            "1.0.0".to_string(),
        );
        let json = serde_json::to_string(&cap_id).expect("String operation failed");
        assert!(json.contains("storage"));
        assert!(json.contains("zfs"));
    }

    // ==================== StorageTier Tests ====================

    #[test]
    fn test_storage_tier_variants() {
        let _hot = StorageTier::Hot;
        let _warm = StorageTier::Warm;
        let _cold = StorageTier::Cold;
        let _archive = StorageTier::Archive;
        let _cache = StorageTier::Cache;
    }

    #[test]
    fn test_storage_tier_equality() {
        assert_eq!(StorageTier::Hot, StorageTier::Hot);
        assert_ne!(StorageTier::Hot, StorageTier::Cold);
    }

    #[test]
    fn test_storage_tier_clone() {
        let tier = StorageTier::Warm;
        let cloned = tier.clone();
        assert_eq!(tier, cloned);
    }

    // ==================== AccessPattern Tests ====================

    #[test]
    fn test_access_pattern_variants() {
        let patterns = [
            AccessPattern::Sequential,
            AccessPattern::Random,
            AccessPattern::WriteOnce,
            AccessPattern::ReadHeavy,
            AccessPattern::WriteHeavy,
            AccessPattern::Streaming,
        ];
        assert_eq!(patterns.len(), 6);
    }

    #[test]
    fn test_access_pattern_equality() {
        assert_eq!(AccessPattern::Sequential, AccessPattern::Sequential);
        assert_ne!(AccessPattern::Sequential, AccessPattern::Random);
    }

    #[test]
    fn test_access_pattern_clone() {
        let pattern = AccessPattern::ReadHeavy;
        let cloned = pattern.clone();
        assert_eq!(pattern, cloned);
    }

    // ==================== ServiceHealth Tests ====================

    #[test]
    fn test_service_health_default() {
        let health = ServiceHealth::default();
        assert!(health.healthy);
        assert_eq!(health.message, "Service operational");
        assert!(health.details.is_empty());
    }

    #[test]
    fn test_service_health_healthy() {
        let health = ServiceHealth::healthy();
        assert!(health.healthy);
        assert_eq!(health.message, "Service is healthy");
        assert!(health.details.is_empty());
    }

    #[test]
    fn test_service_health_shutdown() {
        let health = ServiceHealth::shutdown();
        assert!(!health.healthy);
        assert_eq!(health.message, "Service is shutting down");
        assert!(health.details.is_empty());
    }

    #[test]
    fn test_service_health_clone() {
        let health = ServiceHealth::healthy();
        let cloned = health.clone();
        assert_eq!(health.healthy, cloned.healthy);
        assert_eq!(health.message, cloned.message);
    }

    #[test]
    fn test_service_health_with_details() {
        let mut health = ServiceHealth::healthy();
        health.details.insert("cpu".to_string(), "50%".to_string());
        health
            .details
            .insert("memory".to_string(), "2GB".to_string());
        assert_eq!(health.details.len(), 2);
        assert_eq!(health.details.get("cpu"), Some(&"50%".to_string()));
    }

    // ==================== ServiceMetrics Tests ====================

    #[test]
    fn test_service_metrics_default() {
        let metrics = ServiceMetrics::default();
        assert_eq!(metrics.cpu_usage, 0.0);
        assert_eq!(metrics.memory_usage, 0.0);
        assert_eq!(metrics.network_io, 0);
        assert_eq!(metrics.disk_io, 0);
        assert_eq!(metrics.request_count, 0);
        assert_eq!(metrics.error_count, 0);
        assert_eq!(metrics.response_time_ms, 0);
        assert_eq!(metrics.uptime_seconds, 0);
    }

    #[test]
    fn test_service_metrics_clone() {
        let metrics = ServiceMetrics {
            cpu_usage: 45.5,
            memory_usage: 60.2,
            network_io: 1024,
            disk_io: 2048,
            request_count: 100,
            error_count: 5,
            response_time_ms: 250,
            uptime_seconds: 3600,
        };
        let cloned = metrics.clone();
        assert_eq!(metrics.cpu_usage, cloned.cpu_usage);
        assert_eq!(metrics.request_count, cloned.request_count);
    }

    #[test]
    fn test_service_metrics_serialization() {
        let metrics = ServiceMetrics::default();
        let json = serde_json::to_string(&metrics).expect("String operation failed");
        assert!(json.contains("cpu_usage"));
        assert!(json.contains("memory_usage"));
    }

    // ==================== NetworkConfig Tests ====================

    #[test]
    #[allow(deprecated)]
    fn test_network_config_default() {
        let config = NetworkConfig::default();
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_connections, 1000);
        assert!(!config.enable_tls);
        assert_eq!(config.websocket_port, None);
    }

    #[test]
    #[allow(deprecated)]
    fn test_network_config_clone() {
        let config = NetworkConfig {
            bind_endpoint: "0.0.0.0".to_string(),
            port: 8080,
            timeout_seconds: 60,
            max_connections: 500,
            enable_tls: true,
            websocket_port: Some(8081),
        };
        let cloned = config.clone();
        assert_eq!(config.port, cloned.port);
        assert_eq!(config.enable_tls, cloned.enable_tls);
    }

    // ==================== StorageConfig Tests ====================

    #[test]
    fn test_storage_config_default() {
        let config = StorageConfig::default();
        assert_eq!(config.backend_type, "local");
        assert_eq!(config.cache_size_mb, 512);
        assert!(config.compression_enabled);
        assert!(!config.encryption_enabled);
        assert!(config.backup_enabled);
    }

    #[test]
    fn test_storage_config_clone() {
        let config = StorageConfig {
            backend_type: "zfs".to_string(),
            data_directory: "/data".to_string(),
            cache_size_mb: 1024,
            compression_enabled: true,
            encryption_enabled: true,
            backup_enabled: false,
        };
        let cloned = config.clone();
        assert_eq!(config.backend_type, cloned.backend_type);
        assert_eq!(config.cache_size_mb, cloned.cache_size_mb);
    }

    // ==================== SecurityConfig Tests ====================

    #[test]
    fn test_security_config_default() {
        let config = SecurityConfig::default();
        assert!(config.authentication_enabled);
        assert!(config.authorization_enabled);
        assert_eq!(config.session_timeout_minutes, 60);
        assert_eq!(config.max_login_attempts, 5);
    }

    #[test]
    fn test_security_config_clone() {
        let config = SecurityConfig::default();
        let cloned = config.clone();
        assert_eq!(config.authentication_enabled, cloned.authentication_enabled);
        assert_eq!(config.max_login_attempts, cloned.max_login_attempts);
    }

    // ==================== PasswordPolicy Tests ====================

    #[test]
    fn test_password_policy_default() {
        let policy = PasswordPolicy::default();
        assert_eq!(policy.min_length, 12);
        assert!(policy.require_uppercase);
        assert!(policy.require_lowercase);
        assert!(policy.require_numbers);
        assert!(!policy.require_special);
    }

    #[test]
    fn test_password_policy_clone() {
        let policy = PasswordPolicy {
            min_length: 16,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special: true,
        };
        let cloned = policy.clone();
        assert_eq!(policy.min_length, cloned.min_length);
        assert_eq!(policy.require_special, cloned.require_special);
    }

    #[test]
    fn test_password_policy_serialization() {
        let policy = PasswordPolicy::default();
        let json = serde_json::to_string(&policy).expect("String operation failed");
        assert!(json.contains("min_length"));
        assert!(json.contains("require_uppercase"));
    }

    // ==================== PerformanceConfig Tests ====================

    #[test]
    fn test_performance_config_default() {
        let config = PerformanceConfig::default();
        assert!(config.thread_pool_size > 0);
        assert_eq!(config.buffer_size_kb, 1024);
        assert_eq!(config.batch_size, 100);
        assert!(config.enable_metrics);
        assert_eq!(config.metrics_interval_seconds, 60);
    }

    #[test]
    fn test_performance_config_clone() {
        let config = PerformanceConfig {
            thread_pool_size: 8,
            buffer_size_kb: 2048,
            batch_size: 200,
            enable_metrics: false,
            metrics_interval_seconds: 30,
        };
        let cloned = config.clone();
        assert_eq!(config.thread_pool_size, cloned.thread_pool_size);
        assert_eq!(config.batch_size, cloned.batch_size);
    }

    // ==================== CanonicalRequest Tests ====================

    #[test]
    fn test_canonical_request_creation() {
        let cap_id = CapabilityId::new(
            "storage".to_string(),
            "read".to_string(),
            "1.0.0".to_string(),
        );
        let request = CanonicalRequest {
            id: "req-123".to_string(),
            service_type: UnifiedServiceType::Storage,
            capability: cap_id,
            payload: HashMap::new(),
            timeout: Some(Duration::from_secs(30)),
        };
        assert_eq!(request.id, "req-123");
        assert_eq!(request.service_type, UnifiedServiceType::Storage);
    }

    #[test]
    fn test_canonical_request_with_payload() {
        let mut payload = HashMap::new();
        payload.insert("key".to_string(), serde_json::json!("value"));

        let cap_id = CapabilityId::new("a".to_string(), "b".to_string(), "1.0".to_string());
        let request = CanonicalRequest {
            id: "req-456".to_string(),
            service_type: UnifiedServiceType::Network,
            capability: cap_id,
            payload: payload.clone(),
            timeout: None,
        };
        assert_eq!(request.payload.len(), 1);
        assert_eq!(request.timeout, None);
    }

    // ==================== CanonicalResponse Tests ====================

    #[test]
    fn test_canonical_response_default() {
        let response = CanonicalResponse::default();
        assert_eq!(response.request_id, "unknown");
        assert!(!response.success);
        assert_eq!(response.payload, None);
        assert_eq!(response.error, None);
        assert_eq!(response.metrics, None);
    }

    #[test]
    fn test_canonical_response_success() {
        let response = CanonicalResponse {
            request_id: "req-789".to_string(),
            success: true,
            payload: Some(serde_json::json!({"result": "ok"})),
            error: None,
            metrics: None,
        };
        assert!(response.success);
        assert!(response.payload.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_canonical_response_error() {
        let response = CanonicalResponse {
            request_id: "req-error".to_string(),
            success: false,
            payload: None,
            error: Some("Operation failed".to_string()),
            metrics: None,
        };
        assert!(!response.success);
        assert!(response.error.is_some());
        assert_eq!(
            response.error.as_ref().expect("Operation failed"),
            "Operation failed"
        );
    }

    #[test]
    fn test_canonical_response_with_metrics() {
        let metrics = ServiceMetrics {
            cpu_usage: 25.0,
            memory_usage: 30.0,
            network_io: 512,
            disk_io: 1024,
            request_count: 50,
            error_count: 2,
            response_time_ms: 100,
            uptime_seconds: 7200,
        };
        let response = CanonicalResponse {
            request_id: "req-metrics".to_string(),
            success: true,
            payload: None,
            error: None,
            metrics: Some(metrics),
        };
        assert!(response.metrics.is_some());
        assert_eq!(
            response
                .metrics
                .as_ref()
                .expect("Operation failed")
                .request_count,
            50
        );
    }

    // ==================== CanonicalConfig Tests ====================

    #[test]
    #[allow(deprecated)]
    fn test_canonical_config_default() {
        let config = CanonicalConfig::default();
        assert_eq!(config.service_name, "nestgate");
        assert_eq!(config.version, "2.0.0");
        assert_eq!(config.environment, "production");
        assert!(!config.debug_mode);
        assert_eq!(config.log_level, "info");
    }

    #[test]
    #[allow(deprecated)]
    fn test_canonical_config_clone() {
        let config = CanonicalConfig::default();
        let cloned = config.clone();
        assert_eq!(config.service_name, cloned.service_name);
        assert_eq!(config.version, cloned.version);
    }

    // ==================== AuthConfig Tests ====================

    #[test]
    fn test_auth_config_default() {
        let config = AuthConfig::default();
        assert_eq!(config.jwt_secret, "default-secret");
        assert_eq!(config.token_expiry_hours, 24);
        assert!(config.authorization_enabled);
        assert_eq!(config.max_login_attempts, 5);
        assert_eq!(config.session_timeout_minutes, 60);
    }

    #[test]
    fn test_auth_config_clone() {
        let config = AuthConfig::default();
        let cloned = config.clone();
        assert_eq!(config.jwt_secret, cloned.jwt_secret);
        assert_eq!(config.token_expiry_hours, cloned.token_expiry_hours);
    }

    #[test]
    fn test_auth_config_custom() {
        let config = AuthConfig {
            jwt_secret: "custom-secret-key".to_string(),
            token_expiry_hours: 48,
            authorization_enabled: false,
            max_login_attempts: 3,
            password_policy: PasswordPolicy {
                min_length: 20,
                require_uppercase: true,
                require_lowercase: true,
                require_numbers: true,
                require_special: true,
            },
            session_timeout_minutes: 120,
        };
        assert_eq!(config.jwt_secret, "custom-secret-key");
        assert_eq!(config.password_policy.min_length, 20);
    }

    // ==================== FileAnalysis Tests ====================

    #[test]
    fn test_file_analysis_creation() {
        let mut metadata = HashMap::new();
        metadata.insert("owner".to_string(), "user1".to_string());

        let analysis = FileAnalysis {
            path: "/data/file.txt".to_string(),
            size: 1024,
            access_pattern: AccessPattern::ReadHeavy,
            recommended_tier: StorageTier::Hot,
            compression_ratio: 0.8,
            last_accessed: SystemTime::now(),
            metadata,
        };
        assert_eq!(analysis.path, "/data/file.txt");
        assert_eq!(analysis.size, 1024);
        assert_eq!(analysis.access_pattern, AccessPattern::ReadHeavy);
        assert_eq!(analysis.recommended_tier, StorageTier::Hot);
    }

    #[test]
    fn test_file_analysis_clone() {
        let analysis = FileAnalysis {
            path: "/test".to_string(),
            size: 512,
            access_pattern: AccessPattern::Sequential,
            recommended_tier: StorageTier::Warm,
            compression_ratio: 0.5,
            last_accessed: SystemTime::now(),
            metadata: HashMap::new(),
        };
        let cloned = analysis.clone();
        assert_eq!(analysis.path, cloned.path);
        assert_eq!(analysis.size, cloned.size);
    }
}
