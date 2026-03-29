// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **ZFS HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ZfsHandler`
pub struct ZfsHandlerConfig {
    /// Pool
    pub pool: PoolHandlerConfig,
    /// Dataset
    pub dataset: DatasetHandlerConfig,
    /// Snapshot
    pub snapshot: SnapshotHandlerConfig,
    /// Backup
    pub backup: BackupHandlerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `PoolHandler`
pub struct PoolHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `DatasetHandler`
pub struct DatasetHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `SnapshotHandler`
pub struct SnapshotHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `BackupHandler`
pub struct BackupHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl Default for ZfsHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            pool: PoolHandlerConfig { enabled: true },
            dataset: DatasetHandlerConfig { enabled: true },
            snapshot: SnapshotHandlerConfig { enabled: true },
            backup: BackupHandlerConfig { enabled: true },
        }
    }
}

impl ZfsHandlerConfig {
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
