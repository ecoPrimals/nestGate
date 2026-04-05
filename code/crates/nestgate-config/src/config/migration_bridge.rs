// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Cached global `EnvironmentConfig` accessor.
//!
//! Loads configuration from environment on first access and caches it for the process lifetime.

use super::environment::EnvironmentConfig;
use std::sync::OnceLock;
use tracing::warn;

static GLOBAL_CONFIG: OnceLock<EnvironmentConfig> = OnceLock::new();

fn global_config() -> &'static EnvironmentConfig {
    GLOBAL_CONFIG.get_or_init(|| match EnvironmentConfig::from_env() {
        Ok(config) => config,
        Err(e) => {
            warn!(
                "Failed to load environment config: {e}; using defaults \
                 (set NESTGATE_* environment variables to customize)"
            );
            EnvironmentConfig::default()
        }
    })
}

/// Get the global environment configuration (cached on first access).
#[must_use]
pub fn config() -> &'static EnvironmentConfig {
    global_config()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_config_cached() {
        let config1 = config();
        let config2 = config();
        assert!(std::ptr::eq(config1, config2));
    }

    #[test]
    fn test_modern_config_access() {
        let cfg = config();
        assert!(cfg.network.port.get() >= 1024);
        assert!(!cfg.network.host.is_empty());
        assert!(!cfg.storage.zfs_pool.is_empty());
        assert!(cfg.monitoring.metrics_port.get() >= 1024);
    }
}
