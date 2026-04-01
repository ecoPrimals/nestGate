// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS configuration types
//!
//! Domain: Pool creation, system configuration, re-exports from canonical config

use std::path::PathBuf;

/// Pool creation configuration
#[derive(Debug, Clone)]
pub struct PoolCreationConfig {
    /// Pool name
    pub name: String,
    /// Devices to use for the pool
    pub devices: Vec<PathBuf>,
    /// Mount point for the pool
    pub mountpoint: Option<PathBuf>,
    /// Pool features to enable
    pub features: Vec<String>,
    /// Initial pool properties
    pub properties: std::collections::HashMap<String, String>,
}

// Re-export canonical ZFS configuration types from nestgate-core
pub use nestgate_core::config::canonical_primary::domains::storage_canonical::{
    AlertThresholds, ArcCacheConfig, L2ArcConfig, PrefetchConfig, ZfsCompression, ZfsDatasetConfig,
    ZfsMaintenanceConfig, ZfsMigrationConfig, ZfsMonitoringConfig, ZfsPerformanceConfig,
    ZfsPoolConfig, ZfsPoolSettings, ZfsRedundancy, ZfsSecurityConfig, ZfsSnapshotConfig,
    ZfsStorageConfig, ZilConfig,
};

// Type alias for backward compatibility
pub use ZfsStorageConfig as CanonicalZfsConfig;
