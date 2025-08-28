//
// Configuration for hot/warm/cold storage tiers, performance profiles, and migration rules.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::migration::{CapacityLimits, MigrationRules};

// Use canonical constants system  
use nestgate_core::canonical_modernization::canonical_constants::storage::{
    TIER_HOT, TIER_WARM, TIER_COLD, COMPRESSION_LZ4, COMPRESSION_GZIP,
};
use crate::canonical_modernization::canonical_constants::COMPRESSION_OFF;

// Define missing constants locally
const POOL_DEFAULT: &str = "default";
const POOL_PRODUCTION: &str = "production";

// Define missing ZFS constants locally until they're added to canonical constants
const RECORDSIZE_PROPERTY: &str = "recordsize";
use nestgate_core::canonical_modernization::canonical_constants::zfs::{RECORDSIZE_1M, RECORDSIZE_64K, RECORDSIZE_128K};
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
        config.migration_rules.min_age_hours = 7 * 24; // 7 days in hours
        config.migration_rules.access_threshold = 10;
        config.migration_rules.auto_migration_enabled = true;

        config
    }

    /// Create production-optimized warm tier configuration
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
        config.migration_rules.min_age_hours = 30 * 24; // 30 days in hours
        config.migration_rules.access_threshold = 2;
        config.migration_rules.auto_migration_enabled = true;

        config
    }

    /// Create production-optimized cold tier configuration
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
        config.migration_rules.min_age_hours = 90 * 24; // 90 days in hours
        config.migration_rules.access_threshold = 1;
        config.migration_rules.auto_migration_enabled = true;

        config
    }

    /// Auto-detect hot tier configuration for any pool
    pub fn auto_detect_hot(pool_name: &str) -> Self {
        let mut config = Self::hot_tier_default();
        config.pool_name = pool_name.to_string();
        config.dataset_prefix = TIER_HOT.to_string();
        config
    }

    /// Auto-detect warm tier configuration for any pool
    pub fn auto_detect_warm(pool_name: &str) -> Self {
        let mut config = Self::warm_tier_default();
        config.pool_name = pool_name.to_string();
        config.dataset_prefix = TIER_WARM.to_string();
        config
    }

    /// Auto-detect cold tier configuration for any pool
    pub fn auto_detect_cold(pool_name: &str) -> Self {
        let mut config = Self::cold_tier_default();
        config.pool_name = pool_name.to_string();
        config.dataset_prefix = TIER_COLD.to_string();
        config
    }

    /// Validate tier configuration
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
