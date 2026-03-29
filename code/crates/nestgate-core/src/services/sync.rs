// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// use crate::universal_storage::ConflictResolution; // Module doesn't exist
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Conflictresolution
pub enum ConflictResolution {
    /// Overwrite
    Overwrite,
    /// Merge
    Merge,
    /// Skip
    Skip,
    /// Prefernewest
    PreferNewest,
} // Stub for compilation
use crate::unified_enums::{UnifiedFileType, UnifiedTierType};
/// Synchronization Service Implementation
/// Provides robust file and data synchronization with delta compression,
/// deduplication, and configurable conflict resolution strategies.
use std::time::{Duration, SystemTime};

// This eliminates error type fragmentation in favor of the unified approach
use crate::error::NestGateError;

/// **UNIFIED** Sync Error creation utilities
/// Replaces the local `SyncError` enum with NestGateError-based functions
pub struct SyncErrors;
impl SyncErrors {
    /// Create IO error with sync context
    #[must_use]
    pub fn io_error(error: std::io::Error) -> NestGateError {
        NestGateError::internal_error(format!("Sync IO error: {error}"), "sync_service")
    }

    /// Create configuration error
    pub fn config_error(message: impl Into<String>) -> NestGateError {
        NestGateError::configuration_error("sync_config", &message.into())
    }

    /// Create conflict error with resolution guidance
    pub fn conflict_error(message: impl Into<String>) -> NestGateError {
        NestGateError::validation_error(&format!("Sync conflict: {}", message.into()))
    }

    /// Create timeout error
    #[must_use]
    pub fn timeout_error() -> NestGateError {
        NestGateError::timeout_error("sync_operation", std::time::Duration::from_millis(30000))
    }

    /// Create checksum mismatch error
    #[must_use]
    pub fn checksum_mismatch() -> NestGateError {
        NestGateError::validation_error("File checksum verification failed ")
    }

    /// Create invalid path error
    pub fn invalid_path<P: Into<String>>(path: P) -> NestGateError {
        let path_str = path.into();
        NestGateError::validation_error(&format!("Invalid sync path: {path_str}"))
    }

    /// Create service unavailable error
    #[must_use]
    pub fn service_unavailable() -> NestGateError {
        NestGateError::internal_error("Sync service is not available", "sync_service")
    }
}

/// File metadata for synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Syncfileinfo
pub struct SyncFileInfo {
    /// Size
    pub size: u64,
    /// Modified
    pub modified: SystemTime,
    /// Checksum
    pub checksum: String,
    /// File Type
    pub file_type: UnifiedFileType,
    /// Permissions
    pub permissions: u32,
    /// Tier
    pub tier: UnifiedTierType,
}
/// Sync operation types
///
/// Represents the different types of file synchronization operations that can be performed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncOperation {
    /// Create a new file
    Create(SyncFileInfo),
    /// Update an existing file
    Update(SyncFileInfo),
    /// Delete a file at the specified path
    Delete(PathBuf),
    /// Move a file from one location to another
    Move {
        /// Source path of the file to move
        from: PathBuf,
        /// Destination path where the file should be moved
        to: PathBuf,
    },
}
/// Conflict resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Conflictresolutionstrategy
pub enum ConflictResolutionStrategy {
    /// Usenewer
    UseNewer,
    /// Useolder
    UseOlder,
    /// Uselocal
    UseLocal,
    /// Useremote
    UseRemote,
    /// Manual
    Manual,
    /// Backupboth
    BackupBoth,
}
/// Sync conflict information
#[derive(Debug, Clone)]
/// Syncconflict
pub struct SyncConflict {
    /// Local Info
    pub local_info: SyncFileInfo,
    /// Remote Info
    pub remote_info: SyncFileInfo,
    /// Strategy
    pub strategy: ConflictResolutionStrategy,
    /// Resolved
    pub resolved: bool,
    /// Timestamp when this was created
    pub created_at: SystemTime,
}
/// File change type for change detection
///
/// Tracks the different types of changes that can occur to files during synchronization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileChangeType {
    /// File was newly created
    Created,
    /// File was modified
    Modified,
    /// File was deleted
    Deleted,
    /// File was moved to a new location (contains the new path)
    Moved(PathBuf),
}
/// Change detection event
#[derive(Debug, Clone)]
/// Filechangeevent
pub struct FileChangeEvent {
    /// Change Type
    pub change_type: FileChangeType,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Whether this feature is enabled
    pub enabled: bool,
}
/// **UNIFIED** Sync Configuration using canonical config pattern
/// Consolidates `DeltaSyncConfig`, `SyncSessionConfig`, and `SyncServiceConfig` into unified approach
pub type UnifiedSyncConfig = crate::config::canonical_primary::NestGateCanonicalConfig;
/// Sync-specific configuration extensions
/// Domain-specific fields that don't belong in unified base configs
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Syncextensions
pub struct SyncExtensions {
    /// Delta compression settings
    pub delta: DeltaSyncSettings,
    /// Session management settings
    pub session: SyncSessionSettings,
    /// Service-level settings
    pub service: SyncServiceSettings,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Deltasyncsettings
pub struct DeltaSyncSettings {
    /// Enable Compression
    pub enable_compression: bool,
    /// Compression Level
    pub compression_level: u8,
    /// Size of chunk
    pub chunk_size: usize,
    /// Size of max delta
    pub max_delta_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Syncsessionsettings
pub struct SyncSessionSettings {
    /// Max Concurrent Sessions
    pub max_concurrent_sessions: usize,
    /// Session Timeout
    pub session_timeout: Duration,
    /// Enable Session Resumption
    pub enable_session_resumption: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Syncservicesettings
pub struct SyncServiceSettings {
    /// Enable Deduplication
    pub enable_deduplication: bool,
    /// Conflict Resolution
    pub conflict_resolution: ConflictResolution,
    /// Max File Size in megabytes
    pub max_file_size_mb: u64,
    /// Sync Interval
    pub sync_interval: Duration,
}

impl Default for SyncExtensions {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            delta: DeltaSyncSettings {
                enable_compression: true,
                compression_level: 6,
                chunk_size: 64 * 1024,            // 64KB
                max_delta_size: 10 * 1024 * 1024, // 10MB
            },
            session: SyncSessionSettings {
                max_concurrent_sessions: 100,
                session_timeout: Duration::from_secs(3600), // 1 hour
                enable_session_resumption: true,
            },
            service: SyncServiceSettings {
                enable_deduplication: true,
                conflict_resolution: ConflictResolution::PreferNewest,
                max_file_size_mb: 1024,                  // 1GB
                sync_interval: Duration::from_secs(300), // 5 minutes
            },
        }
    }
}

// - DeltaSyncConfig: Merged into SyncExtensions::delta
// - SyncSessionConfig: Merged into SyncExtensions::session
// - SyncServiceConfig: Merged into SyncExtensions::service

// UnifiedSyncConfig impl removed — type alias for NestGateCanonicalConfig (defined in nestgate-config)

/// Checksum algorithms
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
/// Checksumalgorithm
pub enum ChecksumAlgorithm {
    /// Md5
    Md5,
    /// Sha1
    Sha1,
    /// Sha256
    Sha256,
    #[default]
    /// Blake3
    Blake3,
}
/// Sync service statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Syncservicestats
pub struct SyncServiceStats {
    /// Total Sessions
    pub total_sessions: u64,
    /// Active Sessions
    pub active_sessions: u64,
    /// Completed Sessions
    pub completed_sessions: u64,
    /// Failed Sessions
    pub failed_sessions: u64,
    /// Total Bytes Synced
    pub total_bytes_synced: u64,
    /// Total Files Synced
    pub total_files_synced: u64,
    /// Conflicts Resolved
    pub conflicts_resolved: u64,
    /// Conflicts Pending
    pub conflicts_pending: u64,
    /// Average Sync Time Ms
    pub average_sync_time_ms: f64,
    /// Uptime Seconds
    pub uptime_seconds: u64,
}
