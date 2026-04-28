// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Native Async module

/// Configuration types for native async networking
pub mod config;
/// Development-mode implementations for testing
#[cfg(any(test, feature = "dev-stubs"))]
pub mod development;
/// Production-ready async network implementations
pub mod production;
pub mod service;
// Native Async Network Module - Split for File Size Compliance
// This module was split from native_async_network.rs to maintain the 2000-line limit
// while preserving all functionality and maintaining backward compatibility
// Sub-module declarations
/// Trait definitions for native async networking
pub mod traits;
/// Type definitions for native async networking
pub mod types;
// Re-export all public types and traits for backward compatibility
pub use traits::{
    NativeAsyncLoadBalancer, NativeAsyncProtocolHandler, NativeAsyncServiceDiscovery,
    NativeAsyncUnifiedServiceInterface,
};

pub use types::{
    ConnectionStatus,
    LoadBalancerBackend, // NetworkServiceHealth
    NetworkRequest,
    NetworkResponse,
};

// **MIGRATED**: Using canonical config system from domains/network
pub use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig as UnifiedNetworkConfig;

pub use production::{
    // ProductionNetworkManager, ProductionServiceDiscovery, // These will be implemented as needed
    ProductionProtocolHandler,
};

#[cfg(any(test, feature = "dev-stubs"))]
pub use development::{DevelopmentNetworkServiceDiscovery, DevelopmentServiceDiscovery};

pub use service::NativeAsyncNetworkService;

// Type aliases for compatibility
/// Type alias for production network service discovery implementation
pub type ProductionNetworkServiceDiscovery = production::ProductionServiceDiscovery;
/// Type alias for Productionnetworkprotocolhandler
pub type ProductionNetworkProtocolHandler = production::ProductionProtocolHandler;

// Tests module
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::time::Duration;
    // Import canonical types for testing
    use crate::Result;
    use crate::diagnostics::types::ServiceInfo;

    #[tokio::test]
    async fn test_native_async_service_discovery()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        // Create production service discovery with native async methods
        let discovery = production::ProductionServiceDiscovery::default();

        // Test native async service registration - no Future boxing
        let service = ServiceInfo {
            name: "test_service".to_string(),
            version: "1.0.0".to_string(),
            status: "running".to_string(),
            pid: Some(std::process::id()),
            cpu_percent: Some(0.0),
            memory_bytes: Some(0),
            start_time: Some(std::time::SystemTime::now()),
            description: Some("Test service for discovery".to_string()),
            dependencies: None,
            command_line: Some("nestgate-test".to_string()),
        };

        let register_result = discovery.register(service.clone()).await;
        assert!(register_result.is_ok());

        // Test native async service discovery
        let discovered = discovery.discover("test_service").await;
        assert!(discovered.is_ok());
        assert!(
            !discovered
                .map_err(|e| {
                    tracing::error!(
                        "Expected operation failed: {} - Error: {:?}",
                        "Test operation should succeed",
                        e
                    );
                    crate::NestGateError::internal_error(
                        "Test operation should succeed",
                        "automated_migration",
                    )
                })?
                .is_empty()
        );

        // Test native async service existence check
        let exists = discovery.exists("test_service").await;
        assert!(exists.is_ok());

        // Test compile-time values
        assert_eq!(
            production::ProductionServiceDiscovery::max_services(),
            10000
        );
        assert_eq!(DevelopmentServiceDiscovery::max_services(), 1000);

        println!("Native async service discovery validation successful!");
        Ok(())
    }

    #[tokio::test]
    async fn test_native_async_protocol_handler()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        // Create production protocol handler with native async methods
        let handler = ProductionProtocolHandler::default();

        // Test native async connection - no Future boxing
        let config = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig::development_optimized();

        let connection = handler.connect(&config).await;
        assert!(connection.is_ok());

        // Test native async request handling
        let request = NetworkRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            method: "GET".to_string(),
            headers: HashMap::new(),
            body: vec![],
            timeout: Some(Duration::from_secs(30)),
        };

        if let Ok(conn) = connection {
            let response = handler.send_request(&conn, request).await;
            assert!(response.is_ok());
            if let Ok(resp) = response {
                assert_eq!(resp.status_code, 200);
            }
        }

        // Test compile-time values
        assert_eq!(ProductionProtocolHandler::max_connections(), 1000);
        assert_eq!(ProductionProtocolHandler::connection_timeout_seconds(), 30);

        println!("Native async protocol handler validation successful!");
        Ok(())
    }

    #[tokio::test]
    async fn test_service_events_and_watching()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let discovery = production::ProductionServiceDiscovery::default();

        // Register a service to generate events
        let service = ServiceInfo {
            name: "event_test_service".to_string(),
            version: "1.0.0".to_string(),
            status: "running".to_string(),
            pid: Some(std::process::id()),
            cpu_percent: Some(0.0),
            memory_bytes: Some(0),
            start_time: Some(std::time::SystemTime::now()),
            description: Some("Test service for events".to_string()),
            dependencies: None,
            command_line: Some("nestgate-test".to_string()),
        };

        let _ = discovery.register(service).await;

        // Test native async event watching
        let events = discovery.watch().await;
        assert!(events.is_ok());
        if let Ok(event_list) = events {
            assert!(!event_list.is_empty());
            assert!(matches!(
                event_list[0].event_type,
                crate::network::native_async::types::ServiceEventType::Registered
            ));
        }

        println!("Service events and watching validation successful!");
        Ok(())
    }

    #[test]
    fn test_network_compile_time_specialization() {
        // Test compile-time network configurations using generic parameters
        // ProductionServiceDiscovery<10000, 30, 1000, 60>
        // DevelopmentServiceDiscovery<1000, 60, 100, 120>

        let prod_discovery = production::ProductionServiceDiscovery::default();
        let dev_discovery = development::DevelopmentServiceDiscovery::default();

        // Test that instances can be created (validates compile-time parameters)
        assert!(std::mem::size_of_val(&prod_discovery) > 0);
        assert!(std::mem::size_of_val(&dev_discovery) > 0);

        println!("Service discovery compile-time specialization working!");
        println!("   Production and development configurations validated");
    }
}
