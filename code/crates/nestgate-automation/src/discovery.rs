//! Ecosystem Capability Discovery
//!
//! ✅ **MODERNIZED**: Dynamic ecosystem capability discovery for universal service integration
//! ❌ **DEPRECATED**: Legacy primal-specific discovery methods

use crate::types::CapabilityProvider;
use nestgate_core::error::NestGateError;
use std::time::Duration;
use tracing::{debug, info, warn};

/// Ecosystem capability discovery configuration
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// Discovery timeout in seconds
    pub timeout: u64,
    /// Enable mDNS discovery
    pub enable_mdns: bool,
    /// Enable network scanning
    pub enable_network_scan: bool,
    /// Network scan subnets
    pub scan_subnets: Vec<String>,
    /// Discovery endpoints
    pub discovery_endpoints: Vec<String>,
    /// Discovery interval in seconds
    pub discovery_interval: u64,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            timeout: 30,
            enable_mdns: true,
            enable_network_scan: true,
            scan_subnets: vec![
                "192.168.1.0/24".to_string(),
                "10.0.0.0/24".to_string(),
                "172.16.0.0/24".to_string(),
            ],
            discovery_endpoints: vec![std::env::var("ECOSYSTEM_DISCOVERY_URL")
                .unwrap_or_else(|_| "http://localhost:8080/api/v1/discovery".to_string())],
            discovery_interval: 300, // 5 minutes
        }
    }
}

/// Universal capability discovery service
#[derive(Debug)]
pub struct EcosystemDiscovery {
    config: DiscoveryConfig,
    discovered_capabilities:
        tokio::sync::RwLock<std::collections::HashMap<String, CapabilityProvider>>,
    client: reqwest::Client,
}

impl EcosystemDiscovery {
    /// Create new ecosystem discovery service
    pub fn new() -> crate::Result<Self> {
        let config = DiscoveryConfig::default();
        Ok(Self {
            config,
            discovered_capabilities: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .map_err(|e| NestGateError::network_error(&e.to_string(), "discovery", None))?,
        })
    }

    /// Create with custom configuration
    pub fn with_config(config: DiscoveryConfig) -> crate::Result<Self> {
        Ok(Self {
            config,
            discovered_capabilities: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .map_err(|e| NestGateError::network_error(&e.to_string(), "discovery", None))?,
        })
    }

    /// Discover available capabilities in the ecosystem
    pub async fn discover_capabilities(&self) -> Result<Vec<CapabilityProvider>, NestGateError> {
        info!("🔍 Starting ecosystem capability discovery");

        let mut discovered = Vec::new();

        // Discovery via configured endpoints
        if let Ok(providers) = self.discover_via_endpoints().await {
            discovered.extend(providers);
        }

        // Discovery via network scanning (if enabled)
        if self.config.enable_network_scan {
            if let Ok(providers) = self.discover_via_network_scan().await {
                discovered.extend(providers);
            }
        }

        // Discovery via mDNS (if enabled)
        if self.config.enable_mdns {
            if let Ok(providers) = self.discover_via_mdns().await {
                discovered.extend(providers);
            }
        }

        // Update discovered capabilities cache
        {
            let mut cache = self.discovered_capabilities.write().await;
            cache.clear();
            for provider in &discovered {
                cache.insert(provider.service_id.to_string(), provider.clone());
            }
        }

        info!("✅ Discovered {} capability providers", discovered.len());
        Ok(discovered)
    }

    /// Get cached discovered capabilities
    pub async fn get_cached_capabilities(&self) -> Vec<CapabilityProvider> {
        let cache = self.discovered_capabilities.read().await;
        cache.values().cloned().collect()
    }

    /// Discover capabilities via configured endpoints
    async fn discover_via_endpoints(&self) -> Result<Vec<CapabilityProvider>, NestGateError> {
        let mut providers = Vec::new();

        for endpoint in &self.config.discovery_endpoints {
            debug!("🌐 Querying discovery endpoint: {}", endpoint);

            match self.client.get(endpoint).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<Vec<CapabilityProvider>>().await {
                            Ok(discovered_providers) => {
                                info!(
                                    "✅ Found {} providers from {}",
                                    discovered_providers.len(),
                                    endpoint
                                );
                                providers.extend(discovered_providers);
                            }
                            Err(e) => {
                                warn!("⚠️ Failed to parse providers from {}: {}", endpoint, e);
                            }
                        }
                    }
                }
                Err(e) => {
                    debug!("❌ Failed to query {}: {}", endpoint, e);
                }
            }
        }

        Ok(providers)
    }

    /// Discover capabilities via network scanning
    async fn discover_via_network_scan(&self) -> Result<Vec<CapabilityProvider>, NestGateError> {
        debug!("🔍 Starting network capability scan");
        let mut providers = Vec::new();

        for subnet in &self.config.scan_subnets {
            if let Ok(subnet_providers) = self.scan_subnet(subnet).await {
                providers.extend(subnet_providers);
            }
        }

        Ok(providers)
    }

    /// Scan a specific subnet for capabilities
    async fn scan_subnet(&self, subnet: &str) -> Result<Vec<CapabilityProvider>, NestGateError> {
        debug!("🔍 Scanning subnet: {}", subnet);
        let mut providers = Vec::new();

        // Parse subnet and scan common ports
        let common_ports = vec![8080, 8081, 8082, 8000, 3000, 5000];

        // Simplified subnet scanning - in practice, you'd parse the CIDR
        let base_ip = if subnet.starts_with("192.168.1") {
            "192.168.1"
        } else if subnet.starts_with("10.0.0") {
            "10.0.0"
        } else if subnet.starts_with("172.16.0") {
            "172.16.0"
        } else {
            return Ok(providers);
        };

        // Scan a few common host addresses
        for host in 1..10 {
            for port in &common_ports {
                let endpoint = format!("http://{}.{}:{}/api/v1/capabilities", base_ip, host, port);

                if let Ok(capability_providers) = self.query_capability_endpoint(&endpoint).await {
                    providers.extend(capability_providers);
                }
            }
        }

        Ok(providers)
    }

    /// Query a specific endpoint for capabilities
    async fn query_capability_endpoint(
        &self,
        endpoint: &str,
    ) -> crate::Result<Vec<CapabilityProvider>> {
        match tokio::time::timeout(Duration::from_secs(2), self.client.get(endpoint).send()).await {
            Ok(Ok(response)) => {
                if response.status().is_success() {
                    // Get response text first to avoid move issues
                    match response.text().await {
                        Ok(text) => {
                            // Try parsing as Vec<CapabilityProvider> first
                            if let Ok(providers) =
                                serde_json::from_str::<Vec<CapabilityProvider>>(&text)
                            {
                                Ok(providers)
                            } else if let Ok(provider) =
                                serde_json::from_str::<CapabilityProvider>(&text)
                            {
                                Ok(vec![provider])
                            } else {
                                Ok(vec![])
                            }
                        }
                        Err(_) => Ok(vec![]),
                    }
                } else {
                    Ok(vec![])
                }
            }
            _ => Ok(vec![]),
        }
    }

    /// Discover capabilities via mDNS
    async fn discover_via_mdns(&self) -> Result<Vec<CapabilityProvider>, NestGateError> {
        debug!("🔍 Starting mDNS capability discovery");

        // Placeholder for mDNS implementation
        // In practice, you would use mdns crate to discover services

        Ok(vec![])
    }
}
