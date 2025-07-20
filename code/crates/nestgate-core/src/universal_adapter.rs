//! Universal Primal Adapter
//!
//! This module provides the universal adapter that coordinates between different
//! primal providers and handles auto-discovery without hardcoding specific implementations.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::net::TcpStream;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use crate::universal_traits::*;
use crate::{NestGateError, Result};

/// Universal Primal Adapter that coordinates between different primal providers
pub struct UniversalPrimalAdapter {
    /// Security primal providers (any security implementation)
    security_providers: Arc<RwLock<HashMap<String, Arc<dyn SecurityPrimalProvider>>>>,
    /// Orchestration primal providers (any orchestration implementation)
    orchestration_providers: Arc<RwLock<HashMap<String, Arc<dyn OrchestrationPrimalProvider>>>>,
    /// Compute primal providers (any compute implementation)
    compute_providers: Arc<RwLock<HashMap<String, Arc<dyn ComputePrimalProvider>>>>,
    /// Discovery service for finding available primals
    discovery_service: Arc<PrimalDiscoveryService>,
    /// Configuration for the adapter
    config: UniversalAdapterConfig,
}

/// Configuration for the universal adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalAdapterConfig {
    /// Enable auto-discovery of primal providers
    pub auto_discovery: bool,
    /// Discovery interval in seconds
    pub discovery_interval: u64,
    /// Timeout for primal requests in seconds
    pub request_timeout: u64,
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Fallback behavior when no primal is available
    pub fallback_behavior: FallbackBehavior,
    /// Discovery methods to use
    pub discovery_methods: Vec<DiscoveryMethod>,
}

impl Default for UniversalAdapterConfig {
    fn default() -> Self {
        Self {
            auto_discovery: true,
            discovery_interval: 30,
            request_timeout: 30,
            max_retries: 3,
            fallback_behavior: FallbackBehavior::NoOp,
            discovery_methods: vec![
                DiscoveryMethod::Environment,
                DiscoveryMethod::ServiceRegistry,
                DiscoveryMethod::NetworkScan,
            ],
        }
    }
}

/// Fallback behavior when no primal is available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FallbackBehavior {
    /// Return an error
    Error,
    /// Return a no-op result
    NoOp,
    /// Use a local implementation
    Local,
}

/// Discovery methods for finding primal providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    /// Environment variables
    Environment,
    /// Service registry lookup
    ServiceRegistry,
    /// Network scanning
    NetworkScan,
    /// Configuration file
    Configuration,
}

/// Discovered primal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPrimal {
    pub primal_type: String,
    pub capabilities: Vec<String>,
    pub endpoint: String,
    pub version: String,
    pub discovered_via: String,
    pub last_seen: SystemTime,
}

/// Primal discovery service
pub struct PrimalDiscoveryService {
    /// Discovered primal providers
    discovered_providers: Arc<RwLock<HashMap<String, Arc<dyn PrimalProvider>>>>,
    /// Discovered primal information
    discovered_info: Arc<RwLock<HashMap<String, DiscoveredPrimal>>>,
    /// Discovery configuration
    config: DiscoveryConfig,
}

/// Configuration for primal discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Environment variable patterns to scan
    pub env_patterns: Vec<String>,
    /// Service registry endpoints
    pub registry_endpoints: Vec<String>,
    /// Network ranges to scan
    pub network_ranges: Vec<String>,
    /// Common primal ports to check
    pub common_ports: Vec<u16>,
    /// Discovery timeout
    pub timeout: Duration,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            env_patterns: vec![
                "*_SECURITY_*".to_string(),
                "*_AI_*".to_string(),
                "*_ORCHESTRATION_*".to_string(),
                "*_COMPUTE_*".to_string(),
                "BEARDOG_*".to_string(),
                "SQUIRREL_*".to_string(),
                "SONGBIRD_*".to_string(),
                "TOADSTOOL_*".to_string(),
            ],
            registry_endpoints: vec![
                "http://localhost:8500/v1/catalog/services".to_string(), // Consul
                "http://localhost:2379/v2/keys/_primal".to_string(),     // etcd
            ],
            network_ranges: vec![
                "127.0.0.1/32".to_string(),
                "10.0.0.0/8".to_string(),
                "172.16.0.0/12".to_string(),
                "192.168.0.0/16".to_string(),
            ],
            common_ports: vec![8080, 8443, 9000, 9001, 9002, 9003, 8000, 8001, 8002, 8003],
            timeout: Duration::from_secs(5),
        }
    }
}

impl UniversalPrimalAdapter {
    /// Create a new universal primal adapter
    pub fn new(config: UniversalAdapterConfig) -> Self {
        let discovery_service = Arc::new(PrimalDiscoveryService::new(DiscoveryConfig::default()));

        Self {
            security_providers: Arc::new(RwLock::new(HashMap::new())),
            orchestration_providers: Arc::new(RwLock::new(HashMap::new())),
            compute_providers: Arc::new(RwLock::new(HashMap::new())),
            discovery_service,
            config,
        }
    }

    /// Initialize the adapter and start discovery
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing Universal Primal Adapter");

        if self.config.auto_discovery {
            self.start_discovery().await?;
        }

        Ok(())
    }

    /// Start the discovery process
    async fn start_discovery(&self) -> Result<()> {
        info!("Starting primal discovery");

        // Start discovery service
        self.discovery_service.start_discovery().await?;

        // Start periodic discovery updates
        let discovery_service = Arc::clone(&self.discovery_service);
        let interval = self.config.discovery_interval;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(interval));
            loop {
                interval.tick().await;
                if let Err(e) = discovery_service.discover_providers().await {
                    error!("Discovery failed: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Find providers by capability instead of hardcoded type
    pub async fn find_providers_by_capability(&self, capability: &str) -> Vec<DiscoveredPrimal> {
        let discovered_info = self.discovery_service.discovered_info.read().await;
        discovered_info
            .values()
            .filter(|primal| primal.capabilities.contains(&capability.to_string()))
            .cloned()
            .collect()
    }

    /// Get the best provider for a specific capability
    pub async fn get_best_provider_for_capability(
        &self,
        capability: &str,
    ) -> Option<DiscoveredPrimal> {
        let providers = self.find_providers_by_capability(capability).await;

        // Score providers by last seen time and capability match
        providers.into_iter().max_by_key(|p| p.last_seen)
    }

    /// Register a security primal provider
    pub async fn register_security_provider(
        &self,
        name: String,
        provider: Arc<dyn SecurityPrimalProvider>,
    ) -> Result<()> {
        info!("Registering security provider: {}", name);
        let mut providers = self.security_providers.write().await;
        providers.insert(name, provider);
        Ok(())
    }

    /// Register an orchestration primal provider
    pub async fn register_orchestration_provider(
        &self,
        name: String,
        provider: Arc<dyn OrchestrationPrimalProvider>,
    ) -> Result<()> {
        info!("Registering orchestration provider: {}", name);
        let mut providers = self.orchestration_providers.write().await;
        providers.insert(name, provider);
        Ok(())
    }

    /// Register a compute primal provider
    pub async fn register_compute_provider(
        &self,
        name: String,
        provider: Arc<dyn ComputePrimalProvider>,
    ) -> Result<()> {
        info!("Registering compute provider: {}", name);
        let mut providers = self.compute_providers.write().await;
        providers.insert(name, provider);
        Ok(())
    }

    /// Get a security provider (any available security primal)
    pub async fn get_security_provider(&self) -> Option<Arc<dyn SecurityPrimalProvider>> {
        let providers = self.security_providers.read().await;
        providers.values().next().cloned()
    }

    /// Get an orchestration provider (any available orchestration primal)
    pub async fn get_orchestration_provider(&self) -> Option<Arc<dyn OrchestrationPrimalProvider>> {
        let providers = self.orchestration_providers.read().await;
        providers.values().next().cloned()
    }

    /// Get a compute provider (any available compute primal)
    pub async fn get_compute_provider(&self) -> Option<Arc<dyn ComputePrimalProvider>> {
        let providers = self.compute_providers.read().await;
        providers.values().next().cloned()
    }

    /// Execute a secure operation using any available security primal
    pub async fn execute_secure_operation<F, T>(&self, operation: F) -> Result<T>
    where
        F: Fn(
                Arc<dyn SecurityPrimalProvider>,
            )
                -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>>
            + Send,
        T: Send,
    {
        if let Some(provider) = self.get_security_provider().await {
            debug!("Executing secure operation with available security provider");
            operation(provider).await
        } else {
            match self.config.fallback_behavior {
                FallbackBehavior::Error => Err(NestGateError::Internal(
                    "No security provider available".to_string(),
                )),
                FallbackBehavior::NoOp => {
                    warn!("No security provider available, using fallback behavior");
                    // This would need to be implemented based on the specific operation
                    Err(NestGateError::Internal(
                        "No security provider available".to_string(),
                    ))
                }
                FallbackBehavior::Local => {
                    // Use local implementation
                    Err(NestGateError::Internal(
                        "Local security implementation not available".to_string(),
                    ))
                }
            }
        }
    }

    /// Execute an orchestration operation using any available orchestration primal
    pub async fn execute_orchestration_operation<F, T>(&self, operation: F) -> Result<T>
    where
        F: Fn(
                Arc<dyn OrchestrationPrimalProvider>,
            )
                -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>>
            + Send,
        T: Send,
    {
        if let Some(provider) = self.get_orchestration_provider().await {
            debug!("Executing orchestration operation with available orchestration provider");
            operation(provider).await
        } else {
            match self.config.fallback_behavior {
                FallbackBehavior::Error => Err(NestGateError::Internal(
                    "No orchestration provider available".to_string(),
                )),
                FallbackBehavior::NoOp => {
                    warn!("No orchestration provider available, using fallback behavior");
                    Err(NestGateError::Internal(
                        "No orchestration provider available".to_string(),
                    ))
                }
                FallbackBehavior::Local => {
                    // Use local implementation
                    Err(NestGateError::Internal(
                        "Local orchestration implementation not available".to_string(),
                    ))
                }
            }
        }
    }

    /// Execute a compute operation using any available compute primal
    pub async fn execute_compute_operation<F, T>(&self, operation: F) -> Result<T>
    where
        F: Fn(
                Arc<dyn ComputePrimalProvider>,
            )
                -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>>
            + Send,
        T: Send,
    {
        if let Some(provider) = self.get_compute_provider().await {
            debug!("Executing compute operation with available compute provider");
            operation(provider).await
        } else {
            match self.config.fallback_behavior {
                FallbackBehavior::Error => Err(NestGateError::Internal(
                    "No compute provider available".to_string(),
                )),
                FallbackBehavior::NoOp => {
                    warn!("No compute provider available, using fallback behavior");
                    Err(NestGateError::Internal(
                        "No compute provider available".to_string(),
                    ))
                }
                FallbackBehavior::Local => {
                    // Use local implementation
                    Err(NestGateError::Internal(
                        "Local compute implementation not available".to_string(),
                    ))
                }
            }
        }
    }

    /// Get adapter statistics
    pub async fn get_stats(&self) -> AdapterStats {
        let security_count = self.security_providers.read().await.len();
        let orchestration_count = self.orchestration_providers.read().await.len();
        let compute_count = self.compute_providers.read().await.len();
        let discovered_count = self.discovery_service.discovered_info.read().await.len();

        AdapterStats {
            security_providers: security_count,
            orchestration_providers: orchestration_count,
            compute_providers: compute_count,
            discovered_primals: discovered_count,
            discovery_enabled: self.config.auto_discovery,
            last_discovery: SystemTime::now(),
        }
    }
}

/// Statistics about the adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterStats {
    pub security_providers: usize,
    pub orchestration_providers: usize,
    pub compute_providers: usize,
    pub discovered_primals: usize,
    pub discovery_enabled: bool,
    pub last_discovery: SystemTime,
}

impl PrimalDiscoveryService {
    /// Create a new discovery service
    pub fn new(config: DiscoveryConfig) -> Self {
        Self {
            discovered_providers: Arc::new(RwLock::new(HashMap::new())),
            discovered_info: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Start the discovery process
    pub async fn start_discovery(&self) -> Result<()> {
        info!("Starting primal discovery service");
        self.discover_providers().await
    }

    /// Discover available primal providers
    pub async fn discover_providers(&self) -> Result<()> {
        debug!("Discovering primal providers");

        // Discover via environment variables
        self.discover_via_environment().await?;

        // Discover via service registry
        self.discover_via_service_registry().await?;

        // Discover via network scanning
        self.discover_via_network_scan().await?;

        let provider_count = self.discovered_providers.read().await.len();
        info!("Discovered {} primal providers", provider_count);

        Ok(())
    }

    /// Discover providers via environment variables
    async fn discover_via_environment(&self) -> Result<()> {
        debug!("Discovering providers via environment variables");

        let env_vars: Vec<(String, String)> = std::env::vars().collect();
        for (key, value) in env_vars {
            // Check for any patterns that might indicate a primal endpoint
            if self.matches_primal_pattern(&key) {
                debug!("Found potential primal endpoint: {}={}", key, value);

                // Try to determine primal type from key/value
                if let Some(primal_info) = self.extract_primal_info_from_env(&key, &value) {
                    let mut discovered_info = self.discovered_info.write().await;
                    discovered_info.insert(primal_info.primal_type.clone(), primal_info);
                }
            }
        }

        Ok(())
    }

    /// Check if an environment variable key matches primal patterns
    fn matches_primal_pattern(&self, key: &str) -> bool {
        self.config.env_patterns.iter().any(|pattern| {
            // Simple pattern matching - convert * to regex-like matching
            if pattern.contains('*') {
                let pattern_parts: Vec<&str> = pattern.split('*').collect();
                pattern_parts.iter().all(|part| key.contains(part))
            } else {
                key.contains(pattern)
            }
        })
    }

    /// Extract primal information from environment variable
    fn extract_primal_info_from_env(&self, key: &str, value: &str) -> Option<DiscoveredPrimal> {
        // Determine primal type from key patterns
        let primal_type = if key.contains("BEARDOG") || key.contains("SECURITY") {
            "security"
        } else if key.contains("SQUIRREL") || key.contains("AI") {
            "ai"
        } else if key.contains("SONGBIRD") || key.contains("ORCHESTRATION") {
            "orchestration"
        } else if key.contains("TOADSTOOL") || key.contains("COMPUTE") {
            "compute"
        } else {
            return None;
        };

        // Extract capabilities based on primal type
        let capabilities = match primal_type {
            "security" => vec![
                "encryption".to_string(),
                "authentication".to_string(),
                "access_control".to_string(),
            ],
            "ai" => vec![
                "model_inference".to_string(),
                "agent_framework".to_string(),
                "data_processing".to_string(),
            ],
            "orchestration" => vec![
                "service_discovery".to_string(),
                "load_balancing".to_string(),
                "routing".to_string(),
            ],
            "compute" => vec![
                "resource_allocation".to_string(),
                "container_runtime".to_string(),
                "scaling".to_string(),
            ],
            _ => vec![],
        };

        // Validate endpoint format
        if value.starts_with("http://") || value.starts_with("https://") {
            Some(DiscoveredPrimal {
                primal_type: primal_type.to_string(),
                capabilities,
                endpoint: value.to_string(),
                version: "unknown".to_string(),
                discovered_via: "environment".to_string(),
                last_seen: SystemTime::now(),
            })
        } else {
            None
        }
    }

    /// Discover providers via service registry
    async fn discover_via_service_registry(&self) -> Result<()> {
        debug!("Discovering providers via service registry");

        for endpoint in &self.config.registry_endpoints {
            debug!("Querying service registry: {}", endpoint);

            // Try to query the service registry
            if let Ok(response) =
                tokio::time::timeout(self.config.timeout, reqwest::get(endpoint)).await
            {
                if let Ok(resp) = response {
                    if resp.status().is_success() {
                        debug!("Successfully queried service registry: {}", endpoint);
                        // In a full implementation, parse the response and extract primal info
                    }
                }
            }
        }

        Ok(())
    }

    /// Discover providers via network scan
    async fn discover_via_network_scan(&self) -> Result<()> {
        debug!("Discovering providers via network scanning");

        for range in &self.config.network_ranges {
            debug!("Scanning network range: {}", range);

            // For localhost scanning, check common ports
            if range.starts_with("127.0.0.1") || range.starts_with("localhost") {
                for port in &self.config.common_ports {
                    let addr = format!("127.0.0.1:{}", port);
                    if self.check_endpoint_responsive(&addr).await {
                        debug!("Found responsive endpoint: {}", addr);

                        // Try to determine what type of primal this might be
                        if let Some(primal_info) = self.probe_primal_type(&addr).await {
                            let mut discovered_info = self.discovered_info.write().await;
                            discovered_info.insert(
                                format!("{}:{}", primal_info.primal_type, port),
                                primal_info,
                            );
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Check if an endpoint is responsive
    async fn check_endpoint_responsive(&self, addr: &str) -> bool {
        if let Ok(socket_addr) = addr.parse::<SocketAddr>() {
            tokio::time::timeout(self.config.timeout, TcpStream::connect(socket_addr))
                .await
                .is_ok()
        } else {
            false
        }
    }

    /// Probe an endpoint to determine primal type
    async fn probe_primal_type(&self, addr: &str) -> Option<DiscoveredPrimal> {
        let endpoints_to_try = vec![
            format!("http://{}/health", addr),
            format!("http://{}/api/v1/health", addr),
            format!("http://{}/status", addr),
        ];

        for endpoint in endpoints_to_try {
            if let Ok(Ok(response)) =
                tokio::time::timeout(self.config.timeout, reqwest::get(&endpoint)).await
            {
                if response.status().is_success() {
                    // Try to determine primal type from response headers or content
                    let primal_type = self.guess_primal_type_from_response(&response, addr).await;

                    if let Some(ptype) = primal_type {
                        let capabilities = match ptype.as_str() {
                            "security" => {
                                vec!["encryption".to_string(), "authentication".to_string()]
                            }
                            "ai" => {
                                vec!["model_inference".to_string(), "agent_framework".to_string()]
                            }
                            "orchestration" => vec![
                                "service_discovery".to_string(),
                                "load_balancing".to_string(),
                            ],
                            "compute" => vec![
                                "resource_allocation".to_string(),
                                "container_runtime".to_string(),
                            ],
                            _ => vec!["unknown".to_string()],
                        };

                        return Some(DiscoveredPrimal {
                            primal_type: ptype,
                            capabilities,
                            endpoint: format!("http://{}", addr),
                            version: "unknown".to_string(),
                            discovered_via: "network_scan".to_string(),
                            last_seen: SystemTime::now(),
                        });
                    }
                }
            }
        }

        None
    }

    /// Guess primal type from HTTP response
    async fn guess_primal_type_from_response(
        &self,
        response: &reqwest::Response,
        addr: &str,
    ) -> Option<String> {
        // Check response headers for primal identification
        if let Some(server_header) = response.headers().get("server") {
            if let Ok(server_value) = server_header.to_str() {
                if server_value.to_lowercase().contains("beardog") {
                    return Some("security".to_string());
                } else if server_value.to_lowercase().contains("squirrel") {
                    return Some("ai".to_string());
                } else if server_value.to_lowercase().contains("songbird") {
                    return Some("orchestration".to_string());
                } else if server_value.to_lowercase().contains("toadstool") {
                    return Some("compute".to_string());
                }
            }
        }

        // Guess based on port number
        if let Ok(port) = addr.split(':').nth(1).unwrap_or("").parse::<u16>() {
            match port {
                8443 | 9002 => Some("security".to_string()), // Common BearDog ports
                8080 | 9001 => Some("ai".to_string()),       // Common Squirrel ports
                8000 | 9003 => Some("orchestration".to_string()), // Common Songbird ports
                8081 | 9000 => Some("compute".to_string()),  // Common ToadStool ports
                _ => None,
            }
        } else {
            None
        }
    }

    /// Get discovered providers
    pub async fn get_discovered_providers(&self) -> HashMap<String, Arc<dyn PrimalProvider>> {
        self.discovered_providers.read().await.clone()
    }

    /// Get discovered primal information
    pub async fn get_discovered_info(&self) -> HashMap<String, DiscoveredPrimal> {
        self.discovered_info.read().await.clone()
    }
}

/// Create a default universal adapter for NestGate
pub fn create_default_adapter() -> UniversalPrimalAdapter {
    let config = UniversalAdapterConfig::default();
    UniversalPrimalAdapter::new(config)
}

/// Create a universal adapter with custom configuration
pub fn create_adapter_with_config(config: UniversalAdapterConfig) -> UniversalPrimalAdapter {
    UniversalPrimalAdapter::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_adapter_creation() {
        let adapter = create_default_adapter();
        let stats = adapter.get_stats().await;

        assert_eq!(stats.security_providers, 0);
        assert_eq!(stats.orchestration_providers, 0);
        assert_eq!(stats.compute_providers, 0);
        assert!(stats.discovery_enabled);
    }

    #[tokio::test]
    async fn test_adapter_initialization() {
        let adapter = create_default_adapter();
        let result = adapter.initialize().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_capability_based_discovery() {
        let adapter = create_default_adapter();

        // Test finding providers by capability
        let providers = adapter.find_providers_by_capability("encryption").await;
        // Initially empty, but the method should work
        assert!(providers.is_empty());
    }

    #[tokio::test]
    async fn test_environment_pattern_matching() {
        let discovery = PrimalDiscoveryService::new(DiscoveryConfig::default());

        assert!(discovery.matches_primal_pattern("BEARDOG_ENDPOINT"));
        assert!(discovery.matches_primal_pattern("SQUIRREL_MCP_URL"));
        assert!(discovery.matches_primal_pattern("NESTGATE_SECURITY_PROVIDER"));
        assert!(!discovery.matches_primal_pattern("RANDOM_VAR"));
    }
}
