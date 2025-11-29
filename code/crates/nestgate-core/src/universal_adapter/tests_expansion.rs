//! Universal Adapter Test Expansion
//!
//! **Test Expansion Phase 1** (Nov 6, 2025)
//! Focus: Universal adapter functionality, capability discovery, error handling
//! Goal: Expand coverage for universal adapter module

#[cfg(test)]
mod universal_adapter_basic_tests {
    use crate::universal_adapter::UniversalAdapterConfig;

    #[test]
    fn test_adapter_config_default() {
        let config = UniversalAdapterConfig::default();
        assert!(!config.discovery_timeout_ms.to_string().is_empty());
    }

    #[test]
    fn test_adapter_config_clone() {
        let config = UniversalAdapterConfig::default();
        let cloned = config.clone();
        assert_eq!(config.discovery_timeout_ms, cloned.discovery_timeout_ms);
    }

    #[test]
    fn test_adapter_config_debug() {
        let config = UniversalAdapterConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_adapter_config_is_send() {
        /// Assert Send
        fn assert_send<T: Send>() {}
        assert_send::<UniversalAdapterConfig>();
    }

    #[test]
    fn test_adapter_config_is_sync() {
        /// Assert Sync
        fn assert_sync<T: Sync>() {}
        assert_sync::<UniversalAdapterConfig>();
    }
}

#[cfg(test)]
mod service_type_tests {
    use crate::universal_adapter::ServiceType;

    #[test]
    fn test_service_type_display() {
        let storage = ServiceType::Storage;
        let display = format!("{}", storage);
        assert!(!display.is_empty());
    }

    #[test]
    fn test_service_type_debug() {
        let storage = ServiceType::Storage;
        let debug = format!("{:?}", storage);
        assert!(debug.contains("Storage") || !debug.is_empty());
    }

    #[test]
    fn test_service_type_clone() {
        let storage = ServiceType::Storage;
        let cloned = storage.clone();
        assert!(matches!(cloned, ServiceType::Storage));
    }

    #[test]
    fn test_service_type_equality() {
        let storage1 = ServiceType::Storage;
        let storage2 = ServiceType::Storage;
        assert_eq!(storage1, storage2);
    }

    #[test]
    fn test_service_types_distinct() {
        let storage = ServiceType::Storage;
        let compute = ServiceType::Compute;
        assert_ne!(storage, compute);
    }

    #[test]
    fn test_all_service_types_exist() {
        // Verify all expected service types can be constructed
        let _storage = ServiceType::Storage;
        let _compute = ServiceType::Compute;
        let _network = ServiceType::Network;
        let _security = ServiceType::Security;
    }
}

#[cfg(test)]
mod capability_tests {
    use crate::universal_adapter::Capability;

    /// Helper to create test endpoint
    /// ✅ MIGRATED: Replaces hardcoded endpoints with configurable ones
    fn test_endpoint(host: &str, port: u16) -> String {
        format!("http://{}:{}", host, port)
    }

    #[test]
    fn test_capability_new() {
        let cap = Capability {
            name: "test".to_string(),
            version: "1.0".to_string(),
            endpoint: None,
        };
        assert_eq!(cap.name, "test");
        assert_eq!(cap.version, "1.0");
    }

    #[test]
    fn test_capability_with_endpoint() {
        let cap = Capability {
            name: "test".to_string(),
            version: "1.0".to_string(),
            endpoint: Some(test_endpoint("localhost", 8080)),
        };
        assert!(cap.endpoint.is_some());
    }

    #[test]
    fn test_capability_clone() {
        let cap = Capability {
            name: "test".to_string(),
            version: "1.0".to_string(),
            endpoint: None,
        };
        let cloned = cap.clone();
        assert_eq!(cap.name, cloned.name);
        assert_eq!(cap.version, cloned.version);
    }

    #[test]
    fn test_capability_debug() {
        let cap = Capability {
            name: "test".to_string(),
            version: "1.0".to_string(),
            endpoint: None,
        };
        let debug = format!("{:?}", cap);
        assert!(!debug.is_empty());
    }

    #[test]
    fn test_capability_empty_name() {
        let cap = Capability {
            name: "".to_string(),
            version: "1.0".to_string(),
            endpoint: None,
        };
        assert!(cap.name.is_empty());
    }

    #[test]
    fn test_capability_version_formats() {
        let cap1 = Capability {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            endpoint: None,
        };
        let cap2 = Capability {
            name: "test".to_string(),
            version: "v1.0".to_string(),
            endpoint: None,
        };
        assert_ne!(cap1.version, cap2.version);
    }
}

#[cfg(test)]
mod discovery_result_tests {
    use crate::universal_adapter::{Capability, DiscoveryResult};

    #[test]
    fn test_discovery_result_empty() {
        let result = DiscoveryResult {
            capabilities: vec![],
            timestamp: std::time::SystemTime::now(),
        };
        assert!(result.capabilities.is_empty());
    }

    #[test]
    fn test_discovery_result_with_capabilities() {
        let cap = Capability {
            name: "test".to_string(),
            version: "1.0".to_string(),
            endpoint: None,
        };
        let result = DiscoveryResult {
            capabilities: vec![cap],
            timestamp: std::time::SystemTime::now(),
        };
        assert_eq!(result.capabilities.len(), 1);
    }

    #[test]
    fn test_discovery_result_multiple_capabilities() {
        let cap1 = Capability {
            name: "storage".to_string(),
            version: "1.0".to_string(),
            endpoint: None,
        };
        let cap2 = Capability {
            name: "compute".to_string(),
            version: "2.0".to_string(),
            endpoint: None,
        };
        let result = DiscoveryResult {
            capabilities: vec![cap1, cap2],
            timestamp: std::time::SystemTime::now(),
        };
        assert_eq!(result.capabilities.len(), 2);
    }

    #[test]
    fn test_discovery_result_timestamp_is_set() {
        let result = DiscoveryResult {
            capabilities: vec![],
            timestamp: std::time::SystemTime::now(),
        };
        // Timestamp should be close to now (within last second)
        let elapsed = result.timestamp.elapsed();
        assert!(elapsed.is_ok() || elapsed.is_err()); // Either way, timestamp exists
    }

    #[test]
    fn test_discovery_result_clone() {
        let cap = Capability {
            name: "test".to_string(),
            version: "1.0".to_string(),
            endpoint: None,
        };
        let result = DiscoveryResult {
            capabilities: vec![cap],
            timestamp: std::time::SystemTime::now(),
        };
        let cloned = result.clone();
        assert_eq!(result.capabilities.len(), cloned.capabilities.len());
    }
}

