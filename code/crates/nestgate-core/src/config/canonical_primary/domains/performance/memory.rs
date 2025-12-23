//! Memory performance configuration module
//! Provides unified memory management and optimization settings.

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{NestGateError, Result};

/// Memory performance configuration for optimizing memory usage and allocation.
///
/// Controls memory pooling, garbage collection, and monitoring to optimize
/// application memory footprint and performance.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for MemoryPerformance
pub struct MemoryPerformanceConfig {
    /// Memory pool configuration for object pooling.
    pub pool: MemoryPoolConfig,

    /// Garbage collection settings for memory reclamation.
    pub gc: GarbageCollectionConfig,

    /// Memory monitoring for usage tracking and alerts.
    pub monitoring: MemoryMonitoringConfig,
}

/// Memory pool configuration for efficient object allocation and reuse.
///
/// Implements object pooling to reduce allocation overhead and improve performance.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for MemoryPool
pub struct MemoryPoolConfig {
    /// Initial pool size in bytes (default: 1MB).
    pub initial_size: usize,

    /// Maximum pool size in bytes (default: 1GB).
    pub max_size: usize,

    /// Growth factor for pool expansion (default: 2.0 = double on growth).
    pub growth_factor: f64,
}

/// Garbage collection configuration for automatic memory reclamation.
///
/// Controls when and how memory is reclaimed from unused objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for GarbageCollection
pub struct GarbageCollectionConfig {
    /// Whether garbage collection is enabled (default: true).
    pub enabled: bool,

    /// Interval between GC runs (default: 60 seconds).
    pub interval: Duration,

    /// Memory usage threshold to trigger GC (0.0-1.0, default: 0.8 = 80%).
    pub threshold: f64,
}

/// Memory monitoring configuration for tracking memory usage.
///
/// Enables alerts and metrics for memory consumption.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for MemoryMonitoring
pub struct MemoryMonitoringConfig {
    /// Whether memory monitoring is enabled (default: true).
    pub enabled: bool,

    /// Memory usage threshold for alerts (0.0-1.0, default: 0.8 = 80%).
    pub usage_threshold: f64,
}

impl Default for MemoryPoolConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            initial_size: 1024 * 1024,    // 1MB
            max_size: 1024 * 1024 * 1024, // 1GB
            growth_factor: 2.0,
        }
    }
}

impl Default for GarbageCollectionConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(60),
            threshold: 0.8,
        }
    }
}

impl Default for MemoryMonitoringConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            usage_threshold: 0.8,
        }
    }
}

impl MemoryPerformanceConfig {
    /// Validate memory performance configuration.
    ///
    /// Ensures pool sizes, thresholds, and intervals are properly configured.
    ///
    /// # Errors
    ///
    /// Returns an error if max pool size is less than initial size.
    pub fn validate(&self) -> Result<()> {
        if self.pool.max_size < self.pool.initial_size {
            return Err(NestGateError::configuration_error(
                "memory.pool.max_size",
                "Maximum pool size cannot be less than initial size",
            ));
        }
        Ok(())
    }
}
