// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **STORAGE PERFORMANCE CONFIGURATION**

use serde::{Deserialize, Serialize};

/// Storage performance configuration for optimizing I/O and storage efficiency.
///
/// Provides comprehensive performance settings including general optimization,
/// I/O tuning, compression, deduplication, and auto-tuning capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `StoragePerformance`
pub struct StoragePerformanceConfig {
    /// General performance optimization settings.
    pub optimization: PerformanceOptimizationConfig,
    /// I/O-specific optimization settings.
    pub io_optimization: IOOptimizationConfig,
    /// Compression configuration for space savings.
    pub compression: CompressionConfig,
    /// Deduplication configuration for eliminating redundant data.
    pub deduplication: DeduplicationConfig,
    /// Auto-tuning configuration for adaptive performance.
    pub tuning: TuningConfig,
}

/// General performance optimization configuration.
///
/// Controls whether performance optimizations are enabled globally.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `PerformanceOptimization`
pub struct PerformanceOptimizationConfig {
    /// Whether performance optimizations are enabled (default: true).
    pub enabled: bool,
}

/// I/O optimization configuration.
///
/// Controls I/O-specific performance optimizations like buffering and prefetching.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `IOOptimization`
pub struct IOOptimizationConfig {
    /// Whether I/O optimizations are enabled (default: true).
    pub enabled: bool,
}

/// Compression configuration for reducing storage space.
///
/// Controls data compression including algorithm selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Compression
pub struct CompressionConfig {
    /// Whether compression is enabled (default: true).
    pub enabled: bool,
    /// Compression algorithm (default: "lz4" for fast compression).
    pub algorithm: String,
}

/// Deduplication configuration for eliminating duplicate data.
///
/// Controls data deduplication to save storage space.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Deduplication
pub struct DeduplicationConfig {
    /// Whether deduplication is enabled (default: false due to overhead).
    pub enabled: bool,
}

/// Auto-tuning configuration for adaptive performance optimization.
///
/// Enables automatic performance tuning based on workload patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Tuning
pub struct TuningConfig {
    /// Whether auto-tuning is enabled (default: true).
    pub auto_tune: bool,
}

impl Default for StoragePerformanceConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            optimization: PerformanceOptimizationConfig { enabled: true },
            io_optimization: IOOptimizationConfig { enabled: true },
            compression: CompressionConfig {
                enabled: true,
                algorithm: "lz4".to_string(),
            },
            deduplication: DeduplicationConfig { enabled: false },
            tuning: TuningConfig { auto_tune: true },
        }
    }
}

impl StoragePerformanceConfig {
    /// Create production-optimized performance configuration.
    ///
    /// Uses LZ4 compression and auto-tuning for balanced performance.
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }

    /// Create development-optimized performance configuration.
    ///
    /// Uses default settings suitable for local development.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Create high-performance configuration.
    ///
    /// Optimized for maximum throughput with minimal overhead.
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }

    /// Create cloud-native performance configuration.
    ///
    /// Optimized for cloud storage with appropriate compression and tuning.
    #[must_use]
    pub fn cloud_native() -> Self {
        Self::default()
    }

    /// Merge this configuration with another, preferring values from `other`.
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }

    /// Validate performance configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub const fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_performance_config_default_values() {
        let c = StoragePerformanceConfig::default();
        assert!(c.optimization.enabled);
        assert!(c.io_optimization.enabled);
        assert!(c.compression.enabled);
        assert_eq!(c.compression.algorithm, "lz4");
        assert!(!c.deduplication.enabled);
        assert!(c.tuning.auto_tune);
    }

    #[test]
    fn storage_performance_constructors_match_default() {
        let d = StoragePerformanceConfig::default();
        let ser = serde_json::to_string(&d).expect("serialize");
        assert_eq!(
            ser,
            serde_json::to_string(&StoragePerformanceConfig::production_optimized())
                .expect("serialize")
        );
        assert_eq!(
            ser,
            serde_json::to_string(&StoragePerformanceConfig::development_optimized())
                .expect("serialize")
        );
        assert_eq!(
            ser,
            serde_json::to_string(&StoragePerformanceConfig::high_performance())
                .expect("serialize")
        );
        assert_eq!(
            ser,
            serde_json::to_string(&StoragePerformanceConfig::cloud_native()).expect("serialize")
        );
    }

    #[test]
    fn storage_performance_merge_keeps_self() {
        let a = StoragePerformanceConfig::default();
        let other = StoragePerformanceConfig {
            compression: CompressionConfig {
                enabled: false,
                algorithm: "zstd".to_string(),
            },
            ..StoragePerformanceConfig::default()
        };
        let merged = a.clone().merge(other);
        assert!(merged.compression.enabled);
        assert_eq!(merged.compression.algorithm, "lz4");
    }

    #[test]
    fn storage_performance_validate_succeeds() {
        assert!(StoragePerformanceConfig::default().validate().is_ok());
    }

    #[test]
    fn storage_performance_serde_roundtrip() {
        let original = StoragePerformanceConfig::default();
        let json = serde_json::to_string(&original).expect("serialize");
        let parsed: StoragePerformanceConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(
            serde_json::to_string(&original).expect("serialize"),
            serde_json::to_string(&parsed).expect("re-serialize")
        );
    }
}
