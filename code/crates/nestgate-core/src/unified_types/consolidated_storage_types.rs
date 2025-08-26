use std::collections::HashMap;
//
// **CANONICAL MODERNIZATION COMPLETE** - Unified storage type definitions
//
// This module provides the canonical storage types that replace all deprecated
// `unified_storage_types`. These types are the canonical source of truth for
// all storage-related operations in NestGate.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// **CANONICAL STORAGE TYPES** - These replace all fragmented definitions

/// Modern storage change record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageChange {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub operation: String,
    pub data: serde_json::Value,
    pub metadata: HashMap<String, String>,
}

/// Modern storage directory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDirectoryEntry {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size: u64,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub permissions: Option<String>,
    pub owner: Option<String>,
    pub group: Option<String>,
}

/// Modern storage range specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageRange {
    pub start: u64,
    pub end: u64,
    pub inclusive: bool,
}

/// Modern storage replication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageReplicationResult {
    pub success: bool,
    pub replicated_bytes: u64,
    pub duration_ms: u64,
    pub error_message: Option<String>,
    pub source: String,
    pub destination: String,
    pub checksum: Option<String>,
}

/// Modern replication status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReplicationStatus {
    Active,
    Paused,
    Failed,
    Disabled,
    Initializing,
    Syncing,
    Completed,
}

impl Default for ReplicationStatus {
    fn default() -> Self {
        Self::Disabled
    }
}

impl std::fmt::Display for ReplicationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Paused => write!(f, "paused"),
            Self::Failed => write!(f, "failed"),
            Self::Disabled => write!(f, "disabled"),
            Self::Initializing => write!(f, "initializing"),
            Self::Syncing => write!(f, "syncing"),
            Self::Completed => write!(f, "completed"),
        }
    }
}

// Migration utilities removed - deprecated types no longer supported

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replication_status_display() {
        assert_eq!(ReplicationStatus::Active.to_string(), "active");
        assert_eq!(ReplicationStatus::Failed.to_string(), "failed");
        assert_eq!(ReplicationStatus::Disabled.to_string(), "disabled");
    }

    #[test]
    fn test_storage_range_creation() {
        let range = StorageRange {
            start: 0,
            end: 100,
            inclusive: true,
        };
        assert_eq!(range.start, 0);
        assert_eq!(range.end, 100);
        assert!(range.inclusive);
    }
}
