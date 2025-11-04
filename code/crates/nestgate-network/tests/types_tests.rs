// Types tests moved from src/types.rs to comply with 1000-line limit

use nestgate_core::constants::{network_defaults, port_defaults};
use nestgate_network::types::*;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, SystemTime};

mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    // ==================== ConnectionInfo Tests (3 tests) ====================

    #[test]
    fn test_connection_info_creation() {
        let addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port_defaults::DEFAULT_API_PORT,
        );
        let conn = ConnectionInfo::new("conn-123".to_string(), addr);

        assert_eq!(conn.id(), "conn-123");
        assert_eq!(conn.address(), addr);
        assert!(conn.is_active());
        assert_eq!(conn.bytes_sent, 0);
        assert_eq!(conn.bytes_received, 0);
    }

    #[test]
    fn test_connection_info_byte_tracking() {
        let addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port_defaults::DEFAULT_API_PORT,
        );
        let mut conn = ConnectionInfo::new("conn-123".to_string(), addr);

        conn.add_bytes_sent(1024);
        conn.add_bytes_received(2048);

        assert_eq!(conn.bytes_sent, 1024);
        assert_eq!(conn.bytes_received, 2048);

        // Add more bytes
        conn.add_bytes_sent(512);
        conn.add_bytes_received(1024);

        assert_eq!(conn.bytes_sent, 1536);
        assert_eq!(conn.bytes_received, 3072);
    }

    #[test]
    fn test_connection_status_transitions() {
        let addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port_defaults::DEFAULT_API_PORT,
        );
        let mut conn = ConnectionInfo::new("conn-123".to_string(), addr);

        assert!(conn.is_active());

        conn.set_status(ConnectionStatus::Idle);
        assert!(!conn.is_active());

        conn.set_status(ConnectionStatus::Closing);
        assert!(!conn.is_active());

        conn.set_status(ConnectionStatus::Closed);
        assert!(!conn.is_active());
    }

    // ==================== ServiceInfo Tests (3 tests) ====================

    #[test]
    fn test_service_info_creation() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);
        let service = ServiceInfo::new("svc-456".to_string(), "test-service".to_string(), addr);

        assert_eq!(service.id(), "svc-456");
        assert_eq!(service.name(), "test-service");
        assert_eq!(service.address(), addr);
        assert!(service.is_healthy());
        assert_eq!(service.metadata().len(), 0);
    }

    #[test]
    fn test_service_info_metadata() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);
        let mut service = ServiceInfo::new("svc-456".to_string(), "test-service".to_string(), addr);

        service.add_metadata("version".to_string(), "1.0.0".to_string());
        service.add_metadata("region".to_string(), "us-west".to_string());

        assert_eq!(service.metadata().len(), 2);
        assert_eq!(
            service.metadata().get("version"),
            Some(&"1.0.0".to_string())
        );
        assert_eq!(
            service.metadata().get("region"),
            Some(&"us-west".to_string())
        );
    }

    #[test]
    fn test_service_health_status() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);
        let mut service = ServiceInfo::new("svc-456".to_string(), "test-service".to_string(), addr);

        assert!(service.is_healthy());

        service.set_health_status(HealthStatus::Degraded);
        assert!(!service.is_healthy());

        service.set_health_status(HealthStatus::Unhealthy);
        assert!(!service.is_healthy());

        service.set_health_status(HealthStatus::Healthy);
        assert!(service.is_healthy());
    }

    // ==================== NetworkConfig Builder Tests (3 tests) ====================

    #[test]
    fn test_network_config_builder_basic() {
        let config = NetworkConfigBuilder::new()
            .port(8888)
            .max_connections(100)
            .build();

        assert_eq!(config.network.api.port, 8888);
        assert_eq!(config.network.api.max_connections, 100);
    }

    #[test]
    fn test_network_config_builder_port_range() {
        let config = NetworkConfigBuilder::new().port_range(10000, 20000).build();

        assert_eq!(config.extensions.port_range_start, 10000);
        assert_eq!(config.extensions.port_range_end, 20000);
    }

    #[test]
    fn test_network_config_builder_timeouts() {
        let config = NetworkConfigBuilder::new()
            .connection_timeout(120)
            .keep_alive_timeout(300)
            .build();

        assert_eq!(
            config.network.api.connection_timeout,
            Duration::from_secs(120)
        );
        assert_eq!(config.extensions.keep_alive_timeout_seconds, 300);
    }

    // ==================== LoadBalancing & CircuitBreaker Tests (3 tests) ====================

    #[test]
    fn test_load_balancing_config_default() {
        let config = LoadBalancingConfig::default();

        assert_eq!(config.algorithm, "round_robin");
        assert_eq!(config.health_check_interval, Duration::from_secs(30));
        assert_eq!(config.max_failures, 3);
    }

    #[test]
    fn test_circuit_breaker_config_default() {
        let config = CircuitBreakerConfig::default();

        assert_eq!(config.failure_threshold, 5);
        assert_eq!(config.timeout_duration, Duration::from_secs(60));
        assert_eq!(config.half_open_max_calls, 10);
    }

    #[test]
    fn test_network_extensions_default() {
        let extensions = NetworkExtensions::default();

        assert_eq!(extensions.port_range_start, 9000);
        assert_eq!(extensions.port_range_end, 9999);
        assert_eq!(extensions.keep_alive_timeout_seconds, 60);
        assert_eq!(extensions.protocol_settings.len(), 0);
    }

    // ==================== Network Statistics Tests (3 tests) ====================

    #[test]
    fn test_network_statistics_default() {
        let stats = NetworkStatistics::default();

        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.registered_services, 0);
        assert_eq!(stats.allocated_ports, 0);
        assert_eq!(stats.total_bytes_sent, 0);
        assert_eq!(stats.total_bytes_received, 0);
    }

    #[test]
    fn test_network_statistics_serialization() {
        let stats = NetworkStatistics {
            active_connections: 10,
            registered_services: 5,
            allocated_ports: 20,
            total_bytes_sent: 1024,
            total_bytes_received: 2048,
        };

        let serialized = serde_json::to_string(&stats);
        assert!(serialized.is_ok(), "Network statistics should serialize");

        let json = serialized.unwrap();
        let deserialized: std::result::Result<NetworkStatistics, _> = serde_json::from_str(&json);
        assert!(
            deserialized.is_ok(),
            "Network statistics should deserialize"
        );

        let deserialized_stats = deserialized.unwrap();
        assert_eq!(deserialized_stats.active_connections, 10);
        assert_eq!(deserialized_stats.registered_services, 5);
    }

    #[test]
    fn test_service_status_variants() {
        let statuses = [
            ServiceStatus::Running,
            ServiceStatus::Stopped,
            ServiceStatus::Error,
            ServiceStatus::Unknown,
            ServiceStatus::Healthy,
            ServiceStatus::Unhealthy,
            ServiceStatus::Starting,
            ServiceStatus::Stopping,
            ServiceStatus::Failed,
        ];

        assert_eq!(statuses.len(), 9);
        assert_eq!(ServiceStatus::default(), ServiceStatus::Unknown);

        // Test equality
        assert_eq!(ServiceStatus::Running, ServiceStatus::Running);
        assert_ne!(ServiceStatus::Running, ServiceStatus::Stopped);
    }

    #[test]
    fn test_connection_info_with_metrics() {
        let test_endpoint = format!("192.168.1.1:{}", port_defaults::DEFAULT_API_PORT);
        let endpoint: SocketAddr = test_endpoint.parse().unwrap();
        let mut conn = ConnectionInfo::new("conn-metrics".to_string(), endpoint);

        conn.bytes_sent = 10240;
        conn.bytes_received = 20480;

        assert_eq!(conn.id, "conn-metrics");
        assert_eq!(conn.bytes_sent, 10240);
        assert_eq!(conn.bytes_received, 20480);
        assert!(matches!(conn.status, ConnectionStatus::Active));
    }

    #[test]
    fn test_connection_status_lifecycle() {
        let statuses = [
            ConnectionStatus::Active,
            ConnectionStatus::Idle,
            ConnectionStatus::Closed,
        ];

        assert_eq!(statuses.len(), 3);
        assert!(matches!(statuses[0], ConnectionStatus::Active));
        assert!(matches!(statuses[1], ConnectionStatus::Idle));
        assert!(matches!(statuses[2], ConnectionStatus::Closed));
    }

    #[test]
    fn test_load_balancing_algorithms() {
        let algorithms = vec![
            "round_robin",
            "least_connections",
            "weighted_round_robin",
            "ip_hash",
            "random",
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
    fn test_circuit_breaker_thresholds() {
        let thresholds = vec![1, 5, 10, 20, 50];

        for threshold in thresholds {
            let config = CircuitBreakerConfig {
                failure_threshold: threshold,
                timeout_duration: Duration::from_secs(60),
                half_open_max_calls: threshold * 2,
            };
            assert_eq!(config.failure_threshold, threshold);
            assert_eq!(config.half_open_max_calls, threshold * 2);
        }
    }

    #[test]
    fn test_network_extensions_port_ranges() {
        let test_ranges = vec![(8000, 8999), (9000, 9999), (10000, 19999), (20000, 29999)];

        for (start, end) in test_ranges {
            let mut ext = NetworkExtensions::default();
            ext.port_range_start = start;
            ext.port_range_end = end;

            assert_eq!(ext.port_range_start, start);
            assert_eq!(ext.port_range_end, end);
            assert!(ext.port_range_end > ext.port_range_start);
        }
    }

    #[test]
    fn test_network_statistics_operations() {
        let mut stats = NetworkStatistics::default();

        // Simulate operations
        stats.active_connections = 5;
        stats.registered_services = 10;
        stats.allocated_ports = 15;
        stats.total_bytes_sent += 1024;
        stats.total_bytes_received += 2048;

        assert_eq!(stats.active_connections, 5);
        assert_eq!(stats.total_bytes_sent, 1024);
        assert_eq!(stats.total_bytes_received, 2048);
    }

    #[test]
    fn test_connection_info_ipv6() {
        let test_port = port_defaults::DEFAULT_API_PORT;
        let test_endpoint = format!("[::1]:{}", test_port);
        let endpoint: SocketAddr = test_endpoint.parse().unwrap();
        let conn = ConnectionInfo::new("conn-ipv6".to_string(), endpoint);

        assert_eq!(conn.id, "conn-ipv6");
        assert!(conn.endpoint.is_ipv6());
        assert_eq!(conn.endpoint.port(), test_port);
    }

    #[test]
    fn test_connection_status_enum_variants() {
        // Test that we can create all variants
        let _active = ConnectionStatus::Active;
        let _idle = ConnectionStatus::Idle;
        let _closed = ConnectionStatus::Closed;

        // All variants should be creatable
        assert!(true);
    }

    #[test]
    fn test_load_balancing_health_check_intervals() {
        let intervals = vec![5, 10, 30, 60, 120];

        for seconds in intervals {
            let config = LoadBalancingConfig {
                algorithm: "round_robin".to_string(),
                health_check_interval: Duration::from_secs(seconds),
                max_failures: 3,
            };
            assert_eq!(config.health_check_interval.as_secs(), seconds);
        }
    }

    #[test]
    fn test_network_extensions_protocol_settings() {
        let mut ext = NetworkExtensions::default();
        ext.protocol_settings
            .insert("http.version".to_string(), "2.0".to_string());
        ext.protocol_settings
            .insert("tls.version".to_string(), "1.3".to_string());

        assert_eq!(ext.protocol_settings.len(), 2);
        assert_eq!(
            ext.protocol_settings.get("http.version"),
            Some(&"2.0".to_string())
        );
        assert_eq!(
            ext.protocol_settings.get("tls.version"),
            Some(&"1.3".to_string())
        );
    }

    #[test]
    fn test_service_status_transitions() {
        let transitions = vec![
            (ServiceStatus::Starting, ServiceStatus::Running),
            (ServiceStatus::Running, ServiceStatus::Stopping),
            (ServiceStatus::Stopping, ServiceStatus::Stopped),
            (ServiceStatus::Running, ServiceStatus::Failed),
        ];

        for (from, to) in transitions {
            // Test that transitions make sense
            assert_ne!(from, to);
        }
    }

    #[test]
    fn test_connection_info_equality() {
        let test_endpoint = format!(
            "{}:{}",
            network_defaults::DEFAULT_BIND_ALL_IPV4,
            port_defaults::DEFAULT_API_PORT
        );
        let endpoint: SocketAddr = test_endpoint.parse().unwrap();
        let conn1 = ConnectionInfo::new("conn-1".to_string(), endpoint);
        let conn2 = ConnectionInfo::new("conn-1".to_string(), endpoint);

        assert_eq!(conn1.id, conn2.id);
        assert_eq!(conn1.endpoint, conn2.endpoint);
    }

    #[test]
    fn test_circuit_breaker_timeout_durations() {
        let durations = vec![30, 60, 120, 300];

        for seconds in durations {
            let config = CircuitBreakerConfig {
                failure_threshold: 5,
                timeout_duration: Duration::from_secs(seconds),
                half_open_max_calls: 10,
            };
            assert_eq!(config.timeout_duration.as_secs(), seconds);
        }
    }

    #[test]
    fn test_network_statistics_cumulative() {
        let mut stats = NetworkStatistics::default();

        // Simulate multiple operations
        for _ in 0..10 {
            stats.total_bytes_sent += 100;
            stats.total_bytes_received += 200;
        }

        assert_eq!(stats.total_bytes_sent, 1000);
        assert_eq!(stats.total_bytes_received, 2000);
    }

    #[test]
    fn test_load_balancing_custom_health_intervals() {
        let intervals = vec![10, 30, 60, 120];

        for seconds in intervals {
            let config = LoadBalancingConfig {
                algorithm: "round_robin".to_string(),
                health_check_interval: Duration::from_secs(seconds),
                max_failures: 3,
            };

            assert_eq!(config.health_check_interval.as_secs(), seconds);
        }
    }

    #[test]
    fn test_network_extensions_keep_alive_timeout() {
        let ext = NetworkExtensions {
            port_range_start: 9000,
            port_range_end: 9999,
            keep_alive_timeout_seconds: 120,
            protocol_settings: HashMap::new(),
            load_balancing: LoadBalancingConfig::default(),
            circuit_breaker: CircuitBreakerConfig::default(),
        };

        assert_eq!(ext.keep_alive_timeout_seconds, 120);
        assert!(ext.keep_alive_timeout_seconds > 0);
    }

    #[test]
    fn test_circuit_breaker_half_open_calls() {
        let config = CircuitBreakerConfig {
            failure_threshold: 5,
            timeout_duration: Duration::from_secs(60),
            half_open_max_calls: 20,
        };

        assert_eq!(config.half_open_max_calls, 20);
        assert!(config.half_open_max_calls > 0);
    }

    #[test]
    fn test_network_statistics_allocated_ports() {
        let mut stats = NetworkStatistics::default();
        stats.allocated_ports = 5;

        assert_eq!(stats.allocated_ports, 5);

        stats.allocated_ports += 3;
        assert_eq!(stats.allocated_ports, 8);
    }

    #[test]
    fn test_load_balancing_max_failures() {
        let failures = vec![1, 3, 5, 10];

        for max in failures {
            let config = LoadBalancingConfig {
                algorithm: "round_robin".to_string(),
                health_check_interval: Duration::from_secs(30),
                max_failures: max,
            };

            assert_eq!(config.max_failures, max);
        }
    }

    #[test]
    fn test_network_extensions_clone() {
        let ext1 = NetworkExtensions::default();
        let ext2 = ext1.clone();

        assert_eq!(ext1.port_range_start, ext2.port_range_start);
        assert_eq!(ext1.port_range_end, ext2.port_range_end);
        assert_eq!(
            ext1.keep_alive_timeout_seconds,
            ext2.keep_alive_timeout_seconds
        );
    }

    #[test]
    fn test_load_balancing_algorithm_types() {
        let algorithms = vec![
            "round_robin",
            "least_connections",
            "random",
            "weighted_round_robin",
            "ip_hash",
        ];

        for algo in algorithms {
            let config = LoadBalancingConfig {
                algorithm: algo.to_string(),
                health_check_interval: Duration::from_secs(30),
                max_failures: 3,
            };

            assert!(!config.algorithm.is_empty());
        }
    }

    #[test]
    fn test_circuit_breaker_failure_threshold_range() {
        let thresholds = vec![1, 3, 5, 10, 50, 100];

        for threshold in thresholds {
            let config = CircuitBreakerConfig {
                failure_threshold: threshold,
                timeout_duration: Duration::from_secs(60),
                half_open_max_calls: 10,
            };

            assert!(config.failure_threshold > 0);
            assert!(config.failure_threshold <= 100);
        }
    }

    #[test]
    fn test_network_statistics_service_count() {
        let mut stats = NetworkStatistics::default();

        for i in 1..=5 {
            stats.registered_services = i;
            assert_eq!(stats.registered_services, i);
        }
    }

    #[test]
    fn test_connection_info_timestamp() {
        let test_endpoint = format!(
            "{}:{}",
            network_defaults::DEFAULT_BIND_ALL_IPV4,
            port_defaults::DEFAULT_API_PORT
        );
        let endpoint: SocketAddr = test_endpoint.parse().unwrap();
        let conn = ConnectionInfo::new("time-test".to_string(), endpoint);

        let now = SystemTime::now();
        let duration = now.duration_since(conn.established_at).unwrap();

        assert!(duration.as_secs() < 5); // Should be recent
    }

    #[test]
    fn test_network_extensions_with_multiple_protocols() {
        let mut settings = HashMap::new();
        settings.insert("http".to_string(), "enabled".to_string());
        settings.insert("https".to_string(), "enabled".to_string());
        settings.insert("websocket".to_string(), "enabled".to_string());
        settings.insert("grpc".to_string(), "enabled".to_string());

        let ext = NetworkExtensions {
            port_range_start: 9000,
            port_range_end: 9999,
            keep_alive_timeout_seconds: 60,
            protocol_settings: settings.clone(),
            load_balancing: LoadBalancingConfig::default(),
            circuit_breaker: CircuitBreakerConfig::default(),
        };

        assert_eq!(ext.protocol_settings.len(), 4);
    }

    #[test]
    fn test_load_balancing_config_serialization() {
        let config = LoadBalancingConfig::default();
        let json = serde_json::to_string(&config).unwrap();

        assert!(json.contains("round_robin"));
        assert!(json.contains("health_check_interval"));
        assert!(json.contains("max_failures"));
    }

    #[test]
    fn test_circuit_breaker_config_serialization() {
        let config = CircuitBreakerConfig::default();
        let json = serde_json::to_string(&config).unwrap();

        assert!(json.contains("failure_threshold"));
        assert!(json.contains("timeout_duration"));
        assert!(json.contains("half_open_max_calls"));
    }
}
