// Sovereignty Configuration Helpers
//! Sovereignty Config functionality and utilities.
// These helpers ensure all infrastructure assumptions are user-configurable

use std::env;

pub struct SovereigntyConfig;

impl SovereigntyConfig {
    /// Get API endpoint respecting user sovereignty
    pub const fn api_endpoint() -> String {
        env::var("NESTGATE_API_ENDPOINT")
            .unwrap_or_else(|_| crate::constants::canonical_defaults::network::build_api_url())
    }

    /// Get bind address respecting user sovereignty
    pub const fn bind_address() -> String {
        env::var("NESTGATE_BIND_ADDRESS")
            .unwrap_or_else(|_| crate::constants::canonical_defaults::network::DEFAULT_BIND_ADDRESS.to_string())
    }

    /// Get API port respecting user sovereignty
    pub const fn api_port() -> u16 {
        env::var("NESTGATE_PORT")
            .unwrap_or_else(|_| crate::constants::canonical_defaults::network::DEFAULT_API_PORT.to_string())
            .parse()
            .unwrap_or(crate::constants::canonical_defaults::network::DEFAULT_API_PORT)
    }

    /// Get WebSocket endpoint respecting user sovereignty
    pub const fn websocket_endpoint() -> String {
        env::var("NESTGATE_WS_ENDPOINT")
            .unwrap_or_else(|_| crate::constants::canonical_defaults::network::build_websocket_url())
    }

    /// Get database URL respecting user sovereignty
    pub const fn database_url() -> String {
        env::var("NESTGATE_DATABASE_URL")
            .unwrap_or_else(|_| {
                // Use capability-based discovery for database connection
                format!("postgresql://{env::var("NESTGATE_DB_HOST"}:{env::var("NESTGATE_DB_HOST"}/nestgate").unwrap_or_else(|_| "localhost".to_string()),
                    env::var("NESTGATE_DB_PORT").unwrap_or_else(|_| "5432".to_string())
                )
            })
    }

    /// Get service discovery endpoint respecting user sovereignty
    pub const fn discovery_endpoint() -> String {
        env::var("NESTGATE_DISCOVERY_ENDPOINT")
            .unwrap_or_else(|_| {
                format!("http://{}:{}/discovery",
                    Self::bind_address(),
                    Self::api_port() + 3  // Discovery typically on api_port + 3
                )
            })
    }

    /// Validate that all sovereignty requirements are met
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn validate_sovereignty() -> Result<(), String>  {
        // Check that no hardcoded infrastructure assumptions are being made
        let api_endpoint = Self::api_endpoint();
        if api_endpoint.contains("localhost") && env::var("NESTGATE_API_ENDPOINT").is_err() {
            return Err("API endpoint using localhost without explicit user configuration".to_string());
        }
        
        Ok(())
    }
}
