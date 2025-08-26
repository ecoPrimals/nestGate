use std::collections::HashMap;
//
// Core data structures and type definitions for storage detection system.

use crate::universal_storage::{UnifiedStorageCapability, UnifiedStorageType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Comprehensive information about detected storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedStorage {
    /// Unique identifier for this storage
    pub identifier: String,
    /// Type of storage system
    pub storage_type: UnifiedStorageType,
    /// Path or connection string
    pub path: String,
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
pub struct StorageAnalysisReport {
    pub filesystem_total: u64,
    pub filesystem_used: u64,
    pub filesystem_usage_percent: f64,
    pub memory_total: u64,
    pub memory_free: u64,
    pub memory_usage_percent: f64,
    pub recommendations: Vec<String>,
}

/// Filesystem statistics
#[derive(Debug, Clone)]
pub struct FilesystemStats {
    pub total_bytes: u64,
    pub free_bytes: u64,
    pub used_bytes: u64,
    pub usage_percent: f64,
    pub inode_total: u64,
    pub inode_free: u64,
    pub filesystem_type: String,
    pub mount_point: String,
    pub device: String,
}

/// Memory information
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub used_bytes: u64,
    pub usage_percent: f64,
    pub swap_total: u64,
    pub swap_free: u64,
}

/// Cloud storage bucket information
#[derive(Debug, Clone)]
pub struct CloudBucket {
    pub name: String,
    pub region: String,
    pub storage_class: String,
    pub size_bytes: u64,
    pub object_count: u64,
    pub last_modified: Option<String>,
}

/// Network share information
#[derive(Debug, Clone)]
pub struct NetworkShare {
    pub share_name: String,
    pub protocol: String, // SMB, NFS, etc.
    pub server: String,
    pub mount_point: Option<String>,
    pub available_space: u64,
    pub is_mounted: bool,
}

/// Block device information
#[derive(Debug, Clone)]
pub struct BlockDevice {
    pub device_name: String,
    pub device_path: String,
    pub size_bytes: u64,
    pub device_type: String, // SSD, HDD, NVMe, etc.
    pub is_removable: bool,
    pub is_readonly: bool,
    pub filesystem: Option<String>,
}

impl DetectedStorage {
    /// Create a new detected storage entry
    pub fn new(
        identifier: String,
        storage_type: UnifiedStorageType,
        path: String,
        display_name: String,
    ) -> Self {
        Self {
            identifier,
            storage_type,
            path,
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
    pub fn has_capability(&self, capability: &UnifiedStorageCapability) -> bool {
        self.capabilities.contains(capability)
    }

    /// Add metadata entry
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
} 