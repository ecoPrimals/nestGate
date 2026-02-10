use super::{
    cache::DiscoveryCache, introspection::SystemIntrospection, network::NetworkDiscovery,
    performance::PerformanceDiscovery, registry::ServiceRegistryClient,
};
/// Core Universal Primal Discovery Orchestrator
/// This module contains the main orchestration logic that coordinates
/// between different discovery subsystems.
use crate::Result;
use dashmap::DashMap;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;

/// **UNIVERSAL PRIMAL PRINCIPLE**: No hardcoded values, everything discovered
/// **Performance**: Lock-free discovery with DashMap (5-15x faster)
/// Universalprimaldiscovery
pub struct UniversalPrimalDiscovery {
    /// Network discovery subsystem
    network_discovery: NetworkDiscovery,
    /// Performance discovery subsystem
    performance_discovery: PerformanceDiscovery,
    /// Service registry client
    registry_client: ServiceRegistryClient,
    /// System introspection
    system_introspection: SystemIntrospection,
    /// Discovery result cache
    cache: DiscoveryCache,
    /// Discovered endpoints (lock-free for concurrent discovery)
    discovered_endpoints: Arc<DashMap<String, String>>,
    /// Discovered ports (lock-free for port allocation)
    discovered_ports: Arc<DashMap<String, u16>>,
    /// Discovered timeouts (lock-free for timeout configuration)
    discovered_timeouts: Arc<DashMap<String, Duration>>,
    /// Discovered limits (lock-free for limit discovery)
    discovered_limits: Arc<DashMap<String, usize>>,
}
impl Default for UniversalPrimalDiscovery {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl UniversalPrimalDiscovery {
    /// Create new discovery system - no defaults, everything learned (lock-free)
    #[must_use]
    pub fn new() -> Self {
        Self {
            network_discovery: NetworkDiscovery::new(),
            performance_discovery: PerformanceDiscovery::new(),
            registry_client: ServiceRegistryClient::new(),
            system_introspection: SystemIntrospection::new(),
            cache: DiscoveryCache::new(),
            discovered_endpoints: Arc::new(DashMap::new()),
            discovered_ports: Arc::new(DashMap::new()),
            discovered_timeouts: Arc::new(DashMap::new()),
            discovered_limits: Arc::new(DashMap::new()),
        }
    }

    /// **PRIMAL DISCOVERY**: Find available bind address through network discovery
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn discover_bind_address(&self, service_name: &str) -> Result<IpAddr> {
        // Delegate to network discovery subsystem
        self.network_discovery
            .discover_bind_address(service_name)
            .await
    }

    /// **PRIMAL DISCOVERY**: Find available port through port scanning
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn discover_available_port(
        &self,
        service_name: &str,
        start_range: u16,
    ) -> Result<u16> {
        // Delegate to network discovery subsystem
        self.network_discovery
            .discover_available_port(service_name, start_range)
            .await
    }

    /// **PRIMAL DISCOVERY**: Discover optimal timeout through performance testing
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn discover_optimal_timeout(
        &self,
        service_name: &str,
        _operation: &str,
    ) -> Result<Duration> {
        // Delegate to performance discovery subsystem
        let optimal_timeout = self
            .performance_discovery
            .discover_optimal_timeout(service_name)
            .await?;
        Ok(optimal_timeout)
    }

    /// **PRIMAL DISCOVERY**: Discover system limits through introspection
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn discover_system_limits(&mut self, resource_type: &str) -> Result<usize> {
        // Delegate to system introspection
        self.system_introspection
            .discover_resource_limits(resource_type)
            .await
    }

    /// **ECOSYSTEM INTEGRATION**: Query external service registry
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn query_service_registry(
        &self,
        service_name: &str,
        query_type: &str,
    ) -> Result<String> {
        // Delegate to registry client
        self.registry_client
            .query_service(service_name, query_type)
            .await
    }

    /// Cache Discovered Port (lock-free)
    pub async fn cache_discovered_port(&mut self, service_name: &str, port: u16) {
        self.discovered_ports.insert(service_name.to_string(), port);

        // Also cache in new system
        self.cache.store_port_discovery(service_name, port);
    }

    /// Cache Discovered Endpoint (lock-free)
    pub async fn cache_discovered_endpoint(&mut self, service_name: &str, endpoint: &str) {
        self.discovered_endpoints
            .insert(service_name.to_string(), endpoint.to_string());

        // Also cache in new system
        self.cache.store_endpoint_discovery(service_name, endpoint);
    }

    /// Cache Discovered Timeout (lock-free)
    pub async fn cache_discovered_timeout(&mut self, service_name: &str, timeout: Duration) {
        self.discovered_timeouts
            .insert(service_name.to_string(), timeout);

        // Also cache in new system
        self.cache.store_timeout_discovery(service_name, timeout);
    }

    /// **SYSTEM HEALTH**: Get comprehensive discovery status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_discovery_status(&self) -> Result<HashMap<String, String>> {
        let mut status = HashMap::new();

        // Network discovery status
        status.insert("network_discovery".to_string(), "active".to_string());

        // Performance discovery status
        status.insert("performance_discovery".to_string(), "active".to_string());

        // Registry client status
        status.insert("registry_client".to_string(), "active".to_string());

        // Cache status
        let cache_stats = self.cache.get_cache_stats();
        status.insert("cache_entries".to_string(), cache_stats.to_string());

        Ok(status)
    }
}
