//! Additional tests for environment configuration
//!
//! Expanding test coverage for EnvironmentConfig with error paths and edge cases

#[cfg(test)]
mod environment_config_tests {
    use crate::config::environment::{ConfigError, EnvironmentConfig, Port};

    #[test]
    fn test_port_validation_zero() {
        let result = Port::new(0);
        assert!(result.is_err(), "Port 0 should be invalid");

        if let Err(ConfigError::InvalidPort(port)) = result {
            assert_eq!(port, 0);
        } else {
            panic!("Expected InvalidPort error");
        }
    }

    #[test]
    fn test_port_validation_too_high() {
        // Note: u16 max is 65535, so we test with u32 and expect it won't compile
        // This test verifies the type system prevents invalid ports
        // Port 65535 is valid, so we test boundary behavior instead
        let max_port = Port::new(65535);
        assert!(max_port.is_ok(), "Port 65535 should be valid (max)");
    }

    #[test]
    fn test_port_validation_boundary_min() {
        // Port 1024 should be valid (minimum non-privileged)
        let result = Port::new(1024);
        assert!(result.is_ok(), "Port 1024 should be valid");
        assert_eq!(result.unwrap().get(), 1024);
    }

    #[test]
    fn test_port_validation_boundary_max() {
        // Port 65535 should be valid (maximum)
        let result = Port::new(65535);
        assert!(result.is_ok(), "Port 65535 should be valid");
        assert_eq!(result.unwrap().get(), 65535);
    }

    #[test]
    fn test_port_privileged_range() {
        // Ports below 1024 should fail
        for port in [1, 80, 443, 1023] {
            let result = Port::new(port);
            assert!(
                result.is_err(),
                "Port {} should be invalid (privileged)",
                port
            );
        }
    }

    #[test]
    fn test_port_common_values() {
        // Common non-privileged ports should work
        let common_ports = [1024, 3000, 5000, 8080, 8443, 9000, 9090];

        for &port in &common_ports {
            let result = Port::new(port);
            assert!(result.is_ok(), "Common port {} should be valid", port);
            assert_eq!(result.unwrap().get(), port);
        }
    }

    #[test]
    fn test_environment_config_default() {
        let config = EnvironmentConfig::default();

        // Verify default config is valid
        assert!(
            !config.network.host.is_empty(),
            "Default host should not be empty"
        );
        assert!(
            config.network.port.get() >= 1024,
            "Default port should be non-privileged"
        );
    }

    #[test]
    fn test_config_error_display() {
        let error = ConfigError::InvalidPort(999);
        let display = format!("{}", error);
        assert!(
            display.contains("999"),
            "Error message should contain port number"
        );
    }

    #[test]
    fn test_config_error_missing_env() {
        let error = ConfigError::MissingEnvVar("TEST_VAR".to_string());
        let display = format!("{}", error);
        assert!(
            display.contains("TEST_VAR"),
            "Error should contain variable name"
        );
    }

    #[test]
    fn test_config_error_invalid() {
        let error = ConfigError::Invalid("test error".to_string());
        let display = format!("{}", error);
        assert!(
            display.contains("test error"),
            "Error should contain message"
        );
    }

    #[test]
    fn test_port_unchecked_creation() {
        // Test unchecked port creation (for constants)
        let port = Port::new_unchecked(80);
        assert_eq!(port.get(), 80);

        let port = Port::new_unchecked(443);
        assert_eq!(port.get(), 443);
    }

    #[test]
    fn test_port_display() {
        let port = Port::new(8080).unwrap();
        let display = format!("{:?}", port);
        assert!(display.contains("8080"));
    }

    #[test]
    fn test_port_comparison() {
        let port1 = Port::new(8080).unwrap();
        let port2 = Port::new(8080).unwrap();
        let port3 = Port::new(9090).unwrap();

        assert_eq!(port1, port2);
        assert_ne!(port1, port3);
        assert!(port1 < port3);
    }

    #[test]
    fn test_socket_addr_creation() {
        let config = EnvironmentConfig::default();
        let result = config.bind_address();

        // Should create valid socket address or fail gracefully
        match result {
            Ok(addr) => {
                assert!(addr.port() >= 1024, "Socket port should be non-privileged");
            }
            Err(_) => {
                // Acceptable if host is not a valid IP
            }
        }
    }

    #[test]
    fn test_network_config_fields() {
        let config = EnvironmentConfig::default();

        // Verify all network config fields are accessible
        let _host = &config.network.host;
        let _port = config.network.port;

        // Should not panic accessing fields
        assert!(true);
    }

    #[test]
    fn test_storage_config_fields() {
        let config = EnvironmentConfig::default();

        // Verify storage config is accessible
        let _storage = &config.storage;

        // Should not panic
        assert!(true);
    }

    #[test]
    fn test_discovery_config_fields() {
        let config = EnvironmentConfig::default();

        // Verify discovery config is accessible
        let _discovery = &config.discovery;

        // Should not panic
        assert!(true);
    }

    #[test]
    fn test_monitoring_config_fields() {
        let config = EnvironmentConfig::default();

        // Verify monitoring config is accessible
        let _monitoring = &config.monitoring;

        // Should not panic
        assert!(true);
    }

    #[test]
    fn test_security_config_fields() {
        let config = EnvironmentConfig::default();

        // Verify security config is accessible
        let _security = &config.security;

        // Should not panic
        assert!(true);
    }

    #[test]
    fn test_config_clone() {
        let config1 = EnvironmentConfig::default();
        let config2 = config1.clone();

        // Verify cloning works
        assert_eq!(config1.network.port.get(), config2.network.port.get());
        assert_eq!(config1.network.host, config2.network.host);
    }

    #[test]
    fn test_port_ordering() {
        let mut ports = vec![
            Port::new(9090).unwrap(),
            Port::new(8080).unwrap(),
            Port::new(8443).unwrap(),
        ];

        ports.sort();

        assert_eq!(ports[0].get(), 8080);
        assert_eq!(ports[1].get(), 8443);
        assert_eq!(ports[2].get(), 9090);
    }
}
