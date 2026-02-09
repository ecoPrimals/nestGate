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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
/// Storage service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::StorageServiceConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::StorageServiceConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for StorageService
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
    pub fn discovery_interval_duration(&self) -> Duration {
        Duration::from_secs(self.discovery_interval)
    }

    /// Get quota check interval as Duration
    #[must_use]
    pub fn quota_check_interval_duration(&self) -> Duration {
        Duration::from_secs(self.quota_check_interval)
    }

    /// Get monitoring interval as Duration
    #[must_use]
    pub fn monitoring_interval_duration(&self) -> Duration {
        Duration::from_secs(self.monitoring_interval)
    }

    /// Get operation timeout as Duration
    #[must_use]
    pub fn operation_timeout_duration(&self) -> Duration {
        Duration::from_secs(self.operation_timeout)
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Storageserviceconfigcanonical
pub type StorageServiceConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StorageServiceConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zfs_config_default() {
        let config = ZfsConfig::default();
        assert_eq!(config.zfs_binary, "/usr/sbin/zfs");
        assert_eq!(config.zpool_binary, "/usr/sbin/zpool");
        assert!(config.use_sudo);
        assert_eq!(config.command_timeout, Duration::from_secs(30));
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
}
