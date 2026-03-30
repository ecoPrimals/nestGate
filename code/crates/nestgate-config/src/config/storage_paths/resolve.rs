// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Environment and XDG fallback resolution for each path kind.

use std::env;
use std::path::PathBuf;
use tracing::{debug, warn};

/// Resolve data directory with XDG-compliant fallback
pub fn resolve_data_dir() -> PathBuf {
    if let Ok(path) = env::var("NESTGATE_DATA_DIR") {
        debug!("📂 Data dir from NESTGATE_DATA_DIR: {}", path);
        return PathBuf::from(path);
    }

    if let Ok(xdg_data) = env::var("XDG_DATA_HOME") {
        let path = PathBuf::from(xdg_data).join("nestgate");
        debug!("📂 Data dir from XDG_DATA_HOME: {}", path.display());
        return path;
    }

    if let Ok(home) = env::var("HOME") {
        let path = PathBuf::from(home).join(".local/share/nestgate");
        debug!("📂 Data dir from HOME: {}", path.display());
        return path;
    }

    warn!("📂 Data dir using system fallback (requires permissions): /var/lib/nestgate");
    PathBuf::from("/var/lib/nestgate")
}

/// Resolve config directory with XDG-compliant fallback
pub fn resolve_config_dir() -> PathBuf {
    if let Ok(path) = env::var("NESTGATE_CONFIG_DIR") {
        debug!("📂 Config dir from NESTGATE_CONFIG_DIR: {}", path);
        return PathBuf::from(path);
    }

    if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
        let path = PathBuf::from(xdg_config).join("nestgate");
        debug!("📂 Config dir from XDG_CONFIG_HOME: {}", path.display());
        return path;
    }

    if let Ok(home) = env::var("HOME") {
        let path = PathBuf::from(home).join(".config/nestgate");
        debug!("📂 Config dir from HOME: {}", path.display());
        return path;
    }

    warn!("📂 Config dir using system fallback (requires permissions): /etc/nestgate");
    PathBuf::from("/etc/nestgate")
}

/// Resolve cache directory with XDG-compliant fallback
pub fn resolve_cache_dir() -> PathBuf {
    if let Ok(path) = env::var("NESTGATE_CACHE_DIR") {
        debug!("📂 Cache dir from NESTGATE_CACHE_DIR: {}", path);
        return PathBuf::from(path);
    }

    if let Ok(xdg_cache) = env::var("XDG_CACHE_HOME") {
        let path = PathBuf::from(xdg_cache).join("nestgate");
        debug!("📂 Cache dir from XDG_CACHE_HOME: {}", path.display());
        return path;
    }

    if let Ok(home) = env::var("HOME") {
        let path = PathBuf::from(home).join(".cache/nestgate");
        debug!("📂 Cache dir from HOME: {}", path.display());
        return path;
    }

    warn!("📂 Cache dir using system fallback: /var/cache/nestgate");
    PathBuf::from("/var/cache/nestgate")
}

/// Resolve state directory with XDG-compliant fallback
pub fn resolve_state_dir() -> PathBuf {
    if let Ok(path) = env::var("NESTGATE_STATE_DIR") {
        debug!("📂 State dir from NESTGATE_STATE_DIR: {}", path);
        return PathBuf::from(path);
    }

    if let Ok(xdg_state) = env::var("XDG_STATE_HOME") {
        let path = PathBuf::from(xdg_state).join("nestgate");
        debug!("📂 State dir from XDG_STATE_HOME: {}", path.display());
        return path;
    }

    if let Ok(home) = env::var("HOME") {
        let path = PathBuf::from(home).join(".local/state/nestgate");
        debug!("📂 State dir from HOME: {}", path.display());
        return path;
    }

    warn!("📂 State dir using system fallback: /var/lib/nestgate/state");
    PathBuf::from("/var/lib/nestgate/state")
}

/// Resolve log directory with XDG-compliant fallback
pub fn resolve_log_dir() -> PathBuf {
    if let Ok(path) = env::var("NESTGATE_LOG_DIR") {
        debug!("📂 Log dir from NESTGATE_LOG_DIR: {}", path);
        return PathBuf::from(path);
    }

    if let Ok(xdg_state) = env::var("XDG_STATE_HOME") {
        let path = PathBuf::from(xdg_state).join("nestgate/logs");
        debug!("📂 Log dir from XDG_STATE_HOME: {}", path.display());
        return path;
    }

    if let Ok(home) = env::var("HOME") {
        let path = PathBuf::from(home).join(".local/state/nestgate/logs");
        debug!("📂 Log dir from HOME: {}", path.display());
        return path;
    }

    warn!("📂 Log dir using system fallback (requires permissions): /var/log/nestgate");
    PathBuf::from("/var/log/nestgate")
}

/// Resolve temporary directory with XDG-compliant fallback
pub fn resolve_temp_dir() -> PathBuf {
    if let Ok(path) = env::var("NESTGATE_TEMP_DIR") {
        debug!("📂 Temp dir from NESTGATE_TEMP_DIR: {}", path);
        return PathBuf::from(path);
    }

    if let Ok(tmpdir) = env::var("TMPDIR") {
        let path = PathBuf::from(tmpdir).join("nestgate");
        debug!("📂 Temp dir from TMPDIR: {}", path.display());
        return path;
    }

    debug!("📂 Temp dir using system fallback: /tmp/nestgate");
    PathBuf::from("/tmp/nestgate")
}

/// Resolve runtime directory with XDG-compliant fallback
pub fn resolve_runtime_dir() -> PathBuf {
    if let Ok(path) = env::var("NESTGATE_RUNTIME_DIR") {
        debug!("📂 Runtime dir from NESTGATE_RUNTIME_DIR: {}", path);
        return PathBuf::from(path);
    }

    if let Ok(xdg_runtime) = env::var("XDG_RUNTIME_DIR") {
        let path = PathBuf::from(xdg_runtime).join("nestgate");
        debug!("📂 Runtime dir from XDG_RUNTIME_DIR: {}", path.display());
        return path;
    }

    warn!("📂 Runtime dir using fallback: /tmp/nestgate-runtime");
    PathBuf::from("/tmp/nestgate-runtime")
}
