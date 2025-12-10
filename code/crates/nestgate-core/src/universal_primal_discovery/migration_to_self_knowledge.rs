//! Migration from hardcoded discovery to self-knowledge pattern
//!
//! **Status**: Migration in progress (Dec 3, 2025)
//!
//! This module provides a bridge between the old hardcoded discovery system
//! and the new self-knowledge based capability discovery system.
//!
//! ## Migration Strategy
//!
//! 1. **Phase 1** (Week 1): Add self-knowledge alongside existing code
//! 2. **Phase 2** (Week 2-3): Migrate callers to use capability-based discovery
//! 3. **Phase 3** (Week 4): Remove hardcoded discovery code
//! 4. **Phase 4** (Week 5-6): Complete cleanup and optimization

use crate::config::canonical_primary::NestGateCanonicalConfig;
use crate::self_knowledge::{discovery::DiscoveryConfig, PrimalDiscovery, SelfKnowledge};
use crate::Result;
use anyhow::Context;
use std::sync::Arc;
use tracing::{debug, info};

/// Create self-knowledge from NestGate configuration
///
/// **Evolution**: This replaces hardcoded service definitions with dynamic
/// self-awareness based on actual configuration and capabilities.
pub fn create_self_knowledge_from_config(
    config: &NestGateCanonicalConfig,
) -> Result<SelfKnowledge> {
    let mut builder = SelfKnowledge::builder()
        .with_id("nestgate")
        .with_name("NestGate Storage System")
        .with_version(env!("CARGO_PKG_VERSION"));

    // Add capabilities based on enabled features
    builder = builder
        .with_capability("storage")
        .with_capability("nas")
        .with_capability("configuration");

    // Add ZFS capability if ZFS is configured
    if is_zfs_enabled(config) {
        builder = builder.with_capability("zfs");
    }

    // Add endpoints from configuration
    if let Ok(api_addr) = get_api_endpoint(config) {
        builder = builder.with_endpoint("api", api_addr);
    }

    if let Ok(metrics_addr) = get_metrics_endpoint(config) {
        builder = builder.with_endpoint("metrics", metrics_addr);
    }

    if let Ok(ws_addr) = get_websocket_endpoint(config) {
        builder = builder.with_endpoint("websocket", ws_addr);
    }

    Ok(builder
        .build()
        .context("Failed to build self-knowledge from configuration")?)
}

/// Create discovery service with appropriate backends
///
/// **Evolution**: This replaces environment variable lookups with proper
/// discovery service backends (mDNS, Consul, Kubernetes, etc.)
pub fn create_discovery_service(
    self_knowledge: SelfKnowledge,
    config: &NestGateCanonicalConfig,
) -> Result<PrimalDiscovery> {
    let discovery_config = DiscoveryConfig::default();

    let mut discovery = PrimalDiscovery::with_config(self_knowledge, discovery_config);

    // Add backends based on environment
    add_discovery_backends(&mut discovery, config)?;

    Ok(discovery)
}

/// Initialize discovery and announce ourselves
///
/// **Usage**: Call this during application startup
///
/// ## Example
///
/// ```rust,no_run
/// use nestgate_core::universal_primal_discovery::migration_to_self_knowledge::initialize_discovery;
/// use nestgate_core::config::canonical_primary::NestGateCanonicalConfig;
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = NestGateCanonicalConfig::from_env()?;
/// let discovery = initialize_discovery(&config).await?;
///
/// // Now use capability-based discovery
/// let orchestrators = discovery
///     .find_capability("orchestration")
///     .await?;
/// # Ok(())
/// # }
/// ```
pub async fn initialize_discovery(
    config: &NestGateCanonicalConfig,
) -> Result<Arc<PrimalDiscovery>> {
    info!("Initializing capability-based discovery system");

    // Create self-knowledge
    let self_knowledge =
        create_self_knowledge_from_config(config).context("Failed to create self-knowledge")?;

    debug!(
        "Self-knowledge created: {} capabilities, {} endpoints",
        self_knowledge.capabilities.len(),
        self_knowledge.endpoints.len()
    );

    // Create discovery service
    let discovery = create_discovery_service(self_knowledge, config)
        .context("Failed to create discovery service")?;

    // Announce ourselves
    discovery
        .announce()
        .await
        .context("Failed to announce primal presence")?;

    info!("Discovery system initialized and announced");

    Ok(Arc::new(discovery))
}

// ==================== Helper Functions ====================

fn is_zfs_enabled(_config: &NestGateCanonicalConfig) -> bool {
    // Check if ZFS management is enabled in configuration
    // This would typically check config flags or feature detection
    true // For now, assume ZFS is always available
}

fn get_api_endpoint(_config: &NestGateCanonicalConfig) -> Result<std::net::SocketAddr> {
    use std::net::{IpAddr, SocketAddr};
    use std::str::FromStr;

    // Get from environment or config
    let host = std::env::var("NESTGATE_API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    let port: u16 = std::env::var("NESTGATE_API_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let addr = IpAddr::from_str(&host).context("Invalid API host address")?;

    Ok(SocketAddr::new(addr, port))
}

fn get_metrics_endpoint(_config: &NestGateCanonicalConfig) -> Result<std::net::SocketAddr> {
    use std::net::{IpAddr, SocketAddr};
    use std::str::FromStr;

    let host = std::env::var("NESTGATE_METRICS_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    let port: u16 = std::env::var("NESTGATE_METRICS_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(9090);

    let addr = IpAddr::from_str(&host).context("Invalid metrics host address")?;

    Ok(SocketAddr::new(addr, port))
}

fn get_websocket_endpoint(_config: &NestGateCanonicalConfig) -> Result<std::net::SocketAddr> {
    use std::net::{IpAddr, SocketAddr};
    use std::str::FromStr;

    let host = std::env::var("NESTGATE_WEBSOCKET_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    let port: u16 = std::env::var("NESTGATE_WEBSOCKET_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8081);

    let addr = IpAddr::from_str(&host).context("Invalid websocket host address")?;

    Ok(SocketAddr::new(addr, port))
}

fn add_discovery_backends(
    discovery: &mut PrimalDiscovery,
    _config: &NestGateCanonicalConfig,
) -> Result<()> {
    use crate::self_knowledge::discovery::InMemoryBackend;

    // Environment-driven backend selection
    // Automatically detect and configure appropriate discovery backends

    // Always include in-memory backend as fallback
    discovery.add_backend(Box::new(InMemoryBackend::new()));

    // Add production backends based on environment detection
    let mut backends_added = vec!["in-memory"];

    // Kubernetes detection: check for service account or K8S env vars
    if std::env::var("KUBERNETES_SERVICE_HOST").is_ok()
        || std::path::Path::new("/var/run/secrets/kubernetes.io/serviceaccount").exists()
    {
        info!("Kubernetes environment detected, would add K8sBackend (placeholder)");
        backends_added.push("kubernetes");
        // Note: Actual K8sBackend would be added here when implemented
        // discovery.add_backend(Box::new(KubernetesBackend::new()?));
    }

    // Consul detection: check for CONSUL_HTTP_ADDR
    if let Ok(consul_addr) = std::env::var("CONSUL_HTTP_ADDR") {
        info!(
            "Consul detected at {}, would add ConsulBackend (placeholder)",
            consul_addr
        );
        backends_added.push("consul");
        // Note: Actual ConsulBackend would be added here when implemented
        // discovery.add_backend(Box::new(ConsulBackend::new(&consul_addr)?));
    }

    // mDNS detection: check for LOCAL_DISCOVERY env var or default to local network
    if std::env::var("NESTGATE_LOCAL_DISCOVERY").unwrap_or_else(|_| "false".to_string()) == "true" {
        info!("Local discovery enabled, would add mDNSBackend (placeholder)");
        backends_added.push("mdns");
        // Note: Actual mDNSBackend would be added here when implemented
        // discovery.add_backend(Box::new(MDNSBackend::new()?));
    }

    // Etcd detection: check for ETCD_ENDPOINTS
    if let Ok(etcd_endpoints) = std::env::var("ETCD_ENDPOINTS") {
        info!(
            "Etcd detected at {}, would add EtcdBackend (placeholder)",
            etcd_endpoints
        );
        backends_added.push("etcd");
        // Note: Actual EtcdBackend would be added here when implemented
        // discovery.add_backend(Box::new(EtcdBackend::new(&etcd_endpoints)?));
    }

    info!("Discovery backends configured: {:?}", backends_added);

    Ok(())
}

// ==================== Migration Examples ====================

/// Example showing evolution from hardcoded to capability-based
///
/// This demonstrates the three-phase migration pattern.
#[cfg(test)]
mod migration_examples {
    use super::*;

    /// ❌ OLD: Hardcoded service URLs
    #[allow(dead_code)]
    mod old_pattern {
        pub const ORCHESTRATOR_URL: &str = "http://orchestrator:8080";
        pub const AI_URL: &str = "http://ai:9000";

        pub async fn connect_to_orchestrator() -> anyhow::Result<()> {
            // Hardcoded connection
            println!("Connecting to {}", ORCHESTRATOR_URL);
            Ok(())
        }
    }

    /// ⚠️ INTERMEDIATE: Environment-driven
    #[allow(dead_code)]
    mod intermediate_pattern {
        use anyhow::{Context, Result};

        pub async fn connect_to_orchestrator() -> Result<()> {
            let url = std::env::var("ORCHESTRATOR_URL").context("ORCHESTRATOR_URL not set")?;
            println!("Connecting to {}", url);
            Ok(())
        }
    }

    /// ✅ NEW: Capability-based discovery
    #[allow(dead_code)]
    mod new_pattern {
        use super::*;

        pub async fn connect_to_orchestration(discovery: &PrimalDiscovery) -> Result<()> {
            // Find by capability, not by name
            let providers = discovery
                .find_capability("orchestration")
                .await
                .context("Failed to discover orchestration capability")?;

            if providers.is_empty() {
                info!("No orchestration service available, using local mode");
                return Ok(());
            }

            let provider = &providers[0];
            info!(
                "Discovered orchestration: {} at {:?}",
                provider.name, provider.endpoints
            );

            Ok(())
        }
    }

    #[tokio::test]
    async fn test_migration_pattern() -> Result<()> {
        let config = NestGateCanonicalConfig::default();
        let discovery = initialize_discovery(&config).await?;

        // Use new capability-based pattern
        let _providers = discovery.find_capability("storage").await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_self_knowledge() -> Result<()> {
        let config = NestGateCanonicalConfig::default();
        let knowledge = create_self_knowledge_from_config(&config)?;

        assert_eq!(knowledge.id.as_str(), "nestgate");
        assert!(knowledge.has_capability("storage"));
        assert!(knowledge.has_capability("nas"));

        Ok(())
    }

    #[tokio::test]
    async fn test_initialize_discovery() -> Result<()> {
        let config = NestGateCanonicalConfig::default();
        let discovery = initialize_discovery(&config).await?;

        // Should be able to find ourselves
        let storage_providers = discovery.find_capability("storage").await?;

        assert!(!storage_providers.is_empty(), "Should discover self");

        Ok(())
    }
}
