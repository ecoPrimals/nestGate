// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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
        tracing::info!("🔍 Detecting local filesystems (universal detector)");

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
                "✅ Local filesystem: {} ({}) - {}GB available",
                fs.device,
                fs.fs_type,
                fs.available_bytes / 1_000_000_000
            );
        }

        tracing::info!(
            "✅ Detected {} local filesystems using {}",
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
    pub async fn detect_cloud_storage(&self) -> Result<Vec<DetectedStorage>> {
        if !self.config.enable_cloud_detection {
            return Ok(Vec::new());
        }

        let mut cloud_storage = Vec::new();

        // Detect AWS S3 buckets
        cloud_storage.extend(self.detect_aws_s3().await?);

        // Detect Azure Blob Storage
        cloud_storage.extend(self.detect_azure_blob().await?);

        // Detect Google Cloud Storage
        cloud_storage.extend(self.detect_gcs().await?);

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
    pub async fn detect_network_shares(&self) -> Result<Vec<DetectedStorage>> {
        if !self.config.enable_network_detection {
            return Ok(Vec::new());
        }

        let mut network_shares = Vec::new();

        // Detect SMB/CIFS shares
        network_shares.extend(self.detect_smb_shares().await?);

        // Detect NFS mounts
        network_shares.extend(self.detect_nfs_mounts().await?);

        // Detect iSCSI targets
        network_shares.extend(self.detect_iscsi_targets().await?);

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
        tracing::info!("🔍 Detecting block devices (universal detector)");

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
                    "✅ Block device: {} - {}GB available",
                    fs.device,
                    fs.available_bytes / 1_000_000_000
                );
            }
        }

        tracing::info!("✅ Detected {} block devices", block_devices.len());
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
    pub async fn detect_memory_storage(&self) -> Result<Vec<DetectedStorage>> {
        let mut memory_storage = Vec::new();

        // Detect tmpfs mounts
        memory_storage.extend(self.detect_tmpfs().await?);

        // Detect ramdisk
        memory_storage.extend(self.detect_ramdisk().await?);

        Ok(memory_storage)
    }

    // Helper methods for specific detection logic
    // NOTE: Platform-specific methods removed in favor of universal detection

    /// Detect Aws S3
    async fn detect_aws_s3(&self) -> Result<Vec<DetectedStorage>> {
        // Placeholder for AWS S3 detection
        // In a real implementation, this would use AWS SDK
        Ok(Vec::new())
    }

    /// Detect Azure Blob
    async fn detect_azure_blob(&self) -> Result<Vec<DetectedStorage>> {
        // Placeholder for Azure Blob detection
        Ok(Vec::new())
    }

    /// Detect Gcs
    async fn detect_gcs(&self) -> Result<Vec<DetectedStorage>> {
        // Placeholder for Google Cloud Storage detection
        Ok(Vec::new())
    }

    /// Detect Smb Shares
    async fn detect_smb_shares(&self) -> Result<Vec<DetectedStorage>> {
        // Placeholder for SMB share detection
        Ok(Vec::new())
    }

    /// Detect Nfs Mounts
    async fn detect_nfs_mounts(&self) -> Result<Vec<DetectedStorage>> {
        // Placeholder for NFS mount detection
        Ok(Vec::new())
    }

    /// Detect Iscsi Targets
    async fn detect_iscsi_targets(&self) -> Result<Vec<DetectedStorage>> {
        // Placeholder for iSCSI target detection
        Ok(Vec::new())
    }

    /// Detect Tmpfs
    async fn detect_tmpfs(&self) -> Result<Vec<DetectedStorage>> {
        // Placeholder for tmpfs detection
        Ok(Vec::new())
    }

    /// Detect Ramdisk
    async fn detect_ramdisk(&self) -> Result<Vec<DetectedStorage>> {
        // Placeholder for ramdisk detection
        Ok(Vec::new())
    }

    // Dead helper methods removed - superseded by universal filesystem detection (Phase 3.1)
    // Previously: analyze_block_device(), get_filesystem_stats()
    // Now handled by: UniversalFilesystemDetector in filesystem_detection.rs
}
