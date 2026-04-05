// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **PUBLIC DATA TYPES**
//!
//! Object storage data structures for pools, datasets, snapshots, and properties.

use nestgate_core::canonical_types::StorageTier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::provider::StorageProvider;

/// Object storage pool (S3 bucket)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectPool {
    /// Pool name
    pub name: String,
    /// Bucket name
    pub bucket: String,
    /// Creation time
    pub created_at: std::time::SystemTime,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Object storage dataset (prefix)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectDataset {
    /// Dataset name
    pub name: String,
    /// Pool name
    pub pool: String,
    /// Object prefix
    pub prefix: String,
    /// Storage tier
    pub tier: StorageTier,
    /// Creation time
    pub created_at: std::time::SystemTime,
}

/// Object storage snapshot (versioned object or copy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectSnapshot {
    /// Snapshot name
    pub name: String,
    /// Dataset name
    pub dataset: String,
    /// Snapshot identifier
    pub snapshot_id: String,
    /// Creation time
    pub created_at: std::time::SystemTime,
}

/// Object storage properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectProperties {
    /// Storage endpoint
    pub endpoint: String,
    /// Region
    pub region: String,
    /// Provider (detected from endpoint)
    pub provider: StorageProvider,
    /// Versioning enabled
    pub versioning: bool,
    /// Encryption enabled
    pub encryption: bool,
    /// Custom properties
    pub custom: HashMap<String, String>,
}
