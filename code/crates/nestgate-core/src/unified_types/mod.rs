pub mod access_patterns;
pub mod cache_config;
pub mod error_types;
pub mod network_config;
pub mod retry_config;
pub mod service_config;
pub mod service_metadata;
// Unified Types Module System
// This module system breaks down the large unified_types.rs file into manageable,
// focused modules while maintaining the unified architecture principles.
/// **ACHIEVEMENT**: Reduces file sizes to <2k lines while preserving functionality
// Core configuration modules
pub mod timeout_config;
// Note: unified_storage_types was already created in the root unified_types/ directory
// Re-export all types for backward compatibility and ease of use
pub use access_patterns::{AccessTimePattern, UnifiedAccessPatterns};
pub use error_types::{
    DetailedErrorResponse, SimpleErrorResponse, UnifiedErrorContext, UnifiedErrorCore,
    UnifiedErrorSeverity, UnifiedErrorStatistics, UnifiedErrorType, UnifiedRequestContext,
    UnifiedSystemContext, UnifiedUserContext,
};
pub use network_config::{
    LoadBalanceHealthCheck, NetworkLoadBalanceConfig, NetworkProxyConfig, NetworkQosConfig,
    NetworkRateLimitConfig, NetworkTlsConfig, UnifiedNetworkConfig,
};
pub use retry_config::UnifiedRetryConfig;
pub use service_metadata::{
    CommunicationProtocol, ContactInfo, EndpointType, HealthState, ResourceRequirements,
    ServiceCapability, ServiceEndpoint, ServiceStatus, UniversalServiceMetadata,
    UniversalServiceMetadataBuilder,
};

// Service configuration types - defined inline in this file
// pub use UnifiedServiceConfig; // Already public in this module
// Migration utilities removed - system is mature
pub use timeout_config::UnifiedTimeoutConfig;
// Storage types moved to unified_types - define locally if needed
// pub use crate::universal_storage::{
//     Change, DirectoryEntry, Range, ReplicationResult, ReplicationStatus,
// };

// Re-export unified enums for easy access alongside config types
pub use crate::unified_enums::storage_types::UnifiedStorageType;
pub use crate::unified_enums::{
    UnifiedAccessType, UnifiedAlertSeverity, UnifiedAlertType, UnifiedConnectionStatus,
    UnifiedContentType, UnifiedDataType, UnifiedEventType, UnifiedFileType, UnifiedHealthStatus,
    UnifiedIntegrationType, UnifiedMessageType, UnifiedOperationType, UnifiedProtocolType,
    UnifiedProxyType, UnifiedServiceType, UnifiedTestType, UnifiedTierType,
};

// Additional unified types needed by the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnifiedConfigSource {
    Runtime,
    File(String),
    Environment,
    Database,
    Remote(String),
}

// Network configuration for backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
    pub protocol: String,
    pub timeout_ms: u64,
    pub max_connections: usize,
}
// Helper function for response verification
pub const fn verify_response(response: &serde_json::Value) -> Result<bool, crate::NestGateError> {
    match response.get("success") {
        Some(serde_json::Value::Bool(success)) => Ok(*success),
        _ => Ok(false),
    }
}
// Imports for remaining types that haven't been modularized yet
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
// CLEANED: Removed unused SystemTime import as part of canonical modernization
// CANONICAL MODERNIZATION: Migrated from deprecated UnifiedConfig
use crate::config::canonical_master::NestGateCanonicalConfig as UnifiedConfig;

// ==================== SECTION ====================
// These will be moved to dedicated modules in subsequent phases

// Unified Security Configuration - consolidates all security settings
// **WILL BE MOVED**: To security_config.rs module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedSecurityConfig {
    /// Enable security features
    pub enabled: bool,
    /// Authentication configuration
    pub auth_config: AuthConfig,
    /// Encryption settings
    pub encryption: EncryptionConfig,
    /// Access control settings
    pub access_control: AccessControlConfig,
    /// Security audit settings
    pub audit_config: AuditConfig,
    /// Certificate management
    pub cert_config: CertificateConfig,
    /// Security timeout overrides
    pub security_timeouts: UnifiedTimeoutConfig,
    // Legacy compatibility fields - direct access
    /// Authentication method (legacy compatibility)
    pub auth_method: String,
    /// Require authentication (legacy compatibility)
    pub require_auth: bool,
    /// Maximum failed login attempts (legacy compatibility)
    pub max_failed_attempts: u32,
    /// Enable TLS (legacy compatibility)
    pub enable_tls: bool,
    /// Certificate path (legacy compatibility)
    /// Private key path (legacy compatibility)
    /// Encryption algorithm (legacy compatibility)
    pub encryption_algorithm: String,
    /// Key rotation days (legacy compatibility)
    pub key_rotation_days: u32,
    /// Allowed origins (legacy compatibility)
    pub allowed_origins: Vec<String>,
    /// Allowed IP ranges (legacy compatibility)
    pub allowed_ip_ranges: Vec<String>,
    /// Blocked IP ranges (legacy compatibility)
    pub blocked_ip_ranges: Vec<String>,
    /// Enable RBAC (legacy compatibility)
    pub enable_rbac: bool,
    /// Default role (legacy compatibility)
    pub default_role: Option<String>,
    /// Operation timeout (legacy compatibility)
    pub operation_timeout: u32,
    /// Required capabilities (legacy compatibility)
    pub required_capabilities: Vec<String>,
    /// Minimum consensus (legacy compatibility)
    pub min_consensus: f64,
    /// Timeouts (legacy compatibility)
    pub timeouts: UnifiedTimeoutConfig,
    /// Retry configuration (legacy compatibility)
    pub retry: UnifiedRetryConfig,
}

// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Authentication method (jwt, oauth2, basic)
    pub method: String,
    /// Token expiration time
    pub token_expiry: Duration,
    /// Refresh token expiry
    pub refresh_token_expiry: Duration,
    /// Secret key for signing
    pub secret_key: Option<String>,
    /// Enable multi-factor authentication
    pub mfa_enabled: bool,
    /// Session configuration
    pub session_config: SessionConfig,
}
// Session management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Session timeout
    pub timeout: Duration,
    /// Enable session persistence
    pub persistent: bool,
    /// Session storage backend
    pub storage: String,
    /// Secure cookie settings
    pub secure_cookies: bool,
}
// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Enable encryption
    pub enabled: bool,
    /// Encryption algorithm
    pub algorithm: String,
    /// Key derivation method
    pub key_derivation: String,
    /// Key rotation interval
    pub key_rotation_interval: Duration,
}
// Access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    /// Enable role-based access control
    pub rbac_enabled: bool,
    /// Default user role
    pub default_role: String,
    /// Permission model
    pub permission_model: String,
    /// Access control lists
    pub acls: HashMap<String, Vec<String>>,
}
// Security audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Enable audit logging
    pub enabled: bool,
    /// Audit log retention (days)
    pub retention_days: u32,
    /// Audit log format
    pub format: String,
    /// Log sensitive data
    pub log_sensitive_data: bool,
}
// Certificate management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateConfig {
    /// Auto-generate certificates
    pub auto_generate: bool,
    /// Certificate validity period
    pub validity_days: u32,
    /// Certificate authority settings
    pub ca_config: Option<CaConfig>,
}
// Certificate Authority configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaConfig {
    /// CA certificate path
    /// CA private key path
    /// CA common name
    pub common_name: String,
}
impl Default for UnifiedSecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auth_config: AuthConfig {
                method: "jwt".to_string(),
                token_expiry: Duration::from_secs(3600), // 1 hour
                refresh_token_expiry: Duration::from_secs(86400 * 7), // 7 days
                secret_key: None,
                mfa_enabled: false,
                session_config: SessionConfig {
                    timeout: Duration::from_secs(1800), // 30 minutes
                    persistent: false,
                    storage: "memory".to_string(),
                    secure_cookies: true,
                },
            },
            encryption: EncryptionConfig {
                enabled: true,
                algorithm: "AES-256-GCM".to_string(),
                key_derivation: "PBKDF2".to_string(),
                key_rotation_interval: Duration::from_secs(86400 * 30), // 30 days
            },
            access_control: AccessControlConfig {
                rbac_enabled: true,
                default_role: "user".to_string(),
                permission_model: "rbac".to_string(),
                acls: HashMap::new(),
            },
            audit_config: AuditConfig {
                enabled: true,
                retention_days: 90,
                format: "json".to_string(),
                log_sensitive_data: false,
            },
            cert_config: CertificateConfig {
                auto_generate: true,
                validity_days: 365,
                ca_config: None,
            },
            security_timeouts: UnifiedTimeoutConfig::default(),

            // Legacy compatibility defaults
            auth_method: "jwt".to_string(),
            require_auth: true,
            max_failed_attempts: 3,
            enable_tls: false,
            encryption_algorithm: "aes-256-gcm".to_string(),
            key_rotation_days: 30,
            allowed_origins: vec!["*".to_string()],
            allowed_ip_ranges: vec!["0.0.0.0/0".to_string()],
            blocked_ip_ranges: Vec::new(),
            enable_rbac: true,
            default_role: Some("user".to_string()),
            operation_timeout: 30,
            required_capabilities: Vec::new(),
            min_consensus: 0.8,
            timeouts: UnifiedTimeoutConfig::default(),
            retry: UnifiedRetryConfig::default(),
        }
    }
}

// **THE** Master Unified Configuration - consolidates ALL system configuration
// This is the root configuration structure that ties everything together
// PEDANTIC: Removed duplicate derive macro - conflicts resolved
// ==================== SECTION ====================
//
// **DEPRECATED**: This UnifiedConfig definition is superseded by the canonical
// NestGateCanonicalConfig in crate::config::canonical_master. Use the canonical version instead.
//
// **MIGRATION PATH**:
// - Old: use crate::unified_types::UnifiedConfig
// - New: use crate::config::canonical_master::NestGateCanonicalConfig
//
// ==================== SECTION ====================
// UnifiedConfig has been removed. Use crate::config::canonical_master::NestGateCanonicalConfig instead.
// Deprecated struct completely removed - use canonical_master::NestGateCanonicalConfig

// Placeholder implementations for configurations not yet modularized
// These will be moved to their respective modules

// Service configuration placeholder
#[derive(Debug, Clone, Serialize, Deserialize)] // PEDANTIC: Single derive macro only
pub struct UnifiedServiceConfig {
    pub name: String,
    pub version: String,
    pub description: String,
    pub service_name: String,
    pub service_type: crate::unified_enums::UnifiedServiceType,
    pub enabled: bool,
    pub auto_start: bool,
    pub priority: u8,
    pub max_instances: u32,
    pub health_check_enabled: bool,
    pub capabilities: Vec<String>,
    pub dependencies: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub timeouts: UnifiedTimeoutConfig,
    pub retry: UnifiedRetryConfig,
}
impl Default for UnifiedServiceConfig {
    fn default() -> Self {
        Self {
            // SOVEREIGNTY FIX: Use dynamic service identification
            name: std::env::var("NESTGATE_SERVICE_NAME")
                .unwrap_or_else(|_| format!("nestgate-{}", uuid::Uuid::new_v4().simple())),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "NestGate Unified Service".to_string(),
            service_name: std::env::var("NESTGATE_SERVICE_NAME")
                .unwrap_or_else(|_| format!("nestgate-{}", uuid::Uuid::new_v4().simple())),
            service_type: crate::unified_enums::UnifiedServiceType::Storage,
            enabled: true,
            auto_start: true,
            priority: 5,
            max_instances: 1,
            health_check_enabled: true,
            capabilities: vec!["storage".to_string()],
            dependencies: Vec::new(),
            metadata: HashMap::new(),
            timeouts: UnifiedTimeoutConfig::default(),
            retry: UnifiedRetryConfig::default(),
        }
    }
}

// Monitoring configuration placeholder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMonitoringConfig {
    pub enabled: bool,
    pub metrics_enabled: bool,
    pub tracing_enabled: bool,
    // Legacy compatibility fields
    pub enable_metrics: bool,
    pub enable_tracing: bool,
    pub log_level: String,
}
impl Default for UnifiedMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_enabled: true,
            tracing_enabled: true,
            // Legacy compatibility defaults
            enable_metrics: true,
            enable_tracing: true,
            log_level: "info".to_string(),
        }
    }
}

// Cache configuration placeholder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCacheConfig {
    pub enabled: bool,
    pub max_size: usize,
    pub ttl: Duration,
    // Legacy compatibility fields
    pub ttl_seconds: u64,
    pub hot_tier_size: usize,
    pub warm_tier_size: usize,
    pub cold_tier_unlimited: bool,
    pub cache_dir: String,
    pub policy: String,
    // Additional cache configuration fields
    pub name: String,
    pub eviction_policy: String,
    pub enable_compression: bool,
    pub compression_level: u8,
    pub default_ttl_seconds: u64,
    pub enable_metrics: bool,
    pub metrics_interval_seconds: u64,
    pub enable_persistence: bool,
    pub persistence_interval_seconds: u64,
    pub max_memory_percent: f64,
    pub enable_lru: bool,
    pub concurrent_threads: u32,
}
impl Default for UnifiedCacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_size: 1000,
            ttl: Duration::from_secs(300),
            // Legacy compatibility defaults
            ttl_seconds: 300,
            hot_tier_size: 100 * 1024 * 1024,   // 100MB
            warm_tier_size: 1024 * 1024 * 1024, // 1GB
            cold_tier_unlimited: true,
            cache_dir: "/var/lib/nestgate/cache".to_string(),
            policy: "lru".to_string(),
            // Additional cache configuration defaults
            name: "unified-cache".to_string(),
            eviction_policy: "lru".to_string(),
            enable_compression: false,
            compression_level: 1,
            default_ttl_seconds: 300,
            enable_metrics: true,
            metrics_interval_seconds: 60,
            enable_persistence: false,
            persistence_interval_seconds: 300,
            max_memory_percent: 0.8,
            enable_lru: true,
            concurrent_threads: 4,
        }
    }
}

impl UnifiedCacheConfig {
    /// Development cache configuration with debugging enabled
    #[must_use]
    pub const fn development() -> Self {
        Self {
            enabled: true,
            max_size: 100,
            ttl: Duration::from_secs(60),
            ttl_seconds: 60,
            hot_tier_size: 10 * 1024 * 1024,  // 10MB
            warm_tier_size: 50 * 1024 * 1024, // 50MB
            cold_tier_unlimited: false,
            cache_dir: "/tmp/nestgate-dev-cache".to_string(),
            policy: "lru".to_string(),
            name: "development-cache".to_string(),
            eviction_policy: "lru".to_string(),
            enable_compression: false,
            compression_level: 1,
            default_ttl_seconds: 60,
            enable_metrics: true,
            metrics_interval_seconds: 30,
            enable_persistence: false,
            persistence_interval_seconds: 60,
            max_memory_percent: 0.5,
            enable_lru: true,
            concurrent_threads: 2,
        }
    }

    /// High performance cache configuration for production
    #[must_use]
    pub const fn high_performance() -> Self {
        Self {
            enabled: true,
            max_size: 10000,
            ttl: Duration::from_secs(600),
            ttl_seconds: 600,
            hot_tier_size: 500 * 1024 * 1024,       // 500MB
            warm_tier_size: 2 * 1024 * 1024 * 1024, // 2GB
            cold_tier_unlimited: true,
            cache_dir: "/var/lib/nestgate/cache-performance".to_string(),
            policy: "lfu".to_string(),
            name: "high-performance-cache".to_string(),
            eviction_policy: "lfu".to_string(),
            enable_compression: true,
            compression_level: 3,
            default_ttl_seconds: 600,
            enable_metrics: true,
            metrics_interval_seconds: 10,
            enable_persistence: true,
            persistence_interval_seconds: 120,
            max_memory_percent: 0.9,
            enable_lru: false,
            concurrent_threads: 8,
        }
    }
}

// Storage configuration placeholder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedStorageConfig {
    pub enabled: bool,
    pub backend: String,
    pub pool_name: String,
    pub cache: UnifiedCacheConfig,
}
impl Default for UnifiedStorageConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            backend: "zfs".to_string(),
            pool_name: "nestgate".to_string(),
            cache: UnifiedCacheConfig::default(),
        }
    }
}

// Memory configuration placeholder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMemoryConfig {
    pub enabled: bool,
    pub max_memory: usize,
    pub gc_threshold: f64,
    pub buffer_pools: bool,
}
impl Default for UnifiedMemoryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_memory: 1024 * 1024 * 1024, // 1GB
            gc_threshold: 0.8,
            buffer_pools: true,
        }
    }
}

// Connection pool configuration placeholder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedConnectionPoolConfig {
    pub enabled: bool,
    pub min_connections: usize,
    pub max_connections: usize,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
    pub retry_attempts: u32,
    pub health_check_enabled: bool,
    // Additional connection pool fields
    pub max_idle_time_seconds: u64,
    pub acquire_timeout_seconds: u64,
    pub health_check_interval_seconds: u64,
    pub enable_validation: bool,
    pub retry_delay_seconds: u64,
    pub enable_metrics: bool,
    pub pool_name: String,
}
impl Default for UnifiedConnectionPoolConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_connections: 5,
            max_connections: 100,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
            max_lifetime: Duration::from_secs(3600),
            retry_attempts: 3,
            health_check_enabled: true,
            // Additional connection pool defaults
            max_idle_time_seconds: 600,
            acquire_timeout_seconds: 30,
            health_check_interval_seconds: 300,
            enable_validation: true,
            retry_delay_seconds: 1,
            enable_metrics: true,
            pool_name: "unified-pool".to_string(),
        }
    }
}

// Performance test configuration placeholder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedPerformanceTestConfig {
    pub enabled: bool,
    pub duration_seconds: u64,
    pub concurrent_requests: u32,
    pub test_types: Vec<String>,
    // Legacy compatibility fields
    pub test_name: String,
    pub test_type: String,
    pub concurrent_users: u32,
    pub target_rps: u32,
    pub test_iterations: u32,
    pub baseline_timeout_seconds: u64,
    pub max_timeout_seconds: u64,
    pub percentile_target: f64,
}
impl Default for UnifiedPerformanceTestConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            duration_seconds: 60,
            concurrent_requests: 10,
            test_types: vec!["latency".to_string(), "throughput".to_string()],
            // Legacy compatibility defaults
            test_name: "unified-performance-test".to_string(),
            test_type: "latency".to_string(),
            concurrent_users: 10,
            target_rps: 100,
            test_iterations: 1000,
            baseline_timeout_seconds: 5,
            max_timeout_seconds: 30,
            percentile_target: 95.0,
        }
    }
}

// Note: UnifiedServiceConfig, etc. are automatically exported
// because they're defined as pub structs in this module

// Installer configuration placeholder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedInstallerConfig {
    pub enabled: bool,
    pub install_mode: String,
    pub target_directory: String,
    pub install_dir: String, // Alias for target_directory for compatibility
    pub backup_enabled: bool,
    pub download_timeout: Duration,
    pub verification_enabled: bool,
    pub rollback_enabled: bool,
    pub install_dependencies: bool,
    pub enable_systemd: bool,
    pub components: HashMap<String, bool>,
}
impl Default for UnifiedInstallerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            install_mode: "standard".to_string(),
            target_directory: "/opt/nestgate".to_string(),
            install_dir: "/opt/nestgate".to_string(), // Same as target_directory for compatibility
            backup_enabled: true,
            download_timeout: Duration::from_secs(300), // 5 minutes
            verification_enabled: true,
            rollback_enabled: true,
            install_dependencies: true,
            enable_systemd: true,
            components: {
                let mut components = HashMap::new();
                components.insert("add_to_path".to_string(), true);
                components.insert("create_shortcuts".to_string(), true);
                components.insert("install_service".to_string(), true);
                components
            },
        }
    }
}

impl UnifiedConfig {
    // PEDANTIC: Implementation methods
    #[must_use]
    pub const fn new() -> Self {
        Self::default()
    }
}
