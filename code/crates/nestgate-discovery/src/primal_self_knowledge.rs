// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! Primal Self-Knowledge System
//!
//! Core implementation of the self-knowledge philosophy: Each primal knows what
//! it can do, announces itself, and discovers others at runtime.
//!
//! # Philosophy
//!
//! - **Self-Knowledge**: Each primal introspects its own capabilities
//! - **Announcement**: Primals announce themselves to the ecosystem
//! - **Discovery**: Primals discover others through runtime mechanisms
//! - **No Hardcoding**: Zero assumptions about other primals' locations
//!
//! # Example
//!
//! ```rust,ignore
//! use nestgate_core::primal_self_knowledge::PrimalSelfKnowledge;
//! use anyhow::Result;
//!
//! # async fn example() -> Result<()> {
//! // Initialize with self-knowledge
//! let mut primal = PrimalSelfKnowledge::initialize().await?;
//!
//! // Announce ourselves to the ecosystem
//! primal.announce_self()?;
//!
//! // Discover another primal at runtime by capability
//! let peer = primal.discover_primal("orchestration_provider").await?;
//! println!("Found peer at: {}", peer.primary_endpoint());
//! # Ok(())
//! # }
//! ```

use anyhow::{Context, Result};
use dashmap::DashMap;
use nestgate_types::{EnvSource, ProcessEnv, env_parsed};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Primal Self-Knowledge System
///
/// Implements the complete primal philosophy:
/// - Self-introspection
/// - Capability announcement
/// - Runtime discovery
#[derive(Clone)]
pub struct PrimalSelfKnowledge {
    /// What we know about ourselves
    identity: Arc<PrimalIdentity>,

    /// Capabilities we provide
    capabilities: Arc<Vec<Capability>>,

    /// How we can be reached
    endpoints: Arc<Vec<Endpoint>>,

    /// Discovered other primals (runtime only, lock-free!)
    discovered_primals: Arc<DashMap<String, DiscoveredPrimal>>, // ✅ Lock-free

    /// Discovery mechanisms we support
    discovery_mechanisms: Vec<DiscoveryMechanism>,

    /// Environment source (production: process env; tests: [`MapEnv`](nestgate_types::MapEnv))
    env: Arc<dyn EnvSource>,
}

/// Our identity as a primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalIdentity {
    /// Unique identifier (generated at runtime)
    pub id: String,

    /// Primal type (e.g., "nestgate")
    pub primal_type: String,

    /// Semantic version
    pub version: String,

    /// When we started (birth time)
    pub started_at: std::time::SystemTime,
}

/// A capability we provide
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    /// Capability name (e.g., "storage", "zfs", "encryption")
    pub name: String,

    /// Description of what this capability does
    pub description: String,

    /// Endpoint where this capability is accessible
    pub endpoint: String,

    /// Metadata about this capability
    pub metadata: std::collections::HashMap<String, String>,
}

/// An endpoint where we can be reached
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    /// Protocol (http, grpc, websocket, etc.)
    pub protocol: String,

    /// Network address
    pub address: String,

    /// Port number (from environment or discovery, never hardcoded)
    pub port: u16,

    /// URL path (if applicable)
    pub path: Option<String>,

    /// Health check path
    pub health_path: Option<String>,
}

impl Endpoint {
    /// Get the full URL for this endpoint
    #[must_use]
    pub fn url(&self) -> String {
        let path = self.path.as_deref().unwrap_or("");
        format!("{}://{}:{}{}", self.protocol, self.address, self.port, path)
    }

    /// Get the health check URL
    #[must_use]
    pub fn health_url(&self) -> Option<String> {
        self.health_path.as_ref().map(|health_path| {
            format!(
                "{}://{}:{}{}",
                self.protocol, self.address, self.port, health_path
            )
        })
    }
}

/// Discovery mechanism for finding other primals
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiscoveryMechanism {
    /// Environment variables (explicit configuration)
    Environment,

    /// mDNS/Bonjour/Zeroconf
    MDns,

    /// DNS Service Discovery (RFC 6763)
    DnsSd,

    /// `HashiCorp` Consul
    Consul,

    /// Kubernetes service discovery
    Kubernetes,

    /// File-based configuration
    FileConfig,
}

/// A discovered primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPrimal {
    /// Primal identity
    pub identity: PrimalIdentity,

    /// Capabilities this primal provides
    pub capabilities: Vec<Capability>,

    /// Primary endpoint for this primal
    pub primary_endpoint: Endpoint,

    /// When we discovered this primal
    pub discovered_at: std::time::SystemTime,

    /// How we discovered it
    pub discovery_method: DiscoveryMechanism,
}

impl DiscoveredPrimal {
    /// Get the primary endpoint URL
    #[must_use]
    pub fn primary_endpoint(&self) -> String {
        self.primary_endpoint.url()
    }

    /// Check if this primal provides a specific capability
    #[must_use]
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities
            .iter()
            .any(|c| c.name.eq_ignore_ascii_case(capability))
    }
}

impl PrimalSelfKnowledge {
    /// Initialize primal self-knowledge
    ///
    /// Introspects capabilities and builds self-knowledge at runtime.
    ///
    /// # Errors
    ///
    /// Returns an error if self-introspection fails.
    pub async fn initialize() -> Result<Self> {
        Self::initialize_with_env(Arc::new(ProcessEnv)).await
    }

    /// Initialize with an injectable environment source (use [`MapEnv`](nestgate_types::MapEnv) in tests)
    pub async fn initialize_with_env(env: Arc<dyn EnvSource>) -> Result<Self> {
        info!("Initializing primal self-knowledge");

        // Generate our identity
        let identity = Arc::new(PrimalIdentity {
            id: uuid::Uuid::new_v4().to_string(),
            primal_type: "nestgate".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            started_at: std::time::SystemTime::now(),
        });

        // Introspect our capabilities
        let capabilities = Arc::new(Self::introspect_capabilities().await?);

        // Build our endpoints from environment (no hardcoding)
        let endpoints = Arc::new(Self::build_endpoints_from_env_source(env.as_ref())?);

        // Determine discovery mechanisms
        let discovery_mechanisms = Self::determine_discovery_mechanisms_from_env(env.as_ref());

        Ok(Self {
            identity,
            capabilities,
            endpoints,
            discovered_primals: Arc::new(DashMap::new()), // ✅ Lock-free
            discovery_mechanisms,
            env,
        })
    }

    /// Introspect our capabilities
    async fn introspect_capabilities() -> Result<Vec<Capability>> {
        let mut capabilities = Vec::new();

        // Always provide storage capability
        capabilities.push(Capability {
            name: "storage".to_string(),
            description: "Universal storage management".to_string(),
            endpoint: "/api/v1/storage".to_string(),
            metadata: std::collections::HashMap::new(),
        });

        // Check if ZFS is available (runtime capability detection - universal!)
        if Self::check_zfs_available().await {
            capabilities.push(Capability {
                name: "zfs".to_string(),
                description: "ZFS pool and dataset management".to_string(),
                endpoint: "/api/v1/zfs".to_string(),
                metadata: std::collections::HashMap::from([(
                    "backend".to_string(),
                    "native".to_string(),
                )]),
            });
        }

        Ok(capabilities)
    }

    /// Check if ZFS is available on this system
    ///
    /// **UNIVERSAL**: Works on ALL platforms (runtime capability detection)
    ///
    /// Tries to execute `zfs --version` command. If it succeeds, ZFS is available.
    /// This works regardless of platform - no #[cfg] needed!
    async fn check_zfs_available() -> bool {
        match tokio::process::Command::new("zfs")
            .arg("--version")
            .output()
            .await
        {
            Ok(output) => {
                let available = output.status.success();
                if available {
                    debug!("✅ ZFS capability detected (zfs command available)");
                } else {
                    debug!("ℹ️  ZFS command found but returned error");
                }
                available
            }
            Err(e) => {
                debug!("ℹ️  ZFS not available: {}", e);
                false
            }
        }
    }

    /// Build endpoints from environment (no hardcoded values)
    fn build_endpoints_from_env_source(env: &dyn EnvSource) -> Result<Vec<Endpoint>> {
        let mut endpoints = Vec::new();

        // Get API endpoint from environment
        let api_host = env
            .get("NESTGATE_API_HOST")
            .unwrap_or_else(|| "0.0.0.0".to_string());

        let api_port_str = env.get("NESTGATE_API_PORT").unwrap_or_else(|| {
            nestgate_config::constants::hardcoding::runtime_fallback_ports::API.to_string()
        });

        let api_port = api_port_str
            .parse()
            .with_context(|| format!("Invalid NESTGATE_API_PORT: {api_port_str}"))?;

        endpoints.push(Endpoint {
            protocol: "http".to_string(),
            address: api_host,
            port: api_port,
            path: Some("/api/v1".to_string()),
            health_path: Some("/health".to_string()),
        });

        Ok(endpoints)
    }

    /// Determine which discovery mechanisms to use
    fn determine_discovery_mechanisms_from_env(env: &dyn EnvSource) -> Vec<DiscoveryMechanism> {
        let mut mechanisms = vec![DiscoveryMechanism::Environment];

        // Check if mDNS should be enabled
        if env_parsed::<bool>(env, "NESTGATE_MDNS_ENABLED", false) {
            mechanisms.push(DiscoveryMechanism::MDns);
        }

        // Check for Kubernetes
        if env.get("KUBERNETES_SERVICE_HOST").is_some() {
            mechanisms.push(DiscoveryMechanism::Kubernetes);
        }

        mechanisms
    }

    /// Announce ourselves to the ecosystem
    ///
    /// Makes our capabilities discoverable to other primals.
    ///
    /// # Errors
    ///
    /// Returns an error if announcement fails critically.
    pub fn announce_self(&self) -> Result<()> {
        info!(
            "Announcing primal {} to ecosystem",
            self.identity.primal_type
        );

        for mechanism in &self.discovery_mechanisms {
            match self.announce_via_mechanism(mechanism) {
                Ok(()) => info!("Announced via {:?}", mechanism),
                Err(e) => warn!("Failed to announce via {:?}: {}", mechanism, e),
            }
        }

        Ok(())
    }

    /// Announce via specific mechanism
    fn announce_via_mechanism(&self, mechanism: &DiscoveryMechanism) -> Result<()> {
        match mechanism {
            DiscoveryMechanism::Environment => {
                // Environment doesn't support active announcement
                debug!("Environment mechanism doesn't require announcement");
                Ok(())
            }
            DiscoveryMechanism::MDns => {
                // Future: Implement mDNS announcement
                // This would broadcast our capabilities via multicast DNS
                debug!("mDNS announcement not yet implemented");
                Ok(())
            }
            DiscoveryMechanism::DnsSd => {
                debug!("DNS-SD announcement not yet implemented");
                Ok(())
            }
            DiscoveryMechanism::Consul => {
                debug!("Consul registration not yet implemented");
                Ok(())
            }
            DiscoveryMechanism::Kubernetes => {
                // K8s services are automatically discoverable
                debug!("Kubernetes services auto-registered");
                Ok(())
            }
            DiscoveryMechanism::FileConfig => {
                debug!("File config doesn't require announcement");
                Ok(())
            }
        }
    }

    /// Discover another primal by type
    ///
    /// Pure runtime discovery - no hardcoded locations!
    ///
    /// # Errors
    ///
    /// Returns an error if the primal cannot be found.
    pub async fn discover_primal(&mut self, primal_type: &str) -> Result<DiscoveredPrimal> {
        // 1. Check cache (lock-free!)
        if let Some(cached) = self.discovered_primals.get(primal_type) {
            debug!("Using cached discovery for {}", primal_type);
            return Ok(cached.clone());
        }

        // 2. Try each discovery mechanism
        for mechanism in &self.discovery_mechanisms.clone() {
            match self.discover_via_mechanism(primal_type, mechanism).await {
                Ok(Some(primal)) => {
                    info!("Discovered {} via {:?}", primal_type, mechanism);

                    // Cache the discovery
                    // ✅ Lock-free: Insert discovered primal
                    self.discovered_primals
                        .insert(primal_type.to_string(), primal.clone());

                    return Ok(primal);
                }
                Ok(None) => {}
                Err(e) => {
                    debug!("Discovery via {:?} failed: {}", mechanism, e);
                }
            }
        }

        // 3. Not found - fail clearly (no hardcoded fallback!)
        anyhow::bail!(
            "Primal '{primal_type}' not discovered. Configure environment or enable discovery mechanisms."
        )
    }

    /// Discover via specific mechanism
    async fn discover_via_mechanism(
        &self,
        primal_type: &str,
        mechanism: &DiscoveryMechanism,
    ) -> Result<Option<DiscoveredPrimal>> {
        match mechanism {
            DiscoveryMechanism::Environment => self.discover_from_environment(primal_type),
            DiscoveryMechanism::MDns => {
                // Future: Query mDNS for primal
                debug!("mDNS discovery not yet implemented");
                Ok(None)
            }
            DiscoveryMechanism::DnsSd => {
                debug!("DNS-SD discovery not yet implemented");
                Ok(None)
            }
            DiscoveryMechanism::Consul => {
                debug!("Consul discovery not yet implemented");
                Ok(None)
            }
            DiscoveryMechanism::Kubernetes => self.discover_from_kubernetes(primal_type).await,
            DiscoveryMechanism::FileConfig => {
                debug!("File config discovery not yet implemented");
                Ok(None)
            }
        }
    }

    /// Discover from environment variables
    fn discover_from_environment(&self, primal_type: &str) -> Result<Option<DiscoveredPrimal>> {
        let prefix = format!("{}_{}", primal_type.to_uppercase(), "HOST");
        let port_var = format!("{}_{}", primal_type.to_uppercase(), "PORT");

        if let (Some(host), Some(port_str)) = (self.env.get(&prefix), self.env.get(&port_var)) {
            let port = port_str
                .parse()
                .with_context(|| format!("Invalid port in {port_var}"))?;

            let endpoint = Endpoint {
                protocol: "http".to_string(),
                address: host,
                port,
                path: Some("/api/v1".to_string()),
                health_path: Some("/health".to_string()),
            };

            let identity = PrimalIdentity {
                id: format!("{primal_type}-discovered-via-env"),
                primal_type: primal_type.to_string(),
                version: "unknown".to_string(),
                started_at: std::time::SystemTime::now(),
            };

            return Ok(Some(DiscoveredPrimal {
                identity,
                capabilities: vec![], // Unknown until we query
                primary_endpoint: endpoint,
                discovered_at: std::time::SystemTime::now(),
                discovery_method: DiscoveryMechanism::Environment,
            }));
        }

        Ok(None)
    }

    /// Discover from Kubernetes
    async fn discover_from_kubernetes(
        &self,
        primal_type: &str,
    ) -> Result<Option<DiscoveredPrimal>> {
        // Check if we're in Kubernetes
        if self.env.get("KUBERNETES_SERVICE_HOST").is_none() {
            return Ok(None);
        }

        // Construct expected service name
        let service_name = format!("{primal_type}-service");
        let namespace = self
            .env
            .get("KUBERNETES_NAMESPACE")
            .unwrap_or_else(|| "default".to_string());

        // K8s service DNS: <service>.<namespace>.svc.cluster.local
        let dns_name = format!("{service_name}.{namespace}.svc.cluster.local");

        // Try to resolve the DNS name
        match tokio::net::lookup_host(format!("{dns_name}:80")).await {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    let endpoint = Endpoint {
                        protocol: "http".to_string(),
                        address: dns_name,
                        port: addr.port(),
                        path: Some("/api/v1".to_string()),
                        health_path: Some("/health".to_string()),
                    };

                    let identity = PrimalIdentity {
                        id: format!("{primal_type}-k8s"),
                        primal_type: primal_type.to_string(),
                        version: "unknown".to_string(),
                        started_at: std::time::SystemTime::now(),
                    };

                    return Ok(Some(DiscoveredPrimal {
                        identity,
                        capabilities: vec![],
                        primary_endpoint: endpoint,
                        discovered_at: std::time::SystemTime::now(),
                        discovery_method: DiscoveryMechanism::Kubernetes,
                    }));
                }
            }
            Err(e) => {
                debug!("Failed to resolve {}: {}", dns_name, e);
            }
        }

        Ok(None)
    }

    /// Get our identity
    #[must_use]
    pub fn identity(&self) -> &PrimalIdentity {
        &self.identity
    }

    /// Get our capabilities
    #[must_use]
    pub fn capabilities(&self) -> &[Capability] {
        &self.capabilities
    }

    /// Get our endpoints
    #[must_use]
    pub fn endpoints(&self) -> &[Endpoint] {
        &self.endpoints
    }

    /// Get discovered primals (lock-free!)
    #[must_use]
    pub fn discovered_primals(&self) -> std::collections::HashMap<String, DiscoveredPrimal> {
        self.discovered_primals
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_types::MapEnv;
    use std::sync::Arc;

    #[test]
    fn endpoint_url_without_path_suffix() {
        let endpoint = Endpoint {
            protocol: "https".to_string(),
            address: "10.0.0.1".to_string(),
            port: 443,
            path: None,
            health_path: None,
        };
        assert_eq!(endpoint.url(), "https://10.0.0.1:443");
        assert!(endpoint.health_url().is_none());
    }

    #[tokio::test]
    async fn discover_primal_errors_when_not_configured() {
        let mut primal = PrimalSelfKnowledge::initialize_with_env(Arc::new(MapEnv::new()))
            .await
            .expect("initialize");
        let err = primal
            .discover_primal("orchestration_provider")
            .await
            .expect_err("no discovery source");
        assert!(
            err.to_string().contains("not discovered"),
            "unexpected: {err}"
        );
    }

    #[tokio::test]
    async fn discovered_primals_map_starts_empty() {
        let primal = PrimalSelfKnowledge::initialize().await.expect("initialize");
        assert!(primal.discovered_primals().is_empty());
    }

    #[tokio::test]
    async fn test_primal_initialization() {
        let primal = PrimalSelfKnowledge::initialize().await;
        assert!(primal.is_ok());
    }

    #[tokio::test]
    async fn test_primal_has_identity() {
        let primal = PrimalSelfKnowledge::initialize().await.unwrap();
        let identity = primal.identity();

        assert_eq!(identity.primal_type, "nestgate");
        assert!(!identity.id.is_empty());
        assert!(!identity.version.is_empty());
    }

    #[tokio::test]
    async fn test_primal_has_capabilities() {
        let primal = PrimalSelfKnowledge::initialize().await.unwrap();
        let caps = primal.capabilities();

        assert!(!caps.is_empty());
        assert!(caps.iter().any(|c| c.name == "storage"));
    }

    #[tokio::test]
    async fn test_primal_has_endpoints() {
        let primal = PrimalSelfKnowledge::initialize().await.unwrap();
        let endpoints = primal.endpoints();

        assert!(!endpoints.is_empty());
    }

    #[tokio::test]
    async fn test_endpoint_url() {
        let endpoint = Endpoint {
            protocol: "http".to_string(),
            address: "localhost".to_string(),
            port: 8080,
            path: Some("/api".to_string()),
            health_path: Some("/health".to_string()),
        };

        assert_eq!(endpoint.url(), "http://localhost:8080/api");
        assert_eq!(
            endpoint.health_url(),
            Some("http://localhost:8080/health".to_string())
        );
    }

    #[tokio::test]
    async fn test_discovered_primal_has_capability() {
        let discovered = DiscoveredPrimal {
            identity: PrimalIdentity {
                id: "test".to_string(),
                primal_type: "testprimal".to_string(),
                version: "1.0.0".to_string(),
                started_at: std::time::SystemTime::now(),
            },
            capabilities: vec![Capability {
                name: "storage".to_string(),
                description: "Storage".to_string(),
                endpoint: "/storage".to_string(),
                metadata: std::collections::HashMap::new(),
            }],
            primary_endpoint: Endpoint {
                protocol: "http".to_string(),
                address: "localhost".to_string(),
                port: 8080,
                path: None,
                health_path: None,
            },
            discovered_at: std::time::SystemTime::now(),
            discovery_method: DiscoveryMechanism::Environment,
        };

        assert!(discovered.has_capability("storage"));
        assert!(discovered.has_capability("STORAGE")); // Case insensitive
        assert!(!discovered.has_capability("nonexistent"));
    }

    #[tokio::test]
    async fn test_announce_self() {
        let primal = PrimalSelfKnowledge::initialize().await.unwrap();

        // Should succeed (may do nothing if mechanisms not configured)
        let result = primal.announce_self();
        assert!(result.is_ok());
    }
}
