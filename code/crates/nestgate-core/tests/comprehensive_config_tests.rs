// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Modern Configuration Tests
//!
//! Comprehensive tests for configuration loading, validation, and environment parsing.
//! Tests proper Result<T, E> error handling patterns.

#![cfg(test)]

use nestgate_core::Result;
use nestgate_core::error::NestGateError;

#[cfg(test)]
mod config_validation_tests {
    use super::*;

    #[tokio::test]
    async fn test_port_validation_zero() {
        // Port 0 should be invalid
        fn validate_port(port: u16) -> Result<()> {
            if port == 0 {
                return Err(NestGateError::validation_error("Port cannot be zero"));
            }
            Ok(())
        }

        let result = validate_port(0);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_port_validation_valid_range() {
        fn validate_port(port: u16) -> Result<()> {
            if port == 0 {
                return Err(NestGateError::validation_error("Port cannot be zero"));
            }
            Ok(())
        }

        for port in [80, 443, 8080, 9090, 65535] {
            let result = validate_port(port);
            assert!(result.is_ok(), "Port {} should be valid", port);
        }
    }

    #[tokio::test]
    async fn test_url_validation() {
        fn validate_url(url: &str) -> Result<()> {
            if !url.starts_with("http://") && !url.starts_with("https://") {
                return Err(NestGateError::validation_error("Invalid URL scheme"));
            }
            Ok(())
        }

        assert!(validate_url("https://api.example.com").is_ok());
        assert!(validate_url("http://localhost:8080").is_ok());
        assert!(validate_url("ftp://example.com").is_err());
    }

    #[tokio::test]
    async fn test_hostname_validation() {
        fn validate_hostname(hostname: &str) -> Result<()> {
            if hostname.is_empty() {
                return Err(NestGateError::validation_error("Hostname cannot be empty"));
            }
            if hostname.contains(' ') {
                return Err(NestGateError::validation_error(
                    "Hostname cannot contain spaces",
                ));
            }
            Ok(())
        }

        assert!(validate_hostname("localhost").is_ok());
        assert!(validate_hostname("api.example.com").is_ok());
        assert!(validate_hostname("").is_err());
        assert!(validate_hostname("invalid hostname").is_err());
    }
}

#[cfg(test)]
mod environment_parsing_tests {
    use std::env;

    #[tokio::test]
    async fn test_env_var_loading() {
        // Test safe environment variable loading pattern
        let var_name = "TEST_CONFIG_VAR";
        nestgate_core::env_process::set_var(var_name, "test_value");

        let value = env::var(var_name).unwrap_or_else(|_| "default".to_string());
        assert_eq!(value, "test_value");

        nestgate_core::env_process::remove_var(var_name);
        let value = env::var(var_name).unwrap_or_else(|_| "default".to_string());
        assert_eq!(value, "default");
    }

    #[tokio::test]
    async fn test_port_env_parsing() {
        let var_name = "TEST_PORT";
        nestgate_core::env_process::set_var(var_name, "8080");

        let port: u16 = env::var(var_name)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3000);
        assert_eq!(port, 8080);

        nestgate_core::env_process::remove_var(var_name);
        let port: u16 = env::var(var_name)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3000);
        assert_eq!(port, 3000);
    }
}

#[cfg(test)]
mod error_handling_patterns {
    use super::*;

    #[tokio::test]
    async fn test_result_propagation() {
        fn inner_function() -> Result<i32> {
            Err(NestGateError::configuration_error("field", "test error"))
        }

        fn outer_function() -> Result<i32> {
            inner_function()?;
            Ok(42)
        }

        let result = outer_function();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_map_err_pattern() {
        fn load_config() -> Result<String> {
            std::fs::read_to_string("/nonexistent/path")
                .map_err(|e| NestGateError::configuration_error("config_file", &e.to_string()))
        }

        let result = load_config();
        assert!(result.is_err());
    }
}
