//! **ZERO-COST ZFS TYPES**
//! High-performance data structures for ZFS operations

use nestgate_core::canonical_types::StorageTier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

// Type aliases for complex types to improve readability
pub type PoolInfoMap = Arc<RwLock<HashMap<String, ZeroCostPoolInfo>>>;
pub type DatasetInfoMap = Arc<RwLock<HashMap<String, ZeroCostDatasetInfo>>>;
pub type SnapshotInfoMap = Arc<RwLock<HashMap<String, ZeroCostSnapshotInfo>>>;

/// **ZERO-COST POOL INFORMATION**
/// High-performance pool data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostPoolInfo {
    pub name: String,
    pub size: u64,
    pub used: u64,
    pub available: u64,
    pub health: String,
    pub properties: HashMap<String, String>,
    #[serde(with = "serde_system_time")]
    pub created_at: std::time::SystemTime,
}

/// **ZERO-COST DATASET INFORMATION**
/// High-performance dataset data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostDatasetInfo {
    pub name: String,
    pub pool: String,
    pub tier: StorageTier,
    pub size: u64,
    pub used: u64,
    pub properties: HashMap<String, String>,
    pub mount_point: Option<PathBuf>,
    #[serde(with = "serde_system_time")]
    pub created_at: std::time::SystemTime,
}

/// **ZERO-COST SNAPSHOT INFORMATION**
/// High-performance snapshot data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostSnapshotInfo {
    pub name: String,
    pub dataset: String,
    pub size: u64,
    #[serde(with = "serde_system_time")]
    pub created_at: std::time::SystemTime,
    pub properties: HashMap<String, String>,
}

/// Helper module for serializing SystemTime
mod serde_system_time {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    type SerializeResult<S> = Result<<S as Serializer>::Ok, <S as Serializer>::Error>;

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

