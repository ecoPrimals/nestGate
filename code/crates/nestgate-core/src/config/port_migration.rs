//! Port Migration Helpers - MODERN Capability-Based Discovery
//!
//! **ARCHITECTURAL EVOLUTION COMPLETE**: Now uses unified capability resolver system.
//!
//! **CHANGES**:
//! - ✅ Uses CapabilityResolver trait (works with any registry)
//! - ✅ No hardcoded fallbacks (fail-fast with helpful errors)
//! - ✅ Environment + discovery unified
//! - ✅ Sovereignty-compliant (self-knowledge + runtime discovery)
//!
//! # Philosophy
//!
//! **Fail Fast**: If service isn't configured, error immediately with helpful message.
//! **No Assumptions**: Only explicit configuration or runtime discovery.
//! **Self-Knowledge**: Primals know themselves, discover others at runtime.
//!
//! # Usage
//!
//! ```rust,ignore
//! use nestgate_core::config::port_migration::resolve_capability_port;
//! use nestgate_core::unified_capabilities::UnifiedCapability;
//! use nestgate_core::capability_resolver::EnvironmentResolver;
//!
//! // Create resolver (can be any CapabilityResolver implementation)
//! let resolver = EnvironmentResolver::new();
//!
//! // Resolve port - errors if not configured
//! let port = resolve_capability_port(
//!     &UnifiedCapability::HttpApi,
//!     &resolver
//! ).await?;
//! ```
//!
//! # Configuration
//!
//! Set environment variables for capabilities:
//! ```bash
//! export NESTGATE_CAPABILITY_HTTP_API_ENDPOINT="http://localhost:8080"
//! export NESTGATE_CAPABILITY_STORAGE_ENDPOINT="http://localhost:9000"
//! ```

use crate::capability_resolver::CapabilityResolver;
use crate::unified_capabilities::{
    CapabilityPortResolver, CapabilityResolutionError, UnifiedCapability,
};

/// Resolve port for HTTP API service
///
/// **NO FALLBACK**: Errors if service not configured or discovered.
///
/// # Configuration
/// Set `NESTGATE_CAPABILITY_HTTP_API_ENDPOINT` environment variable
///
/// # Example
/// ```rust,ignore
/// use nestgate_core::capability_resolver::EnvironmentResolver;
/// let resolver = EnvironmentResolver::new();
/// let port = get_api_port_migrated(&resolver).await?;
/// ```
pub async fn get_api_port_migrated<R: CapabilityResolver>(
    resolver: &R,
) -> Result<u16, CapabilityResolutionError> {
    CapabilityPortResolver::resolve_port(&UnifiedCapability::HttpApi, resolver).await
}

/// Resolve port for metrics/monitoring service
///
/// **NO FALLBACK**: Errors if service not configured or discovered.
///
/// # Configuration
/// Set `NESTGATE_CAPABILITY_METRICS_ENDPOINT` environment variable
pub async fn get_metrics_port_migrated<R: CapabilityResolver>(
    resolver: &R,
) -> Result<u16, CapabilityResolutionError> {
    CapabilityPortResolver::resolve_port(&UnifiedCapability::Metrics, resolver).await
}

/// Resolve port for health check service
///
/// **NO FALLBACK**: Errors if service not configured or discovered.
///
/// # Configuration
/// Set `NESTGATE_CAPABILITY_HEALTH_CHECK_ENDPOINT` environment variable
pub async fn get_health_port_migrated<R: CapabilityResolver>(
    resolver: &R,
) -> Result<u16, CapabilityResolutionError> {
    CapabilityPortResolver::resolve_port(&UnifiedCapability::HealthCheck, resolver).await
}

/// Resolve port for admin interface
///
/// **NO FALLBACK**: Errors if service not configured or discovered.
///
/// # Configuration
/// Set `NESTGATE_CAPABILITY_CONFIGURATION_ENDPOINT` environment variable
pub async fn get_admin_port_migrated<R: CapabilityResolver>(
    resolver: &R,
) -> Result<u16, CapabilityResolutionError> {
    CapabilityPortResolver::resolve_port(&UnifiedCapability::Configuration, resolver).await
}

/// Resolve full endpoint for any capability
///
/// This is the MODERN way - discovers services by what they can do, not who they are.
///
/// # Example
/// ```rust,ignore
/// use nestgate_core::config::port_migration::resolve_capability_endpoint;
/// use nestgate_core::unified_capabilities::UnifiedCapability;
/// use nestgate_core::capability_resolver::EnvironmentResolver;
///
/// let resolver = EnvironmentResolver::new();
/// let endpoint = resolve_capability_endpoint(
///     &UnifiedCapability::Storage,
///     &resolver
/// ).await?;
/// // Returns: "http://discovered-host:discovered-port"
/// ```
pub async fn resolve_capability_endpoint<R: CapabilityResolver>(
    capability: &UnifiedCapability,
    resolver: &R,
) -> Result<String, CapabilityResolutionError> {
    CapabilityPortResolver::resolve_endpoint(capability, resolver).await
}

/// Generic capability-based port resolution
///
/// **USE THIS for new code** - it's the modern, capability-based approach.
///
/// # Arguments
/// * `capability` - What capability you need (not which primal)
/// * `resolver` - Any CapabilityResolver implementation
///
/// # Returns
/// Port if discovered, clear error with configuration hints if not
///
/// # Example
/// ```rust,ignore
/// use nestgate_core::capability_resolver::CompositeResolver;
///
/// // Create resolver chain (registry -> environment)
/// let resolver = CompositeResolver::default_chain(Some(&registry));
///
/// // Modern: Ask for capability, not primal name
/// let port = resolve_capability_port(
///     &UnifiedCapability::Storage,
///     &resolver
/// ).await?;
/// ```
pub async fn resolve_capability_port<R: CapabilityResolver>(
    capability: &UnifiedCapability,
    resolver: &R,
) -> Result<u16, CapabilityResolutionError> {
    CapabilityPortResolver::resolve_port(capability, resolver).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability_resolver::EnvironmentResolver;

    #[tokio::test]
    async fn test_api_port_from_env() {
        // Clean environment first
        std::env::remove_var("NESTGATE_CAPABILITY_HTTP_API_ENDPOINT");

        std::env::set_var(
            "NESTGATE_CAPABILITY_HTTP_API_ENDPOINT",
            "http://localhost:9999",
        );
        let resolver = EnvironmentResolver::new();
        let port = get_api_port_migrated(&resolver).await;
        assert!(port.is_ok(), "Expected Ok, got: {:?}", port);
        assert_eq!(port.unwrap(), 9999);
        std::env::remove_var("NESTGATE_CAPABILITY_HTTP_API_ENDPOINT");
    }

    #[tokio::test]
    async fn test_api_port_no_fallback_on_missing() {
        std::env::remove_var("NESTGATE_CAPABILITY_HTTP_API_ENDPOINT");
        let resolver = EnvironmentResolver::new();
        let port = get_api_port_migrated(&resolver).await;
        assert!(port.is_err()); // NO FALLBACK - fail fast
    }

    #[tokio::test]
    async fn test_metrics_port_from_env() {
        std::env::set_var(
            "NESTGATE_CAPABILITY_METRICS_ENDPOINT",
            "http://localhost:19090",
        );
        let resolver = EnvironmentResolver::new();
        let port = get_metrics_port_migrated(&resolver).await;
        assert!(port.is_ok());
        assert_eq!(port.unwrap(), 19090);
        std::env::remove_var("NESTGATE_CAPABILITY_METRICS_ENDPOINT");
    }

    #[tokio::test]
    async fn test_resolve_capability_port() {
        std::env::set_var(
            "NESTGATE_CAPABILITY_STORAGE_ENDPOINT",
            "http://storage:8888",
        );
        let resolver = EnvironmentResolver::new();
        let port = resolve_capability_port(&UnifiedCapability::Storage, &resolver).await;
        assert!(port.is_ok());
        assert_eq!(port.unwrap(), 8888);
        std::env::remove_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT");
    }

    #[tokio::test]
    async fn test_error_message_includes_hint() {
        std::env::remove_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT");
        let resolver = EnvironmentResolver::new();
        let result = resolve_capability_port(&UnifiedCapability::Storage, &resolver).await;
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("NESTGATE_CAPABILITY_STORAGE_ENDPOINT"));
    }

    #[tokio::test]
    async fn test_resolve_endpoint() {
        std::env::set_var(
            "NESTGATE_CAPABILITY_HTTP_API_ENDPOINT",
            "https://api.example.com:443",
        );
        let resolver = EnvironmentResolver::new();
        let endpoint = resolve_capability_endpoint(&UnifiedCapability::HttpApi, &resolver).await;
        assert!(endpoint.is_ok());
        // URL reconstructs with port, even if it's the default for the protocol
        let url = endpoint.unwrap();
        assert!(url.starts_with("https://api.example.com"));
        assert!(url.contains("443"));
        std::env::remove_var("NESTGATE_CAPABILITY_HTTP_API_ENDPOINT");
    }
}
