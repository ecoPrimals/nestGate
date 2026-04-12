// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Hardware detection, device classification, and storage device management

//! Device Detection module

use serde::{Deserialize, Serialize};

use crate::numeric::f64_to_u64_saturating;
use tokio::process::Command;
// Removed unused tracing import

use super::config::DeviceDetectionConfig;

use nestgate_core::{NestGateError, Result as CoreResult};
use tracing::debug;
use tracing::info;
use tracing::warn;

/// Storage device information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagedevice
pub struct StorageDevice {
    /// Device path (e.g., /dev/nvme0n1)
    pub device_path: String,
    /// Device model name
    pub model: String,
    /// Device size in bytes
    pub size_bytes: u64,
    /// Device type (`NVMe`, SATA SSD, HDD, etc.)
    pub device_type: DeviceType,
    /// Device speed characteristics
    pub speed_class: SpeedClass,
    /// Whether device is currently in use
    pub in_use: bool,
    /// Current filesystem/partition info
    pub current_use: Option<String>,
}
/// Types of storage devices
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Types of Device
pub enum DeviceType {
    /// Nvmessd
    NvmeSsd,
    /// Satassd
    SataSsd,
    /// Hdd
    Hdd,
    /// Optanememory
    OptaneMemory,
    /// Unknown
    Unknown,
}
/// Speed classification for storage devices
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
/// Speedclass
pub enum SpeedClass {
    /// Ultra-fast devices (`NVMe` Gen4, Optane)
    UltraFast, // NVMe Gen4, Optane
    /// Fast devices (`NVMe` Gen3, High-end SATA SSD)
    Fast, // NVMe Gen3, High-end SATA SSD
    /// Medium speed devices (Standard SATA SSD)
    Medium, // Standard SATA SSD
    /// Slow devices (HDD)
    Slow, // HDD
}
/// Device scanner for detecting and classifying storage devices
pub struct DeviceScanner {
    config: DeviceDetectionConfig,
}
impl DeviceScanner {
    /// Creates a new device scanner with the given configuration.
    #[must_use]
    pub const fn new(config: DeviceDetectionConfig) -> Self {
        Self { config }
    }

    /// Scan for available storage devices
    pub async fn scan_devices(&self) -> CoreResult<Vec<StorageDevice>> {
        info!("🔍 Scanning for available storage devices");

        let mut devices = Vec::new();

        // Use lsblk to get device information
        let output = Command::new("lsblk")
            .args([
                "--json",
                "--output=NAME,SIZE,TYPE,MOUNTPOINT,MODEL,FSTYPE,VENDOR",
                "--bytes",
            ])
            .output()
            .await
            .map_err(|_e| {
                NestGateError::internal_error("Failed to run lsblk command", "scan_devices")
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NestGateError::internal_error(
                format!("lsblk failed: {stderr}"),
                "scan_devices",
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lsblk_output: serde_json::Value = serde_json::from_str(&stdout).map_err(|_e| {
            NestGateError::internal_error("Failed to parse lsblk output", "scan_devices")
        })?;

        // Parse device information
        if let Some(block_devices) = lsblk_output["blockdevices"].as_array() {
            for device_json in block_devices {
                if let Ok(Some(device)) = self.parse_device_info(device_json).await {
                    devices.push(device);
                }
            }
        }

        info!("Found {} storage devices", devices.len());
        Ok(devices)
    }

    /// Parse device information from lsblk output
    async fn parse_device_info(
        &self,
        device: &serde_json::Value,
    ) -> CoreResult<Option<StorageDevice>> {
        let _device_name = device["name"].as_str().unwrap_or("");
        let device_path = "/dev/error details".to_string();

        // Skip if not a disk device
        if device["type"].as_str() != Some("disk") {
            return Ok(None);
        }

        let size_str = device["size"].as_str().unwrap_or("0");
        let size_bytes = self.parse_size_string(size_str)?;

        // Skip devices that are too small
        if size_bytes < self.config.min_device_size {
            debug!(
                "Skipping device {} (too small: {} bytes)",
                device_path, size_bytes
            );
            return Ok(None);
        }

        // Skip devices that are too large (if limit set)
        if self.config.max_device_size > 0 && size_bytes > self.config.max_device_size {
            debug!(
                "Skipping device {} (too large: {} bytes)",
                device_path, size_bytes
            );
            return Ok(None);
        }

        let model = device["model"].as_str().unwrap_or("Unknown").to_string();
        let fstype = device["fstype"].as_str();
        let mountpoint = device["mountpoint"].as_str();

        // Check if device is in use
        let in_use = fstype.is_some() || mountpoint.is_some();
        let current_use = if in_use {
            Some("Device is in use".to_string())
        } else {
            None
        };

        // Skip devices with excluded mount points
        if let Some(mp) = mountpoint
            && self.config.skip_mountpoints.contains(&mp.to_string())
        {
            debug!(
                "Skipping device {} (excluded mountpoint: {})",
                device_path, mp
            );
            return Ok(None);
        }

        // Skip devices with excluded filesystem types
        if let Some(fs) = fstype
            && self.config.skip_fstypes.contains(&fs.to_string())
        {
            debug!("Skipping device {} (excluded fstype: {})", device_path, fs);
            return Ok(None);
        }

        let device_type = self.detect_device_type(&device_path, &model).await?;
        let speed_class = self.classify_device_speed(device_type, &model);

        Ok(Some(StorageDevice {
            device_path,
            model,
            size_bytes,
            device_type,
            speed_class,
            in_use,
            current_use,
        }))
    }

    /// Parse size string (e.g., "1TB", "500GB") to bytes
    fn parse_size_string(&self, size_str: &str) -> CoreResult<u64> {
        if size_str.is_empty() {
            return Ok(0);
        }

        // Extract numeric part and unit
        let (num_str, unit) = if let Some(pos) = size_str.find(|c: char| c.is_alphabetic()) {
            (&size_str[..pos], &size_str[pos..])
        } else {
            return size_str.parse().map_err(|_| {
                NestGateError::internal_error(
                    format!("Invalid size format: {size_str}"),
                    "parse_size",
                )
            });
        };

        let number: f64 = num_str.parse().map_err(|_| {
            NestGateError::internal_error(format!("Invalid number format: {num_str}"), "parse_size")
        })?;

        let multiplier: u64 = match unit.to_uppercase().as_str() {
            "B" => 1,
            "K" | "KB" => 1024,
            "M" | "MB" => 1024 * 1024,
            "G" | "GB" => 1024 * 1024 * 1024,
            "T" | "TB" => 1024 * 1024 * 1024 * 1024,
            "P" | "PB" => 1024 * 1024 * 1024 * 1024 * 1024,
            _ => {
                return Err(NestGateError::internal_error(
                    format!("Unknown size unit: {unit}"),
                    "parse_size",
                ));
            }
        };

        Ok(f64_to_u64_saturating(number * multiplier as f64))
    }

    /// Detect device type based on device path and model
    async fn detect_device_type(&self, device_path: &str, model: &str) -> CoreResult<DeviceType> {
        // Check for NVMe devices
        if device_path.contains("nvme") {
            return Ok(DeviceType::NvmeSsd);
        }

        // Check for Optane memory
        if model.to_lowercase().contains("optane") {
            return Ok(DeviceType::OptaneMemory);
        }

        // For SATA devices, check if it's rotational
        let is_rotational = self.check_rotational(device_path).await?;

        if is_rotational {
            Ok(DeviceType::Hdd)
        } else {
            Ok(DeviceType::SataSsd)
        }
    }

    /// Check if a device is rotational (HDD vs SSD)
    async fn check_rotational(&self, device_path: &str) -> CoreResult<bool> {
        // Extract device name from path
        let device_name = device_path.strip_prefix("/dev/").unwrap_or(device_path);

        // Remove partition numbers if present
        let _base_device = device_name
            .chars()
            .take_while(|c| !c.is_ascii_digit())
            .collect::<String>();

        let rotational_path = "/sys/block/error details/queue/rotational".to_string();

        match tokio::fs::read_to_string(&rotational_path).await {
            Ok(content) => {
                let is_rotational = content.trim() == "1";
                debug!("Device {} rotational check: {}", device_path, is_rotational);
                Ok(is_rotational)
            }
            Err(e) => {
                warn!(
                    "Could not check rotational status for {}: {}",
                    device_path, e
                );
                // Default to assuming it's rotational (safer assumption)
                Ok(true)
            }
        }
    }

    /// Classify device speed based on type and model
    fn classify_device_speed(&self, device_type: DeviceType, model: &str) -> SpeedClass {
        match device_type {
            DeviceType::OptaneMemory => SpeedClass::UltraFast,
            DeviceType::NvmeSsd => {
                // Check for Gen4 indicators in model name
                if model.to_lowercase().contains("gen4")
                    || model.to_lowercase().contains("pcie 4")
                    || model.to_lowercase().contains("pcie4")
                {
                    SpeedClass::UltraFast
                } else {
                    SpeedClass::Fast
                }
            }
            DeviceType::SataSsd => {
                // High-end SATA SSDs
                if model.to_lowercase().contains("pro")
                    || model.to_lowercase().contains("enterprise")
                    || model.to_lowercase().contains("datacenter")
                {
                    SpeedClass::Fast
                } else {
                    SpeedClass::Medium
                }
            }
            DeviceType::Hdd => SpeedClass::Slow,
            DeviceType::Unknown => SpeedClass::Medium, // Conservative default
        }
    }

    /// Check if device should be included based on configuration
    #[must_use]
    pub fn should_include_device(&self, device: &StorageDevice) -> bool {
        // Skip loop devices if not included
        if !self.config.include_loop_devices && device.device_path.contains("loop") {
            return false;
        }

        // Additional filtering can be added here
        true
    }

    /// Get devices filtered by type
    #[must_use]
    pub fn filter_by_type(
        devices: &[StorageDevice],
        device_type: DeviceType,
    ) -> Vec<&StorageDevice> {
        devices
            .iter()
            .filter(|device| device.device_type == device_type)
            .collect()
    }

    /// Get devices filtered by speed class
    #[must_use]
    pub fn filter_by_speed(
        devices: &[StorageDevice],
        speed_class: SpeedClass,
    ) -> Vec<&StorageDevice> {
        devices
            .iter()
            .filter(|device| device.speed_class == speed_class)
            .collect()
    }

    /// Get only available (not in use) devices
    #[must_use]
    pub fn filter_available(devices: &[StorageDevice]) -> Vec<&StorageDevice> {
        devices.iter().filter(|device| !device.in_use).collect()
    }
}

impl Default for DeviceScanner {
    /// Returns the default instance
    fn default() -> Self {
        Self::new(DeviceDetectionConfig::default())
    }
}

#[cfg(test)]
impl DeviceScanner {
    pub(crate) fn test_parse_size_string(&self, size_str: &str) -> CoreResult<u64> {
        self.parse_size_string(size_str)
    }

    pub(crate) fn test_classify_device_speed(
        &self,
        device_type: DeviceType,
        model: &str,
    ) -> SpeedClass {
        self.classify_device_speed(device_type, model)
    }
}

#[cfg(test)]
mod device_detection_unit_tests {
    use super::{DeviceScanner, DeviceType, SpeedClass, StorageDevice};
    use crate::pool_setup::config::DeviceDetectionConfig;

    #[test]
    fn parse_size_string_units() {
        let s = DeviceScanner::new(DeviceDetectionConfig::default());
        assert_eq!(s.test_parse_size_string("1B").expect("B"), 1);
        assert_eq!(s.test_parse_size_string("2K").expect("K"), 2048);
        assert_eq!(s.test_parse_size_string("1M").expect("M"), 1024 * 1024);
        assert_eq!(s.test_parse_size_string("1G").expect("G"), 1024_u64.pow(3));
        assert_eq!(s.test_parse_size_string("1T").expect("T"), 1024_u64.pow(4));
    }

    #[test]
    fn classify_speed_nvme_gen4_and_hdd() {
        let s = DeviceScanner::new(DeviceDetectionConfig::default());
        assert_eq!(
            s.test_classify_device_speed(DeviceType::NvmeSsd, "PCIe Gen4 NVMe"),
            SpeedClass::UltraFast
        );
        assert_eq!(
            s.test_classify_device_speed(DeviceType::Hdd, "any"),
            SpeedClass::Slow
        );
        assert_eq!(
            s.test_classify_device_speed(DeviceType::Unknown, "x"),
            SpeedClass::Medium
        );
    }

    #[test]
    fn filter_by_type_and_speed_helpers() {
        let d1 = StorageDevice {
            device_path: "/dev/nvme0n1".into(),
            model: "x".into(),
            size_bytes: 1,
            device_type: DeviceType::NvmeSsd,
            speed_class: SpeedClass::Fast,
            in_use: false,
            current_use: None,
        };
        let d2 = StorageDevice {
            device_path: "/dev/sda".into(),
            model: "y".into(),
            size_bytes: 1,
            device_type: DeviceType::Hdd,
            speed_class: SpeedClass::Slow,
            in_use: true,
            current_use: None,
        };
        let all = vec![d1, d2];
        assert_eq!(
            DeviceScanner::filter_by_type(&all, DeviceType::Hdd).len(),
            1
        );
        assert_eq!(
            DeviceScanner::filter_by_speed(&all, SpeedClass::Fast).len(),
            1
        );
        assert_eq!(DeviceScanner::filter_available(&all).len(), 1);
    }

    #[test]
    fn parse_size_string_empty_and_plain_numeric() {
        let s = DeviceScanner::new(DeviceDetectionConfig::default());
        assert_eq!(s.test_parse_size_string("").expect("empty"), 0);
        assert_eq!(s.test_parse_size_string("4096").expect("plain"), 4096);
    }

    #[test]
    fn parse_size_string_extended_units() {
        let s = DeviceScanner::new(DeviceDetectionConfig::default());
        assert_eq!(s.test_parse_size_string("2KB").expect("KB"), 2048);
        assert_eq!(
            s.test_parse_size_string("1gb").expect("gb"),
            1024_u64.pow(3)
        );
        assert_eq!(
            s.test_parse_size_string("1pb").expect("pb"),
            1024_u64.pow(5)
        );
    }

    #[test]
    fn parse_size_string_errors() {
        let s = DeviceScanner::new(DeviceDetectionConfig::default());
        assert!(s.test_parse_size_string("10XB").is_err());
        assert!(s.test_parse_size_string("xyz").is_err());
        assert!(s.test_parse_size_string("12.5ZZ").is_err());
    }

    #[test]
    fn classify_speed_sata_tiers_and_nvme_default() {
        let s = DeviceScanner::new(DeviceDetectionConfig::default());
        assert_eq!(
            s.test_classify_device_speed(DeviceType::SataSsd, "Samsung Enterprise SSD"),
            SpeedClass::Fast
        );
        assert_eq!(
            s.test_classify_device_speed(DeviceType::SataSsd, "consumer sata"),
            SpeedClass::Medium
        );
        assert_eq!(
            s.test_classify_device_speed(DeviceType::NvmeSsd, "Generic NVMe"),
            SpeedClass::Fast
        );
        assert_eq!(
            s.test_classify_device_speed(DeviceType::OptaneMemory, "Intel Optane"),
            SpeedClass::UltraFast
        );
    }

    #[test]
    fn should_include_device_respects_loop_flag() {
        let cfg = DeviceDetectionConfig {
            include_loop_devices: false,
            ..Default::default()
        };
        let s = DeviceScanner::new(cfg);
        let loop_dev = StorageDevice {
            device_path: "/dev/loop0".into(),
            model: "loop".into(),
            size_bytes: 1,
            device_type: DeviceType::Unknown,
            speed_class: SpeedClass::Medium,
            in_use: false,
            current_use: None,
        };
        assert!(!s.should_include_device(&loop_dev));
        let cfg2 = DeviceDetectionConfig {
            include_loop_devices: true,
            ..Default::default()
        };
        let s2 = DeviceScanner::new(cfg2);
        assert!(s2.should_include_device(&loop_dev));
    }
}
