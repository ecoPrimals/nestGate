// **STORAGE DOMAIN CONSTANTS**
//! Storage functionality and utilities.
// Storage-related constants extracted from the consolidated constants system.
// This module consolidates all storage-related constants:
//! - ZFS pool and dataset settings
//! - File system limits and sizes
//! - Backup and retention policies

// ==================== STORAGE DOMAIN CONSTANTS ====================

/// **STORAGE DOMAIN CONSTANTS**
///
/// Consolidates all storage-related constants:
/// - ZFS pool and dataset settings
/// - File system limits and sizes
/// - Backup and retention policies
#[derive(Debug, Clone)]
pub struct StorageDomainConstants {
    /// ZFS-specific constants
    pub zfs: ZfsConstants,

    /// File system constants
    pub filesystem: FilesystemConstants,

    /// Size constants
    pub sizes: SizeConstants,

    /// Backup constants
    pub backup: BackupConstants,

    /// Compression constants
    pub compression: CompressionConstants,
}
#[derive(Debug, Clone)]
pub struct ZfsConstants {
    /// Default ZFS pool name
    pub default_pool: &'static str,

    /// ZFS commands
    pub commands: ZfsCommands,

    /// ZFS properties
    pub properties: ZfsProperties,

    /// ZFS pool states
    pub states: ZfsStates,

    /// Record sizes
    pub record_sizes: ZfsRecordSizes,
}

#[derive(Debug, Clone)]
pub struct ZfsCommands {
    pub zfs: &'static str,
    pub zpool: &'static str,
    pub list: &'static str,
    pub create: &'static str,
    pub destroy: &'static str,
    pub set: &'static str,
    pub get: &'static str,
    pub snapshot: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone)]
pub struct ZfsProperties {
    pub all: &'static str,
    pub metadata: &'static str,
    pub on: &'static str,
    pub off: &'static str,
    pub compression: &'static str,
    pub dedup: &'static str,
    pub quota: &'static str,
    pub reservation: &'static str,
}

#[derive(Debug, Clone)]
pub struct ZfsStates {
    pub online: &'static str,
    pub degraded: &'static str,
    pub faulted: &'static str,
    pub offline: &'static str,
    pub unavail: &'static str,
    pub removed: &'static str,
}

#[derive(Debug, Clone)]
pub struct ZfsRecordSizes {
    pub size_64k: &'static str,
    pub size_128k: &'static str,
    pub size_1m: &'static str,
    pub default: &'static str,
}

#[derive(Debug, Clone)]
pub struct SizeConstants {
    /// Byte multipliers
    pub kb: u64,
    pub mb: u64,
    pub gb: u64,
    pub tb: u64,

    /// Default sizes
    pub default_buffer: usize,
    pub default_cache: usize,
    pub default_file_limit: u64,
    pub default_memory_limit: u64,
    pub default_page_size: usize,
}

#[derive(Debug, Clone)]
pub struct FilesystemConstants {
    /// Default filesystem type
    pub default_type: &'static str,

    /// Default mount options
    pub default_mount_options: &'static str,

    /// Default permissions
    pub default_permissions: u32,
}

#[derive(Debug, Clone)]
pub struct BackupConstants {
    /// Default backup interval in hours
    pub default_interval: u64,

    /// Default retention period in days
    pub default_retention: u64,

    /// Default compression level
    pub default_compression: u8,
}

#[derive(Debug, Clone)]
pub struct CompressionConstants {
    /// Compression algorithms
    pub lz4: &'static str,
    pub gzip: &'static str,
    pub zstd: &'static str,
    pub default: &'static str,
}

impl Default for StorageDomainConstants {
    fn default() -> Self {
        Self {
            zfs: ZfsConstants {
                default_pool: "tank",
                commands: ZfsCommands {
                    zfs: "zfs",
                    zpool: "zpool",
                    list: "list",
                    create: "create",
                    destroy: "destroy",
                    set: "set",
                    get: "get",
                    snapshot: "snapshot",
                    status: "status",
                },
                properties: ZfsProperties {
                    all: "all",
                    metadata: "metadata",
                    on: "on",
                    off: "off",
                    compression: "compression",
                    dedup: "dedup",
                    quota: "quota",
                    reservation: "reservation",
                },
                states: ZfsStates {
                    online: "ONLINE",
                    degraded: "DEGRADED",
                    faulted: "FAULTED",
                    offline: "OFFLINE",
                    unavail: "UNAVAIL",
                    removed: "REMOVED",
                },
                record_sizes: ZfsRecordSizes {
                    size_64k: "64K",
                    size_128k: "128K",
                    size_1m: "1M",
                    default: "128K",
                },
            },
            filesystem: FilesystemConstants::default(),
            sizes: SizeConstants {
                kb: 1024,
                mb: 1024 * 1024,
                gb: 1024 * 1024 * 1024,
                tb: 1024 * 1024 * 1024 * 1024,
                default_buffer: 65536,
                default_cache: 128 * 1024 * 1024,            // 128MB
                default_file_limit: 10 * 1024 * 1024 * 1024, // 10GB
                default_memory_limit: 1024 * 1024 * 1024,    // 1GB
                default_page_size: 4096,
            },
            backup: BackupConstants {
                default_interval: 24,   // 24 hours
                default_retention: 30,  // 30 days
                default_compression: 6, // compression level 6
            },
            compression: CompressionConstants {
                lz4: "lz4",
                gzip: "gzip",
                zstd: "zstd",
                default: "lz4",
            },
        }
    }
}

impl Default for FilesystemConstants {
    fn default() -> Self {
        Self {
            default_type: "zfs",
            default_mount_options: "rw,relatime",
            default_permissions: 0o755,
        }
    }
}

// ==================== CONVENIENCE EXPORTS ====================

/// Convenience module for easy access to storage constants
pub mod storage_defaults {
    use super::*;
    /// Get default storage domain constants
    #[must_use]
    pub const fn constants() -> StorageDomainConstants {
        StorageDomainConstants::default()
    }

    /// Get default ZFS pool name
    pub const DEFAULT_POOL: &str = "tank";

    /// Get default buffer size
    pub const DEFAULT_BUFFER_SIZE: usize = 65536;

    /// Get default cache size
    pub const DEFAULT_CACHE_SIZE: usize = 128 * 1024 * 1024; // 128MB
}
