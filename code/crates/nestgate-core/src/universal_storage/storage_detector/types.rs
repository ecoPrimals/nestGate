use std::collections::HashMap;
//
// Core data structures and type definitions for storage detection system.

use crate::unified_enums::storage_types::{UnifiedStorageCapability, UnifiedStorageType};
use serde::{Deserialize, Serialize};

/// Comprehensive information about detected storage
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Detectedstorage
pub struct DetectedStorage {
    /// Unique identifier for this storage
    pub identifier: String,
    /// Type of storage system
    pub storage_type: UnifiedStorageType,
    /// Path or connection string
    /// Human-readable name
    pub display_name: String,
    /// What this storage system can do
    pub capabilities: Vec<UnifiedStorageCapability>,
    /// Performance characteristics
    pub performance_profile: PerformanceProfile,
    /// Available space in bytes
    pub available_space: u64,
    /// Reliability score (0.0 - 1.0)
    pub reliability_score: f64,
    /// Cost information
    pub cost_profile: CostProfile,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}
/// Performance profile of a storage system
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performanceprofile
pub struct PerformanceProfile {
    /// Sequential read throughput (MB/s)
    pub read_throughput_mbps: f64,
    /// Sequential write throughput (MB/s)
    pub write_throughput_mbps: f64,
    /// Random read latency (microseconds)
    pub read_latency_us: f64,
    /// Random write latency (microseconds)
    pub write_latency_us: f64,
    /// Input/Output Operations Per Second
    pub iops: u32,
    /// Whether this storage supports parallel operations
    pub supports_parallel_io: bool,
    /// Optimal block size for operations
    pub optimal_block_size: u32,
}
impl Default for PerformanceProfile {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            read_throughput_mbps: 100.0,
            write_throughput_mbps: 100.0,
            read_latency_us: 1000.0,
            write_latency_us: 2000.0,
            iops: 1000,
            supports_parallel_io: true,
            optimal_block_size: 4096,
        }
    }
}

/// Cost profile for storage (mainly for cloud storage)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Costprofile
pub struct CostProfile {
    /// Cost per GB per month (USD)
    pub storage_cost_per_gb_month: f64,
    /// Cost per request (USD)
    pub request_cost_per_thousand: f64,
    /// Data transfer cost per GB (USD)
    pub transfer_cost_per_gb: f64,
    /// Whether this is free tier eligible
    pub is_free_tier: bool,
}
impl Default for CostProfile {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            storage_cost_per_gb_month: 0.023, // AWS S3 standard pricing
            request_cost_per_thousand: 0.0004,
            transfer_cost_per_gb: 0.09,
            is_free_tier: false,
        }
    }
}

/// Storage analysis report
#[derive(Debug, Clone)]
/// Storageanalysisreport
pub struct StorageAnalysisReport {
    /// Filesystem Total
    pub filesystem_total: u64,
    /// Filesystem Used
    pub filesystem_used: u64,
    /// Filesystem Usage Percent
    pub filesystem_usage_percent: f64,
    /// Memory Total
    pub memory_total: u64,
    /// Memory Free
    pub memory_free: u64,
    /// Memory Usage Percent
    pub memory_usage_percent: f64,
    /// Recommendations
    pub recommendations: Vec<String>,
}
/// Filesystem statistics
#[derive(Debug, Clone)]
/// Filesystemstats
pub struct FilesystemStats {
    /// Total Bytes
    pub total_bytes: u64,
    /// Free Bytes
    pub free_bytes: u64,
    /// Used Bytes
    pub used_bytes: u64,
    /// Usage Percent
    pub usage_percent: f64,
    /// Inode Total
    pub inode_total: u64,
    /// Inode Free
    pub inode_free: u64,
    /// Filesystem Type
    pub filesystem_type: String,
    /// Mount Point
    pub mount_point: String,
    /// Device
    pub device: String,
}
/// Memory information
#[derive(Debug, Clone)]
/// Memoryinfo
pub struct MemoryInfo {
    /// Total Bytes
    pub total_bytes: u64,
    /// Available Bytes
    pub available_bytes: u64,
    /// Used Bytes
    pub used_bytes: u64,
    /// Usage Percent
    pub usage_percent: f64,
    /// Swap Total
    pub swap_total: u64,
    /// Swap Free
    pub swap_free: u64,
}
/// Cloud storage bucket information
#[derive(Debug, Clone)]
/// Cloudbucket
pub struct CloudBucket {
    /// Name
    pub name: String,
    /// Region
    pub region: String,
    /// Storage Class
    pub storage_class: String,
    /// Size Bytes
    pub size_bytes: u64,
    /// Count of object
    pub object_count: u64,
    /// Last Modified
    pub last_modified: Option<String>,
}
/// Network share information
#[derive(Debug, Clone)]
/// Networkshare
pub struct NetworkShare {
    /// Share name
    pub share_name: String,
    /// Protocol
    pub protocol: String, // SMB, NFS, etc.
    /// Server
    pub server: String,
    /// Mount Point
    pub mount_point: Option<String>,
    /// Available Space
    pub available_space: u64,
    /// Whether mounted
    pub is_mounted: bool,
}
/// Block device information
#[derive(Debug, Clone)]
/// Blockdevice
pub struct BlockDevice {
    /// Device name
    pub device_name: String,
    /// Size Bytes
    pub size_bytes: u64,
    /// Device Type
    pub device_type: String, // SSD, HDD, NVMe, etc.
    /// Whether removable
    pub is_removable: bool,
    /// Whether readonly
    pub is_readonly: bool,
    /// Filesystem
    pub filesystem: Option<String>,
}
impl DetectedStorage {
    /// Create a new detected storage entry
    #[must_use]
    pub fn new(identifier: String, storage_type: UnifiedStorageType, display_name: String) -> Self {
        Self {
            identifier,
            storage_type,
            display_name,
            capabilities: Vec::new(),
            performance_profile: PerformanceProfile::default(),
            available_space: 0,
            reliability_score: 0.5,
            cost_profile: CostProfile::default(),
            metadata: HashMap::new(),
        }
    }

    /// Add a capability to this storage
    pub fn add_capability(&mut self, capability: UnifiedStorageCapability) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
    }

    /// Check if this storage has a specific capability
    #[must_use]
    pub fn has_capability(&self, capability: &UnifiedStorageCapability) -> bool {
        self.capabilities.contains(capability)
    }

    /// Add metadata entry
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata value
    #[must_use]
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}
