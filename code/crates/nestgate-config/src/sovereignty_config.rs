// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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

    /// Get bind address respecting user sovereignty
    #[must_use]
    pub fn bind_address() -> String {
        safe_env_var_or_default(
            "NESTGATE_BIND_ADDRESS",
            crate::constants::canonical_defaults::network::DEFAULT_BIND_ADDRESS,
        )
    }

    /// Get API port respecting user sovereignty
    ///
    /// MIGRATED: Now uses centralized `get_api_port()` function
    #[must_use]
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

    /// Get database URL respecting user sovereignty
    #[must_use]
    pub fn database_url() -> String {
        let default_url = format!(
            "postgresql://{}:{}/nestgate",
            // Using compile-time constant for default
            safe_env_var_or_default(
                "NESTGATE_DB_HOST",
                &std::net::Ipv4Addr::LOCALHOST.to_string()
            ),
            safe_env_var_or_default("NESTGATE_DB_PORT", "5432")
        );
        safe_env_var_or_default("NESTGATE_DATABASE_URL", &default_url)
    }

    /// Get service discovery endpoint respecting user sovereignty
    #[must_use]
    pub fn discovery_endpoint() -> String {
        let default_endpoint = format!(
            "http://{}:{}/discovery",
            Self::bind_address(),
            Self::api_port() + 3 // Discovery typically on api_port + 3
        );
        safe_env_var_or_default("NESTGATE_DISCOVERY_ENDPOINT", &default_endpoint)
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
            return Err(String::from(
                "API endpoint using localhost without explicit user configuration",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn api_endpoint_errors_when_unset() {
        temp_env::with_vars([("NESTGATE_API_ENDPOINT", None::<&str>)], || {
            assert!(SovereigntyConfig::api_endpoint().is_err());
            let err = SovereigntyConfig::api_endpoint().unwrap_err();
            assert!(
                err.contains("sovereignty"),
                "error should mention sovereignty"
            );
        });
    }

    #[test]
    #[serial]
    fn api_endpoint_returns_env_value() {
        temp_env::with_vars(
            [("NESTGATE_API_ENDPOINT", Some("http://10.0.0.5:8443"))],
            || {
                assert_eq!(
                    SovereigntyConfig::api_endpoint().unwrap(),
                    "http://10.0.0.5:8443"
                );
            },
        );
    }

    #[test]
    #[serial]
    fn websocket_endpoint_errors_when_unset() {
        temp_env::with_vars([("NESTGATE_WS_ENDPOINT", None::<&str>)], || {
            assert!(SovereigntyConfig::websocket_endpoint().is_err());
        });
    }

    #[test]
    #[serial]
    fn websocket_endpoint_returns_env_value() {
        temp_env::with_vars([("NESTGATE_WS_ENDPOINT", Some("ws://gate:9000"))], || {
            assert_eq!(
                SovereigntyConfig::websocket_endpoint().unwrap(),
                "ws://gate:9000"
            );
        });
    }

    #[test]
    fn bind_address_has_default() {
        let addr = SovereigntyConfig::bind_address();
        assert!(!addr.is_empty());
    }

    #[test]
    fn discovery_endpoint_constructs_url() {
        let endpoint = SovereigntyConfig::discovery_endpoint();
        assert!(endpoint.starts_with("http://"));
        assert!(endpoint.ends_with("/discovery"));
    }

    #[test]
    #[serial]
    fn discovery_endpoint_respects_env() {
        temp_env::with_vars(
            [("NESTGATE_DISCOVERY_ENDPOINT", Some("http://custom:7703/d"))],
            || {
                assert_eq!(
                    SovereigntyConfig::discovery_endpoint(),
                    "http://custom:7703/d"
                );
            },
        );
    }

    #[test]
    #[serial]
    fn database_url_composes_default() {
        temp_env::with_vars(
            [
                ("NESTGATE_DATABASE_URL", None::<&str>),
                ("NESTGATE_DB_HOST", None::<&str>),
                ("NESTGATE_DB_PORT", None::<&str>),
            ],
            || {
                let url = SovereigntyConfig::database_url();
                assert!(url.starts_with("postgresql://127.0.0.1:5432/nestgate"));
            },
        );
    }

    #[test]
    #[serial]
    fn database_url_respects_override() {
        temp_env::with_vars(
            [("NESTGATE_DATABASE_URL", Some("postgresql://db:5433/custom"))],
            || {
                assert_eq!(
                    SovereigntyConfig::database_url(),
                    "postgresql://db:5433/custom"
                );
            },
        );
    }

    #[test]
    #[serial]
    fn validate_sovereignty_fails_without_endpoint() {
        temp_env::with_vars([("NESTGATE_API_ENDPOINT", None::<&str>)], || {
            assert!(SovereigntyConfig::validate_sovereignty().is_err());
        });
    }

    #[test]
    fn api_port_returns_valid_port() {
        let port = SovereigntyConfig::api_port();
        assert!(port > 0);
    }
}
