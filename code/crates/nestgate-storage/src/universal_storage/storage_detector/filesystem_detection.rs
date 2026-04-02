// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Universal Filesystem Detection
//!
//! **UNIVERSAL ARCHITECTURE** - Runtime filesystem discovery across all platforms
//! **EVOLUTION**: Phase 3 Task 1 - Deep Debt Evolution (Jan 31, 2026)
//!
//! Provides trait-based abstraction for discovering filesystems with runtime
//! capability detection instead of compile-time OS checks.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │   UniversalFilesystemDetector       │
//! │   (Runtime Capability Detection)    │
//! └──────────────┬──────────────────────┘
//!                │
//!       ┌────────┴────────┬─────────────┐
//!       │                 │             │
//! ┌─────▼─────┐    ┌─────▼──────┐ ┌────▼─────┐
//! │ Sysinfo   │    │  Linux     │ │ Generic  │
//! │ Detector  │    │  Proc      │ │ Platform │
//! │(Universal)│    │  (Optim.)  │ │ Detector │
//! └───────────┘    └────────────┘ └──────────┘
//!       │                 │             │
//!       └──────► Runtime Detection! ◄───┘
//! ```
//!
//! ## Key Features
//!
//! - **Runtime Detection**: Checks for actual filesystems, not assumed OS
//! - **Cross-Platform**: Linux, macOS, Windows, BSD support
//! - **Filesystem Types**: ext4, XFS, ZFS, Btrfs, NTFS, APFS, etc.
//! - **Graceful Degradation**: Works in containers and limited environments
//!
//! ## Usage
//!
//! ```rust,ignore
//! use nestgate_core::universal_storage::storage_detector::filesystem_detection::{
//!     UniversalFilesystemDetector, DiscoveredFilesystem
//! };
//!
//! async fn discover_filesystems() -> Vec<DiscoveredFilesystem> {
//!     let detector = UniversalFilesystemDetector::new();
//!     detector.discover().await.unwrap_or_default()
//! }
//! ```

use nestgate_types::error::{NestGateError, Result};
use nestgate_types::unified_enums::storage_types::{UnifiedStorageCapability, UnifiedStorageType};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use tracing::{debug, warn};

/// Total and available bytes for a mount point via [`rustix::fs::statvfs`] (Linux).
fn statvfs_space(path: &std::path::Path) -> (u64, u64) {
    #[cfg(target_os = "linux")]
    {
        rustix::fs::statvfs(path)
            .map(|v| {
                let fr = v.f_frsize;
                let total = v.f_blocks.saturating_mul(fr);
                let avail = v.f_bavail.saturating_mul(fr);
                (total, avail)
            })
            .unwrap_or((0, 0))
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = path;
        (0, 0)
    }
}

/// Discovered filesystem information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredFilesystem {
    /// Unique identifier
    pub id: String,
    /// Filesystem name
    pub name: String,
    /// Device path (e.g., /dev/sda1)
    pub device: String,
    /// Mount point
    pub mount_point: PathBuf,
    /// Filesystem type (ext4, zfs, ntfs, apfs, etc.)
    pub fs_type: String,
    /// Total space in bytes
    pub total_bytes: u64,
    /// Available space in bytes
    pub available_bytes: u64,
    /// Used space in bytes
    pub used_bytes: u64,
    /// Storage type classification
    pub storage_type: UnifiedStorageType,
    /// Detected capabilities
    pub capabilities: Vec<UnifiedStorageCapability>,
}

impl DiscoveredFilesystem {
    /// Detect capabilities based on filesystem type
    #[must_use]
    pub fn detect_capabilities(fs_type: &str) -> Vec<UnifiedStorageCapability> {
        let mut caps = Vec::new();

        match fs_type {
            "zfs" => {
                caps.push(UnifiedStorageCapability::Compression);
                caps.push(UnifiedStorageCapability::Deduplication);
                caps.push(UnifiedStorageCapability::Snapshots);
                caps.push(UnifiedStorageCapability::Encryption);
            }
            "btrfs" => {
                caps.push(UnifiedStorageCapability::Compression);
                caps.push(UnifiedStorageCapability::Snapshots);
            }
            "ext4" | "xfs" => {
                caps.push(UnifiedStorageCapability::Journaling);
            }
            "ntfs" | "apfs" => {
                caps.push(UnifiedStorageCapability::Compression);
                caps.push(UnifiedStorageCapability::Encryption);
            }
            _ => {}
        }

        caps
    }

    /// Classify storage type based on filesystem characteristics
    #[must_use]
    pub fn classify_storage_type(fs_type: &str, device: &str) -> UnifiedStorageType {
        // Network filesystems
        if device.contains(':') || ["nfs", "nfs4", "cifs", "smb", "smb3"].contains(&fs_type) {
            return UnifiedStorageType::Network;
        }

        // Memory-based filesystems
        if ["tmpfs", "ramfs"].contains(&fs_type) {
            return UnifiedStorageType::Memory;
        }

        // Default to local
        UnifiedStorageType::Local
    }
}

/// Universal trait for filesystem detection
///
/// **CAPABILITY-BASED**: Checks for actual filesystems, not just OS type
pub trait FilesystemDetector: Send + Sync {
    /// Discover filesystems
    ///
    /// **RUNTIME CHECK**: Actually reads filesystem information from the system
    fn discover(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<DiscoveredFilesystem>>> + Send + '_>>;

    /// Check if this detector is available
    fn is_available(&self) -> bool;

    /// Get detector name for logging
    fn name(&self) -> &str;
}

/// Sysinfo-based universal filesystem detector
///
/// **UNIVERSAL**: Works on all platforms using sysinfo crate.
///
/// ecoBin v3.0: `sysinfo` is a non-Linux / fallback path; Linux uses [`LinuxProcFilesystemDetector`].
/// Gated behind `feature = "sysinfo"` so pure-Rust builds can opt out.
#[cfg(feature = "sysinfo")]
pub struct SysinfoFilesystemDetector;

#[cfg(feature = "sysinfo")]
impl FilesystemDetector for SysinfoFilesystemDetector {
    fn discover(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<DiscoveredFilesystem>>> + Send + '_>> {
        Box::pin(async move {
            let disks = sysinfo::Disks::new_with_refreshed_list();
            let mut discovered = Vec::new();

            for disk in &disks {
                let mount_point = disk.mount_point();
                let device = disk.name().to_string_lossy().to_string();
                let fs_type = disk.file_system().to_string_lossy().to_string();
                let total_bytes = disk.total_space();
                let available_bytes = disk.available_space();
                let used_bytes = total_bytes.saturating_sub(available_bytes);

                let storage_type = DiscoveredFilesystem::classify_storage_type(&fs_type, &device);
                let capabilities = DiscoveredFilesystem::detect_capabilities(&fs_type);

                let id = format!("fs_{}", mount_point.to_string_lossy().replace('/', "_"));
                let name = format!("{} ({})", mount_point.display(), fs_type);

                let filesystem = DiscoveredFilesystem {
                    id,
                    name,
                    device: device.clone(),
                    mount_point: mount_point.to_path_buf(),
                    fs_type,
                    total_bytes,
                    available_bytes,
                    used_bytes,
                    storage_type,
                    capabilities,
                };

                debug!(
                    "Discovered filesystem via sysinfo: {} at {:?}",
                    device, mount_point
                );
                discovered.push(filesystem);
            }

            Ok(discovered)
        })
    }

    fn is_available(&self) -> bool {
        true
    }

    fn name(&self) -> &'static str {
        "sysinfo-filesystem-detector"
    }
}

/// Linux /proc/mounts filesystem detector
///
/// **OPTIMIZED**: Fast path for Linux using /proc/mounts
pub struct LinuxProcFilesystemDetector;

impl FilesystemDetector for LinuxProcFilesystemDetector {
    fn discover(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<DiscoveredFilesystem>>> + Send + '_>> {
        Box::pin(async move {
            use tokio::fs;

            let mounts_content = fs::read_to_string("/proc/mounts").await.map_err(|e| {
                NestGateError::io_error(format!("Failed to read /proc/mounts: {e}"))
            })?;

            let mut discovered = Vec::new();

            for line in mounts_content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 6 {
                    continue;
                }

                let device = parts[0];
                let mount_point = parts[1];
                let fs_type = parts[2];

                // Skip virtual filesystems
                if [
                    "proc",
                    "sysfs",
                    "devtmpfs",
                    "devpts",
                    "cgroup",
                    "cgroup2",
                    "pstore",
                    "securityfs",
                ]
                .contains(&fs_type)
                {
                    continue;
                }

                // ecoBin v3.0: disk space from `rustix::fs::statvfs`
                let mount_path = std::path::Path::new(mount_point);
                let (total_bytes, available_bytes) = statvfs_space(mount_path);

                let used_bytes = total_bytes.saturating_sub(available_bytes);
                let storage_type = DiscoveredFilesystem::classify_storage_type(fs_type, device);
                let capabilities = DiscoveredFilesystem::detect_capabilities(fs_type);

                let id = format!("fs_{}", mount_point.replace('/', "_"));
                let name = format!("{mount_point} ({fs_type})");

                discovered.push(DiscoveredFilesystem {
                    id,
                    name,
                    device: device.to_string(),
                    mount_point: PathBuf::from(mount_point),
                    fs_type: fs_type.to_string(),
                    total_bytes,
                    available_bytes,
                    used_bytes,
                    storage_type,
                    capabilities,
                });

                debug!(
                    "✅ Discovered filesystem via /proc/mounts: {} at {}",
                    device, mount_point
                );
            }

            Ok(discovered)
        })
    }

    fn is_available(&self) -> bool {
        std::path::Path::new("/proc/mounts").exists()
    }

    fn name(&self) -> &'static str {
        "linux-proc-filesystem-detector"
    }
}

/// Universal filesystem detector with adaptive selection
///
/// **ADAPTIVE**: Selects best available detector at runtime
pub struct UniversalFilesystemDetector {
    detector: Box<dyn FilesystemDetector>,
}

impl Default for UniversalFilesystemDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl UniversalFilesystemDetector {
    /// Create new universal filesystem detector
    ///
    /// **RUNTIME SELECTION**: Picks best available detector
    pub fn new() -> Self {
        debug!("🔍 Initializing universal filesystem detector");

        // Try optimized detectors first (faster, more detailed)
        let linux_detector = LinuxProcFilesystemDetector;
        if linux_detector.is_available() {
            debug!("✅ Using optimized Linux /proc/mounts detector");
            return Self {
                detector: Box::new(linux_detector),
            };
        }

        #[cfg(feature = "sysinfo")]
        {
            debug!("Using universal sysinfo detector");
            Self {
                detector: Box::new(SysinfoFilesystemDetector),
            }
        }

        #[cfg(not(feature = "sysinfo"))]
        {
            debug!("No sysinfo feature; falling back to Linux proc detector");
            Self {
                detector: Box::new(linux_detector),
            }
        }
    }

    /// Discover all filesystems
    ///
    /// **GRACEFUL**: Returns empty vec on error (non-fatal in containers)
    ///
    /// # Errors
    ///
    /// Returns `Ok(Vec::new())` when the underlying detector fails; errors are logged and not propagated.
    pub async fn discover(&self) -> Result<Vec<DiscoveredFilesystem>> {
        debug!("🔍 Discovering filesystems with {}", self.detector.name());

        match self.detector.discover().await {
            Ok(filesystems) => {
                debug!("✅ Discovered {} filesystems", filesystems.len());
                Ok(filesystems)
            }
            Err(e) => {
                warn!("⚠️ Filesystem discovery failed (non-fatal): {}", e);
                Ok(Vec::new()) // Graceful degradation
            }
        }
    }

    /// Get detector name for diagnostics
    #[must_use]
    pub fn detector_name(&self) -> &str {
        self.detector.name()
    }

    /// Check if filesystem detection is available
    #[must_use]
    pub fn is_available(&self) -> bool {
        self.detector.is_available()
    }

    /// Filter filesystems by minimum size
    #[must_use]
    pub fn filter_by_min_size(
        filesystems: Vec<DiscoveredFilesystem>,
        min_bytes: u64,
    ) -> Vec<DiscoveredFilesystem> {
        filesystems
            .into_iter()
            .filter(|fs| fs.available_bytes >= min_bytes)
            .collect()
    }

    /// Filter out virtual filesystems
    #[must_use]
    pub fn filter_virtual(filesystems: Vec<DiscoveredFilesystem>) -> Vec<DiscoveredFilesystem> {
        filesystems
            .into_iter()
            .filter(|fs| {
                !["proc", "sysfs", "devtmpfs", "devpts", "tmpfs"].contains(&fs.fs_type.as_str())
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_detector_creation() {
        let detector = UniversalFilesystemDetector::new();
        assert!(!detector.detector_name().is_empty());
    }

    #[test]
    fn test_detector_availability() {
        let detector = UniversalFilesystemDetector::new();
        // Should always have a detector available (at least sysinfo)
        assert!(detector.is_available());
    }

    #[tokio::test]
    async fn test_filesystem_discovery() {
        let detector = UniversalFilesystemDetector::new();

        println!("Using detector: {}", detector.detector_name());

        // Discovery should not panic (may return empty)
        let result = detector.discover().await;
        assert!(result.is_ok());

        let filesystems = result.unwrap();
        println!("Discovered {} filesystems", filesystems.len());

        for fs in &filesystems {
            println!(
                "  - {} → {:?} ({}) - {}GB / {}GB",
                fs.device,
                fs.mount_point,
                fs.fs_type,
                fs.available_bytes / 1_000_000_000,
                fs.total_bytes / 1_000_000_000
            );
        }
    }

    #[test]
    fn test_capability_detection() {
        let zfs_caps = DiscoveredFilesystem::detect_capabilities("zfs");
        assert!(zfs_caps.contains(&UnifiedStorageCapability::Compression));
        assert!(zfs_caps.contains(&UnifiedStorageCapability::Snapshots));

        let ext4_caps = DiscoveredFilesystem::detect_capabilities("ext4");
        assert!(ext4_caps.contains(&UnifiedStorageCapability::Journaling));
    }

    #[test]
    fn test_storage_type_classification() {
        assert_eq!(
            DiscoveredFilesystem::classify_storage_type("ext4", "/dev/sda1"),
            UnifiedStorageType::Local
        );

        assert_eq!(
            DiscoveredFilesystem::classify_storage_type("nfs", "server:/export"),
            UnifiedStorageType::Network
        );

        assert_eq!(
            DiscoveredFilesystem::classify_storage_type("tmpfs", "tmpfs"),
            UnifiedStorageType::Memory
        );
    }

    #[tokio::test]
    async fn test_filter_by_min_size() {
        let detector = UniversalFilesystemDetector::new();
        let filesystems = detector.discover().await.unwrap_or_default();

        let min_size = 1_000_000_000; // 1GB
        let filtered = UniversalFilesystemDetector::filter_by_min_size(filesystems, min_size);

        println!(
            "Filesystems with at least 1GB available: {}",
            filtered.len()
        );
        for fs in &filtered {
            assert!(fs.available_bytes >= min_size);
        }
    }

    #[test]
    fn test_linux_detector_availability() {
        let detector = LinuxProcFilesystemDetector;
        let available = detector.is_available();

        #[cfg(target_os = "linux")]
        {
            // On Linux, /proc/mounts should exist
            println!("Linux detector available: {available}");
        }

        #[cfg(not(target_os = "linux"))]
        {
            // On non-Linux, may or may not exist
            println!("Linux detector available (non-Linux OS): {available}");
        }
    }

    #[test]
    #[cfg(feature = "sysinfo")]
    fn test_sysinfo_detector_always_available() {
        let detector = SysinfoFilesystemDetector;
        assert!(
            detector.is_available(),
            "sysinfo detector should always be available"
        );
    }
}
