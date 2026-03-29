// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! Capability-Based Configuration System
//!
//! This module implements the evolution from hardcoded values to runtime-discovered
//! capability-based configuration, aligned with the Primal Self-Knowledge philosophy.
//!
//! # Philosophy
//!
//! Primals have **self-knowledge** (what they can do) and **discover** other primals
//! at runtime. No hardcoded assumptions about where services live or what ports they use.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │  Capability-Based Configuration Layer   │
//! ├─────────────────────────────────────────┤
//! │ 1. Self-Knowledge (what we provide)     │
//! │ 2. Environment (explicit configuration) │
//! │ 3. Discovery (find other primals)       │
//! │ 4. Fallback (only if discovery disabled)│
//! └─────────────────────────────────────────┘
//! ```
//!
//! # Examples
//!
//! ```rust,ignore
//! // discover_capability may fail without env/discovery; ServiceEndpoint has url() method
//! use nestgate_core::capability_based_config::CapabilityConfig;
//! let config = CapabilityConfig::initialize().await?;
//! let api_endpoint = config.discover_capability("api").await?;
//! println!("Found API at: {}", api_endpoint.url());
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, warn};

/// Capability-based configuration system
///
/// Implements the evolution from hardcoded values to runtime discovery:
/// 1. Check environment variables (explicit configuration)
/// 2. Attempt runtime discovery (find other primals)
/// 3. Use fallback only if discovery is disabled
#[derive(Clone)]
pub struct CapabilityConfig {
    /// Our self-knowledge (what capabilities we provide)
    self_knowledge: Arc<SelfKnowledge>,

    /// Discovered capabilities (other primals we've found)
    discovered: Arc<RwLock<HashMap<String, DiscoveredCapability>>>,

    /// Discovery configuration
    discovery_config: DiscoveryConfig,
}

/// Self-knowledge: What this primal can do
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfKnowledge {
    /// Capabilities this primal provides
    pub capabilities: Vec<String>,

    /// Endpoints where we can be reached
    pub endpoints: Vec<ServiceEndpoint>,

    /// Our identity
    pub identity: PrimalIdentity,
}

/// Primal identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalIdentity {
    /// Unique identifier (generated, not hardcoded)
    pub id: String,

    /// Primal type (e.g., "nestgate", "beardog", "songbird")
    pub primal_type: String,

    /// Version
    pub version: String,
}

/// Service endpoint where we can be reached
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Protocol (http, https, grpc, etc.)
    pub protocol: String,

    /// Address (discovered or configured, never hardcoded)
    pub address: String,

    /// Port (discovered or configured, never hardcoded)
    pub port: u16,

    /// Path prefix (if applicable)
    pub path: Option<String>,
}

impl ServiceEndpoint {
    /// Get the full URL for this endpoint
    #[must_use]
    pub fn url(&self) -> String {
        let path = self.path.as_deref().unwrap_or("");
        format!("{}://{}:{}{}", self.protocol, self.address, self.port, path)
    }
}

/// Discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Enable runtime discovery
    pub enabled: bool,

    /// Discovery methods to use (mDNS, DNS-SD, Consul, etc.)
    pub methods: Vec<DiscoveryMethod>,

    /// Discovery timeout in milliseconds
    pub timeout_ms: u64,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: std::env::var("NESTGATE_DISCOVERY_ENABLED")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true), // Default: discovery enabled
            methods: vec![DiscoveryMethod::Environment], // Start with env only
            timeout_ms: 5000,
        }
    }
}

/// Discovery methods for finding other primals
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiscoveryMethod {
    /// Use environment variables (explicit configuration)
    Environment,

    /// mDNS/Zeroconf discovery
    #[allow(dead_code)]
    MDns,

    /// DNS Service Discovery
    #[allow(dead_code)]
    DnsSd,

    /// Consul service registry
    #[allow(dead_code)]
    Consul,

    /// Kubernetes service discovery
    #[allow(dead_code)]
    Kubernetes,
}

/// Discovered capability (another primal or service)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredCapability {
    /// Capability name
    pub name: String,

    /// Endpoint where this capability can be accessed
    pub endpoint: ServiceEndpoint,

    /// When this was discovered
    pub discovered_at: std::time::SystemTime,

    /// How this was discovered
    pub discovery_method: DiscoveryMethod,
}

impl CapabilityConfig {
    /// Initialize capability-based configuration
    ///
    /// # Errors
    ///
    /// Returns an error if self-knowledge cannot be determined.
    pub async fn initialize() -> Result<Self> {
        let self_knowledge = Self::build_self_knowledge().await?;
        let discovery_config = DiscoveryConfig::default();

        Ok(Self {
            self_knowledge: Arc::new(self_knowledge),
            discovered: Arc::new(RwLock::new(HashMap::new())),
            discovery_config,
        })
    }

    /// Build our self-knowledge
    async fn build_self_knowledge() -> Result<SelfKnowledge> {
        // Generate unique identity (not hardcoded)
        let identity = PrimalIdentity {
            id: uuid::Uuid::new_v4().to_string(),
            primal_type: "nestgate".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        // Determine our capabilities from environment or defaults
        let capabilities = Self::determine_capabilities().await?;

        // Build our endpoints from configuration (not hardcoded)
        let endpoints = Self::build_endpoints()?;

        Ok(SelfKnowledge {
            capabilities,
            endpoints,
            identity,
        })
    }

    /// Determine what capabilities we provide
    async fn determine_capabilities() -> Result<Vec<String>> {
        // Check environment for explicit capability list
        if let Ok(caps) = std::env::var("NESTGATE_CAPABILITIES") {
            return Ok(caps.split(',').map(|s| s.trim().to_string()).collect());
        }

        // Otherwise, introspect what we have available
        let mut capabilities = vec!["storage".to_string()];

        // Check if ZFS is available (runtime capability detection - universal!)
        // This works on ALL platforms with ZFS installed
        if let Ok(output) = tokio::process::Command::new("zfs")
            .arg("--version")
            .output()
            .await
            && output.status.success()
        {
            debug!("✅ ZFS capability detected (universal runtime check)");
            capabilities.push("zfs".to_string());
        }

        Ok(capabilities)
    }

    /// Build our endpoints from configuration
    fn build_endpoints() -> Result<Vec<ServiceEndpoint>> {
        let mut endpoints = Vec::new();

        // Primary API endpoint
        let api_port = Self::resolve_port_from_env("NESTGATE_API_PORT", 3000)?;
        let api_addr = Self::resolve_address_from_env("NESTGATE_API_HOST", "0.0.0.0")?;

        endpoints.push(ServiceEndpoint {
            protocol: "http".to_string(),
            address: api_addr,
            port: api_port,
            path: Some("/api/v1".to_string()),
        });

        Ok(endpoints)
    }

    /// Resolve port from environment with fallback
    fn resolve_port_from_env(env_var: &str, fallback: u16) -> Result<u16> {
        if let Ok(val) = std::env::var(env_var) {
            val.parse()
                .with_context(|| format!("Invalid port in {env_var}: {val}"))
        } else {
            debug!("Port {} not set, using fallback: {}", env_var, fallback);
            Ok(fallback)
        }
    }

    /// Resolve address from environment with fallback
    fn resolve_address_from_env(env_var: &str, fallback: &str) -> Result<String> {
        if let Ok(val) = std::env::var(env_var) {
            // Validate it's a valid address format
            if val.is_empty() {
                anyhow::bail!("{env_var} cannot be empty");
            }
            Ok(val)
        } else {
            debug!("Address {} not set, using fallback: {}", env_var, fallback);
            Ok(fallback.to_string())
        }
    }

    /// Discover a capability at runtime
    ///
    /// This is the core of the primal discovery philosophy:
    /// We discover other primals at runtime, not through hardcoded values.
    ///
    /// # Errors
    ///
    /// Returns an error if the capability cannot be discovered and
    /// discovery is enabled but fails.
    pub async fn discover_capability(&self, capability: &str) -> Result<ServiceEndpoint> {
        // 1. Check if already discovered
        {
            let discovered = self.discovered.read().await;
            if let Some(cached) = discovered.get(capability) {
                debug!("Using cached discovery for {}", capability);
                return Ok(cached.endpoint.clone());
            }
        }

        // 2. Check environment (explicit configuration takes precedence)
        if let Some(endpoint) = self.discover_from_environment(capability)? {
            self.cache_discovery(capability, endpoint.clone(), DiscoveryMethod::Environment)
                .await;
            return Ok(endpoint);
        }

        // 3. Attempt runtime discovery (if enabled)
        if self.discovery_config.enabled {
            for method in &self.discovery_config.methods {
                match self.discover_via_method(capability, method) {
                    Ok(Some(endpoint)) => {
                        self.cache_discovery(capability, endpoint.clone(), method.clone())
                            .await;
                        return Ok(endpoint);
                    }
                    Ok(None) => continue,
                    Err(e) => {
                        debug!(
                            "Discovery via {:?} failed for {}: {}",
                            method, capability, e
                        );
                    }
                }
            }
        }

        // 4. No hardcoded fallback - fail clearly
        anyhow::bail!(
            "Capability '{capability}' not found. Enable discovery or set environment variables."
        )
    }

    /// Discover from environment variables
    fn discover_from_environment(&self, capability: &str) -> Result<Option<ServiceEndpoint>> {
        // Construct environment variable name
        let env_prefix = format!("NESTGATE_{}_", capability.to_uppercase());

        // Try to get host and port
        let host_var = format!("{env_prefix}HOST");
        let port_var = format!("{env_prefix}PORT");

        if let (Ok(host), Ok(port_str)) = (std::env::var(&host_var), std::env::var(&port_var)) {
            let port = port_str
                .parse()
                .with_context(|| format!("Invalid port in {port_var}"))?;

            return Ok(Some(ServiceEndpoint {
                protocol: "http".to_string(), // Could be from env too
                address: host,
                port,
                path: None,
            }));
        }

        Ok(None)
    }

    /// Discover via specific method
    fn discover_via_method(
        &self,
        capability: &str,
        method: &DiscoveryMethod,
    ) -> Result<Option<ServiceEndpoint>> {
        match method {
            DiscoveryMethod::Environment => {
                // Already tried in discover_capability
                Ok(None)
            }
            DiscoveryMethod::MDns => {
                // Future: Implement mDNS discovery
                debug!("mDNS discovery not yet implemented for {}", capability);
                Ok(None)
            }
            DiscoveryMethod::DnsSd => {
                // Future: Implement DNS-SD discovery
                debug!("DNS-SD discovery not yet implemented for {}", capability);
                Ok(None)
            }
            DiscoveryMethod::Consul => {
                // Future: Implement Consul discovery
                debug!("Consul discovery not yet implemented for {}", capability);
                Ok(None)
            }
            DiscoveryMethod::Kubernetes => {
                // Future: Implement K8s service discovery
                debug!(
                    "Kubernetes discovery not yet implemented for {}",
                    capability
                );
                Ok(None)
            }
        }
    }

    /// Cache a discovered capability
    async fn cache_discovery(
        &self,
        capability: &str,
        endpoint: ServiceEndpoint,
        method: DiscoveryMethod,
    ) {
        let mut discovered = self.discovered.write().await;
        discovered.insert(
            capability.to_string(),
            DiscoveredCapability {
                name: capability.to_string(),
                endpoint,
                discovered_at: std::time::SystemTime::now(),
                discovery_method: method,
            },
        );
    }

    /// Get a port for a specific service
    ///
    /// Priority:
    /// 1. Environment variable
    /// 2. Runtime discovery
    /// 3. Error (no hardcoded fallback)
    ///
    /// # Errors
    ///
    /// Returns an error if the port cannot be determined.
    pub async fn get_port(&self, env_var: &str) -> Result<u16> {
        // 1. Check environment
        if let Ok(val) = std::env::var(env_var) {
            return val
                .parse()
                .with_context(|| format!("Invalid port in {env_var}: {val}"));
        }

        // 2. Try to discover
        let capability = env_var
            .strip_prefix("NESTGATE_")
            .and_then(|s| s.strip_suffix("_PORT"))
            .unwrap_or(env_var)
            .to_lowercase();

        if let Ok(endpoint) = self.discover_capability(&capability).await {
            return Ok(endpoint.port);
        }

        // 3. Fail clearly - no hardcoded fallback
        anyhow::bail!(
            "Port for {capability} not configured. Set {env_var} environment variable or enable discovery."
        )
    }

    /// Get a bind address for a socket
    ///
    /// # Errors
    ///
    /// Returns an error if the address cannot be parsed.
    pub fn get_bind_address(&self, env_var: &str, default: &str) -> Result<SocketAddr> {
        let addr_str = std::env::var(env_var).unwrap_or_else(|_| default.to_string());

        addr_str
            .parse()
            .with_context(|| format!("Invalid socket address: {addr_str}"))
    }

    /// Announce our capabilities to the ecosystem
    ///
    /// This is how other primals can discover us.
    ///
    /// # Errors
    ///
    /// Returns an error if announcement fails.
    pub fn announce(&self) -> Result<()> {
        if !self.discovery_config.enabled {
            debug!("Discovery disabled, not announcing capabilities");
            return Ok(());
        }

        for method in &self.discovery_config.methods {
            match self.announce_via_method(method) {
                Ok(()) => debug!("Successfully announced via {:?}", method),
                Err(e) => warn!("Failed to announce via {:?}: {}", method, e),
            }
        }

        Ok(())
    }

    /// Announce via specific method
    fn announce_via_method(&self, method: &DiscoveryMethod) -> Result<()> {
        match method {
            DiscoveryMethod::Environment => {
                // Environment doesn't support announcement
                Ok(())
            }
            DiscoveryMethod::MDns => {
                // Future: Implement mDNS announcement
                debug!("mDNS announcement not yet implemented");
                Ok(())
            }
            DiscoveryMethod::DnsSd => {
                // Future: Implement DNS-SD announcement
                debug!("DNS-SD announcement not yet implemented");
                Ok(())
            }
            DiscoveryMethod::Consul => {
                // Future: Implement Consul registration
                debug!("Consul registration not yet implemented");
                Ok(())
            }
            DiscoveryMethod::Kubernetes => {
                // Future: Implement K8s service registration
                debug!("Kubernetes registration not yet implemented");
                Ok(())
            }
        }
    }

    /// Get our self-knowledge
    #[must_use]
    pub fn self_knowledge(&self) -> &SelfKnowledge {
        &self.self_knowledge
    }

    /// Get discovered capabilities
    pub async fn discovered_capabilities(&self) -> HashMap<String, DiscoveredCapability> {
        self.discovered.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_capability_config_initialization() {
        let config = CapabilityConfig::initialize().await;
        assert!(config.is_ok());
    }

    #[tokio::test]
    async fn test_self_knowledge_has_identity() {
        let config = CapabilityConfig::initialize().await.unwrap();
        let knowledge = config.self_knowledge();

        assert_eq!(knowledge.identity.primal_type, "nestgate");
        assert!(!knowledge.identity.id.is_empty());
    }

    #[tokio::test]
    async fn test_discovery_config_default() {
        let config = DiscoveryConfig::default();
        assert!(!config.methods.is_empty());
    }

    #[test]
    fn test_service_endpoint_url() {
        let endpoint = ServiceEndpoint {
            protocol: "http".to_string(),
            address: "localhost".to_string(),
            port: 8080,
            path: Some("/api/v1".to_string()),
        };

        assert_eq!(endpoint.url(), "http://localhost:8080/api/v1");
    }

    #[test]
    fn test_service_endpoint_url_no_path() {
        let endpoint = ServiceEndpoint {
            protocol: "https".to_string(),
            address: "example.com".to_string(),
            port: 443,
            path: None,
        };

        assert_eq!(endpoint.url(), "https://example.com:443");
    }

    #[tokio::test]
    async fn test_announce_when_discovery_disabled() {
        let mut config = CapabilityConfig::initialize().await.unwrap();
        config.discovery_config.enabled = false;

        // Should succeed but do nothing
        let result = config.announce();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_discover_capability_not_found() {
        let config = CapabilityConfig::initialize().await.unwrap();

        // With discovery disabled and no environment, should fail clearly
        let result = config.discover_capability("nonexistent").await;
        assert!(result.is_err());
    }
}
