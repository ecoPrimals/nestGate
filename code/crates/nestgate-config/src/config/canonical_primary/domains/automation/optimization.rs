// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};

/// **OPTIMIZATION CONFIGURATION**
///
/// System and resource optimization settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Optimization
pub struct OptimizationConfig {
    /// Enable optimization
    pub enabled: bool,

    /// Optimization frequency in hours
    pub frequency_hours: u32,

    /// Enable storage optimization
    pub storage_optimization: bool,

    /// Enable performance optimization
    pub performance_optimization: bool,

    /// Enable resource balancing
    pub resource_balancing: bool,

    /// Optimization target (e.g., "performance", "efficiency", "balance")
    pub target: String,

    /// Optimization aggressiveness (0-100)
    pub aggressiveness: u8,
}

impl Default for OptimizationConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development()
    }
}

impl OptimizationConfig {
    /// Creates a development-optimized configuration for system optimization
    ///
    /// Returns an `OptimizationConfig` with optimization features disabled and balanced
    /// settings suitable for development environments.
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: false,
            frequency_hours: 24,
            storage_optimization: false,
            performance_optimization: false,
            resource_balancing: false,
            target: "balance".to_string(),
            aggressiveness: 30,
        }
    }

    /// Creates a production-hardened configuration for system optimization
    ///
    /// Returns an `OptimizationConfig` with all optimization features enabled, frequent
    /// optimization runs, and aggressive settings for production workloads.
    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            frequency_hours: 6,
            storage_optimization: true,
            performance_optimization: true,
            resource_balancing: true,
            target: "efficiency".to_string(),
            aggressiveness: 70,
        }
    }
}
