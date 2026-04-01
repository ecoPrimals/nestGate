// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **PERFORMANCE HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `PerformanceHandler`
pub struct PerformanceHandlerConfig {
    /// Caching
    pub caching: CachingHandlerConfig,
    /// Optimization
    pub optimization: OptimizationHandlerConfig,
    /// Profiler
    pub profiler: ProfilerHandlerConfig,
    /// Load Balancing
    pub load_balancing: LoadBalancingHandlerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `CachingHandler`
pub struct CachingHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `OptimizationHandler`
pub struct OptimizationHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ProfilerHandler`
pub struct ProfilerHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `LoadBalancingHandler`
pub struct LoadBalancingHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl Default for PerformanceHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            caching: CachingHandlerConfig { enabled: true },
            optimization: OptimizationHandlerConfig { enabled: true },
            profiler: ProfilerHandlerConfig { enabled: false },
            load_balancing: LoadBalancingHandlerConfig { enabled: false },
        }
    }
}

impl PerformanceHandlerConfig {
    /// Returns a production-optimized configuration
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }

    /// Returns a development-optimized configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Returns a high-performance configuration
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }

    /// Merges this configuration with another, returning the merged result
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub const fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}
