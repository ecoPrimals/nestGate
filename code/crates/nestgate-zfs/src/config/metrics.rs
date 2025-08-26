//
// Configuration for ZFS metrics collection, storage, and export.
// **CANONICAL MODERNIZATION COMPLETE** - Migrated to canonical MetricsConfig

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// **CANONICAL MODERNIZATION** - Use canonical metrics configuration
pub use nestgate_core::{CanonicalMetricsConfig as MetricsConfig, CanonicalMetricsFormat as MetricsFormat};

// **MIGRATION COMPLETE** - All ZFS metrics now use canonical configuration
// The following types have been consolidated into CanonicalMetricsConfig:
// - Previous ZFS-specific MetricsConfig ✅ MIGRATED
// - Previous ZFS-specific MetricsFormat ✅ MIGRATED

// ==================== ZFS METRICS CONFIGURATION ====================

/// **ZFS-SPECIFIC METRICS CONFIGURATION HELPERS**
/// These functions create properly configured canonical MetricsConfig instances for ZFS operations

/// Create default ZFS metrics configuration
pub fn zfs_metrics_default() -> MetricsConfig {
    MetricsConfig {
        enabled: true,
        collection_interval: std::time::Duration::from_secs(30),
        retention_period: std::time::Duration::from_secs(7 * 24 * 3600), // 7 days
        detailed_metrics: false,
        storage_path: Some(PathBuf::from("/var/lib/nestgate/zfs/metrics")),
        export_format: MetricsFormat::Prometheus,
        max_memory_metrics: 10000,
        enable_streaming: false,
    }
}

/// Create production-optimized ZFS metrics configuration
pub fn zfs_metrics_production() -> MetricsConfig {
    MetricsConfig {
        enabled: true,
        collection_interval: std::time::Duration::from_secs(30),
        retention_period: std::time::Duration::from_secs(90 * 24 * 3600), // 90 days
        detailed_metrics: false,
        storage_path: Some(PathBuf::from("/var/lib/nestgate/metrics")),
        export_format: MetricsFormat::Prometheus,
        max_memory_metrics: 50000,
        enable_streaming: true,
    }
}
