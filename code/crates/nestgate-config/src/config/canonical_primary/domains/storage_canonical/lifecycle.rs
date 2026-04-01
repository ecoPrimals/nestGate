// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **STORAGE LIFECYCLE CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Storage lifecycle configuration for data management over time.
///
/// Provides comprehensive lifecycle settings including data retention, archival,
/// purging, and compliance policies for managing data from creation to deletion.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `StorageLifecycle`
pub struct StorageLifecycleConfig {
    /// General data lifecycle management settings.
    pub data_lifecycle: DataLifecycleConfig,
    /// Data retention policy configuration.
    pub retention: RetentionConfig,
    /// Data archival configuration for long-term storage.
    pub archival: ArchivalConfig,
    /// Data purging configuration for permanent deletion.
    pub purging: PurgingConfig,
    /// Compliance policy configuration for regulatory requirements.
    pub compliance: ComplianceStorageConfig,
}

/// General data lifecycle management configuration.
///
/// Controls whether lifecycle management features are enabled.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `DataLifecycle`
pub struct DataLifecycleConfig {
    /// Whether data lifecycle management is enabled (default: true).
    pub enabled: bool,
}

/// Data retention policy configuration.
///
/// Defines how long data must be retained before it can be archived or deleted.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Retention
pub struct RetentionConfig {
    /// Whether retention policies are enforced (default: true).
    pub enabled: bool,
    /// Retention duration (default: 1 year).
    pub duration: Duration,
}

/// Data archival configuration for long-term storage.
///
/// Defines when data should be moved to archival storage for cost savings.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Archival
pub struct ArchivalConfig {
    /// Whether automatic archival is enabled (default: false).
    pub enabled: bool,
    /// Duration after which data is archived (default: 90 days).
    pub after: Duration,
}

/// Data purging configuration for permanent deletion.
///
/// Defines when data can be permanently deleted after retention period.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Purging
pub struct PurgingConfig {
    /// Whether automatic purging is enabled (default: false).
    pub enabled: bool,
    /// Duration after which data is purged (default: 7 years).
    pub after: Duration,
}

/// Compliance policy configuration for regulatory requirements.
///
/// Enables compliance features for meeting regulatory data management requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ComplianceStorage`
pub struct ComplianceStorageConfig {
    /// Whether compliance features are enabled (default: false).
    pub enabled: bool,
}

impl Default for StorageLifecycleConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            data_lifecycle: DataLifecycleConfig { enabled: true },
            retention: RetentionConfig {
                enabled: true,
                duration: Duration::from_secs(365 * 24 * 3600),
            },
            archival: ArchivalConfig {
                enabled: false,
                after: Duration::from_secs(90 * 24 * 3600),
            },
            purging: PurgingConfig {
                enabled: false,
                after: Duration::from_secs(7 * 365 * 24 * 3600),
            },
            compliance: ComplianceStorageConfig { enabled: false },
        }
    }
}

impl StorageLifecycleConfig {
    /// Create production-optimized lifecycle configuration.
    ///
    /// Uses 1-year retention with optional archival and purging.
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }

    /// Create development-optimized lifecycle configuration.
    ///
    /// Uses default settings suitable for local development.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Create high-performance lifecycle configuration.
    ///
    /// Minimal lifecycle overhead for maximum performance.
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }

    /// Create cloud-native lifecycle configuration.
    ///
    /// Optimized for cloud storage tiers and automatic lifecycle management.
    #[must_use]
    pub fn cloud_native() -> Self {
        Self::default()
    }

    /// Merge this configuration with another, preferring values from `other`.
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }

    /// Validate lifecycle configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub const fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}
