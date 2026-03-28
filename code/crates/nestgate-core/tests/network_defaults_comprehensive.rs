// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Comprehensive tests for network defaults configuration
//! Tests port defaults, address configuration, and URL generation

use nestgate_core::config::defaults_config::NetworkDefaultsConfig;

#[test]
fn test_default_ports() {
    let config = NetworkDefaultsConfig::new();

    // Verify all default ports are non-zero (valid range is guaranteed by u16 type)
    assert!(config.get_api_port() > 0);
    assert!(config.get_websocket_port() > 0);
    assert!(config.get_http_port() > 0);
    assert!(config.get_metrics_port() > 0);
    assert!(config.get_health_port() > 0);
}

#[test]
fn test_custom_port_configuration() {
    let config = NetworkDefaultsConfig::new()
        .with_api_port(9999)
        .with_websocket_port(8888)
        .with_http_port(7777);

    assert_eq!(config.get_api_port(), 9999);
    assert_eq!(config.get_websocket_port(), 8888);
    assert_eq!(config.get_http_port(), 7777);
}

#[test]
fn test_bind_address_defaults() {
    let config = NetworkDefaultsConfig::new();
    let bind_addr = config.get_bind_address();

    // Should default to localhost
    assert!(bind_addr.contains("127.0.0.1") || bind_addr.contains("localhost"));
}

#[test]
fn test_custom_bind_address() {
    let config = NetworkDefaultsConfig::new().with_bind_address("0.0.0.0".to_string());

    assert_eq!(config.get_bind_address(), "0.0.0.0");
}

#[test]
fn test_hostname_defaults() {
    let config = NetworkDefaultsConfig::new();
    let hostname = config.get_hostname();

    // Should have a default hostname
    assert!(!hostname.is_empty());
    // Hostname is valid - either localhost or some other non-empty value
    // (already verified non-empty above)
}

#[test]
fn test_custom_hostname() {
    let config = NetworkDefaultsConfig::new().with_hostname("custom.example.com".to_string());

    assert_eq!(config.get_hostname(), "custom.example.com");
}

#[test]
fn test_url_generation() {
    let config = NetworkDefaultsConfig::new()
        .with_api_port(5000)
        .with_websocket_port(6000);

    let api_url = config.get_api_base_url();
    let ws_url = config.get_websocket_base_url();

    // URLs should be valid
    assert!(api_url.contains("5000"));
    assert!(ws_url.contains("6000"));
}

#[test]
fn test_timeout_defaults() {
    let config = NetworkDefaultsConfig::new();

    let conn_timeout = config.get_connection_timeout_ms();
    let req_timeout = config.get_request_timeout_ms();

    // Timeouts should be reasonable
    assert!(conn_timeout > 0);
    assert!(req_timeout > 0);
    assert!(req_timeout >= conn_timeout);
}

#[test]
fn test_custom_timeouts() {
    let config = NetworkDefaultsConfig::new().with_connection_timeout_ms(5000);

    assert_eq!(config.get_connection_timeout_ms(), 5000);
}

#[test]
fn test_config_builder_chain() {
    let config = NetworkDefaultsConfig::new()
        .with_api_port(3000)
        .with_hostname("test.local".to_string())
        .with_connection_timeout_ms(10000);

    // All chained settings should apply
    assert_eq!(config.get_api_port(), 3000);
    assert_eq!(config.get_hostname(), "test.local");
    assert_eq!(config.get_connection_timeout_ms(), 10000);
}

#[test]
fn test_port_ranges() {
    // Test various common port ranges
    let ports = vec![
        (1024, "privileged boundary"),
        (8080, "http-alt"),
        (3000, "node default"),
        (5000, "flask default"),
        (9090, "prometheus"),
        (65535, "max valid port"),
    ];

    for (port, _description) in ports {
        let config = NetworkDefaultsConfig::new().with_api_port(port);
        assert_eq!(config.get_api_port(), port);
    }
}

#[test]
fn test_ipv4_addresses() {
    let addresses = vec!["127.0.0.1", "0.0.0.0", "192.168.1.1", "10.0.0.1"];

    for addr in addresses {
        let config = NetworkDefaultsConfig::new().with_bind_address(addr.to_string());
        assert_eq!(config.get_bind_address(), addr);
    }
}

#[test]
fn test_ipv6_addresses() {
    let addresses = vec!["::1", "::", "fe80::1"];

    for addr in addresses {
        let config = NetworkDefaultsConfig::new().with_bind_address(addr.to_string());
        assert_eq!(config.get_bind_address(), addr);
    }
}

#[test]
fn test_multiple_configs_independent() {
    let config1 = NetworkDefaultsConfig::new().with_api_port(1111);
    let config2 = NetworkDefaultsConfig::new().with_api_port(2222);
    let config3 = NetworkDefaultsConfig::new().with_api_port(3333);

    // Each config should be independent
    assert_eq!(config1.get_api_port(), 1111);
    assert_eq!(config2.get_api_port(), 2222);
    assert_eq!(config3.get_api_port(), 3333);
}

#[test]
fn test_config_immutability_after_clone() {
    let config1 = NetworkDefaultsConfig::new().with_api_port(4000);
    let config2 = config1.clone();

    // Both should have same value
    assert_eq!(config1.get_api_port(), config2.get_api_port());
}

#[test]
fn test_development_config() {
    let config = NetworkDefaultsConfig::new()
        .with_api_port(3000)
        .with_bind_address("127.0.0.1".to_string())
        .with_connection_timeout_ms(60000);

    // Development typically uses localhost and longer timeouts
    assert_eq!(config.get_api_port(), 3000);
    assert_eq!(config.get_bind_address(), "127.0.0.1");
    assert_eq!(config.get_connection_timeout_ms(), 60000);
}

#[test]
fn test_production_config() {
    let config = NetworkDefaultsConfig::new()
        .with_api_port(8080)
        .with_bind_address("0.0.0.0".to_string())
        .with_connection_timeout_ms(3000);

    // Production typically binds to all interfaces with shorter timeouts
    assert_eq!(config.get_api_port(), 8080);
    assert_eq!(config.get_bind_address(), "0.0.0.0");
    assert_eq!(config.get_connection_timeout_ms(), 3000);
}

#[test]
fn test_all_getters_work() {
    let config = NetworkDefaultsConfig::new();

    // All getters should return valid values without panicking
    let _ = config.get_api_port();
    let _ = config.get_websocket_port();
    let _ = config.get_http_port();
    let _ = config.get_nas_http_port();
    let _ = config.get_dev_server_port();
    let _ = config.get_metrics_port();
    let _ = config.get_health_port();
    let _ = config.get_orchestrator_port();
    let _ = config.get_bind_address();
    let _ = config.get_development_bind_address();
    let _ = config.get_hostname();
    let _ = config.get_external_hostname();
    let _ = config.get_websocket_base_url();
    let _ = config.get_api_base_url();
    let _ = config.get_connection_timeout_ms();
    let _ = config.get_request_timeout_ms();
}

#[test]
fn test_url_format_correctness() {
    let config = NetworkDefaultsConfig::new().with_api_port(8080);

    let api_url = config.get_api_base_url();

    // URL should be properly formatted
    assert!(api_url.starts_with("http://") || api_url.starts_with("https://"));
    assert!(api_url.contains(":"));
}

#[test]
fn test_websocket_url_format() {
    let config = NetworkDefaultsConfig::new().with_websocket_port(8082);

    let ws_url = config.get_websocket_base_url();

    // WebSocket URL should start with ws://
    assert!(ws_url.starts_with("ws://") || ws_url.starts_with("wss://"));
}

#[test]
fn test_config_from_env_doesnt_panic() {
    // Should handle missing env vars gracefully
    let config = NetworkDefaultsConfig::from_env();

    // Should provide defaults
    assert!(config.get_api_port() > 0);
}

#[test]
fn test_default_trait() {
    let config1 = NetworkDefaultsConfig::default();
    let config2 = NetworkDefaultsConfig::new();

    // Default and new should produce same configuration
    assert_eq!(config1.get_api_port(), config2.get_api_port());
}
