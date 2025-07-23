//! Metrics Configuration Module
//!
//! Configuration for ZFS metrics collection, storage, and export.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Metrics collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Collection interval in seconds
    pub collection_interval_seconds: u64,
    /// Retention period in days
    pub retention_days: u32,
    /// Metrics storage path
    pub storage_path: Option<PathBuf>,
    /// Export format (prometheus, json, etc.)
    pub export_format: MetricsFormat,
}

/// Metrics export format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricsFormat {
    Prometheus,
    Json,
    InfluxDb,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval_seconds: 30,
            retention_days: 7,
            storage_path: Some(PathBuf::from("/var/lib/nestgate/zfs/metrics")),
            export_format: MetricsFormat::Prometheus,
        }
    }
}

impl MetricsConfig {
    /// Create production-optimized metrics configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            collection_interval_seconds: 30,
            retention_days: 90,
            storage_path: Some(PathBuf::from("/var/lib/nestgate/metrics")),
            export_format: MetricsFormat::Prometheus,
        }
    }
}
