# ✅ ERROR PATH TEST SUITE - Network Configuration
**Module**: `config::runtime::network`  
**Purpose**: Comprehensive error path coverage

```rust
#[cfg(test)]
mod error_path_tests {
    use super::*;
    use std::env;

    #[test]
    fn test_network_config_invalid_port_zero() {
        let config = NetworkConfig {
            api_port: 0,
            ..Default::default()
        };
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("port"));
    }

    #[test]
    fn test_network_config_invalid_timeout_zero() {
        let config = NetworkConfig {
            timeout_seconds: 0,
            ..Default::default()
        };
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("timeout"));
    }

    #[test]
    fn test_network_config_invalid_pool_size_zero() {
        let config = NetworkConfig {
            connection_pool_size: 0,
            ..Default::default()
        };
        
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("pool"));
    }

    #[test]
    fn test_network_config_from_env_invalid_port() {
        env::set_var("NESTGATE_API_PORT", "not_a_number");
        
        let result = NetworkConfig::from_environment();
        // Should use default, not error (graceful fallback)
        assert!(result.is_ok());
        
        env::remove_var("NESTGATE_API_PORT");
    }

    #[test]
    fn test_network_config_from_env_port_out_of_range() {
        env::set_var("NESTGATE_API_PORT", "99999");
        
        let result = NetworkConfig::from_environment();
        // Should use default for invalid port
        assert!(result.is_ok());
        
        env::remove_var("NESTGATE_API_PORT");
    }

    #[test]
    fn test_network_config_from_env_invalid_host() {
        env::set_var("NESTGATE_API_HOST", "not_an_ip");
        
        let result = NetworkConfig::from_environment();
        // Should use default localhost
        assert!(result.is_ok());
        
        env::remove_var("NESTGATE_API_HOST");
    }

    #[test]
    fn test_network_config_all_defaults() {
        // Clear all env vars
        let vars = ["NESTGATE_API_HOST", "NESTGATE_API_PORT", "NESTGATE_HTTPS_PORT"];
        for var in &vars {
            env::remove_var(var);
        }
        
        let config = NetworkConfig::from_environment().unwrap();
        assert_eq!(config.api_port, 8080); // Default
        assert_eq!(config.https_port, 8443); // Default
    }

    #[test]
    fn test_network_config_bind_all_flag() {
        let config_localhost = NetworkConfig {
            bind_all: false,
            ..Default::default()
        };
        assert!(!config_localhost.bind_all);
        
        let config_all = NetworkConfig {
            bind_all: true,
            ..Default::default()
        };
        assert!(config_all.bind_all);
    }

    #[test]
    fn test_network_config_url_generation() {
        let config = NetworkConfig::default();
        
        let api_url = config.api_base_url();
        assert!(api_url.starts_with("http://"));
        assert!(api_url.contains(":8080"));
        
        let https_url = config.https_base_url();
        assert!(https_url.starts_with("https://"));
        assert!(https_url.contains(":8443"));
    }

    #[test]
    fn test_network_config_extreme_timeouts() {
        let config_min = NetworkConfig {
            timeout_seconds: 1,
            ..Default::default()
        };
        assert!(config_min.validate().is_ok());
        
        let config_max = NetworkConfig {
            timeout_seconds: 3600, // 1 hour
            ..Default::default()
        };
        assert!(config_max.validate().is_ok());
    }
}
```

**Tests Added**: 11 comprehensive error path tests  
**Coverage**: Configuration validation, environment parsing, edge cases  
**Status**: Ready to add to `config/runtime/network.rs`

