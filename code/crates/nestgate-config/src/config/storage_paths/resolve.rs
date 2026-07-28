// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Environment and XDG fallback resolution for each path kind.
//!
//! Resolution order (same on all platforms):
//! 1. `NESTGATE_*` explicit env var from the injectable [`EnvSource`]
//! 2. `XDG_*` standard from the injectable [`EnvSource`]
//! 3. User home directory (`HOME`/`USERPROFILE`) from the injectable [`EnvSource`]
//! 4. [`etcetera::base_strategy::choose_base_strategy`] for XDG-compliant defaults (reads real process env)
//! 5. Platform-specific system fallback (FHS on Linux, Application Support on macOS, `%APPDATA%` on Windows)
//!
//! The injected [`EnvSource`] always takes priority over `etcetera` (which reads
//! the real process environment). This ensures test isolation via [`nestgate_types::MapEnv`]
//! and correct behavior in containerized or namespaced environments where the
//! caller provides a custom env source.

use etcetera::BaseStrategy;
use nestgate_types::EnvSource;
use std::path::PathBuf;
use tracing::{debug, warn};

/// Resolve user home directory from env, supporting both Unix (`HOME`) and
/// Windows (`USERPROFILE`).
fn resolve_home(env: &(impl EnvSource + ?Sized)) -> Option<PathBuf> {
    env.get("HOME")
        .or_else(|| env.get("USERPROFILE"))
        .map(PathBuf::from)
}

/// XDG-compliant path via `etcetera` when the process environment allows it.
fn etcetera_data_dir() -> Option<PathBuf> {
    etcetera::base_strategy::choose_base_strategy()
        .ok()
        .map(|strategy| strategy.data_dir().join("nestgate"))
}

fn etcetera_config_dir() -> Option<PathBuf> {
    etcetera::base_strategy::choose_base_strategy()
        .ok()
        .map(|strategy| strategy.config_dir().join("nestgate"))
}

fn etcetera_cache_dir() -> Option<PathBuf> {
    etcetera::base_strategy::choose_base_strategy()
        .ok()
        .map(|strategy| strategy.cache_dir().join("nestgate"))
}

fn etcetera_state_dir() -> Option<PathBuf> {
    etcetera::base_strategy::choose_base_strategy()
        .ok()
        .and_then(|strategy| strategy.state_dir())
        .map(|dir| dir.join("nestgate"))
}

/// Platform-appropriate system fallback for data storage.
fn system_data_fallback() -> PathBuf {
    #[cfg(windows)]
    {
        std::env::var("APPDATA")
            .map(|p| PathBuf::from(p).join("nestgate"))
            .or_else(|_| std::env::var("ProgramData").map(|p| PathBuf::from(p).join("nestgate")))
            .unwrap_or_else(|_| PathBuf::from(r"C:\ProgramData\nestgate"))
    }
    #[cfg(target_os = "macos")]
    {
        resolve_home(&nestgate_types::ProcessEnv)
            .map(|home| home.join("Library/Application Support/nestgate"))
            .unwrap_or_else(|| PathBuf::from("/Library/Application Support/nestgate"))
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        PathBuf::from("/var/lib/nestgate")
    }
}

/// Platform-appropriate system fallback for configuration.
fn system_config_fallback() -> PathBuf {
    #[cfg(windows)]
    {
        std::env::var("APPDATA")
            .map(|p| PathBuf::from(p).join("nestgate"))
            .or_else(|_| {
                std::env::var("ProgramData").map(|p| PathBuf::from(p).join("nestgate\\config"))
            })
            .unwrap_or_else(|_| PathBuf::from(r"C:\ProgramData\nestgate\config"))
    }
    #[cfg(target_os = "macos")]
    {
        resolve_home(&nestgate_types::ProcessEnv)
            .map(|home| home.join("Library/Application Support/nestgate"))
            .unwrap_or_else(|| PathBuf::from("/Library/Application Support/nestgate"))
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        PathBuf::from("/etc/nestgate")
    }
}

/// Platform-appropriate system fallback for cache.
fn system_cache_fallback() -> PathBuf {
    #[cfg(windows)]
    {
        std::env::var("LOCALAPPDATA")
            .map(|p| PathBuf::from(p).join("nestgate\\cache"))
            .or_else(|_| std::env::var("APPDATA").map(|p| PathBuf::from(p).join("nestgate\\cache")))
            .unwrap_or_else(|_| PathBuf::from(r"C:\ProgramData\nestgate\cache"))
    }
    #[cfg(target_os = "macos")]
    {
        resolve_home(&nestgate_types::ProcessEnv)
            .map(|home| home.join("Library/Caches/nestgate"))
            .unwrap_or_else(|| PathBuf::from("/Library/Caches/nestgate"))
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        PathBuf::from("/var/cache/nestgate")
    }
}

/// Platform-appropriate system fallback for state.
fn system_state_fallback() -> PathBuf {
    #[cfg(windows)]
    {
        std::env::var("APPDATA")
            .map(|p| PathBuf::from(p).join("nestgate\\state"))
            .or_else(|_| {
                std::env::var("ProgramData").map(|p| PathBuf::from(p).join("nestgate\\state"))
            })
            .unwrap_or_else(|_| PathBuf::from(r"C:\ProgramData\nestgate\state"))
    }
    #[cfg(target_os = "macos")]
    {
        resolve_home(&nestgate_types::ProcessEnv)
            .map(|home| home.join("Library/Application Support/nestgate/state"))
            .unwrap_or_else(|| PathBuf::from("/Library/Application Support/nestgate/state"))
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        PathBuf::from("/var/lib/nestgate/state")
    }
}

/// Platform-appropriate system fallback for logs.
fn system_log_fallback() -> PathBuf {
    #[cfg(windows)]
    {
        std::env::var("APPDATA")
            .map(|p| PathBuf::from(p).join("nestgate\\logs"))
            .or_else(|_| {
                std::env::var("ProgramData").map(|p| PathBuf::from(p).join("nestgate\\logs"))
            })
            .unwrap_or_else(|_| PathBuf::from(r"C:\ProgramData\nestgate\logs"))
    }
    #[cfg(target_os = "macos")]
    {
        resolve_home(&nestgate_types::ProcessEnv)
            .map(|home| home.join("Library/Logs/nestgate"))
            .unwrap_or_else(|| PathBuf::from("/Library/Logs/nestgate"))
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        PathBuf::from("/var/log/nestgate")
    }
}

/// Resolve data directory from an injectable [`EnvSource`] (e.g. [`nestgate_types::MapEnv`] in tests).
///
/// Resolution order:
/// 1. `NESTGATE_DATA_DIR` from `env`
/// 2. `XDG_DATA_HOME` from `env` + `/nestgate`
/// 3. `HOME`/`USERPROFILE` from `env` + `/.local/share/nestgate`
/// 4. `etcetera` XDG defaults (reads real process env — last-resort auto-detect)
/// 5. Platform system fallback (FHS on Linux, Application Support on macOS, `AppData` on Windows)
#[must_use]
pub fn resolve_data_dir_from_env_source(env: &(impl EnvSource + ?Sized)) -> PathBuf {
    if let Some(path) = env.get("NESTGATE_DATA_DIR") {
        debug!("Data dir from NESTGATE_DATA_DIR: {}", path);
        return PathBuf::from(path);
    }

    if let Some(xdg_data) = env.get("XDG_DATA_HOME") {
        let path = PathBuf::from(xdg_data).join("nestgate");
        debug!("Data dir from XDG_DATA_HOME: {}", path.display());
        return path;
    }

    if let Some(home) = resolve_home(env) {
        let path = home.join(".local").join("share").join("nestgate");
        debug!("Data dir from HOME: {}", path.display());
        return path;
    }

    if let Some(path) = etcetera_data_dir() {
        debug!("Data dir from etcetera: {}", path.display());
        return path;
    }

    let fallback = system_data_fallback();
    warn!(
        "Data dir using system fallback (requires permissions): {}",
        fallback.display()
    );
    fallback
}

/// Resolve config directory from an injectable [`EnvSource`].
#[must_use]
pub fn resolve_config_dir_from_env_source(env: &(impl EnvSource + ?Sized)) -> PathBuf {
    if let Some(path) = env.get("NESTGATE_CONFIG_DIR") {
        debug!("Config dir from NESTGATE_CONFIG_DIR: {}", path);
        return PathBuf::from(path);
    }

    if let Some(xdg_config) = env.get("XDG_CONFIG_HOME") {
        let path = PathBuf::from(xdg_config).join("nestgate");
        debug!("Config dir from XDG_CONFIG_HOME: {}", path.display());
        return path;
    }

    if let Some(home) = resolve_home(env) {
        let path = home.join(".config").join("nestgate");
        debug!("Config dir from HOME: {}", path.display());
        return path;
    }

    if let Some(path) = etcetera_config_dir() {
        debug!("Config dir from etcetera: {}", path.display());
        return path;
    }

    let fallback = system_config_fallback();
    warn!(
        "Config dir using system fallback (requires permissions): {}",
        fallback.display()
    );
    fallback
}

/// Resolve cache directory from an injectable [`EnvSource`].
#[must_use]
pub fn resolve_cache_dir_from_env_source(env: &(impl EnvSource + ?Sized)) -> PathBuf {
    if let Some(path) = env.get("NESTGATE_CACHE_DIR") {
        debug!("Cache dir from NESTGATE_CACHE_DIR: {}", path);
        return PathBuf::from(path);
    }

    if let Some(xdg_cache) = env.get("XDG_CACHE_HOME") {
        let path = PathBuf::from(xdg_cache).join("nestgate");
        debug!("Cache dir from XDG_CACHE_HOME: {}", path.display());
        return path;
    }

    if let Some(home) = resolve_home(env) {
        let path = home.join(".cache").join("nestgate");
        debug!("Cache dir from HOME: {}", path.display());
        return path;
    }

    if let Some(path) = etcetera_cache_dir() {
        debug!("Cache dir from etcetera: {}", path.display());
        return path;
    }

    let fallback = system_cache_fallback();
    warn!("Cache dir using system fallback: {}", fallback.display());
    fallback
}

/// Resolve state directory from an injectable [`EnvSource`].
#[must_use]
pub fn resolve_state_dir_from_env_source(env: &(impl EnvSource + ?Sized)) -> PathBuf {
    if let Some(path) = env.get("NESTGATE_STATE_DIR") {
        debug!("State dir from NESTGATE_STATE_DIR: {}", path);
        return PathBuf::from(path);
    }

    if let Some(xdg_state) = env.get("XDG_STATE_HOME") {
        let path = PathBuf::from(xdg_state).join("nestgate");
        debug!("State dir from XDG_STATE_HOME: {}", path.display());
        return path;
    }

    if let Some(home) = resolve_home(env) {
        let path = home.join(".local").join("state").join("nestgate");
        debug!("State dir from HOME: {}", path.display());
        return path;
    }

    if let Some(path) = etcetera_state_dir() {
        debug!("State dir from etcetera: {}", path.display());
        return path;
    }

    let fallback = system_state_fallback();
    warn!("State dir using system fallback: {}", fallback.display());
    fallback
}

/// Resolve log directory from an injectable [`EnvSource`].
#[must_use]
pub fn resolve_log_dir_from_env_source(env: &(impl EnvSource + ?Sized)) -> PathBuf {
    if let Some(path) = env.get("NESTGATE_LOG_DIR") {
        debug!("Log dir from NESTGATE_LOG_DIR: {}", path);
        return PathBuf::from(path);
    }

    if let Some(xdg_state) = env.get("XDG_STATE_HOME") {
        let path = PathBuf::from(xdg_state).join("nestgate/logs");
        debug!("Log dir from XDG_STATE_HOME: {}", path.display());
        return path;
    }

    if let Some(home) = resolve_home(env) {
        let path = home
            .join(".local")
            .join("state")
            .join("nestgate")
            .join("logs");
        debug!("Log dir from HOME: {}", path.display());
        return path;
    }

    if let Some(state_dir) = etcetera_state_dir() {
        let path = state_dir.join("logs");
        debug!("Log dir from etcetera state dir: {}", path.display());
        return path;
    }

    let fallback = system_log_fallback();
    warn!(
        "Log dir using system fallback (requires permissions): {}",
        fallback.display()
    );
    fallback
}

/// Resolve temporary directory from an injectable [`EnvSource`].
#[must_use]
pub fn resolve_temp_dir_from_env_source(env: &(impl EnvSource + ?Sized)) -> PathBuf {
    if let Some(path) = env.get("NESTGATE_TEMP_DIR") {
        debug!("Temp dir from NESTGATE_TEMP_DIR: {}", path);
        return PathBuf::from(path);
    }

    if let Some(tmpdir) = env.get("TMPDIR") {
        let path = PathBuf::from(tmpdir).join("nestgate");
        debug!("Temp dir from TMPDIR: {}", path.display());
        return path;
    }

    let fallback = std::env::temp_dir().join("nestgate");
    debug!("Temp dir using system fallback: {}", fallback.display());
    fallback
}

/// Resolve runtime directory from an injectable [`EnvSource`].
#[must_use]
pub fn resolve_runtime_dir_from_env_source(env: &(impl EnvSource + ?Sized)) -> PathBuf {
    if let Some(path) = env.get("NESTGATE_RUNTIME_DIR") {
        debug!("Runtime dir from NESTGATE_RUNTIME_DIR: {}", path);
        return PathBuf::from(path);
    }

    if let Some(xdg_runtime) = env.get("XDG_RUNTIME_DIR") {
        let path = PathBuf::from(xdg_runtime).join("nestgate");
        debug!("Runtime dir from XDG_RUNTIME_DIR: {}", path.display());
        return path;
    }

    let fallback = std::env::temp_dir().join("nestgate-runtime");
    warn!("Runtime dir using fallback: {}", fallback.display());
    fallback
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_types::MapEnv;

    #[test]
    fn nestgate_data_dir_wins_over_xdg() {
        let env = MapEnv::from([
            ("NESTGATE_DATA_DIR", "/explicit/data"),
            ("XDG_DATA_HOME", "/xdg/data"),
        ]);
        let p = resolve_data_dir_from_env_source(&env);
        assert_eq!(p, PathBuf::from("/explicit/data"));
    }

    #[test]
    fn xdg_data_home_used_when_nestgate_unset() {
        let env = MapEnv::from([("XDG_DATA_HOME", "/xdg/data")]);
        let p = resolve_data_dir_from_env_source(&env);
        assert_eq!(p, PathBuf::from("/xdg/data/nestgate"));
    }

    #[test]
    fn nestgate_runtime_dir_wins_over_xdg_runtime() {
        let env = MapEnv::from([
            ("NESTGATE_RUNTIME_DIR", "/run/explicit"),
            ("XDG_RUNTIME_DIR", "/run/user/1"),
        ]);
        let p = resolve_runtime_dir_from_env_source(&env);
        assert_eq!(p, PathBuf::from("/run/explicit"));
    }

    #[test]
    fn runtime_dir_joins_nestgate_under_xdg_runtime() {
        let env = MapEnv::from([("XDG_RUNTIME_DIR", "/run/user/99")]);
        let p = resolve_runtime_dir_from_env_source(&env);
        assert_eq!(p, PathBuf::from("/run/user/99/nestgate"));
    }

    #[test]
    fn empty_nestgate_data_dir_is_honored() {
        let env = MapEnv::from([("NESTGATE_DATA_DIR", "")]);
        let p = resolve_data_dir_from_env_source(&env);
        assert_eq!(p, PathBuf::from(""));
    }

    #[test]
    fn log_dir_prefers_explicit_over_xdg_state() {
        let env = MapEnv::from([
            ("NESTGATE_LOG_DIR", "/logs/here"),
            ("XDG_STATE_HOME", "/state"),
        ]);
        let p = resolve_log_dir_from_env_source(&env);
        assert_eq!(p, PathBuf::from("/logs/here"));
    }

    #[test]
    fn temp_dir_fallback_uses_system_temp_dir() {
        let env = MapEnv::new();
        let p = resolve_temp_dir_from_env_source(&env);
        assert!(
            p.starts_with(std::env::temp_dir()),
            "fallback temp dir {p:?} should be under system temp dir"
        );
        assert!(p.ends_with("nestgate"));
    }

    #[test]
    fn temp_dir_prefers_nestgate_env_var() {
        let env = MapEnv::from([("NESTGATE_TEMP_DIR", "/opt/nestgate/tmp")]);
        let p = resolve_temp_dir_from_env_source(&env);
        assert_eq!(p, PathBuf::from("/opt/nestgate/tmp"));
    }

    #[test]
    fn temp_dir_uses_tmpdir_env_when_nestgate_unset() {
        let env = MapEnv::from([("TMPDIR", "/custom/tmp")]);
        let p = resolve_temp_dir_from_env_source(&env);
        assert_eq!(p, PathBuf::from("/custom/tmp/nestgate"));
    }

    #[test]
    fn runtime_dir_fallback_uses_system_temp_dir() {
        let env = MapEnv::new();
        let p = resolve_runtime_dir_from_env_source(&env);
        assert!(
            p.starts_with(std::env::temp_dir()),
            "fallback runtime dir {p:?} should be under system temp dir"
        );
        assert!(p.ends_with("nestgate-runtime"));
    }

    #[test]
    fn no_hardcoded_tmp_in_temp_fallback() {
        let env = MapEnv::new();
        let temp = resolve_temp_dir_from_env_source(&env);
        let runtime = resolve_runtime_dir_from_env_source(&env);
        let temp_str = temp.to_string_lossy();
        let runtime_str = runtime.to_string_lossy();
        assert!(
            !temp_str.contains("/tmp/")
                || temp_str.starts_with(&*std::env::temp_dir().to_string_lossy()),
            "temp path should use std::env::temp_dir(), not hardcoded /tmp"
        );
        assert!(
            !runtime_str.contains("/tmp/")
                || runtime_str.starts_with(&*std::env::temp_dir().to_string_lossy()),
            "runtime path should use std::env::temp_dir(), not hardcoded /tmp"
        );
    }

    #[test]
    fn config_dir_uses_xdg_config_home_from_env_source() {
        let env = MapEnv::from([("XDG_CONFIG_HOME", "/tmp/xdg-config")]);
        let got = resolve_config_dir_from_env_source(&env);
        assert_eq!(got, PathBuf::from("/tmp/xdg-config/nestgate"));
    }

    #[test]
    fn cache_dir_uses_xdg_cache_home_from_env_source() {
        let env = MapEnv::from([("XDG_CACHE_HOME", "/tmp/xdg-cache")]);
        let got = resolve_cache_dir_from_env_source(&env);
        assert_eq!(got, PathBuf::from("/tmp/xdg-cache/nestgate"));
    }

    #[test]
    fn cache_dir_from_home_dot_cache_when_xdg_unset() {
        let env = MapEnv::from([("HOME", "/home/testuser")]);
        let got = resolve_cache_dir_from_env_source(&env);
        assert_eq!(got, PathBuf::from("/home/testuser/.cache/nestgate"));
    }
}
