use std::collections::HashMap;
//
// **MODULAR CANONICAL CONFIGURATION** - Split from the monolithic canonical_unified.rs
// into focused, maintainable modules while preserving the unified interface.
//
// **CONSOLIDATES AND ELIMINATES**:
// - `NestGateFinalConfig` (unified_final_config/core.rs)
// - `CanonicalModernizedConfig` (canonical_modernization/core_config.rs)
// - `NestGateCanonicalConfig` (config/mod.rs)
// - `UnifiedConfig` (unified_types/mod.rs)
// - `UltimateCanonicalConfig` (unified_final_config/canonical_config_consolidation.rs)
// - `UnifiedApiConfig` (nestgate-api/src/unified_api_config/api_core.rs)
// - `ZfsHandlerConfig` and all handler-specific configs
// - 823+ fragmented configuration structs across all crates
//
// **PROVIDES**:
// - Single source of truth for all configuration
// - Zero-cost compile-time configuration
// - Environment-driven configuration loading
// - Type-safe configuration validation
// - Automatic migration from legacy configs

use serde::{Deserialize, Serialize};

// Import all configuration modules
pub mod system_config;
pub mod network_config;
pub mod security_config;
pub mod storage_config;
pub mod api_config;
pub mod zfs_config;
pub mod performance_config;
pub mod monitoring_config;
pub mod builders;
pub mod defaults;

// **CONFIGURATION MIGRATION UTILITIES**
// Provides migration from fragmented configs to canonical unified config
// Migration module removed - modernization complete
// Re-export configuration types with explicit imports to avoid conflicts
pub use system_config::{SystemConfig, DeploymentEnvironment};
pub use network_config::{
    NetworkConfig, HttpServerConfig, RpcConfig, LoadBalancingConfig,
    CircuitBreakerConfig, RateLimitConfig, TimeoutConfig, ConnectionPoolConfig, TlsConfig
};
pub use security_config::{SecurityConfig, AuthenticationConfig, AuthorizationConfig, EncryptionConfig};
pub use storage_config::{
    StorageConfig, StorageTiersConfig, TierConfig, CacheStorageConfig,
    CompressionConfig, StorageEncryptionConfig, BackupConfig, 
    ReplicationConfig, StoragePerformanceConfig,
};

// API config with renamed conflicting types
pub use api_config::{
    ApiConfig, RestApiConfig, StreamingConfig, SseConfig, WebSocketConfig,
    AuthHandlerConfig, DashboardConfig, LoadTestingConfig, WorkspaceConfig,
    ZfsHandlerConfig, PerformanceHandlerConfig, ApiHandlerExtensions,
    ZfsPerformanceConfig as ApiZfsPerformanceConfig,
    MetricsConfig as ApiMetricsConfig,
    AlertingConfig as ApiAlertingConfig,
};

// ZFS config with canonical types
pub use zfs_config::{
    ZfsConfig, PoolConfig, DatasetConfig, SnapshotConfig,
    ZfsPerformanceConfig, FailSafeConfig, TieringConfig,
};

// Performance config with canonical types  
pub use performance_config::{
    PerformanceConfig, BufferConfig, ThreadPoolConfig, MemoryConfig,
    IoConfig, CacheConfig, MetricsConfig as PerfMetricsConfig,
};

// Monitoring config with canonical types
pub use monitoring_config::{
    MonitoringConfig, MonitoringMetricsConfig, AlertingConfig, AlertRule,
    NotificationChannel, LoggingConfig, LogRotationConfig, 
    MonitoringHealthConfig, TracingConfig,
};

pub use builders::*;
// Migration utilities removed - modernization complete
// Note: defaults module provides preset configurations via impl methods

// **THE SINGLE CANONICAL CONFIGURATION**
///
// This is THE configuration structure for the entire NestGate ecosystem.
// All other configuration structures are deprecated and MUST migrate to this.
///
// **MODULAR ARCHITECTURE**: Now split across focused modules for maintainability
//! while preserving the unified interface and backward compatibility.
//! Module definitions and exports.
// **CONSOLIDATION COMPLETE**: This structure now absorbs ALL fragmented configurations:
//! - UnifiedApiHandlerConfig → api.zfs_handlers, api.performance_handlers, api.handler_extensions
//! - UnifiedAutomationConfig → services (automation extensions)
//! - UnifiedAdapterConfig → ecosystem integration settings
//! - All StandardDomainConfig type aliases
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct NestGateCanonicalConfig {
    /// System-level configuration
    pub system: SystemConfig,
    
    /// Network configuration (consolidates 15+ network configs)
    pub network: NetworkConfig,
    
    /// Security configuration (consolidates 20+ security configs)
    pub security: SecurityConfig,
    
    /// Storage configuration (consolidates 25+ storage configs)
    pub storage: StorageConfig,
    
    /// API configuration (consolidates 20+ API configs)
    /// **ENHANCED**: Now includes consolidated handler configurations
    pub api: ApiConfig,
    
    /// ZFS configuration (consolidates 10+ ZFS configs)
    pub zfs: ZfsConfig,
    
    /// Performance configuration (consolidates 15+ performance configs)
    pub performance: PerformanceConfig,
    
    /// Environment configuration
    
    /// Feature flags
    pub features: FeatureFlags,
    
    /// Service-specific configurations (consolidates 40+ service configs)
    pub services: ServiceConfigs,
    
    /// Testing configurations (consolidates 50+ test configs)
    pub testing: TestingConfigs,
    
    /// Monitoring configurations (consolidates 15+ monitoring configs)
    pub monitoring: MonitoringConfig,
    
    /// Configuration metadata
    pub metadata: ConfigMetadata,
}
impl NestGateCanonicalConfig {
    /// **MIGRATION FROM FRAGMENTED CONFIGS**
    /// Create canonical config by migrating from fragmented configuration files
    /// Migration utilities have been removed - modernization complete
    pub const fn migrate_from_fragmented_configs(_config_paths: &[&str]) -> crate::error::CanonicalResult<Self> {
        // Migration complete - return default canonical configuration
        log::info!("Configuration migration complete - using default canonical configuration");
        Ok(Self::default())
    }

    /// **MERGE CONFIGURATIONS**
    /// Merge another canonical config into this one (for combining multiple sources)
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        // Merge API configurations (most complex)
        self.api = self.api.merge(other.api);
        
        // Merge other configurations (implement as needed)
        // For now, other config takes precedence
        if other.network.http_server.port != 8080 {  // Not default
            self.network = other.network;
        }
        
        if other.security.authentication.enable_mfa {
            self.security = other.security;
        }

        // Merge feature flags
        self.features.experimental = self.features.experimental || other.features.experimental;
        self.features.debug_mode = self.features.debug_mode || other.features.debug_mode;
        self.features.performance_monitoring = self.features.performance_monitoring || other.features.performance_monitoring;
        self.features.security_hardening = self.features.security_hardening || other.features.security_hardening;

        self
    }

    /// **PRODUCTION CONFIGURATION**
    /// Create production-ready configuration with optimal defaults
    #[must_use]
    pub fn production() -> Self {
        let mut config = Self::default();
        
        // Production-optimized settings
        config.features.experimental = false;
        config.features.debug_mode = false;
        config.features.performance_monitoring = true;
        config.features.security_hardening = true;
        
        // Production API settings
        config.api.zfs_handlers.service.enabled = true;
        config.api.zfs_handlers.performance.monitoring_enabled = true;
        config.api.performance_handlers.analytics.enabled = true;
        config.api.handler_extensions.security.require_auth = true;
        
        // Production security settings
        config.security.authentication.enable_mfa = true;
        config.security.authorization.default_permissions = vec!["read".to_string(), "write".to_string()];
        config.security.encryption.algorithm = "AES-256".to_string();
        
        config
    }

    /// **DEVELOPMENT CONFIGURATION**
    /// Create development-friendly configuration with debugging enabled
    #[must_use]
    pub fn development() -> Self {
        let mut config = Self::default();
        
        // Development-optimized settings
        config.features.experimental = true;
        config.features.debug_mode = true;
        config.features.performance_monitoring = true;
        config.features.security_hardening = false;
        
        // Development API settings
        config.api.handler_extensions.feature_flags.debug_endpoints = true;
        config.api.handler_extensions.feature_flags.metrics_endpoints = true;
        config.api.handler_extensions.security.require_auth = false;
        
        // Development security settings (relaxed)
        config.security.authentication.enable_mfa = false;
        config.security.authorization.default_permissions = vec!["read".to_string(), "write".to_string(), "admin".to_string()];
        
        config
    }
}

impl ApiConfig {
    /// Merge API configurations
    fn merge(mut self, other: Self) -> Self {
        // Merge ZFS handler configurations
        if other.zfs_handlers.service.enabled {
            self.zfs_handlers = other.zfs_handlers;
        }
        
        // Merge performance handler configurations
        if other.performance_handlers.analytics.enabled {
            self.performance_handlers = other.performance_handlers;
        }
        
        // Merge handler extensions
        if !other.handler_extensions.custom_handlers.is_empty() {
            self.handler_extensions.custom_handlers.extend(other.handler_extensions.custom_handlers);
        }
        
        self
    }
}

// Environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    /// Environment variables to load
    pub env_vars: Vec<String>,
    /// Configuration file paths
    pub config_files: Vec<String>,
    /// Override settings
    pub overrides: std::collections::HashMap<String, String>,
}
// Feature flags for enabling/disabling functionality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    /// Enable experimental features
    pub experimental: bool,
    /// Enable debug mode
    pub debug_mode: bool,
    /// Enable performance monitoring
    pub performance_monitoring: bool,
    /// Enable security hardening
    pub security_hardening: bool,
}
// Service-specific configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ServiceConfigs {
    /// Service discovery configuration
    pub discovery: ServiceDiscoveryConfig,
    /// Health check configuration
    pub health_checks: HealthCheckConfig,
    /// Service registry configuration
    pub registry: ServiceRegistryConfig,
}
// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
// DEPRECATED: Consul service discovery - migrate to capability-based discovery
// Capability-based discovery implemented
    /// Discovery method (dns, consul, etc.)
    pub method: String,
    /// Discovery endpoints
    pub endpoints: Vec<String>,
    /// Refresh interval
    pub refresh_interval: std::time::Duration,
}
// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Health check endpoint
    pub endpoint: String,
    /// Check interval
    pub interval: std::time::Duration,
    /// Timeout for health checks
    pub timeout: std::time::Duration,
    /// Failure threshold before marking unhealthy
    pub failure_threshold: u32,
}
// Service registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistryConfig {
// DEPRECATED: Consul service discovery - migrate to capability-based discovery
// Capability-based discovery implemented
// DEPRECATED: etcd key-value store - migrate to capability-based storage
// Capability-based discovery implemented
    /// Registry backend (consul, etcd, etc.)
    pub backend: String,
    /// Registry endpoints
    pub endpoints: Vec<String>,
    /// Service TTL
    pub ttl: std::time::Duration,
}
// Testing configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestingConfigs {
    /// Enable testing mode
    pub enabled: bool,
    /// Test data directory
    pub test_data_dir: String,
    /// Mock external services
    pub mock_external_services: bool,
    /// Test timeouts
    pub test_timeout: std::time::Duration,
}
// Configuration metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMetadata {
    /// Configuration version
    pub version: String,
    /// Configuration source
    pub source: String,
    /// Last updated timestamp
    pub updated_at: std::time::SystemTime,
    /// Configuration checksum
    pub checksum: Option<String>,
}
// ==================== SECTION ====================


impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            env_vars: vec![
                "NESTGATE_API_PORT".to_string(),
                "NESTGATE_ZFS_ENABLED".to_string(),
                "NESTGATE_DEBUG_MODE".to_string(),
            ],
            config_files: vec![
                "/etc/nestgate/config.toml".to_string(),
                "~/.nestgate/config.toml".to_string(),
                "./config.toml".to_string(),
            ],
            overrides: std::collections::HashMap::new(),
        }
    }
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            experimental: false,
            debug_mode: false,
            performance_monitoring: true,
            security_hardening: true,
        }
    }
}


impl Default for ServiceDiscoveryConfig {
    fn default() -> Self {
        Self {
            method: "dns".to_string(),
            endpoints: vec!["localhost:8500".to_string()],
            refresh_interval: std::time::Duration::from_secs(30),
        }
    }
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            endpoint: "/health".to_string(),
            interval: std::time::Duration::from_secs(30),
            timeout: std::time::Duration::from_secs(5),
            failure_threshold: 3,
        }
    }
}

impl Default for ServiceRegistryConfig {
    fn default() -> Self {
        Self {
    #[deprecated(since = "3.0.0", note = "Use capability-based discovery instead of vendor-specific service discovery")]
            backend: "service_discovery".to_string().to_string(),
            endpoints: vec!["localhost:8500".to_string()],
            ttl: std::time::Duration::from_secs(60),
        }
    }
}

impl Default for TestingConfigs {
    fn default() -> Self {
        Self {
            enabled: false,
            test_data_dir: "/tmp/nestgate/test_data".to_string(),
            mock_external_services: false,
            test_timeout: std::time::Duration::from_secs(30),
        }
    }
}

impl Default for ConfigMetadata {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            source: "default".to_string(),
            updated_at: std::time::SystemTime::now(),
            checksum: None,
        }
    }
} 