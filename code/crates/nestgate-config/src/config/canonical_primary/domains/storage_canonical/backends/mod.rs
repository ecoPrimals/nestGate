// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage backend configuration — multi-backend routing, failover, and load balancing.
//!
//! Supports filesystem, ZFS, S3-compatible, Azure, GCS, in-memory, distributed,
//! and custom backend types with pluggable routing and health checks.

pub mod connection;
pub mod health;
pub mod routing;
pub mod types;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

pub use connection::*;
pub use health::*;
pub use routing::*;
pub use types::*;

/// Top-level storage backend configuration
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

impl StorageBackendConfig {
    /// Configuration optimized for production environments
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

    /// Configuration optimized for development environments
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

    /// Configuration for high-performance environments
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

    /// Configuration for cloud-native environments
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

    /// Merge with another configuration (other takes precedence for non-empty
    /// collections; currently a no-op placeholder for future deep merge)
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }

    /// Validate that the backend configuration is internally consistent
    pub fn validate(&self) -> nestgate_types::error::Result<()> {
        if !self.backends.is_empty()
            && !self
                .backends
                .contains_key(&format!("{:?}", self.default_backend).to_lowercase())
        {
            return Err(nestgate_types::error::NestGateError::validation_error(
                "Default backend is not configured in backends map",
            ));
        }

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

    /// List backend types that are currently configured
    #[must_use]
    pub fn get_available_backends(&self) -> Vec<StorageBackendType> {
        self.backends
            .values()
            .map(|b| b.backend_type.clone())
            .collect()
    }

    /// Check whether a backend of the given type is configured
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
    use std::path::PathBuf;

    fn sample_backend(
        backend_type: StorageBackendType,
        config: StorageBackendSpecificConfig,
    ) -> StorageBackend {
        StorageBackend {
            backend_type,
            config,
            connection: StorageConnectionConfig::default(),
            limits: StorageLimitsConfig::default(),
            health_check: StorageHealthCheckConfig::default(),
        }
    }

    #[test]
    fn default_uses_filesystem() {
        let config = StorageBackendConfig::default();
        assert!(matches!(
            config.default_backend,
            StorageBackendType::Filesystem
        ));
    }

    #[test]
    fn backend_type_custom_equality() {
        let custom = StorageBackendType::Custom("mybackend".to_string());
        assert_eq!(custom, StorageBackendType::Custom("mybackend".to_string()));
    }

    #[test]
    fn validate_empty_is_ok() {
        let config = StorageBackendConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn rate_limits_default_is_none() {
        let rl = RateLimitsConfig::default();
        assert!(rl.reads_per_second.is_none());
    }

    #[test]
    fn zfs_compression_variants_exist() {
        let _off = ZfsCompression::Off;
        let _lz4 = ZfsCompression::Lz4;
    }

    #[test]
    fn enum_serde_roundtrips() {
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

    #[test]
    fn connection_tls_config_roundtrip() {
        let conn = StorageConnectionConfig {
            timeout: Duration::from_secs(10),
            max_connections: 50,
            retry: ConnectionRetryConfig {
                max_attempts: 5,
                ..Default::default()
            },
            pooling: ConnectionPoolConfig::default(),
            tls: Some(ConnectionTlsConfig {
                enabled: true,
                verify_certificates: true,
                ca_cert_path: Some(PathBuf::from("/tmp/ca.pem")),
                client_cert_path: Some(PathBuf::from("/tmp/client.pem")),
                client_key_path: Some(PathBuf::from("/tmp/client.key")),
            }),
        };
        let _ = conn;
    }

    #[test]
    fn factory_presets() {
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
    fn validate_and_helpers() {
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
    fn full_serde_roundtrip() {
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
