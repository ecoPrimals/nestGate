//! # Universal Block Storage Detection Trait
//!
//! **UNIVERSAL ARCHITECTURE** - Platform-agnostic block device detection
//! **EVOLUTION**: Phase 2 - Deep Debt Evolution (Jan 31, 2026)
//!
//! Provides a trait-based abstraction for detecting block storage devices
//! across all platforms with runtime selection of the best strategy.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │   UniversalBlockDetector            │
//! │   (Runtime Platform Selection)      │
//! └──────────────┬──────────────────────┘
//!                │
//!       ┌────────┴────────┐
//!       │                 │
//! ┌─────▼─────┐    ┌─────▼──────┐
//! │  Sysinfo  │    │ Platform   │
//! │ Universal │    │ Optimized  │
//! │ Detector  │    │ Detectors  │
//! └───────────┘    └────────────┘
//!       │                 │
//!       │          ┌──────┴──────┐
//!       │          │             │
//!       │    ┌─────▼────┐  ┌────▼─────┐
//!       │    │  Linux   │  │ Windows  │
//!       │    │/sys/block│  │   WMI    │
//!       │    └──────────┘  └──────────┘
//!       │
//!       └──────► Works Everywhere!
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use nestgate_core::universal_storage::block_detection::{UniversalBlockDetector, BlockDevice};
//!
//! async fn detect_devices() -> Result<Vec<BlockDevice>> {
//!     // Auto-selects best detector for platform
//!     let detector = UniversalBlockDetector::new().await?;
//!     detector.detect_devices().await
//! }
//! ```

use super::{BlockDevice, DeviceType, Result};
use async_trait::async_trait;
use std::path::PathBuf;
use sysinfo::{Disks, DisksExt};
use tracing::{debug, info, warn};

/// Universal trait for block device detection
///
/// Implementations provide platform-specific or universal device detection
#[async_trait]
pub trait BlockDeviceDetector: Send + Sync {
    /// Detect all available block devices
    async fn detect(&self) -> Result<Vec<BlockDevice>>;
    
    /// Get detector name for logging
    fn name(&self) -> &str;
    
    /// Check if this detector is available on current platform
    fn is_available(&self) -> bool;
}

/// Universal block device detector using sysinfo crate
///
// ecoBin v3.0: `sysinfo` fallback when Linux `/sys/block` is unavailable; primary Linux path is [`LinuxSysfsDetector`].
/// **UNIVERSAL** - Works on all platforms that sysinfo supports:
/// - Linux (all architectures)
/// - Windows (x86_64, ARM64)
/// - macOS (Intel, Apple Silicon)
/// - BSD variants
/// - Any future platform sysinfo adds!
pub struct SysinfoDetector;

#[async_trait]
impl BlockDeviceDetector for SysinfoDetector {
    async fn detect(&self) -> Result<Vec<BlockDevice>> {
        info!("🔍 Detecting block devices using universal sysinfo detector");
        
        let disks = Disks::new_with_refreshed_list();
        let mut devices = Vec::new();
        
        for disk in disks.iter() {
            let name = disk.name().to_string_lossy().to_string();
            let mount_point = disk.mount_point().to_path_buf();
            
            // Determine device type based on disk properties
            let device_type = if disk.is_removable() {
                DeviceType::Virtual // USB/removable
            } else {
                // sysinfo doesn't expose SSD vs HDD directly
                // Use heuristic: NVMe devices are typically SSDs
                if name.contains("nvme") {
                    DeviceType::NVMe
                } else {
                    DeviceType::Unknown
                }
            };
            
            let device = BlockDevice {
                name: name.clone(),
                path: mount_point.clone(),
                size: disk.total_space(),
                block_size: 4096, // Standard 4KB blocks
                device_type,
                supports_trim: device_type == DeviceType::SSD || device_type == DeviceType::NVMe,
                metadata: std::collections::HashMap::new(),
            };
            
            devices.push(device);
            debug!("📦 Detected device: {} ({} bytes, type: {:?})", 
                   name, disk.total_space(), device_type);
        }
        
        info!("✅ Detected {} block devices universally", devices.len());
        Ok(devices)
    }
    
    fn name(&self) -> &str {
        "sysinfo-universal"
    }
    
    fn is_available(&self) -> bool {
        true // Always available!
    }
}

/// Linux-optimized block device detector using /sys/block
///
/// **OPTIMIZATION** - Fast path for Linux systems with /sys/block
/// Falls back to universal detector if /sys/block unavailable
pub struct LinuxSysfsDetector;

#[async_trait]
impl BlockDeviceDetector for LinuxSysfsDetector {
    async fn detect(&self) -> Result<Vec<BlockDevice>> {
        info!("🔍 Detecting block devices using Linux /sys/block optimization");
        
        let sys_block = PathBuf::from("/sys/block");
        if !sys_block.exists() {
            warn!("⚠️  /sys/block not available, falling back to universal detector");
            return SysinfoDetector.detect().await;
        }
        
        let mut devices = Vec::new();
        let mut entries = tokio::fs::read_dir(&sys_block).await.map_err(|e| {
            crate::error::NestGateError::io_error(e, "Failed to read /sys/block", "block_storage")
        })?;
        
        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            crate::error::NestGateError::io_error(e, "Failed to read dir entry", "block_storage")
        })? {
            let device_name = entry.file_name().to_string_lossy().to_string();
            
            // Skip loop devices, ramdisks, etc.
            if device_name.starts_with("loop") || device_name.starts_with("ram") {
                continue;
            }
            
            let device_path = PathBuf::from(format!("/dev/{}", device_name));
            let device_type = self.determine_device_type(&device_name).await;
            let size = self.get_device_size(&device_name).await.unwrap_or(0);
            let supports_trim = self.check_trim_support(&device_name).await;
            
            let device = BlockDevice {
                name: device_name.clone(),
                path: device_path,
                size,
                block_size: 4096,
                device_type,
                supports_trim,
                metadata: std::collections::HashMap::new(),
            };
            
            devices.push(device);
            debug!("📦 Detected device: {} ({} bytes, type: {:?})", 
                   device_name, size, device_type);
        }
        
        info!("✅ Detected {} block devices via Linux optimization", devices.len());
        Ok(devices)
    }
    
    fn name(&self) -> &str {
        "linux-sysfs-optimized"
    }
    
    fn is_available(&self) -> bool {
        PathBuf::from("/sys/block").exists()
    }
}

impl LinuxSysfsDetector {
    /// Determine device type from sysfs
    async fn determine_device_type(&self, device_name: &str) -> DeviceType {
        // Check for NVMe
        if device_name.starts_with("nvme") {
            return DeviceType::NVMe;
        }
        
        // Check rotational flag
        let rotational_path = format!("/sys/block/{}/queue/rotational", device_name);
        if let Ok(content) = tokio::fs::read_to_string(&rotational_path).await {
            return if content.trim() == "0" {
                DeviceType::SSD
            } else if content.trim() == "1" {
                DeviceType::HDD
            } else {
                DeviceType::Unknown
            };
        }
        
        DeviceType::Unknown
    }
    
    /// Get device size from sysfs
    async fn get_device_size(&self, device_name: &str) -> Result<u64> {
        let size_path = format!("/sys/block/{}/size", device_name);
        let content = tokio::fs::read_to_string(&size_path).await.map_err(|e| {
            crate::error::NestGateError::io_error(e, "Failed to read device size", "block_storage")
        })?;
        
        let sectors: u64 = content.trim().parse().map_err(|e| {
            crate::error::NestGateError::parse_error(
                format!("Invalid size value: {}", e),
                "block_storage",
            )
        })?;
        
        // Sectors are typically 512 bytes
        Ok(sectors * 512)
    }
    
    /// Check if device supports TRIM/DISCARD
    async fn check_trim_support(&self, device_name: &str) -> bool {
        let discard_path = format!("/sys/block/{}/queue/discard_granularity", device_name);
        if let Ok(content) = tokio::fs::read_to_string(&discard_path).await {
            if let Ok(granularity) = content.trim().parse::<u32>() {
                return granularity > 0;
            }
        }
        false
    }
}

/// Universal block device detector with automatic platform selection
///
/// **ADAPTIVE** - Selects best detector based on platform and availability:
/// 1. Linux /sys/block (if available) - fastest
/// 2. Universal sysinfo - works everywhere
pub struct UniversalBlockDetector {
    detector: Box<dyn BlockDeviceDetector>,
}

impl UniversalBlockDetector {
    /// Create new universal detector with automatic platform selection
    ///
    /// **CAPABILITY-BASED** - Runtime detection, not compile-time!
    pub async fn new() -> Result<Self> {
        info!("🔍 Initializing universal block device detector");
        
        // Try platform-optimized detectors first
        #[cfg(target_os = "linux")]
        {
            let linux_detector = LinuxSysfsDetector;
            if linux_detector.is_available() {
                info!("✅ Using Linux /sys/block optimization (fastest path)");
                return Ok(Self {
                    detector: Box::new(linux_detector),
                });
            }
        }
        
        // Fall back to universal detector (works everywhere!)
        info!("✅ Using universal sysinfo detector (cross-platform)");
        Ok(Self {
            detector: Box::new(SysinfoDetector),
        })
    }
    
    /// Detect all available block devices
    pub async fn detect_devices(&self) -> Result<Vec<BlockDevice>> {
        self.detector.detect().await
    }
    
    /// Get detector name for diagnostics
    pub fn detector_name(&self) -> &str {
        self.detector.name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_universal_detector_creation() {
        let detector = UniversalBlockDetector::new().await;
        assert!(detector.is_ok(), "Universal detector should always be available");
    }
    
    #[tokio::test]
    // ecoBin v3.0: exercises sysinfo-only fallback detector (Linux prefers `/sys/block`).
    async fn test_sysinfo_detector_available() {
        let detector = SysinfoDetector;
        assert!(detector.is_available(), "Sysinfo detector should always be available");
    }
    
    #[tokio::test]
    async fn test_device_detection() {
        let detector = UniversalBlockDetector::new().await.expect("Failed to create detector");
        let devices = detector.detect_devices().await;
        
        // Should succeed even if no devices found
        assert!(devices.is_ok(), "Device detection should not fail");
        
        if let Ok(devices) = devices {
            println!("Detected {} devices", devices.len());
            for device in devices {
                println!("  - {}: {} bytes", device.name, device.size);
            }
        }
    }
}
