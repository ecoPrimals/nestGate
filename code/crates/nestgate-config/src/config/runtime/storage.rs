// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage configuration module
//!
//! Provides configuration for storage backends, paths, quotas, and retention policies.

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

/// Storage configuration for backend management.
///
/// Controls storage paths, backend selection, quotas, and retention policies.
///
/// # Environment Variables
///
/// - `NESTGATE_STORAGE_PATH` - Base storage path (default: "./data")
/// - `NESTGATE_STORAGE_BACKEND` - Backend type (default: "filesystem")
/// - `NESTGATE_STORAGE_QUOTA_GB` - Storage quota in GB (default: 100)
/// - `NESTGATE_STORAGE_RETENTION_DAYS` - Data retention days (default: 30)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Base storage path
    pub base_path: PathBuf,

    /// Storage backend (filesystem, s3, etc.)
    pub backend: String,

    /// Storage quota in GB
    pub quota_gb: u64,

    /// Data retention in days
    pub retention_days: u32,
}

impl StorageConfig {
    /// Load storage configuration from environment variables.
    pub fn from_environment() -> Result<Self> {
        Ok(Self {
            base_path: env::var("NESTGATE_STORAGE_PATH")
                .unwrap_or_else(|_| "./data".into())
                .into(),
            backend: env::var("NESTGATE_STORAGE_BACKEND")
                .unwrap_or_else(|_| "filesystem".into()),
            quota_gb: env::var("NESTGATE_STORAGE_QUOTA_GB")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            retention_days: env::var("NESTGATE_STORAGE_RETENTION_DAYS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
        })
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            base_path: PathBuf::from("./data"),
            backend: "filesystem".into(),
            quota_gb: 100,
            retention_days: 30,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn default_storage_config_values() {
        let cfg = StorageConfig::default();
        assert_eq!(cfg.base_path, PathBuf::from("./data"));
        assert_eq!(cfg.backend, "filesystem");
        assert_eq!(cfg.quota_gb, 100);
        assert_eq!(cfg.retention_days, 30);
    }

    #[test]
    #[serial]
    fn from_environment_uses_defaults_when_no_vars() {
        temp_env::with_vars(
            [
                ("NESTGATE_STORAGE_PATH", None::<&str>),
                ("NESTGATE_STORAGE_BACKEND", None::<&str>),
                ("NESTGATE_STORAGE_QUOTA_GB", None::<&str>),
                ("NESTGATE_STORAGE_RETENTION_DAYS", None::<&str>),
            ],
            || {
                let cfg = StorageConfig::from_environment().unwrap();
                assert_eq!(cfg.base_path, PathBuf::from("./data"));
                assert_eq!(cfg.backend, "filesystem");
                assert_eq!(cfg.quota_gb, 100);
                assert_eq!(cfg.retention_days, 30);
            },
        );
    }

    #[test]
    #[serial]
    fn from_environment_reads_all_env_vars() {
        temp_env::with_vars(
            [
                ("NESTGATE_STORAGE_PATH", Some("/mnt/zfs/nestgate")),
                ("NESTGATE_STORAGE_BACKEND", Some("zfs")),
                ("NESTGATE_STORAGE_QUOTA_GB", Some("500")),
                ("NESTGATE_STORAGE_RETENTION_DAYS", Some("90")),
            ],
            || {
                let cfg = StorageConfig::from_environment().unwrap();
                assert_eq!(cfg.base_path, PathBuf::from("/mnt/zfs/nestgate"));
                assert_eq!(cfg.backend, "zfs");
                assert_eq!(cfg.quota_gb, 500);
                assert_eq!(cfg.retention_days, 90);
            },
        );
    }

    #[test]
    #[serial]
    fn from_environment_invalid_quota_uses_default() {
        temp_env::with_vars(
            [
                ("NESTGATE_STORAGE_QUOTA_GB", Some("not-a-number")),
                ("NESTGATE_STORAGE_RETENTION_DAYS", Some("bad")),
            ],
            || {
                let cfg = StorageConfig::from_environment().unwrap();
                assert_eq!(cfg.quota_gb, 100);
                assert_eq!(cfg.retention_days, 30);
            },
        );
    }

    #[test]
    fn serde_roundtrip() {
        let cfg = StorageConfig {
            base_path: PathBuf::from("/custom/path"),
            backend: "zfs".into(),
            quota_gb: 200,
            retention_days: 60,
        };
        let json = serde_json::to_string(&cfg).unwrap();
        let restored: StorageConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.base_path, cfg.base_path);
        assert_eq!(restored.backend, cfg.backend);
        assert_eq!(restored.quota_gb, cfg.quota_gb);
        assert_eq!(restored.retention_days, cfg.retention_days);
    }
}
