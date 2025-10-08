//! **NETWORK CONSTANTS**
//!
//! Network-related constants and default values.

/// Default API port for `NestGate` services
pub const DEFAULT_API_PORT: u16 = 8080;

/// Default metrics port
pub const DEFAULT_METRICS_PORT: u16 = 9090;

/// Default Grafana port
pub const DEFAULT_GRAFANA_PORT: u16 = 3000;

/// Default health check port
pub const DEFAULT_HEALTH_PORT: u16 = 8082;

/// Default database port (`PostgreSQL`)
pub const DEFAULT_DB_PORT: u16 = 5432;

/// Default Redis port
pub const DEFAULT_REDIS_PORT: u16 = 6379;

/// Default localhost address
pub const LOCALHOST: &str = "127.0.0.1";

/// Default bind address
pub const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0";
