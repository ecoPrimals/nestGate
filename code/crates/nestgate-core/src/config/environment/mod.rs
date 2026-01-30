//! Modern environment-driven configuration system
//!
//! This module provides a unified, type-safe configuration system that loads
//! settings from environment variables with sensible defaults. It follows modern
//! Rust patterns including builder pattern, newtype wrappers, and comprehensive
//! error handling.
//!
//! **Phase 3: Smart Refactoring** - Modularized from 883-line monolith (Jan 30, 2026)
//!
//! ## Module Structure
//!
//! - `mod.rs`: Main config + Port type (~150 lines)
//! - `network.rs`: Network configuration (~95 lines)
//! - `storage.rs`: Storage configuration (~75 lines)
//! - `discovery.rs`: Discovery configuration (~80 lines)
//! - `monitoring.rs`: Monitoring configuration (~78 lines)
//! - `security.rs`: Security configuration (~80 lines)
//!
//! **Total**: ~558 lines across 6 focused modules (vs 883 in 1 file)
//! **Reduction**: 37% more maintainable
//!
//! # Examples
//!
//! ```no_run
//! use nestgate_core::config::environment::EnvironmentConfig;
//!
//! // Load configuration from environment
//! let config = EnvironmentConfig::from_env().expect("Failed to load config");
//!
//! // Access configuration values
//! println!("API listening on {}:{}", config.network.host, config.network.port);
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

// Domain-specific configuration modules
pub mod discovery;
pub mod monitoring;
pub mod network;
pub mod security;
pub mod storage;

// Re-export for convenience
pub use discovery::DiscoveryConfig;
pub use monitoring::MonitoringConfig;
pub use network::NetworkConfig;
pub use security::SecurityConfig;
pub use storage::StorageConfig;

/// Errors that can occur during configuration loading
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// Environment variable was not found
    #[error("Required environment variable '{0}' not found")]
    MissingEnvVar(String),

    /// Failed to parse environment variable value
    #[error("Failed to parse environment variable '{key}': {source}")]
    ParseError {
        /// The environment variable key
        key: String,
        /// The underlying parse error
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Invalid configuration value
    #[error("Invalid configuration: {0}")]
    Invalid(String),

    /// Port number out of valid range
    #[error("Invalid port {0}: must be between 1024 and 65535")]
    InvalidPort(u16),

    /// I/O error during configuration loading
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Centralized environment configuration
///
/// This is the main entry point for all application configuration. It loads
/// settings from environment variables with the `NESTGATE_` prefix.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentConfig {
    /// Network configuration
    pub network: NetworkConfig,

    /// Storage configuration
    pub storage: StorageConfig,

    /// Service discovery configuration
    pub discovery: DiscoveryConfig,

    /// Monitoring and observability configuration
    pub monitoring: MonitoringConfig,

    /// Security configuration
    pub security: SecurityConfig,
}

impl EnvironmentConfig {
    /// Load configuration from environment variables
    ///
    /// Looks for variables with the `NESTGATE_` prefix.
    ///
    /// # Errors
    ///
    /// Returns error if required environment variables are missing or invalid
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            network: NetworkConfig::from_env()?,
            storage: StorageConfig::from_env()?,
            discovery: DiscoveryConfig::from_env()?,
            monitoring: MonitoringConfig::from_env()?,
            security: SecurityConfig::from_env()?,
        })
    }

    /// Load configuration with custom prefix
    ///
    /// # Errors
    ///
    /// Returns error if required environment variables are missing or invalid
    pub fn from_env_with_prefix(prefix: &str) -> Result<Self, ConfigError> {
        Ok(Self {
            network: NetworkConfig::from_env_with_prefix(prefix)?,
            storage: StorageConfig::from_env_with_prefix(prefix)?,
            discovery: DiscoveryConfig::from_env_with_prefix(prefix)?,
            monitoring: MonitoringConfig::from_env_with_prefix(prefix)?,
            security: SecurityConfig::from_env_with_prefix(prefix)?,
        })
    }
}

/// Validated port number (1024-65535)
///
/// Enforces valid port range at construction time for type safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Port(u16);

impl Port {
    /// Create a new Port, validating the range
    ///
    /// # Errors
    ///
    /// Returns error if port is outside valid range (1024-65535)
    pub fn new(port: u16) -> Result<Self, ConfigError> {
        if (1024..=65535).contains(&port) {
            Ok(Self(port))
        } else {
            Err(ConfigError::InvalidPort(port))
        }
    }

    /// Create a port without validation (for const contexts)
    ///
    /// # Safety
    ///
    /// Caller must ensure port is in valid range (1024-65535)
    #[must_use]
    pub const fn new_unchecked(port: u16) -> Self {
        Self(port)
    }

    /// Get the inner port value
    #[must_use]
    pub const fn get(self) -> u16 {
        self.0
    }
}

impl Default for Port {
    fn default() -> Self {
        Self(8080)
    }
}

impl FromStr for Port {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let port: u16 = s
            .parse()
            .map_err(|e| ConfigError::ParseError {
                key: "port".to_string(),
                source: Box::new(e),
            })?;
        Self::new(port)
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
