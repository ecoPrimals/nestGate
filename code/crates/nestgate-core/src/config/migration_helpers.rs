//! Configuration Migration Helpers
//!
//! Helper functions to safely migrate from hardcoded values to configuration.

use crate::error::{NestGateError, Result};
use std::env;

/// Get configuration value with environment variable fallback
///
/// Tries environment variable first, then falls back to config value,
/// then to default value if provided.
pub fn get_with_fallback<T>(
    env_var: &str,
    config_value: Option<T>,
    default: T,
) -> T
where
    T: std::str::FromStr + Clone,
{
    // Try environment variable first
    if let Ok(val) = env::var(env_var) {
        if let Ok(parsed) = val.parse::<T>() {
            return parsed;
        }
    }

    // Fall back to config value
    if let Some(val) = config_value {
        return val;
    }

    // Use default
    default
}

/// Get port with multiple fallback options
///
/// Priority: ENV var > Config > Default
pub fn get_port(env_var: &str, config_port: Option<u16>, default: u16) -> u16 {
    get_with_fallback(env_var, config_port, default)
}

/// Get host with multiple fallback options
///
/// Priority: ENV var > Config > Default
pub fn get_host(env_var: &str, config_host: Option<String>, default: &str) -> String {
    if let Ok(host) = env::var(env_var) {
        return host;
    }
    config_host.unwrap_or_else(|| default.to_string())
}

/// Build address from host and port
pub fn build_address(host: &str, port: u16) -> Result<String> {
    if host.is_empty() {
        return Err(NestGateError::Configuration(
            "Host cannot be empty".to_string(),
        ));
    }
    Ok(format!("{}:{}", host, port))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_port_with_env() {
        env::set_var("TEST_PORT", "9999");
        let port = get_port("TEST_PORT", Some(8080), 3000);
        assert_eq!(port, 9999);
        env::remove_var("TEST_PORT");
    }

    #[test]
    fn test_get_port_with_config() {
        let port = get_port("NONEXISTENT_PORT", Some(8080), 3000);
        assert_eq!(port, 8080);
    }

    #[test]
    fn test_get_port_with_default() {
        let port = get_port("NONEXISTENT_PORT", None, 3000);
        assert_eq!(port, 3000);
    }

    #[test]
    fn test_build_address() {
        let addr = build_address("localhost", 18080).unwrap();
        assert_eq!(addr, "localhost:18080");
    }

    #[test]
    fn test_build_address_empty_host() {
        let result = build_address("", 8080);
        assert!(result.is_err());
    }
}
