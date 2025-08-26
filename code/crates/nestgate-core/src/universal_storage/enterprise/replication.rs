use std::collections::HashMap;
//
// This module handles storage replication, backup creation, and restoration
// across multiple targets and storage systems.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

/// Storage replication target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageTarget {
    pub id: String,
    pub name: String,
    pub target_type: TargetType,
    pub endpoint: String,
    pub credentials: Option<HashMap<String, String>>,
    pub compression: bool,
    pub encryption: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetType {
    Filesystem,
    S3Compatible,
    RemoteNestGate,
    NetworkShare,
}

/// Replication job status and progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationJob {
    pub id: String,
    pub source_snapshot: String,
    pub target: StorageTarget,
    pub status: ReplicationStatus,
    pub progress_percentage: f32,
    pub bytes_transferred: u64,
    pub total_bytes: u64,
    pub started_at: SystemTime,
    pub estimated_completion: Option<SystemTime>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplicationStatus {
    Queued,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Backup manifest containing metadata about backup contents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifest {
    pub id: String,
    pub backup_type: BackupType,
    pub created_at: SystemTime,
    pub source_snapshot: Option<String>,
    pub base_snapshot: Option<String>, // For incremental backups
    pub files: Vec<BackupFileEntry>,
    pub total_size: u64,
    pub compression_ratio: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupType {
    Full,
    Incremental,
    Differential,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupFileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub checksum: String,
    pub modified_at: SystemTime,
    pub backup_location: String,
}

impl StorageTarget {
    pub fn new(name: String, target_type: TargetType, endpoint: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            target_type,
            endpoint,
            credentials: None,
            compression: true,
            encryption: true,
        }
    }

    pub fn with_credentials(mut self, credentials: HashMap<String, String>) -> Self {
        self.credentials = Some(credentials);
        self
    }

    pub fn with_compression(mut self, enabled: bool) -> Self {
        self.compression = enabled;
        self
    }

    pub fn with_encryption(mut self, enabled: bool) -> Self {
        self.encryption = enabled;
        self
    }
}

impl ReplicationJob {
    pub fn new(source_snapshot: String, target: StorageTarget) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source_snapshot,
            target,
            status: ReplicationStatus::Queued,
            progress_percentage: 0.0,
            bytes_transferred: 0,
            total_bytes: 0,
            started_at: SystemTime::now(),
            estimated_completion: None,
            error_message: None,
        }
    }

    pub fn update_progress(&mut self, bytes_transferred: u64, total_bytes: u64) {
        self.bytes_transferred = bytes_transferred;
        self.total_bytes = total_bytes;
        self.progress_percentage = if total_bytes > 0 {
            (bytes_transferred as f32 / total_bytes as f32) * 100.0
        } else {
            0.0
        };
    }

    pub fn mark_completed(&mut self) {
        self.status = ReplicationStatus::Completed;
        self.progress_percentage = 100.0;
    }

    pub fn mark_failed(&mut self, error: String) {
        self.status = ReplicationStatus::Failed;
        self.error_message = Some(error);
    }
}
