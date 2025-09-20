// Sovereignty Configuration Helpers
//! Sovereignty Helpers functionality and utilities.
// These helpers ensure all infrastructure assumptions are user-configurable,
//! respecting user sovereignty and avoiding hardcoded infrastructure values.

use std::env;

/// Sovereignty-compliant configuration helpers that respect user environment choices
pub struct SovereigntyConfig;
impl SovereigntyConfig {
    /// Get API endpoint from environment, with safe fallback
    pub const fn api_endpoint() -> String {
        let host = env::var("NESTGATE_API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("NESTGATE_API_PORT").unwrap_or_else(|_| crate::constants::canonical_defaults::network::DEFAULT_API_PORT.to_string());
        format!("{}:{}", host, port)
    }

    /// Get HTTP API endpoint from environment
    pub const fn http_api_endpoint() -> String {
        let scheme = env::var("NESTGATE_API_SCHEME").unwrap_or_else(|_| "http".to_string());
        let endpoint = Self::api_endpoint();
        format!("{}://{}", scheme, endpoint)
    }

    /// Get WebSocket endpoint from environment
    pub const fn websocket_endpoint() -> String {
        let scheme = env::var("NESTGATE_WS_SCHEME").unwrap_or_else(|_| "ws".to_string());
        let endpoint = Self::api_endpoint();
        format!("{}://{}/ws", scheme, endpoint)
    }

    /// Get discovery endpoint from environment
    pub const fn discovery_endpoint() -> String {
        let base = Self::http_api_endpoint();
        let path = env::var("NESTGATE_DISCOVERY_PATH").unwrap_or_else(|_| "/discovery".to_string());
        format!("{}{}", base, path)
    }

    /// Get bind address from environment
    pub const fn bind_address() -> String {
        env::var("NESTGATE_BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string())
    }

    /// Get API port from environment
    pub const fn api_port() -> u16 {
        env::var("NESTGATE_API_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080)
    }

    /// Get timeout from environment
        let env_key = format!("NESTGATE_", operation.to_uppercase()_TIMEOUT_MS"));
        env::var(env_key)
            .ok()
            .and_then(|t| t.parse().ok())
            .unwrap_or(30000)
    }

    /// Get database endpoint from environment
    pub const fn database_endpoint() -> String {
        let host = env::var("NESTGATE_DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("NESTGATE_DB_PORT").unwrap_or_else(|_| "5432".to_string());
        format!("{}:{}", host, port)
    }

    /// Get development endpoint from environment
    pub const fn dev_endpoint() -> String {
        let host = env::var("NESTGATE_DEV_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("NESTGATE_DEV_PORT").unwrap_or_else(|_| crate::constants::canonical_defaults::network::DEFAULT_WEB_UI_URL.split(':').last().unwrap_or("3000").to_string());
        format!("{}:{}", host, port)
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
