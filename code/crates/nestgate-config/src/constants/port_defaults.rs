// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Port Number Defaults - Environment-Driven Configuration
//!
//! This module provides centralized default port numbers for various services.
//! All ports are configurable via environment variables with smart defaults.
//!
//! **Modern Pattern**: Use `get_*_port()` functions (defined below) instead of constants
//! for runtime environment configuration.

use super::port_defaults_config::PortConfig;

// ==================== NESTGATE SERVICE PORTS ====================

/// Default `NestGate` API server port
///
/// **Environment Variable**: `NESTGATE_API_PORT`\
/// **Usage**: Main API server, HTTP/REST endpoints
pub const DEFAULT_API_PORT: u16 = 8080;

/// Default `NestGate` admin port
///
/// **Environment Variable**: `NESTGATE_ADMIN_PORT`\
/// **Usage**: Admin interface, management endpoints
pub const DEFAULT_ADMIN_PORT: u16 = 8081;

/// Default `NestGate` metrics port
///
/// **Environment Variable**: `NESTGATE_METRICS_PORT`\
/// **Usage**: Prometheus metrics, monitoring
pub const DEFAULT_METRICS_PORT: u16 = 9090;

/// Default `NestGate` health check port
///
/// **Environment Variable**: `NESTGATE_HEALTH_PORT`\
/// **Usage**: Health check endpoint, load balancer
pub const DEFAULT_HEALTH_PORT: u16 = 8082;

// ==================== DEVELOPMENT PORTS ====================

/// Default development server port
///
/// **Environment Variable**: `NESTGATE_DEV_PORT`\
/// **Usage**: Development mode, local testing
pub const DEFAULT_DEV_PORT: u16 = 3000;

/// Alternative development port
///
/// **Environment Variable**: `NESTGATE_DEV_ALT_PORT`\
/// **Usage**: Alternative dev server, parallel instances
pub const DEFAULT_DEV_ALT_PORT: u16 = 5000;

// ==================== DATABASE PORTS ====================

/// Default `PostgreSQL` port
///
/// **Environment Variable**: `NESTGATE_POSTGRES_PORT`\
/// **Usage**: `PostgreSQL` database connections
pub const DEFAULT_POSTGRES_PORT: u16 = 5432;

/// Default `MySQL` port
///
/// **Environment Variable**: `NESTGATE_MYSQL_PORT`\
/// **Usage**: `MySQL` database connections
pub const DEFAULT_MYSQL_PORT: u16 = 3306;

/// Default `MongoDB` port
///
/// **Environment Variable**: `NESTGATE_MONGODB_PORT`\
/// **Usage**: `MongoDB` database connections
pub const DEFAULT_MONGODB_PORT: u16 = 27017;

/// Default Redis port
///
/// **Environment Variable**: `NESTGATE_REDIS_PORT`\
/// **Usage**: Redis cache connections
pub const DEFAULT_REDIS_PORT: u16 = 6379;

// ==================== MONITORING & OBSERVABILITY ====================

/// Default Prometheus port
///
/// **Environment Variable**: `NESTGATE_PROMETHEUS_PORT`\
/// **Usage**: Prometheus scraping, metrics collection
pub const DEFAULT_PROMETHEUS_PORT: u16 = 9090;

/// Default Grafana port
///
/// **Environment Variable**: `NESTGATE_GRAFANA_PORT`\
/// **Usage**: Grafana dashboards, visualization
pub const DEFAULT_GRAFANA_PORT: u16 = 3001;

/// Default Jaeger port
///
/// **Environment Variable**: `NESTGATE_JAEGER_PORT`\
/// **Usage**: Distributed tracing, Jaeger collector
pub const DEFAULT_JAEGER_PORT: u16 = 14268;

// ==================== MESSAGE QUEUES ====================

/// Default `RabbitMQ` port
///
/// **Environment Variable**: `NESTGATE_RABBITMQ_PORT`\
/// **Usage**: `RabbitMQ` message broker
pub const DEFAULT_RABBITMQ_PORT: u16 = 5672;

/// Default Kafka port
///
/// **Environment Variable**: `NESTGATE_KAFKA_PORT`\
/// **Usage**: Apache Kafka streaming
pub const DEFAULT_KAFKA_PORT: u16 = 9092;

// ==================== HELPER FUNCTIONS ====================

/// Get API port from environment or default
/// NOTE: Creates config from env each time. For tests, use `PortConfig` directly.
///
/// Reads from `NESTGATE_API_PORT` environment variable, falls back to default
#[must_use]
pub fn get_api_port() -> u16 {
    PortConfig::from_env().get_api_port()
}

/// Get metrics port from environment or default
/// NOTE: Creates config from env each time. For tests, use `PortConfig` directly.
///
/// Reads from `NESTGATE_METRICS_PORT` environment variable, falls back to default
#[must_use]
pub fn get_metrics_port() -> u16 {
    PortConfig::from_env().get_metrics_port()
}

/// Get health check port from environment or default
/// NOTE: Creates config from env each time. For tests, use `PortConfig` directly.
///
/// Reads from `NESTGATE_HEALTH_PORT` environment variable, falls back to default
#[must_use]
pub fn get_health_port() -> u16 {
    PortConfig::from_env().get_health_port()
}

/// Get admin port from environment or default
/// NOTE: Creates config from env each time. For tests, use `PortConfig` directly.
///
/// Reads from `NESTGATE_ADMIN_PORT` environment variable, falls back to default
#[must_use]
pub fn get_admin_port() -> u16 {
    PortConfig::from_env().get_admin_port()
}

/// Get development port from environment or default
/// NOTE: Creates config from env each time. For tests, use `PortConfig` directly.
///
/// Reads from `NESTGATE_DEV_PORT` environment variable, falls back to default
#[must_use]
pub fn get_dev_port() -> u16 {
    PortConfig::from_env().get_dev_port()
}

/// Get `PostgreSQL` port from environment or default
/// NOTE: Creates config from env each time. For tests, use `PortConfig` directly.
///
/// Reads from `NESTGATE_POSTGRES_PORT` environment variable, falls back to default
#[must_use]
pub fn get_postgres_port() -> u16 {
    PortConfig::from_env().get_postgres_port()
}

/// Get Redis port from environment or default
/// NOTE: Creates config from env each time. For tests, use `PortConfig` directly.
///
/// Reads from `NESTGATE_REDIS_PORT` environment variable, falls back to default
#[must_use]
pub fn get_redis_port() -> u16 {
    PortConfig::from_env().get_redis_port()
}

/// Get Prometheus port from environment or default
/// NOTE: Creates config from env each time. For tests, use `PortConfig` directly.
///
/// Reads from `NESTGATE_PROMETHEUS_PORT` environment variable, falls back to default
#[must_use]
pub fn get_prometheus_port() -> u16 {
    PortConfig::from_env().get_prometheus_port()
}

/// Get Grafana port from environment or default
/// NOTE: Creates config from env each time. For tests, use `PortConfig` directly.
///
/// Reads from `NESTGATE_GRAFANA_PORT` environment variable, falls back to default
#[must_use]
pub fn get_grafana_port() -> u16 {
    PortConfig::from_env().get_grafana_port()
}

/// Parse port from string with validation
///
/// Returns None if port is invalid (0 or > 65535)
#[must_use]
pub fn parse_port(port_str: &str) -> Option<u16> {
    port_str.parse::<u16>().ok().filter(|&p| p > 0)
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_constants_valid() {
        // All ports should be non-zero (u16 type ensures they're <= 65535)
        // These are const values but we verify they're set to non-zero
        assert_ne!(DEFAULT_API_PORT, 0);
        assert_ne!(DEFAULT_METRICS_PORT, 0);
        assert_ne!(DEFAULT_POSTGRES_PORT, 0);
        assert_ne!(DEFAULT_REDIS_PORT, 0);
    }

    #[test]
    fn test_common_ports() {
        assert_eq!(DEFAULT_API_PORT, 8080);
        assert_eq!(DEFAULT_METRICS_PORT, 9090);
        assert_eq!(DEFAULT_POSTGRES_PORT, 5432);
        assert_eq!(DEFAULT_REDIS_PORT, 6379);
    }

    #[test]
    fn test_get_api_port() {
        // Should return default when env var not set
        let port = get_api_port();
        assert!(port > 0, "API port should be valid");
    }

    #[test]
    fn test_parse_port_valid() {
        assert_eq!(parse_port("8080"), Some(8080));
        assert_eq!(parse_port("443"), Some(443));
        assert_eq!(parse_port("65535"), Some(65535));
    }

    #[test]
    fn test_parse_port_invalid() {
        assert_eq!(parse_port("0"), None); // Port 0 is invalid
        assert_eq!(parse_port("99999"), None); // > 65535
        assert_eq!(parse_port("invalid"), None); // Not a number
        assert_eq!(parse_port(""), None); // Empty string
    }
}
