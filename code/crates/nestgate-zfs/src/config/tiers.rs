//
// Configuration for hot/warm/cold storage tiers, performance profiles, and migration rules.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Migration module not yet implemented - using local types
// use super::migration::{CapacityLimits, MigrationRules};
use crate::automation::policies::MigrationRules;

// Temporary local definition until migration module is implemented
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CapacityLimits {
    pub max_size_gb: u64,
    pub min_free_space_gb: u64,
    pub warning_threshold_percent: f64,
}

impl CapacityLimits {
    #[must_use]
    pub fn hot_tier_defaults() -> Self {
        Self {
            max_size_gb: 1000,
            min_free_space_gb: 100,
            warning_threshold_percent: 80.0,
        }
    }

    #[must_use]
    pub fn warm_tier_defaults() -> Self {
        Self {
            max_size_gb: 10000,
            min_free_space_gb: 1000,
            warning_threshold_percent: 85.0,
        }
    }

    #[must_use]
    pub fn cold_tier_defaults() -> Self {
        Self {
            max_size_gb: 100000,
            min_free_space_gb: 10000,
            warning_threshold_percent: 90.0,
        }
    }
}

// Use canonical constants system
use nestgate_core::canonical_modernization::canonical_constants::storage::{
    COMPRESSION_LZ4, TIER_COLD, TIER_HOT, TIER_WARM,
};
// Define missing constants locally
const POOL_DEFAULT: &str = "default";
const POOL_PRODUCTION: &str = "production";
const COMPRESSION_OFF: &str = "off";

// Define missing ZFS constants locally until they're added to canonical constants
const RECORDSIZE_PROPERTY: &str = "recordsize";
use nestgate_core::canonical_modernization::canonical_constants::zfs::{
    RECORDSIZE_128K, RECORDSIZE_1M, RECORDSIZE_64K,
};
const ATIME_PROPERTY: &str = "atime";
const PRIMARYCACHE_PROPERTY: &str = "primarycache";
const CACHE_METADATA: &str = "metadata";
const CACHE_ALL: &str = "all";
const CACHE_NONE: &str = "none";
const SECONDARYCACHE_PROPERTY: &str = "secondarycache";
const SYNC_PROPERTY: &str = "sync";
const COMPRESSION_PROPERTY: &str = "compression";
const COMPRESSION_ZSTD: &str = "zstd";
const COMPRESSION_GZIP_9: &str = "gzip-9";
const LOGBIAS_PROPERTY: &str = "logbias";
const BIAS_LATENCY: &str = "latency";
const BIAS_THROUGHPUT: &str = "throughput";
const DEDUP_PROPERTY: &str = "dedup";
const VALUE_ALWAYS: &str = "always";
const VALUE_ON: &str = "on";
const VALUE_OFF: &str = "off";
const VALUE_STANDARD: &str = "standard";

// Local ZFS-specific constants not in centralized system
const ERROR_TIER_NAME_EMPTY: &str = "Tier name cannot be empty";
const ERROR_POOL_NAME_EMPTY: &str = "Pool name cannot be empty";
const ERROR_DATASET_PREFIX_EMPTY: &str = "Dataset prefix cannot be empty";

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
    #[must_use]
    pub fn production_tiers() -> Self {
        Self {
            hot: TierConfig::hot_tier_production(),
            warm: TierConfig::warm_tier_production(),
            cold: TierConfig::cold_tier_production(),
        }
    }

    /// Auto-detect tier configurations for a given pool
    #[must_use]
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
    #[must_use]
    pub fn hot_tier_default() -> Self {
        let mut properties = HashMap::new();
        properties.insert(
            COMPRESSION_PROPERTY.to_string(),
            COMPRESSION_OFF.to_string(),
        );
        properties.insert(RECORDSIZE_PROPERTY.to_string(), RECORDSIZE_128K.to_string());
        properties.insert(ATIME_PROPERTY.to_string(), VALUE_OFF.to_string());
        properties.insert(PRIMARYCACHE_PROPERTY.to_string(), CACHE_ALL.to_string());
        properties.insert(SECONDARYCACHE_PROPERTY.to_string(), CACHE_ALL.to_string());

        Self {
            name: TIER_HOT.to_string(),
            pool_name: POOL_DEFAULT.to_string(),
            dataset_prefix: TIER_HOT.to_string(),
            properties,
            performance_profile: PerformanceProfile::HighPerformance,
            migration_rules: MigrationRules::hot_tier_defaults(),
            capacity_limits: CapacityLimits::hot_tier_defaults(),
        }
    }

    /// Default configuration for warm tier (balanced)
    #[must_use]
    pub fn warm_tier_default() -> Self {
        let mut properties = HashMap::new();
        properties.insert(
            COMPRESSION_PROPERTY.to_string(),
            COMPRESSION_LZ4.to_string(),
        );
        properties.insert(RECORDSIZE_PROPERTY.to_string(), RECORDSIZE_1M.to_string());
        properties.insert(ATIME_PROPERTY.to_string(), VALUE_ON.to_string());
        properties.insert(
            PRIMARYCACHE_PROPERTY.to_string(),
            CACHE_METADATA.to_string(),
        );
        properties.insert(
            SECONDARYCACHE_PROPERTY.to_string(),
            CACHE_METADATA.to_string(),
        );

        Self {
            name: TIER_WARM.to_string(),
            pool_name: POOL_DEFAULT.to_string(),
            dataset_prefix: TIER_WARM.to_string(),
            properties,
            performance_profile: PerformanceProfile::Balanced,
            migration_rules: MigrationRules::warm_tier_defaults(),
            capacity_limits: CapacityLimits::warm_tier_defaults(),
        }
    }

    /// Default configuration for cold tier (high compression)
    #[must_use]
    pub fn cold_tier_default() -> Self {
        let mut properties = HashMap::new();
        properties.insert(
            COMPRESSION_PROPERTY.to_string(),
            COMPRESSION_ZSTD.to_string(),
        );
        properties.insert(RECORDSIZE_PROPERTY.to_string(), RECORDSIZE_1M.to_string());
        properties.insert(ATIME_PROPERTY.to_string(), VALUE_OFF.to_string());
        properties.insert(
            PRIMARYCACHE_PROPERTY.to_string(),
            CACHE_METADATA.to_string(),
        );
        properties.insert(SECONDARYCACHE_PROPERTY.to_string(), CACHE_NONE.to_string());
        properties.insert(SYNC_PROPERTY.to_string(), VALUE_ALWAYS.to_string());

        Self {
            name: TIER_COLD.to_string(),
            pool_name: POOL_DEFAULT.to_string(),
            dataset_prefix: TIER_COLD.to_string(),
            properties,
            performance_profile: PerformanceProfile::HighCompression,
            migration_rules: MigrationRules::cold_tier_defaults(),
            capacity_limits: CapacityLimits::cold_tier_defaults(),
        }
    }

    /// Create production-optimized hot tier configuration
    #[must_use]
    pub fn hot_tier_production() -> Self {
        let mut config = Self::hot_tier_default();
        config.pool_name = POOL_PRODUCTION.to_string();

        // Performance-optimized properties for hot tier
        config
            .properties
            .insert(RECORDSIZE_PROPERTY.to_string(), RECORDSIZE_64K.to_string());
        config.properties.insert(
            COMPRESSION_PROPERTY.to_string(),
            COMPRESSION_LZ4.to_string(),
        );
        config
            .properties
            .insert(PRIMARYCACHE_PROPERTY.to_string(), CACHE_ALL.to_string());
        config
            .properties
            .insert(SECONDARYCACHE_PROPERTY.to_string(), CACHE_ALL.to_string());
        config
            .properties
            .insert(LOGBIAS_PROPERTY.to_string(), BIAS_LATENCY.to_string());
        config
            .properties
            .insert(SYNC_PROPERTY.to_string(), VALUE_STANDARD.to_string());

        // Aggressive migration rules for hot tier
        config.migration_rules.age_threshold_days = 7; // 7 days
        config.migration_rules.access_frequency_threshold = 10.0;
        config.migration_rules.auto_migration_enabled = true;

        config
    }

    /// Create production-optimized warm tier configuration
    #[must_use]
    pub fn warm_tier_production() -> Self {
        let mut config = Self::warm_tier_default();
        config.pool_name = POOL_PRODUCTION.to_string();

        // Balanced properties for warm tier
        config
            .properties
            .insert(RECORDSIZE_PROPERTY.to_string(), RECORDSIZE_128K.to_string());
        config.properties.insert(
            COMPRESSION_PROPERTY.to_string(),
            COMPRESSION_ZSTD.to_string(),
        );
        config.properties.insert(
            PRIMARYCACHE_PROPERTY.to_string(),
            CACHE_METADATA.to_string(),
        );
        config.properties.insert(
            SECONDARYCACHE_PROPERTY.to_string(),
            CACHE_METADATA.to_string(),
        );
        config
            .properties
            .insert(LOGBIAS_PROPERTY.to_string(), BIAS_THROUGHPUT.to_string());

        // Balanced migration rules
        config.migration_rules.age_threshold_days = 30; // 30 days
        config.migration_rules.access_frequency_threshold = 2.0;
        config.migration_rules.auto_migration_enabled = true;

        config
    }

    /// Create production-optimized cold tier configuration
    #[must_use]
    pub fn cold_tier_production() -> Self {
        let mut config = Self::cold_tier_default();
        config.pool_name = POOL_PRODUCTION.to_string();

        // Space-optimized properties for cold tier
        config
            .properties
            .insert(RECORDSIZE_PROPERTY.to_string(), RECORDSIZE_1M.to_string());
        config.properties.insert(
            COMPRESSION_PROPERTY.to_string(),
            COMPRESSION_GZIP_9.to_string(),
        );
        config.properties.insert(
            PRIMARYCACHE_PROPERTY.to_string(),
            CACHE_METADATA.to_string(),
        );
        config
            .properties
            .insert(SECONDARYCACHE_PROPERTY.to_string(), CACHE_NONE.to_string());
        config
            .properties
            .insert(LOGBIAS_PROPERTY.to_string(), BIAS_THROUGHPUT.to_string());
        config
            .properties
            .insert(DEDUP_PROPERTY.to_string(), VALUE_ON.to_string());

        // Conservative migration rules
        config.migration_rules.age_threshold_days = 90; // 90 days
        config.migration_rules.access_frequency_threshold = 1.0;
        config.migration_rules.auto_migration_enabled = true;

        config
    }

    /// Auto-detect hot tier configuration for any pool
    #[must_use]
    pub fn auto_detect_hot(pool_name: &str) -> Self {
        let mut config = Self::hot_tier_default();
        config.pool_name = pool_name.to_string();
        config.dataset_prefix = TIER_HOT.to_string();
        config
    }

    /// Auto-detect warm tier configuration for any pool
    #[must_use]
    pub fn auto_detect_warm(pool_name: &str) -> Self {
        let mut config = Self::warm_tier_default();
        config.pool_name = pool_name.to_string();
        config.dataset_prefix = TIER_WARM.to_string();
        config
    }

    /// Auto-detect cold tier configuration for any pool
    #[must_use]
    pub fn auto_detect_cold(pool_name: &str) -> Self {
        let mut config = Self::cold_tier_default();
        config.pool_name = pool_name.to_string();
        config.dataset_prefix = TIER_COLD.to_string();
        config
    }

    /// Validate tier configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err(ERROR_TIER_NAME_EMPTY.to_string());
        }
        if self.pool_name.is_empty() {
            return Err(ERROR_POOL_NAME_EMPTY.to_string());
        }
        if self.dataset_prefix.is_empty() {
            return Err(ERROR_DATASET_PREFIX_EMPTY.to_string());
        }
        Ok(())
    }
}
