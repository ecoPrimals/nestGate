/// Performance and resource management configuration - extracted from monolithic config
/// Handles buffer sizes, thread pools, memory management, I/O optimization, and monitoring
use serde::{Deserialize, Serialize};
use std::time::Duration;
/// File system monitor performance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsMonitorPerformanceSettings {
    /// Buffer size settings
    pub buffer_sizes: BufferSizeSettings,
    /// Thread pool settings
    pub thread_pool: ThreadPoolSettings,
    /// Memory management settings
    pub memory_management: MemoryManagementSettings,
    /// I/O optimization settings
    pub io_optimization: IoOptimizationSettings,
    /// Performance monitoring settings
    pub monitoring: PerformanceMonitoringSettings,
    /// Maximum events per batch
    pub max_events_per_batch: u32,
    /// Batch timeout
    pub batch_timeout: Duration,
    /// Worker threads
    pub worker_threads: usize,
    /// Enable event coalescing
    pub enable_event_coalescing: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferSizeSettings {
    /// Event buffer size
    pub event_buffer_size: usize,
    /// Read buffer size
    pub read_buffer_size: usize,
    /// Write buffer size
    pub write_buffer_size: usize,
    /// Network buffer size
    pub network_buffer_size: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadPoolSettings {
    /// Minimum threads
    pub min_threads: usize,
    /// Maximum threads
    pub max_threads: usize,
    /// Thread idle timeout
    pub idle_timeout: Duration,
    /// Queue size
    pub queue_size: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryManagementSettings {
    /// Maximum memory usage (bytes)
    pub max_memory_usage: u64,
    /// Memory cleanup interval
    pub cleanup_interval: Duration,
    /// Enable memory pooling
    pub enable_pooling: bool,
    /// Pool size
    pub pool_size: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoOptimizationSettings {
    /// Enable async I/O
    pub enable_async_io: bool,
    /// I/O batch size
    pub batch_size: usize,
    /// I/O timeout
    pub timeout: Duration,
    /// Enable compression
    pub enable_compression: bool,
    /// Compression level (1-9)
    pub compression_level: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoringSettings {
    /// Enable performance monitoring
    pub enabled: bool,
    /// Metrics collection interval
    pub metrics_interval: Duration,
    /// Performance alerts
    pub alerts: Vec<PerformanceAlert>,
    /// Enable profiling
    pub enable_profiling: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    /// Alert name
    pub name: String,
    /// Metric name
    pub metric: String,
    /// Threshold value
    pub threshold: f64,
    /// Alert enabled
    pub enabled: bool,
}
impl Default for FsMonitorPerformanceSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            buffer_sizes: BufferSizeSettings::default(),
            thread_pool: ThreadPoolSettings::default(),
            memory_management: MemoryManagementSettings::default(),
            io_optimization: IoOptimizationSettings::default(),
            monitoring: PerformanceMonitoringSettings::default(),
            max_events_per_batch: 1000,
            batch_timeout: Duration::from_millis(100),
            worker_threads: num_cpus::get(),
            enable_event_coalescing: true,
        }
    }
}

impl Default for BufferSizeSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            event_buffer_size: 8192,
            read_buffer_size: 65_536,
            write_buffer_size: 65_536,
            network_buffer_size: 32_768,
        }
    }
}

impl Default for ThreadPoolSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            min_threads: 2,
            max_threads: num_cpus::get() * 2,
            idle_timeout: Duration::from_secs(60),
            queue_size: 1000,
        }
    }
}

impl Default for MemoryManagementSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            max_memory_usage: 1024 * 1024 * 1024, // 1GB
            cleanup_interval: Duration::from_secs(300),
            enable_pooling: true,
            pool_size: 100,
        }
    }
}

impl Default for IoOptimizationSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enable_async_io: true,
            batch_size: 100,
            timeout: Duration::from_secs(30),
            enable_compression: false,
            compression_level: 6,
        }
    }
}

impl Default for PerformanceMonitoringSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_interval: Duration::from_secs(60),
            alerts: Vec::new(),
            enable_profiling: false,
        }
    }
}
