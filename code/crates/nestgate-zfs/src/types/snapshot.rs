//! ZFS snapshot-related types
//!
//! Domain: Snapshot information, creation, management

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// ZFS snapshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    /// Snapshot name (dataset@snapshot)
    pub name: String,
    /// Parent dataset name
    pub dataset: String,
    /// Snapshot creation time
    pub created_at: SystemTime,
    /// Space used by this snapshot in bytes
    pub used: u64,
    /// Snapshot size in bytes
    pub size: u64,
    /// Referenced data size in bytes
    pub referenced: u64,
    /// ZFS properties for this snapshot
    pub properties: std::collections::HashMap<String, String>,
}

impl Default for SnapshotInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            dataset: String::new(),
            created_at: SystemTime::now(),
            used: 0,
            size: 0,
            referenced: 0,
            properties: std::collections::HashMap::new(),
        }
    }
}

/// Options for creating snapshots
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SnapshotOptions {
    /// Whether to create snapshots of child datasets recursively
    pub recursive: bool,
    /// Custom properties to set on snapshot
    pub properties: std::collections::HashMap<String, String>,
}
