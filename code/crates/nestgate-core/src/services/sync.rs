// use crate::universal_storage::ConflictResolution; // Module doesn't exist
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    Overwrite,
    Merge,
    Skip,
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
    pub const fn io_error(error: std::io::Error) -> NestGateError {
        NestGateError::internal_error(format!("Sync IO error: {error}"), "sync_service")
    }

    /// Create configuration error
    pub const fn config_error(message: impl Into<String>) -> NestGateError {
        NestGateError::configuration_error("sync_config", &message.into())
    }

    /// Create conflict error with resolution guidance
    pub const fn conflict_error(message: impl Into<String>) -> NestGateError {
        NestGateError::validation_error(&format!("Sync conflict: ", message.into()))
    }

    /// Create timeout error
    #[must_use]
    pub const fn timeout_error() -> NestGateError {
        NestGateError::timeout_error("sync_operation", std::time::Duration::from_millis(30000))
    }

    /// Create checksum mismatch error
    #[must_use]
    pub const fn checksum_mismatch() -> NestGateError {
        NestGateError::validation_error("File checksum verification failed ")
    }

    /// Create invalid path error
    pub fn invalid_path<P: Into<String>>(path: P) -> NestGateError {
        let path_str = path.into();
        NestGateError::validation_error(&format!("Invalid sync path: {path_str}"))
    }

    /// Create service unavailable error
    #[must_use]
    pub const fn service_unavailable() -> NestGateError {
        NestGateError::internal_error("Sync service is not available", "sync_service")
    }
}

/// File metadata for synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncFileInfo {
    pub size: u64,
    pub modified: SystemTime,
    pub checksum: String,
    pub file_type: UnifiedFileType,
    pub permissions: u32,
    pub tier: UnifiedTierType,
}
/// Sync operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncOperation {
    Create(SyncFileInfo),
    Update(SyncFileInfo),
    Delete(PathBuf),
    Move { from: PathBuf, to: PathBuf },
}
/// Conflict resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolutionStrategy {
    UseNewer,
    UseOlder,
    UseLocal,
    UseRemote,
    Manual,
    BackupBoth,
}
/// Sync conflict information
#[derive(Debug, Clone)]
pub struct SyncConflict {
    pub local_info: SyncFileInfo,
    pub remote_info: SyncFileInfo,
    pub strategy: ConflictResolutionStrategy,
    pub resolved: bool,
    pub created_at: SystemTime,
}
/// File change type for change detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileChangeType {
    Created,
    Modified,
    Deleted,
    Moved(PathBuf),
}
/// Change detection event
#[derive(Debug, Clone)]
pub struct FileChangeEvent {
    pub change_type: FileChangeType,
    pub timestamp: SystemTime,
    pub enabled: bool,
}
/// **UNIFIED** Sync Configuration using canonical config pattern
/// Consolidates `DeltaSyncConfig`, `SyncSessionConfig`, and `SyncServiceConfig` into unified approach
pub type UnifiedSyncConfig = crate::config::canonical_master::NestGateCanonicalConfig;
/// Sync-specific configuration extensions
/// Domain-specific fields that don't belong in unified base configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncExtensions {
    /// Delta compression settings
    pub delta: DeltaSyncSettings,
    /// Session management settings
    pub session: SyncSessionSettings,
    /// Service-level settings
    pub service: SyncServiceSettings,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSyncSettings {
    pub enable_compression: bool,
    pub compression_level: u8,
    pub chunk_size: usize,
    pub max_delta_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncSessionSettings {
    pub max_concurrent_sessions: usize,
    pub session_timeout: Duration,
    pub enable_session_resumption: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncServiceSettings {
    pub enable_deduplication: bool,
    pub conflict_resolution: ConflictResolution,
    pub max_file_size_mb: u64,
    pub sync_interval: Duration,
}

impl Default for SyncExtensions {
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

impl UnifiedSyncConfig {
    // IMPLEMENTATION NOTE: Uses unified configuration system
    // Configuration migrated to UnifiedConfig pattern for consistency
}

/// Checksum algorithms
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ChecksumAlgorithm {
    Md5,
    Sha1,
    Sha256,
    #[default]
    Blake3,
}
/// Sync service statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SyncServiceStats {
    pub total_sessions: u64,
    pub active_sessions: u64,
    pub completed_sessions: u64,
    pub failed_sessions: u64,
    pub total_bytes_synced: u64,
    pub total_files_synced: u64,
    pub conflicts_resolved: u64,
    pub conflicts_pending: u64,
    pub average_sync_time_ms: f64,
    pub uptime_seconds: u64,
}
