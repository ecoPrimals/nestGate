// **STORAGE BACKENDS CONFIGURATION**
//! Backends functionality and utilities.
// Comprehensive storage backend configurations supporting multiple storage types
//! including filesystem, ZFS, cloud storage (S3, Azure, GCS), memory, and distributed storage.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

// ==================== STORAGE BACKEND CONFIGURATION ====================

/// Storage backend configuration supporting multiple storage types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageBackendConfig {
    /// Default storage backend to use
    pub default_backend: StorageBackendType,

    /// Available storage backends
    pub backends: HashMap<String, StorageBackend>,

    /// Backend routing rules
    pub routing: StorageRoutingConfig,

    /// Failover configuration
    pub failover: StorageFailoverConfig,

    /// Load balancing across backends
    pub load_balancing: StorageLoadBalancingConfig,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StorageBackendType {
    Filesystem,
    Zfs,
    S3Compatible,
    Azure,
    Gcs,
    Memory,
    Distributed,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageBackend {
    /// Backend type
    pub backend_type: StorageBackendType,

    /// Backend-specific configuration
    pub config: StorageBackendSpecificConfig,

    /// Connection settings
    pub connection: StorageConnectionConfig,

    /// Capacity and limits
    pub limits: StorageLimitsConfig,

    /// Health check configuration
    pub health_check: StorageHealthCheckConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageBackendSpecificConfig {
    Filesystem {
        root_path: PathBuf,
        permissions: u32,
        create_dirs: bool,
    },
    Zfs {
        pool_name: String,
        dataset_prefix: String,
        compression: ZfsCompression,
        deduplication: bool,
    },
    S3Compatible {
        endpoint: String,
        region: String,
        bucket: String,
        access_key_id: String,
        secret_access_key: String,
        use_ssl: bool,
    },
    Azure {
        account_name: String,
        account_key: String,
        container: String,
        endpoint_suffix: Option<String>,
    },
    Gcs {
        project_id: String,
        bucket: String,
        credentials_path: Option<PathBuf>,
        service_account_key: Option<String>,
    },
    Memory {
        max_size: usize,
        eviction_policy: MemoryEvictionPolicy,
    },
    Distributed {
        nodes: Vec<DistributedStorageNode>,
        consistency_level: ConsistencyLevel,
        replication_factor: u32,
    },
    Custom {
        config: serde_json::Value,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsCompression {
    Off,
    Lzjb,
    Gzip,
    Zle,
    Lz4,
    Zstd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryEvictionPolicy {
    Lru,
    Lfu,
    Fifo,
    Random,
    Ttl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedStorageNode {
    /// Node identifier
    pub id: String,

    /// Node endpoint
    pub endpoint: String,

    /// Node weight for load balancing
    pub weight: u32,

    /// Node availability zone
    pub availability_zone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsistencyLevel {
    Eventual,
    Strong,
    Session,
    BoundedStaleness,
}

// ==================== CONNECTION CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConnectionConfig {
    /// Connection timeout
    pub timeout: Duration,

    /// Maximum concurrent connections
    pub max_connections: u32,

    /// Connection retry settings
    pub retry: ConnectionRetryConfig,

    /// Connection pooling settings
    pub pooling: ConnectionPoolConfig,

    /// TLS/SSL settings
    pub tls: Option<ConnectionTlsConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionRetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,

    /// Base delay between retries
    pub base_delay: Duration,

    /// Maximum delay between retries
    pub max_delay: Duration,

    /// Retry strategy
    pub strategy: RetryStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetryStrategy {
    Fixed,
    Linear,
    Exponential,
    Jitter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolConfig {
    /// Minimum pool size
    pub min_size: u32,

    /// Maximum pool size
    pub max_size: u32,

    /// Connection idle timeout
    pub idle_timeout: Duration,

    /// Connection lifetime
    pub max_lifetime: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionTlsConfig {
    /// Enable TLS
    pub enabled: bool,

    /// Verify certificates
    pub verify_certificates: bool,

    /// CA certificate path
    pub ca_cert_path: Option<PathBuf>,

    /// Client certificate path
    pub client_cert_path: Option<PathBuf>,

    /// Client key path
    pub client_key_path: Option<PathBuf>,
}

// ==================== LIMITS AND HEALTH CHECK CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLimitsConfig {
    /// Maximum storage capacity
    pub max_capacity: Option<u64>,

    /// Maximum file size
    pub max_file_size: Option<u64>,

    /// Maximum number of files
    pub max_files: Option<u64>,

    /// Rate limiting
    pub rate_limits: RateLimitsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitsConfig {
    /// Reads per second
    pub reads_per_second: Option<u32>,

    /// Writes per second
    pub writes_per_second: Option<u32>,

    /// Bandwidth limit (bytes per second)
    pub bandwidth_limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageHealthCheckConfig {
    /// Enable health checks
    pub enabled: bool,

    /// Health check interval
    pub interval: Duration,

    /// Health check timeout
    pub timeout: Duration,

    /// Health check endpoint
    pub endpoint: Option<String>,

    /// Failure threshold
    pub failure_threshold: u32,

    /// Recovery threshold
    pub recovery_threshold: u32,
}

// ==================== ROUTING AND LOAD BALANCING ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageRoutingConfig {
    /// Routing rules
    pub rules: Vec<RoutingRule>,

    /// Default backend for unmatched requests
    pub default_backend: String,

    /// Enable content-based routing
    pub content_based_routing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    /// Rule name
    pub name: String,

    /// Rule condition
    pub condition: RoutingCondition,

    /// Target backend
    pub backend: String,

    /// Rule priority
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingCondition {
    PathPrefix(String),
    FileExtension(String),
    FileSize(FileSizeCondition),
    ContentType(String),
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSizeCondition {
    pub operator: ComparisonOperator,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    Equal,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageFailoverConfig {
    /// Enable automatic failover
    pub enabled: bool,

    /// Failover strategy
    pub strategy: FailoverStrategy,

    /// Failover timeout
    pub timeout: Duration,

    /// Health check configuration for failover
    pub health_check: FailoverHealthCheckConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FailoverStrategy {
    RoundRobin,
    Priority,
    Weighted,
    Geolocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverHealthCheckConfig {
    /// Health check interval
    pub interval: Duration,

    /// Consecutive failures before failover
    pub failure_threshold: u32,

    /// Consecutive successes before recovery
    pub recovery_threshold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLoadBalancingConfig {
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,

    /// Sticky sessions
    pub sticky_sessions: bool,

    /// Session affinity timeout
    pub session_timeout: Duration,

    /// Backend weights
    pub weights: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    LeastResponseTime,
    Random,
    Consistent,
}

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for StorageBackendConfig {
    fn default() -> Self {
        Self {
            default_backend: StorageBackendType::Filesystem,
            backends: HashMap::new(),
            routing: StorageRoutingConfig::default(),
            failover: StorageFailoverConfig::default(),
            load_balancing: StorageLoadBalancingConfig::default(),
        }
    }
}

impl Default for StorageRoutingConfig {
    fn default() -> Self {
        Self {
            rules: Vec::new(),
            default_backend: "filesystem".to_string(),
            content_based_routing: false,
        }
    }
}

impl Default for StorageFailoverConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: FailoverStrategy::RoundRobin,
            timeout: Duration::from_secs(30),
            health_check: FailoverHealthCheckConfig::default(),
        }
    }
}

impl Default for FailoverHealthCheckConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(30),
            failure_threshold: 3,
            recovery_threshold: 2,
        }
    }
}

impl Default for StorageLoadBalancingConfig {
    fn default() -> Self {
        Self {
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            sticky_sessions: false,
            session_timeout: Duration::from_secs(300),
            weights: HashMap::new(),
        }
    }
}

impl Default for StorageConnectionConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_connections: 100,
            retry: ConnectionRetryConfig::default(),
            pooling: ConnectionPoolConfig::default(),
            tls: None,
        }
    }
}

impl Default for ConnectionRetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            strategy: RetryStrategy::Exponential,
        }
    }
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            min_size: 1,
            max_size: 10,
            idle_timeout: Duration::from_secs(600),
            max_lifetime: Duration::from_secs(3600),
        }
    }
}

impl Default for StorageLimitsConfig {
    fn default() -> Self {
        Self {
            max_capacity: None,
            max_file_size: Some(1024 * 1024 * 1024), // 1GB
            max_files: None,
            rate_limits: RateLimitsConfig::default(),
        }
    }
}

impl Default for StorageHealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            endpoint: None,
            failure_threshold: 3,
            recovery_threshold: 2,
        }
    }
}

// ==================== BUILDER METHODS ====================

impl StorageBackendConfig {
    /// Create a configuration optimized for production environments
    #[must_use]
    pub fn production_optimized() -> Self {
        Self {
            default_backend: StorageBackendType::Zfs,
            backends: HashMap::new(),
            routing: StorageRoutingConfig {
                content_based_routing: true,
                ..Default::default()
            },
            failover: StorageFailoverConfig {
                enabled: true,
                strategy: FailoverStrategy::Priority,
                timeout: Duration::from_secs(10),
                ..Default::default()
            },
            load_balancing: StorageLoadBalancingConfig {
                algorithm: LoadBalancingAlgorithm::LeastConnections,
                ..Default::default()
            },
        }
    }

    /// Create a configuration optimized for development environments
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            default_backend: StorageBackendType::Filesystem,
            backends: HashMap::new(),
            routing: StorageRoutingConfig::default(),
            failover: StorageFailoverConfig {
                enabled: false,
                ..Default::default()
            },
            load_balancing: StorageLoadBalancingConfig::default(),
        }
    }

    /// Create a configuration for high-performance environments
    #[must_use]
    pub fn high_performance() -> Self {
        Self {
            default_backend: StorageBackendType::Memory,
            backends: HashMap::new(),
            routing: StorageRoutingConfig {
                content_based_routing: true,
                ..Default::default()
            },
            failover: StorageFailoverConfig {
                enabled: true,
                strategy: FailoverStrategy::Weighted,
                timeout: Duration::from_secs(5),
                ..Default::default()
            },
            load_balancing: StorageLoadBalancingConfig {
                algorithm: LoadBalancingAlgorithm::LeastResponseTime,
                ..Default::default()
            },
        }
    }

    /// Create a configuration for cloud-native environments
    #[must_use]
    pub fn cloud_native() -> Self {
        Self {
            default_backend: StorageBackendType::S3Compatible,
            backends: HashMap::new(),
            routing: StorageRoutingConfig {
                content_based_routing: true,
                ..Default::default()
            },
            failover: StorageFailoverConfig {
                enabled: true,
                strategy: FailoverStrategy::Geolocation,
                timeout: Duration::from_secs(15),
                ..Default::default()
            },
            load_balancing: StorageLoadBalancingConfig {
                algorithm: LoadBalancingAlgorithm::Consistent,
                ..Default::default()
            },
        }
    }

    /// Merge with another configuration
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        // Simple merge - in a real implementation, you'd want more sophisticated merging
        self
    }

    /// Validate the backend configuration
    pub fn validate(&self) -> crate::Result<()> {
        // Validate that default backend exists in backends map
        if !self.backends.is_empty()
            && !self
                .backends
                .contains_key(&format!("{:?}", self.default_backend).to_lowercase())
        {
            return Err(crate::NestGateError::validation_error(
                "Default backend is not configured in backends map",
            ));
        }

        // Validate routing configuration
        for rule in &self.routing.rules {
            if !self.backends.contains_key(&rule.backend) {
                return Err(crate::NestGateError::validation_error(&format!(
                    "Routing rule '{}' references non-existent backend '{}'",
                    rule.name, rule.backend
                )));
            }
        }

        Ok(())
    }

    /// Get available storage backends
    #[must_use]
    pub fn get_available_backends(&self) -> Vec<StorageBackendType> {
        self.backends
            .values()
            .map(|b| b.backend_type.clone())
            .collect()
    }

    /// Check if a specific backend is configured
    #[must_use]
    pub fn has_backend(&self, backend_type: &StorageBackendType) -> bool {
        self.backends
            .values()
            .any(|b| &b.backend_type == backend_type)
    }
}
