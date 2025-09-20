//! Performance configuration structures

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Performance and optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Zero-copy optimization settings
    pub zero_copy: ZeroCopyConfig,
    /// Memory management settings
    pub memory: MemoryConfig,
    /// Threading configuration
    pub threading: ThreadingConfig,
    /// Monitoring configuration
    pub monitoring: PerformanceMonitoringConfig,
    /// COMPATIBILITY: Add missing fields for legacy code
    pub cache_enabled: bool,
    /// COMPATIBILITY: Add missing fields for legacy code
    pub cache_size_mb: u32,
    /// COMPATIBILITY: Add missing fields for legacy code
    pub connection_pooling: bool,
    /// COMPATIBILITY: Add missing fields for legacy code
    pub max_pool_size: u32,
    /// COMPATIBILITY: Add missing fields for legacy code
    pub min_pool_size: u32,
    /// COMPATIBILITY: Add missing fields for legacy code
    pub testing: TestingConfig,
}

/// Zero-copy optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCopyConfig {
    /// Enable zero-copy operations
    pub enabled: bool,
    /// Buffer size for zero-copy operations
    pub buffer_size_bytes: u64,
    /// Memory mapping threshold
    pub mmap_threshold_bytes: u64,
}

/// Memory management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Heap memory limit
    pub heap_limit_bytes: u64,
    /// Enable memory pooling
    pub pooling_enabled: bool,
    /// Memory pool size
    pub pool_size_bytes: u64,
    /// Garbage collection threshold
    pub gc_threshold_bytes: u64,
}

/// Threading configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadingConfig {
    /// Worker thread count
    pub worker_threads: u32,
    /// Enable thread affinity
    pub thread_affinity_enabled: bool,
    /// Thread stack size
    pub stack_size_bytes: u64,
}

/// Performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoringConfig {
    /// Enable metrics collection
    pub metrics_enabled: bool,
    /// Metrics collection interval
    pub metrics_interval: Duration,
    /// Enable performance profiling
    pub profiling_enabled: bool,
    /// Log performance metrics
    pub log_performance: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestingConfig {
    pub test_iterations: u32,
    pub baseline_timeout_seconds: u64,
    pub percentile_target: f64,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            zero_copy: ZeroCopyConfig::default(),
            memory: MemoryConfig::default(),
            threading: ThreadingConfig::default(),
            monitoring: PerformanceMonitoringConfig::default(),
            cache_enabled: true,
            cache_size_mb: 256,
            connection_pooling: true,
            max_pool_size: 100,
            min_pool_size: 10,
            testing: TestingConfig::default(),
        }
    }
}

impl Default for ZeroCopyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            buffer_size_bytes: 64 * 1024,      // 64KB
            mmap_threshold_bytes: 1024 * 1024, // 1MB
        }
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            heap_limit_bytes: 512 * 1024 * 1024, // 512 MB
            pooling_enabled: true,
            pool_size_bytes: 64 * 1024 * 1024,     // 64 MB
            gc_threshold_bytes: 256 * 1024 * 1024, // 256 MB
        }
    }
}

impl Default for ThreadingConfig {
    fn default() -> Self {
        Self {
            worker_threads: num_cpus::get().min(u32::MAX as usize) as u32,
            thread_affinity_enabled: false,
            stack_size_bytes: 2 * 1024 * 1024, // 2 MB
        }
    }
}

impl Default for PerformanceMonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            metrics_interval: Duration::from_secs(60),
            profiling_enabled: false,
            log_performance: true,
        }
    }
}

impl Default for TestingConfig {
    fn default() -> Self {
        Self {
            test_iterations: 1000,
            baseline_timeout_seconds: 30,
            percentile_target: 95.0,
        }
    }
}
