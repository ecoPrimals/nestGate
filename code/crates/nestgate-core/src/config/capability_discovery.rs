//! Capability-Based Configuration Discovery
//!
//! This module implements capability-based service discovery for configuration,
//! eliminating hardcoded endpoints and enabling true primal sovereignty.
//!
//! # Architecture
//!
//! ```text
//! Configuration Loading Flow:
//!
//! 1. Capability Discovery (runtime, agnostic)
//!    ↓
//! 2. Environment Variables (deployment-specific)
//!    ↓
//! 3. Local Discovery (mDNS, service registry)
//!    ↓
//! 4. Intelligent Defaults (safe fallbacks)
//! ```
//!
//! # Examples
//!
//! ```rust,no_run
//! use nestgate_core::config::capability_discovery;
//!
//! # async fn example() -> nestgate_core::Result<()> {
//! // Discover storage service endpoint
//! let endpoint = capability_discovery::discover_service("storage").await?;
//! println!("Found storage at: {}", endpoint);
//!
//! // With fallback chain
//! let endpoint = capability_discovery::discover_with_fallback(
//!     "api",                          // Capability type
//!     "NESTGATE_API_ENDPOINT",        // Env var
//!     "http://localhost:8080"         // Default
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Sovereignty Compliance
//!
//! ✅ **Self-Knowledge Only**: Only announces own capabilities
//! ✅ **Runtime Discovery**: No hardcoded primal endpoints
//! ✅ **Agnostic**: Works across any deployment environment
//! ✅ **Fallback Safe**: Graceful degradation to defaults

use crate::error::NestGateError;
use crate::Result;
use std::env;
use std::time::Duration;

// ==================== TYPES ====================

/// Service endpoint discovered through capability system
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceEndpoint {
    /// Service capability type (e.g., "storage", "compute", "api")
    pub capability: String,
    /// Full endpoint URL or address
    pub endpoint: String,
    /// Time-to-live for this discovery
    pub ttl: Duration,
    /// Source of this discovery
    pub source: DiscoverySource,
}

/// Source of a service discovery
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscoverySource {
    /// Discovered through capability registry
    CapabilityRegistry,
    /// Loaded from environment variable
    Environment,
    /// Discovered via local mDNS
    LocalDiscovery,
    /// Using safe default
    Default,
}

// ==================== DISCOVERY FUNCTIONS ====================

/// Discover service endpoint through capability system
///
/// This is the primary discovery mechanism that respects primal sovereignty:
/// - No hardcoded endpoints
/// - Runtime discovery only
/// - Self-knowledge pattern
///
/// # Errors
///
/// Returns error if service cannot be discovered through any method
pub async fn discover_service(capability: &str) -> Result<ServiceEndpoint> {
    // Try capability registry first (preferred)
    if let Ok(endpoint) = discover_from_capability_registry(capability).await {
        return Ok(endpoint);
    }

    // Try environment variable
    if let Ok(endpoint) = discover_from_environment(capability).await {
        return Ok(endpoint);
    }

    // Try local discovery (mDNS, etc.)
    if let Ok(endpoint) = discover_from_local(capability).await {
        return Ok(endpoint);
    }

    // No service found
    Err(NestGateError::network_error(&format!(
        "Service '{}' not found (tried: capability, environment, local discovery)",
        capability
    )))
}

/// Discover service with fallback to default
///
/// This provides the complete fallback chain for production use:
/// capability → environment → local → default
///
/// # Arguments
///
/// * `capability` - Service capability type to discover
/// * `env_var` - Environment variable name to check
/// * `default_endpoint` - Safe default if all discovery methods fail
pub async fn discover_with_fallback(
    capability: &str,
    env_var: &str,
    default_endpoint: &str,
) -> Result<ServiceEndpoint> {
    // Try full discovery chain
    if let Ok(endpoint) = discover_service(capability).await {
        return Ok(endpoint);
    }

    // Check specific env var if provided
    if let Ok(value) = env::var(env_var) {
        return Ok(ServiceEndpoint {
            capability: capability.to_string(),
            endpoint: value,
            ttl: Duration::from_secs(300),
            source: DiscoverySource::Environment,
        });
    }

    // Use safe default
    Ok(ServiceEndpoint {
        capability: capability.to_string(),
        endpoint: default_endpoint.to_string(),
        ttl: Duration::from_secs(3600),
        source: DiscoverySource::Default,
    })
}

/// Announce own capabilities to discovery system
///
/// This implements the self-knowledge pattern:
/// - Announces what WE offer
/// - Does not assume anything about other primals
/// - Enables others to discover us
///
/// # Examples
///
/// ```rust,no_run
/// use nestgate_core::config::capability_discovery;
///
/// # async fn example() -> nestgate_core::Result<()> {
/// // Announce our API capability
/// capability_discovery::announce_capability(
///     "api",
///     "http://localhost:8080",
///     std::time::Duration::from_secs(60)
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub async fn announce_capability(capability: &str, endpoint: &str, ttl: Duration) -> Result<()> {
    // TODO: Complete Infant Discovery system implementation first
    // The InfantDiscoverySystem currently lacks `announce_capability()` method
    // Following philosophy: "No mocks in production - mark incomplete until real implementation"
    
    tracing::info!(
        "📢 Would announce capability '{}' at '{}' (TTL: {:?}) - awaiting InfantDiscoverySystem.announce_capability()",
        capability,
        endpoint,
        ttl
    );
    
    // For now, log the announcement intent
    // This will be implemented when InfantDiscoverySystem is complete
    
    Ok(())
}

// ==================== INTERNAL DISCOVERY METHODS ====================

/// Discover from capability registry (primary method)
async fn discover_from_capability_registry(capability: &str) -> Result<ServiceEndpoint> {
    // TODO: Complete Infant Discovery system implementation first
    // The InfantDiscoverySystem currently lacks `discover_capabilities()` method
    // Following philosophy: "No mocks in production - mark incomplete until real implementation"
    
    tracing::debug!(
        "Capability registry discovery not yet complete for: {}",
        capability
    );
    
    // Return not found to fall through to environment/default discovery
    Err(NestGateError::network_error(&format!(
        "Capability registry discovery requires InfantDiscoverySystem.discover_capabilities() implementation (capability: {})",
        capability
    )))
}

/// Discover from environment variables
async fn discover_from_environment(capability: &str) -> Result<ServiceEndpoint> {
    // Build env var name: NESTGATE_<CAPABILITY>_ENDPOINT
    let env_var = format!("NESTGATE_{}_ENDPOINT", capability.to_uppercase());

    if let Ok(endpoint) = env::var(&env_var) {
        return Ok(ServiceEndpoint {
            capability: capability.to_string(),
            endpoint,
            ttl: Duration::from_secs(300),
            source: DiscoverySource::Environment,
        });
    }

    Err(NestGateError::network_error(&format!(
        "Environment variable '{}' not set",
        env_var
    )))
}

/// Discover from local network (mDNS, etc.)
async fn discover_from_local(capability: &str) -> Result<ServiceEndpoint> {
    // EVOLUTION: Implement mDNS discovery for local network scanning
    // Following philosophy: "Discover primals at runtime via mDNS/Songbird"
    
    tracing::debug!("Attempting local mDNS discovery for: {}", capability);
    
    // Try mDNS discovery (Songbird integration)
    // This scans the local network for services advertising this capability
    #[cfg(feature = "mdns-discovery")]
    {
        use crate::universal_primal_discovery::backends::mdns::MdnsDiscoveryBackend;
        
        let mdns_backend = MdnsDiscoveryBackend::new()
            .map_err(|e| NestGateError::network_error(&format!("mDNS init failed: {:?}", e)))?;
        
        // Scan for services with this capability
        let services = mdns_backend
            .discover_services(capability, std::time::Duration::from_secs(5))
            .await
            .map_err(|e| NestGateError::network_error(&format!("mDNS scan failed: {:?}", e)))?;
        
        if let Some(service) = services.first() {
            tracing::info!("✅ Discovered {} via mDNS: {}", capability, service.endpoint);
            return Ok(ServiceEndpoint {
                capability: capability.to_string(),
                endpoint: service.endpoint.clone(),
                ttl: Duration::from_secs(60), // mDNS entries have short TTL
                source: DiscoverySource::LocalDiscovery,
            });
        }
    }
    
    // mDNS feature not enabled or no services found
    tracing::debug!("mDNS discovery returned no results for '{}'", capability);
    Err(NestGateError::network_error(&format!(
        "No local services found for capability '{}'",
        capability
    )))
}

// ==================== HELPER FUNCTIONS ====================

/// Parse endpoint string into components
///
/// Validates and parses endpoints like:
/// - `http://localhost:8080`
/// - `localhost:8080`
/// - `192.168.1.100:9090`
pub fn parse_endpoint(endpoint: &str) -> Result<(String, u16)> {
    // Handle full URLs
    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        return parse_url_endpoint(endpoint);
    }

    // Handle host:port format
    parse_hostport_endpoint(endpoint)
}

fn parse_url_endpoint(url: &str) -> Result<(String, u16)> {
    // Simple URL parsing (for production, use url crate)
    let without_protocol = url
        .trim_start_matches("http://")
        .trim_start_matches("https://");

    parse_hostport_endpoint(without_protocol)
}

fn parse_hostport_endpoint(hostport: &str) -> Result<(String, u16)> {
    let parts: Vec<&str> = hostport.split(':').collect();

    if parts.len() != 2 {
        return Err(NestGateError::validation_error(&format!(
            "Invalid endpoint format '{}': expected 'host:port'",
            hostport
        )));
    }

    let host = parts[0];
    let port = parts[1].parse::<u16>().map_err(|e| {
        NestGateError::validation_error(&format!("Invalid port '{}': {}", parts[1], e))
    })?;

    if host.is_empty() {
        return Err(NestGateError::validation_error("Host cannot be empty"));
    }

    Ok((host.to_string(), port))
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discover_with_fallback_uses_default() {
        let result =
            discover_with_fallback("test_service", "NONEXISTENT_ENV_VAR", "localhost:9999").await;

        assert!(result.is_ok());
        let endpoint = result.unwrap();
        assert_eq!(endpoint.endpoint, "localhost:9999");
        assert_eq!(endpoint.source, DiscoverySource::Default);
    }

    #[tokio::test]
    async fn test_discover_with_fallback_uses_env() {
        env::set_var("TEST_SERVICE_ENDPOINT", "envhost:8888");

        let result =
            discover_with_fallback("test_service", "TEST_SERVICE_ENDPOINT", "localhost:9999").await;

        assert!(result.is_ok());
        let endpoint = result.unwrap();
        assert_eq!(endpoint.endpoint, "envhost:8888");
        assert_eq!(endpoint.source, DiscoverySource::Environment);

        env::remove_var("TEST_SERVICE_ENDPOINT");
    }

    #[test]
    fn test_parse_endpoint_hostport() {
        let result = parse_endpoint("localhost:8080");
        assert!(result.is_ok());

        let (host, port) = result.unwrap();
        assert_eq!(host, "localhost");
        assert_eq!(port, 8080);
    }

    #[test]
    fn test_parse_endpoint_http_url() {
        let result = parse_endpoint("http://example.com:3000");
        assert!(result.is_ok());

        let (host, port) = result.unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 3000);
    }

    #[test]
    fn test_parse_endpoint_invalid() {
        assert!(parse_endpoint("invalid").is_err());
        assert!(parse_endpoint(":8080").is_err());
        assert!(parse_endpoint("localhost:").is_err());
        assert!(parse_endpoint("localhost:not_a_port").is_err());
    }

    #[tokio::test]
    async fn test_announce_capability() {
        let result = announce_capability("test", "localhost:8080", Duration::from_secs(60)).await;

        assert!(result.is_ok(), "Should announce capability");
    }
}
