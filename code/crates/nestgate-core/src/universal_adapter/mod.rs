// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]
// **UNIVERSAL ADAPTER - O(1) CAPABILITY CONNECTIONS**
// This module implements the universal adapter pattern that replaces
// exponential N² primal connections with linear O(1) capability discovery.
//
// **PRIMAL SOVEREIGNTY PRINCIPLE:**
// Each primal only knows itself and discovers others through the universal adapter.
// - NestGate knows: storage, zfs, nas, data_management
// - Orchestration capabilities discovered dynamically
// - Compute capabilities discovered dynamically
// - AI/ML capabilities discovered dynamically
// - Security capabilities discovered dynamically
// - Ecosystem management discovered dynamically
//
// **NO HARDCODED CONNECTIONS:** All inter-primal communication goes through this adapter.

//! Universal Adapter module

pub mod adapter_config;
mod adapter_connection;
mod adapter_routing;
mod adapter_types;
/// Capability endpoints configuration module
pub mod capability_endpoints_config;
mod capability_query;
/// Universal adapter configuration
pub mod config;
/// Capability discovery module
pub mod discovery;
/// Discovery configuration
pub mod discovery_config;

// Capability-based adapters (no hardcoded primal names)
pub mod capability_discovery;
pub mod capability_system;
pub mod networking_capability;
pub mod primal_sovereignty;
pub mod security_capability;

// Re-export ServiceRegistry for easy access
pub use crate::universal_primal_discovery::service_registry::ServiceRegistry;

// Export config types for external use
pub use adapter_config::{AdapterDiscoveryConfig, SharedDiscoveryConfig};
pub use adapter_types::{
    CachedCapability, CapabilityInfo, CapabilityRequest, CapabilityResponse, UniversalAdapter,
    UniversalAdapterConfig, UniversalAdapterConfigCanonical, validate_primal_sovereignty,
};
pub use capability_endpoints_config::{CapabilityEndpointsConfig, SharedCapabilityEndpointsConfig};
pub use discovery_config::{DiscoveryRuntimeConfig, SharedDiscoveryRuntimeConfig};

// **COMPATIBILITY EXPORTS** - For modules expecting legacy structure
pub use adapter_types::CapabilityRequest as CanonicalCapabilityRequest;
pub use adapter_types::UniversalAdapter as PrimalAgnosticAdapter;

// **NEW CAPABILITY ADAPTERS** - Zero hardcoded primal names
pub use capability_discovery::{CapabilityDiscovery, CapabilityProvider, CapabilityType};
pub use networking_capability::{
    CircuitBreakerRequest, CircuitBreakerResponse, LoadBalanceRequest, LoadBalanceResponse,
    NetworkingCapability,
};
pub use security_capability::{
    InputValidationRequest, InputValidationResponse, IntrusionDetectionRequest,
    IntrusionDetectionResponse, RateLimitRequest, RateLimitResponse, SecurityCapability,
};

// **MODULE STRUCTURE** - Organize exports for compatibility
/// Universal adapter types and query structures
pub mod types {
    pub use super::adapter_types::CapabilityInfo;
    pub use super::capability_query::CapabilityQuery;
}

/// Canonical module re-exports for backwards compatibility
pub mod canonical {
    pub use super::adapter_types::CapabilityRequest as CanonicalCapabilityRequest;
}

// Use comprehensive stats module from stats.rs file
/// Adapter statistics and metrics
pub mod stats;

/// Consolidated canonical adapter module
pub mod consolidated_canonical;

// Re-export consolidated_canonical types for backward compatibility
pub use consolidated_canonical::ConsolidatedCanonicalAdapter;

#[cfg(test)]
mod adapter_edge_cases;
#[cfg(test)]
mod adapter_error_tests; // Nov 23, 2025 - P1 test expansion // Nov 23, 2025 - P1-5 edge case tests

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create test endpoint
    /// ✅ MIGRATED: Replaces hardcoded endpoints with configurable ones
    fn test_endpoint(service: &str, port: u16) -> String {
        format!("http://{service}:{port}")
    }

    #[tokio::test]
    async fn test_universal_adapter_discovery() {
        // Use injected config instead of env vars
        let mut config = AdapterDiscoveryConfig::new();
        config.set_discovery_endpoint("orchestration", test_endpoint("test-orch", 8080));
        config.set_discovery_endpoint("compute", test_endpoint("test-compute", 9090));

        let adapter_endpoint = test_endpoint("localhost", 8080) + "/adapter";
        let mut adapter =
            UniversalAdapter::with_discovery_config(std::sync::Arc::new(config), adapter_endpoint);

        // Test capability discovery without hardcoded primal names
        // Test unwrap OK: test environment is controlled
        let capabilities = adapter
            .discover_capabilities()
            .await
            .expect("Test setup: discovery should succeed in test environment");

        // Verify discovery returns non-empty capability entries (capability-based only)
        for capability in capabilities {
            assert!(!capability.provider.is_empty());
            assert!(!capability.category.is_empty());
        }
    }

    #[tokio::test]
    async fn test_o1_capability_access() {
        // Use injected config for clean test
        let config = AdapterDiscoveryConfig::new(); // Empty config, only storage (self-knowledge)
        let adapter_endpoint = test_endpoint("localhost", 8080) + "/adapter";

        let mut adapter =
            UniversalAdapter::with_discovery_config(std::sync::Arc::new(config), adapter_endpoint);
        adapter
            .discover_capabilities()
            .await
            .expect("Test setup: discovery should succeed in test environment");

        // Test O(1) capability access
        let storage_capability = adapter
            .get_capability("storage")
            .expect("Test setup: storage capability should exist");
        assert_eq!(storage_capability.category, "storage");
        assert_eq!(storage_capability.provider, "nestgate-native");
    }

    #[tokio::test]
    async fn test_adapter_with_discovery_config() {
        // Test the new with_discovery_config constructor
        let mut config = AdapterDiscoveryConfig::new();
        config.set_discovery_endpoint("orchestration", test_endpoint("orch-test", 8080));
        config.set_discovery_endpoint("security", test_endpoint("sec-test", 7070));

        let adapter = UniversalAdapter::with_discovery_config(
            std::sync::Arc::new(config),
            "http://test:3000/adapter".to_string(),
        );

        assert_eq!(adapter.endpoint, "http://test:3000/adapter");
        assert_eq!(adapter.capabilities.len(), 0); // No discovery run yet
    }

    #[tokio::test]
    async fn test_concurrent_adapter_instances() {
        // Test that multiple adapters with different configs work concurrently
        let mut config1 = AdapterDiscoveryConfig::new();
        config1.set_discovery_endpoint("orchestration", test_endpoint("orch1", 8080));

        let mut config2 = AdapterDiscoveryConfig::new();
        config2.set_discovery_endpoint("compute", test_endpoint("compute2", 9090));

        let adapter_endpoint = test_endpoint("localhost", 8080) + "/adapter";
        let mut adapter1 = UniversalAdapter::with_discovery_config(
            std::sync::Arc::new(config1),
            adapter_endpoint.clone(),
        );

        let mut adapter2 =
            UniversalAdapter::with_discovery_config(std::sync::Arc::new(config2), adapter_endpoint);

        // Run discoveries concurrently
        let handle1 = tokio::spawn(async move {
            adapter1.discover_capabilities().await.unwrap();
            adapter1.capabilities
        });

        let handle2 = tokio::spawn(async move {
            adapter2.discover_capabilities().await.unwrap();
            adapter2.capabilities
        });

        let caps1 = handle1.await.unwrap();
        let caps2 = handle2.await.unwrap();

        // Verify each adapter got its own capabilities
        assert!(caps1.contains_key("orchestration"));
        assert!(!caps1.contains_key("compute"));

        assert!(caps2.contains_key("compute"));
        assert!(!caps2.contains_key("orchestration"));

        // Both should have storage (self-knowledge)
        assert!(caps1.contains_key("storage"));
        assert!(caps2.contains_key("storage"));
    }
}
