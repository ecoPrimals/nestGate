///
/// This module contains all cache-related configuration types including cache strategies,
/// eviction policies, and replication settings.
/// Split from unified_types/mod.rs for better maintainability and 2000-line compliance.
use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== SECTION ====================

/// Unified Cache Configuration - consolidates all caching settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCacheConfig {
    /// Enable caching
    pub enabled: bool,
    /// Cache type
    pub cache_type: CacheType,
    /// Maximum cache size in bytes
    pub cache_size_bytes: u64,
    /// Time-to-live for cache entries
    pub ttl: Duration,
    /// Eviction policy
    pub eviction_strategy: EvictionPolicy,
    /// Enable compression
    pub compression: bool,
    /// Enable persistence
    pub persistence: bool,
    /// Replication configuration
    pub replication: CacheReplicationConfig,
    /// Cache monitoring
    pub monitoring: CacheMonitoringConfig,
    /// Cache partitioning
    pub partitioning: CachePartitioningConfig,
}

impl Default for UnifiedCacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_type: CacheType::Read,
            cache_size_bytes: 1024 * 1024 * 100,    // 100MB
            ttl: Duration::from_secs(3600), // 1 hour
            eviction_strategy: EvictionPolicy::Lru,
            compression: false,
            persistence: false,
            replication: CacheReplicationConfig::default(),
            monitoring: CacheMonitoringConfig::default(),
            partitioning: CachePartitioningConfig::default(),
        }
    }
}

// ==================== SECTION ====================

/// Cache types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CacheType {
    /// Read-only cache
    Read,
    /// Write-through cache
    WriteThrough,
    /// Write-back cache
    WriteBack,
    /// Write-around cache
    WriteAround,
    /// Read-through cache
    ReadThrough,
    /// Custom cache type
    Custom(String),
}

// ==================== SECTION ====================

/// Cache eviction policies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvictionPolicy {
    /// Least Recently Used
    Lru,
    /// Least Frequently Used
    Lfu,
    /// First In, First Out
    Fifo,
    /// Last In, First Out
    Lifo,
    /// Random eviction
    Random,
    /// Time To Live based
    Ttl,
    /// Custom eviction policy
    Custom(String),
}

// ==================== SECTION ====================

/// Cache replication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheReplicationConfig {
    /// Enable replication
    pub enabled: bool,
    /// Number of replicas
    pub replicas: u32,
    /// Consistency level
    pub consistency: CacheConsistency,
    /// Synchronization interval
    pub sync_interval: Duration,
    /// Replication strategy
    pub strategy: ReplicationStrategy,
    /// Conflict resolution
    pub conflict_resolution: ConflictResolution,
}

impl Default for CacheReplicationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            replicas: 1,
            consistency: CacheConsistency::Eventual,
            sync_interval: Duration::from_secs(60),
            strategy: ReplicationStrategy::Async,
            conflict_resolution: ConflictResolution::LastWriteWins,
        }
    }
}

/// Cache consistency levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CacheConsistency {
    /// Strong consistency
    Strong,
    /// Eventual consistency
    Eventual,
    /// Session consistency
    Session,
    /// Weak consistency
    Weak,
}

/// Replication strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReplicationStrategy {
    /// Synchronous replication
    Sync,
    /// Asynchronous replication
    Async,
    /// Semi-synchronous replication
    SemiSync,
    /// Custom replication strategy
    Custom(String),
}

/// Conflict resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConflictResolution {
    /// Last write wins
    LastWriteWins,
    /// First write wins
    FirstWriteWins,
    /// Merge conflicts
    Merge,
    /// Manual resolution required
    Manual,
    /// Custom conflict resolution
    Custom(String),
}

// ==================== SECTION ====================

/// Cache monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMonitoringConfig {
    /// Enable cache monitoring
    pub enabled: bool,
    /// Monitoring interval
    pub interval: Duration,
    /// Metrics to collect
    pub metrics: Vec<CacheMetric>,
    /// Performance thresholds
    pub thresholds: CacheThresholds,
    /// Alert configuration
    pub alerts: CacheAlertConfig,
}

impl Default for CacheMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            metrics: vec![
                CacheMetric::HitRate,
                CacheMetric::MissRate,
                CacheMetric::EvictionRate,
                CacheMetric::MemoryUsage,
                CacheMetric::ResponseTime,
            ],
            thresholds: CacheThresholds::default(),
            alerts: CacheAlertConfig::default(),
        }
    }
}

/// Cache metrics to collect
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CacheMetric {
    /// Cache hit rate
    HitRate,
    /// Cache miss rate
    MissRate,
    /// Eviction rate
    EvictionRate,
    /// Memory usage
    MemoryUsage,
    /// Response time
    ResponseTime,
    /// Throughput
    Throughput,
    /// Entry count
    EntryCount,
    /// Custom cache metric
    Custom(String),
}

/// Cache performance thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheThresholds {
    /// Minimum acceptable hit rate
    pub min_hit_rate: f64,
    /// Maximum acceptable response time
    pub max_response_time: Duration,
    /// Maximum memory usage percentage
    pub max_memory_usage: f64,
    /// Maximum eviction rate
    pub max_eviction_rate: f64,
}

impl Default for CacheThresholds {
    fn default() -> Self {
        Self {
            min_hit_rate: 0.8, // 80% hit rate
            max_response_time: Duration::from_millis(10),
            max_memory_usage: 0.9,  // 90% memory usage
            max_eviction_rate: 0.1, // 10% eviction rate
        }
    }
}

/// Cache alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheAlertConfig {
    /// Enable cache alerts
    pub enabled: bool,
    /// Alert rules
    pub rules: Vec<CacheAlertRule>,
    /// Alert cooldown period
    pub cooldown: Duration,
}

impl Default for CacheAlertConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: vec![
                CacheAlertRule {
                    name: "Low Hit Rate".to_string(),
                    condition: CacheAlertCondition::HitRateBelow(0.7),
                    severity: CacheAlertSeverity::Warning,
                    duration: Duration::from_secs(300),
                },
                CacheAlertRule {
                    name: "High Memory Usage".to_string(),
                    condition: CacheAlertCondition::MemoryUsageAbove(0.95),
                    severity: CacheAlertSeverity::Critical,
                    duration: Duration::from_secs(60),
                },
            ],
            cooldown: Duration::from_secs(300),
        }
    }
}

/// Cache alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheAlertRule {
    /// Alert rule name
    pub name: String,
    /// Alert condition
    pub condition: CacheAlertCondition,
    /// Alert severity
    pub severity: CacheAlertSeverity,
    /// Duration condition must be true
    pub duration: Duration,
}

/// Cache alert conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheAlertCondition {
    /// Hit rate below threshold
    HitRateBelow(f64),
    /// Miss rate above threshold
    MissRateAbove(f64),
    /// Memory usage above threshold
    MemoryUsageAbove(f64),
    /// Response time above threshold
    ResponseTimeAbove(Duration),
    /// Custom alert condition
    Custom(String),
}

/// Cache alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum CacheAlertSeverity {
    /// Informational alert
    Info,
    /// Warning alert
    Warning,
    /// Critical alert
    Critical,
}

// ==================== SECTION ====================

/// Cache partitioning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePartitioningConfig {
    /// Enable cache partitioning
    pub enabled: bool,
    /// Partitioning strategy
    pub strategy: PartitioningStrategy,
    /// Number of partitions
    pub partition_count: u32,
    /// Partition key function
    pub key_function: PartitionKeyFunction,
    /// Load balancing across partitions
    pub load_balancing: PartitionLoadBalancing,
}

impl Default for CachePartitioningConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: PartitioningStrategy::Hash,
            partition_count: 4,
            key_function: PartitionKeyFunction::Hash,
            load_balancing: PartitionLoadBalancing::RoundRobin,
        }
    }
}

/// Cache partitioning strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PartitioningStrategy {
    /// Hash-based partitioning
    Hash,
    /// Range-based partitioning
    Range,
    /// Consistent hashing
    ConsistentHash,
    /// Custom partitioning strategy
    Custom(String),
}

/// Partition key functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PartitionKeyFunction {
    /// Simple hash function
    Hash,
    /// CRC32 hash function
    Crc32,
    /// MD5 hash function
    Md5,
    /// Custom key function
    Custom(String),
}

/// Partition load balancing strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PartitionLoadBalancing {
    /// Round-robin balancing
    RoundRobin,
    /// Least-loaded balancing
    LeastLoaded,
    /// Weighted balancing
    Weighted,
    /// Custom load balancing
    Custom(String),
}

// ==================== SECTION ====================

/// Cache storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStorageConfig {
    /// Storage backend
    pub backend: CacheStorageBackend,
    /// Storage path (for file-based backends)
    pub storage_path: Option<String>,
    /// Storage compression
    pub compression: StorageCompressionConfig,
    /// Storage encryption
    pub encryption: StorageEncryptionConfig,
}

impl Default for CacheStorageConfig {
    fn default() -> Self {
        Self {
            backend: CacheStorageBackend::Memory,
            storage_path: None,
            compression: StorageCompressionConfig::default(),
            encryption: StorageEncryptionConfig::default(),
        }
    }
}

/// Cache storage backends
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CacheStorageBackend {
    /// In-memory storage
    Memory,
    /// File-based storage
    File,
    /// Database storage
    Database,
    /// Redis storage
    Redis,
    /// Memcached storage
    Memcached,
    /// Custom storage backend
    Custom(String),
}

/// Storage compression configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCompressionConfig {
    /// Enable compression
    pub enabled: bool,
    /// Compression algorithm
    pub algorithm: CompressionAlgorithm,
    /// Compression level
    pub level: u32,
    /// Minimum size threshold for compression
    pub min_size_threshold: u64,
}

impl Default for StorageCompressionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: CompressionAlgorithm::Gzip,
            level: 6,
            min_size_threshold: 1024, // 1KB
        }
    }
}

/// Compression algorithms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    /// Gzip compression
    Gzip,
    /// Zlib compression
    Zlib,
    /// LZ4 compression
    Lz4,
    /// Zstd compression
    Zstd,
    /// Custom compression algorithm
    Custom(String),
}

/// Storage encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEncryptionConfig {
    /// Enable encryption
    pub enabled: bool,
    /// Encryption algorithm
    pub algorithm: EncryptionAlgorithm,
    /// Key derivation function
    pub key_derivation: KeyDerivationFunction,
    /// Key rotation interval
    pub key_rotation_interval: Duration,
}

impl Default for StorageEncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: EncryptionAlgorithm::Aes256,
            key_derivation: KeyDerivationFunction::Pbkdf2,
            key_rotation_interval: Duration::from_secs(86400 * 30), // 30 days
        }
    }
}

/// Encryption algorithms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    /// AES-256 encryption
    Aes256,
    /// AES-128 encryption
    Aes128,
    /// ChaCha20 encryption
    ChaCha20,
    /// Custom encryption algorithm
    Custom(String),
}

/// Key derivation functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyDerivationFunction {
    /// PBKDF2
    Pbkdf2,
    /// Scrypt
    Scrypt,
    /// Argon2
    Argon2,
    /// Custom key derivation function
    Custom(String),
}

// ==================== SECTION ====================

/// Cache warming configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheWarmingConfig {
    /// Enable cache warming
    pub enabled: bool,
    /// Warming strategy
    pub strategy: WarmingStrategy,
    /// Warming schedule
    pub schedule: WarmingSchedule,
    /// Warming data sources
    pub data_sources: Vec<WarmingDataSource>,
    /// Warming concurrency
    pub concurrency: u32,
}

impl Default for CacheWarmingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: WarmingStrategy::Preload,
            schedule: WarmingSchedule::OnStartup,
            data_sources: vec![],
            concurrency: 4,
        }
    }
}

/// Cache warming strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WarmingStrategy {
    /// Preload data into cache
    Preload,
    /// Lazy loading on first access
    LazyLoad,
    /// Background refresh
    BackgroundRefresh,
    /// Custom warming strategy
    Custom(String),
}

/// Cache warming schedules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WarmingSchedule {
    /// Warm cache on startup
    OnStartup,
    /// Periodic warming
    Periodic(Duration),
    /// Event-triggered warming
    EventTriggered,
    /// Custom warming schedule
    Custom(String),
}

/// Cache warming data source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarmingDataSource {
    /// Data source name
    pub name: String,
    /// Data source type
    pub source_type: DataSourceType,
    /// Connection string or configuration
    pub connection: String,
    /// Query or data selection criteria
    pub query: String,
    /// Priority for this data source
    pub priority: u32,
}

/// Data source types for cache warming
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataSourceType {
    /// Database source
    Database,
    /// File source
    File,
    /// API source
    Api,
    /// Message queue source
    MessageQueue,
    /// Custom data source
    Custom(String),
}
