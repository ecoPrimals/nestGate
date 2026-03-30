// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Storage paths, databases, Redis, and ZFS defaults.

use std::sync::{Arc, OnceLock};

use super::defaults::{env_or, env_or_parse};

/// Storage and database configuration constants
#[derive(Debug, Clone)]
/// Storageconstants
pub struct StorageConstants {
    // Database
    postgres_host: String,
    postgres_port: u16,
    postgres_database: String,
    postgres_max_connections: u32,

    // Redis
    redis_host: String,
    redis_port: u16,
    redis_max_connections: u32,

    // ZFS
    zfs_pool_name: String,
    zfs_dataset_prefix: String,
    zfs_compression: String,
    zfs_dedup: bool,

    // Storage paths
    data_dir: String,
    cache_dir: String,
    log_dir: String,
}

impl Default for StorageConstants {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            // PostgreSQL
            postgres_host: env_or("NESTGATE_POSTGRES_HOST", "127.0.0.1"),
            postgres_port: env_or_parse("NESTGATE_POSTGRES_PORT", 5432),
            postgres_database: env_or("NESTGATE_POSTGRES_DB", "nestgate"),
            postgres_max_connections: env_or_parse("NESTGATE_POSTGRES_MAX_CONN", 100),

            // Redis
            redis_host: env_or("NESTGATE_REDIS_HOST", "127.0.0.1"),
            redis_port: env_or_parse("NESTGATE_REDIS_PORT", 6379),
            redis_max_connections: env_or_parse("NESTGATE_REDIS_MAX_CONN", 50),

            // ZFS
            zfs_pool_name: env_or("NESTGATE_ZFS_POOL", "nestgate_pool"),
            zfs_dataset_prefix: env_or("NESTGATE_ZFS_PREFIX", "nestgate"),
            zfs_compression: env_or("NESTGATE_ZFS_COMPRESSION", "lz4"),
            zfs_dedup: env_or_parse("NESTGATE_ZFS_DEDUP", false),

            // Paths
            data_dir: env_or("NESTGATE_DATA_DIR", "./data"),
            cache_dir: env_or("NESTGATE_CACHE_DIR", "./cache"),
            log_dir: env_or("NESTGATE_LOG_DIR", "./logs"),
        }
    }
}

impl StorageConstants {
    /// Get or initialize the global storage constants
    pub fn get() -> Arc<Self> {
        static INSTANCE: OnceLock<Arc<StorageConstants>> = OnceLock::new();
        INSTANCE.get_or_init(|| Arc::new(Self::default())).clone()
    }

    // Database getters

    /// Returns the `PostgreSQL` host address
    #[must_use]
    pub fn postgres_host(&self) -> &str {
        &self.postgres_host
    }
    /// Postgres Port
    #[must_use]
    pub const fn postgres_port(&self) -> u16 {
        self.postgres_port
    }
    /// Postgres Database
    #[must_use]
    pub fn postgres_database(&self) -> &str {
        &self.postgres_database
    }
    /// Postgres Max Connections
    #[must_use]
    pub const fn postgres_max_connections(&self) -> u32 {
        self.postgres_max_connections
    }
    /// Postgres Url
    #[must_use]
    pub fn postgres_url(&self) -> String {
        format!(
            "postgresql://{}:{}/{}",
            self.postgres_host, self.postgres_port, self.postgres_database
        )
    }

    // Redis getters

    /// Returns the Redis host address
    #[must_use]
    pub fn redis_host(&self) -> &str {
        &self.redis_host
    }
    /// Redis Port
    #[must_use]
    pub const fn redis_port(&self) -> u16 {
        self.redis_port
    }
    /// Redis Max Connections
    #[must_use]
    pub const fn redis_max_connections(&self) -> u32 {
        self.redis_max_connections
    }
    /// Redis Url
    #[must_use]
    pub fn redis_url(&self) -> String {
        format!("redis://{}:{}", self.redis_host, self.redis_port)
    }

    // ZFS getters

    /// Returns the ZFS pool name
    #[must_use]
    pub fn zfs_pool_name(&self) -> &str {
        &self.zfs_pool_name
    }
    /// Zfs Dataset Prefix
    #[must_use]
    pub fn zfs_dataset_prefix(&self) -> &str {
        &self.zfs_dataset_prefix
    }
    /// Zfs Compression
    #[must_use]
    pub fn zfs_compression(&self) -> &str {
        &self.zfs_compression
    }
    /// Zfs Dedup
    #[must_use]
    pub const fn zfs_dedup(&self) -> bool {
        self.zfs_dedup
    }

    // Path getters

    /// Returns the data directory path
    #[must_use]
    pub fn data_dir(&self) -> &str {
        &self.data_dir
    }
    /// Cache Dir
    #[must_use]
    pub fn cache_dir(&self) -> &str {
        &self.cache_dir
    }
    /// Log Dir
    #[must_use]
    pub fn log_dir(&self) -> &str {
        &self.log_dir
    }
}
