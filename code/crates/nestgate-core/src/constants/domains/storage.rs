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
/// Storagedomainconstants
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
/// Zfsconstants
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
/// Zfscommands
pub struct ZfsCommands {
    /// Zfs
    pub zfs: &'static str,
    /// Zpool
    pub zpool: &'static str,
    /// List
    pub list: &'static str,
    /// Create
    pub create: &'static str,
    /// Destroy
    pub destroy: &'static str,
    /// Set
    pub set: &'static str,
    /// Get
    pub get: &'static str,
    /// Snapshot
    pub snapshot: &'static str,
    /// Status
    pub status: &'static str,
}

#[derive(Debug, Clone)]
/// Zfsproperties
pub struct ZfsProperties {
    /// All
    pub all: &'static str,
    /// Additional metadata key-value pairs
    pub metadata: &'static str,
    /// On
    pub on: &'static str,
    /// Off
    pub off: &'static str,
    /// Compression
    pub compression: &'static str,
    /// Dedup
    pub dedup: &'static str,
    /// Quota
    pub quota: &'static str,
    /// Reservation
    pub reservation: &'static str,
}

#[derive(Debug, Clone)]
/// Zfsstates
pub struct ZfsStates {
    /// Online
    pub online: &'static str,
    /// Degraded
    pub degraded: &'static str,
    /// Faulted
    pub faulted: &'static str,
    /// Offline
    pub offline: &'static str,
    /// Unavail
    pub unavail: &'static str,
    /// Removed
    pub removed: &'static str,
}

#[derive(Debug, Clone)]
/// Zfsrecordsizes
pub struct ZfsRecordSizes {
    /// Size 64K
    pub size_64k: &'static str,
    /// Size 128K
    pub size_128k: &'static str,
    /// Size 1M
    pub size_1m: &'static str,
    /// Default
    pub default: &'static str,
}

#[derive(Debug, Clone)]
/// Sizeconstants
pub struct SizeConstants {
    /// Byte multipliers
    pub kb: u64,
    /// Mb
    pub mb: u64,
    /// Gb
    pub gb: u64,
    /// Tb
    pub tb: u64,

    /// Default sizes
    pub default_buffer: usize,
    /// Default Cache
    pub default_cache: usize,
    /// Default File Limit
    pub default_file_limit: u64,
    /// Default Memory Limit
    pub default_memory_limit: u64,
    /// Size of default page
    pub default_page_size: usize,
}

#[derive(Debug, Clone)]
/// Filesystemconstants
pub struct FilesystemConstants {
    /// Default filesystem type
    pub default_type: &'static str,

    /// Default mount options
    pub default_mount_options: &'static str,

    /// Default permissions
    pub default_permissions: u32,
}

#[derive(Debug, Clone)]
/// Backupconstants
pub struct BackupConstants {
    /// Default backup interval in hours
    pub default_interval: u64,

    /// Default retention period in days
    pub default_retention: u64,

    /// Default compression level
    pub default_compression: u8,
}

#[derive(Debug, Clone)]
/// Compressionconstants
pub struct CompressionConstants {
    /// Compression algorithms
    pub lz4: &'static str,
    /// Gzip
    pub gzip: &'static str,
    /// Zstd
    pub zstd: &'static str,
    /// Default
    pub default: &'static str,
}

impl Default for StorageDomainConstants {
    /// Returns the default instance
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
    /// Returns the default instance
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
    pub fn constants() -> StorageDomainConstants {
        StorageDomainConstants::default()
    }

    /// Get default ZFS pool name
    pub const DEFAULT_POOL: &str = "tank";

    /// Get default buffer size
    pub const DEFAULT_BUFFER_SIZE: usize = 65536;

    /// Get default cache size
    pub const DEFAULT_CACHE_SIZE: usize = 128 * 1024 * 1024; // 128MB
}
