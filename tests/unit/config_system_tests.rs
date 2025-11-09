//! **COMPREHENSIVE CONFIG SYSTEM TESTS**
//!
//! Unit tests for the canonical configuration system to achieve 50% coverage target

use nestgate_core::config::{
    canonical_primary::NestGateCanonicalConfig,
    defaults::{Environment, ConfigDefaults},
    network::NetworkConfig,
};
use std::env;
use std::collections::HashMap;

/// **CANONICAL CONFIG TESTS**
#[cfg(test)]
mod canonical_config_tests {
    use super::*;

    #[test]
    fn test_canonical_config_creation() -> Result<(), Box<dyn std::error::Error>> {
        let config = NestGateNestGateCanonicalConfig::default();
        assert!(!config.system.instance_name.is_empty());
        assert!(config.system.version.major > 0);
    Ok(())
    }

    #[test]
    fn test_environment_specific_configs() -> Result<(), Box<dyn std::error::Error>> {
        let dev_config = NestGateCanonicalConfig::for_environment(Environment::Development);
        let prod_config = NestGateCanonicalConfig::for_environment(Environment::Production);
        
        // Development and production should have different characteristics
        assert_ne!(dev_config.system.environment, prod_config.system.environment);
    Ok(())
    }

    #[test]
    fn test_config_validation() -> Result<(), Box<dyn std::error::Error>> {
        let config = NestGateNestGateCanonicalConfig::default();
        let validation_result = config.validate();
        assert!(validation_result.is_ok(), "Default config should be valid");
    Ok(())
    }

    #[test]
    fn test_config_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let config = NestGateNestGateCanonicalConfig::default();
        
        // Test TOML serialization
        let toml_str = toml::to_string(&config)?;
        let deserialized: NestGateCanonicalConfig = toml::from_str(&toml_str)
            ?;
        
        assert_eq!(config.system.instance_name, deserialized.system.instance_name);
    Ok(())
}
}

/// **NETWORK CONFIG TESTS**
#[cfg(test)]
mod network_config_tests {
    use super::*;

    #[test]
    fn test_network_config_defaults() -> Result<(), Box<dyn std::error::Error>> {
        let config = NetworkConfig::default();
        assert!(!config.host.is_empty());
        assert!(config.port > 0);
        assert!(config.websocket_port > 0);
    Ok(())
    }

    #[test]
    fn test_network_config_environment_override() -> Result<(), Box<dyn std::error::Error>> {
        // Set test environment variables
        env::set_var("NESTGATE_NETWORK_HOST", "test.example.com");
        env::set_var("NESTGATE_NETWORK_PORT", "9090");
        
        let config = NetworkConfig::default();
        assert_eq!(config.host, "test.example.com");
        assert_eq!(config.port, 9090);
        
        // Cleanup
        env::remove_var("NESTGATE_NETWORK_HOST");
        env::remove_var("NESTGATE_NETWORK_PORT");
    Ok(())
    }

    #[test]
    fn test_network_config_validation() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = NetworkConfig::default();
        
        // Test valid configuration
        assert!(config.validate().is_ok());
        
        // Test invalid port
        config.port = 0;
        assert!(config.validate().is_err());
        
        // Test invalid host
        config.host = "".to_string();
        config.port = 8080; // Reset to valid
        assert!(config.validate().is_err());
    Ok(())
    }

    #[test]
    fn test_network_config_url_generation() -> Result<(), Box<dyn std::error::Error>> {
        let config = NetworkConfig {
            host: "testhost.example.com".to_string(),
            port: nestgate_core::constants::DEFAULT_API_PORT,
            websocket_port: 8081,
            max_connections: 1000,
            timeout_seconds: 30,
            keep_alive: true,
            compression: true,
            tls_enabled: false,
        };
        
        let api_url = config.get_api_url();
        assert!(api_url.contains("testhost.example.com:8080"));
        
        let ws_url = config.get_websocket_url();
        assert!(ws_url.contains("testhost.example.com:8081"));
    Ok(())
}
}

/// **ENVIRONMENT CONFIG TESTS**
#[cfg(test)]
mod environment_config_tests {
    use super::*;

    #[test]
    fn test_environment_detection() -> Result<(), Box<dyn std::error::Error>> {
        // Test development environment
        env::set_var("NESTGATE_ENV", "development");
        let env_type = Environment::from_env();
        assert_eq!(env_type, Environment::Development);
        
        // Test production environment
        env::set_var("NESTGATE_ENV", "production");
        let env_type = Environment::from_env();
        assert_eq!(env_type, Environment::Production);
        
        // Cleanup
        env::remove_var("NESTGATE_ENV");
    Ok(())
    }

    #[test]
    fn test_environment_defaults() -> Result<(), Box<dyn std::error::Error>> {
        let dev_defaults = ConfigDefaults::for_environment(Environment::Development);
        let prod_defaults = ConfigDefaults::for_environment(Environment::Production);
        
        // Development and production should have different defaults
        assert_ne!(dev_defaults.log_level, prod_defaults.log_level);
        assert_ne!(dev_defaults.debug_enabled, prod_defaults.debug_enabled);
    Ok(())
    }

    #[test]
    fn test_environment_config_loading() -> Result<(), Box<dyn std::error::Error>> {
        // Test loading config for specific environment
        env::set_var("NESTGATE_ENV", "development");
        env::set_var("NESTGATE_DEBUG", "true");
        env::set_var("NESTGATE_LOG_LEVEL", "debug");
        
        let config = NestGateCanonicalConfig::from_environment();
        assert!(config.system.debug_enabled);
        
        // Cleanup
        env::remove_var("NESTGATE_ENV");
        env::remove_var("NESTGATE_DEBUG");
        env::remove_var("NESTGATE_LOG_LEVEL");
    Ok(())
}
}

/// **CONFIG MERGE TESTS**
#[cfg(test)]
mod config_merge_tests {
    use super::*;

    #[test]
    fn test_config_merge_operation() -> Result<(), Box<dyn std::error::Error>> {
        let mut base_config = NestGateNestGateCanonicalConfig::default();
        let override_config = NestGateCanonicalConfig {
            system: nestgate_core::config::canonical_primary::SystemConfig {
                instance_name: "test-override".to_string(),
                ..base_config.system.clone()
            },
            ..base_config.clone()
        };
        
        base_config.merge(override_config);
        assert_eq!(base_config.system.instance_name, "test-override");
    Ok(())
    }

    #[test]
    fn test_config_override_precedence() -> Result<(), Box<dyn std::error::Error>> {
        // Test that environment variables override config file values
        env::set_var("NESTGATE_INSTANCE_NAME", "env-override");
        
        let config = NestGateCanonicalConfig::from_environment();
        assert_eq!(config.system.instance_name, "env-override");
        
        env::remove_var("NESTGATE_INSTANCE_NAME");
    Ok(())
    }

    #[test]
    fn test_config_partial_merge() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = NestGateNestGateCanonicalConfig::default();
        let original_port = config.network.port;
        
        // Create partial override
        let mut overrides = HashMap::new();
        overrides.insert("network.host".to_string(), "newhost.example.com".to_string());
        
        config.apply_overrides(overrides);
        assert_eq!(config.network.host, "newhost.example.com");
        assert_eq!(config.network.port, original_port); // Should remain unchanged
    Ok(())
}
}

/// **CONFIG VALIDATION TESTS**
#[cfg(test)]
mod config_validation_tests {
    use super::*;

    #[test]
    fn test_config_validation_rules() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = NestGateNestGateCanonicalConfig::default();
        
        // Test valid configuration
        assert!(config.validate().is_ok());
        
        // Test invalid network configuration
        config.network.port = 0;
        assert!(config.validate().is_err());
        
        // Test invalid system configuration
        config.network.port = 8080; // Reset
        config.system.instance_name = "".to_string();
        assert!(config.validate().is_err());
    Ok(())
    }

    #[test]
    fn test_config_dependency_validation() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = NestGateNestGateCanonicalConfig::default();
        
        // Test TLS dependency validation
        config.network.tls_enabled = true;
        config.security.tls_cert_path = None;
        assert!(config.validate().is_err(), "TLS enabled requires cert path");
        
        // Fix TLS configuration
        config.security.tls_cert_path = Some("/path/to/cert.pem".to_string());
        config.security.tls_key_path = Some("/path/to/key.pem".to_string());
        assert!(config.validate().is_ok(), "Complete TLS config should be valid");
    Ok(())
    }

    #[test]
    fn test_config_range_validation() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = NestGateNestGateCanonicalConfig::default();
        
        // Test port range validation
        config.network.port = 70000; // Invalid port
        assert!(config.validate().is_err());
        
        config.network.port = 8080; // Valid port
        assert!(config.validate().is_ok());
        
        // Test timeout validation
        config.network.timeout_seconds = 0; // Invalid timeout
        assert!(config.validate().is_err());
    Ok(())
}
}

/// **CONFIG FILE LOADING TESTS**
#[cfg(test)]
mod config_file_tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_from_toml_file() -> Result<(), Box<dyn std::error::Error>> {
        let toml_content = r#"
[system]
instance_name = "test-instance"
environment = "development"
debug_enabled = true

[network]
host = "test.example.com"
port = 9090
websocket_port = 9091
"#;
        
        let temp_file = NamedTempFile::new()?;
        fs::write(temp_file.path(), toml_content)?;
        
        let config = NestGateCanonicalConfig::from_file(temp_file.path())
            ?;
        
        assert_eq!(config.system.instance_name, "test-instance");
        assert_eq!(config.network.host, "test.example.com");
        assert_eq!(config.network.port, 9090);
    Ok(())
    }

    #[test]
    fn test_config_file_not_found() -> Result<(), Box<dyn std::error::Error>> {
        let result = NestGateCanonicalConfig::from_file("/nonexistent/path/config.toml");
        assert!(result.is_err(), "Should fail for non-existent file");
    Ok(())
    }

    #[test]
    fn test_config_invalid_toml() -> Result<(), Box<dyn std::error::Error>> {
        let invalid_toml = "invalid toml content [[[";
        let temp_file = NamedTempFile::new()?;
        fs::write(temp_file.path(), invalid_toml)?;
        
        let result = NestGateCanonicalConfig::from_file(temp_file.path());
        assert!(result.is_err(), "Should fail for invalid TOML");
    Ok(())
}
} 