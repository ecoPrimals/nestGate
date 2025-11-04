//! Comprehensive network tests for increased coverage
//!
//! This test suite adds comprehensive coverage for network functionality
//! to bring coverage from 76% to 80%+.

use nestgate_network::types::{
    CircuitBreakerConfig, ConnectionInfo, ConnectionStatus, HealthStatus, LoadBalancingConfig,
    NetworkConfigBuilder, NetworkExtensions, NetworkStatistics, ServiceInfo, ServiceStatus,
};
use nestgate_network::{
    default_network_config, development_network_config, production_network_config, NetworkError,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

// ==================== NetworkError Tests ====================

#[test]
fn test_network_error_connection_failed() {
    let error = NetworkError::ConnectionFailed {
        message: "Connection refused".to_string(),
    };
    let error_str = format!("{}", error);
    assert!(error_str.contains("Connection failed"));
    assert!(error_str.contains("Connection refused"));
}

#[test]
fn test_network_error_timeout() {
    let error = NetworkError::Timeout {
        b_operation: Some("connect".to_string()),
    };
    let error_str = format!("{}", error);
    assert!(error_str.contains("Timeout occurred"));
}

#[test]
fn test_network_error_timeout_no_operation() {
    let error = NetworkError::Timeout { b_operation: None };
    let error_str = format!("{}", error);
    assert!(error_str.contains("Timeout occurred"));
}

#[test]
fn test_network_error_configuration() {
    let error = NetworkError::Configuration {
        field: "port".to_string(),
        message: "Invalid port number".to_string(),
    };
    let error_str = format!("{}", error);
    assert!(error_str.contains("Configuration error"));
    assert!(error_str.contains("port"));
    assert!(error_str.contains("Invalid port number"));
}

#[test]
fn test_network_error_protocol() {
    let error = NetworkError::Protocol {
        protocol: "HTTP".to_string(),
        message: "Invalid request".to_string(),
    };
    let error_str = format!("{}", error);
    assert!(error_str.contains("Protocol error"));
    assert!(error_str.contains("HTTP"));
}

#[test]
fn test_network_error_service_unavailable() {
    let error = NetworkError::ServiceUnavailable {
        service: "auth-service".to_string(),
    };
    let error_str = format!("{}", error);
    assert!(error_str.contains("Service unavailable"));
    assert!(error_str.contains("auth-service"));
}

#[test]
fn test_network_error_debug() {
    let error = NetworkError::ConnectionFailed {
        message: "test".to_string(),
    };
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("ConnectionFailed"));
}

// ==================== ConnectionStatus Tests ====================

#[test]
fn test_connection_status_debug() {
    let status = ConnectionStatus::Active;
    let debug_str = format!("{:?}", status);
    assert!(debug_str.contains("Active"));
}

#[test]
fn test_connection_status_all_variants() {
    let statuses = [
        ConnectionStatus::Active,
        ConnectionStatus::Idle,
        ConnectionStatus::Closing,
        ConnectionStatus::Closed,
    ];
    assert_eq!(statuses.len(), 4);
}

#[test]
fn test_connection_status_clone() {
    let status = ConnectionStatus::Active;
    let cloned = status.clone();
    // Both should represent Active state
    match (status, cloned) {
        (ConnectionStatus::Active, ConnectionStatus::Active) => (),
        (s, c) => assert!(
            false,
            "Clone should preserve Active state, got: ({:?}, {:?})",
            s, c
        ),
    }
}

// ==================== HealthStatus Tests ====================

#[test]
fn test_health_status_debug() {
    let status = HealthStatus::Healthy;
    let debug_str = format!("{:?}", status);
    assert!(debug_str.contains("Healthy"));
}

#[test]
fn test_health_status_all_variants() {
    let statuses = [
        HealthStatus::Healthy,
        HealthStatus::Degraded,
        HealthStatus::Unhealthy,
        HealthStatus::Unknown,
    ];
    assert_eq!(statuses.len(), 4);
}

#[test]
fn test_health_status_clone() {
    let status = HealthStatus::Healthy;
    let cloned = status.clone();
    match (status, cloned) {
        (HealthStatus::Healthy, HealthStatus::Healthy) => (),
        (s, c) => assert!(
            false,
            "Clone should preserve Healthy state, got: ({:?}, {:?})",
            s, c
        ),
    }
}

// ==================== ConnectionInfo Advanced Tests ====================

#[test]
fn test_connection_info_age() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let conn = ConnectionInfo::new("conn-1".to_string(), addr);

    // Age should be very small (just created)
    let age = conn.age();
    assert!(age.as_millis() < 1000); // Less than 1 second
}

#[test]
fn test_connection_info_large_byte_counts() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let mut conn = ConnectionInfo::new("conn-1".to_string(), addr);

    // Add large byte counts
    conn.add_bytes_sent(u64::MAX / 2);
    conn.add_bytes_sent(100);

    assert!(conn.bytes_sent > u64::MAX / 2);
}

#[test]
fn test_connection_info_status_getter() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let mut conn = ConnectionInfo::new("conn-1".to_string(), addr);

    match conn.status() {
        ConnectionStatus::Active => (),
        other => assert!(false, "Initial status should be Active, got: {:?}", other),
    }

    conn.set_status(ConnectionStatus::Idle);
    match conn.status() {
        ConnectionStatus::Idle => (),
        other => assert!(false, "Status should be Idle, got: {:?}", other),
    }
}

#[test]
fn test_connection_info_clone() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let conn = ConnectionInfo::new("conn-1".to_string(), addr);
    let cloned = conn.clone();

    assert_eq!(conn.id(), cloned.id());
    assert_eq!(conn.address(), cloned.address());
}

#[test]
fn test_connection_info_debug() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let conn = ConnectionInfo::new("conn-1".to_string(), addr);
    let debug_str = format!("{:?}", conn);

    assert!(debug_str.contains("ConnectionInfo"));
}

// ==================== ServiceInfo Advanced Tests ====================

#[test]
fn test_service_info_age() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);
    let service = ServiceInfo::new("svc-1".to_string(), "test".to_string(), addr);

    let age = service.age();
    assert!(age.as_millis() < 1000);
}

#[test]
fn test_service_info_registered_at() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);
    let service = ServiceInfo::new("svc-1".to_string(), "test".to_string(), addr);

    let registered = service.registered_at();
    assert!(
        registered
            .elapsed()
            .expect("Network operation failed")
            .as_millis()
            < 1000
    );
}

#[test]
fn test_service_info_clone() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);
    let service = ServiceInfo::new("svc-1".to_string(), "test".to_string(), addr);
    let cloned = service.clone();

    assert_eq!(service.id(), cloned.id());
    assert_eq!(service.name(), cloned.name());
}

#[test]
fn test_service_info_debug() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);
    let service = ServiceInfo::new("svc-1".to_string(), "test".to_string(), addr);
    let debug_str = format!("{:?}", service);

    assert!(debug_str.contains("ServiceInfo"));
}

#[test]
fn test_service_info_multiple_metadata() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);
    let mut service = ServiceInfo::new("svc-1".to_string(), "test".to_string(), addr);

    for i in 0..10 {
        service.add_metadata(format!("key{}", i), format!("value{}", i));
    }

    assert_eq!(service.metadata().len(), 10);
}

// ==================== NetworkConfig Builder Advanced Tests ====================

#[test]
fn test_network_config_builder_default() {
    let builder = NetworkConfigBuilder::default();
    let config = builder.build();

    assert!(config.network.api.port > 0);
}

#[test]
fn test_network_config_builder_chaining() {
    let config = NetworkConfigBuilder::new()
        .port(9999)
        .max_connections(500)
        .connection_timeout(60)
        .keep_alive_timeout(120)
        .port_range(15000, 25000)
        .build();

    assert_eq!(config.network.api.port, 9999);
    assert_eq!(config.network.api.max_connections, 500);
    assert_eq!(
        config.network.api.connection_timeout,
        Duration::from_secs(60)
    );
    assert_eq!(config.extensions.keep_alive_timeout_seconds, 120);
    assert_eq!(config.extensions.port_range_start, 15000);
    assert_eq!(config.extensions.port_range_end, 25000);
}

#[test]
fn test_network_config_builder_keep_alive() {
    let config = NetworkConfigBuilder::new()
        .keep_alive(true)
        .keep_alive_timeout(300)
        .build();

    assert_eq!(config.extensions.keep_alive_timeout_seconds, 300);
}

// ==================== LoadBalancingConfig Tests ====================

#[test]
fn test_load_balancing_config_clone() {
    let config = LoadBalancingConfig::default();
    let cloned = config.clone();

    assert_eq!(config.algorithm, cloned.algorithm);
    assert_eq!(config.max_failures, cloned.max_failures);
}

#[test]
fn test_load_balancing_config_debug() {
    let config = LoadBalancingConfig::default();
    let debug_str = format!("{:?}", config);

    assert!(debug_str.contains("LoadBalancingConfig"));
}

#[test]
fn test_load_balancing_config_serialize() {
    let config = LoadBalancingConfig::default();
    let serialized = serde_json::to_string(&config);

    assert!(serialized.is_ok());
}

// ==================== CircuitBreakerConfig Tests ====================

#[test]
fn test_circuit_breaker_config_clone() {
    let config = CircuitBreakerConfig::default();
    let cloned = config.clone();

    assert_eq!(config.failure_threshold, cloned.failure_threshold);
    assert_eq!(config.half_open_max_calls, cloned.half_open_max_calls);
}

#[test]
fn test_circuit_breaker_config_debug() {
    let config = CircuitBreakerConfig::default();
    let debug_str = format!("{:?}", config);

    assert!(debug_str.contains("CircuitBreakerConfig"));
}

#[test]
fn test_circuit_breaker_config_serialize() {
    let config = CircuitBreakerConfig::default();
    let serialized = serde_json::to_string(&config);

    assert!(serialized.is_ok());
}

// ==================== NetworkExtensions Tests ====================

#[test]
fn test_network_extensions_clone() {
    let ext = NetworkExtensions::default();
    let cloned = ext.clone();

    assert_eq!(ext.port_range_start, cloned.port_range_start);
    assert_eq!(ext.port_range_end, cloned.port_range_end);
}

#[test]
fn test_network_extensions_debug() {
    let ext = NetworkExtensions::default();
    let debug_str = format!("{:?}", ext);

    assert!(debug_str.contains("NetworkExtensions"));
}

#[test]
fn test_network_extensions_protocol_settings() {
    let mut ext = NetworkExtensions::default();
    ext.protocol_settings
        .insert("http".to_string(), "v1".to_string());
    ext.protocol_settings
        .insert("ws".to_string(), "v2".to_string());

    assert_eq!(ext.protocol_settings.len(), 2);
}

#[test]
fn test_network_extensions_serialize() {
    let ext = NetworkExtensions::default();
    let serialized = serde_json::to_string(&ext);

    assert!(serialized.is_ok());
}

// ==================== NetworkStatistics Advanced Tests ====================

#[test]
fn test_network_statistics_clone() {
    let stats = NetworkStatistics::default();
    let cloned = stats.clone();

    assert_eq!(stats.active_connections, cloned.active_connections);
}

#[test]
fn test_network_statistics_debug() {
    let stats = NetworkStatistics::default();
    let debug_str = format!("{:?}", stats);

    assert!(debug_str.contains("NetworkStatistics"));
}

#[test]
fn test_network_statistics_with_values() {
    let stats = NetworkStatistics {
        active_connections: 100,
        registered_services: 50,
        allocated_ports: 200,
        total_bytes_sent: 1_000_000,
        total_bytes_received: 2_000_000,
    };

    assert_eq!(stats.active_connections, 100);
    assert_eq!(stats.registered_services, 50);
    assert_eq!(stats.allocated_ports, 200);
}

// ==================== ServiceStatus Tests ====================

#[test]
fn test_service_status_default() {
    let status = ServiceStatus::default();
    assert_eq!(status, ServiceStatus::Unknown);
}

#[test]
fn test_service_status_clone() {
    let status = ServiceStatus::Running;
    let cloned = status.clone();
    assert_eq!(status, cloned);
}

#[test]
fn test_service_status_debug() {
    let status = ServiceStatus::Running;
    let debug_str = format!("{:?}", status);
    assert!(debug_str.contains("Running"));
}

#[test]
fn test_service_status_serialize() {
    let status = ServiceStatus::Healthy;
    let serialized = serde_json::to_string(&status);
    assert!(serialized.is_ok());
}

#[test]
fn test_service_status_deserialize() {
    let json = r#""Running""#;
    let status: Result<ServiceStatus, _> = serde_json::from_str(json);
    assert!(status.is_ok());
    assert_eq!(
        status.expect("Network operation failed"),
        ServiceStatus::Running
    );
}

// ==================== Configuration Function Tests ====================

#[test]
fn test_default_network_config() {
    let config = default_network_config();
    assert!(config.network.api.port > 0);
    assert!(config.network.api.max_connections > 0);
}

#[test]
fn test_production_vs_development_timeouts() {
    let prod = production_network_config();
    let dev = development_network_config();

    // Production should have shorter timeout
    assert!(prod.network.api.connection_timeout < dev.network.api.connection_timeout);
}

#[test]
fn test_configuration_consistency() {
    let default = default_network_config();
    let prod = production_network_config();
    let dev = development_network_config();

    // All should have valid port ranges
    assert!(default.extensions.port_range_end > default.extensions.port_range_start);
    assert!(prod.extensions.port_range_end > prod.extensions.port_range_start);
    assert!(dev.extensions.port_range_end > dev.extensions.port_range_start);
}

// ==================== Edge Case Tests ====================

#[test]
fn test_connection_info_zero_bytes() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let conn = ConnectionInfo::new("conn-1".to_string(), addr);

    assert_eq!(conn.bytes_sent, 0);
    assert_eq!(conn.bytes_received, 0);
}

#[test]
fn test_service_info_empty_metadata() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);
    let service = ServiceInfo::new("svc-1".to_string(), "test".to_string(), addr);

    assert_eq!(service.metadata().len(), 0);
}

#[test]
fn test_network_statistics_zero_values() {
    let stats = NetworkStatistics::default();

    assert_eq!(stats.active_connections, 0);
    assert_eq!(stats.registered_services, 0);
    assert_eq!(stats.total_bytes_sent, 0);
}

#[test]
fn test_network_config_builder_minimal() {
    let config = NetworkConfigBuilder::new().build();

    // Should have valid defaults
    assert!(config.network.api.port > 0);
    assert!(config.network.api.max_connections > 0);
}

#[test]
fn test_ipv6_address_support() {
    use std::net::Ipv6Addr;

    let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 8080);
    let conn = ConnectionInfo::new("conn-ipv6".to_string(), addr);

    assert_eq!(conn.address(), addr);
}

#[test]
fn test_high_port_numbers() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 65535);
    let service = ServiceInfo::new("svc-high-port".to_string(), "test".to_string(), addr);

    assert_eq!(service.address().port(), 65535);
}

// ==================== Integration Tests ====================

#[test]
fn test_full_connection_lifecycle() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let mut conn = ConnectionInfo::new("conn-lifecycle".to_string(), addr);

    // Initial state
    assert!(conn.is_active());
    assert_eq!(conn.bytes_sent, 0);

    // Transfer data
    conn.add_bytes_sent(1024);
    conn.add_bytes_received(2048);
    assert_eq!(conn.bytes_sent, 1024);

    // Transition to idle
    conn.set_status(ConnectionStatus::Idle);
    assert!(!conn.is_active());

    // Close connection
    conn.set_status(ConnectionStatus::Closed);
    assert!(!conn.is_active());
}

#[test]
fn test_full_service_lifecycle() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);
    let mut service = ServiceInfo::new("svc-lifecycle".to_string(), "api".to_string(), addr);

    // Initial healthy state
    assert!(service.is_healthy());

    // Add metadata
    service.add_metadata("version".to_string(), "1.0.0".to_string());
    service.add_metadata("region".to_string(), "us-west".to_string());
    assert_eq!(service.metadata().len(), 2);

    // Degrade service
    service.set_health_status(HealthStatus::Degraded);
    assert!(!service.is_healthy());

    // Mark unhealthy
    service.set_health_status(HealthStatus::Unhealthy);
    assert!(!service.is_healthy());

    // Recover
    service.set_health_status(HealthStatus::Healthy);
    assert!(service.is_healthy());
}

#[test]
fn test_network_config_production_scenario() {
    let config = NetworkConfigBuilder::new()
        .port(443)
        .max_connections(10000)
        .connection_timeout(5)
        .keep_alive_timeout(60)
        .port_range(30000, 40000)
        .build();

    // Production-like settings
    assert_eq!(config.network.api.port, 443);
    assert_eq!(config.network.api.max_connections, 10000);
    assert_eq!(
        config.network.api.connection_timeout,
        Duration::from_secs(5)
    );
    assert_eq!(config.extensions.port_range_start, 30000);
}
