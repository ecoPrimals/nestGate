// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Strategic Configuration Tests - December 11, 2025
//!
//! Comprehensive tests for configuration edge cases and scenarios

#[cfg(test)]
mod config_strategic_tests {
    #![allow(clippy::panic)] // test assertions via `let ... else { panic!(...) }`

    use crate::config::environment::{EnvironmentConfig, NetworkConfig, Port};
    use std::str::FromStr;

    // ==================== Port Tests ====================

    #[test]
    fn test_port_valid_range() {
        let ports = vec![1024, 3000, 8080, 8443, 9090, 65535];

        for port_num in ports {
            let port = Port::new(port_num);
            assert!(port.is_ok(), "Port {port_num} should be valid");
            let Ok(port) = port else {
                panic!("Port {port_num} should be valid");
            };
            assert_eq!(port.get(), port_num);
        }
    }

    #[test]
    fn test_port_from_string_success() {
        let port_strings = vec!["1024", "3000", "8080", "9090", "65535"];

        for port_str in port_strings {
            let port = Port::from_str(port_str);
            assert!(port.is_ok(), "Port string '{port_str}' should parse");
        }
    }

    #[test]
    fn test_port_get() {
        let port = Port::new_unchecked(8080);
        assert_eq!(port.get(), 8080);
    }

    #[test]
    fn test_port_debug() {
        let port = Port::new_unchecked(3000);
        let debug_str = format!("{port:?}");
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_port_clone() {
        let port1 = Port::new_unchecked(8080);
        let port2 = port1;
        assert_eq!(port1.get(), port2.get());
    }

    #[test]
    fn test_port_copy() {
        let port1 = Port::new_unchecked(8080);
        let port2 = port1; // Copy
        assert_eq!(port1.get(), port2.get());
    }

    // ==================== NetworkConfig Tests ====================

    #[test]
    fn test_network_config_creation() {
        let config = NetworkConfig {
            host: "127.0.0.1".to_string(),
            port: Port::new_unchecked(8080),
            timeout_secs: 30,
            max_connections: 1000,
            read_timeout_secs: 10,
            write_timeout_secs: 10,
            keepalive_secs: 60,
        };

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port.get(), 8080);
        assert_eq!(config.max_connections, 1000);
        assert_eq!(config.timeout_secs, 30);
    }

    #[test]
    fn test_network_config_various_hosts() {
        let hosts = vec![
            "localhost",
            "127.0.0.1",
            "0.0.0.0",
            "192.168.1.1",
            "10.0.0.1",
            "172.16.0.1",
        ];

        for host in hosts {
            let config = NetworkConfig {
                host: host.to_string(),
                port: Port::new_unchecked(8080),
                timeout_secs: 30,
                max_connections: 1000,
                read_timeout_secs: 10,
                write_timeout_secs: 10,
                keepalive_secs: 60,
            };

            assert_eq!(config.host, host);
        }
    }

    #[test]
    fn test_network_config_clone() {
        let config1 = NetworkConfig {
            host: "127.0.0.1".to_string(),
            port: Port::new_unchecked(8080),
            timeout_secs: 30,
            max_connections: 1000,
            read_timeout_secs: 10,
            write_timeout_secs: 10,
            keepalive_secs: 60,
        };

        let config2 = config1.clone();
        assert_eq!(config1.host, config2.host);
        assert_eq!(config1.port.get(), config2.port.get());
    }

    #[test]
    fn test_network_config_debug() {
        let config = NetworkConfig {
            host: "localhost".to_string(),
            port: Port::new_unchecked(3000),
            timeout_secs: 30,
            max_connections: 500,
            read_timeout_secs: 10,
            write_timeout_secs: 10,
            keepalive_secs: 60,
        };

        let debug_str = format!("{config:?}");
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("NetworkConfig"));
    }

    // ==================== EnvironmentConfig Tests ====================

    #[test]
    fn test_environment_config_default() {
        let config = EnvironmentConfig::default();
        // Should have reasonable defaults
        assert!(!config.network.host.is_empty());
    }

    #[test]
    fn test_environment_config_clone() {
        let config1 = EnvironmentConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1.network.host, config2.network.host);
        assert_eq!(config1.network.port.get(), config2.network.port.get());
    }

    #[test]
    fn test_environment_config_debug() {
        let config = EnvironmentConfig::default();
        let debug_str = format!("{config:?}");
        assert!(!debug_str.is_empty());
    }

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_port_boundary_values() {
        // Test exact boundaries
        assert!(Port::new(1024).is_ok());
        assert!(Port::new(65535).is_ok());
    }

    #[test]
    fn test_max_connections_variants() {
        let connection_limits = vec![1, 10, 100, 1000, 10000, 100_000];

        for limit in connection_limits {
            let config = NetworkConfig {
                host: "localhost".to_string(),
                port: Port::new_unchecked(8080),
                timeout_secs: 30,
                max_connections: limit,
                read_timeout_secs: 10,
                write_timeout_secs: 10,
                keepalive_secs: 60,
            };

            assert_eq!(config.max_connections, limit);
        }
    }

    #[test]
    fn test_timeout_variants() {
        let timeouts = vec![1, 5, 10, 30, 60, 120, 300];

        for timeout in timeouts {
            let config = NetworkConfig {
                host: "localhost".to_string(),
                port: Port::new_unchecked(8080),
                timeout_secs: timeout,
                max_connections: 1000,
                read_timeout_secs: timeout,
                write_timeout_secs: timeout,
                keepalive_secs: timeout * 2,
            };

            assert_eq!(config.timeout_secs, timeout);
            assert_eq!(config.read_timeout_secs, timeout);
            assert_eq!(config.write_timeout_secs, timeout);
        }
    }

    #[test]
    fn test_network_config_serialization() {
        let config = NetworkConfig {
            host: "127.0.0.1".to_string(),
            port: Port::new_unchecked(8080),
            timeout_secs: 30,
            max_connections: 1000,
            read_timeout_secs: 10,
            write_timeout_secs: 10,
            keepalive_secs: 60,
        };

        let json = serde_json::to_string(&config);
        assert!(json.is_ok(), "NetworkConfig should serialize");

        let Ok(serialized) = json else {
            panic!("NetworkConfig should serialize");
        };
        assert!(serialized.contains("127.0.0.1"));
        assert!(serialized.contains("8080"));
    }

    #[test]
    fn test_network_config_deserialization() {
        let json = r#"{
            "host": "localhost",
            "port": 3000,
            "timeout_secs": 30,
            "max_connections": 500,
            "read_timeout_secs": 10,
            "write_timeout_secs": 10,
            "keepalive_secs": 60
        }"#;

        let config: Result<NetworkConfig, _> = serde_json::from_str(json);
        assert!(config.is_ok(), "NetworkConfig should deserialize");

        let Ok(config) = config else {
            panic!("NetworkConfig should deserialize");
        };
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port.get(), 3000);
        assert_eq!(config.max_connections, 500);
    }

    // ==================== Real-World Scenarios ====================

    #[test]
    fn test_development_network_config() {
        let config = NetworkConfig {
            host: "localhost".to_string(),
            port: Port::new_unchecked(3000),
            timeout_secs: 60,
            max_connections: 100,
            read_timeout_secs: 30,
            write_timeout_secs: 30,
            keepalive_secs: 120,
        };

        assert_eq!(config.host, "localhost");
        assert_eq!(config.max_connections, 100);
    }

    #[test]
    fn test_production_network_config() {
        let config = NetworkConfig {
            host: "0.0.0.0".to_string(),
            port: Port::new_unchecked(8080),
            timeout_secs: 30,
            max_connections: 10000,
            read_timeout_secs: 10,
            write_timeout_secs: 10,
            keepalive_secs: 60,
        };

        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port.get(), 8080);
        assert_eq!(config.max_connections, 10000);
    }

    #[test]
    fn test_high_performance_network_config() {
        let config = NetworkConfig {
            host: "0.0.0.0".to_string(),
            port: Port::new_unchecked(9090),
            timeout_secs: 5,
            max_connections: 100_000,
            read_timeout_secs: 2,
            write_timeout_secs: 2,
            keepalive_secs: 30,
        };

        assert_eq!(config.max_connections, 100_000);
        assert_eq!(config.timeout_secs, 5); // Low timeout for high performance
    }

    // ==================== String Representation Tests ====================

    #[test]
    fn test_port_string_conversion_roundtrip() {
        let original = Port::new_unchecked(8080);
        let as_string = original.get().to_string();
        let Ok(parsed) = Port::from_str(&as_string) else {
            panic!("port roundtrip parse");
        };

        assert_eq!(original.get(), parsed.get());
    }

    #[test]
    fn test_network_config_with_ipv6() {
        let config = NetworkConfig {
            host: "::1".to_string(), // IPv6 localhost
            port: Port::new_unchecked(8080),
            timeout_secs: 30,
            max_connections: 1000,
            read_timeout_secs: 10,
            write_timeout_secs: 10,
            keepalive_secs: 60,
        };

        assert_eq!(config.host, "::1");
    }

    #[test]
    fn test_network_config_with_hostname() {
        let config = NetworkConfig {
            host: "example.com".to_string(),
            port: Port::new_unchecked(8443),
            timeout_secs: 30,
            max_connections: 5000,
            read_timeout_secs: 15,
            write_timeout_secs: 15,
            keepalive_secs: 90,
        };

        assert_eq!(config.host, "example.com");
        assert_eq!(config.port.get(), 8443);
    }
}
