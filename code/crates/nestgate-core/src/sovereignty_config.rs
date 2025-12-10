// Sovereignty Configuration Helpers
//! Sovereignty Config functionality and utilities.
// These helpers ensure all infrastructure assumptions are user-configurable

use std::env;

use crate::error::utilities::safe_env_var_or_default;

/// Configuration for Sovereignty
pub struct SovereigntyConfig;

impl SovereigntyConfig {
    /// Get API endpoint respecting user sovereignty
    ///
    /// **IMPORTANT**: Returns environment variable value only. No hardcoded defaults
    /// are used as that would violate sovereignty principles.
    ///
    /// # Panics
    ///
    /// Panics if `NESTGATE_API_ENDPOINT` is not set. This is intentional to ensure
    /// explicit configuration.
    pub fn api_endpoint() -> String {
        env::var("NESTGATE_API_ENDPOINT").expect(
            "NESTGATE_API_ENDPOINT must be set explicitly - no hardcoded defaults for sovereignty",
        )
    }

    /// Get API endpoint with fallback (for backwards compatibility during migration)
    ///
    /// **DEPRECATED**: Use `api_endpoint()` which requires explicit configuration.
    #[deprecated(
        since = "0.10.0",
        note = "Use api_endpoint() which enforces explicit configuration"
    )]
    pub fn api_endpoint_with_fallback() -> String {
        #[allow(deprecated)]
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
    ///
    /// Returns environment variable value only. No hardcoded defaults.
    pub fn websocket_endpoint() -> String {
        env::var("NESTGATE_WS_ENDPOINT").expect(
            "NESTGATE_WS_ENDPOINT must be set explicitly - no hardcoded defaults for sovereignty",
        )
    }

    /// Get WebSocket endpoint with fallback (backwards compatibility)
    #[deprecated(
        since = "0.10.0",
        note = "Use websocket_endpoint() which enforces explicit configuration"
    )]
    pub fn websocket_endpoint_with_fallback() -> String {
        #[allow(deprecated)]
        let default_url = crate::constants::canonical_defaults::network::build_websocket_url();
        safe_env_var_or_default("NESTGATE_WS_ENDPOINT", &default_url).to_string()
    }

    /// Get database URL respecting user sovereignty
    pub fn database_url() -> String {
        let default_url = format!(
            "postgresql://{}:{}/nestgate",
            // ✅ Using compile-time constant for default
            safe_env_var_or_default(
                "NESTGATE_DB_HOST",
                &std::net::Ipv4Addr::LOCALHOST.to_string()
            ),
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
