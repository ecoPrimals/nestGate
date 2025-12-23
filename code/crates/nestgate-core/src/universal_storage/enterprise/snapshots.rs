use std::collections::HashMap;
//
// This module handles snapshot creation, management, and restoration functionality
// for enterprise storage systems.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Snapshot information and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Snapshotinfo
pub struct SnapshotInfo {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Size Bytes
    pub size_bytes: u64,
    /// Count of file
    pub file_count: u32,
    /// Human-readable description
    pub description: Option<String>,
    /// Tags
    pub tags: HashMap<String, String>,
    /// Checksum
    pub checksum: String,
}
impl SnapshotInfo {
    #[must_use]
    pub fn new(name: String, description: Option<String>) -> Self {
        let id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: id.clone(),
            name,
            created_at: SystemTime::now(),
            size_bytes: 0,
            file_count: 0,
            description,
            tags: HashMap::new(),
            checksum: String::new(),
        }
    }

    /// Age
    pub fn age(&self) -> Duration {
        SystemTime::now()
            .duration_since(self.created_at)
            .unwrap_or_default()
    }

    /// Add Tag
    pub fn add_tag(&mut self, key: String, value: String) {
        self.tags.insert(key, value);
    }

    /// Gets Tag
    pub fn get_tag(&self, key: &str) -> Option<&String> {
        self.tags.get(key)
    }

    /// Updates  Size Info
    pub fn update_size_info(&mut self, size_bytes: u64, file_count: u32) {
        self.size_bytes = size_bytes;
        self.file_count = file_count;
    }

    /// Sets Checksum
    pub fn set_checksum(&mut self, checksum: String) {
        self.checksum = checksum;
    }
}
