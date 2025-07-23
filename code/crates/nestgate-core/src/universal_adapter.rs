// Removed unused error imports
/// Universal Primal Adapter
///
/// This module provides the universal adapter that coordinates between different
/// primal providers and handles auto-discovery without hardcoding specific implementations.
use std::collections::HashMap;
// Removed unused std import
use std::sync::Arc;
use std::time::SystemTime;
// Removed unused std import
// Removed unused tracing import

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::universal_traits::*;
use crate::{NestGateError, Result};
use std::time::Duration;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::warn;

/// Static capability constants for zero-copy string operations
mod capability_constants {
    // Security capabilities

    // AI capabilities

    // Orchestration capabilities

    // Compute capabilities
}

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

impl PrimalDiscoveryService {
    /// Create a new primal discovery service
    pub fn new(_config: DiscoveryConfig) -> Self {
        Self {
            discovered_providers: Arc::new(RwLock::new(HashMap::new())),
            discovered_info: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Start the discovery service
    pub async fn start_discovery(&self) -> crate::Result<()> {
        // Removed unused tracing import
        info!("🚀 Starting primal discovery service");

        // Initial discovery scan
        self.discover_providers().await?;

        Ok(())
    }

    /// Discover available primal providers
    pub async fn discover_providers(&self) -> crate::Result<()> {
        // Removed unused tracing import

        info!("🔍 Scanning for primal providers...");

        // Environment-based discovery
        self.discover_from_environment().await;

        // Network-based discovery
        self.discover_from_network().await;

        // Registry-based discovery
        self.discover_from_registries().await;

        let provider_count = self.discovered_providers.read().await.len();
        info!("✅ Discovery complete: {} providers found", provider_count);

        Ok(())
    }

    /// Discover providers from environment variables
    async fn discover_from_environment(&self) {
        // Removed unused tracing import
        debug!("Scanning environment variables for primal providers");
        // Implementation would scan env vars matching patterns
        // For now, this is a stub that can be expanded
    }

    /// Discover providers from network scanning
    async fn discover_from_network(&self) {
        // Removed unused tracing import
        debug!("Scanning network ranges for primal providers");
        // Implementation would scan network ranges and common ports
        // For now, this is a stub that can be expanded
    }

    /// Discover providers from service registries
    async fn discover_from_registries(&self) {
        // Removed unused tracing import
        debug!("Querying service registries for primal providers");
        // Implementation would query Consul, etcd, etc.
        // For now, this is a stub that can be expanded
    }
}

impl UniversalPrimalAdapter {
    /// Get adapter statistics
    pub async fn get_stats(&self) -> AdapterStats {
        AdapterStats {
            security_providers: self.security_providers.read().await.len(),
            orchestration_providers: self.orchestration_providers.read().await.len(),
            compute_providers: self.compute_providers.read().await.len(),
            discovery_attempts: get_discovery_attempts(),
            successful_discoveries: get_successful_discoveries(),
            last_discovery: get_last_discovery_time(),
        }
    }

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
                FallbackBehavior::Error => Err(NestGateError::Internal {
                    message: "No security provider available".to_string(),
                    location: Some(file!().to_string()),
                    debug_info: None,
                    is_bug: false,
                }),
                FallbackBehavior::NoOp => {
                    warn!("No security provider available, using fallback behavior");
                    // Return appropriate no-op response
                    Err(NestGateError::Internal {
                        message: "No security provider available".to_string(),
                        location: Some(file!().to_string()),
                        debug_info: None,
                        is_bug: false,
                    })
                }
                FallbackBehavior::Local => {
                    // Use local implementation
                    Err(NestGateError::Internal {
                        message: "Local security implementation not available".to_string(),
                        location: Some(file!().to_string()),
                        debug_info: None,
                        is_bug: false,
                    })
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
                FallbackBehavior::Error => Err(NestGateError::Internal {
                    message: "No orchestration provider available".to_string(),
                    location: Some(file!().to_string()),
                    debug_info: None,
                    is_bug: false,
                }),
                FallbackBehavior::NoOp => {
                    warn!("No orchestration provider available, using fallback behavior");
                    Err(NestGateError::Internal {
                        message: "No orchestration provider available".to_string(),
                        location: Some(file!().to_string()),
                        debug_info: None,
                        is_bug: false,
                    })
                }
                FallbackBehavior::Local => Err(NestGateError::Internal {
                    message: "Local orchestration implementation not available".to_string(),
                    location: Some(file!().to_string()),
                    debug_info: None,
                    is_bug: false,
                }),
            }
        }
    }
}

/// Statistics for universal adapter performance and discovery
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdapterStats {
    /// Number of discovered security providers
    pub security_providers: usize,
    /// Number of discovered orchestration providers  
    pub orchestration_providers: usize,
    /// Number of discovered compute providers
    pub compute_providers: usize,
    /// Total discovery attempts
    pub discovery_attempts: u64,
    /// Successful discoveries
    pub successful_discoveries: u64,
    /// Last discovery time
    pub last_discovery: Option<std::time::SystemTime>,
}

/// Create a universal primal adapter with default configuration
pub fn create_default_adapter() -> UniversalPrimalAdapter {
    UniversalPrimalAdapter::new(UniversalAdapterConfig::default())
}

/// Create a universal primal adapter with custom configuration
pub fn create_adapter_with_config(config: UniversalAdapterConfig) -> UniversalPrimalAdapter {
    UniversalPrimalAdapter::new(config)
}

/// Get total discovery attempts count
fn get_discovery_attempts() -> u64 {
    static DISCOVERY_ATTEMPTS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    DISCOVERY_ATTEMPTS.load(std::sync::atomic::Ordering::Relaxed)
}

/// Get successful discoveries count
fn get_successful_discoveries() -> u64 {
    static SUCCESSFUL_DISCOVERIES: std::sync::atomic::AtomicU64 =
        std::sync::atomic::AtomicU64::new(0);
    SUCCESSFUL_DISCOVERIES.load(std::sync::atomic::Ordering::Relaxed)
}

/// Get last discovery time
fn get_last_discovery_time() -> Option<std::time::SystemTime> {
    static LAST_DISCOVERY: std::sync::OnceLock<std::sync::Mutex<Option<std::time::SystemTime>>> =
        std::sync::OnceLock::new();
    let mutex = LAST_DISCOVERY.get_or_init(|| std::sync::Mutex::new(None));
    mutex.lock().map(|guard| guard.clone()).unwrap_or(None)
}

/// Increment discovery attempts counter
pub fn increment_discovery_attempts() {
    static DISCOVERY_ATTEMPTS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    DISCOVERY_ATTEMPTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

/// Increment successful discoveries counter and update last discovery time
pub fn increment_successful_discoveries() {
    static SUCCESSFUL_DISCOVERIES: std::sync::atomic::AtomicU64 =
        std::sync::atomic::AtomicU64::new(0);
    SUCCESSFUL_DISCOVERIES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    // Update last discovery time
    static LAST_DISCOVERY: std::sync::OnceLock<std::sync::Mutex<Option<std::time::SystemTime>>> =
        std::sync::OnceLock::new();
    let mutex = LAST_DISCOVERY.get_or_init(|| std::sync::Mutex::new(None));
    if let Ok(mut last_discovery) = mutex.lock() {
        *last_discovery = Some(std::time::SystemTime::now());
    }
}
