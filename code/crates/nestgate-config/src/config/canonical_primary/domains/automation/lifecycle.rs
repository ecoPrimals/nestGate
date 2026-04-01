// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};

/// **LIFECYCLE CONFIGURATION**
///
/// Data lifecycle management settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Lifecycle
pub struct LifecycleConfig {
    /// Enable lifecycle management
    pub enabled: bool,

    /// Hot tier retention in days
    pub hot_retention_days: u32,

    /// Warm tier retention in days
    pub warm_retention_days: u32,

    /// Cold tier retention in days
    pub cold_retention_days: u32,

    /// Archive retention in days
    pub archive_retention_days: u32,

    /// Enable automatic tiering
    pub auto_tiering: bool,

    /// Tier transition rules
    pub transition_rules: Vec<String>,
}

impl Default for LifecycleConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development()
    }
}

impl LifecycleConfig {
    /// Creates a development-optimized configuration for data lifecycle management
    ///
    /// Returns a `LifecycleConfig` with lifecycle features disabled and shorter retention
    /// periods suitable for development environments.
    #[must_use]
    pub const fn development() -> Self {
        Self {
            enabled: false, // Disabled in dev
            hot_retention_days: 7,
            warm_retention_days: 30,
            cold_retention_days: 90,
            archive_retention_days: 365,
            auto_tiering: false,
            transition_rules: vec![],
        }
    }

    /// Creates a production-hardened configuration for data lifecycle management
    ///
    /// Returns a `LifecycleConfig` with lifecycle enabled, auto-tiering, longer retention
    /// periods, and comprehensive transition rules for production workloads.
    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            hot_retention_days: 30,
            warm_retention_days: 90,
            cold_retention_days: 365,
            archive_retention_days: 1825, // 5 years
            auto_tiering: true,
            transition_rules: vec![
                "hot_to_warm: age > 30d".to_string(),
                "warm_to_cold: age > 90d".to_string(),
                "cold_to_archive: age > 365d".to_string(),
            ],
        }
    }
}
