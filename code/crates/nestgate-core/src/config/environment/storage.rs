//! # Storage Configuration
//!
//! Storage-specific environment configuration extracted for logical cohesion.
//!
//! **Phase 3: Smart Refactoring** - Extracted from monolithic `environment.rs` (Jan 30, 2026)

use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;

use super::ConfigError;

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// ZFS pool name (default: tank)
    pub zfs_pool: String,

    /// Data directory path (default: XDG-compliant or /var/lib/nestgate)
    pub data_dir: String,

    /// Cache size in megabytes (default: 512)
    pub cache_size_mb: usize,

    /// Enable compression (default: true)
    pub compression_enabled: bool,

    /// Snapshot retention days (default: 30)
    pub snapshot_retention_days: u32,
}

impl StorageConfig {
    /// Load from environment with NESTGATE_ prefix
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_env_with_prefix("NESTGATE")
    }

    /// Load from environment with custom prefix
    pub fn from_env_with_prefix(prefix: &str) -> Result<Self, ConfigError> {
        Ok(Self {
            zfs_pool: Self::env_var_or(prefix, "ZFS_POOL", "tank".to_string())?,
            data_dir: Self::env_var_or(
                prefix,
                "DATA_DIR",
                crate::config::storage_paths::StoragePaths::from_environment()
                    .data_dir()
                    .to_string_lossy()
                    .to_string(),
            )?,
            cache_size_mb: Self::env_var_or(prefix, "CACHE_SIZE_MB", 512)?,
            compression_enabled: Self::env_var_or(prefix, "COMPRESSION", true)?,
            snapshot_retention_days: Self::env_var_or(prefix, "SNAPSHOT_RETENTION_DAYS", 30)?,
        })
    }

    /// Helper to get environment variable or use default
    fn env_var_or<T: FromStr>(prefix: &str, key: &str, default: T) -> Result<T, ConfigError>
    where
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        let var_name = format!("{}_{}", prefix, key);
        match env::var(&var_name) {
            Ok(val) => val.parse().map_err(|e| ConfigError::ParseError {
                key: var_name,
                source: Box::new(e),
            }),
            Err(_) => Ok(default),
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            zfs_pool: "tank".to_string(),
            data_dir: crate::config::storage_paths::StoragePaths::from_environment()
                .data_dir()
                .to_string_lossy()
                .to_string(),
            cache_size_mb: 512,
            compression_enabled: true,
            snapshot_retention_days: 30,
        }
    }
}
