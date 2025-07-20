//! Main ZFS Configuration Module
//!
//! The central ZFS configuration structure that integrates all configuration modules.

use nestgate_core::{NestGateError, Result, StorageTier};
use serde::{Deserialize, Serialize};

use super::{
    automation::DatasetAutomationConfig, health::HealthMonitoringConfig, metrics::MetricsConfig,
    migration::MigrationConfig, pool::PoolDiscoveryConfig, security::SecurityConfig,
    tiers::TierConfigurations,
};

/// Main ZFS configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsConfig {
    /// API endpoint for ZFS service
    pub api_endpoint: String,
    /// Default pool name for operations
    pub default_pool: String,
    /// Use real ZFS commands (false = mock mode for testing)
    pub use_real_zfs: bool,
    /// Tier configurations
    pub tiers: TierConfigurations,
    /// Pool discovery settings
    pub pool_discovery: PoolDiscoveryConfig,
    /// Health monitoring configuration
    pub health_monitoring: HealthMonitoringConfig,
    /// Metrics collection settings
    pub metrics: MetricsConfig,
    /// Migration settings
    pub migration: MigrationConfig,
    /// Security settings
    pub security: SecurityConfig,
    /// Enable AI integration features
    pub enable_ai_integration: Option<bool>,
    /// Health monitoring interval in seconds
    pub monitoring_interval: u64,
    /// Optional snapshot policies configuration file
    pub snapshot_policies_file: Option<String>,
    /// Dataset automation configuration
    pub automation: Option<DatasetAutomationConfig>,
    /// Ecosystem integration settings
    pub ecosystem_orchestrator_url: String,
    pub enable_ecosystem_integration: bool,
}

impl Default for ZfsConfig {
    fn default() -> Self {
        Self {
            api_endpoint: std::env::var("NESTGATE_API_ENDPOINT")
                .unwrap_or_else(|_| {
                    format!(
                        "http://localhost:{}",
                        nestgate_core::constants::network::api_port()
                    )
                })
                .to_string(),
            default_pool: "nestpool".to_string(),
            use_real_zfs: true,
            tiers: TierConfigurations::default(),
            pool_discovery: PoolDiscoveryConfig::default(),
            health_monitoring: HealthMonitoringConfig::default(),
            metrics: MetricsConfig::default(),
            migration: MigrationConfig::default(),
            security: SecurityConfig::default(),
            enable_ai_integration: Some(true),
            monitoring_interval: 300, // 5 minutes
            snapshot_policies_file: None,
            automation: None,
            ecosystem_orchestrator_url: std::env::var("ECOSYSTEM_ORCHESTRATOR_URL").unwrap_or_else(
                |_| {
                    format!(
                        "http://localhost:{}",
                        nestgate_core::constants::network::api_port()
                    )
                },
            ),
            enable_ecosystem_integration: std::env::var("ENABLE_ECOSYSTEM_INTEGRATION")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(false),
        }
    }
}

impl ZfsConfig {
    /// Load configuration from file with advanced integration patterns
    pub async fn load_from_file(path: &std::path::Path) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await.map_err(|e| {
            NestGateError::Configuration(format!("Failed to read config file: {e}"))
        })?;

        // Support multiple formats (YAML, JSON)
        if path.extension().and_then(|s| s.to_str()) == Some("yaml")
            || path.extension().and_then(|s| s.to_str()) == Some("yml")
        {
            serde_yaml::from_str(&content)
                .map_err(|e| NestGateError::Configuration(format!("YAML parsing error: {e}")))
        } else {
            serde_json::from_str(&content)
                .map_err(|e| NestGateError::Configuration(format!("JSON parsing error: {e}")))
        }
    }

    /// Save configuration to file
    pub async fn save_to_file(&self, path: &std::path::Path) -> Result<()> {
        let content = if path.extension().and_then(|s| s.to_str()) == Some("yaml")
            || path.extension().and_then(|s| s.to_str()) == Some("yml")
        {
            serde_yaml::to_string(self).map_err(|e| {
                NestGateError::Configuration(format!("YAML serialization error: {e}"))
            })?
        } else {
            serde_json::to_string_pretty(self).map_err(|e| {
                NestGateError::Configuration(format!("JSON serialization error: {e}"))
            })?
        };

        tokio::fs::write(path, content).await.map_err(|e| {
            NestGateError::Configuration(format!("Failed to write config file: {e}"))
        })?;

        Ok(())
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Validate API endpoint
        url::Url::parse(&self.api_endpoint)
            .map_err(|e| NestGateError::Configuration(format!("Invalid API endpoint: {e}")))?;

        // Validate pool name
        if self.default_pool.is_empty() {
            return Err(NestGateError::Configuration(
                "Default pool name cannot be empty".to_string(),
            ));
        }

        // Validate tier configurations
        self.tiers.hot.validate().map_err(|e| {
            NestGateError::Configuration(format!("Hot tier validation failed: {e}"))
        })?;
        self.tiers.warm.validate().map_err(|e| {
            NestGateError::Configuration(format!("Warm tier validation failed: {e}"))
        })?;
        self.tiers.cold.validate().map_err(|e| {
            NestGateError::Configuration(format!("Cold tier validation failed: {e}"))
        })?;

        Ok(())
    }

    /// Create a production configuration with auto-detected pools
    pub async fn production_config() -> Result<Self> {
        let mut config = Self::default();

        // Auto-detect available ZFS pools
        let available_pools = Self::detect_available_pools().await?;

        // Prefer production pool if available
        if available_pools.contains(&"nestpool-prod".to_string()) {
            config.default_pool = "nestpool-prod".to_string();
            config.tiers = TierConfigurations::production_tiers();
        } else if available_pools.contains(&"nestpool".to_string()) {
            config.default_pool = "nestpool".to_string();
            config.tiers = TierConfigurations::default();
        } else if !available_pools.is_empty() {
            // Use first available pool
            config.default_pool = available_pools[0].clone();
            config.tiers = TierConfigurations::auto_detect_tiers(&config.default_pool);
        }

        // Enable production-optimized settings
        config.use_real_zfs = true;
        config.health_monitoring.enabled = true;
        config.metrics.enabled = true;
        config.migration.background_migration = true;

        Ok(config)
    }

    /// Detect available ZFS pools on the system
    async fn detect_available_pools() -> Result<Vec<String>> {
        // Mock implementation for now - in production this would use ZFS commands
        Ok(vec!["nestpool".to_string()])
    }

    /// Get tier configuration by storage tier
    pub fn get_tier_config(&self, tier: &StorageTier) -> &super::tiers::TierConfig {
        match tier {
            StorageTier::Hot => &self.tiers.hot,
            StorageTier::Warm => &self.tiers.warm,
            StorageTier::Cold => &self.tiers.cold,
            StorageTier::Cache => &self.tiers.hot, // Use hot tier for cache
        }
    }

    /// Check if a feature is enabled
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        match feature {
            "ai_integration" => self.enable_ai_integration.unwrap_or(false),
            "ecosystem_integration" => self.enable_ecosystem_integration,
            "health_monitoring" => self.health_monitoring.enabled,
            "metrics" => self.metrics.enabled,
            "background_migration" => self.migration.background_migration,
            "encryption" => self.security.enable_encryption,
            _ => false,
        }
    }
}
