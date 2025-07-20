//! Tier Configuration Module
//!
//! Configuration for hot/warm/cold storage tiers, performance profiles, and migration rules.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::migration::{CapacityLimits, MigrationRules};

/// Tier-specific configurations for hot/warm/cold storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierConfigurations {
    pub hot: TierConfig,
    pub warm: TierConfig,
    pub cold: TierConfig,
}

/// Individual tier configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierConfig {
    /// Tier name
    pub name: String,
    /// Pool name for this tier
    pub pool_name: String,
    /// Dataset prefix for this tier
    pub dataset_prefix: String,
    /// ZFS properties for this tier
    pub properties: HashMap<String, String>,
    /// Performance profile
    pub performance_profile: PerformanceProfile,
    /// Migration rules
    pub migration_rules: MigrationRules,
    /// Capacity limits
    pub capacity_limits: CapacityLimits,
}

/// Performance profile for tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceProfile {
    HighPerformance, // Hot tier - optimized for speed
    Balanced,        // Warm tier - balance of speed and compression
    HighCompression, // Cold tier - optimized for space efficiency
}

impl Default for TierConfigurations {
    fn default() -> Self {
        Self {
            hot: TierConfig::hot_tier_default(),
            warm: TierConfig::warm_tier_default(),
            cold: TierConfig::cold_tier_default(),
        }
    }
}

impl TierConfigurations {
    /// Create production-optimized tier configurations
    pub fn production_tiers() -> Self {
        Self {
            hot: TierConfig::hot_tier_production(),
            warm: TierConfig::warm_tier_production(),
            cold: TierConfig::cold_tier_production(),
        }
    }

    /// Auto-detect tier configurations for a given pool
    pub fn auto_detect_tiers(pool_name: &str) -> Self {
        Self {
            hot: TierConfig::auto_detect_hot(pool_name),
            warm: TierConfig::auto_detect_warm(pool_name),
            cold: TierConfig::auto_detect_cold(pool_name),
        }
    }
}

impl TierConfig {
    /// Default configuration for hot tier (high performance)
    pub fn hot_tier_default() -> Self {
        let mut properties = HashMap::new();
        properties.insert("compression".to_string(), "off".to_string());
        properties.insert("recordsize".to_string(), "128K".to_string());
        properties.insert("atime".to_string(), "off".to_string());
        properties.insert("primarycache".to_string(), "all".to_string());
        properties.insert("secondarycache".to_string(), "all".to_string());

        Self {
            name: "hot".to_string(),
            pool_name: "nestpool".to_string(),
            dataset_prefix: "hot".to_string(),
            properties,
            performance_profile: PerformanceProfile::HighPerformance,
            migration_rules: MigrationRules::hot_tier_defaults(),
            capacity_limits: CapacityLimits::hot_tier_defaults(),
        }
    }

    /// Default configuration for warm tier (balanced)
    pub fn warm_tier_default() -> Self {
        let mut properties = HashMap::new();
        properties.insert("compression".to_string(), "lz4".to_string());
        properties.insert("recordsize".to_string(), "1M".to_string());
        properties.insert("atime".to_string(), "on".to_string());
        properties.insert("primarycache".to_string(), "metadata".to_string());
        properties.insert("secondarycache".to_string(), "metadata".to_string());

        Self {
            name: "warm".to_string(),
            pool_name: "nestpool".to_string(),
            dataset_prefix: "warm".to_string(),
            properties,
            performance_profile: PerformanceProfile::Balanced,
            migration_rules: MigrationRules::warm_tier_defaults(),
            capacity_limits: CapacityLimits::warm_tier_defaults(),
        }
    }

    /// Default configuration for cold tier (high compression)
    pub fn cold_tier_default() -> Self {
        let mut properties = HashMap::new();
        properties.insert("compression".to_string(), "zstd".to_string());
        properties.insert("recordsize".to_string(), "1M".to_string());
        properties.insert("atime".to_string(), "off".to_string());
        properties.insert("primarycache".to_string(), "metadata".to_string());
        properties.insert("secondarycache".to_string(), "none".to_string());
        properties.insert("sync".to_string(), "always".to_string());

        Self {
            name: "cold".to_string(),
            pool_name: "nestpool".to_string(),
            dataset_prefix: "cold".to_string(),
            properties,
            performance_profile: PerformanceProfile::HighCompression,
            migration_rules: MigrationRules::cold_tier_defaults(),
            capacity_limits: CapacityLimits::cold_tier_defaults(),
        }
    }

    /// Create production-optimized hot tier configuration
    pub fn hot_tier_production() -> Self {
        let mut config = Self::hot_tier_default();
        config.pool_name = "nestpool-prod".to_string();

        // Performance-optimized properties for hot tier
        config
            .properties
            .insert("recordsize".to_string(), "64K".to_string());
        config
            .properties
            .insert("compression".to_string(), "lz4".to_string());
        config
            .properties
            .insert("primarycache".to_string(), "all".to_string());
        config
            .properties
            .insert("secondarycache".to_string(), "all".to_string());
        config
            .properties
            .insert("logbias".to_string(), "latency".to_string());
        config
            .properties
            .insert("sync".to_string(), "standard".to_string());

        // Aggressive migration rules for hot tier
        config.migration_rules.age_threshold_days = 7;
        config.migration_rules.access_frequency_threshold = 10.0;
        config.migration_rules.auto_migration_enabled = true;

        config
    }

    /// Create production-optimized warm tier configuration
    pub fn warm_tier_production() -> Self {
        let mut config = Self::warm_tier_default();
        config.pool_name = "nestpool-prod".to_string();

        // Balanced properties for warm tier
        config
            .properties
            .insert("recordsize".to_string(), "128K".to_string());
        config
            .properties
            .insert("compression".to_string(), "zstd".to_string());
        config
            .properties
            .insert("primarycache".to_string(), "metadata".to_string());
        config
            .properties
            .insert("secondarycache".to_string(), "metadata".to_string());
        config
            .properties
            .insert("logbias".to_string(), "throughput".to_string());

        // Balanced migration rules
        config.migration_rules.age_threshold_days = 30;
        config.migration_rules.access_frequency_threshold = 2.0;
        config.migration_rules.auto_migration_enabled = true;

        config
    }

    /// Create production-optimized cold tier configuration
    pub fn cold_tier_production() -> Self {
        let mut config = Self::cold_tier_default();
        config.pool_name = "nestpool-prod".to_string();

        // Space-optimized properties for cold tier
        config
            .properties
            .insert("recordsize".to_string(), "1M".to_string());
        config
            .properties
            .insert("compression".to_string(), "gzip-9".to_string());
        config
            .properties
            .insert("primarycache".to_string(), "metadata".to_string());
        config
            .properties
            .insert("secondarycache".to_string(), "none".to_string());
        config
            .properties
            .insert("logbias".to_string(), "throughput".to_string());
        config
            .properties
            .insert("dedup".to_string(), "on".to_string());

        // Conservative migration rules
        config.migration_rules.age_threshold_days = 90;
        config.migration_rules.access_frequency_threshold = 0.1;
        config.migration_rules.auto_migration_enabled = true;

        config
    }

    /// Auto-detect hot tier configuration for any pool
    pub fn auto_detect_hot(pool_name: &str) -> Self {
        let mut config = Self::hot_tier_default();
        config.pool_name = pool_name.to_string();
        config.dataset_prefix = format!("{pool_name}/hot");
        config
    }

    /// Auto-detect warm tier configuration for any pool
    pub fn auto_detect_warm(pool_name: &str) -> Self {
        let mut config = Self::warm_tier_default();
        config.pool_name = pool_name.to_string();
        config.dataset_prefix = format!("{pool_name}/warm");
        config
    }

    /// Auto-detect cold tier configuration for any pool
    pub fn auto_detect_cold(pool_name: &str) -> Self {
        let mut config = Self::cold_tier_default();
        config.pool_name = pool_name.to_string();
        config.dataset_prefix = format!("{pool_name}/cold");
        config
    }

    /// Validate tier configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Tier name cannot be empty".to_string());
        }
        if self.pool_name.is_empty() {
            return Err("Pool name cannot be empty".to_string());
        }
        if self.dataset_prefix.is_empty() {
            return Err("Dataset prefix cannot be empty".to_string());
        }
        Ok(())
    }
}
