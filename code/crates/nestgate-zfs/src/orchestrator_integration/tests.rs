// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Tests for orchestrator integration
//!
//! Comprehensive test coverage for service registration, health reporting,
//! and orchestrator coordination functionality.

use super::types::{ServiceInfo, ServiceRegistration, ZfsHealthStatus, ZfsServiceConfig};

// Test constants - environment-driven configuration
mod test_constants {
    /// Test server port (distinct from production 8080)
    pub const TEST_PORT: u16 = 18080;
    pub const TEST_HOST: &str = "localhost";

    /// Generate test endpoint
    pub fn endpoint() -> String {
        std::env::var("NESTGATE_TEST_ENDPOINT")
            .unwrap_or_else(|_| format!("http://{}:{}", TEST_HOST, TEST_PORT))
    }

    /// Generate test endpoint with path
    pub fn endpoint_with_path(path: &str) -> String {
        format!("{}{}", endpoint(), path)
    }
}

#[cfg(test)]
mod service_registration_tests {
    use super::*;
    use test_constants::endpoint;

    #[test]
    fn test_service_registration_creation() {
        let registration = ServiceRegistration {
            service_id: "svc-001".to_string(),
            service_type: "zfs-storage".to_string(),
            capabilities: vec!["pool-management".to_string(), "snapshots".to_string()],
            endpoints: vec![endpoint()],
            metadata: std::collections::HashMap::new(),
        };

        assert_eq!(registration.service_id, "svc-001");
        assert_eq!(registration.service_type, "zfs-storage");
        assert_eq!(registration.capabilities.len(), 2);
        assert_eq!(registration.endpoints.len(), 1);
    }

    #[test]
    fn test_service_registration_with_metadata() {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("region".to_string(), "us-west".to_string());
        metadata.insert("zone".to_string(), "zone-a".to_string());

        let registration = ServiceRegistration {
            service_id: "svc-002".to_string(),
            service_type: "zfs-compute".to_string(),
            capabilities: vec!["tier-management".to_string()],
            endpoints: vec!["http://node1:9091".to_string()],
            metadata,
        };

        assert_eq!(registration.metadata.len(), 2);
        assert_eq!(
            registration.metadata.get("region"),
            Some(&"us-west".to_string())
        );
        assert_eq!(
            registration.metadata.get("zone"),
            Some(&"zone-a".to_string())
        );
    }

    #[test]
    fn test_service_registration_empty_capabilities() {
        let registration = ServiceRegistration {
            service_id: "svc-003".to_string(),
            service_type: "zfs-storage".to_string(),
            capabilities: vec![],
            endpoints: vec![endpoint()],
            metadata: std::collections::HashMap::new(),
        };

        assert!(registration.capabilities.is_empty());
    }

    #[test]
    fn test_service_registration_multiple_endpoints() {
        let registration = ServiceRegistration {
            service_id: "svc-004".to_string(),
            service_type: "zfs-storage".to_string(),
            capabilities: vec!["pool-management".to_string()],
            endpoints: vec![
                format!(
                    "http://localhost:{}",
                    nestgate_core::constants::network_hardcoded::ports::HTTP_DEFAULT
                ),
                format!(
                    "http://localhost:{}",
                    nestgate_core::constants::network_hardcoded::ports::HEALTH_CHECK_DEFAULT
                ),
                format!("https://node1.cluster.local:{}", 9443), // Cluster port - site-specific
            ],
            metadata: std::collections::HashMap::new(),
        };

        assert_eq!(registration.endpoints.len(), 3);
        assert!(
            registration
                .endpoints
                .contains(&"http://localhost:8080".to_string())
        );
        assert!(
            registration
                .endpoints
                .contains(&"https://node1.cluster.local:9443".to_string())
        );
    }

    #[test]
    fn test_service_registration_serialization() {
        let registration = ServiceRegistration {
            service_id: "svc-005".to_string(),
            service_type: "zfs-storage".to_string(),
            capabilities: vec!["snapshots".to_string()],
            endpoints: vec!["http://localhost:8080".to_string()],
            metadata: std::collections::HashMap::new(),
        };

        let serialized = serde_json::to_string(&registration);
        assert!(serialized.is_ok(), "Serialization should succeed");

        // Migrated from .expect() for better test ergonomics
        let json = serialized.unwrap();
        assert!(json.contains("svc-005"));
        assert!(json.contains("zfs-storage"));
        assert!(json.contains("snapshots"));
    }

    #[test]
    fn test_service_registration_deserialization() {
        let json = r#"{
            "service_id": "svc-006",
            "service_type": "zfs-storage",
            "capabilities": ["pool-management"],
            "endpoints": ["http://localhost:8080"],
            "metadata": {}
        }"#;

        let result: Result<ServiceRegistration, _> = serde_json::from_str(json);
        assert!(result.is_ok(), "Deserialization should succeed");

        let registration = result.expect("Deserialization succeeded");
        assert_eq!(registration.service_id, "svc-006");
        assert_eq!(registration.service_type, "zfs-storage");
    }
}

#[cfg(test)]
mod health_status_tests {
    use super::*;

    #[test]
    fn test_health_status_healthy() {
        let health = ZfsHealthStatus {
            node_id: "node-001".to_string(),
            status: "healthy".to_string(),
            pools_healthy: true,
            datasets_healthy: true,
            system_healthy: true,
            total_capacity: 10_000_000_000,
            available_capacity: 5_000_000_000,
            last_check: 1638360000,
        };

        assert_eq!(health.status, "healthy");
        assert!(health.pools_healthy);
        assert!(health.datasets_healthy);
        assert!(health.system_healthy);
        assert_eq!(health.total_capacity, 10_000_000_000);
        assert_eq!(health.available_capacity, 5_000_000_000);
    }

    #[test]
    fn test_health_status_degraded() {
        let health = ZfsHealthStatus {
            node_id: "node-002".to_string(),
            status: "degraded".to_string(),
            pools_healthy: true,
            datasets_healthy: false,
            system_healthy: true,
            total_capacity: 20_000_000_000,
            available_capacity: 1_000_000_000,
            last_check: 1638360100,
        };

        assert_eq!(health.status, "degraded");
        assert!(!health.datasets_healthy);
        assert!(health.total_capacity > health.available_capacity);
    }

    #[test]
    fn test_health_status_unhealthy() {
        let health = ZfsHealthStatus {
            node_id: "node-003".to_string(),
            status: "unhealthy".to_string(),
            pools_healthy: false,
            datasets_healthy: false,
            system_healthy: false,
            total_capacity: 10_000_000_000,
            available_capacity: 0,
            last_check: 1638360200,
        };

        assert_eq!(health.status, "unhealthy");
        assert!(!health.pools_healthy);
        assert!(!health.datasets_healthy);
        assert!(!health.system_healthy);
        assert_eq!(health.available_capacity, 0);
    }

    #[test]
    fn test_health_status_serialization() {
        let health = ZfsHealthStatus {
            node_id: "node-004".to_string(),
            status: "healthy".to_string(),
            pools_healthy: true,
            datasets_healthy: true,
            system_healthy: true,
            total_capacity: 10_000_000_000,
            available_capacity: 5_000_000_000,
            last_check: 1638360300,
        };

        let serialized = serde_json::to_string(&health);
        assert!(
            serialized.is_ok(),
            "Health status serialization should succeed"
        );
    }

    #[test]
    fn test_health_status_capacity_calculation() {
        let health = ZfsHealthStatus {
            node_id: "node-005".to_string(),
            status: "healthy".to_string(),
            pools_healthy: true,
            datasets_healthy: true,
            system_healthy: true,
            total_capacity: 10_000_000_000,
            available_capacity: 3_000_000_000,
            last_check: 1638360400,
        };

        let used_capacity = health.total_capacity - health.available_capacity();
        let usage_percent = (used_capacity as f64 / health.total_capacity as f64) * 100.0;

        assert_eq!(used_capacity, 7_000_000_000);
        assert!((usage_percent - 70.0).abs() < 0.01);
    }
}

#[cfg(test)]
mod service_info_tests {
    use super::*;

    #[test]
    fn test_service_info_new() {
        let info = ServiceInfo::new("svc-100", "zfs-storage");

        assert_eq!(info.service_id, "svc-100");
        assert_eq!(info.service_type, "zfs-storage");
        assert!(info.endpoints.is_empty());
        assert!(info.capabilities.is_empty());
        assert!(info.metadata.is_empty());
        assert_eq!(info.last_heartbeat, None);
    }

    #[test]
    fn test_service_info_generate_id() {
        let id1 = ServiceInfo::generate_id();
        let id2 = ServiceInfo::generate_id();

        assert_ne!(id1, id2, "Generated IDs should be unique");
        assert!(!id1.is_empty());
        assert!(!id2.is_empty());
    }

    #[test]
    fn test_service_info_with_heartbeat() {
        let mut info = ServiceInfo::new("svc-101", "zfs-compute");
        info.last_heartbeat = Some(1638360500);

        assert_eq!(info.last_heartbeat, Some(1638360500));
    }

    #[test]
    fn test_service_info_add_capabilities() {
        let mut info = ServiceInfo::new("svc-102", "zfs-storage");
        info.capabilities.push("pool-management".to_string());
        info.capabilities.push("snapshots".to_string());
        info.capabilities.push("tier-management".to_string());

        assert_eq!(info.capabilities.len(), 3);
        assert!(info.capabilities.contains(&"pool-management".to_string()));
        assert!(info.capabilities.contains(&"snapshots".to_string()));
    }

    #[test]
    fn test_service_info_add_endpoints() {
        let mut info = ServiceInfo::new("svc-103", "zfs-storage");
        info.endpoints.push("http://localhost:8080".to_string());
        info.endpoints.push("http://node1:9091".to_string());

        assert_eq!(info.endpoints.len(), 2);
    }

    #[test]
    fn test_service_info_add_metadata() {
        let mut info = ServiceInfo::new("svc-104", "zfs-storage");
        info.metadata
            .insert("datacenter".to_string(), "dc1".to_string());
        info.metadata.insert("rack".to_string(), "r42".to_string());

        assert_eq!(info.metadata.len(), 2);
        assert_eq!(info.metadata.get("datacenter"), Some(&"dc1".to_string()));
    }

    #[test]
    fn test_service_info_serialization_roundtrip() {
        let mut info = ServiceInfo::new("svc-105", "zfs-storage");
        info.capabilities.push("snapshots".to_string());
        info.endpoints.push("http://localhost:8080".to_string());
        info.last_heartbeat = Some(1638360600);

        let serialized = serde_json::to_string(&info).expect("Serialization should succeed");
        let deserialized: ServiceInfo =
            serde_json::from_str(&serialized).expect("Deserialization should succeed");

        assert_eq!(deserialized.service_id, info.service_id);
        assert_eq!(deserialized.service_type, info.service_type);
        assert_eq!(deserialized.capabilities, info.capabilities);
        assert_eq!(deserialized.endpoints, info.endpoints);
        assert_eq!(deserialized.last_heartbeat, info.last_heartbeat);
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_zfs_service_config_default() {
        let config = ZfsServiceConfig::default();

        assert_eq!(config.service_name, "nestgate-zfs");
        assert!(!config.bind_address.is_empty());
        assert!(config.port > 0);
        assert_eq!(config.health_check_interval, 30);
        assert!(!config.capabilities.is_empty());
    }

    #[test]
    #[allow(deprecated)]
    fn test_zfs_service_config_custom() {
        let mut config = ZfsServiceConfig::default();
        config.service_name = "custom-zfs".to_string();
        config.bind_address = "192.168.1.100".to_string();
        config.port = 9999;
        config.health_check_interval = 60;

        assert_eq!(config.service_name, "custom-zfs");
        assert_eq!(config.bind_address, "192.168.1.100");
        assert_eq!(config.port, 9999);
        assert_eq!(config.health_check_interval, 60);
    }

    #[test]
    #[allow(deprecated)]
    fn test_zfs_service_config_capabilities() {
        let config = ZfsServiceConfig::default();

        assert!(
            config
                .capabilities
                .contains(&"zfs-pool-management".to_string())
        );
        assert!(
            config
                .capabilities
                .contains(&"zfs-dataset-management".to_string())
        );
        assert!(
            config
                .capabilities
                .contains(&"zfs-snapshot-management".to_string())
        );
        assert!(config.capabilities.contains(&"tier-management".to_string()));
    }

    #[test]
    #[allow(deprecated)]
    fn test_zfs_service_config_add_orchestrator_endpoint() {
        let mut config = ZfsServiceConfig::default();
        config
            .orchestrator_endpoints
            // ✅ FIXED: Use generic orchestrator endpoint, not primal name
            .push("http://orchestrator:9091".to_string());
        config
            .orchestrator_endpoints
            .push("http://k8s-api:6443".to_string());

        assert_eq!(config.orchestrator_endpoints.len(), 2);
    }

    #[test]
    #[allow(deprecated)]
    fn test_zfs_service_config_add_metadata() {
        let mut config = ZfsServiceConfig::default();
        config
            .metadata
            .insert("environment".to_string(), "production".to_string());
        config
            .metadata
            .insert("version".to_string(), "0.10.0".to_string());

        assert_eq!(config.metadata.len(), 2);
        assert_eq!(
            config.metadata.get("environment"),
            Some(&"production".to_string())
        );
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_service_registration_very_long_id() {
        let long_id = "svc-".to_string() + &"x".repeat(1000);
        let registration = ServiceRegistration {
            service_id: long_id.clone(),
            service_type: "zfs-storage".to_string(),
            capabilities: vec![],
            endpoints: vec![],
            metadata: std::collections::HashMap::new(),
        };

        assert_eq!(registration.service_id.len(), 1004);
    }

    #[test]
    fn test_health_status_zero_capacity() {
        let health = ZfsHealthStatus {
            node_id: "node-empty".to_string(),
            status: "unhealthy".to_string(),
            pools_healthy: false,
            datasets_healthy: false,
            system_healthy: false,
            total_capacity: 0,
            available_capacity: 0,
            last_check: 1638360700,
        };

        assert_eq!(health.total_capacity, 0);
        assert_eq!(health.available_capacity, 0);
    }

    #[test]
    fn test_health_status_max_capacity() {
        let health = ZfsHealthStatus {
            node_id: "node-huge".to_string(),
            status: "healthy".to_string(),
            pools_healthy: true,
            datasets_healthy: true,
            system_healthy: true,
            total_capacity: u64::MAX,
            available_capacity: u64::MAX / 2,
            last_check: 1638360800,
        };

        assert_eq!(health.total_capacity, u64::MAX);
        assert!(health.available_capacity < health.total_capacity);
    }

    #[test]
    fn test_service_info_many_capabilities() {
        let mut info = ServiceInfo::new("svc-many-caps", "zfs-storage");
        for i in 0..100 {
            info.capabilities.push(format!("capability-{}", i));
        }

        assert_eq!(info.capabilities.len(), 100);
    }

    #[test]
    fn test_service_info_many_endpoints() {
        let mut info = ServiceInfo::new("svc-many-endpoints", "zfs-storage");
        for i in 8080..8180 {
            info.endpoints.push(format!("http://localhost:{}", i));
        }

        assert_eq!(info.endpoints.len(), 100);
    }

    #[test]
    fn test_service_registration_special_characters_in_type() {
        let registration = ServiceRegistration {
            service_id: "svc-special".to_string(),
            service_type: "zfs-storage-v2.0-beta_test".to_string(),
            capabilities: vec![],
            endpoints: vec![],
            metadata: std::collections::HashMap::new(),
        };

        assert!(registration.service_type.contains('-'));
        assert!(registration.service_type.contains('.'));
        assert!(registration.service_type.contains('_'));
    }

    #[test]
    fn test_health_status_future_timestamp() {
        let future_timestamp = u64::MAX;
        let health = ZfsHealthStatus {
            node_id: "node-future".to_string(),
            status: "healthy".to_string(),
            pools_healthy: true,
            datasets_healthy: true,
            system_healthy: true,
            total_capacity: 10_000_000_000,
            available_capacity: 5_000_000_000,
            last_check: future_timestamp,
        };

        assert_eq!(health.last_check, future_timestamp);
    }

    #[test]
    fn test_service_info_unicode_metadata() {
        let mut info = ServiceInfo::new("svc-unicode", "zfs-storage");
        info.metadata
            .insert("location".to_string(), "北京".to_string());
        info.metadata
            .insert("description".to_string(), "🚀 Fast storage".to_string());

        assert_eq!(info.metadata.get("location"), Some(&"北京".to_string()));
        assert!(
            info.metadata
                .get("description")
                .expect("description exists")
                .contains("🚀")
        );
    }
}

// Helper trait for capacity calculation in tests
trait CapacityCalculations {
    fn available_capacity(&self) -> u64;
    fn used_capacity(&self) -> u64;
    fn usage_percentage(&self) -> f64;
}

impl CapacityCalculations for ZfsHealthStatus {
    fn available_capacity(&self) -> u64 {
        self.available_capacity
    }

    fn used_capacity(&self) -> u64 {
        self.total_capacity.saturating_sub(self.available_capacity)
    }

    fn usage_percentage(&self) -> f64 {
        if self.total_capacity == 0 {
            0.0
        } else {
            (self.used_capacity() as f64 / self.total_capacity as f64) * 100.0
        }
    }
}

#[cfg(test)]
mod capacity_calculation_tests {
    use super::*;

    #[test]
    fn test_capacity_calculations() {
        let health = ZfsHealthStatus {
            node_id: "node-calc".to_string(),
            status: "healthy".to_string(),
            pools_healthy: true,
            datasets_healthy: true,
            system_healthy: true,
            total_capacity: 1_000_000_000,
            available_capacity: 300_000_000,
            last_check: 1638360900,
        };

        assert_eq!(health.used_capacity(), 700_000_000);
        assert!((health.usage_percentage() - 70.0).abs() < 0.01);
    }

    #[test]
    fn test_capacity_calculations_edge_zero() {
        let health = ZfsHealthStatus {
            node_id: "node-zero".to_string(),
            status: "unhealthy".to_string(),
            pools_healthy: false,
            datasets_healthy: false,
            system_healthy: false,
            total_capacity: 0,
            available_capacity: 0,
            last_check: 1638361000,
        };

        assert_eq!(health.used_capacity(), 0);
        assert_eq!(health.usage_percentage(), 0.0);
    }

    #[test]
    fn test_capacity_calculations_full() {
        let health = ZfsHealthStatus {
            node_id: "node-full".to_string(),
            status: "degraded".to_string(),
            pools_healthy: true,
            datasets_healthy: true,
            system_healthy: true,
            total_capacity: 1_000_000_000,
            available_capacity: 0,
            last_check: 1638361100,
        };

        assert_eq!(health.used_capacity(), 1_000_000_000);
        assert!((health.usage_percentage() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_capacity_calculations_half_full() {
        let health = ZfsHealthStatus {
            node_id: "node-half".to_string(),
            status: "healthy".to_string(),
            pools_healthy: true,
            datasets_healthy: true,
            system_healthy: true,
            total_capacity: 2_000_000_000,
            available_capacity: 1_000_000_000,
            last_check: 1638361200,
        };

        assert_eq!(health.used_capacity(), 1_000_000_000);
        assert!((health.usage_percentage() - 50.0).abs() < 0.01);
    }
}
