// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Database configuration module
//!
//! Provides configuration for database connection pooling and credentials.

use nestgate_types::error::Result;
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
    ///
    /// # Errors
    ///
    /// Returns an error if `NESTGATE_DB_HOST` is not set. Database is an external service
    /// and must be explicitly configured - no hardcoded localhost assumption.
    ///
    /// # Philosophy
    ///
    /// External services (databases, Redis) must be explicitly configured.
    /// Hardcoded "localhost" violates sovereignty principles and hides
    /// deployment configuration issues.
    ///
    /// # Security
    ///
    /// Database credentials should NEVER have defaults. Empty password default
    /// is for local development only and should fail in production.
    ///
    /// # Migration
    ///
    /// **Before** (silently used localhost):
    /// ```ignore
    /// let config = DatabaseConfig::from_environment()?;
    /// ```
    ///
    /// **After** (requires explicit config):
    /// ```bash
    /// export NESTGATE_DB_HOST="postgres.internal"
    /// export NESTGATE_DB_PORT="5432"
    /// export NESTGATE_DB_NAME="nestgate"
    /// export NESTGATE_DB_USER="app_user"
    /// export NESTGATE_DB_PASSWORD="secure_password"
    /// ```
    pub fn from_environment() -> Result<Self> {
        let host = env::var("NESTGATE_DB_HOST")
            .map_err(|_| nestgate_types::error::NestGateError::configuration_error(
                "database_host",
                "NESTGATE_DB_HOST must be set explicitly. No hardcoded localhost for external services."
            ))?;

        Ok(Self {
            host,
            port: env::var("NESTGATE_DB_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5432), // Port 5432 is PostgreSQL default (industry standard)
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

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use temp_env::with_vars;

    #[test]
    fn database_config_default() {
        let c = DatabaseConfig::default();
        assert_eq!(c.host, "localhost");
        assert_eq!(c.port, 5432);
        assert_eq!(c.pool_size, 10);
        assert_eq!(c.max_connections, 100);
        assert!(c.password.is_empty());
    }

    #[test]
    fn database_config_connection_url() {
        let c = DatabaseConfig::default();
        let url = c.connection_url();
        assert!(url.contains("postgresql://"));
        assert!(url.contains("localhost"));
        assert!(url.contains("5432"));
        assert!(url.contains("nestgate"));
    }

    #[test]
    #[serial]
    fn database_config_from_environment_requires_host() {
        let res = with_vars(vec![("NESTGATE_DB_HOST", None::<&str>)], || {
            DatabaseConfig::from_environment()
        });
        assert!(res.is_err());
    }

    #[test]
    #[serial]
    fn database_config_from_environment_success() {
        let c = with_vars(
            vec![
                ("NESTGATE_DB_HOST", Some("db.example.com")),
                ("NESTGATE_DB_PORT", Some("5433")),
                ("NESTGATE_DB_NAME", Some("app")),
                ("NESTGATE_DB_USER", Some("user")),
                ("NESTGATE_DB_PASSWORD", Some("secret")),
                ("NESTGATE_DB_POOL_SIZE", Some("5")),
                ("NESTGATE_DB_MAX_CONNECTIONS", Some("50")),
            ],
            || DatabaseConfig::from_environment().expect("from_environment"),
        );
        assert_eq!(c.host, "db.example.com");
        assert_eq!(c.port, 5433);
        assert_eq!(c.name, "app");
        assert_eq!(c.user, "user");
        assert_eq!(c.password, "secret");
        assert_eq!(c.pool_size, 5);
        assert_eq!(c.max_connections, 50);
    }

    #[test]
    fn database_config_serialize_omits_password_field() {
        let original = DatabaseConfig::default();
        let json = serde_json::to_string(&original).expect("serialize DatabaseConfig");
        assert!(!json.contains("password"));
    }

    #[test]
    fn database_config_deserialize_with_explicit_password() {
        let json = r#"{"host":"h","port":5432,"name":"n","user":"u","password":"p","pool_size":3,"max_connections":30}"#;
        let parsed: DatabaseConfig = serde_json::from_str(json).expect("deserialize");
        assert_eq!(parsed.password, "p");
    }

    #[test]
    #[serial]
    fn database_config_from_environment_only_host_uses_defaults_for_optional_fields() {
        let c = with_vars(
            vec![
                ("NESTGATE_DB_HOST", Some("pg.internal")),
                ("NESTGATE_DB_PORT", None::<&str>),
                ("NESTGATE_DB_NAME", None::<&str>),
                ("NESTGATE_DB_USER", None::<&str>),
                ("NESTGATE_DB_PASSWORD", None::<&str>),
                ("NESTGATE_DB_POOL_SIZE", None::<&str>),
                ("NESTGATE_DB_MAX_CONNECTIONS", None::<&str>),
            ],
            || DatabaseConfig::from_environment().expect("from_environment"),
        );
        assert_eq!(c.host, "pg.internal");
        assert_eq!(c.port, 5432);
        assert_eq!(c.name, "nestgate");
        assert_eq!(c.user, "nestgate");
        assert!(c.password.is_empty());
        assert_eq!(c.pool_size, 10);
        assert_eq!(c.max_connections, 100);
    }

    #[test]
    fn database_config_connection_url_includes_credentials() {
        let c = DatabaseConfig {
            host: "db.example.com".to_string(),
            port: 5432,
            name: "app".to_string(),
            user: "appuser".to_string(),
            password: "s3cret".to_string(),
            pool_size: 4,
            max_connections: 40,
        };
        let url = c.connection_url();
        assert_eq!(url, "postgresql://appuser:s3cret@db.example.com:5432/app");
    }
}
