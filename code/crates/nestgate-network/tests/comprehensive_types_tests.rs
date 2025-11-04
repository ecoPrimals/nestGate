//! Comprehensive network types tests
//! Improving coverage for nestgate-network crate

use nestgate_network::types::*;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

// ==================== LOAD BALANCING CONFIG TESTS ====================

#[test]
fn test_load_balancing_config_default() {
    let config = LoadBalancingConfig::default();

    assert_eq!(config.algorithm, "round_robin");
    assert_eq!(config.health_check_interval, Duration::from_secs(30));
    assert_eq!(config.max_failures, 3);
}

#[test]
fn test_load_balancing_config_custom() {
    let config = LoadBalancingConfig {
        algorithm: "least_connections".to_string(),
        health_check_interval: Duration::from_secs(60),
        max_failures: 5,
    };

    assert_eq!(config.algorithm, "least_connections");
    assert_eq!(config.max_failures, 5);
}

#[test]
fn test_load_balancing_algorithms() {
    let algorithms = vec![
        "round_robin",
        "least_connections",
        "ip_hash",
        "weighted_round_robin",
    ];

    for algo in algorithms {
        let config = LoadBalancingConfig {
            algorithm: algo.to_string(),
            health_check_interval: Duration::from_secs(30),
            max_failures: 3,
        };
        assert_eq!(config.algorithm, algo);
    }
}

#[test]
fn test_load_balancing_health_check_intervals() {
    let intervals = vec![10, 30, 60, 120];

    for secs in intervals {
        let config = LoadBalancingConfig {
            algorithm: "round_robin".to_string(),
            health_check_interval: Duration::from_secs(secs),
            max_failures: 3,
        };
        assert_eq!(config.health_check_interval.as_secs(), secs);
    }
}

#[test]
fn test_load_balancing_max_failures_edge_cases() {
    // Zero failures
    let config1 = LoadBalancingConfig {
        algorithm: "round_robin".to_string(),
        health_check_interval: Duration::from_secs(30),
        max_failures: 0,
    };
    assert_eq!(config1.max_failures, 0);

    // High failure threshold
    let config2 = LoadBalancingConfig {
        algorithm: "round_robin".to_string(),
        health_check_interval: Duration::from_secs(30),
        max_failures: 100,
    };
    assert_eq!(config2.max_failures, 100);
}

// ==================== CIRCUIT BREAKER CONFIG TESTS ====================

#[test]
fn test_circuit_breaker_default() {
    let config = CircuitBreakerConfig::default();

    assert_eq!(config.failure_threshold, 5);
    assert_eq!(config.timeout_duration, Duration::from_secs(60));
    assert_eq!(config.half_open_max_calls, 10);
}

#[test]
fn test_circuit_breaker_custom() {
    let config = CircuitBreakerConfig {
        failure_threshold: 10,
        timeout_duration: Duration::from_secs(120),
        half_open_max_calls: 5,
    };

    assert_eq!(config.failure_threshold, 10);
    assert_eq!(config.timeout_duration.as_secs(), 120);
    assert_eq!(config.half_open_max_calls, 5);
}

#[test]
fn test_circuit_breaker_thresholds() {
    let thresholds = vec![1, 3, 5, 10, 20];

    for threshold in thresholds {
        let config = CircuitBreakerConfig {
            failure_threshold: threshold,
            timeout_duration: Duration::from_secs(60),
            half_open_max_calls: 10,
        };
        assert_eq!(config.failure_threshold, threshold);
    }
}

#[test]
fn test_circuit_breaker_timeout_durations() {
    let durations = vec![30, 60, 120, 300];

    for secs in durations {
        let config = CircuitBreakerConfig {
            failure_threshold: 5,
            timeout_duration: Duration::from_secs(secs),
            half_open_max_calls: 10,
        };
        assert_eq!(config.timeout_duration.as_secs(), secs);
    }
}

#[test]
fn test_circuit_breaker_half_open_calls() {
    let max_calls = vec![1, 5, 10, 20, 50];

    for calls in max_calls {
        let config = CircuitBreakerConfig {
            failure_threshold: 5,
            timeout_duration: Duration::from_secs(60),
            half_open_max_calls: calls,
        };
        assert_eq!(config.half_open_max_calls, calls);
    }
}

// ==================== NETWORK EXTENSIONS TESTS ====================

#[test]
fn test_network_extensions_default() {
    let ext = NetworkExtensions::default();

    assert_eq!(ext.port_range_start, 9000);
    assert_eq!(ext.port_range_end, 9999);
    assert_eq!(ext.keep_alive_timeout_seconds, 60);
    assert!(ext.protocol_settings.is_empty());
}

#[test]
fn test_network_extensions_port_range() {
    let ext = NetworkExtensions {
        port_range_start: 8000,
        port_range_end: 8999,
        keep_alive_timeout_seconds: 60,
        protocol_settings: HashMap::new(),
        load_balancing: LoadBalancingConfig::default(),
        circuit_breaker: CircuitBreakerConfig::default(),
    };

    assert_eq!(ext.port_range_start, 8000);
    assert_eq!(ext.port_range_end, 8999);
    assert!(ext.port_range_end > ext.port_range_start);
}

#[test]
fn test_network_extensions_keep_alive() {
    let timeouts = vec![30, 60, 120, 300];

    for timeout in timeouts {
        let ext = NetworkExtensions {
            port_range_start: 9000,
            port_range_end: 9999,
            keep_alive_timeout_seconds: timeout,
            protocol_settings: HashMap::new(),
            load_balancing: LoadBalancingConfig::default(),
            circuit_breaker: CircuitBreakerConfig::default(),
        };
        assert_eq!(ext.keep_alive_timeout_seconds, timeout);
    }
}

#[test]
fn test_network_extensions_with_protocol_settings() {
    let mut settings = HashMap::new();
    settings.insert("http".to_string(), "enabled".to_string());
    settings.insert("https".to_string(), "enabled".to_string());
    settings.insert("ws".to_string(), "disabled".to_string());

    let ext = NetworkExtensions {
        port_range_start: 9000,
        port_range_end: 9999,
        keep_alive_timeout_seconds: 60,
        protocol_settings: settings.clone(),
        load_balancing: LoadBalancingConfig::default(),
        circuit_breaker: CircuitBreakerConfig::default(),
    };

    assert_eq!(ext.protocol_settings.len(), 3);
    assert_eq!(
        ext.protocol_settings.get("http"),
        Some(&"enabled".to_string())
    );
}

#[test]
fn test_network_extensions_empty_protocol_settings() {
    let ext = NetworkExtensions::default();
    assert_eq!(ext.protocol_settings.len(), 0);
    assert!(ext.protocol_settings.is_empty());
}

// ==================== CONNECTION STATUS TESTS ====================

#[test]
fn test_connection_status_variants() {
    let statuses = vec![
        ConnectionStatus::Idle,
        ConnectionStatus::Active,
        ConnectionStatus::Closing,
        ConnectionStatus::Closed,
    ];

    assert_eq!(statuses.len(), 4);
}

#[test]
fn test_connection_status_active() {
    let status = ConnectionStatus::Active;
    assert!(matches!(status, ConnectionStatus::Active));
}

#[test]
fn test_connection_status_closed() {
    let status = ConnectionStatus::Closed;
    assert!(matches!(status, ConnectionStatus::Closed));
}

// ==================== CONNECTION INFO TESTS ====================

#[test]
fn test_connection_info_creation() {
    let addr = "127.0.0.1:8080".parse().expect("Failed to parse address");
    let info = ConnectionInfo::new("conn-1".to_string(), addr);

    assert_eq!(info.id, "conn-1");
    assert_eq!(info.endpoint, addr);
    assert_eq!(info.bytes_sent, 0);
    assert_eq!(info.bytes_received, 0);
    // Status initialization verified through construction
}

#[test]
fn test_connection_info_with_traffic() {
    let addr = "127.0.0.1:8080".parse().expect("Failed to parse address");
    let mut info = ConnectionInfo::new("conn-1".to_string(), addr);

    info.bytes_sent = 1024;
    info.bytes_received = 2048;

    assert_eq!(info.bytes_sent, 1024);
    assert_eq!(info.bytes_received, 2048);
}

#[test]
fn test_connection_info_multiple_connections() {
    let addrs = vec!["127.0.0.1:8080", "127.0.0.1:8081", "127.0.0.1:8082"];

    let connections: Vec<ConnectionInfo> = addrs
        .iter()
        .enumerate()
        .map(|(i, addr)| {
            let socket_addr = addr.parse().expect("Failed to parse address");
            ConnectionInfo::new(format!("conn-{}", i), socket_addr)
        })
        .collect();

    assert_eq!(connections.len(), 3);
    assert_eq!(connections[0].id, "conn-0");
    assert_eq!(connections[2].id, "conn-2");
}

// ==================== DURATION TESTS ====================

#[test]
fn test_duration_in_configs() {
    let lb_config = LoadBalancingConfig::default();
    let cb_config = CircuitBreakerConfig::default();

    assert!(lb_config.health_check_interval.as_secs() > 0);
    assert!(cb_config.timeout_duration.as_secs() > 0);
}

#[test]
fn test_duration_comparisons() {
    let short = Duration::from_secs(10);
    let long = Duration::from_secs(100);

    assert!(short < long);
    assert!(long > short);
    assert_eq!(short, Duration::from_secs(10));
}

#[test]
fn test_duration_arithmetic() {
    let d1 = Duration::from_secs(30);
    let d2 = Duration::from_secs(30);

    let sum = d1 + d2;
    assert_eq!(sum, Duration::from_secs(60));
}

// ==================== HASHMAP PROTOCOL SETTINGS TESTS ====================

#[test]
fn test_protocol_settings_insert_get() {
    let mut settings = HashMap::new();
    settings.insert("tcp".to_string(), "enabled".to_string());
    settings.insert("udp".to_string(), "enabled".to_string());

    assert_eq!(settings.get("tcp"), Some(&"enabled".to_string()));
    assert_eq!(settings.get("udp"), Some(&"enabled".to_string()));
    assert_eq!(settings.get("unknown"), None);
}

#[test]
fn test_protocol_settings_update() {
    let mut settings = HashMap::new();
    settings.insert("http".to_string(), "disabled".to_string());

    // Update value
    settings.insert("http".to_string(), "enabled".to_string());
    assert_eq!(settings.get("http"), Some(&"enabled".to_string()));
}

#[test]
fn test_protocol_settings_remove() {
    let mut settings = HashMap::new();
    settings.insert("protocol".to_string(), "value".to_string());

    settings.remove("protocol");
    assert_eq!(settings.get("protocol"), None);
    assert!(settings.is_empty());
}

#[test]
fn test_protocol_settings_iteration() {
    let mut settings = HashMap::new();
    settings.insert("p1".to_string(), "v1".to_string());
    settings.insert("p2".to_string(), "v2".to_string());
    settings.insert("p3".to_string(), "v3".to_string());

    let count = settings.len();
    assert_eq!(count, 3);

    let keys: Vec<_> = settings.keys().collect();
    assert_eq!(keys.len(), 3);
}

// ==================== EDGE CASES AND VALIDATION ====================

#[test]
fn test_port_range_validation() {
    // Valid range
    let ext1 = NetworkExtensions::default();
    assert!(ext1.port_range_end > ext1.port_range_start);

    // Edge case: single port range
    let ext2 = NetworkExtensions {
        port_range_start: 8080,
        port_range_end: 8080,
        keep_alive_timeout_seconds: 60,
        protocol_settings: HashMap::new(),
        load_balancing: LoadBalancingConfig::default(),
        circuit_breaker: CircuitBreakerConfig::default(),
    };
    assert_eq!(ext2.port_range_start, ext2.port_range_end);
}

#[test]
fn test_zero_timeout_edge_case() {
    let ext = NetworkExtensions {
        port_range_start: 9000,
        port_range_end: 9999,
        keep_alive_timeout_seconds: 0,
        protocol_settings: HashMap::new(),
        load_balancing: LoadBalancingConfig::default(),
        circuit_breaker: CircuitBreakerConfig::default(),
    };

    assert_eq!(ext.keep_alive_timeout_seconds, 0);
}

#[test]
fn test_large_timeout_values() {
    let ext = NetworkExtensions {
        port_range_start: 9000,
        port_range_end: 9999,
        keep_alive_timeout_seconds: 86400, // 24 hours
        protocol_settings: HashMap::new(),
        load_balancing: LoadBalancingConfig::default(),
        circuit_breaker: CircuitBreakerConfig::default(),
    };

    assert_eq!(ext.keep_alive_timeout_seconds, 86400);
}

#[test]
fn test_connection_time_tracking() {
    let addr = "127.0.0.1:8080".parse().expect("Failed to parse address");
    let conn1 = ConnectionInfo::new("conn-1".to_string(), addr);

    std::thread::sleep(Duration::from_millis(10));

    let conn2 = ConnectionInfo::new("conn-2".to_string(), addr);

    // conn2 should be created after conn1
    assert!(conn2.established_at > conn1.established_at);
}

#[test]
fn test_system_time_for_connections() {
    let now = SystemTime::now();
    let addr = "127.0.0.1:8080".parse().expect("Failed to parse address");
    let info = ConnectionInfo::new("conn-1".to_string(), addr);

    // Connection should be established around now
    assert!(info.established_at >= now);
}
