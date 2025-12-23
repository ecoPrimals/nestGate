//! Storage metrics types and data structures.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Storage analytics data point
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storageanalyticspoint
pub struct StorageAnalyticsPoint {
    /// Timestamp of the data point
    pub timestamp: SystemTime,
    /// Dataset or pool name
    pub dataset_name: String,
    /// Performance metrics
    pub performance: PerformanceMetrics,
    /// Usage metrics
    pub usage: UsageMetrics,
    /// I/O metrics
    pub io_metrics: IoMetrics,
    /// Health metrics
    pub health: HealthMetrics,
}

/// Performance metrics for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancemetrics
pub struct PerformanceMetrics {
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Memory usage percentage
    pub memory_usage_percent: f64,
    /// Average I/O latency in milliseconds
    pub avg_io_latency_ms: f64,
    /// Peak I/O latency in milliseconds
    pub peak_io_latency_ms: f64,
    /// Throughput in bytes per second
    pub throughput_bps: u64,
    /// IOPS (Input/Output Operations Per Second)
    pub iops: u32,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
    /// Queue depth
    pub queue_depth: u32,
}

/// Usage metrics for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Usagemetrics
pub struct UsageMetrics {
    /// Total capacity in bytes
    pub total_capacity_bytes: u64,
    /// Used space in bytes
    pub used_bytes: u64,
    /// Available space in bytes
    pub available_bytes: u64,
    /// Usage percentage
    pub usage_percent: f64,
    /// Number of files/objects
    pub file_count: u64,
    /// Average file size in bytes
    pub avg_file_size_bytes: u64,
    /// Compression ratio
    pub compression_ratio: f64,
    /// Deduplication savings
    pub dedup_savings_percent: f64,
}

/// I/O metrics for storage operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Iometrics
pub struct IoMetrics {
    /// Read operations per second
    pub reads_per_sec: u32,
    /// Write operations per second
    pub writes_per_sec: u32,
    /// Bytes read per second
    pub read_bytes_per_sec: u64,
    /// Bytes written per second
    pub write_bytes_per_sec: u64,
    /// Average read latency in milliseconds
    pub avg_read_latency_ms: f64,
    /// Average write latency in milliseconds
    pub avg_write_latency_ms: f64,
    /// Read queue depth
    pub read_queue_depth: u32,
    /// Write queue depth
    pub write_queue_depth: u32,
}

/// Health metrics for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Healthmetrics
pub struct HealthMetrics {
    /// Overall health score (0.0 to 1.0)
    pub health_score: f64,
    /// Number of errors in the last period
    pub error_count: u32,
    /// Number of warnings in the last period
    pub warning_count: u32,
    /// Last scrub status
    pub scrub_status: ScrubStatus,
    /// Temperature in Celsius (for drives that support it)
    pub temperature_celsius: Option<f32>,
    /// Drive health status
    pub drive_health: Vec<DriveHealth>,
}

/// ZFS scrub status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Scrub
pub enum ScrubStatus {
    /// No scrub has been performed
    None,
    /// Scrub is currently in progress
    InProgress { percent_complete: f64 },
    /// Scrub completed successfully
    Completed { last_completion: SystemTime },
    /// Scrub failed
    Failed { error_message: String },
    /// Scrub was cancelled
    Cancelled,
}

/// Individual drive health information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Drivehealth
pub struct DriveHealth {
    /// Drive identifier
    pub drive_id: String,
    /// Health status
    pub status: DriveStatus,
    /// SMART attributes if available
    pub smart_attributes: Option<SmartAttributes>,
}

/// Drive health status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Drive
pub enum DriveStatus {
    /// Drive is healthy
    Healthy,
    /// Drive has warnings but is functional
    Warning,
    /// Drive is failing or has failed
    Critical,
    /// Drive status is unknown
    Unknown,
}

/// SMART attributes for drive health monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Smartattributes
pub struct SmartAttributes {
    /// Power-on hours
    pub power_on_hours: u64,
    /// Power cycle count
    pub power_cycle_count: u32,
    /// Reallocated sectors count
    pub reallocated_sectors: u32,
    /// Current pending sectors
    pub pending_sectors: u32,
    /// Uncorrectable sectors
    pub uncorrectable_sectors: u32,
} 