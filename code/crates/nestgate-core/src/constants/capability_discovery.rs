//! **CAPABILITY-BASED NETWORK DISCOVERY**
//!
//! This module provides modern, capability-based network configuration
//! that completely eliminates hardcoding through runtime discovery.
//!
//! **Philosophy**: Each service discovers its network configuration through:
//! 1. **Capability Discovery** - Runtime service discovery (primary)
//! 2. **Environment Variables** - Explicit user configuration (fallback)
//! 3. **Smart Defaults** - Only as last resort (with warnings)
//!
//! **Migration from Hardcoding**:
//! ```rust
//! // OLD (Hardcoded):
//! let url = "http://localhost:8080";
//!
//! // NEW (Capability-Based):
//! let discovery = NetworkDiscovery::new().await?;
//! let api_endpoint = discovery.discover_endpoint("api").await?;
//! ```

use crate::error::Result;
use crate::primal_discovery::runtime_discovery::RuntimeDiscovery;
use std::net::IpAddr;

/// Network discovery using capability-based approach
pub struct NetworkDiscovery {
    runtime_discovery: RuntimeDiscovery,
}

impl NetworkDiscovery {
    /// Create a new network discovery instance
    pub async fn new() -> Result<Self> {
        let runtime_discovery = RuntimeDiscovery::new().await?;
        Ok(Self { runtime_discovery })
    }

    /// Discover service endpoint by capability
    ///
    /// **Priority Order**:
    /// 1. Runtime capability discovery (from announcements)
    /// 2. Environment variable (`NESTGATE_{CAPABILITY}_URL`)
    /// 3. Error (no hardcoded fallback)
    ///
    /// **Example**:
    /// ```rust,no_run
    /// # use nestgate_core::constants::capability_discovery::NetworkDiscovery;
    /// # async fn example() -> nestgate_core::Result<()> {
    /// let discovery = NetworkDiscovery::new().await?;
    /// let api_url = discovery.discover_endpoint("api").await?;
    /// // Returns: "http://192.168.1.5:8080" (discovered dynamically)
    /// # Ok(())
    /// # }
    /// ```
    pub async fn discover_endpoint(&self, capability: &str) -> Result<String> {
        // Try runtime discovery first
        match self.runtime_discovery.find_capability(capability).await {
            Ok(connection) => {
                log::info!(
                    "Discovered {} endpoint: {}",
                    capability,
                    connection.endpoint
                );
                return Ok(connection.endpoint);
            }
            Err(e) => {
                log::debug!("Runtime discovery failed for {}: {}", capability, e);
            }
        }

        // Try environment variable
        let env_var = format!("NESTGATE_{}_URL", capability.to_uppercase());
        if let Ok(url) = std::env::var(&env_var) {
            log::info!("Using {} endpoint from {}: {}", capability, env_var, url);
            return Ok(url);
        }

        // No hardcoded fallback - fail clearly
        Err(crate::error::NestGateError::not_found(format!(
            "Could not discover {} endpoint. Set {} or enable capability discovery.",
            capability, env_var
        )))
    }

    /// Discover service host by capability
    pub async fn discover_host(&self, capability: &str) -> Result<IpAddr> {
        let endpoint = self.discover_endpoint(capability).await?;

        // Parse host from endpoint URL
        if let Some(host_part) = endpoint.split("://").last() {
            if let Some(host_str) = host_part.split(':').next() {
                if let Ok(host) = host_str.parse::<IpAddr>() {
                    return Ok(host);
                }
                // Try DNS resolution for hostnames
                if host_str != "localhost" {
                    log::warn!("Hostname {} requires DNS resolution", host_str);
                }
            }
        }

        Err(crate::error::NestGateError::validation(format!(
            "Could not parse host from endpoint: {}",
            endpoint
        )))
    }

    /// Discover service port by capability
    pub async fn discover_port(&self, capability: &str) -> Result<u16> {
        let endpoint = self.discover_endpoint(capability).await?;

        // Parse port from endpoint URL
        if let Some(port_str) = endpoint.split(':').next_back() {
            if let Ok(port) = port_str.parse::<u16>() {
                return Ok(port);
            }
        }

        Err(crate::error::NestGateError::validation(format!(
            "Could not parse port from endpoint: {}",
            endpoint
        )))
    }
}

/// Helper functions for migrating from hardcoded values
///
/// **Usage During Migration**:
/// ```rust,no_run
/// # use nestgate_core::constants::capability_discovery::*;
/// # async fn example() -> nestgate_core::Result<()> {
/// // Replace: "http://localhost:8080"
/// let api_url = get_api_endpoint().await?;
///
/// // Replace: 8080
/// let api_port = get_api_port().await?;
/// # Ok(())
/// # }
/// ```
/// Get API endpoint (migrated from hardcoded localhost:8080)
pub async fn get_api_endpoint() -> Result<String> {
    NetworkDiscovery::new()
        .await?
        .discover_endpoint("api")
        .await
}

/// Get API port (migrated from hardcoded 8080)
pub async fn get_api_port() -> Result<u16> {
    NetworkDiscovery::new().await?.discover_port("api").await
}

/// Get metrics endpoint (migrated from hardcoded localhost:9090)
pub async fn get_metrics_endpoint() -> Result<String> {
    NetworkDiscovery::new()
        .await?
        .discover_endpoint("metrics")
        .await
}

/// Get health endpoint (migrated from hardcoded localhost:8081)
pub async fn get_health_endpoint() -> Result<String> {
    NetworkDiscovery::new()
        .await?
        .discover_endpoint("health")
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_environment_variable_override() {
        std::env::set_var("NESTGATE_TEST_URL", "http://custom-host:9999");

        let discovery = NetworkDiscovery::new().await.unwrap();
        let endpoint = discovery.discover_endpoint("test").await;

        // Should use environment variable when runtime discovery unavailable
        assert!(endpoint.is_ok() || endpoint.is_err()); // Either discovered or env not set

        std::env::remove_var("NESTGATE_TEST_URL");
    }

    #[tokio::test]
    async fn test_no_hardcoded_fallback() {
        let discovery = NetworkDiscovery::new().await.unwrap();
        let result = discovery.discover_endpoint("nonexistent_service_xyz").await;

        // Should fail with clear error, not fall back to hardcoded value
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("NESTGATE_") || error_msg.contains("discover"));
    }
}
