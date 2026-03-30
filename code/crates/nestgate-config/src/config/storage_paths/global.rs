// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Global singleton and thin convenience accessors.

use super::paths::StoragePaths;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static STORAGE_PATHS: OnceLock<StoragePaths> = OnceLock::new();

/// Get or initialize the global storage paths configuration
#[must_use]
pub fn get_storage_paths() -> &'static StoragePaths {
    STORAGE_PATHS.get_or_init(StoragePaths::from_environment)
}

/// Get data directory path (convenience function)
#[must_use]
pub fn get_data_dir() -> &'static Path {
    get_storage_paths().data_dir()
}

/// Get config directory path (convenience function)
#[must_use]
pub fn get_config_dir() -> &'static Path {
    get_storage_paths().config_dir()
}

/// Get cache directory path (convenience function)
#[must_use]
pub fn get_cache_dir() -> &'static Path {
    get_storage_paths().cache_dir()
}

/// Get log directory path (convenience function)
#[must_use]
pub fn get_log_dir() -> &'static Path {
    get_storage_paths().log_dir()
}

/// Get temp directory path (convenience function)
#[must_use]
pub fn get_temp_dir() -> &'static Path {
    get_storage_paths().temp_dir()
}

/// Get runtime directory path (convenience function)
#[must_use]
pub fn get_runtime_dir() -> &'static Path {
    get_storage_paths().runtime_dir()
}

/// Get storage base path (convenience function)
#[must_use]
pub fn get_storage_base_path() -> PathBuf {
    get_storage_paths().storage_base_path()
}
