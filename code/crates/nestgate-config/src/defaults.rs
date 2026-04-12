// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(deprecated, reason = "migration to RuntimePortResolver in progress")]

//! # Default Constants for `NestGate`
//!
//! Centralized default values for network, storage, security, and timeouts.
//! For runtime configuration, use `EnvironmentConfig::from_env()` or capability discovery.

use crate::constants::hardcoding::{addresses, runtime_fallback_ports};
use std::time::Duration;

/// **NETWORK DEFAULTS**
pub mod network {
    use super::{addresses, runtime_fallback_ports};

    /// Default API port - can be overridden with `NESTGATE_API_PORT`
    pub const DEFAULT_API_PORT: u16 = runtime_fallback_ports::HTTP;

    /// Default bind address - can be overridden with `NESTGATE_BIND_ADDRESS`  
    pub const DEFAULT_BIND_ADDRESS: &str = addresses::BIND_ALL_IPV4;

    /// Default hostname for development - can be overridden with `NESTGATE_HOSTNAME`
    pub const DEFAULT_HOSTNAME: &str = addresses::LOCALHOST_NAME;

    /// Default WebSocket port - can be overridden with `NESTGATE_WS_PORT`
    pub const DEFAULT_WS_PORT: u16 = runtime_fallback_ports::WEBSOCKET;

    /// Default health check port - can be overridden with `NESTGATE_HEALTH_PORT`
    pub const DEFAULT_HEALTH_PORT: u16 = runtime_fallback_ports::HEALTH;
}

/// **DATABASE DEFAULTS**
pub mod database {
    use super::{addresses, runtime_fallback_ports};

    /// Default `PostgreSQL` port - can be overridden with `NESTGATE_DB_PORT`
    pub const DEFAULT_POSTGRES_PORT: u16 = runtime_fallback_ports::POSTGRES;

    /// Default Redis port - can be overridden with `NESTGATE_REDIS_PORT`
    pub const DEFAULT_REDIS_PORT: u16 = runtime_fallback_ports::REDIS;

    /// Default `MongoDB` port - can be overridden with `NESTGATE_MONGODB_PORT`
    pub const DEFAULT_MONGODB_PORT: u16 = runtime_fallback_ports::MONGODB;

    /// Default `MySQL` port - can be overridden with `NESTGATE_MYSQL_PORT`
    pub const DEFAULT_MYSQL_PORT: u16 = runtime_fallback_ports::MYSQL;

    /// Default database host - can be overridden with `NESTGATE_DB_HOST`
    pub const DEFAULT_DB_HOST: &str = addresses::LOCALHOST_NAME;
}

/// **MONITORING DEFAULTS**
pub mod monitoring {
    use super::runtime_fallback_ports;

    /// Default Prometheus port - can be overridden with `NESTGATE_METRICS_PORT`
    pub const DEFAULT_METRICS_PORT: u16 = runtime_fallback_ports::METRICS;

    /// Default Grafana port - can be overridden with `NESTGATE_GRAFANA_PORT`
    pub const DEFAULT_GRAFANA_PORT: u16 = runtime_fallback_ports::API;
}

/// **TIMEOUT DEFAULTS**
pub mod timeouts {
    use super::Duration;

    /// Default API request timeout
    pub const DEFAULT_API_TIMEOUT: Duration = Duration::from_secs(30);

    /// Default database connection timeout
    pub const DEFAULT_DB_TIMEOUT: Duration = Duration::from_secs(10);

    /// Default health check timeout
    pub const DEFAULT_HEALTH_TIMEOUT: Duration = Duration::from_secs(5);

    /// Default WebSocket timeout
    pub const DEFAULT_WS_TIMEOUT: Duration = Duration::from_secs(60);
}

/// **STORAGE DEFAULTS**
pub mod storage {
    /// Default ZFS pool name
    pub const DEFAULT_POOL_NAME: &str = "nestgate-pool";

    /// Default cache size in MB
    pub const DEFAULT_CACHE_SIZE_MB: u64 = 1024;

    /// Default compression algorithm
    pub const DEFAULT_COMPRESSION: &str = "lz4";

    /// Default storage service port - can be overridden with `NESTGATE_STORAGE_PORT`
    pub const DEFAULT_STORAGE_PORT: u16 = 5000;

    /// Default NFS port - can be overridden with `NESTGATE_NFS_PORT`
    pub const DEFAULT_NFS_PORT: u16 = 2049;

    /// Default SMB port - can be overridden with `NESTGATE_SMB_PORT`
    pub const DEFAULT_SMB_PORT: u16 = 445;
}

/// **SECURITY DEFAULTS**
pub mod security {
    use super::Duration;

    /// Default session timeout
    pub const DEFAULT_SESSION_TIMEOUT: Duration = Duration::from_secs(3600);

    /// Default token expiry
    pub const DEFAULT_TOKEN_EXPIRY: Duration = Duration::from_secs(1800);

    /// Default max login attempts
    pub const DEFAULT_MAX_LOGIN_ATTEMPTS: u32 = 5;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::environment::EnvironmentConfig;
    use crate::constants::hardcoding::runtime_fallback_ports;
    use nestgate_types::MapEnv;

    #[test]
    fn test_default_constants() {
        assert_eq!(network::DEFAULT_API_PORT, runtime_fallback_ports::HTTP);
        assert_eq!(network::DEFAULT_BIND_ADDRESS, addresses::BIND_ALL_IPV4);
        assert_eq!(database::DEFAULT_POSTGRES_PORT, 5432);
    }

    #[test]
    fn test_environment_override() {
        let env = MapEnv::from([("NESTGATE_API_PORT", "9999")]);
        let cfg = EnvironmentConfig::from_env_source(&env).expect("config");
        assert_eq!(cfg.network.port.get(), 9999);

        let env_default = MapEnv::new();
        let cfg = EnvironmentConfig::from_env_source(&env_default).expect("config");
        assert_eq!(cfg.network.port.get(), runtime_fallback_ports::HTTP);
    }

    #[test]
    fn test_network_defaults() {
        assert_eq!(network::DEFAULT_API_PORT, runtime_fallback_ports::HTTP);
        assert_eq!(network::DEFAULT_BIND_ADDRESS, addresses::BIND_ALL_IPV4);
        assert_eq!(network::DEFAULT_HOSTNAME, addresses::LOCALHOST_NAME);
        assert_eq!(network::DEFAULT_WS_PORT, runtime_fallback_ports::WEBSOCKET);
        assert_eq!(network::DEFAULT_HEALTH_PORT, runtime_fallback_ports::HEALTH);
    }

    #[test]
    fn test_database_defaults() {
        assert_eq!(database::DEFAULT_POSTGRES_PORT, 5432);
        assert_eq!(database::DEFAULT_REDIS_PORT, 6379);
        assert_eq!(database::DEFAULT_DB_HOST, "localhost");
    }

    #[test]
    fn test_monitoring_defaults() {
        assert_eq!(monitoring::DEFAULT_METRICS_PORT, 9090);
        assert_eq!(monitoring::DEFAULT_GRAFANA_PORT, 3000);
    }

    #[test]
    fn test_timeout_defaults() {
        assert_eq!(timeouts::DEFAULT_API_TIMEOUT, Duration::from_secs(30));
        assert_eq!(timeouts::DEFAULT_DB_TIMEOUT, Duration::from_secs(10));
        assert_eq!(timeouts::DEFAULT_HEALTH_TIMEOUT, Duration::from_secs(5));
        assert_eq!(timeouts::DEFAULT_WS_TIMEOUT, Duration::from_secs(60));
    }

    #[test]
    fn test_storage_defaults() {
        assert_eq!(storage::DEFAULT_POOL_NAME, "nestgate-pool");
        assert_eq!(storage::DEFAULT_CACHE_SIZE_MB, 1024);
        assert_eq!(storage::DEFAULT_COMPRESSION, "lz4");
        assert_eq!(storage::DEFAULT_STORAGE_PORT, 5000);
        assert_eq!(storage::DEFAULT_NFS_PORT, 2049);
        assert_eq!(storage::DEFAULT_SMB_PORT, 445);
    }

    #[test]
    fn test_security_defaults() {
        assert_eq!(security::DEFAULT_SESSION_TIMEOUT, Duration::from_secs(3600));
        assert_eq!(security::DEFAULT_TOKEN_EXPIRY, Duration::from_secs(1800));
        assert_eq!(security::DEFAULT_MAX_LOGIN_ATTEMPTS, 5);
    }

    #[test]
    fn test_env_config_api_port() {
        let env = MapEnv::new();
        assert_eq!(
            EnvironmentConfig::from_env_source(&env)
                .expect("config")
                .network
                .port
                .get(),
            8080
        );
        let env = MapEnv::from([("NESTGATE_API_PORT", "3000")]);
        assert_eq!(
            EnvironmentConfig::from_env_source(&env)
                .expect("config")
                .network
                .port
                .get(),
            3000
        );
        let env = MapEnv::from([("NESTGATE_API_PORT", "invalid")]);
        assert!(
            EnvironmentConfig::from_env_source(&env).is_err(),
            "invalid NESTGATE_API_PORT should fail to parse"
        );
    }

    #[test]
    fn test_timeout_duration_values() {
        assert!(timeouts::DEFAULT_API_TIMEOUT.as_secs() > 0);
        assert!(timeouts::DEFAULT_DB_TIMEOUT.as_secs() > 0);
        assert!(timeouts::DEFAULT_HEALTH_TIMEOUT.as_secs() > 0);
        assert!(timeouts::DEFAULT_WS_TIMEOUT.as_secs() > 0);

        assert!(timeouts::DEFAULT_HEALTH_TIMEOUT < timeouts::DEFAULT_DB_TIMEOUT);
        assert!(timeouts::DEFAULT_DB_TIMEOUT < timeouts::DEFAULT_API_TIMEOUT);
        assert!(timeouts::DEFAULT_API_TIMEOUT < timeouts::DEFAULT_WS_TIMEOUT);
    }

    #[test]
    fn test_database_port_references() {
        assert_eq!(
            database::DEFAULT_POSTGRES_PORT,
            runtime_fallback_ports::POSTGRES
        );
        assert_eq!(database::DEFAULT_REDIS_PORT, runtime_fallback_ports::REDIS);
        assert_eq!(
            database::DEFAULT_MONGODB_PORT,
            runtime_fallback_ports::MONGODB
        );
        assert_eq!(database::DEFAULT_MYSQL_PORT, runtime_fallback_ports::MYSQL);
    }

    #[test]
    fn test_monitoring_port_references() {
        assert_eq!(
            monitoring::DEFAULT_METRICS_PORT,
            runtime_fallback_ports::METRICS
        );
        assert_eq!(
            monitoring::DEFAULT_GRAFANA_PORT,
            runtime_fallback_ports::API
        );
    }

    #[test]
    fn test_network_port_references() {
        assert_eq!(network::DEFAULT_API_PORT, runtime_fallback_ports::HTTP);
        assert_eq!(network::DEFAULT_WS_PORT, runtime_fallback_ports::WEBSOCKET);
        assert_eq!(network::DEFAULT_HEALTH_PORT, runtime_fallback_ports::HEALTH);
    }

    #[test]
    fn test_network_address_references() {
        assert_eq!(network::DEFAULT_BIND_ADDRESS, addresses::BIND_ALL_IPV4);
        assert_eq!(network::DEFAULT_HOSTNAME, addresses::LOCALHOST_NAME);
        assert_eq!(database::DEFAULT_DB_HOST, addresses::LOCALHOST_NAME);
    }
}
