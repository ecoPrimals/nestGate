use std::collections::HashMap;
//
// This module handles snapshot creation, management, and restoration functionality
// for enterprise storage systems.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Snapshot information and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    pub id: String,
    pub name: String,
    pub created_at: SystemTime,
    pub size_bytes: u64,
    pub file_count: u32,
    pub description: Option<String>,
    pub tags: HashMap<String, String>,
    pub checksum: String,
    pub storage_path: PathBuf,
}

impl SnapshotInfo {
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
            storage_path: PathBuf::from(format!("snapshots/{timestamp}/{id}")),
        }
    }

    pub fn age(&self) -> Duration {
        SystemTime::now()
            .duration_since(self.created_at)
            .unwrap_or_default()
    }

    pub fn add_tag(&mut self, key: String, value: String) {
        self.tags.insert(key, value);
    }

    pub fn get_tag(&self, key: &str) -> Option<&String> {
        self.tags.get(key)
    }

    pub fn update_size_info(&mut self, size_bytes: u64, file_count: u32) {
        self.size_bytes = size_bytes;
        self.file_count = file_count;
    }

    pub fn set_checksum(&mut self, checksum: String) {
        self.checksum = checksum;
    }
}
