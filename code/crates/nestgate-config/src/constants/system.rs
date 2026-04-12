// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **SYSTEM CONSTANTS**
//!
//! System-level constants and default values with environment variable support.

// Import the configuration module for concurrent-safe access
use super::system_config::SystemConfig;
use nestgate_types::{EnvSource, ProcessEnv};

/// Default instance name for `NestGate` services
pub const DEFAULT_INSTANCE_NAME: &str = "nestgate-default";

/// Default service name
pub const DEFAULT_SERVICE_NAME: &str = "nestgate";

/// Default ecosystem directory segment under runtime paths (for example under `XDG_RUNTIME_DIR`).
///
/// Override with `ECOSYSTEM_NAME` (preferred) or legacy `BIOMEOS_SERVICE_NAME` via
/// [`ecosystem_name`] / [`ecosystem_path_segment`].
pub const ECOSYSTEM_NAME: &str = "biomeos";

/// Ecosystem directory name for paths and discovery, from `env` or [`ECOSYSTEM_NAME`].
///
/// Reads `ECOSYSTEM_NAME` first, then `BIOMEOS_SERVICE_NAME` for backward compatibility.
#[must_use]
pub fn ecosystem_name(env: &(impl EnvSource + ?Sized)) -> String {
    env.get("ECOSYSTEM_NAME")
        .or_else(|| env.get("BIOMEOS_SERVICE_NAME"))
        .unwrap_or_else(|| ECOSYSTEM_NAME.to_owned())
}

/// Directory name for ecosystem-scoped runtime paths (e.g. `$XDG_RUNTIME_DIR/<segment>/...`).
#[must_use]
pub fn ecosystem_path_segment() -> String {
    ecosystem_name(&ProcessEnv)
}

/// Get timeout in milliseconds from environment or default
#[must_use]
pub fn timeout_ms() -> u64 {
    SystemConfig::from_env().timeout_ms()
}

/// Get max connections from environment or default
#[must_use]
pub fn max_connections() -> usize {
    SystemConfig::from_env().max_connections()
}

/// Get buffer size from environment or default
#[must_use]
pub fn buffer_size() -> usize {
    SystemConfig::from_env().buffer_size()
}

/// Get default retry attempts from environment or default
#[must_use]
pub fn default_retry_attempts() -> u32 {
    SystemConfig::from_env().retry_attempts()
}

/// Get health check interval from environment or default
#[must_use]
pub fn health_check_interval() -> u64 {
    SystemConfig::from_env().health_check_interval()
}

/// Get API port from environment or default
#[must_use]
pub fn api_port() -> u16 {
    SystemConfig::from_env().api_port()
}

/// Get bind host from environment or default
#[must_use]
pub fn bind_host() -> String {
    SystemConfig::from_env().bind_host()
}

/// Get API URL from environment or default
#[must_use]
pub fn api_url() -> String {
    SystemConfig::from_env().api_url()
}

/// Legacy constants for backward compatibility
///
/// **Deprecation**: These will be removed in v1.0.0. Use capability-based configuration instead.
/// Default connection timeout
///
/// **Legacy**: Prefer capability-based timeout discovery from service metadata.
pub const DEFAULT_TIMEOUT_MS: u64 = 5000;

/// Maximum connection pool size
///
/// **Legacy**: Prefer dynamic pool sizing based on system resource detection.
pub const MAX_CONNECTIONS: usize = 1000;

/// Default I/O buffer size
///
/// **Legacy**: Prefer operation-specific buffer sizes from performance module.
pub const BUFFER_SIZE: usize = 8192;
/// Default value for retry attempts
pub const DEFAULT_RETRY_ATTEMPTS: u32 = 3;
/// Default value for health check interval
pub const DEFAULT_HEALTH_CHECK_INTERVAL: u64 = 30;
/// Default value for bind host
pub const DEFAULT_BIND_HOST: &str = "127.0.0.1";

#[cfg(test)]
mod tests {
    use super::super::system_config::SystemConfig;
    use super::*;
    use nestgate_types::MapEnv;

    #[test]
    fn ecosystem_name_defaults() {
        let env = MapEnv::new();
        assert_eq!(ecosystem_name(&env), ECOSYSTEM_NAME);
    }

    #[test]
    fn ecosystem_name_from_ecosystem_name_env() {
        let env = MapEnv::from([("ECOSYSTEM_NAME", "myeco")]);
        assert_eq!(ecosystem_name(&env), "myeco");
    }

    #[test]
    fn ecosystem_name_prefers_ecosystem_name_over_legacy() {
        let env = MapEnv::from([
            ("ECOSYSTEM_NAME", "primary"),
            ("BIOMEOS_SERVICE_NAME", "legacy"),
        ]);
        assert_eq!(ecosystem_name(&env), "primary");
    }

    #[test]
    fn ecosystem_name_legacy_biomeos_service_name() {
        let env = MapEnv::from([("BIOMEOS_SERVICE_NAME", "legacy_only")]);
        assert_eq!(ecosystem_name(&env), "legacy_only");
    }

    #[test]
    fn test_default_constants() {
        assert_eq!(DEFAULT_INSTANCE_NAME, "nestgate-default");
        assert_eq!(DEFAULT_SERVICE_NAME, "nestgate");
        assert_eq!(DEFAULT_TIMEOUT_MS, 5000);
        assert_eq!(MAX_CONNECTIONS, 1000);
        assert_eq!(BUFFER_SIZE, 8192);
        assert_eq!(DEFAULT_RETRY_ATTEMPTS, 3);
        assert_eq!(DEFAULT_HEALTH_CHECK_INTERVAL, 30);
    }

    #[test]
    fn timeout_ms_default() {
        let cfg = SystemConfig::from_env_source(&MapEnv::new());
        assert_eq!(cfg.timeout_ms(), 5000);
    }

    #[test]
    fn timeout_ms_from_env() {
        let env = MapEnv::from([("NESTGATE_TIMEOUT_MS", "10000")]);
        assert_eq!(SystemConfig::from_env_source(&env).timeout_ms(), 10000);
    }

    #[test]
    fn timeout_ms_invalid_env() {
        let env = MapEnv::from([("NESTGATE_TIMEOUT_MS", "invalid")]);
        assert_eq!(SystemConfig::from_env_source(&env).timeout_ms(), 5000);
    }

    #[test]
    fn max_connections_default() {
        let cfg = SystemConfig::from_env_source(&MapEnv::new());
        assert_eq!(cfg.max_connections(), 1000);
    }

    #[test]
    fn max_connections_from_env() {
        let env = MapEnv::from([("NESTGATE_MAX_CONNECTIONS", "2000")]);
        assert_eq!(SystemConfig::from_env_source(&env).max_connections(), 2000);
    }

    #[test]
    fn max_connections_invalid_env() {
        let env = MapEnv::from([("NESTGATE_MAX_CONNECTIONS", "not_a_number")]);
        assert_eq!(SystemConfig::from_env_source(&env).max_connections(), 1000);
    }

    #[test]
    fn buffer_size_default() {
        let cfg = SystemConfig::from_env_source(&MapEnv::new());
        assert_eq!(cfg.buffer_size(), 8192);
    }

    #[test]
    fn buffer_size_from_env() {
        let env = MapEnv::from([("NESTGATE_BUFFER_SIZE", "16384")]);
        assert_eq!(SystemConfig::from_env_source(&env).buffer_size(), 16384);
    }

    #[test]
    fn buffer_size_invalid_env() {
        let env = MapEnv::from([("NESTGATE_BUFFER_SIZE", "invalid")]);
        assert_eq!(SystemConfig::from_env_source(&env).buffer_size(), 8192);
    }

    #[test]
    fn retry_attempts_default() {
        let cfg = SystemConfig::from_env_source(&MapEnv::new());
        assert_eq!(cfg.retry_attempts(), 3);
    }

    #[test]
    fn retry_attempts_from_env() {
        let env = MapEnv::from([("NESTGATE_RETRY_ATTEMPTS", "5")]);
        assert_eq!(SystemConfig::from_env_source(&env).retry_attempts(), 5);
    }

    #[test]
    fn retry_attempts_invalid_env() {
        let env = MapEnv::from([("NESTGATE_RETRY_ATTEMPTS", "not_a_number")]);
        assert_eq!(SystemConfig::from_env_source(&env).retry_attempts(), 3);
    }

    #[test]
    fn health_check_interval_default() {
        let cfg = SystemConfig::from_env_source(&MapEnv::new());
        assert_eq!(cfg.health_check_interval(), 30);
    }

    #[test]
    fn health_check_interval_from_env() {
        let env = MapEnv::from([("NESTGATE_HEALTH_CHECK_INTERVAL", "60")]);
        assert_eq!(
            SystemConfig::from_env_source(&env).health_check_interval(),
            60
        );
    }

    #[test]
    fn health_check_interval_invalid_env() {
        let env = MapEnv::from([("NESTGATE_HEALTH_CHECK_INTERVAL", "invalid")]);
        assert_eq!(
            SystemConfig::from_env_source(&env).health_check_interval(),
            30
        );
    }

    #[test]
    fn edge_cases() {
        let zero = MapEnv::from([("NESTGATE_TIMEOUT_MS", "0")]);
        assert_eq!(SystemConfig::from_env_source(&zero).timeout_ms(), 0);

        let negative = MapEnv::from([("NESTGATE_MAX_CONNECTIONS", "-1")]);
        assert_eq!(
            SystemConfig::from_env_source(&negative).max_connections(),
            1000
        );

        let large = MapEnv::from([("NESTGATE_BUFFER_SIZE", "1048576")]);
        assert_eq!(
            SystemConfig::from_env_source(&large).buffer_size(),
            1_048_576
        );
    }

    #[test]
    fn multiple_env_vars_simultaneously() {
        let env = MapEnv::from([
            ("NESTGATE_TIMEOUT_MS", "2000"),
            ("NESTGATE_MAX_CONNECTIONS", "500"),
            ("NESTGATE_BUFFER_SIZE", "4096"),
        ]);
        let cfg = SystemConfig::from_env_source(&env);
        assert_eq!(cfg.timeout_ms(), 2000);
        assert_eq!(cfg.max_connections(), 500);
        assert_eq!(cfg.buffer_size(), 4096);
    }
}
