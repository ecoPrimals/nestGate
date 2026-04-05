// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

///
/// This module contains all configuration structures and settings
/// for the storage management system.
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::info;
/// ZFS configuration for command execution
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Zfs
pub struct ZfsConfig {
    /// Zfs Binary
    pub zfs_binary: String,
    /// Zpool Binary
    pub zpool_binary: String,
    /// Use Sudo
    pub use_sudo: bool,
    /// Command Timeout
    pub command_timeout: Duration,
}
impl Default for ZfsConfig {
    /// Returns the default instance with environment-aware binary paths
    ///
    /// **Evolution** (Jan 30, 2026): Now uses environment variables for binary paths.
    fn default() -> Self {
        // ✅ EVOLVED: Use environment-aware paths instead of hardcoded /usr/sbin/
        let zfs_binary = crate::config::storage_paths::get_storage_paths()
            .zfs_binary_path()
            .to_string_lossy()
            .to_string();

        let zpool_binary = crate::config::storage_paths::get_storage_paths()
            .zpool_binary_path()
            .to_string_lossy()
            .to_string();

        Self {
            zfs_binary,
            zpool_binary,
            use_sudo: true,
            command_timeout: Duration::from_secs(30),
        }
    }
}

/// Cache configuration policies
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cachepolicies
pub struct CachePolicies {
    /// Eviction Strategy
    pub eviction_strategy: EvictionPolicy,
    /// Compression
    pub compression: bool,
    /// Deduplication
    pub deduplication: bool,
}
impl Default for CachePolicies {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            eviction_strategy: EvictionPolicy::Lru,
            compression: false,
            deduplication: false,
        }
    }
}

/// Cache eviction policies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Evictionpolicy
pub enum EvictionPolicy {
    /// Least Recently Used eviction policy
    Lru,
    /// Least Frequently Used eviction policy
    Lfu,
    /// ZFS Adaptive Replacement Cache eviction policy
    Arc,
    /// Random eviction policy
    Random,
}
/// Runtime configuration for [`crate::services::storage::StorageManagerService`]
/// (ZFS paths, discovery intervals, quotas, caching).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageServiceConfig {
    /// Base path for storage (filesystem backend)
    pub base_path: String,
    /// ZFS configuration
    pub zfs: ZfsConfig,
    /// Enable automatic pool discovery
    pub auto_discover_pools: bool,
    /// Pool discovery interval in seconds
    pub discovery_interval: u64,
    /// Enable quota management
    pub enable_quotas: bool,
    /// Quota check interval in seconds
    pub quota_check_interval: u64,
    /// Enable caching
    pub enable_caching: bool,
    /// Default cache policies
    pub default_cache_policies: CachePolicies,
    /// Maximum concurrent operations
    pub max_concurrent_operations: usize,
    /// Operation timeout in seconds
    pub operation_timeout: u64,
    /// Enable background monitoring
    pub enable_monitoring: bool,
    /// Monitoring interval in seconds
    pub monitoring_interval: u64,
}
impl Default for StorageServiceConfig {
    /// Returns the default instance with XDG-compliant paths
    ///
    /// **Evolution** (Jan 30, 2026): Now uses XDG-compliant storage paths
    /// instead of hardcoded `/var/lib/nestgate/storage`.
    fn default() -> Self {
        // ✅ EVOLVED: Use XDG-compliant storage path instead of hardcoded /var/lib/
        let base_path = crate::config::storage_paths::get_storage_base_path()
            .to_string_lossy()
            .to_string();

        Self {
            base_path,
            zfs: ZfsConfig::default(),
            auto_discover_pools: true,
            discovery_interval: 300, // 5 minutes
            enable_quotas: true,
            quota_check_interval: 600, // 10 minutes
            enable_caching: true,
            default_cache_policies: CachePolicies::default(),
            max_concurrent_operations: 10,
            operation_timeout: 300, // 5 minutes
            enable_monitoring: true,
            monitoring_interval: 60, // 1 minute
        }
    }
}

impl ZfsConfig {
    /// Create a new ZFS configuration
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a configuration for development (no sudo)
    #[must_use]
    pub fn development() -> Self {
        Self {
            use_sudo: false,
            command_timeout: Duration::from_secs(10),
            ..Default::default()
        }
    }

    /// Create a configuration for production
    #[must_use]
    pub fn production() -> Self {
        Self {
            use_sudo: true,
            command_timeout: Duration::from_secs(60),
            ..Default::default()
        }
    }

    /// Validate the configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate(&self) -> Result<(), String> {
        if self.zfs_binary.is_empty() {
            return Err("ZFS binary path cannot be empty".to_string());
        }

        if self.zpool_binary.is_empty() {
            return Err("ZPool binary path cannot be empty".to_string());
        }

        if self.command_timeout.as_secs() == 0 {
            return Err("Command timeout must be greater than zero".to_string());
        }

        // Check if binaries exist (in a real implementation)
        // For now, we'll assume they exist

        Ok(())
    }
}

impl StorageServiceConfig {
    /// Create a new storage service configuration
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create configuration with auto-detected backend capabilities
    ///
    /// ✅ DEEP DEBT: Agnostic, capability-based (no hardcoding)
    /// ✅ UNIVERSAL: Works on ANY filesystem
    /// ✅ OPTIMIZED: Uses ZFS features when available
    #[must_use]
    pub fn with_auto_detect() -> Self {
        use super::capabilities;

        // Detect available storage backends
        let caps = capabilities::detect_and_log();

        let mut config = Self::default();

        // ✅ CAPABILITY-BASED: Only enable ZFS features if ZFS is available
        match caps.backend_type {
            capabilities::BackendType::Zfs => {
                // ZFS available - enable optimization features
                config.auto_discover_pools = true;
                config.enable_quotas = true;
                info!("🚀 ZFS optimization enabled (native features available)");
            }
            capabilities::BackendType::Filesystem => {
                // No ZFS - use filesystem-only mode
                config.auto_discover_pools = false;
                config.enable_quotas = false;
                info!("🌍 Filesystem mode (universal compatibility)");
                info!("   Works on: ext4, NTFS, APFS, btrfs, XFS, etc.");
            }
        }

        config
    }

    /// Create a development configuration
    #[must_use]
    pub fn development() -> Self {
        Self {
            zfs: ZfsConfig::development(),
            discovery_interval: 60,    // 1 minute for faster development
            quota_check_interval: 120, // 2 minutes
            monitoring_interval: 30,   // 30 seconds
            operation_timeout: 60,     // 1 minute
            ..Default::default()
        }
    }

    /// Create a production configuration
    #[must_use]
    pub fn production() -> Self {
        Self {
            zfs: ZfsConfig::production(),
            discovery_interval: 600,    // 10 minutes
            quota_check_interval: 1800, // 30 minutes
            monitoring_interval: 300,   // 5 minutes
            operation_timeout: 600,     // 10 minutes
            max_concurrent_operations: 20,
            ..Default::default()
        }
    }

    /// Validate the configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate(&self) -> Result<(), String> {
        // Validate ZFS configuration
        self.zfs.validate()?;

        if self.discovery_interval == 0 {
            return Err("Discovery interval must be greater than zero".to_string());
        }

        if self.quota_check_interval == 0 {
            return Err("Quota check interval must be greater than zero".to_string());
        }

        if self.monitoring_interval == 0 {
            return Err("Monitoring interval must be greater than zero".to_string());
        }

        if self.operation_timeout == 0 {
            return Err("Operation timeout must be greater than zero".to_string());
        }

        if self.max_concurrent_operations == 0 {
            return Err("Max concurrent operations must be greater than zero".to_string());
        }

        Ok(())
    }

    /// Get discovery interval as Duration
    #[must_use]
    pub const fn discovery_interval_duration(&self) -> Duration {
        Duration::from_secs(self.discovery_interval)
    }

    /// Get quota check interval as Duration
    #[must_use]
    pub const fn quota_check_interval_duration(&self) -> Duration {
        Duration::from_secs(self.quota_check_interval)
    }

    /// Get monitoring interval as Duration
    #[must_use]
    pub const fn monitoring_interval_duration(&self) -> Duration {
        Duration::from_secs(self.monitoring_interval)
    }

    /// Get operation timeout as Duration
    #[must_use]
    pub const fn operation_timeout_duration(&self) -> Duration {
        Duration::from_secs(self.operation_timeout)
    }
}

/// Domain-level consolidated storage settings (`nestgate-config` canonical primary).
///
/// [`StorageServiceConfig`] remains the focused runtime type for the core storage service;
/// use this alias when integrating with full domain configuration.
pub type StorageServiceConfigCanonical =
    crate::config::canonical_primary::domains::storage_canonical::CanonicalStorageConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zfs_config_default() {
        // Save/restore to avoid env-var race conditions with parallel tests
        let orig_zfs = std::env::var("NESTGATE_ZFS_BINARY").ok();
        let orig_zpool = std::env::var("NESTGATE_ZPOOL_BINARY").ok();
        crate::env_process::remove_var("NESTGATE_ZFS_BINARY");
        crate::env_process::remove_var("NESTGATE_ZPOOL_BINARY");

        let config = ZfsConfig::default();
        assert_eq!(config.zfs_binary, "/usr/sbin/zfs");
        assert_eq!(config.zpool_binary, "/usr/sbin/zpool");
        assert!(config.use_sudo);
        assert_eq!(config.command_timeout, Duration::from_secs(30));

        match orig_zfs {
            Some(v) => crate::env_process::set_var("NESTGATE_ZFS_BINARY", v),
            None => crate::env_process::remove_var("NESTGATE_ZFS_BINARY"),
        }
        match orig_zpool {
            Some(v) => crate::env_process::set_var("NESTGATE_ZPOOL_BINARY", v),
            None => crate::env_process::remove_var("NESTGATE_ZPOOL_BINARY"),
        }
    }

    #[test]
    fn test_zfs_config_development() {
        let config = ZfsConfig::development();
        assert!(!config.use_sudo);
        assert_eq!(config.command_timeout, Duration::from_secs(10));
    }

    #[test]
    fn test_zfs_config_validation() {
        let config = ZfsConfig::default();
        assert!(config.validate().is_ok());

        let config = ZfsConfig {
            zfs_binary: String::new(),
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_storage_service_config_default() {
        let config = StorageServiceConfig::default();
        assert!(config.auto_discover_pools);
        assert!(config.enable_quotas);
        assert!(config.enable_caching);
        assert!(config.enable_monitoring);
        assert_eq!(config.max_concurrent_operations, 10);
    }

    #[test]
    fn test_storage_service_config_validation() {
        let config = StorageServiceConfig::default();
        assert!(config.validate().is_ok());

        let mut invalid_config = config;
        invalid_config.discovery_interval = 0;
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn zfs_config_validate_errors_on_empty_zpool_and_zero_timeout() {
        let mut c = ZfsConfig::default();
        c.zpool_binary = String::new();
        assert!(c.validate().is_err());

        let mut c2 = ZfsConfig::default();
        c2.command_timeout = std::time::Duration::from_secs(0);
        assert!(c2.validate().is_err());
    }

    #[test]
    fn storage_service_config_interval_durations() {
        let c = StorageServiceConfig::default();
        assert_eq!(
            c.discovery_interval_duration(),
            std::time::Duration::from_secs(c.discovery_interval)
        );
        assert_eq!(
            c.quota_check_interval_duration(),
            std::time::Duration::from_secs(c.quota_check_interval)
        );
        assert_eq!(
            c.monitoring_interval_duration(),
            std::time::Duration::from_secs(c.monitoring_interval)
        );
        assert_eq!(
            c.operation_timeout_duration(),
            std::time::Duration::from_secs(c.operation_timeout)
        );
    }

    #[test]
    fn storage_service_config_validate_zero_intervals_rejected() {
        let mut c = StorageServiceConfig::default();
        c.quota_check_interval = 0;
        assert!(c.validate().is_err());

        let mut c = StorageServiceConfig::default();
        c.monitoring_interval = 0;
        assert!(c.validate().is_err());

        let mut c = StorageServiceConfig::default();
        c.operation_timeout = 0;
        assert!(c.validate().is_err());

        let mut c = StorageServiceConfig::default();
        c.max_concurrent_operations = 0;
        assert!(c.validate().is_err());
    }

    #[test]
    fn storage_service_config_production_and_development_constructors() {
        let dev = StorageServiceConfig::development();
        assert!(dev.discovery_interval < StorageServiceConfig::default().discovery_interval);

        let prod = StorageServiceConfig::production();
        assert!(
            prod.max_concurrent_operations
                >= StorageServiceConfig::default().max_concurrent_operations
        );
    }

    #[test]
    fn cache_policies_and_eviction_policy_serde_roundtrip() {
        let p = CachePolicies {
            eviction_strategy: EvictionPolicy::Arc,
            compression: true,
            deduplication: true,
        };
        let json = serde_json::to_string(&p).unwrap();
        let back: CachePolicies = serde_json::from_str(&json).unwrap();
        assert_eq!(back.eviction_strategy, EvictionPolicy::Arc);
        assert!(back.compression);
    }
}
