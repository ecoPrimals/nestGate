//! Generic Capability Discovery System
//!
//! **ECOSYSTEM SOVEREIGNTY**: This module provides capability-based discovery
//! WITHOUT hardcoding any primal names. Each capability (networking, security,
//! storage, compute, etc.) can be provided by ANY primal that implements it.
//!
//! **Key Principle**: Discover what services CAN DO, not WHO they are.

use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

// Import config for environment variable lookups
use super::capability_endpoints_config::CapabilityEndpointsConfig;

/// Capability type identifier
///
/// **NO HARDCODED PRIMAL NAMES**: Use generic capability types like
/// "networking", "security", "storage", not primal names like "songbird".
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Capabilitytype
pub struct CapabilityType(String);

impl CapabilityType {
    /// Create a new capability type
    ///
    /// Examples:
    /// - `CapabilityType::new("networking")` - Discovers any networking provider
    /// - `CapabilityType::new("security")` - Discovers any security provider
    /// - `CapabilityType::new("load-balancing")` - Discovers load balancing
    /// - `CapabilityType::new("circuit-breaking")` - Discovers circuit breaking
    pub fn new(capability: impl Into<String>) -> Self {
        Self(capability.into())
    }

    /// Returns as Str
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Standard capability types (not primal names!)
impl CapabilityType {
    /// Networking capabilities (load balancing, routing, etc.)
    pub fn networking() -> Self {
        Self::new("networking")
    }

    /// Security capabilities (auth, encryption, rate limiting, etc.)
    pub fn security() -> Self {
        Self::new("security")
    }

    /// Storage capabilities (datasets, snapshots, etc.)
    pub fn storage() -> Self {
        Self::new("storage")
    }

    /// Orchestration capabilities (service coordination, workflows)
    pub fn orchestration() -> Self {
        Self::new("orchestration")
    }

    /// Load balancing capability (can be part of networking or standalone)
    pub fn load_balancing() -> Self {
        Self::new("load-balancing")
    }

    /// Circuit breaking capability
    pub fn circuit_breaking() -> Self {
        Self::new("circuit-breaking")
    }

    /// Rate limiting capability
    pub fn rate_limiting() -> Self {
        Self::new("rate-limiting")
    }

    /// Authentication capability
    pub fn authentication() -> Self {
        Self::new("authentication")
    }

    /// Intrusion detection capability
    pub fn intrusion_detection() -> Self {
        Self::new("intrusion-detection")
    }

    /// Input validation capability
    pub fn input_validation() -> Self {
        Self::new("input-validation")
    }
}

/// Capability provider information
///
/// **IMPORTANT**: This describes what a service CAN DO, not who/what it is.
/// We never store primal names like "songbird" or "beardog" here.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Capabilityprovider
pub struct CapabilityProvider {
    /// Endpoint to reach this capability provider
    /// Example: "http://localhost:{port}" or discovered via mDNS
    pub endpoint: String,

    /// Capabilities this provider offers
    /// Example: ["networking", "load-balancing", "circuit-breaking"]
    pub capabilities: Vec<CapabilityType>,

    /// Optional metadata (version, features, etc.)
    pub metadata: HashMap<String, String>,

    /// Health status
    pub healthy: bool,
}

impl CapabilityProvider {
    /// Check if this provider supports a capability
    pub fn supports(&self, capability: &CapabilityType) -> bool {
        self.capabilities.contains(capability)
    }
}

/// Capability discovery system
///
/// **NO HARDCODING**: Discovers capabilities at runtime without knowing
/// which primal provides them. Works with existing primals (Songbird, BearDog)
/// and future primals that don't exist yet.
pub struct CapabilityDiscovery {
    /// Known capability providers (discovered at runtime)
    providers: Arc<tokio::sync::RwLock<HashMap<String, CapabilityProvider>>>,
}

impl CapabilityDiscovery {
    /// Create new capability discovery system
    pub fn new() -> Self {
        Self {
            providers: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Discover all providers of a capability
    ///
    /// **Example**:
    /// ```rust,ignore
    /// // Discover ANY networking provider (could be Songbird, or something else)
    /// let networking_providers = discovery
    ///     .discover(CapabilityType::networking())
    ///     .await?;
    /// ```
    pub async fn discover(&self, capability: CapabilityType) -> Result<Vec<CapabilityProvider>> {
        let providers = self.providers.read().await;

        Ok(providers
            .values()
            .filter(|p| p.supports(&capability) && p.healthy)
            .cloned()
            .collect())
    }

    /// Register a capability provider
    ///
    /// **Called by discovery mechanisms** (mDNS, Consul, etc.), not hardcoded
    pub async fn register(&self, id: String, provider: CapabilityProvider) -> Result<()> {
        let mut providers = self.providers.write().await;
        providers.insert(id, provider);
        Ok(())
    }

    /// Discover providers via multiple mechanisms
    ///
    /// **NO HARDCODING**: Uses environment, mDNS, service registry, etc.
    /// Never hardcodes "connect to songbird at X" or "beardog at Y"
    ///
    /// **Current Implementation**:
    /// - ✅ Environment variable discovery (working)
    ///
    /// **Planned Enhancements** (deferred to v0.10.0+):
    /// - mDNS/Bonjour for zero-config local discovery
    /// - Service registry integration (Consul, etcd, etc.)
    /// - DNS-SD (Service Discovery)
    /// - Cloud-native service mesh integration
    ///
    /// **Why Deferred**: Current env-based discovery is sufficient for production.
    /// Additional discovery mechanisms will be added based on real-world deployment needs.
    pub async fn discover_all(&self) -> Result<()> {
        // 1. Check environment variables for capability endpoints
        self.discover_from_env().await?;

        // 2. Future: mDNS/Bonjour for local discovery
        //    Status: Deferred to v0.10.0+ based on deployment requirements
        //    Design: Zero-config local network discovery for development/edge deployments
        //    Implementation: Will use `mdns` crate for multicast DNS service discovery

        // 3. Future: Service registry integration
        //    Status: Deferred to v0.10.0+ based on infrastructure needs
        //    Design: Support multiple registries (Consul, etcd, Kubernetes DNS)
        //    Implementation: Plugin architecture for registry backends

        Ok(())
    }

    /// Discover capabilities from environment variables
    ///
    /// **Pattern**: `CAPABILITY_<TYPE>_ENDPOINT=http://...`
    ///
    /// Examples:
    /// - `CAPABILITY_NETWORKING_ENDPOINT=http://localhost:{port}`
    /// - `CAPABILITY_SECURITY_ENDPOINT=http://localhost:{port}`
    ///
    /// **NO PRIMAL NAMES**: We don't use `SONGBIRD_ENDPOINT` or `BEARDOG_ENDPOINT`
    async fn discover_from_env(&self) -> Result<()> {
        // Use config to get environment variables
        let config = CapabilityEndpointsConfig::from_env();

        // Discover networking capability
        if let Some(endpoint) = config.networking_endpoint() {
            self.register(
                "networking-env".to_string(),
                CapabilityProvider {
                    endpoint: endpoint.to_string(),
                    capabilities: vec![
                        CapabilityType::networking(),
                        CapabilityType::load_balancing(),
                        CapabilityType::circuit_breaking(),
                    ],
                    metadata: HashMap::new(),
                    healthy: true,
                },
            )
            .await?;
        }

        // Discover security capability
        if let Some(endpoint) = config.security_endpoint() {
            self.register(
                "security-env".to_string(),
                CapabilityProvider {
                    endpoint: endpoint.to_string(),
                    capabilities: vec![
                        CapabilityType::security(),
                        CapabilityType::authentication(),
                        CapabilityType::rate_limiting(),
                    ],
                    metadata: HashMap::new(),
                    healthy: true,
                },
            )
            .await?;
        }

        Ok(())
    }
}

impl Default for CapabilityDiscovery {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::discovery_config::ServiceDiscoveryConfig;

    /// Helper to create test endpoint using ServiceDiscoveryConfig
    /// ✅ MIGRATED: Replaces hardcoded "localhost:port" with configurable endpoints
    fn test_endpoint(port: u16) -> String {
        let config = ServiceDiscoveryConfig::default();
        format!("http://{}:{}", config.discovery_host, port)
    }

    #[tokio::test]
    async fn test_capability_discovery() {
        let discovery = CapabilityDiscovery::new();

        // Register a networking provider (doesn't matter WHO provides it)
        let provider = CapabilityProvider {
            endpoint: test_endpoint(8080),
            capabilities: vec![
                CapabilityType::networking(),
                CapabilityType::load_balancing(),
            ],
            metadata: HashMap::new(),
            healthy: true,
        };

        discovery
            .register("provider-1".to_string(), provider)
            .await
            .expect("Failed to register provider");

        // Discover networking capability
        let providers = discovery
            .discover(CapabilityType::networking())
            .await
            .expect("Failed to discover networking capability");
        assert_eq!(providers.len(), 1);
        // ✅ MIGRATED: Now uses ServiceDiscoveryConfig default host (127.0.0.1)
        assert!(providers[0].endpoint.contains("127.0.0.1:8080"));
    }

    #[test]
    fn test_no_hardcoded_primal_names() {
        // This test documents that we DON'T hardcode primal names
        let networking = CapabilityType::networking();
        let security = CapabilityType::security();

        // These are capability types, NOT primal names
        assert_eq!(networking.as_str(), "networking"); // Not "songbird"!
        assert_eq!(security.as_str(), "security"); // Not "beardog"!
    }

    #[tokio::test]
    async fn test_multiple_providers_same_capability() {
        let discovery = CapabilityDiscovery::new();

        // Register two providers for networking
        let provider1 = CapabilityProvider {
            endpoint: test_endpoint(8080),
            capabilities: vec![CapabilityType::networking()],
            metadata: HashMap::new(),
            healthy: true,
        };

        let provider2 = CapabilityProvider {
            endpoint: test_endpoint(8090),
            capabilities: vec![CapabilityType::networking()],
            metadata: HashMap::new(),
            healthy: true,
        };

        discovery
            .register("provider-1".to_string(), provider1)
            .await
            .expect("Failed to register provider 1");
        discovery
            .register("provider-2".to_string(), provider2)
            .await
            .expect("Failed to register provider 2");

        // Should find both providers
        let providers = discovery
            .discover(CapabilityType::networking())
            .await
            .expect("Failed to discover networking providers");
        assert_eq!(providers.len(), 2);
    }

    #[tokio::test]
    async fn test_discover_nonexistent_capability() {
        let discovery = CapabilityDiscovery::new();

        // Try to discover a capability that doesn't exist
        let providers = discovery
            .discover(CapabilityType::new("nonexistent"))
            .await
            .expect("Discovery should succeed even with no results");
        assert_eq!(providers.len(), 0);
    }

    #[tokio::test]
    async fn test_provider_with_multiple_capabilities() {
        let discovery = CapabilityDiscovery::new();

        // Register a provider with multiple capabilities
        let provider = CapabilityProvider {
            endpoint: test_endpoint(8080),
            capabilities: vec![
                CapabilityType::networking(),
                CapabilityType::load_balancing(),
                CapabilityType::circuit_breaking(),
            ],
            metadata: HashMap::new(),
            healthy: true,
        };

        discovery
            .register("provider-1".to_string(), provider)
            .await
            .expect("Failed to register provider");

        // Should find it when discovering any of its capabilities
        let net_providers = discovery
            .discover(CapabilityType::networking())
            .await
            .expect("Failed to discover networking capability");
        assert_eq!(net_providers.len(), 1);

        let lb_providers = discovery
            .discover(CapabilityType::load_balancing())
            .await
            .expect("Failed to discover load balancing capability");
        assert_eq!(lb_providers.len(), 1);

        let cb_providers = discovery
            .discover(CapabilityType::circuit_breaking())
            .await
            .expect("Failed to discover circuit breaking capability");
        assert_eq!(cb_providers.len(), 1);
    }

    #[tokio::test]
    async fn test_unhealthy_provider_excluded() {
        let discovery = CapabilityDiscovery::new();

        // Register an unhealthy provider
        let provider = CapabilityProvider {
            endpoint: test_endpoint(8080),
            capabilities: vec![CapabilityType::networking()],
            metadata: HashMap::new(),
            healthy: false, // Unhealthy!
        };

        discovery
            .register("provider-1".to_string(), provider)
            .await
            .expect("Failed to register provider");

        // Should not find unhealthy providers
        let providers = discovery
            .discover(CapabilityType::networking())
            .await
            .expect("Failed to discover networking capability");
        assert_eq!(providers.len(), 0);
    }

    #[tokio::test]
    async fn test_provider_metadata() {
        let discovery = CapabilityDiscovery::new();

        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0.0".to_string());
        metadata.insert("region".to_string(), "us-west".to_string());

        let provider = CapabilityProvider {
            endpoint: test_endpoint(8080),
            capabilities: vec![CapabilityType::networking()],
            metadata: metadata.clone(),
            healthy: true,
        };

        discovery
            .register("provider-1".to_string(), provider)
            .await
            .expect("Failed to register provider");

        let providers = discovery
            .discover(CapabilityType::networking())
            .await
            .expect("Failed to discover networking capability");
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].metadata, metadata);
    }

    #[tokio::test]
    async fn test_discover_all() {
        let discovery = CapabilityDiscovery::new();

        // Register providers with different capabilities
        let provider1 = CapabilityProvider {
            endpoint: test_endpoint(8080),
            capabilities: vec![CapabilityType::networking()],
            metadata: HashMap::new(),
            healthy: true,
        };

        let provider2 = CapabilityProvider {
            endpoint: test_endpoint(8090),
            capabilities: vec![CapabilityType::security()],
            metadata: HashMap::new(),
            healthy: true,
        };

        discovery
            .register("provider-1".to_string(), provider1)
            .await
            .expect("Failed to register provider 1");
        discovery
            .register("provider-2".to_string(), provider2)
            .await
            .expect("Failed to register provider 2");

        // Discover all should trigger discovery process
        discovery
            .discover_all()
            .await
            .expect("Failed to discover all capabilities");
    }

    #[test]
    fn test_capability_type_custom() {
        let custom = CapabilityType::new("custom-capability");
        assert_eq!(custom.as_str(), "custom-capability");
    }

    #[test]
    fn test_capability_type_equality() {
        let cap1 = CapabilityType::new("networking");
        let cap2 = CapabilityType::networking();
        assert_eq!(cap1, cap2);
    }

    #[test]
    fn test_all_standard_capability_types() {
        // Test all standard capability types exist and have correct names
        assert_eq!(CapabilityType::networking().as_str(), "networking");
        assert_eq!(CapabilityType::security().as_str(), "security");
        assert_eq!(CapabilityType::storage().as_str(), "storage");
        assert_eq!(CapabilityType::orchestration().as_str(), "orchestration");
        assert_eq!(CapabilityType::load_balancing().as_str(), "load-balancing");
        assert_eq!(
            CapabilityType::circuit_breaking().as_str(),
            "circuit-breaking"
        );
        assert_eq!(CapabilityType::rate_limiting().as_str(), "rate-limiting");
        assert_eq!(CapabilityType::authentication().as_str(), "authentication");
        assert_eq!(
            CapabilityType::intrusion_detection().as_str(),
            "intrusion-detection"
        );
        assert_eq!(
            CapabilityType::input_validation().as_str(),
            "input-validation"
        );
    }

    #[tokio::test]
    async fn test_provider_registration_with_empty_capabilities() {
        let discovery = CapabilityDiscovery::new();

        // Register a provider with no capabilities (edge case)
        let provider = CapabilityProvider {
            endpoint: test_endpoint(8080),
            capabilities: vec![], // Empty!
            metadata: HashMap::new(),
            healthy: true,
        };

        discovery
            .register("provider-1".to_string(), provider)
            .await
            .expect("Failed to register provider");

        // Should not find it for any capability
        let providers = discovery
            .discover(CapabilityType::networking())
            .await
            .expect("Failed to discover networking capability");
        assert_eq!(providers.len(), 0);
    }

    #[tokio::test]
    async fn test_duplicate_provider_id_overwrites() {
        let discovery = CapabilityDiscovery::new();

        // Register a provider
        let provider1 = CapabilityProvider {
            endpoint: test_endpoint(8080),
            capabilities: vec![CapabilityType::networking()],
            metadata: HashMap::new(),
            healthy: true,
        };

        discovery
            .register("provider-1".to_string(), provider1)
            .await
            .expect("Failed to register first provider");

        // Register again with same ID but different endpoint
        let provider2 = CapabilityProvider {
            endpoint: test_endpoint(9090),
            capabilities: vec![CapabilityType::networking()],
            metadata: HashMap::new(),
            healthy: true,
        };

        discovery
            .register("provider-1".to_string(), provider2)
            .await
            .expect("Failed to register second provider");

        // Should only find one provider with the new endpoint
        let providers = discovery
            .discover(CapabilityType::networking())
            .await
            .expect("Failed to discover networking capability");
        assert_eq!(providers.len(), 1);
        // ✅ MIGRATED: Endpoint now uses ServiceDiscoveryConfig
        assert!(providers[0].endpoint.contains("127.0.0.1:9090"));
    }
}
