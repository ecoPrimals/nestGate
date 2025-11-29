/// Performance optimization settings and monitoring configuration
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use nestgate_core::constants::canonical::network::SEND_BUFFER_SIZE;
// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewarePerformanceSettings {
    /// Thread pool configuration
    pub thread_pool: ThreadPoolSettings,
    /// Memory management
    pub memory: MemorySettings,
    /// Caching configuration
    pub caching: CachingSettings,
    /// Connection pooling
    pub connection_pooling: ConnectionPoolingSettings,
    /// Request optimization
    pub request_optimization: RequestOptimizationSettings,
    /// Response optimization
    pub response_optimization: ResponseOptimizationSettings,
    /// Worker threads
    pub worker_threads: usize,
    /// Queue capacity
    pub queue_capacity: usize,
    /// Memory optimization
    pub memory_optimization: MemoryOptimizationSettings,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadPoolSettings {
    /// Minimum threads
    pub min_threads: usize,
    /// Maximum threads
    pub max_threads: usize,
    /// Thread idle timeout
    pub idle_timeout: Duration,
    /// Thread stack size
    pub stack_size: Option<usize>,
    /// Work stealing enabled
    pub work_stealing: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySettings {
    /// Maximum memory usage (bytes)
    pub max_memory: Option<usize>,
    /// Memory pool size
    pub pool_size: usize,
    /// Garbage collection interval
    pub gc_interval: Duration,
    /// Memory pressure threshold
    pub pressure_threshold: f64,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingSettings {
    /// Enable caching
    pub enabled: bool,
    /// Cache backend
    pub backend: CacheBackend,
    /// Default TTL
    pub default_ttl: Duration,
    /// Max cache size
    pub max_size: usize,
    /// Cache eviction policy
    pub eviction_policy: EvictionPolicy,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheBackend {
    Memory,
    Redis,
    Database,
    Custom(String),
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvictionPolicy {
    Lru,
    Lfu,
    Fifo,
    Random,
    Custom(String),
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolingSettings {
    /// Enable connection pooling
    pub enabled: bool,
    /// Minimum connections
    pub min_connections: usize,
    /// Maximum connections
    pub max_connections: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Idle timeout
    pub idle_timeout: Duration,
    /// Pool validation
    pub validate_connections: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestOptimizationSettings {
    /// Enable request compression
    pub compression: CompressionSettings,
    /// Request timeout
    pub timeout: Duration,
    /// Request size limits
    pub size_limits: SizeLimits,
    /// Request batching
    pub batching: BatchingSettings,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionSettings {
    /// Enable compression
    pub enabled: bool,
    /// Compression algorithm
    pub algorithm: CompressionAlgorithm,
    /// Compression level
    pub level: u32,
    /// Minimum size for compression
    pub min_size: usize,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    Gzip,
    Deflate,
    Brotli,
    Zstd,
    Custom(String),
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeLimits {
    /// Maximum request body size
    pub max_body_size: usize,
    /// Maximum header size
    pub max_header_size: usize,
    /// Maximum number of headers
    pub max_headers: usize,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchingSettings {
    /// Enable request batching
    pub enabled: bool,
    /// Batch size
    pub batch_size: usize,
    /// Batch timeout
    pub batch_timeout: Duration,
}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponseOptimizationSettings {
    /// Response caching
    pub caching: ResponseCachingSettings,
    /// Response streaming
    pub streaming: StreamingSettings,
    /// Response compression
    pub compression: CompressionSettings,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseCachingSettings {
    /// Enable response caching
    pub enabled: bool,
    /// Cache rules
    pub cache_rules: Vec<CacheRule>,
    /// Default cache TTL
    pub default_ttl: Duration,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheRule {
    /// Path pattern
    pub pattern: String,
    /// Cache TTL
    pub ttl: Duration,
    /// Cache key strategy
    pub key_strategy: CacheKeyStrategy,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheKeyStrategy {
    Url,
    UrlWithQuery,
    UrlWithHeaders,
    Custom(String),
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingSettings {
    /// Enable streaming
    pub enabled: bool,
    /// Buffer size
    pub buffer_size: usize,
    /// Streaming threshold
    pub threshold: usize,
}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryOptimizationSettings {
    /// Enable memory optimization
    pub enabled: bool,
    /// Object pooling
    pub object_pooling: bool,
    /// String interning
    pub string_interning: bool,
    /// Memory mapping
    pub memory_mapping: bool,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MiddlewareObservabilitySettings {
    /// Logging configuration
    pub logging: LoggingSettings,
    /// Metrics collection
    pub metrics: MetricsSettings,
    /// Tracing configuration
    pub tracing: TracingSettings,
    /// Health monitoring
    pub health_monitoring: HealthMonitoringSettings,
    /// Alert configuration
    pub alerting: AlertingSettings,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    /// Log level
    pub level: LogLevel,
    /// Log format
    pub format: LogFormat,
    /// Log destination
    pub destination: LogDestination,
    /// Request logging
    pub request_logging: RequestLoggingSettings,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    Json,
    Text,
    Custom(String),
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogDestination {
    Stdout,
    Stderr,
    File(String),
    Syslog,
    Custom(String),
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLoggingSettings {
    /// Enable request logging
    pub enabled: bool,
    /// Log request headers
    pub log_headers: bool,
    /// Log request body
    pub log_body: bool,
    /// Log response headers
    pub log_response_headers: bool,
    /// Log response body
    pub log_response_body: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSettings {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics backend
    pub backend: MetricsBackend,
    /// Metrics prefix
    pub prefix: String,
    /// Collection interval
    pub interval: Duration,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricsBackend {
    Prometheus,
    StatsD,
    InfluxDB,
    Custom(String),
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingSettings {
    /// Enable distributed tracing
    pub enabled: bool,
    /// Tracing backend
    pub backend: TracingBackend,
    /// Sampling rate
    pub sampling_rate: f64,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TracingBackend {
    Jaeger,
    Zipkin,
    OpenTelemetry,
    Custom(String),
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitoringSettings {
    /// Enable health monitoring
    pub enabled: bool,
    /// Health check interval
    pub check_interval: Duration,
    /// Health endpoints
    pub endpoints: Vec<HealthEndpoint>,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthEndpoint {
    /// Endpoint name
    pub name: String,
    /// Endpoint URL
    pub url: String,
    /// Check timeout
    pub timeout: Duration,
}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertingSettings {
    /// Enable alerting
    pub enabled: bool,
    /// Alert rules
    pub rules: Vec<AlertRule>,
    /// Alert destinations
    pub destinations: Vec<AlertDestination>,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Rule name
    pub name: String,
    /// Rule condition
    pub condition: String,
    /// Alert severity
    pub severity: AlertSeverity,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertDestination {
    Email(String),
    Slack(String),
    Webhook(String),
    Custom(HashMap<String, String>),
}

// ==================== SECTION ====================

impl Default for MiddlewarePerformanceSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            thread_pool: ThreadPoolSettings::default(),
            memory: MemorySettings::default(),
            caching: CachingSettings::default(),
            connection_pooling: ConnectionPoolingSettings::default(),
            request_optimization: RequestOptimizationSettings::default(),
            response_optimization: ResponseOptimizationSettings::default(),
            worker_threads: num_cpus::get(),
            queue_capacity: 1000,
            memory_optimization: MemoryOptimizationSettings::default(),
         }
}

impl MiddlewarePerformanceSettings {
    /// Development performance settings
    pub fn development() -> Self { Self {
            worker_threads: 2,
            queue_capacity: 100,
            caching: CachingSettings {
                enabled: false, // Disabled for development
                ..Default::default()
            , connection_pooling: ConnectionPoolingSettings {
                enabled: true,
                min_connections: 1,
                max_connections: 10,
                ..Default::default()
             }
            memory_optimization: MemoryOptimizationSettings {
                enabled: false, // Disabled for easier debugging
                ..Default::default()
            }
            ..Default::default()
        }
    }

    /// Production performance settings
    pub fn production() -> Self { Self {
            worker_threads: num_cpus::get() * 2,
            queue_capacity: 10_000,
            caching: CachingSettings {
                enabled: true,
                max_size: 10_000,
                default_ttl: Duration::from_secs(300),
                ..Default::default()
            , connection_pooling: ConnectionPoolingSettings {
                enabled: true,
                min_connections: 10,
                max_connections: 100,
                ..Default::default()
             }
            memory_optimization: MemoryOptimizationSettings {
                enabled: true,
                object_pooling: true,
                string_interning: true,
                ..Default::default()
            }
            ..Default::default()
        }
    }
}

impl Default for ThreadPoolSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            min_threads: 1,
            max_threads: num_cpus::get(),
            idle_timeout: Duration::from_secs(60),
            stack_size: None,
            work_stealing: true,
         }
}

impl Default for MemorySettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            max_memory: None,
            pool_size: 1024,
            gc_interval: Duration::from_secs(30),
            pressure_threshold: 0.8,
         }
}

impl Default for CachingSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            enabled: false,
            backend: CacheBackend::Memory,
            default_ttl: Duration::from_secs(3600),
            max_size: 1000,
            eviction_policy: EvictionPolicy::Lru,
         }
}

impl Default for ConnectionPoolingSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            enabled: true,
            min_connections: 5,
            max_connections: 50,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
            validate_connections: true,
         }
}

impl Default for RequestOptimizationSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            compression: CompressionSettings::default(),
            timeout: Duration::from_secs(30),
            size_limits: SizeLimits::default(),
            batching: BatchingSettings::default(),
         }
}

impl Default for CompressionSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            enabled: false,
            algorithm: CompressionAlgorithm::Gzip,
            level: 6,
            min_size: 1024,
         }
}

impl Default for SizeLimits {
    /// Returns the default instance
    fn default() -> Self { Self {
            max_body_size: 1024 * 1024 * 10, // 10MB
            max_header_size: 8192,           // 8KB
            max_headers: 100,
         }
}

impl Default for BatchingSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            enabled: false,
            batch_size: 10,
            batch_timeout: Duration::from_millis(100),
         }
}

impl Default for ResponseCachingSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            enabled: false,
            cache_rules: Vec::new(),
            default_ttl: Duration::from_secs(300),
         }
}

impl Default for StreamingSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            enabled: false,
            buffer_size: SEND_BUFFER_SIZE,
            threshold: 1024 * 1024, // 1MB
         }
}

impl Default for LoggingSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            level: LogLevel::Info,
            format: LogFormat::Json,
            destination: LogDestination::Stdout,
            request_logging: RequestLoggingSettings::default(),
         }
}

impl Default for RequestLoggingSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            enabled: true,
            log_headers: false,
            log_body: false,
            log_response_headers: false,
            log_response_body: false,
         }
}

impl Default for MetricsSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            enabled: false,
            backend: MetricsBackend::Prometheus,
            prefix: "nestgate_middleware".to_string(),
            interval: Duration::from_secs(60),
         }
}

impl Default for TracingSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            enabled: false,
            backend: TracingBackend::Jaeger,
            sampling_rate: 0.1,
         }
}

impl Default for HealthMonitoringSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            enabled: false,
            check_interval: Duration::from_secs(30),
            endpoints: Vec::new(),
         }
}
