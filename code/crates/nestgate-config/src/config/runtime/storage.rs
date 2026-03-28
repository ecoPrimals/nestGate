// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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
                .unwrap_or_else(|_| "./data".to_string())
                .into(),
            backend: env::var("NESTGATE_STORAGE_BACKEND")
                .unwrap_or_else(|_| "filesystem".to_string()),
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
            backend: "filesystem".to_string(),
            quota_gb: 100,
            retention_days: 30,
        }
    }
}
