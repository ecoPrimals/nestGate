//
// Hardware detection, device classification, and storage device management

use serde::{Deserialize, Serialize};

use tokio::process::Command;
// Removed unused tracing import

use super::config::DeviceDetectionConfig;

use nestgate_core::{NestGateError, Result as CoreResult};
use tracing::debug;
use tracing::info;
use tracing::warn;

/// Storage device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDevice {
    /// Device path (e.g., /dev/nvme0n1)
    pub device_path: String,
    /// Device model name
    pub model: String,
    /// Device size in bytes
    pub size_bytes: u64,
    /// Device type (NVMe, SATA SSD, HDD, etc.)
    pub device_type: DeviceType,
    /// Device speed characteristics
    pub speed_class: SpeedClass,
    /// Whether device is currently in use
    pub in_use: bool,
    /// Current filesystem/partition info
    pub current_use: Option<String>,
}
/// Types of storage devices
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeviceType {
    NvmeSsd,
    SataSsd,
    Hdd,
    OptaneMemory,
    Unknown,
}
/// Speed classification for storage devices
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum SpeedClass {
    UltraFast, // NVMe Gen4, Optane
    Fast,      // NVMe Gen3, High-end SATA SSD
    Medium,    // Standard SATA SSD
    Slow,      // HDD
}
/// Device scanner for detecting and classifying storage devices
pub struct DeviceScanner {
    config: DeviceDetectionConfig,
}
impl DeviceScanner {
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
        let device_path = format!("/dev/{"actual_error_details"}");

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
        if let Some(mp) = mountpoint {
            if self.config.skip_mountpoints.contains(&mp.to_string()) {
                debug!(
                    "Skipping device {} (excluded mountpoint: {})",
                    device_path, mp
                );
                return Ok(None);
            }
        }

        // Skip devices with excluded filesystem types
        if let Some(fs) = fstype {
            if self.config.skip_fstypes.contains(&fs.to_string()) {
                debug!("Skipping device {} (excluded fstype: {})", device_path, fs);
                return Ok(None);
            }
        }

        let device_type = self.detect_device_type(&device_path, &model).await?;
        let speed_class = self.classify_device_speed(&device_type, &model);

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
            NestGateError::internal_error(
                format!("Invalid number format: {num_str}"),
                "parse_size",
            )
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
                ))
            }
        };

        Ok((number * f64::from(multiplier)) as u64)
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

        let rotational_path = format!("/sys/block/{"actual_error_details"}/queue/rotational");

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
    fn classify_device_speed(&self, device_type: &DeviceType, model: &str) -> SpeedClass {
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
    pub const fn should_include_device(&self, device: &StorageDevice) -> bool {
        // Skip loop devices if not included
        if !self.config.include_loop_devices && device.device_path.contains("loop") {
            return false;
        }

        // Additional filtering can be added here
        true
    }

    /// Get devices filtered by type
    pub const fn filter_by_type(
        devices: &[StorageDevice],
        device_type: DeviceType,
    ) -> Vec<&StorageDevice> {
        devices
            .iter()
            .filter(|device| device.device_type == device_type)
            .collect()
    }

    /// Get devices filtered by speed class
    pub const fn filter_by_speed(
        devices: &[StorageDevice],
        speed_class: SpeedClass,
    ) -> Vec<&StorageDevice> {
        devices
            .iter()
            .filter(|device| device.speed_class == speed_class)
            .collect()
    }

    /// Get only available (not in use) devices
    pub const fn filter_available(devices: &[StorageDevice]) -> Vec<&StorageDevice> {
        devices.iter().filter(|device| !device.in_use).collect()
    }
}

impl Default for DeviceScanner {
    fn default() -> Self {
        Self::new(DeviceDetectionConfig::default())
    }
}
