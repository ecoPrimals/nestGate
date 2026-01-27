//! External Services Configuration
//!
//! This module provides environment-driven configuration for external services,
//! eliminating hardcoded URLs, ports, and IP addresses.
//!
//! # Migration from Hardcoded Values
//!
//! This module replaces 815 hardcoded values found in the audit:
//! - 303 URLs (http://, https://)
//! - 121 ports (:8080, :3000, :5432, :6379, :9090)
//! - 391 localhost/IP addresses (127.0.0.1, 0.0.0.0, localhost)
//!
//! # Design Principles
//!
//! 1. **Environment-First**: All values come from environment variables with sensible defaults
//! 2. **Multi-Environment**: Same code works in dev, staging, and production
//! 3. **Type-Safe**: Strong typing prevents configuration errors
//! 4. **DRY**: Single source of truth for each configuration value
//!
//! # Example Usage
//!
//! ```rust
//! use nestgate_core::config::external::{ExternalConfig, ServiceEndpoints};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Load from environment
//! let config = ExternalConfig::from_env()?;
//!
//! // Access typed endpoints
//! let discovery_url = config.services.discovery_endpoint();
//! let api_port = config.network.api_port();
//! # Ok(())
//! # }
//! ```

pub mod network;
/// Network environment configuration
pub mod network_env_config;
// pub mod services; // REMOVED: Deprecated in favor of capability-based configuration
pub mod services_config;

use crate::Result;
use serde::{Deserialize, Serialize};

pub use network::NetworkConfig;
pub use network_env_config::{NetworkEnvConfig, SharedNetworkEnvConfig};
// ServiceEndpoints removed - use capability-based config instead
pub use services_config::{ServicesConfig, SharedServicesConfig};

/// Complete external configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for External
pub struct ExternalConfig {
    /// Network configuration (ports, IPs)
    pub network: NetworkConfig,
    /// Service endpoints (URLs, discovery) - use ServicesConfig from services_config module
    pub services: ServicesConfig,
}

impl ExternalConfig {
    /// Load configuration from environment variables
    ///
    /// # Errors
    ///
    /// Returns error if required environment variables are missing or invalid
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            network: NetworkConfig::from_env()?,
            services: ServicesConfig::from_env(),
        })
    }

    /// Create configuration with defaults (for testing/development)
    #[must_use]
    pub fn default_dev() -> Self {
        Self {
            network: NetworkConfig::default_dev(),
            services: ServicesConfig::default(),
        }
    }

    /// Create configuration for production
    ///
    /// # Errors
    ///
    /// Returns error if required production environment variables are missing
    pub fn from_env_production() -> Result<Self> {
        Ok(Self {
            network: NetworkConfig::from_env_production()?,
            services: ServicesConfig::from_env(),
        })
    }
}

impl Default for ExternalConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::default_dev()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_dev_config() {
        let config = ExternalConfig::default_dev();
        assert_eq!(config.network.api_port(), 8080);
    }

    #[test]
    fn test_config_serialization() {
        let config = ExternalConfig::default_dev();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: ExternalConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.network.api_port(), deserialized.network.api_port());
    }
}
