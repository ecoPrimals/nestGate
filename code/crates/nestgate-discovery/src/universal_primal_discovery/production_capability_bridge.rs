// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **PRODUCTION CAPABILITY BRIDGE**
//!
//! Bridges old environment-based discovery with new capability-based discovery.
//! Provides backward compatibility while enabling migration to modern patterns.
//!
//! ## Migration Strategy
//!
//! ```rust,ignore
//! // OLD WAY (still works, deprecated):
//! let discovery = ProductionServiceDiscovery::new(config)?;
//! let port = discovery.discover_port("api")?;  // Hardcoded fallbacks
//!
//! // NEW WAY (capability-based):
//! let discovery = CapabilityAwareDiscovery::initialize(config).await?;
//! let services = discovery.find_service("api").await?;  // No hardcoding!
//! let best = services.first().ok_or_else(|| Error::not_found("api"))?;
//! ```
//!
//! ## Evolution Timeline
//! - **Phase 1** (Current): Bridge operational, both APIs work
//! - **Phase 2** (Week 2): Migrate callers to capability-based API
//! - **Phase 3** (Week 3): Deprecate environment-only discovery
//! - **Phase 4** (Week 4): Remove deprecated code

use crate::universal_primal_discovery::backends::InMemoryDiscoveryBackend;
use crate::universal_primal_discovery::capability_based_discovery::{
    CapabilityDiscoveryManager, DiscoveryQuery, PeerDescriptor, PrimalCapability,
};
use nestgate_config::config::canonical_primary::NestGateCanonicalConfig;
use nestgate_types::error::{NestGateError, Result};
use nestgate_types::{EnvSource, ProcessEnv};
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, info, warn};

/// Capability-aware discovery that integrates old and new systems
///
/// This provides a migration path from environment-based to capability-based discovery.
pub struct CapabilityAwareDiscovery {
    /// New capability-based discovery
    capability_discovery: Arc<CapabilityDiscoveryManager>,
    /// Configuration for backward compatibility and future extension
    #[expect(
        dead_code,
        reason = "Carried for migration parity; hot paths use capability_discovery only"
    )]
    config: Arc<NestGateCanonicalConfig>,
}

impl CapabilityAwareDiscovery {
    /// Initialize capability-aware discovery system
    ///
    /// This is the main entry point for modern capability-based service discovery.
    /// It automatically detects this primal's capabilities and sets up appropriate
    /// discovery backends (mDNS, in-memory, etc.) based on the environment.
    ///
    /// # Self-Knowledge Pattern
    ///
    /// Following the self-knowledge principle, this function:
    /// - Introspects only this primal's own capabilities
    /// - Does NOT hardcode knowledge of other primals
    /// - Discovers peers at runtime through capability queries
    /// - Announces self to the network for others to discover
    ///
    /// # Arguments
    ///
    /// * `config` - `NestGate` canonical configuration
    ///
    /// # Returns
    ///
    /// A fully initialized `CapabilityAwareDiscovery` instance ready for queries
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Capability detection fails
    /// - Backend initialization fails
    /// - Network announcement fails
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use nestgate_core::universal_primal_discovery::production_capability_bridge::CapabilityAwareDiscovery;
    /// # use nestgate_core::config::canonical_primary::NestGateCanonicalConfig;
    /// # async fn example(config: &NestGateCanonicalConfig) -> nestgate_core::Result<()> {
    /// // Initialize discovery (detects own capabilities automatically)
    /// let discovery = CapabilityAwareDiscovery::initialize(config).await?;
    ///
    /// // Find services by capability (no hardcoding!)
    /// let storage_services = discovery.find_service("storage").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn initialize(config: &NestGateCanonicalConfig) -> Result<Self> {
        Self::initialize_from_env_source(config, &ProcessEnv).await
    }

    /// Like [`Self::initialize`], but reads capability-related flags from an injectable [`EnvSource`].
    pub async fn initialize_from_env_source(
        config: &NestGateCanonicalConfig,
        env: &(impl EnvSource + ?Sized),
    ) -> Result<Self> {
        // Detect capabilities this primal provides
        let capabilities = Self::detect_own_capabilities(config, env);

        info!(
            "Initializing capability-aware discovery with {:?}",
            capabilities
        );

        // Create capability discovery manager
        let mut manager = CapabilityDiscoveryManager::initialize(capabilities).await?;

        // Add backends based on environment
        Self::setup_backends(&mut manager, config);

        // Announce self to network
        manager.start_announcing().await?;

        Ok(Self {
            capability_discovery: Arc::new(manager),
            config: Arc::new(config.clone()),
        })
    }

    /// Detect what capabilities this primal provides
    ///
    /// Inspects configuration and environment to determine capabilities.
    fn detect_own_capabilities(
        config: &NestGateCanonicalConfig,
        env: &(impl EnvSource + ?Sized),
    ) -> Vec<PrimalCapability> {
        let mut capabilities = Vec::new();

        // Check for API capability
        if Self::has_api_capability(env, config) {
            capabilities.push(PrimalCapability::ApiGateway);
            info!("Detected API Gateway capability");
        }

        // Check for ZFS capability
        if Self::has_zfs_capability() {
            capabilities.push(PrimalCapability::ZfsStorage);
            info!("Detected ZFS Storage capability");
        }

        // Check for observability capability
        if Self::has_observability_capability(env, config) {
            capabilities.push(PrimalCapability::Observability);
            info!("Detected Observability capability");
        }

        // Check for NFS capability
        if Self::has_nfs_capability(env) {
            capabilities.push(PrimalCapability::NetworkFileSystem(
                crate::universal_primal_discovery::capability_based_discovery::NfsVersion::V4,
            ));
            info!("Detected NFS capability");
        }

        // Always provide service discovery
        capabilities.push(PrimalCapability::ServiceDiscovery);

        if capabilities.is_empty() {
            warn!("No capabilities detected, defaulting to service discovery only");
        }

        capabilities
    }

    /// Check if API capability is available
    fn has_api_capability(
        env: &(impl EnvSource + ?Sized),
        _config: &NestGateCanonicalConfig,
    ) -> bool {
        // Check if API server is configured
        env.get("NESTGATE_API_ENABLED")
            .is_none_or(|v| v == "true" || v == "1") // Default to true when unset
    }

    /// Check if ZFS capability is available
    fn has_zfs_capability() -> bool {
        // Check if ZFS commands are available
        std::process::Command::new("zfs")
            .arg("version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Check if observability capability is available
    fn has_observability_capability(
        env: &(impl EnvSource + ?Sized),
        _config: &NestGateCanonicalConfig,
    ) -> bool {
        // Check if metrics/tracing is enabled
        env.get("NESTGATE_OBSERVABILITY_ENABLED")
            .is_some_and(|v| v == "true" || v == "1")
    }

    /// Check if NFS capability is available
    fn has_nfs_capability(env: &(impl EnvSource + ?Sized)) -> bool {
        // Check if NFS is configured
        env.get("NESTGATE_NFS_ENABLED")
            .is_some_and(|v| v == "true" || v == "1")
    }

    /// Setup discovery backends based on environment
    fn setup_backends(manager: &mut CapabilityDiscoveryManager, _config: &NestGateCanonicalConfig) {
        // Always add in-memory backend for local testing/fallback
        manager.add_backend(Arc::new(InMemoryDiscoveryBackend::new()));
        info!("Added in-memory discovery backend");

        // mDNS removed — local network discovery delegated to orchestration-provider IPC

        // FUTURE: Add Kubernetes backend when deploying to k8s (auto-detected via env)
        // if std::env::var("KUBERNETES_SERVICE_HOST").is_ok() {
        //     manager.add_backend(Arc::new(KubernetesBackend::new()?));
        // }

        // FUTURE: Add Consul backend when configured (auto-detected via env)
        // if let Ok(consul_addr) = std::env::var("CONSUL_HTTP_ADDR") {
        //     manager.add_backend(Arc::new(ConsulBackend::new(&consul_addr)?));
        // }
    }

    /// Find services by name (capability-based)
    ///
    /// This is the NEW API - uses capability discovery without hardcoded fallbacks.
    ///
    /// # Errors
    ///
    /// Returns error if discovery fails or no services found
    pub async fn find_service(&self, service_name: &str) -> Result<Vec<PeerDescriptor>> {
        debug!("Finding service: {}", service_name);

        // Map service names to capabilities
        let capability = match service_name {
            "api" | "api-gateway" => PrimalCapability::ApiGateway,
            "zfs" | "storage" => PrimalCapability::ZfsStorage,
            "metrics" | "observability" => PrimalCapability::Observability,
            "auth" | "authentication" => PrimalCapability::Authentication,
            "nfs" => PrimalCapability::NetworkFileSystem(
                crate::universal_primal_discovery::capability_based_discovery::NfsVersion::V4,
            ),
            _ => PrimalCapability::Custom(service_name.to_string()),
        };

        self.capability_discovery.find_capability(capability).await
    }

    /// Query services with complex capability requirements
    ///
    /// Provides advanced querying capabilities for finding services that match
    /// specific criteria beyond simple name-based lookup. Useful for scenarios
    /// requiring specific combinations of capabilities, versions, or other attributes.
    ///
    /// # Arguments
    ///
    /// * `query` - A `DiscoveryQuery` specifying requirements (capabilities, tags, etc.)
    ///
    /// # Returns
    ///
    /// A vector of `PeerDescriptor` instances matching the query criteria
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Query execution fails
    /// - Network communication errors occur
    /// - Invalid query parameters
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Requires initialized CapabilityAwareDiscovery; see tests for full example
    /// let query = DiscoveryQuery::for_capability(PrimalCapability::ZfsStorage);
    /// let services = discovery.query_services(&query).await?;
    /// ```
    pub async fn query_services(&self, query: &DiscoveryQuery) -> Result<Vec<PeerDescriptor>> {
        self.capability_discovery.query(query).await
    }

    /// Get best service endpoint for a capability
    ///
    /// Selects the best service endpoint based on latency and health
    ///
    /// This convenience method finds all services matching the name, ranks them
    /// by latency and health status, and returns the endpoint of the best option.
    /// Useful for load balancing and automatic failover scenarios.
    ///
    /// # Selection Criteria
    ///
    /// Services are ranked by:
    /// 1. Health status (healthy services preferred)
    /// 2. Latency (lower latency preferred)
    /// 3. Load (less loaded services preferred)
    ///
    /// # Arguments
    ///
    /// * `service_name` - Name of the service to find
    ///
    /// # Returns
    ///
    /// A `SocketAddr` representing the best available endpoint
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - No services found with the specified name
    /// - All discovered services are unhealthy
    /// - Network communication fails
    /// - Service metadata is invalid
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use nestgate_core::universal_primal_discovery::production_capability_bridge::CapabilityAwareDiscovery;
    /// # async fn example(discovery: &CapabilityAwareDiscovery) -> nestgate_core::Result<()> {
    /// // Get best endpoint automatically
    /// let endpoint = discovery.get_best_endpoint("api").await?;
    /// println!("Connecting to: {}", endpoint);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_best_endpoint(&self, service_name: &str) -> Result<SocketAddr> {
        let services = self.find_service(service_name).await?;

        let best = services
            .into_iter()
            .filter(|s| {
                matches!(
                s.health,
                crate::universal_primal_discovery::capability_based_discovery::HealthStatus::Healthy
            )
            })
            .min_by_key(|s| s.latency.unwrap_or(Duration::from_secs(999)))
            .ok_or_else(|| {
                NestGateError::not_found(format!("No healthy {service_name} service"))
            })?;

        Ok(best.endpoint.address)
    }

    /// Discover port - DEPRECATED (use `find_service` instead)
    ///
    /// Maintained for backward compatibility. New code should use `find_service`.
    ///
    /// # Errors
    ///
    /// Returns error if discovery fails
    #[deprecated(
        since = "0.12.0",
        note = "Use find_service() for capability-based discovery without hardcoded fallbacks"
    )]
    pub async fn discover_port_compat(&self, service_name: &str) -> Result<u16> {
        match self.find_service(service_name).await {
            Ok(services) if !services.is_empty() => Ok(services[0].endpoint.address.port()),
            Ok(_) => {
                // No services found - return error (no hardcoded fallback!)
                Err(NestGateError::not_found(format!(
                    "Service '{service_name}' not found in discovery"
                )))
            }
            Err(e) => Err(e),
        }
    }

    /// Discover bind address - DEPRECATED
    ///
    /// # Errors
    ///
    /// Returns error if discovery fails
    #[deprecated(
        since = "0.12.0",
        note = "Use find_service() for capability-based discovery"
    )]
    pub async fn discover_bind_address_compat(&self, service_name: &str) -> Result<IpAddr> {
        match self.find_service(service_name).await {
            Ok(services) if !services.is_empty() => Ok(services[0].endpoint.address.ip()),
            Ok(_) => Err(NestGateError::not_found(format!(
                "Service '{service_name}' not found"
            ))),
            Err(e) => Err(e),
        }
    }

    /// Gracefully shutdown discovery and unannounce from network
    ///
    /// Performs cleanup operations including:
    /// - Unannouncing this primal from the network
    /// - Closing discovery backend connections
    /// - Releasing network resources
    ///
    /// Should be called during application shutdown for clean exit.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Network unannouncement fails
    /// - Backend cleanup encounters errors
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use nestgate_core::universal_primal_discovery::production_capability_bridge::CapabilityAwareDiscovery;
    /// # async fn example(discovery: CapabilityAwareDiscovery) -> nestgate_core::Result<()> {
    /// // During application shutdown
    /// discovery.shutdown().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down capability-aware discovery");
        self.capability_discovery.shutdown().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> NestGateCanonicalConfig {
        NestGateCanonicalConfig::default()
    }

    #[tokio::test]
    async fn test_capability_detection() {
        let config = test_config();
        let capabilities = CapabilityAwareDiscovery::detect_own_capabilities(&config, &ProcessEnv);

        // Should at least have ServiceDiscovery
        assert!(!capabilities.is_empty());
        assert!(capabilities.contains(&PrimalCapability::ServiceDiscovery));
    }

    #[tokio::test]
    async fn test_initialization() {
        let config = test_config();
        let discovery = CapabilityAwareDiscovery::initialize(&config).await;

        // Should initialize without error
        assert!(discovery.is_ok());
    }

    #[tokio::test]
    async fn test_backend_setup() {
        let config = test_config();
        let mut manager = CapabilityDiscoveryManager::initialize(vec![])
            .await
            .unwrap();

        CapabilityAwareDiscovery::setup_backends(&mut manager, &config);
    }

    #[tokio::test]
    async fn test_find_nonexistent_service() {
        let config = test_config();
        let discovery = CapabilityAwareDiscovery::initialize(&config).await.unwrap();

        // Should return empty or error (no hardcoded fallback)
        let result = discovery.find_service("nonexistent").await;

        // Either empty list or error is fine (no hardcoded fallback!)
        match result {
            Ok(services) => assert!(services.is_empty()),
            Err(_) => {} // Expected - no hardcoded fallback
        }
    }
}
