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
/// Apihandlerextensions
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
/// Configuration for ZfsHandler
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
    /// Returns the default instance
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
/// Configuration for UnifiedPool
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
    /// Returns the default instance
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
/// Configuration for UnifiedDataset
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
    /// Returns the default instance
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
/// Configuration for UnifiedSnapshot
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
/// Zfsservicesettings
pub struct ZfsServiceSettings {
    /// Service backend configuration
    pub backend: ZfsBackendConfig,
    /// Fail-safe and reliability settings
    pub fail_safe: ZfsFailSafeConfig,
    /// Performance optimization settings
    pub performance: ZfsPerformanceConfig,
}
impl Default for ZfsServiceSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            backend: ZfsBackendConfig::Auto,
            fail_safe: ZfsFailSafeConfig::default(),
            performance: ZfsPerformanceConfig::default(),
         }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfsbackendconfig
pub enum ZfsBackendConfig {
    /// Auto
    Auto,
    /// Native
    Native,
    /// Mock
    Mock,
    /// Remote
    Remote { endpoint: String, timeout: Duration }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ZfsFailSafe
pub struct ZfsFailSafeConfig {
    /// Circuit Breaker Enabled
    pub circuit_breaker_enabled: bool,
    /// Retry Attempts
    pub retry_attempts: u32,
    /// Timeout Seconds
    pub timeout_seconds: u64,
    /// Graceful Degradation
    pub graceful_degradation: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ZfsPerformance
pub struct ZfsPerformanceConfig {
    /// Size of connection pool
    pub connection_pool_size: u32,
    /// Max Concurrent Operations
    pub max_concurrent_operations: u32,
    /// Cache Enabled
    pub cache_enabled: bool,
    /// Cache Ttl Seconds
    pub cache_ttl_seconds: u64,
    /// Size of batch
    pub batch_size: u32,
}

impl Default for ZfsPerformanceConfig {
    /// Returns the default instance
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
/// Configuration for PerformanceHandler
pub struct PerformanceHandlerConfig {
    /// Metrics collection settings
    pub metrics: PerformanceMetricsConfig,
    /// Analytics and processing settings
    pub analytics: PerformanceAnalyticsConfig,
    /// Alert threshold settings
    pub alerts: PerformanceAlertConfig,
}
impl Default for PerformanceHandlerConfig {
    /// Returns the default instance
    fn default() -> Self { Self {
            metrics: PerformanceMetricsConfig::default(),
            analytics: PerformanceAnalyticsConfig::default(),
            alerts: PerformanceAlertConfig::default(),
         }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for PerformanceMetrics
pub struct PerformanceMetricsConfig {
    /// Collection Interval Seconds
    pub collection_interval_seconds: u64,
    /// Retention Days
    pub retention_days: u32,
    /// Enabled Metrics
    pub enabled_metrics: Vec<String>,
    /// Custom Metrics
    pub custom_metrics: HashMap<String, String>,
}

impl Default for PerformanceMetricsConfig {
    /// Returns the default instance
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
/// Configuration for PerformanceAnalytics
pub struct PerformanceAnalyticsConfig {
    /// Predictive Enabled
    pub predictive_enabled: bool,
    /// Machine Learning Enabled
    pub machine_learning_enabled: bool,
    /// Trend Analysis Enabled
    pub trend_analysis_enabled: bool,
    /// Anomaly Detection Enabled
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
/// Configuration for PerformanceAlert
pub struct PerformanceAlertConfig {
    /// Cpu Threshold
    pub cpu_threshold: f64,
    /// Memory Threshold
    pub memory_threshold: f64,
    /// Disk Threshold
    pub disk_threshold: f64,
    /// Network Latency Threshold
    pub network_latency_threshold: f64,
    /// Zfs Health Threshold
    pub zfs_health_threshold: f64,
    /// Custom Thresholds
    pub custom_thresholds: HashMap<String, f64>,
}

impl Default for PerformanceAlertConfig {
    /// Returns the default instance
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
/// Configuration for DashboardHandler
pub struct DashboardHandlerConfig {
    /// Dashboard layout and UI settings
    pub layout: DashboardLayoutConfig,
    /// Data refresh and caching settings
    pub data: DashboardDataConfig,
    /// Visualization settings
    pub visualization: DashboardVisualizationConfig,
}
impl Default for DashboardHandlerConfig {
    /// Returns the default instance
    fn default() -> Self { Self {
            layout: DashboardLayoutConfig::default(),
            data: DashboardDataConfig::default(),
            visualization: DashboardVisualizationConfig::default(),
         }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for DashboardLayout
pub struct DashboardLayoutConfig {
    /// Theme
    pub theme: String,
    /// Columns
    pub columns: u32,
    /// Auto Refresh Enabled
    pub auto_refresh_enabled: bool,
    /// Refresh Interval Seconds
    pub refresh_interval_seconds: u64,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for DashboardData
pub struct DashboardDataConfig {
    /// Cache Enabled
    pub cache_enabled: bool,
    /// Cache Ttl Seconds
    pub cache_ttl_seconds: u64,
    /// Max Data Points
    pub max_data_points: u32,
    /// Data Sources
    pub data_sources: Vec<String>,
}

impl Default for DashboardDataConfig {
    /// Returns the default instance
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
/// Configuration for DashboardVisualization
pub struct DashboardVisualizationConfig {
    /// Chart Types
    pub chart_types: Vec<String>,
    /// Color Scheme
    pub color_scheme: String,
    /// Animation Enabled
    pub animation_enabled: bool,
    /// Responsive Design
    pub responsive_design: bool,
}

// ==================== SECTION ====================

/// **UNIFIED LOAD TESTING HANDLER CONFIGURATION**
/// Replaces: load_testing.rs::LoadTestConfig, load_testing.rs::TestDataConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for LoadTestingHandler
pub struct LoadTestingHandlerConfig {
    /// Test execution settings
    pub execution: LoadTestExecutionConfig,
    /// Test data generation settings
    pub data: LoadTestDataConfig,
    /// Test scenario definitions
    pub scenarios: LoadTestScenariosConfig,
}
impl Default for LoadTestingHandlerConfig {
    /// Returns the default instance
    fn default() -> Self { Self {
            execution: LoadTestExecutionConfig::default(),
            data: LoadTestDataConfig::default(),
            scenarios: LoadTestScenariosConfig::default(),
         }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for LoadTestExecution
pub struct LoadTestExecutionConfig {
    /// Duration Seconds
    pub duration_seconds: u64,
    /// Concurrent Users
    pub concurrent_users: u32,
    /// Requests Per Second
    pub requests_per_second: f64,
    /// Ramp Up Seconds
    pub ramp_up_seconds: u64,
    /// Ramp Down Seconds
    pub ramp_down_seconds: u64,
}

impl Default for LoadTestExecutionConfig {
    /// Returns the default instance
    fn default() -> Self { Self {
            duration_seconds: 300,
            concurrent_users: 10,
            requests_per_second: 1.0,
            ramp_up_seconds: 30,
            ramp_down_seconds: 30,
         }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for LoadTestData
pub struct LoadTestDataConfig {
    /// Test Workspaces
    pub test_workspaces: u32,
    /// Dataset Size in megabytes
    pub dataset_size_mb: u32,
    /// Concurrent Zfs Ops
    pub concurrent_zfs_ops: u32,
    /// Use Real Zfs
    pub use_real_zfs: bool,
    /// Data Patterns
    pub data_patterns: Vec<String>,
}

impl Default for LoadTestDataConfig {
    /// Returns the default instance
    fn default() -> Self { Self {
            test_workspaces: 5,
            dataset_size_mb: 100,
            concurrent_zfs_ops: 10,
            use_real_zfs: false,
            data_patterns: vec!["random".to_string(), "sequential".to_string()],
         }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for LoadTestScenarios
pub struct LoadTestScenariosConfig {
    /// User Workflow Enabled
    pub user_workflow_enabled: bool,
    /// Api Stress Test Enabled
    pub api_stress_test_enabled: bool,
    /// Storage Operations Enabled
    pub storage_operations_enabled: bool,
    /// Mixed Workload Enabled
    pub mixed_workload_enabled: bool,
    /// Custom Scenarios
    pub custom_scenarios: HashMap<String, serde_json::Value>,
}

// ==================== SECTION ====================

/// **UNIFIED WORKSPACE HANDLER CONFIGURATION**
/// Consolidates workspace management settings from various handlers
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for WorkspaceHandler
pub struct WorkspaceHandlerConfig {
    /// Optimization
    pub optimization: WorkspaceOptimizationConfig,
    /// Lifecycle
    pub lifecycle: WorkspaceLifecycleConfig,
    /// Security
    pub security: WorkspaceSecurityConfig,
}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for WorkspaceOptimization
pub struct WorkspaceOptimizationConfig {
    /// Auto Cleanup Enabled
    pub auto_cleanup_enabled: bool,
    /// Cleanup Interval Hours
    pub cleanup_interval_hours: u64,
    /// Storage Optimization Enabled
    pub storage_optimization_enabled: bool,
    /// Compression Enabled
    pub compression_enabled: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for WorkspaceLifecycle
pub struct WorkspaceLifecycleConfig {
    /// Auto Backup Enabled
    pub auto_backup_enabled: bool,
    /// Backup Interval Hours
    pub backup_interval_hours: u64,
    /// Snapshot Retention Days
    pub snapshot_retention_days: u32,
    /// Archival Enabled
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
/// Configuration for WorkspaceSecurity
pub struct WorkspaceSecurityConfig {
    /// Access Control Enabled
    pub access_control_enabled: bool,
    /// Encryption Enabled
    pub encryption_enabled: bool,
    /// Audit Logging Enabled
    pub audit_logging_enabled: bool,
    /// Isolation Level
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
/// Configuration for AuthHandler
pub struct AuthHandlerConfig {
    /// Authentication
    pub authentication: AuthenticationConfig,
    /// Authorization
    pub authorization: AuthorizationConfig,
    /// Session
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
/// Configuration for Authentication
pub struct AuthenticationConfig {
    /// Methods
    pub methods: Vec<String>,
    /// Token Lifetime Seconds
    pub token_lifetime_seconds: u64,
    /// Multi Factor Enabled
    pub multi_factor_enabled: bool,
    /// Password Policy Enabled
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
/// Configuration for Authorization
pub struct AuthorizationConfig {
    /// Rbac Enabled
    pub rbac_enabled: bool,
    /// Permissions Model
    pub permissions_model: String,
    /// Default Permissions
    pub default_permissions: Vec<String>,
    /// Admin Permissions
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
/// Configuration for Session
pub struct SessionConfig {
    /// Timeout Seconds
    pub timeout_seconds: u64,
    /// Concurrent Sessions Limit
    pub concurrent_sessions_limit: u32,
    /// Session Persistence Enabled
    pub session_persistence_enabled: bool,
    /// Secure Cookies Enabled
    pub secure_cookies_enabled: bool,
}
impl Default for SessionConfig {
    /// Returns the default instance
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
/// Type alias for Zfshandlerconfigcanonical
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
/// Type alias for Performancehandlerconfigcanonical
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
/// Type alias for Dashboardhandlerconfigcanonical
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
/// Type alias for Performancealertconfigcanonical
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
/// Type alias for Authenticationconfigcanonical
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
/// Type alias for Authhandlerconfigcanonical
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
/// Type alias for Authorizationconfigcanonical
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
/// Type alias for Sessionconfigcanonical
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
/// Type alias for Workspacesecurityconfigcanonical
pub type WorkspaceSecurityConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using WorkspaceSecurityConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

