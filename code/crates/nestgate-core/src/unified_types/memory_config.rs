///
/// This module contains all memory-related configuration types including memory pools,
/// allocation strategies, and memory management settings.
/// Split from unified_types/mod.rs for better maintainability and 2000-line compliance.
use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== UNIFIED MEMORY CONFIGURATION ====================

/// Unified Memory Configuration - consolidates all memory management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMemoryConfig {
    /// Enable memory management
    pub enabled: bool,
    /// Memory pool configuration
    pub pool_config: MemoryPoolConfig,
    /// Allocation strategy
    pub allocation_strategy: AllocationStrategy,
    /// Memory limits
    pub limits: MemoryLimits,
    /// Garbage collection settings
    pub gc_config: GarbageCollectionConfig,
    /// Memory monitoring
    pub monitoring: MemoryMonitoringConfig,
}

impl Default for UnifiedMemoryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            pool_config: MemoryPoolConfig::default(),
            allocation_strategy: AllocationStrategy::default(),
            limits: MemoryLimits::default(),
            gc_config: GarbageCollectionConfig::default(),
            monitoring: MemoryMonitoringConfig::default(),
        }
    }
}

// ==================== MEMORY POOL CONFIGURATION ====================

/// Memory pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPoolConfig {
    /// Enable memory pooling
    pub enabled: bool,
    /// Initial pool size in bytes
    pub initial_size: u64,
    /// Maximum pool size in bytes
    pub cache_size_bytes: u64,
    /// Pool growth factor
    pub growth_factor: f64,
    /// Pool shrink threshold
    pub shrink_threshold: f64,
    /// Pool types configuration
    pub pools: Vec<PoolTypeConfig>,
}

impl Default for MemoryPoolConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            initial_size: 64 * 1024 * 1024, // 64MB
            cache_size_bytes: 512 * 1024 * 1024,    // 512MB
            growth_factor: 1.5,
            shrink_threshold: 0.25,
            pools: vec![
                PoolTypeConfig {
                    pool_type: PoolType::Small,
                    block_size: 64,
                    initial_blocks: 1000,
                    max_blocks: 10000,
                },
                PoolTypeConfig {
                    pool_type: PoolType::Medium,
                    block_size: 1024,
                    initial_blocks: 500,
                    max_blocks: 5000,
                },
                PoolTypeConfig {
                    pool_type: PoolType::Large,
                    block_size: 16384,
                    initial_blocks: 100,
                    max_blocks: 1000,
                },
            ],
        }
    }
}

/// Pool type configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolTypeConfig {
    /// Pool type
    pub pool_type: PoolType,
    /// Block size for this pool
    pub block_size: u64,
    /// Initial number of blocks
    pub initial_blocks: u32,
    /// Maximum number of blocks
    pub max_blocks: u32,
}

/// Types of memory pools
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PoolType {
    /// Small allocations (< 1KB)
    Small,
    /// Medium allocations (1KB - 16KB)
    Medium,
    /// Large allocations (> 16KB)
    Large,
    /// Custom pool type
    Custom(String),
}

// ==================== ALLOCATION STRATEGY ====================

/// Memory allocation strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationStrategy {
    /// Primary allocation method
    pub primary_method: AllocationMethod,
    /// Fallback allocation method
    pub fallback_method: AllocationMethod,
    /// Alignment requirements
    pub alignment: u64,
    /// Enable zero-fill for new allocations
    pub zero_fill: bool,
    /// Allocation tracking
    pub tracking: AllocationTracking,
}

impl Default for AllocationStrategy {
    fn default() -> Self {
        Self {
            primary_method: AllocationMethod::Pool,
            fallback_method: AllocationMethod::System,
            alignment: 8,
            zero_fill: false,
            tracking: AllocationTracking::default(),
        }
    }
}

/// Memory allocation methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AllocationMethod {
    /// Pool-based allocation
    Pool,
    /// System allocation (malloc/free)
    System,
    /// Stack allocation where possible
    Stack,
    /// Memory-mapped allocation
    Mmap,
    /// Custom allocation method
    Custom(String),
}

/// Allocation tracking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationTracking {
    /// Enable allocation tracking
    pub enabled: bool,
    /// Track allocation stack traces
    pub stack_traces: bool,
    /// Track allocation statistics
    pub statistics: bool,
    /// Maximum tracked allocations
    pub max_tracked: u32,
}

impl Default for AllocationTracking {
    fn default() -> Self {
        Self {
            enabled: false, // Disabled by default for performance
            stack_traces: false,
            statistics: true,
            max_tracked: 10000,
        }
    }
}

// ==================== MEMORY LIMITS ====================

/// Memory usage limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLimits {
    /// Maximum total memory usage in bytes
    pub max_total_memory: Option<u64>,
    /// Maximum heap memory in bytes
    pub max_heap_memory: Option<u64>,
    /// Maximum stack memory in bytes
    pub max_stack_memory: Option<u64>,
    /// Memory pressure thresholds
    pub pressure_thresholds: MemoryPressureThresholds,
    /// Out-of-memory handling
    pub oom_handling: OomHandling,
}

impl Default for MemoryLimits {
    fn default() -> Self {
        Self {
            max_total_memory: None, // No limit by default
            max_heap_memory: None,
            max_stack_memory: Some(8 * 1024 * 1024), // 8MB stack limit
            pressure_thresholds: MemoryPressureThresholds::default(),
            oom_handling: OomHandling::default(),
        }
    }
}

/// Memory pressure thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPressureThresholds {
    /// Low pressure threshold (percentage)
    pub low_threshold: f64,
    /// Medium pressure threshold (percentage)
    pub medium_threshold: f64,
    /// High pressure threshold (percentage)
    pub high_threshold: f64,
    /// Critical pressure threshold (percentage)
    pub critical_threshold: f64,
}

impl Default for MemoryPressureThresholds {
    fn default() -> Self {
        Self {
            low_threshold: 60.0,
            medium_threshold: 75.0,
            high_threshold: 85.0,
            critical_threshold: 95.0,
        }
    }
}

/// Out-of-memory handling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OomHandling {
    /// OOM handling strategy
    pub strategy: OomStrategy,
    /// Enable OOM killer protection
    pub killer_protection: bool,
    /// Recovery actions
    pub recovery_actions: Vec<RecoveryAction>,
}

impl Default for OomHandling {
    fn default() -> Self {
        Self {
            strategy: OomStrategy::GracefulShutdown,
            killer_protection: false,
            recovery_actions: vec![
                RecoveryAction::ClearCaches,
                RecoveryAction::ForceGarbageCollection,
                RecoveryAction::ReduceBuffers,
            ],
        }
    }
}

/// Out-of-memory handling strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OomStrategy {
    /// Abort the process immediately
    Abort,
    /// Attempt graceful shutdown
    GracefulShutdown,
    /// Try to recover and continue
    Recover,
    /// Custom OOM handling
    Custom(String),
}

/// Recovery actions for memory pressure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecoveryAction {
    /// Clear all caches
    ClearCaches,
    /// Force garbage collection
    ForceGarbageCollection,
    /// Reduce buffer sizes
    ReduceBuffers,
    /// Release unused memory pools
    ReleaseUnusedPools,
    /// Custom recovery action
    Custom(String),
}

// ==================== GARBAGE COLLECTION CONFIGURATION ====================

/// Garbage collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarbageCollectionConfig {
    /// Enable garbage collection
    pub enabled: bool,
    /// GC strategy
    pub strategy: GcStrategy,
    /// GC trigger conditions
    pub triggers: GcTriggers,
    /// GC tuning parameters
    pub tuning: GcTuning,
}

impl Default for GarbageCollectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strategy: GcStrategy::Incremental,
            triggers: GcTriggers::default(),
            tuning: GcTuning::default(),
        }
    }
}

/// Garbage collection strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GcStrategy {
    /// Stop-the-world GC
    StopTheWorld,
    /// Incremental GC
    Incremental,
    /// Concurrent GC
    Concurrent,
    /// Generational GC
    Generational,
    /// Custom GC strategy
    Custom(String),
}

/// Garbage collection trigger conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcTriggers {
    /// Memory usage threshold for triggering GC
    pub memory_threshold: f64,
    /// Time interval for periodic GC
    pub time_interval: Option<Duration>,
    /// Allocation count threshold
    pub allocation_threshold: Option<u64>,
    /// Pressure-based triggering
    pub pressure_based: bool,
}

impl Default for GcTriggers {
    fn default() -> Self {
        Self {
            memory_threshold: 80.0,                        // 80% memory usage
            time_interval: Some(Duration::from_secs(300)), // 5 minutes
            allocation_threshold: None,
            pressure_based: true,
        }
    }
}

/// Garbage collection tuning parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcTuning {
    /// Maximum GC pause time
    pub max_pause_time: Duration,
    /// GC thread count
    pub thread_count: Option<u32>,
    /// Heap sizing parameters
    pub heap_sizing: HeapSizing,
    /// GC logging
    pub logging: GcLogging,
}

impl Default for GcTuning {
    fn default() -> Self {
        Self {
            max_pause_time: Duration::from_millis(100),
            thread_count: None, // Auto-detect
            heap_sizing: HeapSizing::default(),
            logging: GcLogging::default(),
        }
    }
}

/// Heap sizing parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeapSizing {
    /// Initial heap size
    pub initial_size: Option<u64>,
    /// Maximum heap size
    pub cache_size_bytes: Option<u64>,
    /// Heap growth factor
    pub growth_factor: f64,
    /// Heap shrink threshold
    pub shrink_threshold: f64,
}

impl Default for HeapSizing {
    fn default() -> Self {
        Self {
            initial_size: None,
            cache_size_bytes: None,
            growth_factor: 1.5,
            shrink_threshold: 0.3,
        }
    }
}

/// Garbage collection logging configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GcLogging {
    /// Enable GC logging
    pub enabled: bool,
    /// Log GC events
    pub log_events: bool,
    /// Log GC statistics
    pub log_statistics: bool,
    /// Log file path
    pub log_file: Option<String>,
}

// ==================== MEMORY MONITORING CONFIGURATION ====================

/// Memory monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMonitoringConfig {
    /// Enable memory monitoring
    pub enabled: bool,
    /// Monitoring interval
    pub interval: Duration,
    /// Metrics to collect
    pub metrics: Vec<MemoryMetric>,
    /// Memory profiling
    pub profiling: MemoryProfiling,
    /// Alert thresholds
    pub alerts: MemoryAlerts,
}

impl Default for MemoryMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(60),
            metrics: vec![
                MemoryMetric::TotalUsage,
                MemoryMetric::HeapUsage,
                MemoryMetric::StackUsage,
                MemoryMetric::PoolUsage,
                MemoryMetric::FragmentationRatio,
            ],
            profiling: MemoryProfiling::default(),
            alerts: MemoryAlerts::default(),
        }
    }
}

/// Memory metrics to collect
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryMetric {
    /// Total memory usage
    TotalUsage,
    /// Heap memory usage
    HeapUsage,
    /// Stack memory usage
    StackUsage,
    /// Memory pool usage
    PoolUsage,
    /// Memory fragmentation ratio
    FragmentationRatio,
    /// Allocation rate
    AllocationRate,
    /// Deallocation rate
    DeallocationRate,
    /// GC frequency
    GcFrequency,
    /// Custom memory metric
    Custom(String),
}

/// Memory profiling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryProfiling {
    /// Enable memory profiling
    pub enabled: bool,
    /// Profiling mode
    pub mode: ProfilingMode,
    /// Sample rate for profiling
    pub sample_rate: f64,
    /// Profile output format
    pub output_format: ProfileFormat,
    /// Profile output path
    pub output_path: String,
}

impl Default for MemoryProfiling {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: ProfilingMode::Sampling,
            sample_rate: 0.01, // 1% sampling
            output_format: ProfileFormat::Pprof,
            output_path: "/tmp/memory_profile".to_string(),
        }
    }
}

/// Memory profiling modes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProfilingMode {
    /// Sampling profiler
    Sampling,
    /// Instrumentation profiler
    Instrumentation,
    /// Hybrid profiling
    Hybrid,
}

/// Profile output formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProfileFormat {
    /// pprof format
    Pprof,
    /// JSON format
    Json,
    /// Custom format
    Custom(String),
}

/// Memory alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAlerts {
    /// Enable memory alerts
    pub enabled: bool,
    /// Alert rules
    pub rules: Vec<MemoryAlertRule>,
    /// Alert cooldown period
    pub cooldown: Duration,
}

impl Default for MemoryAlerts {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: vec![
                MemoryAlertRule {
                    name: "High Memory Usage".to_string(),
                    condition: AlertCondition::MemoryUsagePercent(85.0),
                    severity: AlertSeverity::Warning,
                    duration: Duration::from_secs(300),
                },
                MemoryAlertRule {
                    name: "Critical Memory Usage".to_string(),
                    condition: AlertCondition::MemoryUsagePercent(95.0),
                    severity: AlertSeverity::Critical,
                    duration: Duration::from_secs(60),
                },
            ],
            cooldown: Duration::from_secs(300),
        }
    }
}

/// Memory alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAlertRule {
    /// Alert rule name
    pub name: String,
    /// Alert condition
    pub condition: AlertCondition,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Duration condition must be true
    pub duration: Duration,
}

/// Memory alert conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCondition {
    /// Memory usage percentage threshold
    MemoryUsagePercent(f64),
    /// Absolute memory usage threshold in bytes
    MemoryUsageBytes(u64),
    /// Memory fragmentation threshold
    FragmentationPercent(f64),
    /// Allocation rate threshold
    AllocationRate(u64),
    /// Custom alert condition
    Custom(String),
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    /// Informational alert
    Info,
    /// Warning alert
    Warning,
    /// Critical alert
    Critical,
}
