// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Core dataset types and manager handle.

use crate::config::ZfsConfig;
use crate::pool::ZfsPoolManager;
use nestgate_core::canonical_types::StorageTier as CoreStorageTier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Dataset information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Datasetinfo
pub struct DatasetInfo {
    /// Dataset name
    pub name: String,
    /// Used space in bytes
    pub used_space: u64,
    /// Available space in bytes
    pub available_space: u64,
    /// File count (optional)
    pub file_count: Option<u64>,
    /// Compression ratio (optional)
    pub compression_ratio: Option<f64>,
    /// Mount point
    pub mount_point: String,
    /// Storage tier
    pub tier: CoreStorageTier,
    /// Properties
    pub properties: HashMap<String, String>,
}

/// ZFS Dataset Manager - handles dataset operations
#[derive(Debug)]
/// Manager for `ZfsDataset` operations
pub struct ZfsDatasetManager {
    pub(super) config: Arc<ZfsConfig>,
    pub(super) pool_manager: Arc<ZfsPoolManager>,
}

impl ZfsDatasetManager {
    /// Create a new ZFS dataset manager
    #[must_use]
    pub fn new(config: ZfsConfig, pool_manager: Arc<ZfsPoolManager>) -> Self {
        Self {
            config: Arc::new(config),
            pool_manager,
        }
    }

    /// Create a new ZFS dataset manager with shared config (zero-copy)
    #[must_use]
    pub const fn with_shared_config(
        config: Arc<ZfsConfig>,
        pool_manager: Arc<ZfsPoolManager>,
    ) -> Self {
        Self {
            config,
            pool_manager,
        }
    }
}
