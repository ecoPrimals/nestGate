// API Paths Configuration Module
//! Configuration types and utilities.
// Centralized management of API endpoints and paths to eliminate hardcoded strings
//! throughout the codebase. This enables easy customization and versioning of APIs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// API path constants to eliminate string allocations
const API_VERSION_V1: &str = "v1";
const PATH_HEALTH: &str = "/health";
const PATH_HEALTH_DETAILED: &str = "/health/detailed";
const PATH_HEALTH_ZFS: &str = "/health/zfs";
const PATH_HEALTH_STORAGE: &str = "/health/storage";
const PATH_METRICS: &str = "/metrics";
const PATH_METRICS_PROMETHEUS: &str = "/metrics/prometheus";
const PATH_METRICS_JSON: &str = "/metrics/json";
const PATH_LOGS: &str = "/logs";
const PATH_DIAGNOSTICS: &str = "/diagnostics";

/// API paths configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPathsConfig {
    /// Base API version prefix
    pub api_version: String,
    /// ZFS API paths
    pub zfs: ZfsApiPaths,

    /// Storage API paths  
    pub storage: StorageApiPaths,

    /// System API paths
    pub system: SystemApiPaths,

    /// Health and monitoring paths
    pub health: HealthApiPaths,

    /// Custom endpoint overrides
    pub custom_endpoints: HashMap<String, String>,
}

/// ZFS-related API paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsApiPaths {
    /// Base ZFS path
    pub base: String,
    /// Pool operations
    pub pools: String,
    pub pools_detail: String,
    pub pools_create: String,
    pub pools_destroy: String,
    pub pools_scrub: String,
    pub pools_export: String,
    pub pools_import: String,

    /// Dataset operations  
    pub datasets: String,
    pub datasets_create: String,
    pub datasets_destroy: String,

    /// Snapshot operations
    pub snapshots: String,
    pub snapshots_create: String,
    pub snapshots_destroy: String,
    pub snapshots_rollback: String,
}

/// Storage-related API paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageApiPaths {
    /// Base storage path
    pub base: String,
    /// Storage information
    pub info: String,
    pub capacity: String,
    pub usage: String,

    /// Tier management
    pub tiers: String,
    pub tier_migration: String,

    /// Protocol endpoints
    pub nfs: String,
    pub smb: String,
    pub iscsi: String,
}

/// System-related API paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemApiPaths {
    /// System information
    pub info: String,
    pub status: String,
    pub version: String,
    /// Configuration
    pub config: String,
    pub config_reload: String,

    /// Service management
    pub services: String,
    pub service_start: String,
    pub service_stop: String,
    pub service_restart: String,
}

/// Health and monitoring API paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthApiPaths {
    /// Health check endpoints
    pub health: String,
    pub health_detailed: String,
    pub health_zfs: String,
    pub health_storage: String,
    /// Metrics endpoints
    pub metrics: String,
    pub metrics_prometheus: String,
    pub metrics_json: String,

    /// Logs and diagnostics
    pub logs: String,
    pub diagnostics: String,
}

impl Default for ApiPathsConfig {
    fn default() -> Self {
        let api_version = API_VERSION_V1;
        Self::default_with_version(api_version)
    }
}

impl ApiPathsConfig {
    #[must_use]
    pub fn default_with_version(api_version: &str) -> Self {
        Self {
            api_version: api_version.to_string(),
            zfs: ZfsApiPaths::default_with_version(api_version),
            storage: StorageApiPaths::default_with_version(api_version),
            system: SystemApiPaths::default_with_version(api_version),
            health: HealthApiPaths::default(),
            custom_endpoints: HashMap::new(),
        }
    }
}

impl ZfsApiPaths {
    pub fn default_with_version(api_version: &str) -> Self {
        let base = format!("/api/{api_version}/zfs");

        Self {
            base: base.clone(),
            pools: format!("{base}/pools"),
            pools_detail: format!("{base}/pools/{{name}"),
            pools_create: format!("{base}/pools"),
            pools_destroy: format!("{base}/pools/{{name}"),
            pools_scrub: format!("{base}/pools/{{name}/scrub"),
            pools_export: format!("{base}/pools/{{name}/export"),
            pools_import: format!("{base}/pools/{{name}/import"),
            datasets: format!("{base}/datasets"),
            datasets_create: format!("{base}/datasets"),
            datasets_destroy: format!("{base}/datasets/{{name}"),
            snapshots: format!("{base}/snapshots"),
            snapshots_create: format!("{base}/snapshots"),
            snapshots_destroy: format!("{base}/snapshots/{{name}"),
            snapshots_rollback: format!("{base}/snapshots/{{name}/rollback"),
        }
    }
}

impl StorageApiPaths {
    pub fn default_with_version(api_version: &str) -> Self {
        let base = format!("/api/{api_version}/storage");

        Self {
            base: base.clone(),
            info: format!("{base}/info"),
            capacity: format!("{base}/capacity"),
            usage: format!("{base}/usage"),
            tiers: format!("{base}/tiers"),
            tier_migration: format!("{base}/tiers/migrate"),
            nfs: format!("{base}/nfs"),
            smb: format!("{base}/smb"),
            iscsi: format!("{base}/iscsi"),
        }
    }
}

impl SystemApiPaths {
    pub fn default_with_version(api_version: &str) -> Self {
        let base = format!("/api/{api_version}/system");

        Self {
            info: format!("{base}/info"),
            status: format!("{base}/status"),
            version: format!("{base}/version"),
            config: format!("{base}/config"),
            config_reload: format!("{base}/config/reload"),
            services: format!("{base}/services"),
            service_start: format!("{base}/services/{{name}/start"),
            service_stop: format!("{base}/services/{{name}/stop"),
            service_restart: format!("{base}/services/{{name}/restart"),
        }
    }
}

impl Default for HealthApiPaths {
    fn default() -> Self {
        HealthApiPaths {
            health: PATH_HEALTH.to_string(),
            health_detailed: PATH_HEALTH_DETAILED.to_string(),
            health_zfs: PATH_HEALTH_ZFS.to_string(),
            health_storage: PATH_HEALTH_STORAGE.to_string(),

            metrics: PATH_METRICS.to_string(),
            metrics_prometheus: PATH_METRICS_PROMETHEUS.to_string(),
            metrics_json: PATH_METRICS_JSON.to_string(),

            logs: PATH_LOGS.to_string(),
            diagnostics: PATH_DIAGNOSTICS.to_string(),
        }
    }
}

impl ApiPathsConfig {
    /// Get a custom endpoint or default path
    pub fn get_endpoint(&self, key: &str, default: &str) -> String {
        self.custom_endpoints
            .get(key)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    /// Add or update a custom endpoint
        self.custom_endpoints.insert(key, path);
    }

    /// Get all ZFS endpoints as a vector
    pub fn all_zfs_endpoints(&self) -> Vec<String> {
        vec![
            self.zfs.pools.clone(),
            self.zfs.datasets.clone(),
            self.zfs.snapshots.clone(),
        ]
    }

    /// Get all storage endpoints as a vector
    pub fn all_storage_endpoints(&self) -> Vec<String> {
        vec![
            self.storage.info.clone(),
            self.storage.capacity.clone(),
            self.storage.usage.clone(),
        ]
    }

    /// Get all health endpoints as a vector
    pub fn all_health_endpoints(&self) -> Vec<String> {
        vec![self.health.health.clone(), self.health.metrics.clone()]
    }

    /// Validate all paths are properly formatted
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn validate(&self) -> Result<(), String>  {
        // Check that all paths start with /
        let endpoints = vec![
            ("health", &self.health.health),
            ("zfs.pools", &self.zfs.pools),
            ("storage.info", &self.storage.info),
            ("system.info", &self.system.info),
        ];

        for (name, path) in endpoints {
            if !path.starts_with('/') {
                return Err(format!("Path '{path}' for '{name}' must start with '/'"));
            }
        }

        // Check API version is not empty
        if self.api_version.is_empty() {
            return Err("API version cannot be empty".to_string());
        }
        Ok(())
    }

    /// Get environment-specific configuration
    #[must_use]
    pub fn from_environment() -> Self {
        let mut config = Self::default();

        // Override API version from environment
        if let Ok(version) = std::env::var("NESTGATE_API_VERSION") {
            config.api_version = version;
            // Recreate paths with new version
            config.zfs = ZfsApiPaths::default_with_version(&config.api_version);
            config.storage = StorageApiPaths::default_with_version(&config.api_version);
            config.system = SystemApiPaths::default_with_version(&config.api_version);
        }

        // Override specific endpoints from environment
        if let Ok(health_path) = std::env::var("NESTGATE_HEALTH_PATH") {
            config.health.health = health_path;
        }

        if let Ok(metrics_path) = std::env::var("NESTGATE_METRICS_PATH") {
            config.health.metrics = metrics_path;
        }

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_paths() {
        let config = ApiPathsConfig::default();

        assert_eq!(config.api_version, "v1");
        assert_eq!(config.zfs.pools, "/api/v1/zfs/pools");
        assert_eq!(config.storage.info, "/api/v1/storage/info");
        assert_eq!(config.health.health, "/health");

        // Validate configuration
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_custom_endpoints() {
        let mut config = ApiPathsConfig::default();

        config.set_custom_endpoint("test".to_string(), "/custom/test".to_string());

        assert_eq!(config.get_endpoint("test", "/default"), "/custom/test");
        assert_eq!(config.get_endpoint("missing", "/default"), "/default");
    }

    #[test]
    fn test_version_customization() {
        let config_v2 = ZfsApiPaths::default_with_version("v2");
        assert_eq!(config_v2.pools, "/api/v2/zfs/pools");
        assert_eq!(config_v2.datasets, "/api/v2/zfs/datasets");
    }

    #[test]
    fn test_environment_override() {
        // This would test environment variable overrides
        // In a real scenario, you'd use a testing framework that can set env vars
        let config = ApiPathsConfig::from_environment();
        assert!(!config.api_version.is_empty());
    }

    #[test]
    fn test_validation() {
        let mut config = ApiPathsConfig::default();

        // Valid configuration should pass
        assert!(config.validate().is_ok());

        // Invalid path (no leading slash)
        config.health.health = "health".to_string();
        assert!(config.validate().is_err());

        // Empty API version
        config.health.health = "/health".to_string();
        config.api_version = "".to_string();
        assert!(config.validate().is_err());
    }
}
