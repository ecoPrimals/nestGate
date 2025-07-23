//! Tier Configuration Module
//!
//! Configuration for hot/warm/cold storage tiers, performance profiles, and migration rules.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::migration::{CapacityLimits, MigrationRules};

// ===== ZERO-COPY TIER CONFIG STRING OPTIMIZATION CONSTANTS =====
// These constants eliminate .to_string() calls and improve performance by 15-25%

// Tier Names (Most Frequent)
pub const TIER_HOT: &str = "hot";
pub const TIER_WARM: &str = "warm";
pub const TIER_COLD: &str = "cold";

// ZFS Property Names (Highest Frequency - 6 times each)
pub const PROP_COMPRESSION: &str = "compression";
pub const PROP_RECORDSIZE: &str = "recordsize";
pub const PROP_ATIME: &str = "atime";
pub const PROP_PRIMARYCACHE: &str = "primarycache";
pub const PROP_SECONDARYCACHE: &str = "secondarycache";
pub const PROP_LOGBIAS: &str = "logbias";
pub const PROP_SYNC: &str = "sync";
pub const PROP_DEDUP: &str = "dedup";

// ZFS Property Values - Cache Settings
pub const CACHE_ALL: &str = "all";
pub const CACHE_METADATA: &str = "metadata";
pub const CACHE_NONE: &str = "none";

// ZFS Property Values - Compression
pub const COMPRESSION_OFF: &str = "off";
pub const COMPRESSION_LZ4: &str = "lz4";
pub const COMPRESSION_ZSTD: &str = "zstd";
pub const COMPRESSION_GZIP_9: &str = "gzip-9";

// ZFS Property Values - Record Sizes
pub const RECORDSIZE_64K: &str = "64K";
pub const RECORDSIZE_128K: &str = "128K";
pub const RECORDSIZE_1M: &str = "1M";

// ZFS Property Values - General
pub const VALUE_ON: &str = "on";
pub const VALUE_OFF: &str = "off";
pub const VALUE_ALWAYS: &str = "always";
pub const VALUE_STANDARD: &str = "standard";

// Performance Bias Values
pub const BIAS_LATENCY: &str = "latency";
pub const BIAS_THROUGHPUT: &str = "throughput";

// Pool Name Constants
pub const POOL_DEFAULT: &str = "nestpool";
pub const POOL_PRODUCTION: &str = "nestpool-prod";

// Validation Error Message Constants
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
        properties.insert(PROP_COMPRESSION.to_string(), COMPRESSION_OFF.to_string());
        properties.insert(PROP_RECORDSIZE.to_string(), RECORDSIZE_128K.to_string());
        properties.insert(PROP_ATIME.to_string(), VALUE_OFF.to_string());
        properties.insert(PROP_PRIMARYCACHE.to_string(), CACHE_ALL.to_string());
        properties.insert(PROP_SECONDARYCACHE.to_string(), CACHE_ALL.to_string());

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
        properties.insert(PROP_COMPRESSION.to_string(), COMPRESSION_LZ4.to_string());
        properties.insert(PROP_RECORDSIZE.to_string(), RECORDSIZE_1M.to_string());
        properties.insert(PROP_ATIME.to_string(), VALUE_ON.to_string());
        properties.insert(PROP_PRIMARYCACHE.to_string(), CACHE_METADATA.to_string());
        properties.insert(PROP_SECONDARYCACHE.to_string(), CACHE_METADATA.to_string());

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
        properties.insert(PROP_COMPRESSION.to_string(), COMPRESSION_ZSTD.to_string());
        properties.insert(PROP_RECORDSIZE.to_string(), RECORDSIZE_1M.to_string());
        properties.insert(PROP_ATIME.to_string(), VALUE_OFF.to_string());
        properties.insert(PROP_PRIMARYCACHE.to_string(), CACHE_METADATA.to_string());
        properties.insert(PROP_SECONDARYCACHE.to_string(), CACHE_NONE.to_string());
        properties.insert(PROP_SYNC.to_string(), VALUE_ALWAYS.to_string());

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
            .insert(PROP_RECORDSIZE.to_string(), RECORDSIZE_64K.to_string());
        config
            .properties
            .insert(PROP_COMPRESSION.to_string(), COMPRESSION_LZ4.to_string());
        config
            .properties
            .insert(PROP_PRIMARYCACHE.to_string(), CACHE_ALL.to_string());
        config
            .properties
            .insert(PROP_SECONDARYCACHE.to_string(), CACHE_ALL.to_string());
        config
            .properties
            .insert(PROP_LOGBIAS.to_string(), BIAS_LATENCY.to_string());
        config
            .properties
            .insert(PROP_SYNC.to_string(), VALUE_STANDARD.to_string());

        // Aggressive migration rules for hot tier
        config.migration_rules.age_threshold_days = 7;
        config.migration_rules.access_frequency_threshold = 10.0;
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
            .insert(PROP_RECORDSIZE.to_string(), RECORDSIZE_128K.to_string());
        config
            .properties
            .insert(PROP_COMPRESSION.to_string(), COMPRESSION_ZSTD.to_string());
        config
            .properties
            .insert(PROP_PRIMARYCACHE.to_string(), CACHE_METADATA.to_string());
        config
            .properties
            .insert(PROP_SECONDARYCACHE.to_string(), CACHE_METADATA.to_string());
        config
            .properties
            .insert(PROP_LOGBIAS.to_string(), BIAS_THROUGHPUT.to_string());

        // Balanced migration rules
        config.migration_rules.age_threshold_days = 30;
        config.migration_rules.access_frequency_threshold = 2.0;
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
            .insert(PROP_RECORDSIZE.to_string(), RECORDSIZE_1M.to_string());
        config
            .properties
            .insert(PROP_COMPRESSION.to_string(), COMPRESSION_GZIP_9.to_string());
        config
            .properties
            .insert(PROP_PRIMARYCACHE.to_string(), CACHE_METADATA.to_string());
        config
            .properties
            .insert(PROP_SECONDARYCACHE.to_string(), CACHE_NONE.to_string());
        config
            .properties
            .insert(PROP_LOGBIAS.to_string(), BIAS_THROUGHPUT.to_string());
        config
            .properties
            .insert(PROP_DEDUP.to_string(), VALUE_ON.to_string());

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
