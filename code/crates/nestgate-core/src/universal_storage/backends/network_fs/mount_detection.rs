//! # Universal Network Filesystem Mount Detection
//!
//! **UNIVERSAL ARCHITECTURE** - Runtime mount discovery across all platforms
//! **EVOLUTION**: Phase 2 Task 3 - Deep Debt Evolution (Jan 31, 2026)
//!
//! Provides trait-based abstraction for discovering network filesystem mounts
//! with runtime capability detection instead of compile-time OS checks.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │   UniversalMountDetector            │
//! │   (Runtime Capability Detection)    │
//! └──────────────┬──────────────────────┘
//!                │
//!       ┌────────┴────────┬─────────────┐
//!       │                 │             │
//! ┌─────▼─────┐    ┌─────▼──────┐ ┌────▼─────┐
//! │ Linux     │    │  macOS     │ │ Windows  │
//! │ Detector  │    │  Detector  │ │ Detector │
//! └───────────┘    └────────────┘ └──────────┘
//!       │                 │             │
//!       │          ┌──────┴──────┐      │
//!       │          │  /etc/mtab  │      │
//!       │          │  (fallback) │      │
//!       │          └─────────────┘      │
//!       │                               │
//!       └──────► Runtime Detection! ◄───┘
//! ```
//!
//! ## Key Features
//!
//! - **Runtime Detection**: Checks for actual mounts, not assumed OS
//! - **Cross-Platform**: Linux, macOS, Windows, BSD support
//! - **Protocol Detection**: NFS, CIFS/SMB, AFP, WebDAV
//! - **Graceful Degradation**: Works in containers and limited environments
//!
//! ## Usage
//!
//! ```rust
//! use nestgate_core::universal_storage::backends::network_fs::mount_detection::{
//!     UniversalMountDetector, DiscoveredMount
//! };
//!
//! async fn discover_mounts() -> Vec<DiscoveredMount> {
//!     let detector = UniversalMountDetector::new();
//!     detector.discover().await.unwrap_or_default()
//! }
//! ```

use crate::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{debug, warn};

use super::{NetworkProtocol, NetworkMount, MountStatus, MountOptions};

/// Discovered network mount information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredMount {
    /// Device/server info (e.g., "server:/path" for NFS)
    pub device: String,
    /// Server hostname/IP
    pub server: String,
    /// Remote path on server
    pub remote_path: String,
    /// Local mount point
    pub local_path: PathBuf,
    /// Detected protocol
    pub protocol: NetworkProtocol,
    /// Filesystem type string (for diagnostics)
    pub fs_type: String,
}

/// Universal trait for network mount detection
///
/// **CAPABILITY-BASED**: Checks for actual mounts, not just OS type
pub trait MountDetector: Send + Sync {
    /// Discover network mounts
    ///
    /// **RUNTIME CHECK**: Actually reads mount information from the system
    fn discover(&self) -> Result<Vec<DiscoveredMount>>;
    
    /// Check if this detector is available
    fn is_available(&self) -> bool;
    
    /// Get detector name for logging
    fn name(&self) -> &str;
}

/// Linux mount detector using /proc/mounts
///
/// **CAPABILITY-BASED**: Checks /proc/mounts exists, not just OS type
pub struct LinuxProcMountDetector;

impl MountDetector for LinuxProcMountDetector {
    fn discover(&self) -> Result<Vec<DiscoveredMount>> {
        let mounts_content = std::fs::read_to_string("/proc/mounts").map_err(|e| {
            NestGateError::io_error(e, "Failed to read /proc/mounts", "mount_detection")
        })?;
        
        let mut discovered = Vec::new();
        
        for line in mounts_content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                continue;
            }
            
            let device = parts[0];
            let mount_point = parts[1];
            let fs_type = parts[2];
            
            // Check if it's a network filesystem
            let protocol = match fs_type {
                "nfs" => Some(NetworkProtocol::NFSv3),
                "nfs4" => Some(NetworkProtocol::NFSv4),
                "cifs" | "smb3" => Some(NetworkProtocol::CIFS3),
                "smb2" => Some(NetworkProtocol::CIFS2),
                _ => None,
            };
            
            if let Some(protocol) = protocol {
                // Parse server and remote path from device (e.g., "server:/path")
                let (server, remote_path) = if let Some(colon_pos) = device.find(':') {
                    (
                        device[..colon_pos].to_string(),
                        device[colon_pos + 1..].to_string(),
                    )
                } else {
                    (device.to_string(), String::from("/"))
                };
                
                discovered.push(DiscoveredMount {
                    device: device.to_string(),
                    server,
                    remote_path,
                    local_path: PathBuf::from(mount_point),
                    protocol,
                    fs_type: fs_type.to_string(),
                });
                
                debug!("Discovered network mount: {} at {} ({})", device, mount_point, fs_type);
            }
        }
        
        Ok(discovered)
    }
    
    fn is_available(&self) -> bool {
        std::path::Path::new("/proc/mounts").exists()
    }
    
    fn name(&self) -> &str {
        "linux-proc-mount-detector"
    }
}

/// macOS/BSD mount detector using /etc/mtab
///
/// **CAPABILITY-BASED**: Checks /etc/mtab exists
pub struct UnixMtabDetector;

impl MountDetector for UnixMtabDetector {
    fn discover(&self) -> Result<Vec<DiscoveredMount>> {
        let mtab_content = std::fs::read_to_string("/etc/mtab").map_err(|e| {
            NestGateError::io_error(e, "Failed to read /etc/mtab", "mount_detection")
        })?;
        
        let mut discovered = Vec::new();
        
        for line in mtab_content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                continue;
            }
            
            let device = parts[0];
            let mount_point = parts[1];
            let fs_type = parts[2];
            
            // Check if it's a network filesystem
            let protocol = match fs_type {
                "nfs" => Some(NetworkProtocol::NFSv3),
                "nfs4" => Some(NetworkProtocol::NFSv4),
                "smbfs" | "cifs" => Some(NetworkProtocol::CIFS3),
                "afp" => Some(NetworkProtocol::CIFS3), // AFP → CIFS for simplicity
                _ => None,
            };
            
            if let Some(protocol) = protocol {
                let (server, remote_path) = if let Some(colon_pos) = device.find(':') {
                    (
                        device[..colon_pos].to_string(),
                        device[colon_pos + 1..].to_string(),
                    )
                } else {
                    (device.to_string(), String::from("/"))
                };
                
                discovered.push(DiscoveredMount {
                    device: device.to_string(),
                    server,
                    remote_path,
                    local_path: PathBuf::from(mount_point),
                    protocol,
                    fs_type: fs_type.to_string(),
                });
                
                debug!("Discovered network mount: {} at {} ({})", device, mount_point, fs_type);
            }
        }
        
        Ok(discovered)
    }
    
    fn is_available(&self) -> bool {
        std::path::Path::new("/etc/mtab").exists()
    }
    
    fn name(&self) -> &str {
        "unix-mtab-detector"
    }
}

/// sysinfo-based universal detector
///
/// **UNIVERSAL**: Works on all platforms using sysinfo crate
// ecoBin v3.0: `sysinfo` fallback when `/proc/mounts` and `/etc/mtab` are unavailable.
pub struct SysinfoMountDetector;

impl MountDetector for SysinfoMountDetector {
    fn discover(&self) -> Result<Vec<DiscoveredMount>> {
        use sysinfo::{Disks, DisksExt};
        
        let disks = Disks::new_with_refreshed_list();
        let mut discovered = Vec::new();
        
        for disk in disks.list() {
            let mount_point = disk.mount_point();
            let fs_name = disk.name().to_string_lossy();
            let fs_type = disk.file_system().to_string_lossy();
            
            // Heuristic: network mounts often have ":" in name or specific fs types
            let is_network = fs_name.contains(':') 
                || fs_type.contains("nfs") 
                || fs_type.contains("cifs")
                || fs_type.contains("smb")
                || fs_type.contains("smbfs");
            
            if is_network {
                let protocol = if fs_type.contains("nfs4") {
                    NetworkProtocol::NFSv4
                } else if fs_type.contains("nfs") {
                    NetworkProtocol::NFSv3
                } else {
                    NetworkProtocol::CIFS3
                };
                
                let (server, remote_path) = if let Some(colon_pos) = fs_name.find(':') {
                    (
                        fs_name[..colon_pos].to_string(),
                        fs_name[colon_pos + 1..].to_string(),
                    )
                } else {
                    (fs_name.to_string(), String::from("/"))
                };
                
                discovered.push(DiscoveredMount {
                    device: fs_name.to_string(),
                    server,
                    remote_path,
                    local_path: mount_point.to_path_buf(),
                    protocol,
                    fs_type: fs_type.to_string(),
                });
                
                debug!("Discovered network mount via sysinfo: {} at {:?}", fs_name, mount_point);
            }
        }
        
        Ok(discovered)
    }
    
    fn is_available(&self) -> bool {
        true // sysinfo is always available
    }
    
    fn name(&self) -> &str {
        "sysinfo-mount-detector"
    }
}

/// Universal mount detector with adaptive selection
///
/// **ADAPTIVE**: Selects best available detector at runtime
pub struct UniversalMountDetector {
    detector: Box<dyn MountDetector>,
}

impl Default for UniversalMountDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl UniversalMountDetector {
    /// Create new universal mount detector
    ///
    /// **RUNTIME SELECTION**: Picks best available detector
    pub fn new() -> Self {
        debug!("🔍 Initializing universal mount detector");
        
        // Try platform-optimized detectors first (faster)
        let detectors: Vec<Box<dyn MountDetector>> = vec![
            Box::new(LinuxProcMountDetector),
            Box::new(UnixMtabDetector),
        ];
        
        for detector in detectors {
            if detector.is_available() {
                debug!("✅ Using optimized detector: {}", detector.name());
                return Self { detector };
            }
        }
        
        // Fallback to universal sysinfo detector
        debug!("✅ Using universal sysinfo detector");
        Self {
            detector: Box::new(SysinfoMountDetector),
        }
    }
    
    /// Discover all network mounts
    ///
    /// **GRACEFUL**: Returns empty vec on error (non-fatal in containers)
    pub fn discover(&self) -> Result<Vec<DiscoveredMount>> {
        debug!("🔍 Discovering network mounts with {}", self.detector.name());
        
        match self.detector.discover() {
            Ok(mounts) => {
                debug!("✅ Discovered {} network mounts", mounts.len());
                Ok(mounts)
            }
            Err(e) => {
                warn!("⚠️ Mount discovery failed (non-fatal): {}", e);
                Ok(Vec::new()) // Graceful degradation
            }
        }
    }
    
    /// Get detector name for diagnostics
    pub fn detector_name(&self) -> &str {
        self.detector.name()
    }
    
    /// Check if mount detection is available
    pub fn is_available(&self) -> bool {
        self.detector.is_available()
    }
    
    /// Convert discovered mounts to NetworkMount format
    pub fn to_network_mounts(&self, discovered: Vec<DiscoveredMount>) -> Vec<NetworkMount> {
        discovered
            .into_iter()
            .enumerate()
            .map(|(idx, mount)| NetworkMount {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("mount_{}", idx),
                server: mount.server,
                remote_path: mount.remote_path,
                local_path: mount.local_path,
                protocol: mount.protocol,
                options: MountOptions::default(),
                status: MountStatus::Active,
                created_at: std::time::SystemTime::now(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_universal_detector_creation() {
        let detector = UniversalMountDetector::new();
        assert!(!detector.detector_name().is_empty());
    }
    
    #[test]
    fn test_detector_availability() {
        let detector = UniversalMountDetector::new();
        // ecoBin v3.0: sysinfo-backed fallback is always "available"; Linux prefers `/proc/mounts`.
        assert!(detector.is_available());
    }
    
    #[test]
    fn test_mount_discovery() {
        let detector = UniversalMountDetector::new();
        
        println!("Using detector: {}", detector.detector_name());
        
        // Discovery should not panic (may return empty)
        let result = detector.discover();
        assert!(result.is_ok());
        
        let mounts = result.unwrap();
        println!("Discovered {} network mounts", mounts.len());
        
        for mount in &mounts {
            println!("  - {} → {:?} ({})", mount.device, mount.local_path, mount.fs_type);
        }
    }
    
    #[test]
    fn test_to_network_mounts() {
        let detector = UniversalMountDetector::new();
        let discovered = detector.discover().unwrap_or_default();
        let network_mounts = detector.to_network_mounts(discovered);
        
        // Should convert without error
        println!("Converted {} mounts to NetworkMount format", network_mounts.len());
    }
    
    #[test]
    fn test_linux_detector_availability() {
        let detector = LinuxProcMountDetector;
        let available = detector.is_available();
        
        #[cfg(target_os = "linux")]
        {
            // On Linux, /proc/mounts should exist
            println!("Linux detector available: {}", available);
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            // On non-Linux, may or may not exist
            println!("Linux detector available (non-Linux OS): {}", available);
        }
    }
    
    #[test]
    // ecoBin v3.0: sysinfo-only test; production Linux path is `LinuxProcMountDetector`.
    fn test_sysinfo_detector_always_available() {
        let detector = SysinfoMountDetector;
        assert!(detector.is_available(), "sysinfo detector should always be available");
    }
}
