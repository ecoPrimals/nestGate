// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};
    use temp_env::with_vars;

    use super::super::resolve::{
        resolve_cache_dir, resolve_config_dir, resolve_data_dir, resolve_log_dir,
        resolve_runtime_dir, resolve_state_dir, resolve_temp_dir,
    };

    #[test]
    fn global_storage_paths_singleton_accessors() {
        let p = get_storage_paths();
        assert!(!p.data_dir().as_os_str().is_empty());
        assert_eq!(get_data_dir(), p.data_dir());
        assert_eq!(get_config_dir(), p.config_dir());
        assert_eq!(get_cache_dir(), p.cache_dir());
        assert_eq!(get_log_dir(), p.log_dir());
        assert_eq!(get_temp_dir(), p.temp_dir());
        assert_eq!(get_runtime_dir(), p.runtime_dir());
        let base = get_storage_base_path();
        assert!(!base.as_os_str().is_empty());
    }

    #[test]
    fn resolve_data_dir_prefers_nestgate_env() {
        let want = "/tmp/nestgate-resolve-test-data";
        let got = with_vars(vec![("NESTGATE_DATA_DIR", Some(want))], resolve_data_dir);
        assert_eq!(got, PathBuf::from(want));
    }

    #[test]
    fn resolve_config_dir_uses_xdg_config_home() {
        let got = with_vars(
            vec![
                ("NESTGATE_CONFIG_DIR", None::<&str>),
                ("XDG_CONFIG_HOME", Some("/tmp/xdg-config")),
                ("HOME", None::<&str>),
            ],
            resolve_config_dir,
        );
        assert_eq!(got, PathBuf::from("/tmp/xdg-config/nestgate"));
    }

    #[test]
    fn resolve_cache_dir_from_home_dot_cache() {
        let got = with_vars(
            vec![
                ("NESTGATE_CACHE_DIR", None::<&str>),
                ("XDG_CACHE_HOME", None::<&str>),
                ("HOME", Some("/home/testuser")),
            ],
            resolve_cache_dir,
        );
        assert_eq!(got, PathBuf::from("/home/testuser/.cache/nestgate"));
    }

    #[test]
    fn resolve_state_dir_from_xdg_state_home() {
        let got = with_vars(
            vec![
                ("NESTGATE_STATE_DIR", None::<&str>),
                ("XDG_STATE_HOME", Some("/run/user/1000/state")),
                ("HOME", None::<&str>),
            ],
            resolve_state_dir,
        );
        assert_eq!(got, PathBuf::from("/run/user/1000/state/nestgate"));
    }

    #[test]
    fn resolve_log_dir_under_xdg_state() {
        let got = with_vars(
            vec![
                ("NESTGATE_LOG_DIR", None::<&str>),
                ("XDG_STATE_HOME", Some("/tmp/xdg-state")),
                ("HOME", None::<&str>),
            ],
            resolve_log_dir,
        );
        assert_eq!(got, PathBuf::from("/tmp/xdg-state/nestgate/logs"));
    }

    #[test]
    fn resolve_temp_dir_from_tmpdir() {
        let got = with_vars(
            vec![
                ("NESTGATE_TEMP_DIR", None::<&str>),
                ("TMPDIR", Some("/var/tmp")),
            ],
            resolve_temp_dir,
        );
        assert_eq!(got, PathBuf::from("/var/tmp/nestgate"));
    }

    #[test]
    fn resolve_runtime_dir_from_xdg_runtime() {
        let got = with_vars(
            vec![
                ("NESTGATE_RUNTIME_DIR", None::<&str>),
                ("XDG_RUNTIME_DIR", Some("/run/user/1000")),
                ("HOME", None::<&str>),
            ],
            resolve_runtime_dir,
        );
        assert_eq!(got, PathBuf::from("/run/user/1000/nestgate"));
    }

    #[test]
    fn storage_paths_from_environment_matches_resolve_helpers() {
        let data = "/tmp/nestgate-sp-data";
        let cfg = "/tmp/nestgate-sp-config";
        let paths = with_vars(
            vec![
                ("NESTGATE_DATA_DIR", Some(data)),
                ("NESTGATE_CONFIG_DIR", Some(cfg)),
                ("NESTGATE_CACHE_DIR", Some("/tmp/nestgate-sp-cache")),
                ("NESTGATE_STATE_DIR", Some("/tmp/nestgate-sp-state")),
                ("NESTGATE_LOG_DIR", Some("/tmp/nestgate-sp-logs")),
                ("NESTGATE_TEMP_DIR", Some("/tmp/nestgate-sp-temp")),
                ("NESTGATE_RUNTIME_DIR", Some("/tmp/nestgate-sp-run")),
            ],
            super::super::paths::StoragePaths::from_environment,
        );
        assert_eq!(paths.data_dir(), Path::new(data));
        assert_eq!(paths.config_dir(), Path::new(cfg));
        assert_eq!(
            paths.storage_base_path(),
            PathBuf::from(data).join("storage")
        );
        assert_eq!(
            paths.pid_file_path(),
            PathBuf::from("/tmp/nestgate-sp-run/nestgate.pid")
        );
        assert_eq!(paths.database_dir(), PathBuf::from(data).join("db"));
    }
}
