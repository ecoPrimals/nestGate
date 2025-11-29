//! Additional error handling tests for uncovered paths
//!
//! Tests for error propagation, error context, and error recovery patterns.

#[cfg(test)]
mod error_handling_tests {
    use crate::error::NestGateError;
    use crate::Result;

    #[test]
    fn test_config_error_creation() {
        let error = NestGateError::config_error("Invalid configuration", None);
        assert!(format!("{:?}", error).contains("config"));
    }

    #[test]
    fn test_network_error_creation() {
        let error = NestGateError::network_error("Connection failed");
        assert!(format!("{:?}", error).contains("network"));
    }

    #[test]
    fn test_storage_error_creation() {
        let error = NestGateError::storage_error("Disk full");
        assert!(format!("{:?}", error).contains("storage"));
    }

    #[test]
    fn test_error_with_source() {
        let source_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error =
            NestGateError::config_error("Failed to read config", Some(Box::new(source_error)));

        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("config") || debug_str.contains("Failed"));
    }

    #[test]
    fn test_result_type_ok() {
        let result: Result<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_type_err() {
        let result: Result<i32> = Err(NestGateError::config_error("test error", None));
        assert!(result.is_err());
    }

    #[test]
    fn test_error_propagation() {
        /// Inner Function
        fn inner_function() -> Result<i32> {
            Err(NestGateError::network_error("inner error"))
        }

        /// Outer Function
        fn outer_function() -> Result<i32> {
            inner_function()?;
            Ok(42)
        }

        let result = outer_function();
        assert!(result.is_err());
    }

    #[test]
    fn test_error_context_preserved() {
        let error = NestGateError::operation_failed(
            "test operation",
            NestGateError::config_error("root cause", None),
        );

        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("operation") || debug_str.contains("test"));
    }

    #[test]
    fn test_multiple_error_types() {
        let config_err = NestGateError::config_error("config", None);
        let network_err = NestGateError::network_error("network");
        let storage_err = NestGateError::storage_error("storage");

        assert!(format!("{:?}", config_err).len() > 0);
        assert!(format!("{:?}", network_err).len() > 0);
        assert!(format!("{:?}", storage_err).len() > 0);
    }

    #[test]
    fn test_error_display_format() {
        let error = NestGateError::config_error("test message", None);
        let display_str = format!("{}", error);
        assert!(!display_str.is_empty());
    }
}

#[cfg(test)]
mod config_validation_edge_cases {
    use crate::config::validation::*;

    #[test]
    fn test_port_zero_is_invalid() {
        let result = validate_port(0);
        assert!(result.is_err(), "Port 0 should be invalid");
    }

    #[test]
    fn test_port_65535_is_valid() {
        let result = validate_port(65535);
        assert!(result.is_ok(), "Port 65535 should be valid");
    }

    #[test]
    fn test_port_65536_is_invalid() {
        let result = validate_port(65536);
        assert!(result.is_err(), "Port 65536 should be invalid");
    }

    #[test]
    fn test_empty_host_is_invalid() {
        let result = validate_host("");
        assert!(result.is_err(), "Empty host should be invalid");
    }

    #[test]
    fn test_localhost_is_valid() {
        let result = validate_host("localhost");
        assert!(result.is_ok(), "localhost should be valid");
    }

    #[test]
    fn test_ipv4_address_is_valid() {
        let result = validate_host("192.168.1.1");
        assert!(result.is_ok(), "IPv4 address should be valid");
    }

    #[test]
    fn test_bind_address_requires_port() {
        let result = validate_bind_address("127.0.0.1");
        assert!(
            result.is_err(),
            "Bind address without port should be invalid"
        );
    }

    #[test]
    fn test_bind_address_with_port_is_valid() {
        let result = validate_bind_address("127.0.0.1:8080");
        assert!(result.is_ok(), "Valid bind address should be accepted");
    }

    #[test]
    fn test_url_requires_scheme() {
        let result = validate_url("example.com");
        assert!(result.is_err(), "URL without scheme should be invalid");
    }

    #[test]
    fn test_http_url_is_valid() {
        let result = validate_url("http://localhost:8080");
        assert!(result.is_ok(), "HTTP URL should be valid");
    }

    #[test]
    fn test_https_url_is_valid() {
        let result = validate_url("https://api.example.com");
        assert!(result.is_ok(), "HTTPS URL should be valid");
    }
}

#[cfg(test)]
mod network_config_tests {
    use crate::config::network_defaults::*;

    #[test]
    fn test_api_host_returns_string() {
        let host = api_host();
        assert!(!host.is_empty(), "API host should not be empty");
    }

    #[test]
    fn test_api_port_returns_valid_port() {
        let port = api_port();
        assert!(
            port > 0 && port <= 65535,
            "API port should be in valid range"
        );
    }

    #[test]
    fn test_bind_address_format() {
        let bind = api_bind_address();
        assert!(
            bind.contains(":"),
            "Bind address should contain port separator"
        );
    }

    #[test]
    fn test_api_url_format() {
        let url = api_url();
        assert!(
            url.starts_with("http://") || url.starts_with("https://"),
            "API URL should have valid scheme"
        );
    }

    #[test]
    fn test_metrics_port_is_configured() {
        let port = metrics_port();
        assert!(port > 0, "Metrics port should be configured");
    }

    #[test]
    fn test_bind_all_interfaces_format() {
        let bind = bind_all_interfaces(8080);
        assert!(
            bind.starts_with("0.0.0.0:"),
            "Should bind to all interfaces"
        );
        assert!(bind.contains("8080"), "Should contain specified port");
    }

    #[test]
    fn test_bind_localhost_format() {
        let bind = bind_localhost(8080);
        assert!(bind.starts_with("127.0.0.1:"), "Should bind to localhost");
        assert!(bind.contains("8080"), "Should contain specified port");
    }
}

#[cfg(test)]
mod port_config_edge_cases {
    use crate::config::port_config::*;

    #[test]
    fn test_all_ports_positive() {
        let ports = get_all_ports();
        for (_, port) in ports {
            assert!(port > 0, "All ports must be positive");
        }
    }

    #[test]
    fn test_database_ports_standard() {
        assert_eq!(postgres_port(), 5432, "PostgreSQL standard port");
        assert_eq!(redis_port(), 6379, "Redis standard port");
        assert_eq!(mongodb_port(), 27017, "MongoDB standard port");
    }

    #[test]
    fn test_service_ports_non_privileged() {
        // Most services should use non-privileged ports (>1024)
        assert!(api_port() >= 1024, "API should use non-privileged port");
        assert!(
            metrics_port() >= 1024,
            "Metrics should use non-privileged port"
        );
    }

    #[test]
    fn test_grpc_port_standard() {
        assert_eq!(grpc_port(), 50051, "gRPC standard port");
    }

    #[test]
    fn test_port_consistency() {
        // Calling same function twice should return same value
        assert_eq!(api_port(), api_port());
        assert_eq!(redis_port(), redis_port());
        assert_eq!(postgres_port(), postgres_port());
    }
}
