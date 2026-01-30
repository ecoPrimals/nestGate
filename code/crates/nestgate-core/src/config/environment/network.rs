//! # Network Configuration
//!
//! Network-specific environment configuration extracted for logical cohesion.
//!
//! **Phase 3: Smart Refactoring** - Extracted from monolithic `environment.rs` (Jan 30, 2026)

use super::{ConfigError, Port};
use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;
use std::time::Duration;

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Port to bind to (default: 8080)
    pub port: Port,

    /// Host address to bind to (default: 127.0.0.1)
    pub host: String,

    /// Connection timeout in seconds (default: 30)
    pub timeout_secs: u64,

    /// Maximum concurrent connections (default: 1000)
    pub max_connections: usize,

    /// Read timeout in seconds (default: 10)
    pub read_timeout_secs: u64,

    /// Write timeout in seconds (default: 10)
    pub write_timeout_secs: u64,

    /// Keep-alive timeout in seconds (default: 60)
    pub keepalive_secs: u64,
}

impl NetworkConfig {
    /// Load from environment with NESTGATE_ prefix
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_env_with_prefix("NESTGATE")
    }

    /// Load from environment with custom prefix
    pub fn from_env_with_prefix(prefix: &str) -> Result<Self, ConfigError> {
        Ok(Self {
            port: Self::env_var_or(prefix, "PORT", Port::default())?,
            // ✅ Using compile-time constant for default host
            host: Self::env_var_or(prefix, "HOST", std::net::Ipv4Addr::LOCALHOST.to_string())?,
            timeout_secs: Self::env_var_or(prefix, "TIMEOUT_SECS", 30)?,
            max_connections: Self::env_var_or(prefix, "MAX_CONNECTIONS", 1000)?,
            read_timeout_secs: Self::env_var_or(prefix, "READ_TIMEOUT_SECS", 10)?,
            write_timeout_secs: Self::env_var_or(prefix, "WRITE_TIMEOUT_SECS", 10)?,
            keepalive_secs: Self::env_var_or(prefix, "KEEPALIVE_SECS", 60)?,
        })
    }

    /// Helper to get environment variable or use default (public for other modules)
    #[doc(hidden)]
    pub fn env_var_or<T: FromStr>(prefix: &str, key: &str, default: T) -> Result<T, ConfigError>
    where
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        let var_name = format!("{}_{}", prefix, key);
        match env::var(&var_name) {
            Ok(val) => val.parse().map_err(|e| ConfigError::ParseError {
                key: var_name,
                source: Box::new(e),
            }),
            Err(_) => Ok(default),
        }
    }

    /// Get connection timeout as Duration
    #[must_use]
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_secs)
    }

    /// Get read timeout as Duration
    #[must_use]
    pub fn read_timeout(&self) -> Duration {
        Duration::from_secs(self.read_timeout_secs)
    }

    /// Get write timeout as Duration
    #[must_use]
    pub fn write_timeout(&self) -> Duration {
        Duration::from_secs(self.write_timeout_secs)
    }

    /// Get keepalive timeout as Duration
    #[must_use]
    pub fn keepalive(&self) -> Duration {
        Duration::from_secs(self.keepalive_secs)
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            port: Port::default(),
            host: "127.0.0.1".to_string(),
            timeout_secs: 30,
            max_connections: 1000,
            read_timeout_secs: 10,
            write_timeout_secs: 10,
            keepalive_secs: 60,
        }
    }
}
