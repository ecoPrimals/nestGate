// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **CANONICAL STORAGE CONFIGURATION MODULE**
//!
//! The single source of truth for all storage configuration across NestGate.
//! Consolidates StorageConfig, UnifiedStorageConfig, and storage domain configs.
//!
//! This module consolidates ALL storage configuration variants across the NestGate ecosystem
//! into a single, authoritative configuration structure.
//!
//! **CONSOLIDATES**:
//! - nestgate-canonical/src/types.rs → `StorageConfig`
//! - nestgate-core/src/canonical_modernization/unified_types.rs → `CanonicalStorageConfig`
//! - nestgate-zfs/src/config.rs → `ZfsConfig`
//! - 25+ other storage-related configuration structures
//!
//! **MODULAR STRUCTURE**:
//! - `backends`: Storage backend configurations (FS, ZFS, S3, Azure, GCS)
//! - `zfs`: ZFS-specific storage configurations
//! - `caching`: Caching and performance configurations
//! - `replication`: Data replication and backup configurations
//! - `encryption`: Storage encryption and security configurations
//! - `performance`: Performance optimization configurations
//! - `monitoring`: Storage monitoring and observability configurations
//! - `lifecycle`: Data lifecycle management configurations
//! - `environment`: Environment-specific storage settings

use serde::{Deserialize, Serialize};

// Import all storage configuration modules
pub mod backends;

/// **Caching Configuration Module**
///
/// Configures caching behavior, cache sizes, TTLs, and invalidation strategies
/// for storage operations.
pub mod caching;
/// **Encryption Configuration Module**
///
/// Configures encryption at rest, key management, and cryptographic algorithms
/// for storage security.
pub mod encryption;
/// **Environment Configuration Module**
///
/// Configures environment-specific storage settings, paths, and backend selection
/// for different deployment environments.
pub mod environment;
/// **Lifecycle Management Configuration Module**
///
/// Configures data lifecycle policies, retention periods, archival rules,
/// and automated data management.
pub mod lifecycle;
/// **Monitoring Configuration Module**
///
/// Configures storage metrics collection, health checks, alerts, and observability
/// for storage operations.
pub mod monitoring;
/// **Performance Configuration Module**
///
/// Configures performance tuning, buffer sizes, I/O optimization, and throughput
/// settings for storage operations.
pub mod performance;
/// **Replication Configuration Module**
///
/// Configures data replication, redundancy levels, failover strategies,
/// and distributed storage synchronization.
pub mod replication;
pub mod zfs;

// Re-export all ZFS types for easier access
pub use zfs::{
    AlertThresholds,
    ArcCacheConfig,
    L2ArcConfig,
    PrefetchConfig,
    // Sub-configurations
    RetentionPolicy,
    // Enums
    ZfsCompression,
    ZfsDatasetConfig,
    ZfsMaintenanceConfig,
    ZfsMigrationConfig,
    ZfsMonitoringConfig,
    ZfsPerformanceConfig,
    ZfsPoolConfig,
    ZfsPoolSettings,
    ZfsRedundancy,
    ZfsSecurityConfig,
    ZfsSnapshotConfig,
    // Main configurations
    ZfsStorageConfig,
    ZilConfig,
};

// Re-export all configuration types
pub use backends::{
    StorageBackend, StorageBackendConfig, StorageBackendSpecificConfig, StorageBackendType,
    StorageConnectionConfig, StorageFailoverConfig, StorageHealthCheckConfig, StorageLimitsConfig,
    StorageLoadBalancingConfig, StorageRoutingConfig,
};
pub use caching::{
    CacheConfig, CacheConsistencyConfig, CacheEvictionPolicy, CacheMonitoringConfig,
    CachePerformanceConfig, StorageCachingConfig,
};
pub use encryption::{
    DataEncryptionConfig, EncryptionAlgorithmStorageConfig, EncryptionConfig,
    KeyManagementStorageConfig, StorageEncryptionConfig, TransitEncryptionConfig,
};
pub use environment::{
    DeploymentStorageConfig, EnvironmentStorageSettings, RuntimeStorageConfig,
    StorageEnvironmentConfig, StorageFeatureConfig,
};
pub use lifecycle::{
    ArchivalConfig, ComplianceStorageConfig, DataLifecycleConfig, PurgingConfig, RetentionConfig,
    StorageLifecycleConfig,
};
pub use monitoring::{
    AlertingStorageConfig, DiagnosticsConfig, HealthCheckStorageConfig, LoggingStorageConfig,
    MetricsStorageConfig, StorageMonitoringConfig,
};
pub use performance::{
    CompressionConfig, DeduplicationConfig, IOOptimizationConfig, PerformanceOptimizationConfig,
    StoragePerformanceConfig, TuningConfig,
};
pub use replication::{
    BackupConfig, BackupStrategy, DisasterRecoveryConfig, RecoveryStrategy, ReplicationConfig,
    ReplicationStrategy, StorageReplicationConfig,
};

// ==================== CANONICAL STORAGE CONFIGURATION ====================

// **THE** canonical storage configuration for the entire NestGate ecosystem
// This replaces ALL other StorageConfig variants
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for CanonicalStorage
pub struct CanonicalStorageConfig {
    /// Storage backend configurations
    pub backends: StorageBackendConfig,

    /// ZFS-specific configuration
    pub zfs: ZfsStorageConfig,

    /// Caching and performance configuration
    pub caching: StorageCachingConfig,

    /// Data replication and backup configuration
    pub replication: StorageReplicationConfig,

    /// Encryption and security configuration
    pub encryption: StorageEncryptionConfig,

    /// Performance and optimization settings
    pub performance: StoragePerformanceConfig,

    /// Monitoring and observability
    pub monitoring: StorageMonitoringConfig,

    /// Lifecycle management configuration
    pub lifecycle: StorageLifecycleConfig,

    /// Environment-specific overrides
    pub environment: StorageEnvironmentConfig,
}

impl CanonicalStorageConfig {
    /// Create a new canonical storage configuration
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a configuration optimized for production environments
    #[must_use]
    pub fn production_optimized() -> Self {
        Self {
            backends: StorageBackendConfig::production_optimized(),
            zfs: ZfsStorageConfig::production_optimized(),
            caching: StorageCachingConfig::production_optimized(),
            replication: StorageReplicationConfig::production_optimized(),
            encryption: StorageEncryptionConfig::production_optimized(),
            performance: StoragePerformanceConfig::production_optimized(),
            monitoring: StorageMonitoringConfig::production_optimized(),
            lifecycle: StorageLifecycleConfig::production_optimized(),
            environment: StorageEnvironmentConfig::production_optimized(),
        }
    }

    /// Create a configuration optimized for development environments
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            backends: StorageBackendConfig::development_optimized(),
            zfs: ZfsStorageConfig::development_optimized(),
            caching: StorageCachingConfig::development_optimized(),
            replication: StorageReplicationConfig::development_optimized(),
            encryption: StorageEncryptionConfig::development_optimized(),
            performance: StoragePerformanceConfig::development_optimized(),
            monitoring: StorageMonitoringConfig::development_optimized(),
            lifecycle: StorageLifecycleConfig::development_optimized(),
            environment: StorageEnvironmentConfig::development_optimized(),
        }
    }

    /// Create a configuration for high-performance environments
    #[must_use]
    pub fn high_performance() -> Self {
        Self {
            backends: StorageBackendConfig::high_performance(),
            zfs: ZfsStorageConfig::high_performance(),
            caching: StorageCachingConfig::high_performance(),
            replication: StorageReplicationConfig::high_performance(),
            encryption: StorageEncryptionConfig::high_performance(),
            performance: StoragePerformanceConfig::high_performance(),
            monitoring: StorageMonitoringConfig::high_performance(),
            lifecycle: StorageLifecycleConfig::high_performance(),
            environment: StorageEnvironmentConfig::high_performance(),
        }
    }

    /// Create a configuration for cloud-native environments
    #[must_use]
    pub fn cloud_native() -> Self {
        Self {
            backends: StorageBackendConfig::cloud_native(),
            zfs: ZfsStorageConfig::cloud_native(),
            caching: StorageCachingConfig::cloud_native(),
            replication: StorageReplicationConfig::cloud_native(),
            encryption: StorageEncryptionConfig::cloud_native(),
            performance: StoragePerformanceConfig::cloud_native(),
            monitoring: StorageMonitoringConfig::cloud_native(),
            lifecycle: StorageLifecycleConfig::cloud_native(),
            environment: StorageEnvironmentConfig::cloud_native(),
        }
    }

    /// Merge with another configuration (other takes precedence)
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.backends = self.backends.merge(other.backends);
        self.zfs = self.zfs.merge(other.zfs);
        self.caching = self.caching.merge(other.caching);
        self.replication = self.replication.merge(other.replication);
        self.encryption = self.encryption.merge(other.encryption);
        self.performance = self.performance.merge(other.performance);
        self.monitoring = self.monitoring.merge(other.monitoring);
        self.lifecycle = self.lifecycle.merge(other.lifecycle);
        self.environment = self.environment.merge(other.environment);
        self
    }

    /// Validate the storage configuration for completeness and consistency
    pub fn validate(&self) -> nestgate_types::error::Result<()> {
        // Validate backend configurations
        self.backends.validate()?;

        // Validate ZFS configuration
        self.zfs.validate()?;

        // Validate caching configuration
        self.caching.validate()?;

        // Validate replication configuration
        self.replication.validate()?;

        // Validate encryption configuration
        self.encryption.validate()?;

        // Validate performance configuration
        self.performance.validate()?;

        // Validate monitoring configuration
        self.monitoring.validate()?;

        // Validate lifecycle configuration
        self.lifecycle.validate()?;

        // Validate environment configuration
        self.environment.validate()?;

        Ok(())
    }

    /// Get storage capacity information
    #[must_use]
    pub fn get_total_capacity(&self) -> u64 {
        // Implementation would calculate total capacity across all backends
        0
    }

    /// Get available storage backends
    #[must_use]
    pub fn get_available_backends(&self) -> Vec<StorageBackendType> {
        self.backends.get_available_backends()
    }

    /// Check if a specific backend is configured
    #[must_use]
    pub fn has_backend(&self, backend_type: &StorageBackendType) -> bool {
        self.backends.has_backend(backend_type)
    }
}

// ==================== BACKWARD COMPATIBILITY ALIASES ====================

/// Backward compatibility alias for existing StorageConfig usage
pub type StorageConfig = CanonicalStorageConfig;

/// Backward compatibility alias for UnifiedStorageConfig
pub type UnifiedStorageConfig = CanonicalStorageConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_storage_config_new() {
        let config = CanonicalStorageConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_canonical_storage_config_get_total_capacity() {
        let config = CanonicalStorageConfig::default();
        assert_eq!(config.get_total_capacity(), 0);
    }

    #[test]
    fn test_canonical_storage_config_has_backend() {
        let config = CanonicalStorageConfig::default();
        assert!(!config.has_backend(&StorageBackendType::Zfs));
    }
}
