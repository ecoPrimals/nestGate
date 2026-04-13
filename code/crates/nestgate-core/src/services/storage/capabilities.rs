// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage Backend Capability Detection
//!
//! ✅ DEEP DEBT PRINCIPLE #5: Hardcoding Elimination
//! - No assumptions about deployment environment
//! - Agnostic to filesystem type
//! - Capability-based backend selection
//!
//! ✅ DEEP DEBT PRINCIPLE #6: Primal Self-Knowledge
//! - Discovers own capabilities at runtime
//! - No hardcoded assumptions
//! - Runtime environment detection

use std::process::Command;
use tracing::{debug, info};

/// Backend type identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendType {
    /// ZFS (Zettabyte File System) - Advanced features available
    Zfs,
    /// Standard filesystem (ext4, NTFS, APFS, etc.) - Universal compatibility
    Filesystem,
}

/// Backend capabilities
///
/// ✅ MODERN IDIOMATIC: Clear capability advertisement
#[derive(Debug, Clone)]
pub struct BackendCapabilities {
    /// Backend type
    pub backend_type: BackendType,

    /// Native snapshot support (instant, copy-on-write)
    pub native_snapshots: bool,

    /// Native deduplication (automatic space savings)
    pub native_deduplication: bool,

    /// Native compression (transparent, efficient)
    pub native_compression: bool,

    /// Native checksums (data integrity verification)
    pub native_checksums: bool,

    /// Native replication (efficient send/receive)
    pub native_replication: bool,

    /// Always available (filesystem-level operations)
    pub basic_operations: bool,
}

impl BackendCapabilities {
    /// ZFS backend capabilities (full feature set)
    #[must_use]
    pub const fn zfs() -> Self {
        Self {
            backend_type: BackendType::Zfs,
            native_snapshots: true,
            native_deduplication: true,
            native_compression: true,
            native_checksums: true,
            native_replication: true,
            basic_operations: true,
        }
    }

    /// Filesystem backend capabilities (universal compatibility)
    #[must_use]
    pub const fn filesystem() -> Self {
        Self {
            backend_type: BackendType::Filesystem,
            native_snapshots: false,
            native_deduplication: false,
            native_compression: false,
            native_checksums: false,
            native_replication: false,
            basic_operations: true,
        }
    }
}

/// Detect if ZFS is available on the system
///
/// ✅ AGNOSTIC: No assumptions about environment
/// ✅ SAFE: Graceful handling of all error cases
#[must_use]
pub fn is_zfs_available() -> bool {
    // Try to execute `zpool version` (safer than `zpool list`)
    // This checks if ZFS kernel modules are loaded without requiring pools
    match Command::new("zpool").arg("version").output() {
        Ok(output) if output.status.success() => {
            debug!("ZFS detected (zpool version succeeded)");
            true
        }
        Ok(output) => {
            let exit_code = output.status.code().unwrap_or(-1);
            debug!("ZFS not available (zpool version exit code: {})", exit_code);

            // Log stderr for debugging (but don't treat as error)
            if !output.stderr.is_empty() {
                if let Ok(stderr) = String::from_utf8(output.stderr) {
                    debug!("   ZFS stderr: {}", stderr.trim());
                }
            }
            false
        }
        Err(e) => {
            debug!("ZFS not available (zpool command not found: {})", e);
            false
        }
    }
}

/// Detect available storage backends
///
/// ✅ CAPABILITY-BASED: Discovers what's available, never assumes
/// ✅ RUNTIME DISCOVERY: No compile-time assumptions
#[must_use]
pub fn detect_backend() -> BackendCapabilities {
    let zfs_available = is_zfs_available();

    if zfs_available {
        info!("Storage backend: ZFS (optimized features available)");
        info!("   • Native snapshots ");
        info!("   • Native deduplication ");
        info!("   • Native compression ");
        info!("   • Native checksums ");
        info!("   • Native replication ");
        BackendCapabilities::zfs()
    } else {
        info!("Storage backend: Filesystem (universal compatibility)");
        info!("   • Basic operations ");
        info!("   • Software snapshots available (file copy)");
        info!("   • Software deduplication available (hash-based)");
        BackendCapabilities::filesystem()
    }
}

/// Detect and log backend capabilities (for service initialization)
///
/// ✅ TRANSPARENT: Clear logging of capabilities
pub fn detect_and_log() -> BackendCapabilities {
    info!("Detecting storage backend capabilities...");
    let capabilities = detect_backend();

    match capabilities.backend_type {
        BackendType::Zfs => {
            info!("NestGate: Universal Data Orchestrator with ZFS Optimization");
        }
        BackendType::Filesystem => {
            info!("NestGate: Universal Data Orchestrator (Agnostic Mode)");
            info!("   Works on ANY filesystem: ext4, NTFS, APFS, btrfs, etc.");
        }
    }

    capabilities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zfs_capabilities() {
        let caps = BackendCapabilities::zfs();
        assert_eq!(caps.backend_type, BackendType::Zfs);
        assert!(caps.native_snapshots);
        assert!(caps.native_deduplication);
        assert!(caps.native_compression);
        assert!(caps.native_checksums);
        assert!(caps.native_replication);
        assert!(caps.basic_operations);
    }

    #[test]
    fn test_filesystem_capabilities() {
        let caps = BackendCapabilities::filesystem();
        assert_eq!(caps.backend_type, BackendType::Filesystem);
        assert!(!caps.native_snapshots);
        assert!(!caps.native_deduplication);
        assert!(!caps.native_compression);
        assert!(!caps.native_checksums);
        assert!(!caps.native_replication);
        assert!(caps.basic_operations);
    }

    #[test]
    fn test_zfs_detection_doesnt_panic() {
        // Should never panic, regardless of environment
        let _ = is_zfs_available();
    }

    #[test]
    fn test_backend_detection_doesnt_panic() {
        // Should never panic, regardless of environment
        let _ = detect_backend();
    }
}
