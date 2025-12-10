//! **CAPABILITY-BASED CONFIGURATION EXAMPLES**
//!
//! Practical examples showing migration from hardcoded values to capability-based discovery.

use super::*;
use std::env;

/// Example 1: Simple API server configuration
///
/// Shows how to configure an API server without hardcoding the port.
pub fn example_api_server() -> Result<SocketAddr> {
    // OLD (hardcoded):
    // const API_PORT: u16 = 8080;
    // let addr = SocketAddr::new("0.0.0.0".parse()?, API_PORT);

    // NEW (capability-based):
    let config = CapabilityConfig::from_env()?;
    let addr = config.get_endpoint("api")?;

    println!("API server will listen on: {}", addr);
    Ok(addr)
}

/// Example 2: Multiple service endpoints
///
/// Shows how to configure multiple services without hardcoding.
pub fn example_multiple_services() -> Result<()> {
    // Environment:
    // NESTGATE_CAPABILITIES=api,metrics,health
    // NESTGATE_API_ENDPOINT=0.0.0.0:8080
    // NESTGATE_METRICS_ENDPOINT=0.0.0.0:9090
    // NESTGATE_HEALTH_ENDPOINT=0.0.0.0:8081

    let config = CapabilityConfig::from_env()?;

    let api_addr = config.get_endpoint("api")?;
    let metrics_addr = config.get_endpoint("metrics")?;
    let health_addr = config.get_endpoint("health")?;

    println!("Services configured:");
    println!("  API:     {}", api_addr);
    println!("  Metrics: {}", metrics_addr);
    println!("  Health:  {}", health_addr);

    Ok(())
}

/// Example 3: Development vs Production configuration
///
/// Shows how to use different configurations for different environments.
pub fn example_environment_aware() -> Result<SocketAddr> {
    let mut config = CapabilityConfig::from_env()?;

    // Add environment-appropriate fallbacks
    let fallbacks = match env::var("NESTGATE_ENV").as_deref() {
        Ok("production") => CapabilityDefaults::secure(),
        Ok("development") => CapabilityDefaults::development(),
        _ => CapabilityDefaults::development(),
    };

    config = config.with_fallback(fallbacks)?;

    let addr = config.get_endpoint("api")?;
    Ok(addr)
}

/// Example 4: Primal self-knowledge without hardcoded names
///
/// Shows how to define self-knowledge without hardcoding other primal names.
pub fn example_primal_self_knowledge() -> Result<()> {
    // OLD (hardcoded primal names - BAD!):
    // const BEARDOG_ENDPOINT: &str = "localhost:3000";
    // let security = connect_to_beardog(BEARDOG_ENDPOINT)?;

    // NEW (self-knowledge - GOOD!):
    use crate::self_knowledge::*;

    // Know thyself - NO hardcoded knowledge of other primals!
    let self_knowledge = SelfKnowledge::builder()
        .with_id("nestgate")
        .with_name("NestGate")
        .with_capability("storage")
        .with_capability("zfs-management")
        .build()?;

    println!("Self-knowledge established:");
    println!("  ID: {}", self_knowledge.id);
    println!("  Name: {}", self_knowledge.name);
    println!("  Capabilities: {:?}", self_knowledge.capabilities);
    println!("  (Will discover other primals at runtime by capability)");

    Ok(())
}

/// Example 5: Custom discovery backends
///
/// Shows how to use different discovery mechanisms.
pub fn example_custom_discovery() -> Result<()> {
    // Environment:
    // NESTGATE_DISCOVERY_BACKENDS=dns-srv,mdns
    // NESTGATE_DNS_DOMAIN=nestgate.local

    let config = CapabilityConfig::from_env()?;

    println!("Discovery backends configured:");
    for backend in &config.discovery_backends {
        match backend {
            DiscoveryBackend::DnsSrv { domain } => {
                println!("  - DNS-SRV: {}", domain);
            }
            DiscoveryBackend::MDns { service_type } => {
                println!("  - mDNS: {}", service_type);
            }
            DiscoveryBackend::Consul { address } => {
                println!("  - Consul: {}", address);
            }
            DiscoveryBackend::Kubernetes { namespace } => {
                println!("  - Kubernetes: namespace={}", namespace);
            }
            DiscoveryBackend::Environment => {
                println!("  - Environment variables");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_fallback_config() {
        // Test that we can create config with fallbacks
        let config = CapabilityConfig {
            capabilities: Default::default(),
            fallbacks: Some(CapabilityDefaults::development()),
            discovery_backends: vec![DiscoveryBackend::Environment],
        };

        assert!(config.fallbacks.is_some());
    }

    #[test]
    fn test_environment_defaults() {
        let dev = CapabilityDefaults::development();
        assert_eq!(dev.bind_address, "127.0.0.1");

        let prod = CapabilityDefaults::secure();
        assert_eq!(prod.bind_address, "0.0.0.0");
    }
}
