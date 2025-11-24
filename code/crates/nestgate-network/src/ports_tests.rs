//! **COMPREHENSIVE TESTS FOR PORT MANAGER**
//!
//! Additional tests for port allocation and management system.

#[cfg(test)]
mod port_manager_extended_tests {
    use crate::ports::*;

    // ==================== PORT RANGE TESTS ====================

    #[test]
    fn test_allocate_within_valid_range() {
        let mut manager = PortManager::new();
        let port = manager.allocate_port("test-service");
        assert!(port >= 1024, "Port should be >= 1024 (non-privileged)");
        // u16 is always <= 65535, so no need to check upper bound
    }

    #[test]
    fn test_allocate_sequential_ports() {
        let mut manager = PortManager::new();

        let port1 = manager.allocate_port("service-1");
        let port2 = manager.allocate_port("service-2");
        let port3 = manager.allocate_port("service-3");
        let port4 = manager.allocate_port("service-4");
        let port5 = manager.allocate_port("service-5");

        // Ports should be sequential or at least distinct
        assert_ne!(port1, port2);
        assert_ne!(port2, port3);
        assert_ne!(port3, port4);
        assert_ne!(port4, port5);
    }

    // ==================== SERVICE NAME TESTS ====================

    #[test]
    fn test_service_names_stored_correctly() {
        let mut manager = PortManager::new();

        let port1 = manager.allocate_port("database");
        let port2 = manager.allocate_port("web-server");
        let port3 = manager.allocate_port("api-gateway");

        assert_eq!(manager.get_service(port1), Some(&"database".to_string()));
        assert_eq!(manager.get_service(port2), Some(&"web-server".to_string()));
        assert_eq!(manager.get_service(port3), Some(&"api-gateway".to_string()));
    }

    #[test]
    fn test_service_with_special_characters() {
        let mut manager = PortManager::new();

        let port = manager.allocate_port("my-service_123");
        assert_eq!(
            manager.get_service(port),
            Some(&"my-service_123".to_string())
        );
    }

    #[test]
    fn test_service_with_empty_string() {
        let mut manager = PortManager::new();

        let port = manager.allocate_port("");
        assert_eq!(manager.get_service(port), Some(&"".to_string()));
    }

    // ==================== PORT RELEASE TESTS ====================

    #[test]
    fn test_release_and_reallocate() {
        let mut manager = PortManager::new();

        let port1 = manager.allocate_port("temp-service");
        assert!(manager.is_allocated(port1));

        assert!(manager.release_port(port1));
        assert!(!manager.is_allocated(port1));

        // Allocate a new port after release
        let port2 = manager.allocate_port("new-service");
        assert!(manager.is_allocated(port2));
    }

    #[test]
    fn test_release_multiple_ports() {
        let mut manager = PortManager::new();

        let ports: Vec<u16> = (0..5)
            .map(|i| manager.allocate_port(&format!("service-{}", i)))
            .collect();

        // All ports should be allocated
        for port in &ports {
            assert!(manager.is_allocated(*port));
        }

        // Release all ports
        for port in &ports {
            assert!(manager.release_port(*port));
        }

        // All ports should be released
        for port in &ports {
            assert!(!manager.is_allocated(*port));
        }
    }

    // ==================== ALLOCATION STATE TESTS ====================

    #[test]
    fn test_is_allocated_unallocated_port() {
        let manager = PortManager::new();

        // Test a high port that definitely isn't allocated
        assert!(!manager.is_allocated(50000));
        assert!(!manager.is_allocated(60000));
    }

    #[test]
    fn test_get_service_for_unallocated_port() {
        let manager = PortManager::new();

        assert_eq!(manager.get_service(12345), None);
        assert_eq!(manager.get_service(54321), None);
    }

    // ==================== BOUNDARY CONDITION TESTS ====================

    #[test]
    fn test_allocate_many_ports() {
        let mut manager = PortManager::new();

        let mut allocated_ports = Vec::new();
        for i in 0..100 {
            let port = manager.allocate_port(&format!("service-{}", i));
            allocated_ports.push(port);
        }

        // Verify all ports are unique
        let unique_count = allocated_ports
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len();
        assert_eq!(unique_count, 100, "All ports should be unique");
    }

    #[test]
    fn test_allocation_tracking() {
        let mut manager = PortManager::new();

        let port = manager.allocate_port("tracked-service");

        // Port should be tracked
        assert!(manager.is_allocated(port));
        assert_eq!(
            manager.get_service(port),
            Some(&"tracked-service".to_string())
        );

        // Release and verify tracking is cleared
        assert!(manager.release_port(port));
        assert!(!manager.is_allocated(port));
        assert_eq!(manager.get_service(port), None);
    }

    // ==================== SERVICE LOOKUP TESTS ====================

    #[test]
    fn test_find_service_by_name() {
        let mut manager = PortManager::new();

        let db_port = manager.allocate_port("database");
        let web_port = manager.allocate_port("web");
        let api_port = manager.allocate_port("api");

        // Verify we can look up services by their allocated port
        assert_eq!(manager.get_service(db_port), Some(&"database".to_string()));
        assert_eq!(manager.get_service(web_port), Some(&"web".to_string()));
        assert_eq!(manager.get_service(api_port), Some(&"api".to_string()));
    }

    // ==================== MANAGER CREATION TESTS ====================

    #[test]
    fn test_manager_new_creates_empty_manager() {
        let manager = PortManager::new();

        // New manager should have no allocations
        // Test some random ports
        assert!(!manager.is_allocated(8080));
        assert!(!manager.is_allocated(3000));
        assert!(!manager.is_allocated(5432));
    }

    #[test]
    fn test_multiple_managers_independent() {
        let mut manager1 = PortManager::new();
        let mut manager2 = PortManager::new();

        let port1 = manager1.allocate_port("service-1");
        let port2 = manager2.allocate_port("service-2");

        // Each manager tracks independently
        assert!(manager1.is_allocated(port1));
        assert!(manager2.is_allocated(port2));

        // Manager1 doesn't know about manager2's ports
        if port1 != port2 {
            assert!(!manager1.is_allocated(port2));
            assert!(!manager2.is_allocated(port1));
        }
    }

    // ==================== EDGE CASE TESTS ====================

    #[test]
    fn test_release_same_port_repeatedly() {
        let mut manager = PortManager::new();

        let port = manager.allocate_port("test");

        assert!(manager.release_port(port)); // First release should succeed
        assert!(!manager.release_port(port)); // Second release should fail
        assert!(!manager.release_port(port)); // Third release should fail
    }

    #[test]
    fn test_service_name_unicode() {
        let mut manager = PortManager::new();

        let port = manager.allocate_port("服务-サービス");
        assert_eq!(
            manager.get_service(port),
            Some(&"服务-サービス".to_string())
        );
    }

    #[test]
    fn test_service_name_very_long() {
        let mut manager = PortManager::new();

        let long_name = "a".repeat(1000);
        let port = manager.allocate_port(&long_name);
        assert_eq!(manager.get_service(port), Some(&long_name));
    }
}
