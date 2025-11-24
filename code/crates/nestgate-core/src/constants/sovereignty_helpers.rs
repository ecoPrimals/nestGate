// Sovereignty Configuration Helpers
//! Sovereignty Helpers functionality and utilities.
// These helpers ensure all infrastructure assumptions are user-configurable,
//! respecting user sovereignty and avoiding hardcoded infrastructure values.

// Import the configuration module for concurrent-safe access
use super::sovereignty_helpers_config::SovereigntyHelpersConfig;

/// Sovereignty-compliant configuration helpers that respect user environment choices
pub struct SovereigntyConfig;
impl SovereigntyConfig {
    /// Get API endpoint from environment, with safe fallback
    pub fn api_endpoint() -> String {
        SovereigntyHelpersConfig::from_env().api_endpoint()
    }

    /// Get HTTP API endpoint from environment
    pub fn http_api_endpoint() -> String {
        SovereigntyHelpersConfig::from_env().http_api_endpoint()
    }

    /// Get WebSocket endpoint from environment
    pub fn websocket_endpoint() -> String {
        SovereigntyHelpersConfig::from_env().websocket_endpoint()
    }

    /// Get discovery endpoint from environment
    pub fn discovery_endpoint() -> String {
        SovereigntyHelpersConfig::from_env().discovery_endpoint()
    }

    /// Get bind address from environment
    pub fn bind_address() -> String {
        SovereigntyHelpersConfig::from_env().bind_address()
    }

    /// Get API port from environment
    pub fn api_port() -> u16 {
        SovereigntyHelpersConfig::from_env().api_port()
    }

    /// Get timeout from environment (removed - was broken syntax)
    /// This function had a syntax error and has been removed.
    /// Use crate::constants::system::timeout_ms() instead.
    /// Get database endpoint from environment
    pub fn database_endpoint() -> String {
        SovereigntyHelpersConfig::from_env().database_endpoint()
    }

    /// Get development endpoint from environment
    pub fn dev_endpoint() -> String {
        SovereigntyHelpersConfig::from_env().dev_endpoint()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_sovereignty_config() -> std::result::Result<(), Box<dyn std::error::Error>> {
        // Test default values
        assert!(SovereigntyConfig::api_endpoint().contains("127.0.0.1"));
        assert!(SovereigntyConfig::http_api_endpoint().starts_with("http://"));

        // Test environment override
        env::set_var("NESTGATE_API_HOST", "custom.example.com");
        env::set_var("NESTGATE_API_PORT", "9090");
        assert_eq!(SovereigntyConfig::api_endpoint(), "custom.example.com:9090");

        // Cleanup
        env::remove_var("NESTGATE_API_HOST");
        env::remove_var("NESTGATE_API_PORT");

        Ok(())
    }
}
