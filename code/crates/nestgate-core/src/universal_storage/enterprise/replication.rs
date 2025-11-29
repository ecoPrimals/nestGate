use std::collections::HashMap;
//
// This module handles storage replication, backup creation, and restoration
// across multiple targets and storage systems.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Storage replication target
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagetarget
pub struct StorageTarget {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Target Type
    pub target_type: TargetType,
    /// Endpoint
    pub endpoint: String,
    /// Credentials
    pub credentials: Option<HashMap<String, String>>,
    /// Compression
    pub compression: bool,
    /// Encryption
    pub encryption: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Target
pub enum TargetType {
    /// Filesystem
    Filesystem,
    /// S3Compatible
    S3Compatible,
    /// Remotenestgate
    RemoteNestGate,
    /// Networkshare
    NetworkShare,
}

/// Replication job status and progress
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Replicationjob
pub struct ReplicationJob {
    /// Unique identifier
    pub id: String,
    /// Source Snapshot
    pub source_snapshot: String,
    /// Target
    pub target: StorageTarget,
    /// Status
    pub status: ReplicationStatus,
    /// Progress Percentage
    pub progress_percentage: f32,
    /// Bytes Transferred
    pub bytes_transferred: u64,
    /// Total Bytes
    pub total_bytes: u64,
    /// Started At
    pub started_at: SystemTime,
    /// Estimated Completion
    pub estimated_completion: Option<SystemTime>,
    /// Error Message
    pub error_message: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Replication
pub enum ReplicationStatus {
    /// Queued
    Queued,
    /// Inprogress
    InProgress,
    /// Completed
    Completed,
    /// Failed
    Failed,
    /// Cancelled
    Cancelled,
}

/// Backup manifest containing metadata about backup contents
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Backupmanifest
pub struct BackupManifest {
    /// Unique identifier
    pub id: String,
    /// Backup Type
    pub backup_type: BackupType,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Source Snapshot
    pub source_snapshot: Option<String>,
    /// Base Snapshot
    pub base_snapshot: Option<String>, // For incremental backups
    /// Files
    pub files: Vec<BackupFileEntry>,
    /// Size of total
    pub total_size: u64,
    /// Compression Ratio
    pub compression_ratio: f32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Backup
pub enum BackupType {
    /// Full
    Full,
    /// Incremental
    Incremental,
    /// Differential
    Differential,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Backupfileentry
pub struct BackupFileEntry {
    /// Size
    pub size: u64,
    /// Checksum
    pub checksum: String,
    /// Modified At
    pub modified_at: SystemTime,
    /// Backup Location
    pub backup_location: String,
}

impl StorageTarget {
    /// Creates a new instance
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

    #[must_use]
    pub fn with_credentials(mut self, credentials: HashMap<String, String>) -> Self {
        self.credentials = Some(credentials);
        self
    }

    #[must_use]
    pub fn with_compression(mut self, enabled: bool) -> Self {
        self.compression = enabled;
        self
    }

    #[must_use]
    pub fn with_encryption(mut self, enabled: bool) -> Self {
        self.encryption = enabled;
        self
    }
}

impl ReplicationJob {
    /// Creates a new instance
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

    /// Updates  Progress
    pub fn update_progress(&mut self, bytes_transferred: u64, total_bytes: u64) {
        self.bytes_transferred = bytes_transferred;
        self.total_bytes = total_bytes;
        self.progress_percentage = if total_bytes > 0 {
            (f32::from(bytes_transferred) / f32::from(total_bytes)) * 100.0
        } else {
            0.0
        };
    }

    /// Mark Completed
    pub fn mark_completed(&mut self) {
        self.status = ReplicationStatus::Completed;
        self.progress_percentage = 100.0;
    }

    /// Mark Failed
    pub fn mark_failed(&mut self, error: String) {
        self.status = ReplicationStatus::Failed;
        self.error_message = Some(error);
    }
}
