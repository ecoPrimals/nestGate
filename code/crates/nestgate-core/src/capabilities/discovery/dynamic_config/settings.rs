//
// Core configuration structures for the unified dynamic discovery system.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **UNIFIED DYNAMIC DISCOVERY EXTENSIONS**
/// Consolidates all dynamic discovery configuration patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedDynamicDiscoveryExtensions {
    /// Timeout discovery settings
    pub timeout: TimeoutDiscoverySettings,
    /// Network discovery settings  
    pub network: NetworkDiscoverySettings,
    /// Security discovery settings
    pub security: SecurityDiscoverySettings,
    /// Environment discovery settings
    /// Storage discovery settings
    pub storage: StorageDiscoverySettings,
    /// Cache discovery settings
    pub cache: CacheDiscoverySettings,
}
/// Timeout discovery configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutDiscoverySettings {
    /// Enable dynamic timeout discovery
    pub enable_dynamic_timeouts: bool,
    /// Timeout cache TTL
    pub cache_ttl: Duration,
    /// Maximum timeout value
    pub max_timeout: Duration,
    /// Default timeout fallback
    pub default_timeout: Duration,
}
/// Network discovery configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDiscoverySettings {
    /// Enable dynamic network discovery
    pub enable_network_discovery: bool,
    /// Network scan interval
    pub scan_interval: Duration,
    /// Maximum concurrent network scans
    pub max_concurrent_scans: usize,
    /// Network timeout
    pub network_timeout: Duration,
}
/// Security discovery configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityDiscoverySettings {
    /// Enable dynamic security discovery
    pub enable_security_discovery: bool,
    /// Certificate discovery settings
    pub certificate_discovery: CertificateDiscoverySettings,
    /// Security scan interval
    pub scan_interval: Duration,
}
/// Environment discovery configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentDiscoverySettings {
    /// Enable dynamic environment discovery
    pub enable_environment_discovery: bool,
    /// Environment refresh interval
    pub refresh_interval: Duration,
    /// Maximum environment variables to track
    pub max_env_vars: usize,
}
/// Storage discovery configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDiscoverySettings {
    /// Enable dynamic storage discovery
    pub enable_storage_discovery: bool,
    /// Storage capacity thresholds
    pub capacity_thresholds: StorageCapacityThresholds,
    /// Storage scan interval
    pub scan_interval: Duration,
}
/// Cache discovery configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheDiscoverySettings {
    /// Enable dynamic cache discovery
    pub enable_cache_discovery: bool,
    /// Cache size limits
    pub max_cache_size: usize,
    /// Cache TTL
    pub default_ttl: Duration,
}
/// Certificate discovery configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateDiscoverySettings {
    /// Enable certificate discovery
    pub enabled: bool,
    /// Certificate paths to scan
    pub cert_paths: Vec<String>,
    /// Certificate renewal threshold (days)
    pub renewal_threshold_days: u32,
}
/// Storage capacity threshold settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapacityThresholds {
    /// Warning threshold percentage
    pub warning_threshold: f64,
    /// Critical threshold percentage
    pub critical_threshold: f64,
}
impl Default for UnifiedDynamicDiscoveryExtensions {
    fn default() -> Self {
        Self {
            timeout: TimeoutDiscoverySettings::default(),
            network: NetworkDiscoverySettings::default(),
            security: SecurityDiscoverySettings::default(),
            storage: StorageDiscoverySettings::default(),
            cache: CacheDiscoverySettings::default(),
        }
    }
}

impl Default for TimeoutDiscoverySettings {
    fn default() -> Self {
        Self {
            enable_dynamic_timeouts: true,
            cache_ttl: Duration::from_secs(300),
            max_timeout: Duration::from_secs(60),
            default_timeout: Duration::from_secs(30),
        }
    }
}

impl Default for NetworkDiscoverySettings {
    fn default() -> Self {
        Self {
            enable_network_discovery: true,
            scan_interval: Duration::from_secs(60),
            max_concurrent_scans: 10,
            network_timeout: Duration::from_secs(5),
        }
    }
}

impl Default for SecurityDiscoverySettings {
    fn default() -> Self {
        Self {
            enable_security_discovery: true,
            certificate_discovery: CertificateDiscoverySettings::default(),
            scan_interval: Duration::from_secs(300),
        }
    }
}

impl Default for EnvironmentDiscoverySettings {
    fn default() -> Self {
        Self {
            enable_environment_discovery: true,
            refresh_interval: Duration::from_secs(30),
            max_env_vars: 1000,
        }
    }
}

impl Default for StorageDiscoverySettings {
    fn default() -> Self {
        Self {
            enable_storage_discovery: true,
            capacity_thresholds: StorageCapacityThresholds::default(),
            scan_interval: Duration::from_secs(60),
        }
    }
}

impl Default for CacheDiscoverySettings {
    fn default() -> Self {
        Self {
            enable_cache_discovery: true,
            max_cache_size: 10000,
            default_ttl: Duration::from_secs(300),
        }
    }
}

impl Default for CertificateDiscoverySettings {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_paths: vec!["/etc/ssl/certs".to_string()],
            renewal_threshold_days: 30,
        }
    }
}

impl Default for StorageCapacityThresholds {
    fn default() -> Self {
        Self {
            warning_threshold: 80.0,
            critical_threshold: 95.0,
        }
    }
} 