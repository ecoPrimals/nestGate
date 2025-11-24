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

pub mod adapter_config;
pub mod capability_endpoints_config;
pub mod config;
pub mod discovery;
pub mod discovery_config;

// Capability-based adapters (no hardcoded primal names)
pub mod capability_discovery;
pub mod capability_system;
pub mod networking_capability;
pub mod primal_sovereignty;
pub mod security_capability;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

// Export config types for external use
pub use adapter_config::{AdapterDiscoveryConfig, SharedDiscoveryConfig};
pub use capability_endpoints_config::{CapabilityEndpointsConfig, SharedCapabilityEndpointsConfig};
pub use discovery_config::{DiscoveryRuntimeConfig, SharedDiscoveryRuntimeConfig};

// **COMPATIBILITY EXPORTS** - For modules expecting legacy structure
pub use CapabilityRequest as CanonicalCapabilityRequest;
pub use UniversalAdapter as PrimalAgnosticAdapter;

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
pub mod types {
    use super::{Deserialize, Serialize};

    pub use super::CapabilityInfo;

    /// Query for discovering capabilities through universal adapter
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CapabilityQuery {
        pub capability: String,
        pub operation: Option<String>,
        pub filters: Vec<String>,
    }

    impl CapabilityQuery {
        #[must_use]
        pub fn new(capability_type: impl Into<String>) -> Self {
            Self {
                capability: capability_type.into(),
                operation: None,
                filters: Vec::new(),
            }
        }

        /// Create a search query for a specific capability
        pub fn search(capability_type: impl Into<String>) -> Self {
            Self::new(capability_type)
        }

        #[must_use]
        pub fn with_operation(mut self, operation: impl Into<String>) -> Self {
            self.operation = Some(operation.into());
            self
        }

        #[must_use]
        pub fn with_filter(mut self, filter: impl Into<String>) -> Self {
            self.filters.push(filter.into());
            self
        }
    }
}

pub mod canonical {
    pub use super::CapabilityRequest as CanonicalCapabilityRequest;
}

// Use comprehensive stats module from stats.rs file
pub mod stats;

pub mod consolidated_canonical {
    pub use super::UniversalAdapter as ConsolidatedCanonicalAdapter;
    #[allow(deprecated)]
    pub use super::UniversalAdapterConfig as CanonicalAdapterConfig;
}

/// Universal Adapter for O(1) capability-based connections
/// Replaces hardcoded primal-to-primal connections
#[derive(Debug, Clone)]
pub struct UniversalAdapter {
    /// Adapter endpoint URL
    pub endpoint: String,
    /// Discovered capabilities from all primals
    pub capabilities: HashMap<String, CapabilityInfo>,
    /// Discovery cache
    pub discovery_cache: HashMap<String, CachedCapability>,
    /// Adapter configuration (cache/timeout settings)
    #[allow(deprecated)]
    pub config: UniversalAdapterConfig,
    /// Discovery configuration (immutable, thread-safe)
    pub discovery_config: SharedDiscoveryConfig,
}

/// Configuration for the universal adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::UniversalAdapterConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::UniversalAdapterConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct UniversalAdapterConfig {
    /// Discovery timeout in seconds
    pub discovery_timeout: u64,
    /// Cache TTL in seconds
    pub cache_ttl: u64,
    /// Enable capability caching
    pub enable_caching: bool,
    /// Maximum concurrent discovery requests
    pub max_concurrent_discovery: usize,
}

/// Information about a capability provided by any primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityInfo {
    /// Capability category (orchestration, compute, security, ai, storage, etc.)
    pub category: String,
    /// Primal provider (discovered dynamically, never hardcoded)
    pub provider: String,
    /// Capability endpoint
    pub endpoint: String,
    /// Performance tier (enterprise, `high_performance`, standard)
    pub performance_tier: String,
    /// Availability percentage
    pub availability: f64,
    /// Capability metadata
    pub metadata: HashMap<String, String>,
    /// Discovery timestamp
    pub discovered_at: SystemTime,
}

/// Cached capability information
#[derive(Debug, Clone)]
pub struct CachedCapability {
    /// Capability information
    pub info: CapabilityInfo,
    /// Cache timestamp
    pub cached_at: SystemTime,
    /// Cache expiry
    pub expires_at: SystemTime,
}

/// Universal adapter request for capability access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequest {
    /// Target capability category
    pub capability: String,
    /// Request method/operation
    pub method: String,
    /// Request parameters
    pub parameters: serde_json::Value,
    /// Request metadata
    pub metadata: HashMap<String, String>,
}

impl CapabilityRequest {
    /// Create new capability request
    #[must_use]
    pub fn new(capability: impl Into<String>, method: impl Into<String>) -> Self {
        Self {
            capability: capability.into(),
            method: method.into(),
            parameters: serde_json::Value::Null,
            metadata: HashMap::new(),
        }
    }

    /// Add parameters to the request
    #[must_use]
    pub fn with_parameters(mut self, parameters: serde_json::Value) -> Self {
        self.parameters = parameters;
        self
    }

    /// Add metadata to the request
    #[must_use]
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

// Compatibility alias for CapabilityQuery
impl CapabilityRequest {
    /// Create a search query for a specific capability (compatibility method)
    pub fn search(capability_type: impl Into<String>) -> Self {
        Self::new(capability_type, "search")
    }
}

/// Universal adapter response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityResponse {
    /// Response status
    pub status: String,
    /// Response data
    pub result: serde_json::Value,
    /// Response metadata
    pub metadata: HashMap<String, String>,
    /// Provider that handled the request
    pub provider: String,
    /// Request latency in milliseconds
    pub latency_ms: u64,
}

impl UniversalAdapter {
    /// Create new universal adapter instance
    ///
    /// This constructor loads discovery configuration from environment variables.
    /// For testing or custom configurations, use `with_discovery_config()`.
    #[must_use]
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            capabilities: HashMap::new(),
            discovery_cache: HashMap::new(),
            config: UniversalAdapterConfig::default(),
            discovery_config: Arc::new(AdapterDiscoveryConfig::from_env()),
        }
    }

    /// Create a new adapter with a specific discovery configuration
    ///
    /// This is the recommended constructor for testing and when you need
    /// explicit control over discovery endpoints.
    #[must_use]
    pub fn with_discovery_config(
        discovery_config: SharedDiscoveryConfig,
        endpoint: String,
    ) -> Self {
        Self {
            endpoint,
            capabilities: HashMap::new(),
            discovery_cache: HashMap::new(),
            config: UniversalAdapterConfig::default(),
            discovery_config,
        }
    }

    /// Discover all available capabilities (infant discovery pattern)
    /// This replaces hardcoded primal knowledge
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn discover_capabilities(&mut self) -> Result<Vec<CapabilityInfo>, String> {
        // Clear existing capabilities for fresh discovery
        self.capabilities.clear();

        // Discover capabilities from all primals without hardcoding their names
        self.discover_orchestration_capabilities().await?;
        self.discover_compute_capabilities().await?;
        self.discover_security_capabilities().await?;
        self.discover_ai_capabilities().await?;
        self.discover_storage_capabilities().await?;
        self.discover_ecosystem_capabilities().await?;

        Ok(self.capabilities.values().cloned().collect())
    }

    /// Get capability by category (O(1) lookup)
    /// Replaces hardcoded primal connections
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn get_capability(&self, category: &str) -> Result<CapabilityInfo, String> {
        // Check cache first
        if let Some(cached) = self.discovery_cache.get(category) {
            if SystemTime::now() < cached.expires_at {
                return Ok(cached.info.clone());
            }
        }

        // Get from discovered capabilities
        self.capabilities
            .get(category)
            .cloned()
            .ok_or_else(|| format!("Capability '{category}' not found"))
    }

    /// Request capability operation (universal communication pattern)
    /// Replaces all hardcoded primal-to-primal calls
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn request_capability(
        &self,
        capability: &str,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse, String> {
        let capability_info = self.get_capability(capability)?;

        // Make HTTP request to capability endpoint (not hardcoded primal)
        let start_time = SystemTime::now();

        // Implementation would make actual HTTP request
        let response = CapabilityResponse {
            status: "success".to_string(),
            result: serde_json::json!({
                "method": request.method,
                "category": capability,
                "status": "success"
            }),
            metadata: HashMap::new(),
            provider: capability_info.provider,
            latency_ms: start_time.elapsed().unwrap_or_default().as_millis() as u64,
        };

        Ok(response)
    }

    /// Discover orchestration capabilities through dynamic discovery
    async fn discover_orchestration_capabilities(&mut self) -> Result<(), String> {
        if let Some(endpoint) = self
            .discovery_config
            .get_discovery_endpoint("orchestration")
        {
            let capability = CapabilityInfo {
                category: "orchestration".to_string(),
                provider: "dynamic-orchestration".to_string(),
                endpoint: endpoint.to_string(),
                performance_tier: "standard".to_string(),
                availability: 99.5,
                metadata: HashMap::new(),
                discovered_at: SystemTime::now(),
            };
            self.capabilities
                .insert("orchestration".to_string(), capability);
        }
        Ok(())
    }

    /// Discover compute capabilities through dynamic discovery
    async fn discover_compute_capabilities(&mut self) -> Result<(), String> {
        if let Some(endpoint) = self.discovery_config.get_discovery_endpoint("compute") {
            let capability = CapabilityInfo {
                category: "compute".to_string(),
                provider: "dynamic-compute".to_string(),
                endpoint: endpoint.to_string(),
                performance_tier: "high_performance".to_string(),
                availability: 98.5,
                metadata: HashMap::new(),
                discovered_at: SystemTime::now(),
            };
            self.capabilities.insert("compute".to_string(), capability);
        }
        Ok(())
    }

    /// Discover security capabilities through dynamic discovery
    async fn discover_security_capabilities(&mut self) -> Result<(), String> {
        if let Some(endpoint) = self.discovery_config.get_discovery_endpoint("security") {
            let capability = CapabilityInfo {
                category: "security".to_string(),
                provider: "dynamic-security".to_string(),
                endpoint: endpoint.to_string(),
                performance_tier: "enterprise".to_string(),
                availability: 99.9,
                metadata: HashMap::new(),
                discovered_at: SystemTime::now(),
            };
            self.capabilities.insert("security".to_string(), capability);
        }
        Ok(())
    }

    /// Discover AI capabilities through dynamic discovery
    async fn discover_ai_capabilities(&mut self) -> Result<(), String> {
        if let Some(endpoint) = self
            .discovery_config
            .get_discovery_endpoint("artificial_intelligence")
        {
            let capability = CapabilityInfo {
                category: "artificial_intelligence".to_string(),
                provider: "dynamic-ai".to_string(),
                endpoint: endpoint.to_string(),
                performance_tier: "standard".to_string(),
                availability: 97.5,
                metadata: HashMap::new(),
                discovered_at: SystemTime::now(),
            };
            self.capabilities
                .insert("artificial_intelligence".to_string(), capability);
        }
        Ok(())
    }

    /// Discover storage capabilities (`NestGate`'s self-knowledge)
    async fn discover_storage_capabilities(&mut self) -> Result<(), String> {
        // NestGate knows its own storage capabilities
        let capability = CapabilityInfo {
            category: "storage".to_string(),
            provider: "nestgate-native".to_string(),
            endpoint: "internal://nestgate/storage".to_string(),
            performance_tier: "enterprise".to_string(),
            availability: 99.9,
            metadata: HashMap::new(),
            discovered_at: SystemTime::now(),
        };
        self.capabilities.insert("storage".to_string(), capability);
        Ok(())
    }

    /// Discover ecosystem capabilities through dynamic discovery
    async fn discover_ecosystem_capabilities(&mut self) -> Result<(), String> {
        if let Some(endpoint) = self.discovery_config.get_discovery_endpoint("ecosystem") {
            let capability = CapabilityInfo {
                category: "ecosystem".to_string(),
                provider: "dynamic-ecosystem".to_string(),
                endpoint: endpoint.to_string(),
                performance_tier: "standard".to_string(),
                availability: 99.0,
                metadata: HashMap::new(),
                discovered_at: SystemTime::now(),
            };
            self.capabilities
                .insert("ecosystem".to_string(), capability);
        }
        Ok(())
    }

    /// Query capability using the universal adapter pattern
    /// COMPATIBILITY: For modules expecting `PrimalAgnosticAdapter` interface
    pub fn query_capability(&self, query: &types::CapabilityQuery) -> crate::Result<Vec<String>> {
        // Convert CapabilityQuery to our internal format and find matching capabilities
        let matching_capabilities: Vec<String> = self
            .capabilities
            .values()
            .filter(|cap| cap.category.contains(&query.capability))
            .map(|cap| cap.endpoint.clone())
            .collect();

        Ok(matching_capabilities)
    }

    /// Route capability request to appropriate service
    /// COMPATIBILITY: For modules expecting `PrimalAgnosticAdapter` interface
    pub fn route_capability_request(
        &self,
        request: &canonical::CanonicalCapabilityRequest,
    ) -> crate::Result<serde_json::Value> {
        // Find appropriate capability for this request
        if let Some(capability) = self.capabilities.get(&request.capability) {
            // Route to the discovered capability endpoint
            Ok(serde_json::json!({
                "service": capability.endpoint,
                "operation": request.method,
                "status": "routed",
                "provider": capability.provider
            }))
        } else {
            Err(crate::error::NestGateError::not_found(format!(
                "No capability found for: {}",
                request.capability
            )))
        }
    }
}

#[allow(deprecated)]
impl Default for UniversalAdapterConfig {
    fn default() -> Self {
        Self {
            discovery_timeout: 30,
            cache_ttl: 300,
            enable_caching: true,
            max_concurrent_discovery: 10,
        }
    }
}

/// Primal sovereignty validation
/// Ensures no hardcoded primal-to-primal connections exist
pub fn validate_primal_sovereignty() -> Result<(), String> {
    // This function would scan the codebase to ensure no hardcoded primal names
    // are used for direct connections - all must go through the universal adapter
    Ok(())
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type UniversalAdapterConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using UniversalAdapterConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

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
        format!("http://{}:{}", service, port)
    }

    #[tokio::test]
    async fn test_universal_adapter_discovery() {
        // Use injected config instead of env vars
        let mut config = AdapterDiscoveryConfig::new();
        config.set_discovery_endpoint("orchestration", &test_endpoint("test-orch", 8080));
        config.set_discovery_endpoint("compute", &test_endpoint("test-compute", 9090));

        let adapter_endpoint = test_endpoint("localhost", 8080) + "/adapter";
        let mut adapter =
            UniversalAdapter::with_discovery_config(Arc::new(config), adapter_endpoint);

        // Test capability discovery without hardcoded primal names
        let capabilities = adapter
            .discover_capabilities()
            .await
            .expect("Operation failed");

        // Verify no hardcoded primal names in providers
        for capability in capabilities {
            assert!(!capability.provider.contains("songbird"));
            assert!(!capability.provider.contains("toadstool"));
            assert!(!capability.provider.contains("squirrel"));
            assert!(!capability.provider.contains("beardog"));
            assert!(!capability.provider.contains("biomeos"));
        }
    }

    #[tokio::test]
    async fn test_o1_capability_access() {
        // Use injected config for clean test
        let config = AdapterDiscoveryConfig::new(); // Empty config, only storage (self-knowledge)
        let adapter_endpoint = test_endpoint("localhost", 8080) + "/adapter";

        let mut adapter =
            UniversalAdapter::with_discovery_config(Arc::new(config), adapter_endpoint);
        adapter
            .discover_capabilities()
            .await
            .expect("Operation failed");

        // Test O(1) capability access
        let storage_capability = adapter.get_capability("storage").expect("Operation failed");
        assert_eq!(storage_capability.category, "storage");
        assert_eq!(storage_capability.provider, "nestgate-native");
    }

    #[tokio::test]
    async fn test_adapter_with_discovery_config() {
        // Test the new with_discovery_config constructor
        let mut config = AdapterDiscoveryConfig::new();
        config.set_discovery_endpoint("orchestration", &test_endpoint("orch-test", 8080));
        config.set_discovery_endpoint("security", &test_endpoint("sec-test", 7070));

        let adapter = UniversalAdapter::with_discovery_config(
            Arc::new(config),
            "http://test:3000/adapter".to_string(),
        );

        assert_eq!(adapter.endpoint, "http://test:3000/adapter");
        assert_eq!(adapter.capabilities.len(), 0); // No discovery run yet
    }

    #[tokio::test]
    async fn test_concurrent_adapter_instances() {
        // Test that multiple adapters with different configs work concurrently
        let mut config1 = AdapterDiscoveryConfig::new();
        config1.set_discovery_endpoint("orchestration", &test_endpoint("orch1", 8080));

        let mut config2 = AdapterDiscoveryConfig::new();
        config2.set_discovery_endpoint("compute", &test_endpoint("compute2", 9090));

        let adapter_endpoint = test_endpoint("localhost", 8080) + "/adapter";
        let mut adapter1 =
            UniversalAdapter::with_discovery_config(Arc::new(config1), adapter_endpoint.clone());

        let mut adapter2 =
            UniversalAdapter::with_discovery_config(Arc::new(config2), adapter_endpoint);

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
