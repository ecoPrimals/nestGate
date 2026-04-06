// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Port configuration constants and environment-driven helpers.
//!
//! For runtime configuration, prefer `EnvironmentConfig::from_env()` or capability discovery.
//! Constants serve as compile-time fallbacks; the `get_*` functions resolve from env vars at runtime.

use nestgate_types::{EnvSource, ProcessEnv, env_parsed, env_var_or_default};

/// Default port for the main `NestGate` API server
///
/// **Environment Variable**: `NESTGATE_API_PORT`\
/// **Default**: 8080\
/// **Usage**: Main HTTP API endpoints
pub const API_SERVER_DEFAULT: u16 = 8080;

/// Default port for development server
///
/// **Environment Variable**: `NESTGATE_DEV_PORT`\
/// **Default**: 3000\
/// **Usage**: Development and hot-reload server
pub const DEV_SERVER_DEFAULT: u16 = 3000;

/// Default port for Prometheus metrics
///
/// **Environment Variable**: `NESTGATE_METRICS_PORT`\
/// **Default**: 9090\
/// **Usage**: Prometheus metrics endpoint
pub const METRICS_SERVER_DEFAULT: u16 = 9090;

/// Default port for Grafana dashboard
///
/// **Environment Variable**: `GRAFANA_PORT`\
/// **Default**: 3001\
/// **Usage**: Grafana monitoring dashboard
pub const GRAFANA_DEFAULT: u16 = 3001;

// ==================== DATABASE PORTS ====================

/// Default port for `PostgreSQL` database
///
/// **Environment Variable**: `POSTGRES_PORT`\
/// **Default**: 5432\
/// **Usage**: `PostgreSQL` database connections
pub const POSTGRES_DEFAULT: u16 = 5432;

/// Default port for Redis cache
///
/// **Environment Variable**: `REDIS_PORT`\
/// **Default**: 6379\
/// **Usage**: Redis cache and session store
pub const REDIS_DEFAULT: u16 = 6379;

/// Default port for `MongoDB` database
///
/// **Environment Variable**: `MONGODB_PORT`\
/// **Default**: 27017\
/// **Usage**: `MongoDB` document database
pub const MONGODB_DEFAULT: u16 = 27017;

// ==================== PRIMAL DISCOVERY PORTS ====================

/// Default port for Primal Discovery service
///
/// **Environment Variable**: `PRIMAL_DISCOVERY_PORT`\
/// **Default**: 5000\
/// **Usage**: Infant Discovery architecture service discovery
pub const PRIMAL_DISCOVERY_DEFAULT: u16 = 5000;

/// Default port for networking service (capability-based discovery preferred)
///
/// **Environment Variable**: `NETWORKING_SERVICE_PORT`\
/// **Default**: 9091\
/// **Usage**: Generic networking service communication (use capability discovery)
pub const NETWORKING_SERVICE_DEFAULT: u16 = 9091;

/// Default port for security service (capability-based discovery preferred)
///
/// **Environment Variable**: `SECURITY_SERVICE_PORT`\
/// **Default**: 9092\
/// **Usage**: Generic security service communication (use capability discovery)
pub const SECURITY_SERVICE_DEFAULT: u16 = 9092;

// ==================== MODERN ENVIRONMENT-DRIVEN HELPERS ====================

/// Get API server address (host:port) from environment
///
/// **Environment Variables**:
/// - `NESTGATE_HOST`: Bind host (default: "0.0.0.0")
/// - `NESTGATE_PORT`: Bind port (default: 8080)
///
/// # Returns
/// Formatted address string like "0.0.0.0:8080"
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::constants::ports;
///
/// // With defaults
/// let addr = ports::get_api_server_addr();
/// assert_eq!(addr, "0.0.0.0:8080");
///
/// // With environment override
/// nestgate_core::env_process::set_var("NESTGATE_PORT", "9090");
/// // addr will use 9090
/// ```
#[must_use]
pub fn get_api_server_addr() -> String {
    get_api_server_addr_from_env_source(&ProcessEnv)
}

/// [`get_api_server_addr`] reading from an injectable [`EnvSource`].
#[must_use]
pub fn get_api_server_addr_from_env_source(env: &dyn EnvSource) -> String {
    let host = env_var_or_default(env, "NESTGATE_HOST", "0.0.0.0");
    let port = env_parsed(env, "NESTGATE_PORT", API_SERVER_DEFAULT);
    format!("{host}:{port}")
}

/// Get RPC server address (host:port) from environment
///
/// **Environment Variables**:
/// - `NESTGATE_RPC_HOST`: Bind host (default: "0.0.0.0")
/// - `NESTGATE_RPC_PORT`: Bind port (default: 8091)
///
/// # Returns
/// Formatted address string like "0.0.0.0:8091"
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::constants::ports;
///
/// let addr = ports::get_rpc_server_addr();
/// // Default: "0.0.0.0:8091"
/// ```
#[must_use]
pub fn get_rpc_server_addr() -> String {
    get_rpc_server_addr_from_env_source(&ProcessEnv)
}

/// [`get_rpc_server_addr`] reading from an injectable [`EnvSource`].
#[must_use]
pub fn get_rpc_server_addr_from_env_source(env: &dyn EnvSource) -> String {
    let host = env_var_or_default(env, "NESTGATE_RPC_HOST", "0.0.0.0");
    let port = env_parsed(env, "NESTGATE_RPC_PORT", 8091u16);
    format!("{host}:{port}")
}

/// Default **outbound** tarpc URL when capability discovery and `NESTGATE_<CAP>_ENDPOINT` are unset.
///
/// Use this for tarpc **clients** connecting to a peer. Bind addresses for servers remain
/// [`get_rpc_server_addr`] (often `0.0.0.0`).
///
/// **Environment variables** (precedence):
/// 1. `NESTGATE_RPC_ENDPOINT` — full URL or `host:port` (optional `tarpc://` prefix added when missing)
/// 2. `NESTGATE_RPC_CONNECT_HOST` (default `127.0.0.1`) and `NESTGATE_RPC_PORT` (default `8091`)
#[must_use]
pub fn default_tarpc_client_endpoint() -> String {
    default_tarpc_client_endpoint_from_env_source(&ProcessEnv)
}

/// [`default_tarpc_client_endpoint`] reading from an injectable [`EnvSource`].
#[must_use]
pub fn default_tarpc_client_endpoint_from_env_source(env: &dyn EnvSource) -> String {
    if let Some(raw) = env.get("NESTGATE_RPC_ENDPOINT") {
        let s = raw.trim();
        if !s.is_empty() {
            if s.starts_with("tarpc://") {
                return s.to_string();
            }
            if let Some(rest) = s.strip_prefix("http://") {
                return format!("tarpc://{rest}");
            }
            if let Some(rest) = s.strip_prefix("https://") {
                return format!("tarpc://{rest}");
            }
            return format!("tarpc://{s}");
        }
    }
    let host = env_var_or_default(env, "NESTGATE_RPC_CONNECT_HOST", "127.0.0.1");
    let port = env_parsed(env, "NESTGATE_RPC_PORT", 8091u16);
    format!("tarpc://{host}:{port}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_ports() {
        assert_eq!(API_SERVER_DEFAULT, 8080);
        assert_eq!(DEV_SERVER_DEFAULT, 3000);
        assert_eq!(POSTGRES_DEFAULT, 5432);
        assert_eq!(REDIS_DEFAULT, 6379);
        assert_eq!(MONGODB_DEFAULT, 27017);
        assert_eq!(PRIMAL_DISCOVERY_DEFAULT, 5000);
    }

    #[test]
    fn default_tarpc_client_endpoint_format() {
        let ep = default_tarpc_client_endpoint();
        assert!(ep.starts_with("tarpc://"));
        let hostport = ep.strip_prefix("tarpc://").expect("tarpc URL");
        assert!(hostport.contains(':'));
        let port_str = hostport.rsplit_once(':').expect("host:port").1;
        assert!(port_str.parse::<u16>().is_ok());
    }

    #[test]
    fn test_get_api_server_addr_format() {
        let addr = get_api_server_addr();
        assert!(addr.contains(':'));
    }

    #[test]
    fn test_get_rpc_server_addr_format() {
        let addr = get_rpc_server_addr();
        assert!(addr.contains(':'));
    }
}
