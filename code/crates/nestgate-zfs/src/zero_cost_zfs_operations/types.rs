// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **ZERO-COST ZFS TYPES**
//! High-performance data structures for ZFS operations

use nestgate_core::canonical_types::StorageTier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Type alias for thread-safe pool information map.
pub type PoolInfoMap = Arc<RwLock<HashMap<String, ZeroCostPoolInfo>>>;
/// Type alias for Datasetinfomap
pub type DatasetInfoMap = Arc<RwLock<HashMap<String, ZeroCostDatasetInfo>>>;
/// Type alias for Snapshotinfomap
pub type SnapshotInfoMap = Arc<RwLock<HashMap<String, ZeroCostSnapshotInfo>>>;

/// **ZERO-COST POOL INFORMATION**
/// High-performance pool data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zerocostpoolinfo
pub struct ZeroCostPoolInfo {
    /// Name
    pub name: String,
    /// Size
    pub size: u64,
    /// Used
    pub used: u64,
    /// Available
    pub available: u64,
    /// Health
    pub health: String,
    /// Properties
    pub properties: HashMap<String, String>,
    #[serde(with = "serde_system_time")]
    /// Timestamp when this was created
    pub created_at: std::time::SystemTime,
}

/// **ZERO-COST DATASET INFORMATION**
/// High-performance dataset data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zerocostdatasetinfo
pub struct ZeroCostDatasetInfo {
    /// Name
    pub name: String,
    /// Pool
    pub pool: String,
    /// Tier
    pub tier: StorageTier,
    /// Size
    pub size: u64,
    /// Used
    pub used: u64,
    /// Properties
    pub properties: HashMap<String, String>,
    /// Mount Point
    pub mount_point: Option<PathBuf>,
    #[serde(with = "serde_system_time")]
    /// Timestamp when this was created
    pub created_at: std::time::SystemTime,
}

/// **ZERO-COST SNAPSHOT INFORMATION**
/// High-performance snapshot data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zerocostsnapshotinfo
pub struct ZeroCostSnapshotInfo {
    /// Name
    pub name: String,
    /// Dataset
    pub dataset: String,
    /// Size
    pub size: u64,
    #[serde(with = "serde_system_time")]
    /// Timestamp when this was created
    pub created_at: std::time::SystemTime,
    /// Properties
    pub properties: HashMap<String, String>,
}

/// Helper module for serializing `SystemTime`
mod serde_system_time {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Type alias for `SerializeResult`
    type SerializeResult<S> = Result<<S as Serializer>::Ok, <S as Serializer>::Error>;

    /// Serialize
    pub fn serialize<S>(time: &SystemTime, serializer: S) -> SerializeResult<S>
    where
        S: Serializer,
    {
        let duration = time
            .duration_since(UNIX_EPOCH)
            .map_err(serde::ser::Error::custom)?;
        duration.as_secs().serialize(serializer)
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + std::time::Duration::from_secs(secs))
    }
}
