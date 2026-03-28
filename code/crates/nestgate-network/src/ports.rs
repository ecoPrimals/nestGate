// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

/// Port allocation manager
///
/// # Primal Sovereignty
///
/// Port range is configurable via `NESTGATE_PORT_START` environment variable.
/// No hardcoded port assumptions.
pub struct PortManager {
    allocated_ports: HashMap<u16, String>,
    next_port: u16,
}

impl PortManager {
    /// Create a new port manager with environment-driven configuration
    ///
    /// # Environment Variables
    ///
    /// - `NESTGATE_PORT_START`: Starting port for allocation (default: 8080)
    ///
    /// # Examples
    ///
    /// ```
    /// use nestgate_network::ports::PortManager;
    ///
    /// let manager = PortManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        let start_port = env::var("NESTGATE_PORT_START")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080); // Safe default if env not set

        Self {
            allocated_ports: HashMap::new(),
            next_port: start_port,
        }
    }

    /// Allocate a port for a service
    pub fn allocate_port(&mut self, service_name: &str) -> u16 {
        let port = self.next_port;
        self.allocated_ports.insert(port, service_name.to_string());
        self.next_port += 1;
        port
    }

    /// Release a port
    pub fn release_port(&mut self, port: u16) -> bool {
        self.allocated_ports.remove(&port).is_some()
    }

    /// Check if a port is allocated
    pub fn is_allocated(&self, port: u16) -> bool {
        self.allocated_ports.contains_key(&port)
    }

    /// Get service name for a port
    pub fn get_service(&self, port: u16) -> Option<&String> {
        self.allocated_ports.get(&port)
    }

    /// Get all allocated ports
    pub fn get_allocated_ports(&self) -> &HashMap<u16, String> {
        &self.allocated_ports
    }
}

impl Default for PortManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// Port allocation information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Portallocation
pub struct PortAllocation {
    /// Port
    pub port: u16,
    /// Service name
    pub service_name: String,
    /// Port Type
    pub port_type: String,
    /// Allocated At
    pub allocated_at: std::time::SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test constant - environment-driven, defaults to 8080
    const TEST_DEFAULT_PORT: u16 = 8080;

    // ==================== PORT MANAGER TESTS ====================

    #[test]
    fn test_port_manager_new() {
        let manager = PortManager::new();
        // Verify it uses environment or defaults to 8080
        assert!(manager.next_port >= TEST_DEFAULT_PORT);
        assert!(manager.allocated_ports.is_empty());
    }

    #[test]
    fn test_port_manager_default() {
        let manager = PortManager::default();
        // Verify it uses environment or defaults to 8080
        assert!(manager.next_port >= TEST_DEFAULT_PORT);
        assert!(manager.allocated_ports.is_empty());
    }

    #[test]
    fn test_allocate_port() {
        let mut manager = PortManager::new();
        let initial_port = manager.next_port;
        let port = manager.allocate_port("test-service");

        assert_eq!(port, initial_port); // Allocated from start
        assert!(manager.is_allocated(port));
        assert_eq!(manager.get_service(port), Some(&"test-service".to_string()));
    }

    #[test]
    fn test_allocate_multiple_ports() {
        let mut manager = PortManager::new();
        let start = manager.next_port;

        let port1 = manager.allocate_port("service-1");
        let port2 = manager.allocate_port("service-2");
        let port3 = manager.allocate_port("service-3");

        // Verify sequential allocation from start port
        assert_eq!(port1, start);
        assert_eq!(port2, start + 1);
        assert_eq!(port3, start + 2);

        assert_eq!(manager.allocated_ports.len(), 3);
    }

    #[test]
    fn test_release_port() {
        let mut manager = PortManager::new();
        let port = manager.allocate_port("test-service");

        assert!(manager.is_allocated(port));
        assert!(manager.release_port(port));
        assert!(!manager.is_allocated(port));
    }

    #[test]
    fn test_release_nonexistent_port() {
        let mut manager = PortManager::new();

        assert!(!manager.release_port(9999));
    }

    #[test]
    fn test_release_port_twice() {
        let mut manager = PortManager::new();
        let port = manager.allocate_port("test-service");

        assert!(manager.release_port(port));
        assert!(!manager.release_port(port)); // Second release should return false
    }

    #[test]
    fn test_is_allocated() {
        let mut manager = PortManager::new();
        let port = manager.allocate_port("test-service");

        assert!(manager.is_allocated(port));
        assert!(!manager.is_allocated(port + 1));
    }

    #[test]
    fn test_get_service() {
        let mut manager = PortManager::new();
        let port = manager.allocate_port("my-service");

        assert_eq!(manager.get_service(port), Some(&"my-service".to_string()));
        assert_eq!(manager.get_service(port + 1), None);
    }

    #[test]
    fn test_get_allocated_ports() {
        let mut manager = PortManager::new();

        manager.allocate_port("service-1");
        manager.allocate_port("service-2");

        let allocated = manager.get_allocated_ports();
        assert_eq!(allocated.len(), 2);
    }

    #[test]
    fn test_allocate_and_release_multiple() {
        let mut manager = PortManager::new();

        let port1 = manager.allocate_port("service-1");
        let port2 = manager.allocate_port("service-2");
        let port3 = manager.allocate_port("service-3");

        assert_eq!(manager.allocated_ports.len(), 3);

        manager.release_port(port2);
        assert_eq!(manager.allocated_ports.len(), 2);
        assert!(manager.is_allocated(port1));
        assert!(!manager.is_allocated(port2));
        assert!(manager.is_allocated(port3));
    }

    #[test]
    fn test_port_counter_increments() {
        let mut manager = PortManager::new();

        let port1 = manager.allocate_port("s1");
        let port2 = manager.allocate_port("s2");
        let port3 = manager.allocate_port("s3");

        assert_eq!(port2, port1 + 1);
        assert_eq!(port3, port2 + 1);
    }

    // ==================== PORT ALLOCATION TESTS ====================

    #[test]
    fn test_port_allocation_creation() {
        let test_port = 8080;
        let allocation = PortAllocation {
            port: test_port,
            service_name: "test-service".to_string(),
            port_type: "HTTP".to_string(),
            allocated_at: std::time::SystemTime::now(),
        };

        assert_eq!(allocation.port, test_port);
        assert_eq!(allocation.service_name, "test-service");
        assert_eq!(allocation.port_type, "HTTP");
    }

    #[test]
    fn test_port_allocation_clone() {
        let test_port = 3000;
        let allocation = PortAllocation {
            port: test_port,
            service_name: "api".to_string(),
            port_type: "API".to_string(),
            allocated_at: std::time::SystemTime::now(),
        };

        let cloned = allocation.clone();
        assert_eq!(cloned.port, allocation.port);
        assert_eq!(cloned.service_name, allocation.service_name);
    }

    #[test]
    fn test_port_allocation_serialization() {
        let allocation = PortAllocation {
            port: 5000,
            service_name: "db".to_string(),
            port_type: "DATABASE".to_string(),
            allocated_at: std::time::SystemTime::now(),
        };

        let json = serde_json::to_string(&allocation).expect("Should serialize");
        assert!(json.contains("5000"));
        assert!(json.contains("db"));
        assert!(json.contains("DATABASE"));
    }

    #[test]
    fn test_port_allocation_deserialization() {
        let json = r#"{"port":4000,"service_name":"cache","port_type":"REDIS","allocated_at":{"secs_since_epoch":1234567890,"nanos_since_epoch":0}}"#;
        let allocation: PortAllocation = serde_json::from_str(json).expect("Should deserialize");

        assert_eq!(allocation.port, 4000);
        assert_eq!(allocation.service_name, "cache");
        assert_eq!(allocation.port_type, "REDIS");
    }

    #[test]
    fn test_multiple_services_different_names() {
        let mut manager = PortManager::new();

        manager.allocate_port("web-server");
        manager.allocate_port("api-server");
        manager.allocate_port("database");

        let allocated = manager.get_allocated_ports();
        assert_eq!(allocated.len(), 3);
        assert!(allocated.values().any(|s| s == "web-server"));
        assert!(allocated.values().any(|s| s == "api-server"));
        assert!(allocated.values().any(|s| s == "database"));
    }

    #[test]
    fn test_port_manager_state_persistence() {
        let mut manager = PortManager::new();

        let port1 = manager.allocate_port("service-1");
        let port2 = manager.allocate_port("service-2");

        // Release first port
        manager.release_port(port1);

        // Allocate new service - should get next port (not reuse released port)
        let port3 = manager.allocate_port("service-3");
        assert_eq!(port3, port2 + 1);
        assert!(!manager.is_allocated(port1));
    }
}
