// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! [`StoragePaths`] aggregate and derived path helpers.

use super::resolve::{
    resolve_cache_dir, resolve_config_dir, resolve_data_dir, resolve_log_dir, resolve_runtime_dir,
    resolve_state_dir, resolve_temp_dir,
};
use std::env;
use std::path::{Path, PathBuf};
use tracing::info;

/// XDG-compliant storage path configuration
#[derive(Debug, Clone)]
pub struct StoragePaths {
    data_dir: PathBuf,
    config_dir: PathBuf,
    cache_dir: PathBuf,
    state_dir: PathBuf,
    log_dir: PathBuf,
    temp_dir: PathBuf,
    runtime_dir: PathBuf,
}

impl Default for StoragePaths {
    fn default() -> Self {
        Self::from_environment()
    }
}

impl StoragePaths {
    /// Create storage paths from environment with XDG-compliant fallback
    #[must_use]
    pub fn from_environment() -> Self {
        let data_dir = resolve_data_dir();
        let config_dir = resolve_config_dir();
        let cache_dir = resolve_cache_dir();
        let state_dir = resolve_state_dir();
        let log_dir = resolve_log_dir();
        let temp_dir = resolve_temp_dir();
        let runtime_dir = resolve_runtime_dir();

        info!("📂 Storage paths initialized (XDG-compliant):");
        info!("   Data:    {}", data_dir.display());
        info!("   Config:  {}", config_dir.display());
        info!("   Cache:   {}", cache_dir.display());
        info!("   Logs:    {}", log_dir.display());
        info!("   Temp:    {}", temp_dir.display());
        info!("   Runtime: {}", runtime_dir.display());

        Self {
            data_dir,
            config_dir,
            cache_dir,
            state_dir,
            log_dir,
            temp_dir,
            runtime_dir,
        }
    }

    /// Get data directory path (persistent application data)
    #[must_use]
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// Get configuration directory path
    #[must_use]
    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    /// Get cache directory path (ephemeral, can be cleared)
    #[must_use]
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    /// Get state directory path (application state data)
    #[must_use]
    pub fn state_dir(&self) -> &Path {
        &self.state_dir
    }

    /// Get log directory path
    #[must_use]
    pub fn log_dir(&self) -> &Path {
        &self.log_dir
    }

    /// Get temporary directory path (ephemeral)
    #[must_use]
    pub fn temp_dir(&self) -> &Path {
        &self.temp_dir
    }

    /// Get runtime directory path (sockets, PIDs, ephemeral runtime data)
    #[must_use]
    pub fn runtime_dir(&self) -> &Path {
        &self.runtime_dir
    }

    /// Get storage base path (datasets, objects)
    #[must_use]
    pub fn storage_base_path(&self) -> PathBuf {
        self.data_dir.join("storage")
    }

    /// Get ZFS binary path with environment override
    #[must_use]
    pub fn zfs_binary_path(&self) -> PathBuf {
        env::var("NESTGATE_ZFS_BINARY")
            .map_or_else(|_| PathBuf::from("/usr/sbin/zfs"), PathBuf::from)
    }

    /// Get zpool binary path with environment override
    #[must_use]
    pub fn zpool_binary_path(&self) -> PathBuf {
        env::var("NESTGATE_ZPOOL_BINARY")
            .map_or_else(|_| PathBuf::from("/usr/sbin/zpool"), PathBuf::from)
    }

    /// Get PID file path
    #[must_use]
    pub fn pid_file_path(&self) -> PathBuf {
        self.runtime_dir.join("nestgate.pid")
    }

    /// Get lock file path
    #[must_use]
    pub fn lock_file_path(&self) -> PathBuf {
        self.runtime_dir.join("nestgate.lock")
    }

    /// Get database path (for embedded databases)
    #[must_use]
    pub fn database_dir(&self) -> PathBuf {
        self.data_dir.join("db")
    }

    /// Get backup directory path
    #[must_use]
    pub fn backup_dir(&self) -> PathBuf {
        self.data_dir.join("backups")
    }

    /// Log summary of resolved paths
    pub fn log_summary(&self) {
        info!("═══════════════════════════════════════════════════════════");
        info!("📂 STORAGE PATHS CONFIGURATION (XDG-Compliant)");
        info!("═══════════════════════════════════════════════════════════");
        info!("Data Dir:     {}", self.data_dir.display());
        info!("Config Dir:   {}", self.config_dir.display());
        info!("Cache Dir:    {}", self.cache_dir.display());
        info!("State Dir:    {}", self.state_dir.display());
        info!("Log Dir:      {}", self.log_dir.display());
        info!("Temp Dir:     {}", self.temp_dir.display());
        info!("Runtime Dir:  {}", self.runtime_dir.display());
        info!("───────────────────────────────────────────────────────────");
        info!("Storage Base: {}", self.storage_base_path().display());
        info!("Database:     {}", self.database_dir().display());
        info!("Backups:      {}", self.backup_dir().display());
        info!("PID File:     {}", self.pid_file_path().display());
        info!("═══════════════════════════════════════════════════════════");
    }
}
