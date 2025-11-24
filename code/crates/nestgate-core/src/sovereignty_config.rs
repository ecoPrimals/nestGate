// Sovereignty Configuration Helpers
//! Sovereignty Config functionality and utilities.
// These helpers ensure all infrastructure assumptions are user-configurable

use std::env;

use crate::error::utilities::safe_env_var_or_default;

pub struct SovereigntyConfig;

impl SovereigntyConfig {
    /// Get API endpoint respecting user sovereignty
    pub fn api_endpoint() -> String {
        let default_url = crate::constants::canonical_defaults::network::build_api_url();
        safe_env_var_or_default("NESTGATE_API_ENDPOINT", &default_url).to_string()
    }

    /// Get bind address respecting user sovereignty
    pub fn bind_address() -> String {
        safe_env_var_or_default(
            "NESTGATE_BIND_ADDRESS",
            crate::constants::canonical_defaults::network::DEFAULT_BIND_ADDRESS,
        )
        .to_string()
    }

    /// Get API port respecting user sovereignty
    pub fn api_port() -> u16 {
        env::var("NESTGATE_PORT")
            .unwrap_or_else(|_| {
                crate::constants::canonical_defaults::network::DEFAULT_API_PORT.to_string()
            })
            .parse()
            .unwrap_or(crate::constants::canonical_defaults::network::DEFAULT_API_PORT)
    }

    /// Get WebSocket endpoint respecting user sovereignty
    pub fn websocket_endpoint() -> String {
        let default_url = crate::constants::canonical_defaults::network::build_websocket_url();
        safe_env_var_or_default("NESTGATE_WS_ENDPOINT", &default_url).to_string()
    }

    /// Get database URL respecting user sovereignty
    pub fn database_url() -> String {
        let default_url = format!(
            "postgresql://{}:{}/nestgate",
            safe_env_var_or_default("NESTGATE_DB_HOST", "localhost"),
            safe_env_var_or_default("NESTGATE_DB_PORT", "5432")
        );
        safe_env_var_or_default("NESTGATE_DATABASE_URL", &default_url).to_string()
    }

    /// Get service discovery endpoint respecting user sovereignty
    pub fn discovery_endpoint() -> String {
        let default_endpoint = format!(
            "http://{}:{}/discovery",
            Self::bind_address(),
            Self::api_port() + 3 // Discovery typically on api_port + 3
        );
        safe_env_var_or_default("NESTGATE_DISCOVERY_ENDPOINT", &default_endpoint).to_string()
    }

    /// Validate that all sovereignty requirements are met
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate_sovereignty() -> Result<(), String> {
        // Check that no hardcoded infrastructure assumptions are being made
        let api_endpoint = Self::api_endpoint();
        if api_endpoint.contains("localhost") && env::var("NESTGATE_API_ENDPOINT").is_err() {
            return Err(
                "API endpoint using localhost without explicit user configuration".to_string(),
            );
        }

        Ok(())
    }
}
