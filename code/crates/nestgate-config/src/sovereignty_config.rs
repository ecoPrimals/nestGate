// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// Sovereignty Configuration Helpers
//! Sovereignty Config functionality and utilities.
// These helpers ensure all infrastructure assumptions are user-configurable

use std::env;

use nestgate_types::error::utilities::safe_env_var_or_default;

/// Configuration for Sovereignty
pub struct SovereigntyConfig;

impl SovereigntyConfig {
    /// Get API endpoint respecting user sovereignty
    ///
    /// **IMPORTANT**: Returns environment variable value only. No hardcoded defaults
    /// are used as that would violate sovereignty principles.
    ///
    /// # Errors
    ///
    /// Returns an error if `NESTGATE_API_ENDPOINT` is not set. This forces explicit
    /// configuration and makes missing configuration visible.
    ///
    /// # Migration from previous version
    ///
    /// This method previously panicked. Now it returns `Result` for proper error handling.
    ///
    /// ```rust,ignore
    /// // OLD (panicked on missing env var):
    /// let endpoint = SovereigntyConfig::api_endpoint();
    ///
    /// // NEW (proper error handling):
    /// let endpoint = SovereigntyConfig::api_endpoint()
    ///     .map_err(|e| MyError::Configuration(e))?;
    /// ```
    pub fn api_endpoint() -> Result<String, String> {
        env::var("NESTGATE_API_ENDPOINT").map_err(|_| {
            "NESTGATE_API_ENDPOINT must be set explicitly - no hardcoded defaults for sovereignty"
                .to_string()
        })
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
    ///
    /// ✅ MIGRATED: Now uses centralized get_api_port() function
    pub fn api_port() -> u16 {
        // Use centralized environment-driven configuration
        crate::constants::get_api_port()
    }

    /// Get WebSocket endpoint respecting user sovereignty
    ///
    /// Returns environment variable value only. No hardcoded defaults.
    ///
    /// # Errors
    ///
    /// Returns an error if `NESTGATE_WS_ENDPOINT` is not set. This forces explicit
    /// configuration and makes missing configuration visible.
    pub fn websocket_endpoint() -> Result<String, String> {
        env::var("NESTGATE_WS_ENDPOINT").map_err(|_| {
            "NESTGATE_WS_ENDPOINT must be set explicitly - no hardcoded defaults for sovereignty"
                .to_string()
        })
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
    /// Returns an error if sovereignty principles are violated:
    /// - API endpoint uses localhost without explicit configuration
    /// - Required environment variables are missing
    pub fn validate_sovereignty() -> Result<(), String> {
        // Check that API endpoint is explicitly configured
        let api_endpoint = Self::api_endpoint()?; // Propagate error instead of panic

        if api_endpoint.contains("localhost") && env::var("NESTGATE_API_ENDPOINT").is_err() {
            return Err(
                "API endpoint using localhost without explicit user configuration".to_string(),
            );
        }

        Ok(())
    }
}
