// **LOAD BALANCING TESTS**
//
// Comprehensive tests for load balancing configuration, strategies, and backend management

#[cfg(test)]
//! Tests module

mod load_balancing_tests {
    use crate::load_balancing::*;
    
    // ==================== LOAD BALANCING STRATEGY TESTS ====================
    
    #[test]
    fn test_strategy_variants() {
        let strategies = vec![
            LoadBalancingStrategy::RoundRobin,
            LoadBalancingStrategy::LeastConnections,
            LoadBalancingStrategy::WeightedRandom,
            LoadBalancingStrategy::IpHash,
        ];
        
        assert_eq!(strategies.len(), 4);
    }
    
    #[test]
    fn test_strategy_equality() {
        assert_eq!(LoadBalancingStrategy::RoundRobin, LoadBalancingStrategy::RoundRobin);
        assert_ne!(LoadBalancingStrategy::RoundRobin, LoadBalancingStrategy::LeastConnections);
    }
    
    #[test]
    fn test_strategy_clone() {
        let strategy1 = LoadBalancingStrategy::WeightedRandom;
        let strategy2 = strategy1.clone();
        assert_eq!(strategy1, strategy2);
    }
    
    #[test]
    fn test_strategy_debug_format() {
        let strategy = LoadBalancingStrategy::IpHash;
        let debug_str = format!("{:?}", strategy);
        assert!(debug_str.contains("IpHash"));
    }
    
    // ==================== LOAD BALANCING CONFIG TESTS ====================
    
    #[test]
    fn test_config_default() {
        let config = LoadBalancingConfig::default();
        
        assert_eq!(config.strategy, LoadBalancingStrategy::RoundRobin);
        assert_eq!(config.health_check_interval_secs, 30);
        assert_eq!(config.max_retries, 3);
    }
    
    #[test]
    fn test_config_custom_strategy() {
        let config = LoadBalancingConfig {
            strategy: LoadBalancingStrategy::LeastConnections,
            health_check_interval_secs: 60,
            max_retries: 5,
        };
        
        assert_eq!(config.strategy, LoadBalancingStrategy::LeastConnections);
        assert_eq!(config.health_check_interval_secs, 60);
        assert_eq!(config.max_retries, 5);
    }
    
    #[test]
    fn test_config_weighted_random() {
        let config = LoadBalancingConfig {
            strategy: LoadBalancingStrategy::WeightedRandom,
            ..Default::default()
        };
        
        assert_eq!(config.strategy, LoadBalancingStrategy::WeightedRandom);
        assert_eq!(config.max_retries, 3); // Default preserved
    }
    
    #[test]
    fn test_config_health_check_intervals() {
        let configs = vec![
            LoadBalancingConfig { health_check_interval_secs: 10, ..Default::default() },
            LoadBalancingConfig { health_check_interval_secs: 30, ..Default::default() },
            LoadBalancingConfig { health_check_interval_secs: 60, ..Default::default() },
            LoadBalancingConfig { health_check_interval_secs: 300, ..Default::default() },
        ];
        
        assert_eq!(configs[0].health_check_interval_secs, 10);
        assert_eq!(configs[1].health_check_interval_secs, 30);
        assert_eq!(configs[2].health_check_interval_secs, 60);
        assert_eq!(configs[3].health_check_interval_secs, 300);
    }
    
    #[test]
    fn test_config_max_retries_boundary() {
        let config_no_retry = LoadBalancingConfig {
            max_retries: 0,
            ..Default::default()
        };
        assert_eq!(config_no_retry.max_retries, 0);
        
        let config_many_retries = LoadBalancingConfig {
            max_retries: 10,
            ..Default::default()
        };
        assert_eq!(config_many_retries.max_retries, 10);
    }
    
    #[test]
    fn test_config_clone() {
        let config1 = LoadBalancingConfig::default();
        let config2 = config1.clone();
        
        assert_eq!(config1.strategy, config2.strategy);
        assert_eq!(config1.health_check_interval_secs, config2.health_check_interval_secs);
    }
    
    // ==================== BACKEND SERVER TESTS ====================
    
    #[test]
    fn test_backend_server_creation() {
        let server = BackendServer {
            id: "server-1".to_string(),
            address: "192.168.1.100:8080".to_string(),
            weight: 100,
            healthy: true,
        };
        
        assert_eq!(server.id, "server-1");
        assert_eq!(server.address, "192.168.1.100:8080");
        assert_eq!(server.weight, 100);
        assert!(server.healthy);
    }
    
    #[test]
    fn test_backend_server_unhealthy() {
        let server = BackendServer {
            id: "server-failed".to_string(),
            address: "192.168.1.200:8080".to_string(),
            weight: 50,
            healthy: false,
        };
        
        assert!(!server.healthy);
        assert_eq!(server.weight, 50);
    }
    
    #[test]
    fn test_backend_server_weight_zero() {
        let server = BackendServer {
            id: "server-disabled".to_string(),
            address: "192.168.1.300:8080".to_string(),
            weight: 0,
            healthy: true,
        };
        
        assert_eq!(server.weight, 0);
        assert!(server.healthy); // Healthy but zero weight (disabled)
    }
    
    #[test]
    fn test_backend_server_high_weight() {
        let server = BackendServer {
            id: "server-priority".to_string(),
            address: "192.168.1.400:8080".to_string(),
            weight: 1000,
            healthy: true,
        };
        
        assert_eq!(server.weight, 1000);
    }
    
    #[test]
    fn test_backend_server_clone() {
        let server1 = BackendServer {
            id: "clone-test".to_string(),
            address: "localhost:8080".to_string(),
            weight: 100,
            healthy: true,
        };
        
        let server2 = server1.clone();
        
        assert_eq!(server1.id, server2.id);
        assert_eq!(server1.address, server2.address);
        assert_eq!(server1.weight, server2.weight);
    }
    
    #[test]
    fn test_backend_server_debug_format() {
        let server = BackendServer {
            id: "debug-test".to_string(),
            address: "example.com:443".to_string(),
            weight: 100,
            healthy: true,
        };
        
        let debug_str = format!("{:?}", server);
        assert!(debug_str.contains("debug-test"));
        assert!(debug_str.contains("example.com:443"));
    }
    
    // ==================== BACKEND LIST MANAGEMENT TESTS ====================
    
    #[test]
    fn test_multiple_backends() {
        let backends = vec![
            BackendServer {
                id: "server-1".to_string(),
                address: "192.168.1.1:8080".to_string(),
                weight: 100,
                healthy: true,
            },
            BackendServer {
                id: "server-2".to_string(),
                address: "192.168.1.2:8080".to_string(),
                weight: 100,
                healthy: true,
            },
            BackendServer {
                id: "server-3".to_string(),
                address: "192.168.1.3:8080".to_string(),
                weight: 100,
                healthy: true,
            },
        ];
        
        assert_eq!(backends.len(), 3);
        assert!(backends.iter().all(|b| b.healthy));
    }
    
    #[test]
    fn test_backend_filtering_healthy() {
        let backends = vec![
            BackendServer { id: "s1".to_string(), address: "addr1".to_string(), weight: 100, healthy: true },
            BackendServer { id: "s2".to_string(), address: "addr2".to_string(), weight: 100, healthy: false },
            BackendServer { id: "s3".to_string(), address: "addr3".to_string(), weight: 100, healthy: true },
        ];
        
        let healthy_backends: Vec<_> = backends.iter().filter(|b| b.healthy).collect();
        assert_eq!(healthy_backends.len(), 2);
    }
    
    #[test]
    fn test_backend_weight_distribution() {
        let backends = vec![
            BackendServer { id: "high".to_string(), address: "h".to_string(), weight: 500, healthy: true },
            BackendServer { id: "medium".to_string(), address: "m".to_string(), weight: 300, healthy: true },
            BackendServer { id: "low".to_string(), address: "l".to_string(), weight: 200, healthy: true },
        ];
        
        let total_weight: u32 = backends.iter().map(|b| b.weight).sum();
        assert_eq!(total_weight, 1000);
    }
    
    #[test]
    fn test_empty_backend_list() {
        let backends: Vec<BackendServer> = vec![];
        assert!(backends.is_empty());
    }
    
    #[test]
    fn test_single_backend() {
        let backends = vec![
            BackendServer {
                id: "only-server".to_string(),
                address: "192.168.1.1:8080".to_string(),
                weight: 100,
                healthy: true,
            },
        ];
        
        assert_eq!(backends.len(), 1);
        assert_eq!(backends[0].id, "only-server");
    }
    
    // ==================== SERIALIZATION TESTS ====================
    
    #[test]
    fn test_strategy_serialization() {
        use serde_json;
        
        let strategy = LoadBalancingStrategy::RoundRobin;
        let serialized = serde_json::to_string(&strategy);
        assert!(serialized.is_ok());
        
        let deserialized: Result<LoadBalancingStrategy, _> = serde_json::from_str(&serialized.unwrap());
        assert!(deserialized.is_ok());
        assert_eq!(deserialized.unwrap(), LoadBalancingStrategy::RoundRobin);
    }
    
    #[test]
    fn test_config_serialization() {
        use serde_json;
        
        let config = LoadBalancingConfig::default();
        let serialized = serde_json::to_string(&config);
        assert!(serialized.is_ok());
        
        let deserialized: Result<LoadBalancingConfig, _> = serde_json::from_str(&serialized.unwrap());
        assert!(deserialized.is_ok());
        
        let recovered = deserialized.unwrap();
        assert_eq!(recovered.strategy, LoadBalancingStrategy::RoundRobin);
        assert_eq!(recovered.max_retries, 3);
    }
    
    #[test]
    fn test_backend_server_serialization() {
        use serde_json;
        
        let server = BackendServer {
            id: "test-server".to_string(),
            address: "localhost:8080".to_string(),
            weight: 100,
            healthy: true,
        };
        
        let serialized = serde_json::to_string(&server);
        assert!(serialized.is_ok());
        
        let deserialized: Result<BackendServer, _> = serde_json::from_str(&serialized.unwrap());
        assert!(deserialized.is_ok());
        
        let recovered = deserialized.unwrap();
        assert_eq!(recovered.id, "test-server");
        assert_eq!(recovered.weight, 100);
    }
    
    // ==================== EDGE CASE TESTS ====================
    
    #[test]
    fn test_config_extreme_health_check_interval() {
        let config_fast = LoadBalancingConfig {
            health_check_interval_secs: 1,
            ..Default::default()
        };
        assert_eq!(config_fast.health_check_interval_secs, 1);
        
        let config_slow = LoadBalancingConfig {
            health_check_interval_secs: 3600, // 1 hour
            ..Default::default()
        };
        assert_eq!(config_slow.health_check_interval_secs, 3600);
    }
    
    #[test]
    fn test_backend_server_empty_id() {
        let server = BackendServer {
            id: "".to_string(),
            address: "192.168.1.1:8080".to_string(),
            weight: 100,
            healthy: true,
        };
        
        assert!(server.id.is_empty());
    }
    
    #[test]
    fn test_backend_server_ipv6_address() {
        let server = BackendServer {
            id: "ipv6-server".to_string(),
            address: "[::1]:8080".to_string(),
            weight: 100,
            healthy: true,
        };
        
        assert!(server.address.contains("::1"));
    }
    
    #[test]
    fn test_all_backends_unhealthy() {
        let backends = vec![
            BackendServer { id: "s1".to_string(), address: "a1".to_string(), weight: 100, healthy: false },
            BackendServer { id: "s2".to_string(), address: "a2".to_string(), weight: 100, healthy: false },
        ];
        
        let healthy_count = backends.iter().filter(|b| b.healthy).count();
        assert_eq!(healthy_count, 0);
    }
}

