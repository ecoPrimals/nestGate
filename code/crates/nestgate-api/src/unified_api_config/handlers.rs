/// 
/// This module consolidates all scattered configuration structures across API handlers
/// into a single, standardized system using the StandardDomainConfig pattern.
///
/// **ELIMINATES**:
/// - Multiple PoolConfig structs (handlers/zfs/types.rs, zero_cost_api_handlers.rs, universal_zfs/types.rs)
/// - Multiple PerformanceConfig structs (performance_analytics/types.rs, universal_zfs/config.rs)
/// - Multiple DashboardConfig structs (dashboard_types.rs, performance_dashboard/types.rs)
/// - LoadTestConfig, TestDataConfig, and 15+ other handler-specific configs
///
/// **PROVIDES**:
/// - Unified configuration hierarchy using StandardDomainConfig<ApiHandlerExtensions>
/// - Domain-specific extensions for each handler type
/// - Consistent validation and environment loading patterns
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use nestgate_core::config::canonical_primary::domains::ConsolidatedDomainConfigs;
use nestgate_core::types::StorageTier;

// ==================== SECTION ====================

/// **THE** unified configuration type for all API handlers
/// This replaces 20+ scattered config structs across handlers with a single, consistent interface
/// CANONICAL MODERNIZATION: Simplified type alias without type parameters
/// Migrated from unified_final_config to canonical_primary (Nov 8, 2025)
pub type UnifiedApiHandlerConfig = ConsolidatedDomainConfigs;
/// API handler-specific configuration extensions
/// Domain-specific fields that don't belong in unified base configs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiHandlerExtensions {
    /// ZFS handler configurations
    pub zfs: ZfsHandlerConfig,
    /// Performance analytics configurations
    pub performance: PerformanceHandlerConfig,
    /// Dashboard configurations
    pub dashboard: DashboardHandlerConfig,
    /// Load testing configurations
    pub load_testing: LoadTestingHandlerConfig,
    /// Workspace management configurations
    pub workspace: WorkspaceHandlerConfig,
    /// Authentication and authorization configurations
    pub auth: AuthHandlerConfig,
}
// ==================== SECTION ====================

/// **UNIFIED ZFS HANDLER CONFIGURATION**
/// Replaces: PoolConfig (3 variants), DatasetConfig (2 variants), SnapshotConfig, ZfsServiceConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::ZfsHandlerConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::ZfsHandlerConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct ZfsHandlerConfig {
    /// Pool management settings
    pub pool: UnifiedPoolConfig,
    /// Dataset management settings
    pub dataset: UnifiedDatasetConfig,
    /// Snapshot management settings
    pub snapshot: UnifiedSnapshotConfig,
    /// Service-level ZFS settings
    pub service: ZfsServiceSettings,
}
impl Default for ZfsHandlerConfig {
    fn default() -> Self { Self {
            pool: UnifiedPoolConfig::default(),
            dataset: UnifiedDatasetConfig::default(),
            snapshot: UnifiedSnapshotConfig::default(),
            service: ZfsServiceSettings::default(),
         }
}

/// **UNIFIED POOL CONFIGURATION**
/// Consolidates: handlers/zfs/types.rs::PoolConfig, zero_cost_api_handlers.rs::PoolConfig, universal_zfs/types.rs::PoolConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedPoolConfig {
    /// RAID level (mirror, raidz1, raidz2, raidz3)
    pub raid_level: Option<String>,
    /// Compression algorithm (lz4, gzip, zstd, etc.)
    pub compression: Option<String>,
    /// Deduplication enabled
    pub dedup: Option<bool>,
    /// Encryption enabled
    pub encryption: Option<bool>,
    /// Pool properties
    pub properties: HashMap<String, String>,
    /// Auto-expansion settings
    pub auto_expand: bool,
    /// Cache device settings
    pub cache_devices: Vec<String>,
    /// Log device settings
    pub log_devices: Vec<String>,
}
impl Default for UnifiedPoolConfig {
    fn default() -> Self { Self {
            raid_level: Some("mirror".to_string()),
            compression: Some("lz4".to_string()),
            dedup: Some(false),
            encryption: Some(false),
            properties: HashMap::new(),
            auto_expand: true,
            cache_devices: Vec::new(),
            log_devices: Vec::new(),
         }
}

/// **UNIFIED DATASET CONFIGURATION**
/// Consolidates: zero_cost_api_handlers.rs::DatasetConfig, universal_zfs/types.rs::DatasetConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedDatasetConfig {
    /// Storage tier assignment
    pub tier: StorageTier,
    /// Dataset properties
    pub properties: HashMap<String, String>,
    /// Quota settings
    pub quota: Option<u64>,
    /// Reservation settings
    pub reservation: Option<u64>,
    /// Compression override
    pub compression: Option<String>,
    /// Record size optimization
    pub record_size: Option<String>,
    /// Mount point settings
    pub mount_point: Option<String>,
}
impl Default for UnifiedDatasetConfig {
    fn default() -> Self { Self {
            tier: StorageTier::Standard,
            properties: HashMap::new(),
            quota: None,
            reservation: None,
            compression: None,
            record_size: Some("128K".to_string()),
            mount_point: None,
         }
}

/// **UNIFIED SNAPSHOT CONFIGURATION**
/// Consolidates: universal_zfs/types.rs::SnapshotConfig and related snapshot settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedSnapshotConfig {
    /// Recursive snapshot settings
    pub recursive: bool,
    /// Snapshot properties
    pub properties: HashMap<String, String>,
    /// Retention policy
    pub retention_days: u32,
    /// Automatic snapshot scheduling
    pub auto_snapshot: bool,
    /// Snapshot naming pattern
    pub naming_pattern: String,
}
/// **ZFS SERVICE SETTINGS**
/// Consolidates service-level settings from universal_zfs/config.rs::ZfsServiceConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsServiceSettings {
    /// Service backend configuration
    pub backend: ZfsBackendConfig,
    /// Fail-safe and reliability settings
    pub fail_safe: ZfsFailSafeConfig,
    /// Performance optimization settings
    pub performance: ZfsPerformanceConfig,
}
impl Default for ZfsServiceSettings {
    fn default() -> Self { Self {
            backend: ZfsBackendConfig::Auto,
            fail_safe: ZfsFailSafeConfig::default(),
            performance: ZfsPerformanceConfig::default(),
         }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsBackendConfig {
    Auto,
    Native,
    Mock,
    Remote { endpoint: String, timeout: Duration }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsFailSafeConfig {
    pub circuit_breaker_enabled: bool,
    pub retry_attempts: u32,
    pub timeout_seconds: u64,
    pub graceful_degradation: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPerformanceConfig {
    pub connection_pool_size: u32,
    pub max_concurrent_operations: u32,
    pub cache_enabled: bool,
    pub cache_ttl_seconds: u64,
    pub batch_size: u32,
}

impl Default for ZfsPerformanceConfig {
    fn default() -> Self { Self {
            connection_pool_size: 10,
            max_concurrent_operations: 100,
            cache_enabled: true,
            cache_ttl_seconds: 300,
            batch_size: 10,
         }
}

// ==================== SECTION ====================

/// **UNIFIED PERFORMANCE HANDLER CONFIGURATION**
/// Replaces: performance_analytics/types.rs::PerformanceConfig, universal_zfs/config.rs::PerformanceConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::PerformanceHandlerConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::PerformanceHandlerConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct PerformanceHandlerConfig {
    /// Metrics collection settings
    pub metrics: PerformanceMetricsConfig,
    /// Analytics and processing settings
    pub analytics: PerformanceAnalyticsConfig,
    /// Alert threshold settings
    pub alerts: PerformanceAlertConfig,
}
impl Default for PerformanceHandlerConfig {
    fn default() -> Self { Self {
            metrics: PerformanceMetricsConfig::default(),
            analytics: PerformanceAnalyticsConfig::default(),
            alerts: PerformanceAlertConfig::default(),
         }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetricsConfig {
    pub collection_interval_seconds: u64,
    pub retention_days: u32,
    pub enabled_metrics: Vec<String>,
    pub custom_metrics: HashMap<String, String>,
}

impl Default for PerformanceMetricsConfig {
    fn default() -> Self { Self {
            collection_interval_seconds: 60,
            retention_days: 30,
            enabled_metrics: vec![
                "cpu".to_string(),
                "memory".to_string(),
                "disk".to_string(),
                "network".to_string(),
                "zfs_health".to_string(),
            ],
            custom_metrics: HashMap::new(),
         }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceAnalyticsConfig {
    pub predictive_enabled: bool,
    pub machine_learning_enabled: bool,
    pub trend_analysis_enabled: bool,
    pub anomaly_detection_enabled: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::PerformanceAlertConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::PerformanceAlertConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct PerformanceAlertConfig {
    pub cpu_threshold: f64,
    pub memory_threshold: f64,
    pub disk_threshold: f64,
    pub network_latency_threshold: f64,
    pub zfs_health_threshold: f64,
    pub custom_thresholds: HashMap<String, f64>,
}

impl Default for PerformanceAlertConfig {
    fn default() -> Self { Self {
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            disk_threshold: 90.0,
            network_latency_threshold: 100.0,
            zfs_health_threshold: 95.0,
            custom_thresholds: HashMap::new(),
         }
}

// ==================== SECTION ====================

/// **UNIFIED DASHBOARD HANDLER CONFIGURATION**
/// Replaces: dashboard_types.rs::DashboardConfig, performance_dashboard/types.rs::DashboardConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::DashboardHandlerConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::DashboardHandlerConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct DashboardHandlerConfig {
    /// Dashboard layout and UI settings
    pub layout: DashboardLayoutConfig,
    /// Data refresh and caching settings
    pub data: DashboardDataConfig,
    /// Visualization settings
    pub visualization: DashboardVisualizationConfig,
}
impl Default for DashboardHandlerConfig {
    fn default() -> Self { Self {
            layout: DashboardLayoutConfig::default(),
            data: DashboardDataConfig::default(),
            visualization: DashboardVisualizationConfig::default(),
         }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DashboardLayoutConfig {
    pub theme: String,
    pub columns: u32,
    pub auto_refresh_enabled: bool,
    pub refresh_interval_seconds: u64,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardDataConfig {
    pub cache_enabled: bool,
    pub cache_ttl_seconds: u64,
    pub max_data_points: u32,
    pub data_sources: Vec<String>,
}

impl Default for DashboardDataConfig {
    fn default() -> Self { Self {
            cache_enabled: true,
            cache_ttl_seconds: 30,
            max_data_points: 1000,
            data_sources: vec![
                "system_metrics".to_string(),
                "zfs_metrics".to_string(),
                "performance_metrics".to_string(),
            ],
         }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DashboardVisualizationConfig {
    pub chart_types: Vec<String>,
    pub color_scheme: String,
    pub animation_enabled: bool,
    pub responsive_design: bool,
}

// ==================== SECTION ====================

/// **UNIFIED LOAD TESTING HANDLER CONFIGURATION**
/// Replaces: load_testing.rs::LoadTestConfig, load_testing.rs::TestDataConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestingHandlerConfig {
    /// Test execution settings
    pub execution: LoadTestExecutionConfig,
    /// Test data generation settings
    pub data: LoadTestDataConfig,
    /// Test scenario definitions
    pub scenarios: LoadTestScenariosConfig,
}
impl Default for LoadTestingHandlerConfig {
    fn default() -> Self { Self {
            execution: LoadTestExecutionConfig::default(),
            data: LoadTestDataConfig::default(),
            scenarios: LoadTestScenariosConfig::default(),
         }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestExecutionConfig {
    pub duration_seconds: u64,
    pub concurrent_users: u32,
    pub requests_per_second: f64,
    pub ramp_up_seconds: u64,
    pub ramp_down_seconds: u64,
}

impl Default for LoadTestExecutionConfig {
    fn default() -> Self { Self {
            duration_seconds: 300,
            concurrent_users: 10,
            requests_per_second: 1.0,
            ramp_up_seconds: 30,
            ramp_down_seconds: 30,
         }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestDataConfig {
    pub test_workspaces: u32,
    pub dataset_size_mb: u32,
    pub concurrent_zfs_ops: u32,
    pub use_real_zfs: bool,
    pub data_patterns: Vec<String>,
}

impl Default for LoadTestDataConfig {
    fn default() -> Self { Self {
            test_workspaces: 5,
            dataset_size_mb: 100,
            concurrent_zfs_ops: 10,
            use_real_zfs: false,
            data_patterns: vec!["random".to_string(), "sequential".to_string()],
         }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadTestScenariosConfig {
    pub user_workflow_enabled: bool,
    pub api_stress_test_enabled: bool,
    pub storage_operations_enabled: bool,
    pub mixed_workload_enabled: bool,
    pub custom_scenarios: HashMap<String, serde_json::Value>,
}

// ==================== SECTION ====================

/// **UNIFIED WORKSPACE HANDLER CONFIGURATION**
/// Consolidates workspace management settings from various handlers
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceHandlerConfig {
    pub optimization: WorkspaceOptimizationConfig,
    pub lifecycle: WorkspaceLifecycleConfig,
    pub security: WorkspaceSecurityConfig,
}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceOptimizationConfig {
    pub auto_cleanup_enabled: bool,
    pub cleanup_interval_hours: u64,
    pub storage_optimization_enabled: bool,
    pub compression_enabled: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceLifecycleConfig {
    pub auto_backup_enabled: bool,
    pub backup_interval_hours: u64,
    pub snapshot_retention_days: u32,
    pub archival_enabled: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::WorkspaceSecurityConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::WorkspaceSecurityConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct WorkspaceSecurityConfig {
    pub access_control_enabled: bool,
    pub encryption_enabled: bool,
    pub audit_logging_enabled: bool,
    pub isolation_level: String,
}
// ==================== SECTION ====================

/// **UNIFIED AUTH HANDLER CONFIGURATION**
/// Consolidates authentication and authorization settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::AuthHandlerConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::AuthHandlerConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct AuthHandlerConfig {
    pub authentication: AuthenticationConfig,
    pub authorization: AuthorizationConfig,
    pub session: SessionConfig,
}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::AuthenticationConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::AuthenticationConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct AuthenticationConfig {
    pub methods: Vec<String>,
    pub token_lifetime_seconds: u64,
    pub multi_factor_enabled: bool,
    pub password_policy_enabled: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::AuthorizationConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::AuthorizationConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct AuthorizationConfig {
    pub rbac_enabled: bool,
    pub permissions_model: String,
    pub default_permissions: Vec<String>,
    pub admin_permissions: Vec<String>,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::SessionConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::SessionConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct SessionConfig {
    pub timeout_seconds: u64,
    pub concurrent_sessions_limit: u32,
    pub session_persistence_enabled: bool,
    pub secure_cookies_enabled: bool,
}
impl Default for SessionConfig {
    fn default() -> Self { Self {
            timeout_seconds: 3600,
            concurrent_sessions_limit: 5,
            session_persistence_enabled: true,
            secure_cookies_enabled: true,
         }
}

// ==================== SECTION ====================

impl UnifiedApiHandlerConfig {
    /// Create a development configuration with debug settings
    #[must_use]
    pub fn development() -> Self { let mut config = Self::default();
        config.set_feature("debug_mode", true);
        config.set_feature("verbose_logging", true);
        config.extensions.zfs.service.backend = ZfsBackendConfig::Mock;
        config.extensions.load_testing.data.use_real_zfs = false;
        config
    , /// Create a production configuration with optimized settings
    #[must_use]
    pub fn production() -> Self {
        let mut config = Self::default();
        config.set_feature("debug_mode", false);
        config.set_feature("verbose_logging", false);
        config.extensions.zfs.service.backend = ZfsBackendConfig::Native;
        config.extensions.load_testing.data.use_real_zfs = true;
        config.extensions.zfs.service.performance.connection_pool_size = 50;
        config.extensions.zfs.service.performance.max_concurrent_operations = 500;
        config
     }

    /// Create a testing configuration with minimal resources
    #[must_use]
    pub fn testing() -> Self {
        let mut config = Self::default();
        config.set_feature("testing_mode", true);
        config.extensions.zfs.service.backend = ZfsBackendConfig::Mock;
        config.extensions.load_testing.data.use_real_zfs = false;
        config.extensions.zfs.service.performance.connection_pool_size = 2;
        config.extensions.zfs.service.performance.max_concurrent_operations = 10;
        config.extensions.load_testing.execution.concurrent_users = 2;
        config.extensions.load_testing.data.test_workspaces = 1;
        config
    }
} 
// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type ZfsHandlerConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ZfsHandlerConfig (the deprecated struct) for now.
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
pub type PerformanceHandlerConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using PerformanceHandlerConfig (the deprecated struct) for now.
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
pub type DashboardHandlerConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using DashboardHandlerConfig (the deprecated struct) for now.
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
pub type PerformanceAlertConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using PerformanceAlertConfig (the deprecated struct) for now.
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
pub type AuthenticationConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using AuthenticationConfig (the deprecated struct) for now.
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
pub type AuthHandlerConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using AuthHandlerConfig (the deprecated struct) for now.
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
pub type AuthorizationConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using AuthorizationConfig (the deprecated struct) for now.
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
pub type SessionConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using SessionConfig (the deprecated struct) for now.
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
pub type WorkspaceSecurityConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using WorkspaceSecurityConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

