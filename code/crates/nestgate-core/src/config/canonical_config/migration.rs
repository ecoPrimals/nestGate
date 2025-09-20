use crate::NestGateError;
use std::collections::HashMap;
//
// This module provides utilities for migrating from fragmented configuration
// systems to the unified canonical configuration system.
//
// **MIGRATES FROM**:
// - UnifiedApiHandlerConfig (nestgate-api)
// - UnifiedAutomationConfig (nestgate-automation)  
// - UnifiedAdapterConfig (ecosystem_integration)
// - StandardDomainConfig type aliases
// - All scattered handler-specific configurations
//
// **MIGRATES TO**:
// - NestGateCanonicalConfig (single source of truth)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{Result, NestGateError};

use super::{
    NestGateCanonicalConfig, ApiConfig, PerformanceHandlerConfig,
};
use super::api_config::{
    ZfsHandlerConfig, UnifiedPoolConfig, UnifiedDatasetConfig, UnifiedSnapshotConfig,
    ZfsServiceSettings,
};

/// **CONFIGURATION MIGRATION MANAGER**
/// Handles migration from fragmented configs to canonical unified config
#[derive(Debug)]
pub struct ConfigMigrationManager {
    /// Migration statistics
    pub stats: MigrationStats,
    /// Migration warnings and issues
    pub warnings: Vec<MigrationWarning>,
}
/// Migration statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrationStats {
    /// Total configurations migrated
    pub configs_migrated: u32,
    /// Handler configs consolidated
    pub handler_configs_consolidated: u32,
    /// Type aliases resolved
    pub type_aliases_resolved: u32,
    /// Default values applied
    pub defaults_applied: u32,
    /// Validation errors encountered
    pub validation_errors: u32,
}
/// Migration warnings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationWarning {
    /// Warning category
    pub category: MigrationWarningCategory,
    /// Warning message
    pub message: String,
    /// Source configuration path
    pub source: Option<String>,
    /// Suggested action
    pub suggested_action: String,
}
/// Migration warning categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationWarningCategory {
    /// Configuration value was deprecated
    DeprecatedValue,
    /// Default value was applied
    DefaultApplied,
    /// Type conversion occurred
    TypeConversion,
    /// Configuration was ignored
    ConfigIgnored,
    /// Validation failed
    ValidationFailed,
}
impl std::fmt::Display for MigrationWarningCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationWarningCategory::DeprecatedValue => write!(f, "DeprecatedValue"),
            MigrationWarningCategory::DefaultApplied => write!(f, "DefaultApplied"),
            MigrationWarningCategory::TypeConversion => write!(f, "TypeConversion"),
            MigrationWarningCategory::ConfigIgnored => write!(f, "ConfigIgnored"),
            MigrationWarningCategory::ValidationFailed => write!(f, "ValidationFailed"),
        }
    }
}

impl ConfigMigrationManager {
    /// Create new migration manager
    #[must_use]
    pub fn new() -> Self { Self {
            stats: MigrationStats::default(),
            warnings: Vec::new(),
         }

    /// **MIGRATE FROM FRAGMENTED API HANDLER CONFIG**
    /// Migrates UnifiedApiHandlerConfig to canonical ApiConfig
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn migrate_api_handler_config(
        &mut self,
        source: &FragmentedApiHandlerConfig,
    ) -> Result<ApiConfig>  {
        self.stats.configs_migrated += 1;

        // Start with default canonical config
        let mut api_config = ApiConfig::default();

        // Migrate ZFS handler configuration
        if let Some(zfs_config) = &source.zfs {
            api_config.zfs_handlers = self.migrate_zfs_handler_config(zfs_config)?;
            self.stats.handler_configs_consolidated += 1;
        }

        // Migrate performance handler configuration
        if let Some(perf_config) = &source.performance {
            api_config.performance_handlers = self.migrate_performance_handler_config(perf_config)?;
            self.stats.handler_configs_consolidated += 1;
        }

        // Migrate dashboard configuration
        if let Some(dashboard_config) = &source.dashboard {
            self.migrate_dashboard_config(dashboard_config, &mut api_config)?;
        }

        // Migrate load testing configuration
        if let Some(load_test_config) = &source.load_testing {
            self.migrate_load_testing_config(load_test_config, &mut api_config)?;
        }

        // Apply any custom configurations
        if !source.custom_properties.is_empty() {
            api_config.handler_extensions.custom_handlers = source.custom_properties.clone();
            self.add_warning(
                MigrationWarningCategory::TypeConversion,
                "Custom properties migrated to handler extensions".to_string(),
                Some("api_handler_config.custom_properties".to_string()),
                "Review migrated custom properties in api.handler_extensions.custom_handlers".to_string(),
            );
        }

        Ok(api_config)
    }

    /// **MIGRATE ZFS HANDLER CONFIGURATION**
    fn migrate_zfs_handler_config(
        &mut self,
        source: &FragmentedZfsHandlerConfig,
    ) -> Result<ZfsHandlerConfig> {
        let mut zfs_config = ZfsHandlerConfig::default();

        // Migrate pool configuration
        if let Some(pool_config) = &source.pool {
            zfs_config.pool = self.migrate_pool_config(pool_config)?;
        }

        // Migrate dataset configuration
        if let Some(dataset_config) = &source.dataset {
            zfs_config.dataset = self.migrate_dataset_config(dataset_config)?;
        }

        // Migrate snapshot configuration
        if let Some(snapshot_config) = &source.snapshot {
            zfs_config.snapshot = self.migrate_snapshot_config(snapshot_config)?;
        }

        // Migrate service settings
        if let Some(service_config) = &source.service {
            zfs_config.service = self.migrate_zfs_service_settings(service_config)?;
        }
    Ok(zfs_config)
    }

    /// **MIGRATE POOL CONFIGURATION**
    fn migrate_pool_config(
        &mut self,
        source: &FragmentedPoolConfig,
    ) -> Result<UnifiedPoolConfig> {
        let mut pool_config = UnifiedPoolConfig::default();

        // Migrate basic settings
        if let Some(raid_level) = &source.raid_level {
            pool_config.raid_level = Some(raid_level.clone());
        }

        if let Some(compression) = &source.compression {
            pool_config.compression = Some(compression.clone());
        }

        if let Some(dedup) = source.dedup {
            pool_config.dedup = Some(dedup);
        }

        if let Some(encryption) = source.encryption {
            pool_config.encryption = Some(encryption);
        }

        // Migrate properties
        if !source.properties.is_empty() {
            pool_config.properties = source.properties.clone();
        }

        // Handle legacy fields
        if let Some(legacy_cache_size) = source.legacy_cache_size {
            pool_config.arc_max = Some(legacy_cache_size);
            self.add_warning(
                MigrationWarningCategory::TypeConversion,
                "Legacy cache_size migrated to arc_max".to_string(),
                Some("pool_config.legacy_cache_size".to_string()),
                "Verify arc_max setting is appropriate for your system".to_string(),
            );
        }
    Ok(pool_config)
    }

    /// **MIGRATE DATASET CONFIGURATION**
    fn migrate_dataset_config(
        &mut self,
        source: &FragmentedDatasetConfig,
    ) -> Result<UnifiedDatasetConfig> {
        let mut dataset_config = UnifiedDatasetConfig::default();

        dataset_config.mount_point = source.mount_point.clone();
        dataset_config.quota = source.quota.clone();
        dataset_config.reservation = source.reservation.clone();
        dataset_config.properties = source.properties.clone();
        dataset_config.compression = source.compression.clone();
        dataset_config.dedup = source.dedup;
        dataset_config.encryption = source.encryption;

        Ok(dataset_config)
    }

    /// **MIGRATE SNAPSHOT CONFIGURATION**
    fn migrate_snapshot_config(
        &mut self,
        source: &FragmentedSnapshotConfig,
    ) -> Result<UnifiedSnapshotConfig> {
        let mut snapshot_config = UnifiedSnapshotConfig::default();

        if let Some(retention) = &source.retention_policy {
            snapshot_config.retention_policy.daily = retention.daily.unwrap_or(7);
            snapshot_config.retention_policy.weekly = retention.weekly.unwrap_or(4);
            snapshot_config.retention_policy.monthly = retention.monthly.unwrap_or(12);
            snapshot_config.retention_policy.yearly = retention.yearly.unwrap_or(3);
        }

        snapshot_config.auto_snapshot = source.auto_snapshot.unwrap_or(true);
        snapshot_config.naming_pattern = source.naming_pattern
            .clone()
            .unwrap_or_else(|| "auto-%Y%m%d-%H%M%S".to_string());
        snapshot_config.max_snapshots = source.max_snapshots.unwrap_or(100);

        Ok(snapshot_config)
    }

    /// **MIGRATE ZFS SERVICE SETTINGS**
    fn migrate_zfs_service_settings(
        &mut self,
        source: &FragmentedZfsServiceConfig,
    ) -> Result<ZfsServiceSettings> {
        let mut service_settings = ZfsServiceSettings::default();

        service_settings.enabled = source.enabled.unwrap_or(true);
        service_settings.timeout = source.timeout.unwrap_or_else(|| std::time::Duration::from_secs(30));
        service_settings.max_concurrent_ops = source.max_concurrent_ops.unwrap_or(10);
        service_settings.endpoint = source.endpoint.clone();

        Ok(service_settings)
    }

    /// **MIGRATE PERFORMANCE HANDLER CONFIGURATION**
    fn migrate_performance_handler_config(
        &mut self,
        source: &FragmentedPerformanceHandlerConfig,
    ) -> Result<PerformanceHandlerConfig> {
        let mut perf_config = PerformanceHandlerConfig::default();

        // Enable analytics if any analytics config is present
        if source.analytics_enabled.unwrap_or(false) {
            perf_config.analytics.enabled = true;
            if let Some(retention) = source.analytics_retention_days {
                perf_config.analytics.retention_days = retention;
            }
        }

        // Enable metrics if any metrics config is present
        if source.metrics_enabled.unwrap_or(false) {
            perf_config.metrics.enabled = true;
            if let Some(interval) = source.metrics_interval {
                perf_config.metrics.interval = interval;
            }
        }

        Ok(perf_config)
    }

    /// **MIGRATE DASHBOARD CONFIGURATION**
    fn migrate_dashboard_config(
        &mut self,
        source: &FragmentedDashboardConfig,
        api_config: &mut ApiConfig,
    ) -> Result<()> {
        if let Some(enabled) = source.enabled {
            api_config.performance_handlers.dashboard_integration.enabled = enabled;
        }

        if let Some(endpoint) = &source.endpoint {
            api_config.performance_handlers.dashboard_integration.endpoint = endpoint.clone();
        }
    Ok(())
    }

    /// **MIGRATE LOAD TESTING CONFIGURATION**
    fn migrate_load_testing_config(
        &mut self,
        source: &FragmentedLoadTestingConfig,
        api_config: &mut ApiConfig,
    ) -> Result<()> {
        if let Some(enabled) = source.enabled {
            api_config.performance_handlers.load_testing.enabled = enabled;
        }

        if let Some(max_sessions) = source.max_concurrent_sessions {
            api_config.performance_handlers.load_testing.max_concurrent_sessions = max_sessions;
        }
    Ok(())
    }

    /// **RESOLVE TYPE ALIAS CONFIGURATIONS**
    /// Handles migration from StandardDomainConfig type aliases
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn resolve_type_alias_config(
        &mut self,
        alias_name: &str,
        source_data: &serde_json::Value,
    ) -> Result<NestGateCanonicalConfig>  {
        self.stats.type_aliases_resolved += 1;

                    let mut canonical_config = NestGateCanonicalConfig::default();

        match alias_name {
            "UnifiedApiHandlerConfig" => {
                // Parse as fragmented API handler config and migrate
                let fragmented: FragmentedApiHandlerConfig = serde_json::from_value(source_data.clone())
                    .map_err(|_e| NestGateError::configuration(
                        config_source: crate::error::UnifiedConfigSource::Runtime,
                        suggested_fix: Some("Check configuration format".to_string()),
                        field: None,
                    )?;
                
                canonical_config.api = self.migrate_api_handler_config(&fragmented)?;
            }
            "UnifiedAutomationConfig" => {
                self.add_warning(
                    MigrationWarningCategory::ConfigIgnored,
                    "Automation config migration not yet implemented".to_string(),
                    Some(alias_name.to_string()),
                    "Implement automation config migration".to_string(),
                );
            }
            _ => {
                self.add_warning(
                    MigrationWarningCategory::ConfigIgnored,
                    format!("Unknown type alias: {"actual_error_details"}"),
                    Some(alias_name.to_string()),
                    "Add migration support for this config type".to_string(),
                );
            }
        }

        Ok(canonical_config)
    }

    /// Add migration warning
    fn add_warning(
        &mut self,
        category: MigrationWarningCategory,
        message: String,
        source: Option<String>,
        suggested_action: String,
    ) {
        self.warnings.push(MigrationWarning {
            category,
            message,
            source,
            suggested_action,
        );
    }

    /// Get migration summary
    pub const fn get_summary(&self) -> MigrationSummary {
        MigrationSummary {
            stats: self.stats.clone(),
            warnings_count: self.warnings.len(),
            success_rate: if self.stats.configs_migrated > 0 {
                ((self.stats.configs_migrated - self.stats.validation_errors) as f64 / self.stats.f64::from(configs_migrated)) * 100.0
            } else {
                100.0
            }
        }
    }
}

/// Migration summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationSummary {
    /// Migration statistics
    pub stats: MigrationStats,
    /// Number of warnings generated
    pub warnings_count: usize,
    /// Success rate percentage
    pub success_rate: f64,
}
// ==================== FRAGMENTED CONFIG TYPES FOR MIGRATION ====================

/// **FRAGMENTED API HANDLER CONFIG** (for migration from nestgate-api)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FragmentedApiHandlerConfig {
    /// ZFS handler configurations
    pub zfs: Option<FragmentedZfsHandlerConfig>,
    /// Performance analytics configurations
    pub performance: Option<FragmentedPerformanceHandlerConfig>,
    /// Dashboard configurations
    pub dashboard: Option<FragmentedDashboardConfig>,
    /// Load testing configurations
    pub load_testing: Option<FragmentedLoadTestingConfig>,
    /// Custom properties
    pub custom_properties: HashMap<String, serde_json::Value>,
}
/// **FRAGMENTED ZFS HANDLER CONFIG** (for migration)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FragmentedZfsHandlerConfig {
    /// Pool management settings
    pub pool: Option<FragmentedPoolConfig>,
    /// Dataset management settings
    pub dataset: Option<FragmentedDatasetConfig>,
    /// Snapshot management settings
    pub snapshot: Option<FragmentedSnapshotConfig>,
    /// Service-level ZFS settings
    pub service: Option<FragmentedZfsServiceConfig>,
}
/// **FRAGMENTED POOL CONFIG** (for migration)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FragmentedPoolConfig {
    /// RAID level
    pub raid_level: Option<String>,
    /// Compression algorithm
    pub compression: Option<String>,
    /// Deduplication enabled
    pub dedup: Option<bool>,
    /// Encryption enabled
    pub encryption: Option<bool>,
    /// Pool properties
    pub properties: HashMap<String, String>,
    /// Legacy cache size (to be migrated to arc_max)
    pub legacy_cache_size: Option<u64>,
}
/// **FRAGMENTED DATASET CONFIG** (for migration)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FragmentedDatasetConfig {
    /// Mount point
    pub mount_point: Option<String>,
    /// Quota settings
    pub quota: Option<String>,
    /// Reservation settings
    pub reservation: Option<String>,
    /// Dataset properties
    pub properties: HashMap<String, String>,
    /// Compression settings
    pub compression: Option<String>,
    /// Deduplication settings
    pub dedup: Option<bool>,
    /// Encryption settings
    pub encryption: Option<bool>,
}
/// **FRAGMENTED SNAPSHOT CONFIG** (for migration)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FragmentedSnapshotConfig {
    /// Retention policy
    pub retention_policy: Option<FragmentedRetentionPolicy>,
    /// Automatic snapshot scheduling
    pub auto_snapshot: Option<bool>,
    /// Snapshot naming pattern
    pub naming_pattern: Option<String>,
    /// Maximum snapshots to keep
    pub max_snapshots: Option<u32>,
}
/// **FRAGMENTED RETENTION POLICY** (for migration)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FragmentedRetentionPolicy {
    /// Daily snapshots to keep
    pub daily: Option<u32>,
    /// Weekly snapshots to keep  
    pub weekly: Option<u32>,
    /// Monthly snapshots to keep
    pub monthly: Option<u32>,
    /// Yearly snapshots to keep
    pub yearly: Option<u32>,
}
/// **FRAGMENTED ZFS SERVICE CONFIG** (for migration)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FragmentedZfsServiceConfig {
    /// Enable ZFS service
    pub enabled: Option<bool>,
    /// Service timeout
    pub timeout: Option<std::time::Duration>,
    /// Maximum concurrent operations
    pub max_concurrent_ops: Option<u32>,
    /// Service endpoint
    pub endpoint: Option<String>,
}
/// **FRAGMENTED PERFORMANCE HANDLER CONFIG** (for migration)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FragmentedPerformanceHandlerConfig {
    /// Enable analytics
    pub analytics_enabled: Option<bool>,
    /// Analytics retention days
    pub analytics_retention_days: Option<u32>,
    /// Enable metrics
    pub metrics_enabled: Option<bool>,
    /// Metrics collection interval
    pub metrics_interval: Option<std::time::Duration>,
}
/// **FRAGMENTED DASHBOARD CONFIG** (for migration)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FragmentedDashboardConfig {
    /// Enable dashboard
    pub enabled: Option<bool>,
    /// Dashboard endpoint
    pub endpoint: Option<String>,
}
/// **FRAGMENTED LOAD TESTING CONFIG** (for migration)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FragmentedLoadTestingConfig {
    /// Enable load testing
    pub enabled: Option<bool>,
    /// Maximum concurrent sessions
    pub max_concurrent_sessions: Option<u32>,
}
/// **CONFIGURATION MIGRATION UTILITIES**
impl Default for ConfigMigrationManager {
    fn default() -> Self {
        Self::new()
    }
}
/// **MIGRATION CONVENIENCE FUNCTIONS**
impl ConfigMigrationManager {
    /// Migrate from JSON configuration file
        let content = std::fs::read_to_string(file_path)
            .map_err(|_e| NestGateError::configuration(
                config_source: crate::error::UnifiedConfigSource::File(file_path.to_string()),
                suggested_fix: Some("Check file path and permissions".to_string()),
                field: None,
            )?;
        let jsonvalue: serde_json::Value = serde_json::from_str(&content)
            .map_err(|_e| NestGateError::configuration(
                config_source: crate::error::UnifiedConfigSource::File(file_path.to_string()),
                suggested_fix: Some("Check JSON syntax".to_string()),
                field: None,
            )?;

        // Try to detect config type and migrate accordingly
        self.migrate_from_jsonvalue(&jsonvalue)
    }

    /// Migrate from JSON value
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn migrate_from_jsonvalue(&mut self, jsonvalue: &serde_json::Value) -> Result<NestGateCanonicalConfig>  {
        // Try to parse as fragmented API handler config first
        if let Ok(fragmented) = serde_json::from_value::<FragmentedApiHandlerConfig>(jsonvalue.clone()) {
            let mut canonical_config = NestGateCanonicalConfig::default();
            canonical_config.api = self.migrate_api_handler_config(&fragmented)?;
            return Ok(canonical_config);
        }

        // If direct parsing fails, return default config with warning
        self.add_warning(
            MigrationWarningCategory::ConfigIgnored,
            "Could not determine config type, using defaults".to_string(),
            None,
            "Manually configure the canonical config".to_string(),
        );

        Ok(NestGateCanonicalConfig::default())
    }
} 