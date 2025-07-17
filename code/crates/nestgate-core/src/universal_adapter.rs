//! Universal Primal Adapter
//!
//! This module provides the universal adapter that coordinates between different
//! primal providers and handles auto-discovery without hardcoding specific implementations.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use crate::universal_traits::*;
use crate::{NestGateError, Result};

/// Universal Primal Adapter that coordinates between different primal providers
pub struct UniversalPrimalAdapter {
    /// Security primal providers (any security implementation)
    security_providers: Arc<RwLock<HashMap<String, Arc<dyn SecurityPrimalProvider>>>>,
    /// AI primal providers (any AI implementation)
    ai_providers: Arc<RwLock<HashMap<String, Arc<dyn AiPrimalProvider>>>>,
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

/// Primal discovery service
pub struct PrimalDiscoveryService {
    /// Discovered primal providers
    discovered_providers: Arc<RwLock<HashMap<String, Arc<dyn PrimalProvider>>>>,
    /// Discovery configuration
    config: DiscoveryConfig,
}

/// Configuration for primal discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Environment variable prefixes to scan
    pub env_prefixes: Vec<String>,
    /// Service registry endpoints
    pub registry_endpoints: Vec<String>,
    /// Network ranges to scan
    pub network_ranges: Vec<String>,
    /// Discovery timeout
    pub timeout: Duration,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            env_prefixes: vec![
                "NESTGATE_SECURITY_".to_string(),
                "NESTGATE_AI_".to_string(),
                "NESTGATE_ORCHESTRATION_".to_string(),
                "NESTGATE_COMPUTE_".to_string(),
            ],
            registry_endpoints: vec![
                "http://localhost:8500".to_string(), // Consul
                "http://localhost:2379".to_string(), // etcd
            ],
            network_ranges: vec![
                "127.0.0.1/32".to_string(),
                "10.0.0.0/8".to_string(),
                "172.16.0.0/12".to_string(),
                "192.168.0.0/16".to_string(),
            ],
            timeout: Duration::from_secs(10),
        }
    }
}

impl UniversalPrimalAdapter {
    /// Create a new universal primal adapter
    pub fn new(config: UniversalAdapterConfig) -> Self {
        let discovery_service = Arc::new(PrimalDiscoveryService::new(DiscoveryConfig::default()));
        
        Self {
            security_providers: Arc::new(RwLock::new(HashMap::new())),
            ai_providers: Arc::new(RwLock::new(HashMap::new())),
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

    /// Register an AI primal provider
    pub async fn register_ai_provider(
        &self,
        name: String,
        provider: Arc<dyn AiPrimalProvider>,
    ) -> Result<()> {
        info!("Registering AI provider: {}", name);
        let mut providers = self.ai_providers.write().await;
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

    /// Get an AI provider (any available AI primal)
    pub async fn get_ai_provider(&self) -> Option<Arc<dyn AiPrimalProvider>> {
        let providers = self.ai_providers.read().await;
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
        F: Fn(Arc<dyn SecurityPrimalProvider>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>> + Send,
        T: Send,
    {
        if let Some(provider) = self.get_security_provider().await {
            debug!("Executing secure operation with available security provider");
            operation(provider).await
        } else {
            match self.config.fallback_behavior {
                FallbackBehavior::Error => {
                    Err(NestGateError::Internal("No security provider available".to_string()))
                }
                FallbackBehavior::NoOp => {
                    warn!("No security provider available, using fallback behavior");
                    // This would need to be implemented based on the specific operation
                    Err(NestGateError::Internal("No security provider available".to_string()))
                }
                FallbackBehavior::Local => {
                    // Use local implementation
                    Err(NestGateError::Internal("Local security implementation not available".to_string()))
                }
            }
        }
    }

    /// Execute an AI operation using any available AI primal
    pub async fn execute_ai_operation<F, T>(&self, operation: F) -> Result<T>
    where
        F: Fn(Arc<dyn AiPrimalProvider>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>> + Send,
        T: Send,
    {
        if let Some(provider) = self.get_ai_provider().await {
            debug!("Executing AI operation with available AI provider");
            operation(provider).await
        } else {
            match self.config.fallback_behavior {
                FallbackBehavior::Error => {
                    Err(NestGateError::Internal("No AI provider available".to_string()))
                }
                FallbackBehavior::NoOp => {
                    warn!("No AI provider available, using fallback behavior");
                    // Return a default/fallback result
                    Err(NestGateError::Internal("No AI provider available".to_string()))
                }
                FallbackBehavior::Local => {
                    // Use local implementation
                    Err(NestGateError::Internal("Local AI implementation not available".to_string()))
                }
            }
        }
    }

    /// Execute an orchestration operation using any available orchestration primal
    pub async fn execute_orchestration_operation<F, T>(&self, operation: F) -> Result<T>
    where
        F: Fn(Arc<dyn OrchestrationPrimalProvider>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>> + Send,
        T: Send,
    {
        if let Some(provider) = self.get_orchestration_provider().await {
            debug!("Executing orchestration operation with available orchestration provider");
            operation(provider).await
        } else {
            match self.config.fallback_behavior {
                FallbackBehavior::Error => {
                    Err(NestGateError::Internal("No orchestration provider available".to_string()))
                }
                FallbackBehavior::NoOp => {
                    warn!("No orchestration provider available, using fallback behavior");
                    Err(NestGateError::Internal("No orchestration provider available".to_string()))
                }
                FallbackBehavior::Local => {
                    // Use local implementation
                    Err(NestGateError::Internal("Local orchestration implementation not available".to_string()))
                }
            }
        }
    }

    /// Execute a compute operation using any available compute primal
    pub async fn execute_compute_operation<F, T>(&self, operation: F) -> Result<T>
    where
        F: Fn(Arc<dyn ComputePrimalProvider>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>> + Send,
        T: Send,
    {
        if let Some(provider) = self.get_compute_provider().await {
            debug!("Executing compute operation with available compute provider");
            operation(provider).await
        } else {
            match self.config.fallback_behavior {
                FallbackBehavior::Error => {
                    Err(NestGateError::Internal("No compute provider available".to_string()))
                }
                FallbackBehavior::NoOp => {
                    warn!("No compute provider available, using fallback behavior");
                    Err(NestGateError::Internal("No compute provider available".to_string()))
                }
                FallbackBehavior::Local => {
                    // Use local implementation
                    Err(NestGateError::Internal("Local compute implementation not available".to_string()))
                }
            }
        }
    }

    /// Get adapter statistics
    pub async fn get_stats(&self) -> AdapterStats {
        let security_count = self.security_providers.read().await.len();
        let ai_count = self.ai_providers.read().await.len();
        let orchestration_count = self.orchestration_providers.read().await.len();
        let compute_count = self.compute_providers.read().await.len();
        
        AdapterStats {
            security_providers: security_count,
            ai_providers: ai_count,
            orchestration_providers: orchestration_count,
            compute_providers: compute_count,
            discovery_enabled: self.config.auto_discovery,
            last_discovery: SystemTime::now(),
        }
    }
}

/// Statistics about the adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterStats {
    pub security_providers: usize,
    pub ai_providers: usize,
    pub orchestration_providers: usize,
    pub compute_providers: usize,
    pub discovery_enabled: bool,
    pub last_discovery: SystemTime,
}

impl PrimalDiscoveryService {
    /// Create a new discovery service
    pub fn new(config: DiscoveryConfig) -> Self {
        Self {
            discovered_providers: Arc::new(RwLock::new(HashMap::new())),
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
        
        for prefix in &self.config.env_prefixes {
            for (key, value) in std::env::vars() {
                if key.starts_with(prefix) && key.ends_with("_ENDPOINT") {
                    let provider_name = key
                        .strip_prefix(prefix)
                        .unwrap_or(&key)
                        .strip_suffix("_ENDPOINT")
                        .unwrap_or(&key)
                        .to_lowercase();
                    
                    info!("Found potential provider '{}' at endpoint: {}", provider_name, value);
                    
                    // Here you would create a client for the discovered provider
                    // For now, we'll just log it
                }
            }
        }
        
        Ok(())
    }

    /// Discover providers via service registry
    async fn discover_via_service_registry(&self) -> Result<()> {
        debug!("Discovering providers via service registry");
        
        for endpoint in &self.config.registry_endpoints {
            // Here you would query the service registry
            // For now, we'll just log it
            debug!("Checking service registry at: {}", endpoint);
        }
        
        Ok(())
    }

    /// Discover providers via network scanning
    async fn discover_via_network_scan(&self) -> Result<()> {
        debug!("Discovering providers via network scanning");
        
        for range in &self.config.network_ranges {
            // Here you would scan the network range
            // For now, we'll just log it
            debug!("Scanning network range: {}", range);
        }
        
        Ok(())
    }

    /// Get discovered providers
    pub async fn get_discovered_providers(&self) -> HashMap<String, Arc<dyn PrimalProvider>> {
        self.discovered_providers.read().await.clone()
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
        assert_eq!(stats.ai_providers, 0);
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
} 