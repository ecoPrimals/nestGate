// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **CACHING PERFORMANCE CONFIGURATION**

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};

/// Cache performance configuration for optimizing data caching.
///
/// Controls cache optimization and warming strategies for improved performance.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for CachePerformance
pub struct CachePerformanceConfig {
    /// Cache optimization settings.
    pub optimization: CacheOptimizationConfig,
    /// Cache warming configuration.
    pub warming: CacheWarmingConfig,
}

/// Cache optimization configuration.
///
/// Enables cache-specific performance optimizations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for CacheOptimization
pub struct CacheOptimizationConfig {
    /// Whether cache optimization is enabled.
    pub enabled: bool,
}

/// Cache warming configuration for preloading cache data.
///
/// Controls how and when cache data is preloaded to reduce cold-start latency.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for CacheWarming
pub struct CacheWarmingConfig {
    /// Whether cache warming is enabled.
    pub enabled: bool,
    /// Cache warming strategy to use.
    pub strategy: WarmingStrategy,
    /// Batch size for warming operations.
    pub batch_size: usize,
}

/// Cache warming strategy.
///
/// Determines when and how cache data is preloaded.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Warmingstrategy
pub enum WarmingStrategy {
    /// Eager - preload all cache data at startup.
    #[default]
    /// Eager
    Eager,
    /// Lazy - load cache data on first access.
    Lazy,
    /// Predictive - preload based on usage patterns.
    Predictive,
}

impl CachePerformanceConfig {
    /// Validate cache performance configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

// ==================== BACKWARD COMPATIBILITY ALIASES ====================

/// Backward compatibility alias for UnifiedCacheConfig
pub type UnifiedCacheConfig = CachePerformanceConfig;
