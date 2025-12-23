//! Database configuration module
//!
//! Provides configuration for database connection pooling and credentials.

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::env;

/// Database configuration for connection management.
///
/// # Environment Variables
///
/// - `NESTGATE_DB_HOST` - Database host (default: "localhost")
/// - `NESTGATE_DB_PORT` - Database port (default: 5432)
/// - `NESTGATE_DB_NAME` - Database name (default: "nestgate")
/// - `NESTGATE_DB_USER` - Database user (default: "nestgate")
/// - `NESTGATE_DB_PASSWORD` - Database password (default: "")
/// - `NESTGATE_DB_POOL_SIZE` - Connection pool size (default: 10)
/// - `NESTGATE_DB_MAX_CONNECTIONS` - Max connections (default: 100)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database host
    pub host: String,

    /// Database port
    pub port: u16,

    /// Database name
    pub name: String,

    /// Database user
    pub user: String,

    /// Database password (sensitive)
    #[serde(skip_serializing)]
    pub password: String,

    /// Connection pool size
    pub pool_size: usize,

    /// Maximum connections
    pub max_connections: usize,
}

impl DatabaseConfig {
    /// Load database configuration from environment variables.
    pub fn from_environment() -> Result<Self> {
        Ok(Self {
            host: env::var("NESTGATE_DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("NESTGATE_DB_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5432),
            name: env::var("NESTGATE_DB_NAME").unwrap_or_else(|_| "nestgate".to_string()),
            user: env::var("NESTGATE_DB_USER").unwrap_or_else(|_| "nestgate".to_string()),
            password: env::var("NESTGATE_DB_PASSWORD").unwrap_or_default(),
            pool_size: env::var("NESTGATE_DB_POOL_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            max_connections: env::var("NESTGATE_DB_MAX_CONNECTIONS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
        })
    }

    /// Get database connection URL.
    #[must_use]
    pub fn connection_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            name: "nestgate".to_string(),
            user: "nestgate".to_string(),
            password: String::new(),
            pool_size: 10,
            max_connections: 100,
        }
    }
}
