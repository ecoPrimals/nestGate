//
// This module provides a unified configuration pattern for all API handlers,
// eliminating fragmented handler-specific configurations and establishing
// a consistent interface for handler setup and management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use nestgate_core::canonical_modernization::canonical_constants::{
    DEFAULT_API_PORT, REQUEST_TIMEOUT_SECS, DEFAULT_TIMEOUT_SECS
};

/// **UNIFIED HANDLER CONFIGURATION**
/// 
/// Generic configuration pattern that all handlers should adopt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedHandlerConfig<T: HandlerDomain> {
    /// Handler identification
    pub handler_id: String,
    /// Handler version
    pub version: String,
    /// Domain-specific configuration
    pub domain: T,
    /// Common handler settings
    pub common: CommonHandlerSettings,
    /// Environment variable overrides
    pub environment: EnvironmentOverrides,
    /// Custom properties for extensibility
    pub custom_properties: HashMap<String, String>,
}

/// **COMMON HANDLER SETTINGS**
/// 
/// Settings that apply to all handlers regardless of domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonHandlerSettings {
    /// Request timeout in seconds
    pub request_timeout_secs: u64,
    /// Maximum concurrent requests
    pub max_concurrent_requests: usize,
    /// Enable request logging
    pub enable_logging: bool,
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Rate limiting configuration
    pub rate_limit: Option<RateLimitConfig>,
    /// Health check configuration
    pub health_check: HealthCheckConfig,
}

/// **RATE LIMITING CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum requests per minute
    pub requests_per_minute: u32,
    /// Burst allowance
    pub burst_size: u32,
}

/// **HEALTH CHECK CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Health check interval in seconds
    pub interval_secs: u64,
    /// Health check timeout in seconds
    pub timeout_secs: u64,
    /// Enable deep health checks
    pub enable_deep_checks: bool,
}

/// **ENVIRONMENT OVERRIDES**
/// 
/// Allow runtime configuration through environment variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentOverrides {
    /// Override request timeout from environment
    pub timeout_override: Option<String>,
    /// Override port from environment
    pub port_override: Option<String>,
    /// Override log level from environment
    pub log_level_override: Option<String>,
}

/// **HANDLER DOMAIN TRAIT**
/// 
/// Marker trait for domain-specific configuration types
pub trait HandlerDomain: Clone + Send + Sync + Serialize + for<'de> Deserialize<'de> + 'static {}

/// **ZFS HANDLER DOMAIN CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsHandlerDomain {
    /// ZFS backend type
    pub backend_type: ZfsBackendType,
    /// Pool operation settings
    pub pool_settings: ZfsPoolSettings,
    /// Dataset operation settings
    pub dataset_settings: ZfsDatasetSettings,
    /// Snapshot settings
    pub snapshot_settings: ZfsSnapshotSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsBackendType {
    Native,
    Remote { endpoint: String },
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPoolSettings {
    pub max_pools: usize,
    pub default_compression: String,
    pub enable_deduplication: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsDatasetSettings {
    pub max_datasets_per_pool: usize,
    pub default_recordsize: String,
    pub enable_snapshots: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsSnapshotSettings {
    pub max_snapshots_per_dataset: usize,
    pub retention_days: u32,
    pub enable_auto_snapshot: bool,
}

impl HandlerDomain for ZfsHandlerDomain {}

/// **DASHBOARD HANDLER DOMAIN CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardHandlerDomain {
    /// Enable real-time updates
    pub enable_real_time: bool,
    /// Update interval for real-time data
    pub update_interval_secs: u64,
    /// Maximum history points to keep
    pub max_history_points: usize,
    /// Enable predictive analytics
    pub enable_predictions: bool,
    /// Alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
}

impl HandlerDomain for DashboardHandlerDomain {}

/// **LOAD TESTING HANDLER DOMAIN CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestingHandlerDomain {
    /// Maximum concurrent test scenarios
    pub max_concurrent_tests: usize,
    /// Default test duration in seconds
    pub default_test_duration_secs: u64,
    /// Enable test result persistence
    pub persist_results: bool,
    /// Test result retention days
    pub result_retention_days: u32,
}

impl HandlerDomain for LoadTestingHandlerDomain {}

/// **DEFAULT IMPLEMENTATIONS**

impl Default for CommonHandlerSettings {
    fn default() -> Self {
        Self {
            request_timeout_secs: REQUEST_TIMEOUT_SECS,
            max_concurrent_requests: 1000,
            enable_logging: true,
            enable_metrics: true,
            rate_limit: Some(RateLimitConfig::default()),
            health_check: HealthCheckConfig::default(),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 1000,
            burst_size: 100,
        }
    }
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            interval_secs: 30,
            timeout_secs: 5,
            enable_deep_checks: false,
        }
    }
}

impl Default for EnvironmentOverrides {
    fn default() -> Self {
        Self {
            timeout_override: std::env::var("HANDLER_TIMEOUT").ok(),
            port_override: std::env::var("HANDLER_PORT").ok(),
            log_level_override: std::env::var("LOG_LEVEL").ok(),
        }
    }
}

impl Default for ZfsHandlerDomain {
    fn default() -> Self {
        Self {
            backend_type: ZfsBackendType::Auto,
            pool_settings: ZfsPoolSettings::default(),
            dataset_settings: ZfsDatasetSettings::default(),
            snapshot_settings: ZfsSnapshotSettings::default(),
        }
    }
}

impl Default for ZfsPoolSettings {
    fn default() -> Self {
        Self {
            max_pools: 100,
            default_compression: "lz4".to_string(),
            enable_deduplication: false,
        }
    }
}

impl Default for ZfsDatasetSettings {
    fn default() -> Self {
        Self {
            max_datasets_per_pool: 1000,
            default_recordsize: "128K".to_string(),
            enable_snapshots: true,
        }
    }
}

impl Default for ZfsSnapshotSettings {
    fn default() -> Self {
        Self {
            max_snapshots_per_dataset: 100,
            retention_days: 30,
            enable_auto_snapshot: false,
        }
    }
}

impl Default for DashboardHandlerDomain {
    fn default() -> Self {
        let mut alert_thresholds = HashMap::new();
        alert_thresholds.insert("cpu_usage".to_string(), 80.0);
        alert_thresholds.insert("memory_usage".to_string(), 85.0);
        alert_thresholds.insert("disk_usage".to_string(), 90.0);
        alert_thresholds.insert("latency_ms".to_string(), 1000.0);
        alert_thresholds.insert("error_rate".to_string(), 5.0);

        Self {
            enable_real_time: true,
            update_interval_secs: 1,
            max_history_points: 1000,
            enable_predictions: true,
            alert_thresholds,
        }
    }
}

impl Default for LoadTestingHandlerDomain {
    fn default() -> Self {
        Self {
            max_concurrent_tests: 10,
            default_test_duration_secs: 300,
            persist_results: true,
            result_retention_days: 30,
        }
    }
}

/// **CONFIGURATION BUILDER PATTERN**

pub struct UnifiedHandlerConfigBuilder<T: HandlerDomain> {
    config: UnifiedHandlerConfig<T>,
}

impl<T: HandlerDomain + Default> UnifiedHandlerConfigBuilder<T> {
    pub fn new(handler_id: impl Into<String>) -> Self {
        Self {
            config: UnifiedHandlerConfig {
                handler_id: handler_id.into(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                domain: T::default(),
                common: CommonHandlerSettings::default(),
                environment: EnvironmentOverrides::default(),
                custom_properties: HashMap::new(),
            },
        }
    }

    pub fn with_domain(mut self, domain: T) -> Self {
        self.config.domain = domain;
        self
    }

    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.config.common.request_timeout_secs = timeout_secs;
        self
    }

    pub fn with_max_concurrent_requests(mut self, max_requests: usize) -> Self {
        self.config.common.max_concurrent_requests = max_requests;
        self
    }

    pub fn with_rate_limit(mut self, requests_per_minute: u32, burst_size: u32) -> Self {
        self.config.common.rate_limit = Some(RateLimitConfig {
            requests_per_minute,
            burst_size,
        });
        self
    }

    pub fn disable_rate_limit(mut self) -> Self {
        self.config.common.rate_limit = None;
        self
    }

    pub fn with_custom_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config.custom_properties.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> UnifiedHandlerConfig<T> {
        self.config
    }
}

/// **TYPE ALIASES FOR COMMON HANDLER CONFIGURATIONS**

/// ZFS handler configuration
pub type ZfsHandlerConfig = UnifiedHandlerConfig<ZfsHandlerDomain>;

/// Dashboard handler configuration
pub type DashboardHandlerConfig = UnifiedHandlerConfig<DashboardHandlerDomain>;

/// Load testing handler configuration
pub type LoadTestingHandlerConfig = UnifiedHandlerConfig<LoadTestingHandlerDomain>;

/// **CONVENIENCE CONSTRUCTORS**

impl ZfsHandlerConfig {
    pub fn new() -> Self {
        UnifiedHandlerConfigBuilder::new("zfs-handler").build()
    }

    pub fn with_native_backend() -> Self {
        let mut domain = ZfsHandlerDomain::default();
        domain.backend_type = ZfsBackendType::Native;
        UnifiedHandlerConfigBuilder::new("zfs-handler")
            .with_domain(domain)
            .build()
    }

    pub fn with_remote_backend(endpoint: impl Into<String>) -> Self {
        let mut domain = ZfsHandlerDomain::default();
        domain.backend_type = ZfsBackendType::Remote { endpoint: endpoint.into() };
        UnifiedHandlerConfigBuilder::new("zfs-handler")
            .with_domain(domain)
            .build()
    }
}

impl DashboardHandlerConfig {
    pub fn new() -> Self {
        UnifiedHandlerConfigBuilder::new("dashboard-handler").build()
    }

    pub fn with_real_time_disabled() -> Self {
        let mut domain = DashboardHandlerDomain::default();
        domain.enable_real_time = false;
        UnifiedHandlerConfigBuilder::new("dashboard-handler")
            .with_domain(domain)
            .build()
    }
}

impl LoadTestingHandlerConfig {
    pub fn new() -> Self {
        UnifiedHandlerConfigBuilder::new("load-testing-handler").build()
    }

    pub fn with_high_concurrency() -> Self {
        let mut domain = LoadTestingHandlerDomain::default();
        domain.max_concurrent_tests = 50;
        UnifiedHandlerConfigBuilder::new("load-testing-handler")
            .with_domain(domain)
            .with_max_concurrent_requests(5000)
            .build()
    }
} 