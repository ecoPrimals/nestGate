// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// Core detection logic for different storage types.
//
//! Detection module
//!
//! **UNIVERSAL ARCHITECTURE** - Phase 3 Task 1 (Jan 31, 2026)
//! **EVOLUTION**: Platform-specific code eliminated, universal trait-based detection

use super::config::DetectionConfig;
use super::filesystem_detection::UniversalFilesystemDetector;
use super::types::DetectedStorage;
use nestgate_types::error::Result;
use nestgate_types::unified_enums::storage_types::UnifiedStorageType;

/// Detection engine for various storage types
pub struct DetectionEngine<'a> {
    config: &'a DetectionConfig,
}
impl<'a> DetectionEngine<'a> {
    /// Create new detection engine with configuration
    #[must_use]
    pub const fn new(config: &'a DetectionConfig) -> Self {
        Self { config }
    }

    /// Detect local filesystem mounts using universal detection
    ///
    /// **UNIVERSAL**: Works on Linux, macOS, Windows, BSD via runtime detection
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn detect_local_filesystems(&self) -> Result<Vec<DetectedStorage>> {
        tracing::info!("Detecting local filesystems (universal detector)");

        // Use universal filesystem detector (no platform-specific code!)
        let detector = UniversalFilesystemDetector::new();
        let discovered = detector.discover().await?;

        tracing::debug!(
            "Detector: {} | Discovered: {} filesystems",
            detector.detector_name(),
            discovered.len()
        );

        // Convert to DetectedStorage format and filter
        let mut filesystems = Vec::new();
        for fs in discovered {
            // Filter by storage type (local only)
            if fs.storage_type != UnifiedStorageType::Local {
                continue;
            }

            // Filter by minimum size
            if fs.available_bytes < self.config.minimum_storage_size {
                continue;
            }

            let mut storage = DetectedStorage::new(fs.id.clone(), fs.storage_type, fs.name.clone());

            storage.available_space = fs.available_bytes;
            storage.add_metadata("filesystem_type".to_string(), fs.fs_type.clone());
            storage.add_metadata("device".to_string(), fs.device.clone());
            storage.add_metadata("total_bytes".to_string(), fs.total_bytes.to_string());
            storage.add_metadata(
                "mount_point".to_string(),
                fs.mount_point.to_string_lossy().to_string(),
            );

            // Add detected capabilities
            for cap in fs.capabilities {
                storage.add_capability(cap);
            }

            filesystems.push(storage);
            tracing::debug!(
                "Local filesystem: {} ({}) - {}GB available",
                fs.device,
                fs.fs_type,
                fs.available_bytes / 1_000_000_000
            );
        }

        tracing::info!(
            "Detected {} local filesystems using {}",
            filesystems.len(),
            detector.detector_name()
        );
        Ok(filesystems)
    }

    /// Detect cloud storage services
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    ///
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn detect_cloud_storage(&self) -> Result<Vec<DetectedStorage>> {
        if !self.config.enable_cloud_detection {
            return Ok(Vec::new());
        }

        let mut cloud_storage = Vec::new();

        // Detect AWS S3 buckets
        cloud_storage.extend(self.detect_aws_s3()?);

        // Detect Azure Blob Storage
        cloud_storage.extend(self.detect_azure_blob()?);

        // Detect Google Cloud Storage
        cloud_storage.extend(self.detect_gcs()?);

        Ok(cloud_storage)
    }

    /// Detect network shares
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    ///
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn detect_network_shares(&self) -> Result<Vec<DetectedStorage>> {
        if !self.config.enable_network_detection {
            return Ok(Vec::new());
        }

        let mut network_shares = Vec::new();

        // Detect SMB/CIFS shares
        network_shares.extend(self.detect_smb_shares()?);

        // Detect NFS mounts
        network_shares.extend(self.detect_nfs_mounts()?);

        // Detect iSCSI targets
        network_shares.extend(self.detect_iscsi_targets()?);

        Ok(network_shares)
    }

    /// Detect block devices using universal detection
    ///
    /// **UNIVERSAL**: Works on all platforms via filesystem detection
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn detect_block_devices(&self) -> Result<Vec<DetectedStorage>> {
        tracing::info!("Detecting block devices (universal detector)");

        // Use universal filesystem detector to find block-backed filesystems
        let detector = UniversalFilesystemDetector::new();
        let discovered = detector.discover().await?;

        let mut block_devices = Vec::new();
        for fs in discovered {
            // Filter for block devices (not tmpfs, network, etc.)
            if fs.storage_type == UnifiedStorageType::Local && fs.device.starts_with("/dev/") {
                if fs.available_bytes < self.config.minimum_storage_size {
                    continue;
                }

                let mut storage = DetectedStorage::new(
                    fs.id.clone(),
                    UnifiedStorageType::Local, // Block devices are local storage
                    fs.name.clone(),
                );

                storage.available_space = fs.available_bytes;
                storage.add_metadata("device".to_string(), fs.device.clone());
                storage.add_metadata("filesystem_type".to_string(), fs.fs_type.clone());
                storage.add_metadata("total_bytes".to_string(), fs.total_bytes.to_string());

                for cap in fs.capabilities {
                    storage.add_capability(cap);
                }

                block_devices.push(storage);
                tracing::debug!(
                    "Block device: {} - {}GB available",
                    fs.device,
                    fs.available_bytes / 1_000_000_000
                );
            }
        }

        tracing::info!("Detected {} block devices", block_devices.len());
        Ok(block_devices)
    }

    /// Detect memory-based storage
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    ///
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn detect_memory_storage(&self) -> Result<Vec<DetectedStorage>> {
        let mut memory_storage = Vec::new();

        // Detect tmpfs mounts
        memory_storage.extend(self.detect_tmpfs()?);

        // Detect ramdisk
        memory_storage.extend(self.detect_ramdisk()?);

        Ok(memory_storage)
    }

    /// Cloud storage (S3, Azure, GCS) is not `NestGate`'s responsibility — it is delegated
    /// to orchestration providers via capability discovery.
    fn detect_aws_s3(&self) -> Result<Vec<DetectedStorage>> {
        tracing::debug!(
            "Cloud S3 detection delegated to orchestration provider via capability discovery"
        );
        Ok(Vec::new())
    }

    /// Cloud storage detection delegated to orchestration.
    fn detect_azure_blob(&self) -> Result<Vec<DetectedStorage>> {
        tracing::debug!(
            "Cloud Azure Blob detection delegated to orchestration provider via capability discovery"
        );
        Ok(Vec::new())
    }

    /// Cloud storage detection delegated to orchestration.
    fn detect_gcs(&self) -> Result<Vec<DetectedStorage>> {
        tracing::debug!(
            "Cloud GCS detection delegated to orchestration provider via capability discovery"
        );
        Ok(Vec::new())
    }

    /// Detect SMB/CIFS shares by parsing `/proc/mounts` for `cifs` filesystem type.
    fn detect_smb_shares(&self) -> Result<Vec<DetectedStorage>> {
        Self::detect_mounts_by_fs_type(
            &["cifs", "smb", "smb3"],
            &UnifiedStorageType::Network,
            "smb",
        )
    }

    /// Detect NFS mounts by parsing `/proc/mounts` for `nfs`/`nfs4` filesystem types.
    fn detect_nfs_mounts(&self) -> Result<Vec<DetectedStorage>> {
        Self::detect_mounts_by_fs_type(&["nfs", "nfs4"], &UnifiedStorageType::Network, "nfs")
    }

    /// Detect iSCSI targets by checking `/sys/class/iscsi_host/` and cross-referencing
    /// block devices in `/proc/mounts`.
    fn detect_iscsi_targets(&self) -> Result<Vec<DetectedStorage>> {
        let iscsi_host_dir = std::path::Path::new("/sys/class/iscsi_host");
        if !iscsi_host_dir.exists() {
            return Ok(Vec::new());
        }

        let hosts: Vec<_> = std::fs::read_dir(iscsi_host_dir)
            .map(|rd| rd.filter_map(std::result::Result::ok).collect())
            .unwrap_or_default();

        if hosts.is_empty() {
            return Ok(Vec::new());
        }

        let mut targets = Vec::new();
        for (i, host) in hosts.iter().enumerate() {
            let id = format!("iscsi_{}", host.file_name().to_string_lossy());
            let mut storage =
                DetectedStorage::new(id, UnifiedStorageType::Network, format!("iSCSI host {i}"));
            storage.add_metadata(
                "iscsi_host".to_string(),
                host.file_name().to_string_lossy().to_string(),
            );
            targets.push(storage);
        }

        tracing::debug!("Detected {} iSCSI host(s)", targets.len());
        Ok(targets)
    }

    /// Detect tmpfs mounts by parsing `/proc/mounts`.
    fn detect_tmpfs(&self) -> Result<Vec<DetectedStorage>> {
        Self::detect_mounts_by_fs_type(&["tmpfs"], &UnifiedStorageType::Memory, "tmpfs")
    }

    /// Detect ramdisk (ramfs) mounts by parsing `/proc/mounts`.
    fn detect_ramdisk(&self) -> Result<Vec<DetectedStorage>> {
        Self::detect_mounts_by_fs_type(&["ramfs"], &UnifiedStorageType::Memory, "ramdisk")
    }

    /// Shared helper: parse `/proc/mounts` for lines matching any of the given fs types
    /// and return them as `DetectedStorage`. Falls back to an empty vec on non-Linux.
    fn detect_mounts_by_fs_type(
        fs_types: &[&str],
        storage_type: &UnifiedStorageType,
        label_prefix: &str,
    ) -> Result<Vec<DetectedStorage>> {
        let proc_mounts = std::path::Path::new("/proc/mounts");
        if !proc_mounts.exists() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(proc_mounts).map_err(|e| {
            nestgate_types::error::NestGateError::io_error(format!(
                "Failed to read /proc/mounts: {e}"
            ))
        })?;

        let mut results = Vec::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 4 {
                continue;
            }

            let device = parts[0];
            let mount_point = parts[1];
            let fs_type = parts[2];

            if !fs_types.contains(&fs_type) {
                continue;
            }

            let id = format!("{label_prefix}_{}", mount_point.replace('/', "_"));
            let display = format!("{mount_point} ({fs_type})");
            let mut storage = DetectedStorage::new(id, storage_type.clone(), display);

            let mount_path = std::path::Path::new(mount_point);
            if let Ok(stat) = rustix::fs::statvfs(mount_path) {
                let block_size = stat.f_frsize;
                let total = stat.f_blocks * block_size;
                let avail = stat.f_bavail * block_size;
                storage.available_space = avail;
                storage.add_metadata("total_bytes".to_string(), total.to_string());
            }

            storage.add_metadata("device".to_string(), device.to_string());
            storage.add_metadata("filesystem_type".to_string(), fs_type.to_string());
            storage.add_metadata("mount_point".to_string(), mount_point.to_string());
            results.push(storage);
        }

        if !results.is_empty() {
            tracing::debug!("Detected {} {label_prefix} mount(s)", results.len());
        }
        Ok(results)
    }
}

#[cfg(test)]
mod detection_tests {
    #![expect(clippy::field_reassign_with_default)]
    use super::{DetectionConfig, DetectionEngine};

    #[test]
    fn detect_cloud_storage_returns_empty_when_disabled() {
        let mut config = DetectionConfig::default();
        config.enable_cloud_detection = false;
        let engine = DetectionEngine::new(&config);
        let v = engine.detect_cloud_storage().expect("detect_cloud_storage");
        assert!(v.is_empty(), "expected empty when cloud detection disabled");
    }

    #[test]
    fn detect_network_shares_returns_empty_when_disabled() {
        let mut config = DetectionConfig::default();
        config.enable_network_detection = false;
        let engine = DetectionEngine::new(&config);
        let v = engine
            .detect_network_shares()
            .expect("detect_network_shares");
        assert!(
            v.is_empty(),
            "expected empty when network detection disabled"
        );
    }

    /// Cloud detection (S3, Azure, GCS) is delegated to orchestration — always returns empty.
    #[test]
    fn cloud_detection_delegated_to_orchestration() {
        let mut config = DetectionConfig::default();
        config.enable_cloud_detection = true;
        let engine = DetectionEngine::new(&config);
        let v = engine.detect_cloud_storage().expect("detect_cloud_storage");
        assert!(
            v.is_empty(),
            "cloud detection is delegated — should always return empty"
        );
    }

    /// Network share detection reads /proc/mounts when enabled (no panic).
    #[test]
    fn network_share_detection_does_not_panic() {
        let mut config = DetectionConfig::default();
        config.enable_network_detection = true;
        let engine = DetectionEngine::new(&config);
        let _ = engine
            .detect_network_shares()
            .expect("detect_network_shares should not error");
    }

    /// Memory storage detection reads /proc/mounts for tmpfs/ramfs (no panic).
    #[test]
    fn memory_storage_detection_does_not_panic() {
        let config = DetectionConfig::default();
        let engine = DetectionEngine::new(&config);
        let _ = engine
            .detect_memory_storage()
            .expect("detect_memory_storage should not error");
    }

    /// On Linux, tmpfs mounts are expected (at least /dev/shm, /tmp, or /run).
    #[cfg(target_os = "linux")]
    #[test]
    fn detect_tmpfs_finds_mounts_on_linux() {
        let config = DetectionConfig::default();
        let engine = DetectionEngine::new(&config);
        let v = engine
            .detect_memory_storage()
            .expect("detect_memory_storage");
        assert!(
            !v.is_empty(),
            "Linux systems should have at least one tmpfs mount"
        );
    }

    /// `detect_mounts_by_fs_type` returns empty on non-Linux (no /proc/mounts).
    #[cfg(not(target_os = "linux"))]
    #[test]
    fn detect_mounts_returns_empty_on_non_linux() {
        let config = DetectionConfig::default();
        let engine = DetectionEngine::new(&config);
        let v = engine
            .detect_memory_storage()
            .expect("detect_memory_storage");
        assert!(v.is_empty(), "non-Linux has no /proc/mounts");
    }
}
