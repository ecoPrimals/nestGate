// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// Removed unused import: NestGateError
//
// Performance benchmarking and profiling for storage systems.

//! Profiling module

// Simulated benchmark outputs use `f64` for throughput and latency aggregates.
#![allow(clippy::cast_precision_loss, clippy::cast_sign_loss)]

use super::types::{DetectedStorage, PerformanceProfile};
use nestgate_types::error::Result;
use nestgate_types::unified_enums::storage_types::UnifiedStorageType;

/// Performance profiler for storage systems (type-based estimates; no simulated I/O delays).
pub struct PerformanceProfiler;

impl Default for PerformanceProfiler {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceProfiler {
    /// Create new performance profiler with default settings
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Profile performance characteristics of a storage system
    ///
    /// Uses the same type-based estimates as [`Self::quick_assessment`] (no simulated I/O delays).
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    ///
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn profile_performance(&self, storage: &DetectedStorage) -> Result<PerformanceProfile> {
        self.quick_assessment(storage)
    }

    /// Test if storage supports parallel I/O operations
    fn test_parallel_io(&self, _storage: &DetectedStorage) -> bool {
        // Most modern storage systems support parallel I/O
        // In a real implementation, this would test concurrent operations
        true
    }

    /// Quick performance assessment (faster than full profiling)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    ///
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn quick_assessment(&self, storage: &DetectedStorage) -> Result<PerformanceProfile> {
        let mut profile = PerformanceProfile::default();

        // Use storage type to estimate performance characteristics
        match storage.storage_type {
            UnifiedStorageType::Local => {
                profile.read_throughput_mbps = 500.0; // Typical SSD
                profile.write_throughput_mbps = 400.0;
                profile.read_latency_us = 100.0;
                profile.write_latency_us = 200.0;
                profile.iops = 10000;
            }
            UnifiedStorageType::Network => {
                profile.read_throughput_mbps = 100.0; // Network limited
                profile.write_throughput_mbps = 80.0;
                profile.read_latency_us = 1000.0;
                profile.write_latency_us = 1500.0;
                profile.iops = 1000;
            }
            UnifiedStorageType::Cloud => {
                profile.read_throughput_mbps = 50.0; // Internet limited
                profile.write_throughput_mbps = 30.0;
                profile.read_latency_us = 5000.0;
                profile.write_latency_us = 8000.0;
                profile.iops = 500;
            }
            _ => {
                // Use defaults
            }
        }

        profile.supports_parallel_io = self.test_parallel_io(storage);
        profile.optimal_block_size = 4096;

        Ok(profile)
    }

    /// Generate performance report
    #[must_use]
    pub fn generate_report(&self, profile: &PerformanceProfile) -> String {
        format!(
            "Performance Profile:\n\
             - Read Throughput: {:.2} MB/s\n\
             - Write Throughput: {:.2} MB/s\n\
             - Read Latency: {:.2} μs\n\
             - Write Latency: {:.2} μs\n\
             - IOPS: {}\n\
             - Parallel I/O: {}\n\
             - Optimal Block Size: {} bytes",
            profile.read_throughput_mbps,
            profile.write_throughput_mbps,
            profile.read_latency_us,
            profile.write_latency_us,
            profile.iops,
            if profile.supports_parallel_io {
                "Yes"
            } else {
                "No"
            },
            profile.optimal_block_size
        )
    }
}
