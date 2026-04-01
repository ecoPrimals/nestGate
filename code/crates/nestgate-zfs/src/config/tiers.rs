// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Configuration for hot/warm/cold storage tiers, performance profiles, and migration rules.

//! Tiers module

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Migration module not yet implemented - using local types
use crate::automation::policies::MigrationRules;

// Temporary local definition until migration module is implemented
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Capacitylimits
pub struct CapacityLimits {
    /// Max Size in gigabytes
    pub max_size_gb: u64,
    /// Min Free Space in gigabytes
    pub min_free_space_gb: u64,
    /// Warning Threshold Percent
    pub warning_threshold_percent: f64,
}

impl CapacityLimits {
    /// Returns hot tier default capacity limits.
    #[must_use]
    pub const fn hot_tier_defaults() -> Self {
        use crate::constants::{
            HOT_TIER_MAX_SIZE_GB, HOT_TIER_MIN_FREE_GB, HOT_TIER_WARNING_THRESHOLD,
        };
        Self {
            max_size_gb: HOT_TIER_MAX_SIZE_GB,
            min_free_space_gb: HOT_TIER_MIN_FREE_GB,
            warning_threshold_percent: HOT_TIER_WARNING_THRESHOLD,
        }
    }

    /// Returns warm tier default capacity limits.
    #[must_use]
    pub const fn warm_tier_defaults() -> Self {
        use crate::constants::{
            WARM_TIER_MAX_SIZE_GB, WARM_TIER_MIN_FREE_GB, WARM_TIER_WARNING_THRESHOLD,
        };
        Self {
            max_size_gb: WARM_TIER_MAX_SIZE_GB,
            min_free_space_gb: WARM_TIER_MIN_FREE_GB,
            warning_threshold_percent: WARM_TIER_WARNING_THRESHOLD,
        }
    }

    /// Returns cold tier default capacity limits.
    #[must_use]
    pub const fn cold_tier_defaults() -> Self {
        use crate::constants::{
            COLD_TIER_MAX_SIZE_GB, COLD_TIER_MIN_FREE_GB, COLD_TIER_WARNING_THRESHOLD,
        };
        Self {
            max_size_gb: COLD_TIER_MAX_SIZE_GB,
            min_free_space_gb: COLD_TIER_MIN_FREE_GB,
            warning_threshold_percent: COLD_TIER_WARNING_THRESHOLD,
        }
    }
}

// Use canonical constants system
use nestgate_core::canonical_modernization::canonical_constants::storage::{
    COMPRESSION_LZ4, TIER_COLD, TIER_HOT, TIER_WARM,
};
// Define missing constants locally
const POOL_DEFAULT: &str = "default";
/// Pool Production
const POOL_PRODUCTION: &str = "production";
/// Compression Off
const COMPRESSION_OFF: &str = "off";

// Define missing ZFS constants locally until they're added to canonical constants
const RECORDSIZE_PROPERTY: &str = "recordsize";
use nestgate_core::canonical_modernization::canonical_constants::zfs::{
    RECORDSIZE_1M, RECORDSIZE_64K, RECORDSIZE_128K,
};
/// Atime Property
const ATIME_PROPERTY: &str = "atime";
/// Primarycache Property
const PRIMARYCACHE_PROPERTY: &str = "primarycache";
/// Cache Metadata
const CACHE_METADATA: &str = "metadata";
/// Cache All
const CACHE_ALL: &str = "all";
/// Cache None
const CACHE_NONE: &str = "none";
/// Secondarycache Property
const SECONDARYCACHE_PROPERTY: &str = "secondarycache";
/// Sync Property
const SYNC_PROPERTY: &str = "sync";
/// Compression Property
const COMPRESSION_PROPERTY: &str = "compression";
/// Compression Zstd
const COMPRESSION_ZSTD: &str = "zstd";
/// Compression Gzip 9
const COMPRESSION_GZIP_9: &str = "gzip-9";
/// Logbias Property
const LOGBIAS_PROPERTY: &str = "logbias";
/// Bias Latency
const BIAS_LATENCY: &str = "latency";
/// Bias Throughput
const BIAS_THROUGHPUT: &str = "throughput";
/// Dedup Property
const DEDUP_PROPERTY: &str = "dedup";
/// Value Always
const VALUE_ALWAYS: &str = "always";
/// Value On
const VALUE_ON: &str = "on";
/// Value Off
const VALUE_OFF: &str = "off";
/// Value Standard
const VALUE_STANDARD: &str = "standard";

// Local ZFS-specific constants not in centralized system
const ERROR_TIER_NAME_EMPTY: &str = "Tier name cannot be empty";
/// Error Pool Name Empty
const ERROR_POOL_NAME_EMPTY: &str = "Pool name cannot be empty";
/// Error Dataset Prefix Empty
const ERROR_DATASET_PREFIX_EMPTY: &str = "Dataset prefix cannot be empty";

#[inline]
fn str_string_map<const N: usize>(pairs: [(&str, &str); N]) -> HashMap<String, String> {
    pairs
        .into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect()
}

/// Tier-specific configurations for hot/warm/cold storage
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tierconfigurations
pub struct TierConfigurations {
    /// Hot
    pub hot: TierConfig,
    /// Warm
    pub warm: TierConfig,
    /// Cold
    pub cold: TierConfig,
}
/// Individual tier configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Tier
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
/// Performanceprofile
pub enum PerformanceProfile {
    /// High-performance tier - optimized for speed
    HighPerformance, // Hot tier - optimized for speed
    /// Balanced tier - balance of speed and compression
    Balanced, // Warm tier - balance of speed and compression
    /// High-compression tier - optimized for space efficiency
    HighCompression, // Cold tier - optimized for space efficiency
}
impl Default for TierConfigurations {
    /// Returns the default instance
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
        let properties = str_string_map([
            (COMPRESSION_PROPERTY, COMPRESSION_OFF),
            (RECORDSIZE_PROPERTY, RECORDSIZE_128K),
            (ATIME_PROPERTY, VALUE_OFF),
            (PRIMARYCACHE_PROPERTY, CACHE_ALL),
            (SECONDARYCACHE_PROPERTY, CACHE_ALL),
        ]);

        Self {
            name: TIER_HOT.into(),
            pool_name: POOL_DEFAULT.into(),
            dataset_prefix: TIER_HOT.into(),
            properties,
            performance_profile: PerformanceProfile::HighPerformance,
            migration_rules: MigrationRules::hot_tier_defaults(),
            capacity_limits: CapacityLimits::hot_tier_defaults(),
        }
    }

    /// Default configuration for warm tier (balanced)
    #[must_use]
    pub fn warm_tier_default() -> Self {
        let properties = str_string_map([
            (COMPRESSION_PROPERTY, COMPRESSION_LZ4),
            (RECORDSIZE_PROPERTY, RECORDSIZE_1M),
            (ATIME_PROPERTY, VALUE_ON),
            (PRIMARYCACHE_PROPERTY, CACHE_METADATA),
            (SECONDARYCACHE_PROPERTY, CACHE_METADATA),
        ]);

        Self {
            name: TIER_WARM.into(),
            pool_name: POOL_DEFAULT.into(),
            dataset_prefix: TIER_WARM.into(),
            properties,
            performance_profile: PerformanceProfile::Balanced,
            migration_rules: MigrationRules::warm_tier_defaults(),
            capacity_limits: CapacityLimits::warm_tier_defaults(),
        }
    }

    /// Default configuration for cold tier (high compression)
    #[must_use]
    pub fn cold_tier_default() -> Self {
        let properties = str_string_map([
            (COMPRESSION_PROPERTY, COMPRESSION_ZSTD),
            (RECORDSIZE_PROPERTY, RECORDSIZE_1M),
            (ATIME_PROPERTY, VALUE_OFF),
            (PRIMARYCACHE_PROPERTY, CACHE_METADATA),
            (SECONDARYCACHE_PROPERTY, CACHE_NONE),
            (SYNC_PROPERTY, VALUE_ALWAYS),
        ]);

        Self {
            name: TIER_COLD.into(),
            pool_name: POOL_DEFAULT.into(),
            dataset_prefix: TIER_COLD.into(),
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
        config.pool_name = POOL_PRODUCTION.into();

        // Performance-optimized properties for hot tier
        config
            .properties
            .insert(RECORDSIZE_PROPERTY.into(), RECORDSIZE_64K.into());
        config
            .properties
            .insert(COMPRESSION_PROPERTY.into(), COMPRESSION_LZ4.into());
        config
            .properties
            .insert(PRIMARYCACHE_PROPERTY.into(), CACHE_ALL.into());
        config
            .properties
            .insert(SECONDARYCACHE_PROPERTY.into(), CACHE_ALL.into());
        config
            .properties
            .insert(LOGBIAS_PROPERTY.into(), BIAS_LATENCY.into());
        config
            .properties
            .insert(SYNC_PROPERTY.into(), VALUE_STANDARD.into());

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
        config.pool_name = POOL_PRODUCTION.into();

        // Balanced properties for warm tier
        config
            .properties
            .insert(RECORDSIZE_PROPERTY.into(), RECORDSIZE_128K.into());
        config
            .properties
            .insert(COMPRESSION_PROPERTY.into(), COMPRESSION_ZSTD.into());
        config
            .properties
            .insert(PRIMARYCACHE_PROPERTY.into(), CACHE_METADATA.into());
        config
            .properties
            .insert(SECONDARYCACHE_PROPERTY.into(), CACHE_METADATA.into());
        config
            .properties
            .insert(LOGBIAS_PROPERTY.into(), BIAS_THROUGHPUT.into());

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
        config.pool_name = POOL_PRODUCTION.into();

        // Space-optimized properties for cold tier
        config
            .properties
            .insert(RECORDSIZE_PROPERTY.into(), RECORDSIZE_1M.into());
        config
            .properties
            .insert(COMPRESSION_PROPERTY.into(), COMPRESSION_GZIP_9.into());
        config
            .properties
            .insert(PRIMARYCACHE_PROPERTY.into(), CACHE_METADATA.into());
        config
            .properties
            .insert(SECONDARYCACHE_PROPERTY.into(), CACHE_NONE.into());
        config
            .properties
            .insert(LOGBIAS_PROPERTY.into(), BIAS_THROUGHPUT.into());
        config
            .properties
            .insert(DEDUP_PROPERTY.into(), VALUE_ON.into());

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
        config.pool_name = pool_name.into();
        config.dataset_prefix = TIER_HOT.into();
        config
    }

    /// Auto-detect warm tier configuration for any pool
    #[must_use]
    pub fn auto_detect_warm(pool_name: &str) -> Self {
        let mut config = Self::warm_tier_default();
        config.pool_name = pool_name.into();
        config.dataset_prefix = TIER_WARM.into();
        config
    }

    /// Auto-detect cold tier configuration for any pool
    #[must_use]
    pub fn auto_detect_cold(pool_name: &str) -> Self {
        let mut config = Self::cold_tier_default();
        config.pool_name = pool_name.into();
        config.dataset_prefix = TIER_COLD.into();
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
    pub const fn validate(&self) -> Result<(), &'static str> {
        if self.name.is_empty() {
            return Err(ERROR_TIER_NAME_EMPTY);
        }
        if self.pool_name.is_empty() {
            return Err(ERROR_POOL_NAME_EMPTY);
        }
        if self.dataset_prefix.is_empty() {
            return Err(ERROR_DATASET_PREFIX_EMPTY);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacity_limits_tier_defaults_are_nonzero() {
        let h = CapacityLimits::hot_tier_defaults();
        assert!(h.max_size_gb > 0);
        let w = CapacityLimits::warm_tier_defaults();
        assert!(w.min_free_space_gb > 0);
        let c = CapacityLimits::cold_tier_defaults();
        assert!(c.warning_threshold_percent >= 0.0);
    }

    #[test]
    fn tier_configurations_default_has_three_named_tiers() {
        let t = TierConfigurations::default();
        assert!(!t.hot.name.is_empty());
        assert!(!t.warm.name.is_empty());
        assert!(!t.cold.name.is_empty());
        t.hot.validate().expect("test: hot valid");
        t.warm.validate().expect("test: warm valid");
        t.cold.validate().expect("test: cold valid");
    }

    #[test]
    fn tier_configurations_production_and_auto_detect() {
        let p = TierConfigurations::production_tiers();
        assert_eq!(p.hot.pool_name, "production");
        let a = TierConfigurations::auto_detect_tiers("mypool");
        assert_eq!(a.hot.pool_name, "mypool");
        assert_eq!(a.warm.pool_name, "mypool");
        assert_eq!(a.cold.pool_name, "mypool");
    }

    #[test]
    fn tier_config_hot_warm_cold_defaults_have_expected_profiles() {
        assert!(matches!(
            TierConfig::hot_tier_default().performance_profile,
            PerformanceProfile::HighPerformance
        ));
        assert!(matches!(
            TierConfig::warm_tier_default().performance_profile,
            PerformanceProfile::Balanced
        ));
        assert!(matches!(
            TierConfig::cold_tier_default().performance_profile,
            PerformanceProfile::HighCompression
        ));
    }

    #[test]
    fn tier_config_production_overrides_pool_and_properties() {
        let hot = TierConfig::hot_tier_production();
        assert_eq!(hot.pool_name, "production");
        assert!(hot.properties.contains_key("logbias"));

        let warm = TierConfig::warm_tier_production();
        assert_eq!(warm.migration_rules.age_threshold_days, 30);

        let cold = TierConfig::cold_tier_production();
        assert!(cold.properties.contains_key("dedup"));
    }

    #[test]
    fn tier_config_validate_errors() {
        let mut bad = TierConfig::hot_tier_default();
        bad.name.clear();
        assert_eq!(
            bad.validate().expect_err("test: empty name"),
            ERROR_TIER_NAME_EMPTY
        );

        let mut bad_pool = TierConfig::warm_tier_default();
        bad_pool.pool_name.clear();
        assert_eq!(
            bad_pool.validate().expect_err("test: empty pool"),
            ERROR_POOL_NAME_EMPTY
        );

        let mut bad_prefix = TierConfig::cold_tier_default();
        bad_prefix.dataset_prefix.clear();
        assert_eq!(
            bad_prefix.validate().expect_err("test: empty prefix"),
            ERROR_DATASET_PREFIX_EMPTY
        );
    }

    #[test]
    fn tier_config_serde_roundtrip() {
        let cfg = TierConfigurations::default();
        let json = serde_json::to_string(&cfg).expect("test: serialize tiers");
        let back: TierConfigurations =
            serde_json::from_str(&json).expect("test: deserialize tiers");
        assert_eq!(back.hot.name, cfg.hot.name);
        assert_eq!(back.cold.pool_name, cfg.cold.pool_name);
    }
}
