use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::unified_enums::UnifiedTierType;

// ==================== SECTION ====================

/// **THE** Universal Directory Entry structure
/// Comprehensive file system entry information for all storage protocols
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalDirectoryEntry {
    /// Entry name (file/directory name)
    pub name: String,
    /// Full path to the entry
    pub path: String,
    /// Type of file system entry
    pub entry_type: UnifiedEntryType,
    /// Size in bytes (0 for directories)
    pub size: u64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,
    /// Last access timestamp
    pub accessed_at: Option<DateTime<Utc>>,
    /// File permissions/mode
    pub permissions: UnifiedPermissions,
    /// MIME type detection
    pub mime_type: Option<String>,
    /// Checksum/hash for integrity
    pub checksum: Option<String>,
    /// Storage tier recommendation
    pub recommended_tier: Option<UnifiedTierType>,
    /// Extended attributes
    pub extended_attributes: HashMap<String, String>,
    /// Symlink target (if applicable)
    pub symlink_target: Option<String>,
}
/// Universal entry type classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UnifiedEntryType {
    /// Regular file
    File,
    /// Directory
    Directory,
    /// Symbolic link
    SymbolicLink,
    /// Hard link
    HardLink,
    /// Block device
    BlockDevice,
    /// Character device
    CharacterDevice,
    /// Named pipe (FIFO)
    NamedPipe,
    /// Socket
    Socket,
    /// Unknown or unsupported type
    Unknown,
}
/// Universal permissions structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedPermissions {
    /// POSIX-style permissions string (e.g., "rwxr-xr-x")
    pub posix: String,
    /// Octal permissions (e.g., 755)
    pub octal: u16,
    /// Owner user ID
    pub owner_uid: Option<u32>,
    /// Group ID
    pub group_gid: Option<u32>,
    /// Human-readable owner name
    pub owner_name: Option<String>,
    /// Human-readable group name
    pub group_name: Option<String>,
    /// Special permissions (sticky bit, setuid, etc.)
    pub special_bits: SpecialPermissions,
}
/// Special file system permissions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpecialPermissions {
    /// Sticky bit
    pub sticky: bool,
    /// Set user ID
    pub setuid: bool,
    /// Set group ID
    pub setgid: bool,
}
/// **THE** Universal Range structure for file operations
/// Generic range type for byte ranges, line ranges, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalRange<T> {
    /// Start of range (inclusive)
    pub start: T,
    /// End of range (exclusive)
    pub end: T,
    /// Total size of the range
    pub size: Option<u64>,
}
impl<T> UniversalRange<T> {
    /// Create a new range
    pub fn new(start: T, end: T) -> Self {
        Self {
            start,
            end,
            size: None,
        }
    }

    /// Create a range with known size
    pub fn with_size(start: T, end: T, size: u64) -> Self {
        Self {
            start,
            end,
            size: Some(size),
        }
    }
}

/// **THE** Universal Change tracking structure
/// Comprehensive change tracking for real-time synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalChange {
    /// Path that changed
    /// Type of operation that occurred
    /// Timestamp when change occurred
    pub timestamp: DateTime<Utc>,
    /// Source of the change (user, system, replication, etc.)
    pub source: ChangeSource,
    /// Before and after metadata (for detailed tracking)
    pub metadata_changes: Option<MetadataChanges>,
    /// Size of data changed (in bytes)
    pub bytes_affected: Option<u64>,
    /// Checksum before change
    pub checksum_before: Option<String>,
    /// Checksum after change
    pub checksum_after: Option<String>,
    /// Change sequence number (for ordering)
    pub sequence_number: Option<u64>,
    /// Related changes (for multi-step operations)
    pub related_changes: Vec<String>,
}
/// Change operation types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChangeOperation {
    /// File or directory created
    Create,
    /// Content modified
    Modify,
    /// File or directory deleted
    Delete,
    /// File or directory moved/renamed
    Move { from: String, to: String },
    /// File copied
    Copy { from: String, to: String },
    /// Permissions changed
    PermissionChange,
    /// Metadata changed (timestamps, attributes)
    MetadataChange,
    /// Symbolic link created/modified
    SymlinkChange,
    /// Unknown operation
    Unknown,
}
/// Source of changes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChangeSource {
    /// Direct user action
    User(String),
    /// System process
    System(String),
    /// Replication from remote
    Replication(String),
    /// Automated process
    Automation(String),
    /// Backup/restore operation
    Backup(String),
    /// Unknown source
    Unknown,
}
/// Metadata changes for detailed tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataChanges {
    /// Size before and after
    pub size_change: Option<(u64, u64)>,
    /// Permission changes
    pub permission_change: Option<(String, String)>,
    /// Timestamp changes
    pub timestamp_changes: HashMap<String, (DateTime<Utc>, DateTime<Utc>)>,
    /// Extended attribute changes
    pub attribute_changes: HashMap<String, (Option<String>, Option<String>)>,
}
// ==================== SECTION ====================

/// **THE** Universal Replication Status structure
/// Comprehensive replication status tracking with rich metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalReplicationStatus {
    /// Current replication state
    pub state: ReplicationState,
    /// Progress percentage (0.0 to 100.0)
    pub progress: f64,
    /// Total bytes to replicate
    pub total_bytes: Option<u64>,
    /// Bytes replicated so far
    pub bytes_replicated: u64,
    /// Files processed so far
    pub files_processed: u64,
    /// Total files to process
    pub total_files: Option<u64>,
    /// Current transfer rate (bytes per second)
    pub transfer_rate: Option<u64>,
    /// Estimated time to completion
    pub eta_seconds: Option<u64>,
    /// Last synchronization timestamp
    pub last_sync: DateTime<Utc>,
    /// Started timestamp
    pub started_at: DateTime<Utc>,
    /// Source location
    pub source: String,
    /// Target location
    pub target: String,
    /// Replication session ID
    pub session_id: String,
    /// Current file being processed
    pub current_file: Option<String>,
    /// Errors encountered
    pub errors: Vec<ReplicationError>,
    /// Warnings issued
    pub warnings: Vec<String>,
    /// Performance metrics
    pub metrics: ReplicationMetrics,
}
/// Replication state enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReplicationState {
    /// Replication is idle/not running
    Idle,
    /// Preparing for replication (scanning, planning)
    Preparing,
    /// Actively replicating data
    Running,
    /// Paused (can be resumed)
    Paused,
    /// Completed successfully
    Completed,
    /// Failed with errors
    Failed,
    /// Cancelled by user
    Cancelled,
    /// Verification phase
    Verifying,
    /// Cleanup phase
    Cleanup,
}
/// Replication error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationError {
    /// Error message
    pub message: String,
    /// Error code
    pub code: String,
    /// File or path that caused the error
    /// Timestamp when error occurred
    pub timestamp: DateTime<Utc>,
    /// Whether error is recoverable
    /// Recovery suggestion
    pub recovery_hint: Option<String>,
}
/// Replication performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationMetrics {
    /// Average transfer rate over session (bytes/sec)
    pub avg_transfer_rate: f64,
    /// Peak transfer rate (bytes/sec)
    pub peak_transfer_rate: f64,
    /// Network latency (milliseconds)
    pub network_latency_ms: Option<f64>,
    /// CPU usage during replication
    pub cpu_usage_percent: Option<f64>,
    /// Memory usage during replication (bytes)
    pub memory_usage_bytes: Option<u64>,
    /// Disk I/O operations per second
    pub disk_iops: Option<f64>,
}
/// **THE** Universal Replication Result structure
/// Comprehensive replication completion report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalReplicationResult {
    /// Whether replication completed successfully
    pub success: bool,
    /// Overall result message
    pub message: String,
    /// Detailed result code
    pub result_code: ReplicationResultCode,
    /// Total bytes transferred
    pub bytes_transferred: u64,
    /// Total files transferred
    pub files_transferred: u64,
    /// Total duration in milliseconds
    pub duration_ms: u64,
    /// Average transfer rate (bytes/sec)
    pub avg_transfer_rate: f64,
    /// Errors encountered during replication
    pub errors: Vec<ReplicationError>,
    /// Warnings issued during replication
    pub warnings: Vec<String>,
    /// Verification result (if performed)
    pub verification_result: Option<VerificationResult>,
    /// Performance summary
    pub performance_summary: ReplicationMetrics,
    /// Start and end timestamps
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    /// Source and target paths
    pub source: String,
    pub target: String,
    /// Session ID for tracking
    pub session_id: String,
}
/// Replication result codes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReplicationResultCode {
    /// Completed successfully
    Success,
    /// Completed with warnings
    SuccessWithWarnings,
    /// Partially completed (some files failed)
    PartialSuccess,
    /// Failed due to network issues
    NetworkFailure,
    /// Failed due to permission issues
    PermissionDenied,
    /// Failed due to insufficient space
    InsufficientSpace,
    /// Failed due to timeout
    Timeout,
    /// Cancelled by user
    Cancelled,
    /// Unknown failure
    UnknownFailure,
}
/// Verification result for replication integrity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Overall verification status
    pub status: VerificationStatus,
    /// Files verified successfully
    pub files_verified: u64,
    /// Files that failed verification
    pub files_failed: u64,
    /// Verification method used
    pub method: VerificationMethod,
    /// Time taken for verification (ms)
    pub verification_time_ms: u64,
    /// Detailed verification errors
    pub verification_errors: Vec<VerificationError>,
}
/// Verification status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationStatus {
    /// All files verified successfully
    Passed,
    /// Some files failed verification
    Failed,
    /// Verification was skipped
    Skipped,
    /// Verification is in progress
    InProgress,
}
/// Verification methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationMethod {
    /// Checksum comparison
    Checksum(String),
    /// File size comparison
    FileSize,
    /// Timestamp comparison
    Timestamp,
    /// Full binary comparison
    BinaryComparison,
    /// Custom verification logic
    Custom(String),
}
/// Verification error details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationError {
    /// File path that failed verification
    /// Error message
    pub message: String,
    /// Expected value
    pub expected: Option<String>,
    /// Actual value
    pub currentvalue: Option<String>,
    /// Verification method that failed
    pub method: VerificationMethod,
}
// ==================== SECTION ====================

impl Default for UniversalDirectoryEntry {
    fn default() -> Self {
        Self {
            name: String::new(),
            path: String::new(),
            entry_type: UnifiedEntryType::Unknown,
            size: 0,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            accessed_at: None,
            permissions: UnifiedPermissions::default(),
            mime_type: None,
            checksum: None,
            recommended_tier: None,
            extended_attributes: HashMap::new(),
            symlink_target: None,
        }
    }
}

impl Default for UnifiedPermissions {
    fn default() -> Self {
        Self {
            posix: "----------".to_string(),
            octal: 0,
            owner_uid: None,
            group_gid: None,
            owner_name: None,
            group_name: None,
            special_bits: SpecialPermissions::default(),
        }
    }
}

impl Default for UniversalChange {
    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            source: ChangeSource::Unknown,
            metadata_changes: None,
            bytes_affected: None,
            checksum_before: None,
            checksum_after: None,
            sequence_number: None,
            related_changes: Vec::new(),
        }
    }
}

impl Default for UniversalReplicationStatus {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            state: ReplicationState::Idle,
            progress: 0.0,
            total_bytes: None,
            bytes_replicated: 0,
            files_processed: 0,
            total_files: None,
            transfer_rate: None,
            eta_seconds: None,
            last_sync: now,
            started_at: now,
            source: String::new(),
            target: String::new(),
            session_id: uuid::Uuid::new_v4().to_string(),
            current_file: None,
            errors: Vec::new(),
            warnings: Vec::new(),
            metrics: ReplicationMetrics::default(),
        }
    }
}

impl Default for ReplicationMetrics {
    fn default() -> Self {
        Self {
            avg_transfer_rate: 0.0,
            peak_transfer_rate: 0.0,
            network_latency_ms: None,
            cpu_usage_percent: None,
            memory_usage_bytes: None,
            disk_iops: None,
        }
    }
}

impl Default for UniversalReplicationResult {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            success: true,
            message: "Replication completed successfully".to_string(),
            result_code: ReplicationResultCode::Success,
            bytes_transferred: 0,
            files_transferred: 0,
            duration_ms: 0,
            avg_transfer_rate: 0.0,
            errors: Vec::new(),
            warnings: Vec::new(),
            verification_result: None,
            performance_summary: ReplicationMetrics::default(),
            started_at: now,
            completed_at: now,
            source: String::new(),
            target: String::new(),
            session_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

// ==================== SECTION ====================

impl UniversalDirectoryEntry {
    /// Check if this entry is a file
    pub fn is_file(&self) -> bool {
        self.entry_type == UnifiedEntryType::File
    }

    /// Check if this entry is a directory
    pub fn is_directory(&self) -> bool {
        self.entry_type == UnifiedEntryType::Directory
    }

    /// Check if this entry is a symbolic link
    pub fn is_symlink(&self) -> bool {
        self.entry_type == UnifiedEntryType::SymbolicLink
    }

    /// Get file extension if available
    pub fn extension(&self) -> Option<&str> {
        if self.is_file() {
            std::path::Path::new(&self.path)
                .extension()
                .and_then(|ext| ext.to_str())
        } else {
            None
        }
    }

    /// Get human-readable size
    pub fn human_size(&self) -> String {
        human_readable_size(self.size)
    }
}

impl UniversalReplicationStatus {
    /// Check if replication is currently active
    pub fn is_active(&self) -> bool {
        matches!(
            self.state,
            ReplicationState::Running | ReplicationState::Preparing | ReplicationState::Verifying
        )
    }

    /// Check if replication has completed (successfully or not)
    pub fn is_completed(&self) -> bool {
        matches!(
            self.state,
            ReplicationState::Completed | ReplicationState::Failed | ReplicationState::Cancelled
        )
    }

    /// Get estimated completion time
    pub fn estimated_completion(&self) -> Option<DateTime<Utc>> {
        self.eta_seconds
            .map(|eta| Utc::now() + chrono::Duration::seconds(eta as i64))
    }
}

impl UniversalReplicationResult {
    /// Create a successful result
    pub fn success(bytes_transferred: u64, duration_ms: u64) -> Self {
        Self {
            success: true,
            message: "Replication completed successfully".to_string(),
            result_code: ReplicationResultCode::Success,
            bytes_transferred,
            duration_ms,
            avg_transfer_rate: if duration_ms > 0 {
                (bytes_transferred as f64 * 1000.0) / duration_ms as f64
            } else {
                0.0
            },
            ..Default::default()
        }
    }

    /// Create a failed result
    pub fn failure(message: &str, result_code: ReplicationResultCode) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            result_code,
            ..Default::default()
        }
    }
}

// ==================== SECTION ====================

/// Convert bytes to human-readable format
fn human_readable_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

// ==================== SECTION ====================

/// Backward compatibility alias for DirectoryEntry
pub type DirectoryEntry = UniversalDirectoryEntry;
/// Backward compatibility alias for Range
pub type Range<T> = UniversalRange<T>;
/// Backward compatibility alias for Change
pub type Change = UniversalChange;
/// Backward compatibility alias for ReplicationStatus
pub type ReplicationStatus = UniversalReplicationStatus;
/// Backward compatibility alias for ReplicationResult
pub type ReplicationResult = UniversalReplicationResult;
// ==================== SECTION ====================

/// Migrate from legacy DirectoryEntry structures
impl UniversalDirectoryEntry {
    /// Create from simple name, path, and size
    pub fn new(name: String, path: String, is_directory: bool, size: u64) -> Self {
        Self {
            name,
            path,
            entry_type: if is_directory {
                UnifiedEntryType::Directory
            } else {
                UnifiedEntryType::File
            },
            size,
            ..Default::default()
        }
    }
}
