//
// Performance-related configuration including buffers, thread pools,
// memory management, and I/O optimization.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Performance configuration (consolidates 15+ performance configs)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct PerformanceConfig {
    /// Buffer sizes
    pub buffers: BufferConfig,
    /// Thread pool configuration
    pub thread_pools: ThreadPoolConfig,
    /// Memory management
    pub memory: MemoryConfig,
    /// I/O configuration
    pub io: IoConfig,
    /// Cache configuration
    pub cache: CacheConfig,
    /// Metrics collection
    pub metrics: MetricsConfig,
}

/// Buffer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferConfig {
    /// Read buffer size
    pub read_buffer_size: usize,
    /// Write buffer size
    pub write_buffer_size: usize,
    /// Network buffer size
    pub network_buffer_size: usize,
}

/// Thread pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadPoolConfig {
    /// Core thread count
    pub core_threads: usize,
    /// Maximum thread count
    pub max_threads: usize,
    /// Thread keep-alive time
    pub keep_alive_time: Duration,
}

/// Memory configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Memory limit (bytes)
    pub limit_bytes: Option<u64>,
    /// Enable memory compression
    pub enable_compression: bool,
    /// Garbage collection frequency
    pub gc_frequency: Duration,
}

/// I/O configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoConfig {
    /// I/O queue depth
    pub queue_depth: usize,
    /// Read-ahead size
    pub read_ahead_size: usize,
    /// Enable direct I/O
    pub direct_io: bool,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Cache size (bytes)
    pub size_bytes: u64,
    /// Cache eviction policy
    pub eviction_policy: String,
    /// Enable cache warming
    pub enable_warming: bool,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Collection interval
    pub collection_interval: Duration,
    /// Metrics retention
    pub retention_days: u32,
}


impl Default for BufferConfig {
    fn default() -> Self {
        Self {
            read_buffer_size: 64 * 1024,  // 64KB
            write_buffer_size: 64 * 1024, // 64KB
            network_buffer_size: 32 * 1024, // 32KB
        }
    }
}

impl Default for ThreadPoolConfig {
    fn default() -> Self {
        // Use available CPU cores, fallback to 4 if detection fails
        let cpu_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        
        Self {
            core_threads: cpu_count,
            max_threads: cpu_count * 2,
            keep_alive_time: Duration::from_secs(60),
        }
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            limit_bytes: None,
            enable_compression: false,
            gc_frequency: Duration::from_secs(300),
        }
    }
}

impl Default for IoConfig {
    fn default() -> Self {
        Self {
            queue_depth: 32,
            read_ahead_size: 128 * 1024, // 128KB
            direct_io: false,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            size_bytes: 128 * 1024 * 1024, // 128MB
            eviction_policy: "lru".to_string(),
            enable_warming: true,
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(10),
            retention_days: 30,
        }
    }
} 