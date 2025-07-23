//! Main ZFS Configuration
//!
//! Central configuration struct for ZFS operations

use super::{
    automation::AiAutomationSettings,
    health::HealthMonitoringConfig,
    metrics::MetricsConfig,
    migration::MigrationConfig,
    pool::PoolDiscoveryConfig,
    security::SecurityConfig,
    tiers::{TierConfig, TierConfigurations},
};
use nestgate_core::StorageTier;
use serde::{Deserialize, Serialize};

/// Main ZFS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsConfig {
    /// Tier configurations
    pub tiers: TierConfigurations,
    /// Pool discovery configuration
    pub pool_discovery: PoolDiscoveryConfig,
    /// Health monitoring configuration
    pub health_monitoring: HealthMonitoringConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Migration configuration
    pub migration: MigrationConfig,
    /// Metrics configuration
    pub metrics: MetricsConfig,
    /// AI automation settings
    pub ai_automation: AiAutomationSettings,
    /// API endpoint for service registration
    pub api_endpoint: String,
}

impl Default for ZfsConfig {
    fn default() -> Self {
        Self {
            tiers: TierConfigurations::default(),
            pool_discovery: PoolDiscoveryConfig::default(),
            health_monitoring: HealthMonitoringConfig::default(),
            security: SecurityConfig::default(),
            migration: MigrationConfig::default(),
            metrics: MetricsConfig::default(),
            ai_automation: AiAutomationSettings::default(),
            api_endpoint: "http://localhost:8080".to_string(),
        }
    }
}

impl ZfsConfig {
    /// Validate the ZFS configuration
    pub fn validate(&self) -> Result<(), String> {
        // Basic validation - ensure required fields are set
        if self.api_endpoint.is_empty() {
            return Err("API endpoint cannot be empty".to_string());
        }

        if self.pool_discovery.default_pool.is_empty() {
            return Err("Default pool name cannot be empty".to_string());
        }

        // Validate discovery interval is reasonable
        if self.pool_discovery.discovery_interval_seconds < 10 {
            return Err("Discovery interval must be at least 10 seconds".to_string());
        }

        Ok(())
    }
}

impl ZfsConfig {
    /// Create production-optimized configuration
    pub fn production_optimized() -> Self {
        Self {
            tiers: TierConfigurations::production_tiers(),
            pool_discovery: PoolDiscoveryConfig::production(),
            health_monitoring: HealthMonitoringConfig::production(),
            security: SecurityConfig::production(),
            migration: MigrationConfig::production(),
            metrics: MetricsConfig::production(),
            ai_automation: AiAutomationSettings::production(),
            api_endpoint: std::env::var("NESTGATE_API_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
        }
    }

    /// Get tier configuration for a specific storage tier
    pub fn get_tier_config(&self, tier: &StorageTier) -> &TierConfig {
        match tier {
            StorageTier::Hot => &self.tiers.hot,
            StorageTier::Warm => &self.tiers.warm,
            StorageTier::Cold => &self.tiers.cold,
            StorageTier::Cache => &self.tiers.hot, // Map cache to hot tier
        }
    }
}
