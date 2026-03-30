// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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
/// Configuration for `StorageBackend`
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of `StorageBackend`
pub enum StorageBackendType {
    /// Filesystem
    Filesystem,
    /// Zfs
    Zfs,
    /// `S3Compatible`
    S3Compatible,
    /// Azure
    Azure,
    /// Gcs
    Gcs,
    /// Memory
    Memory,
    /// Distributed
    Distributed,
    /// Custom storage backend type
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagebackend
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
/// Storage backend specific configuration
pub enum StorageBackendSpecificConfig {
    /// Filesystem backend configuration
    Filesystem {
        /// Root path for filesystem storage
        root_path: PathBuf,
        /// File permissions (Unix mode)
        permissions: u32,
        /// Whether to create directories if they don't exist
        create_dirs: bool,
    },
    /// ZFS backend configuration
    Zfs {
        /// ZFS pool name
        pool_name: String,
        /// Dataset prefix for namespacing
        dataset_prefix: String,
        /// ZFS compression algorithm
        compression: ZfsCompression,
        /// Enable ZFS deduplication
        deduplication: bool,
    },
    /// S3-compatible storage configuration
    S3Compatible {
        /// S3 endpoint URL
        endpoint: String,
        /// AWS region
        region: String,
        /// S3 bucket name
        bucket: String,
        /// AWS access key ID
        access_key_id: String,
        /// AWS secret access key
        secret_access_key: String,
        /// Use SSL/TLS for connections
        use_ssl: bool,
    },
    /// Azure Blob Storage configuration
    Azure {
        /// Azure storage account name
        account_name: String,
        /// Azure storage account key
        account_key: String,
        /// Azure blob container name
        container: String,
        /// Optional Azure endpoint suffix (e.g., for sovereign clouds)
        endpoint_suffix: Option<String>,
    },
    /// Google Cloud Storage configuration
    Gcs {
        /// GCP project ID
        project_id: String,
        /// GCS bucket name
        bucket: String,
        /// Path to service account credentials JSON file
        credentials_path: Option<PathBuf>,
        /// Service account key as JSON string
        service_account_key: Option<String>,
    },
    /// In-memory storage configuration
    Memory {
        /// Maximum memory size in bytes
        max_size: usize,
        /// Memory eviction policy
        eviction_policy: MemoryEvictionPolicy,
    },
    /// Distributed storage configuration
    Distributed {
        /// Distributed storage nodes
        nodes: Vec<DistributedStorageNode>,
        /// Consistency level for distributed operations
        consistency_level: ConsistencyLevel,
        /// Replication factor across nodes
        replication_factor: u32,
    },
    /// Custom storage backend configuration
    Custom {
        /// Custom JSON configuration
        config: serde_json::Value,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfscompression
pub enum ZfsCompression {
    /// Off
    Off,
    /// Lzjb
    Lzjb,
    /// Gzip
    Gzip,
    /// Zle
    Zle,
    /// Lz4
    Lz4,
    /// Zstd
    Zstd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Memoryevictionpolicy
pub enum MemoryEvictionPolicy {
    /// Lru
    Lru,
    /// Lfu
    Lfu,
    /// Fifo
    Fifo,
    /// Random
    Random,
    /// Ttl
    Ttl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Distributedstoragenode
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
/// Consistencylevel
pub enum ConsistencyLevel {
    /// Eventual
    Eventual,
    /// Strong
    Strong,
    /// Session
    Session,
    /// Boundedstaleness
    BoundedStaleness,
}

// ==================== CONNECTION CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `StorageConnection`
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
/// Configuration for `ConnectionRetry`
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
/// Retrystrategy
pub enum RetryStrategy {
    /// Fixed
    Fixed,
    /// Linear
    Linear,
    /// Exponential
    Exponential,
    /// Jitter
    Jitter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ConnectionPool`
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
/// Configuration for `ConnectionTls`
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
/// Configuration for `StorageLimits`
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
/// Configuration for `RateLimits`
pub struct RateLimitsConfig {
    /// Reads per second
    pub reads_per_second: Option<u32>,

    /// Writes per second
    pub writes_per_second: Option<u32>,

    /// Bandwidth limit (bytes per second)
    pub bandwidth_limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `StorageHealthCheck`
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
/// Configuration for `StorageRouting`
pub struct StorageRoutingConfig {
    /// Routing rules
    pub rules: Vec<RoutingRule>,

    /// Default backend for unmatched requests
    pub default_backend: String,

    /// Enable content-based routing
    pub content_based_routing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Routingrule
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
/// Routing condition for backend selection
pub enum RoutingCondition {
    /// Match based on path prefix
    PathPrefix(String),
    /// Match based on file extension
    FileExtension(String),
    /// Match based on file size
    FileSize(FileSizeCondition),
    /// Match based on content type
    ContentType(String),
    /// Custom routing condition
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Filesizecondition
pub struct FileSizeCondition {
    /// Operator
    pub operator: ComparisonOperator,
    /// Size
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Comparisonoperator
pub enum ComparisonOperator {
    /// Greaterthan
    GreaterThan,
    /// Lessthan
    LessThan,
    /// Equal
    Equal,
    /// Greaterthanorequal
    GreaterThanOrEqual,
    /// Lessthanorequal
    LessThanOrEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `StorageFailover`
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
/// Failoverstrategy
pub enum FailoverStrategy {
    /// Roundrobin
    RoundRobin,
    /// Priority
    Priority,
    /// Weighted
    Weighted,
    /// Geolocation
    Geolocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `FailoverHealthCheck`
pub struct FailoverHealthCheckConfig {
    /// Health check interval
    pub interval: Duration,

    /// Consecutive failures before failover
    pub failure_threshold: u32,

    /// Consecutive successes before recovery
    pub recovery_threshold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `StorageLoadBalancing`
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
/// Loadbalancingalgorithm
pub enum LoadBalancingAlgorithm {
    /// Roundrobin
    RoundRobin,
    /// Weightedroundrobin
    WeightedRoundRobin,
    /// Leastconnections
    LeastConnections,
    /// Leastresponsetime
    LeastResponseTime,
    /// Random
    Random,
    /// Consistent
    Consistent,
}

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for StorageBackendConfig {
    /// Returns the default instance
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
    /// Returns the default instance
    fn default() -> Self {
        Self {
            rules: Vec::new(),
            default_backend: "filesystem".to_string(),
            content_based_routing: false,
        }
    }
}

impl Default for StorageFailoverConfig {
    /// Returns the default instance
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
    /// Returns the default instance
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(30),
            failure_threshold: 3,
            recovery_threshold: 2,
        }
    }
}

impl Default for StorageLoadBalancingConfig {
    /// Returns the default instance
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
    /// Returns the default instance
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
    /// Returns the default instance
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
    /// Returns the default instance
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
    /// Returns the default instance
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
    /// Returns the default instance
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
    pub fn validate(&self) -> nestgate_types::error::Result<()> {
        // Validate that default backend exists in backends map
        if !self.backends.is_empty()
            && !self
                .backends
                .contains_key(&format!("{:?}", self.default_backend).to_lowercase())
        {
            return Err(nestgate_types::error::NestGateError::validation_error(
                "Default backend is not configured in backends map",
            ));
        }

        // Validate routing configuration
        for rule in &self.routing.rules {
            if !self.backends.contains_key(&rule.backend) {
                return Err(nestgate_types::error::NestGateError::validation_error(
                    format!(
                        "Routing rule '{}' references non-existent backend '{}'",
                        rule.name, rule.backend
                    ),
                ));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_backend_config_default() {
        let config = StorageBackendConfig::default();
        assert!(matches!(
            config.default_backend,
            StorageBackendType::Filesystem
        ));
    }

    #[test]
    fn test_storage_backend_type_variants() {
        let _fs = StorageBackendType::Filesystem;
        let _zfs = StorageBackendType::Zfs;
        let custom = StorageBackendType::Custom("mybackend".to_string());
        assert_eq!(custom, StorageBackendType::Custom("mybackend".to_string()));
    }

    #[test]
    fn test_storage_backend_config_validate_empty() {
        let config = StorageBackendConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_rate_limits_config_default() {
        let rl = RateLimitsConfig::default();
        assert!(rl.reads_per_second.is_none());
    }

    #[test]
    fn test_zfs_compression_enum() {
        let _off = ZfsCompression::Off;
        let _lz4 = ZfsCompression::Lz4;
    }

    #[test]
    fn comparison_and_eviction_enums_serde_roundtrip() {
        for op in [
            ComparisonOperator::GreaterThan,
            ComparisonOperator::LessThan,
            ComparisonOperator::Equal,
            ComparisonOperator::GreaterThanOrEqual,
            ComparisonOperator::LessThanOrEqual,
        ] {
            let json = serde_json::to_string(&op).expect("ser op");
            let back: ComparisonOperator = serde_json::from_str(&json).expect("de op");
            assert_eq!(format!("{op:?}"), format!("{back:?}"));
        }
        for pol in [
            MemoryEvictionPolicy::Lru,
            MemoryEvictionPolicy::Lfu,
            MemoryEvictionPolicy::Fifo,
            MemoryEvictionPolicy::Random,
            MemoryEvictionPolicy::Ttl,
        ] {
            let json = serde_json::to_string(&pol).expect("ser pol");
            let back: MemoryEvictionPolicy = serde_json::from_str(&json).expect("de pol");
            assert_eq!(format!("{pol:?}"), format!("{back:?}"));
        }
        for level in [
            ConsistencyLevel::Eventual,
            ConsistencyLevel::Strong,
            ConsistencyLevel::Session,
            ConsistencyLevel::BoundedStaleness,
        ] {
            let json = serde_json::to_string(&level).expect("ser cl");
            let back: ConsistencyLevel = serde_json::from_str(&json).expect("de cl");
            assert_eq!(format!("{level:?}"), format!("{back:?}"));
        }
        for s in [
            RetryStrategy::Fixed,
            RetryStrategy::Linear,
            RetryStrategy::Exponential,
            RetryStrategy::Jitter,
        ] {
            let json = serde_json::to_string(&s).expect("ser retry");
            let back: RetryStrategy = serde_json::from_str(&json).expect("de retry");
            assert_eq!(format!("{s:?}"), format!("{back:?}"));
        }
        for f in [
            FailoverStrategy::RoundRobin,
            FailoverStrategy::Priority,
            FailoverStrategy::Weighted,
            FailoverStrategy::Geolocation,
        ] {
            let json = serde_json::to_string(&f).expect("ser fo");
            let back: FailoverStrategy = serde_json::from_str(&json).expect("de fo");
            assert_eq!(format!("{f:?}"), format!("{back:?}"));
        }
        for lb in [
            LoadBalancingAlgorithm::RoundRobin,
            LoadBalancingAlgorithm::WeightedRoundRobin,
            LoadBalancingAlgorithm::LeastConnections,
            LoadBalancingAlgorithm::LeastResponseTime,
            LoadBalancingAlgorithm::Random,
            LoadBalancingAlgorithm::Consistent,
        ] {
            let json = serde_json::to_string(&lb).expect("ser lb");
            let back: LoadBalancingAlgorithm = serde_json::from_str(&json).expect("de lb");
            assert_eq!(format!("{lb:?}"), format!("{back:?}"));
        }
    }

    fn sample_backend(
        backend_type: StorageBackendType,
        cfg: StorageBackendSpecificConfig,
    ) -> StorageBackend {
        StorageBackend {
            backend_type,
            config: cfg,
            connection: StorageConnectionConfig::default(),
            limits: StorageLimitsConfig::default(),
            health_check: StorageHealthCheckConfig::default(),
        }
    }

    #[test]
    fn storage_subconfig_defaults_exercise_impls() {
        let _ = StorageRoutingConfig::default();
        let _ = StorageFailoverConfig::default();
        let _ = FailoverHealthCheckConfig::default();
        let _ = StorageLoadBalancingConfig::default();
        let _ = ConnectionRetryConfig::default();
        let _ = ConnectionPoolConfig::default();
        let _ = StorageLimitsConfig::default();
        let _ = StorageHealthCheckConfig::default();
        let mut conn = StorageConnectionConfig::default();
        conn.tls = Some(ConnectionTlsConfig {
            enabled: true,
            verify_certificates: false,
            ca_cert_path: Some(PathBuf::from("/tmp/ca.pem")),
            client_cert_path: Some(PathBuf::from("/tmp/client.pem")),
            client_key_path: Some(PathBuf::from("/tmp/client.key")),
        });
        let _ = conn;
    }

    #[test]
    fn storage_backend_factory_presets() {
        let prod = StorageBackendConfig::production_optimized();
        assert!(matches!(prod.default_backend, StorageBackendType::Zfs));
        let dev = StorageBackendConfig::development_optimized();
        assert!(matches!(
            dev.default_backend,
            StorageBackendType::Filesystem
        ));
        let hp = StorageBackendConfig::high_performance();
        assert!(matches!(hp.default_backend, StorageBackendType::Memory));
        let cloud = StorageBackendConfig::cloud_native();
        assert!(matches!(
            cloud.default_backend,
            StorageBackendType::S3Compatible
        ));
    }

    #[test]
    fn storage_backend_validate_and_helpers() {
        let base = StorageBackendConfig::default();
        assert!(base.clone().merge(base.clone()).validate().is_ok());

        let mut bad_default = StorageBackendConfig::default();
        bad_default.default_backend = StorageBackendType::Zfs;
        bad_default.backends.insert(
            "other".to_string(),
            sample_backend(
                StorageBackendType::Filesystem,
                StorageBackendSpecificConfig::Filesystem {
                    root_path: PathBuf::from("/tmp"),
                    permissions: 0o755,
                    create_dirs: true,
                },
            ),
        );
        assert!(bad_default.validate().is_err());

        let mut bad_rule = StorageBackendConfig::default();
        bad_rule.backends.insert(
            "filesystem".to_string(),
            sample_backend(
                StorageBackendType::Filesystem,
                StorageBackendSpecificConfig::Filesystem {
                    root_path: PathBuf::from("/data"),
                    permissions: 0o755,
                    create_dirs: false,
                },
            ),
        );
        bad_rule.routing.rules.push(RoutingRule {
            name: "missing-target".to_string(),
            condition: RoutingCondition::PathPrefix("/x".to_string()),
            backend: "nowhere".to_string(),
            priority: 1,
        });
        assert!(bad_rule.validate().is_err());

        let mut ok = StorageBackendConfig::default();
        ok.backends.insert(
            "filesystem".to_string(),
            sample_backend(
                StorageBackendType::Filesystem,
                StorageBackendSpecificConfig::Filesystem {
                    root_path: PathBuf::from("/mnt"),
                    permissions: 0o755,
                    create_dirs: true,
                },
            ),
        );
        let backends = ok.get_available_backends();
        assert_eq!(backends.len(), 1);
        assert!(ok.has_backend(&StorageBackendType::Filesystem));
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn storage_backend_config_full_serde_roundtrip() {
        let distributed_node = DistributedStorageNode {
            id: "n1".to_string(),
            endpoint: "10.0.0.1:9000".to_string(),
            weight: 3,
            availability_zone: Some("a".to_string()),
        };
        let mut backends_map = HashMap::new();
        backends_map.insert(
            "filesystem".to_string(),
            sample_backend(
                StorageBackendType::Filesystem,
                StorageBackendSpecificConfig::Filesystem {
                    root_path: PathBuf::from("/data/fs"),
                    permissions: 0o700,
                    create_dirs: true,
                },
            ),
        );
        backends_map.insert(
            "zfs".to_string(),
            sample_backend(
                StorageBackendType::Zfs,
                StorageBackendSpecificConfig::Zfs {
                    pool_name: "tank".to_string(),
                    dataset_prefix: "app".to_string(),
                    compression: ZfsCompression::Zstd,
                    deduplication: false,
                },
            ),
        );
        backends_map.insert(
            "s3".to_string(),
            sample_backend(
                StorageBackendType::S3Compatible,
                StorageBackendSpecificConfig::S3Compatible {
                    endpoint: "https://s3.example.com".to_string(),
                    region: "us-east-1".to_string(),
                    bucket: "b".to_string(),
                    access_key_id: "k".to_string(),
                    secret_access_key: "s".to_string(),
                    use_ssl: true,
                },
            ),
        );
        backends_map.insert(
            "azure".to_string(),
            sample_backend(
                StorageBackendType::Azure,
                StorageBackendSpecificConfig::Azure {
                    account_name: "acct".to_string(),
                    account_key: "key".to_string(),
                    container: "c".to_string(),
                    endpoint_suffix: Some("core.windows.net".to_string()),
                },
            ),
        );
        backends_map.insert(
            "gcs".to_string(),
            sample_backend(
                StorageBackendType::Gcs,
                StorageBackendSpecificConfig::Gcs {
                    project_id: "p".to_string(),
                    bucket: "gb".to_string(),
                    credentials_path: Some(PathBuf::from("/tmp/cred.json")),
                    service_account_key: None,
                },
            ),
        );
        backends_map.insert(
            "memory".to_string(),
            sample_backend(
                StorageBackendType::Memory,
                StorageBackendSpecificConfig::Memory {
                    max_size: 1024,
                    eviction_policy: MemoryEvictionPolicy::Lfu,
                },
            ),
        );
        backends_map.insert(
            "distributed".to_string(),
            sample_backend(
                StorageBackendType::Distributed,
                StorageBackendSpecificConfig::Distributed {
                    nodes: vec![distributed_node],
                    consistency_level: ConsistencyLevel::Strong,
                    replication_factor: 3,
                },
            ),
        );
        backends_map.insert(
            "custom".to_string(),
            sample_backend(
                StorageBackendType::Custom("legacy".to_string()),
                StorageBackendSpecificConfig::Custom {
                    config: serde_json::json!({ "k": 1 }),
                },
            ),
        );

        let cfg = StorageBackendConfig {
            default_backend: StorageBackendType::Filesystem,
            backends: backends_map,
            routing: StorageRoutingConfig {
                rules: vec![
                    RoutingRule {
                        name: "p".to_string(),
                        condition: RoutingCondition::PathPrefix("/a".to_string()),
                        backend: "filesystem".to_string(),
                        priority: 10,
                    },
                    RoutingRule {
                        name: "e".to_string(),
                        condition: RoutingCondition::FileExtension("txt".to_string()),
                        backend: "zfs".to_string(),
                        priority: 5,
                    },
                    RoutingRule {
                        name: "fsz".to_string(),
                        condition: RoutingCondition::FileSize(FileSizeCondition {
                            operator: ComparisonOperator::GreaterThanOrEqual,
                            size: 100,
                        }),
                        backend: "s3".to_string(),
                        priority: 3,
                    },
                    RoutingRule {
                        name: "ct".to_string(),
                        condition: RoutingCondition::ContentType("application/json".to_string()),
                        backend: "azure".to_string(),
                        priority: 2,
                    },
                    RoutingRule {
                        name: "c".to_string(),
                        condition: RoutingCondition::Custom("x".to_string()),
                        backend: "gcs".to_string(),
                        priority: 1,
                    },
                ],
                default_backend: "filesystem".to_string(),
                content_based_routing: true,
            },
            failover: StorageFailoverConfig::default(),
            load_balancing: StorageLoadBalancingConfig::default(),
        };

        let json = serde_json::to_string(&cfg).expect("serialize");
        let back: StorageBackendConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.backends.len(), cfg.backends.len());
        assert_eq!(back.routing.rules.len(), cfg.routing.rules.len());
        assert!(back.validate().is_ok());
    }
}
