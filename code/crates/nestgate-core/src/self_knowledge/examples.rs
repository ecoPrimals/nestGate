//! Examples showing evolution from hardcoded to capability-based discovery

#![allow(dead_code, unused_variables)]

use super::{SelfKnowledge, PrimalDiscovery, InMemoryBackend};
use anyhow::Result;

/// ❌ OLD PATTERN: Hardcoded primal dependencies
///
/// **Problems**:
/// - Violates sovereignty (hardcoded knowledge of other primals)
/// - Breaks if orchestrator moves or scales
/// - Can't handle multiple orchestrators
/// - Compile-time coupling between primals
mod old_hardcoded_pattern {
    use anyhow::Result;
    
    // ❌ Hardcoded URL - anti-pattern for demonstration
    // In production, use environment-driven discovery:
    // - Use service discovery (mDNS, Consul, k8s service names)
    // - Or environment variables: ORCHESTRATOR_URL, AI_SERVICE_URL, etc.
    const ORCHESTRATOR_URL: &str = "http://orchestrator:8080";  // Demo only
    const AI_SERVICE_URL: &str = "http://ai-service:9000";      // Demo only
    const SECURITY_SERVICE_URL: &str = "http://beardog:8443";   // Demo only
    
    pub async fn connect_to_orchestrator() -> Result<()> {
        // ❌ Directly uses hardcoded URL
        println!("Connecting to orchestrator at {}", ORCHESTRATOR_URL);
        // let client = HttpClient::new();
        // client.connect(ORCHESTRATOR_URL).await?;
        Ok(())
    }
    
    pub async fn connect_to_ai() -> Result<()> {
        // ❌ Hardcoded AI service location
        println!("Connecting to AI at {}", AI_SERVICE_URL);
        Ok(())
    }
}

/// ⚠️ INTERMEDIATE: Environment-driven (better, but not sovereign)
///
/// **Improvements**:
/// - No compile-time hardcoding
/// - Can be configured per environment
///
/// **Still has issues**:
/// - Still requires manual configuration
/// - No automatic discovery
/// - Can't handle dynamic topology changes
mod environment_driven_pattern {
    use anyhow::{Result, Context};
    use std::env;
    
    pub async fn connect_to_orchestrator() -> Result<()> {
        // ⚠️ Better: Uses environment variable
        let url = env::var("ORCHESTRATOR_URL")
            .context("ORCHESTRATOR_URL not set")?;
        
        println!("Connecting to orchestrator at {}", url);
        Ok(())
    }
    
    pub async fn connect_to_service(service_name: &str) -> Result<()> {
        // ⚠️ Still requires knowing the service name
        let env_var = format!("{}_URL", service_name.to_uppercase());
        let url = env::var(&env_var)
            .context(format!("{} not set", env_var))?;
        
        println!("Connecting to {} at {}", service_name, url);
        Ok(())
    }
}

/// ✅ NEW PATTERN: Capability-based discovery (sovereign and flexible)
///
/// **Benefits**:
/// - No hardcoded primal names or addresses
/// - Automatic discovery at runtime
/// - Handles multiple providers gracefully
/// - Survives topology changes
/// - True sovereignty between primals
mod capability_based_pattern {
    use super::*;
    
    pub async fn setup_discovery() -> Result<PrimalDiscovery> {
        // 1. Define what this primal knows about ITSELF
        let self_knowledge = SelfKnowledge::builder()
            .with_id("nestgate")
            .with_name("NestGate Storage")
            .with_version(env!("CARGO_PKG_VERSION"))
            .with_capability("storage")
            .with_capability("zfs")
            .with_capability("nas")
            .with_endpoint("api", format!("0.0.0.0:{}", crate::constants::network_hardcoded::get_api_port()).parse()?)
            .with_endpoint("metrics", format!("0.0.0.0:{}", crate::constants::network_hardcoded::get_metrics_port()).parse()?)
            .build()?;
        
        // 2. Create discovery service
        let mut discovery = PrimalDiscovery::new(self_knowledge);
        
        // 3. Add discovery backend(s)
        // In production: mDNS, Consul, Kubernetes, etc.
        // For testing: InMemoryBackend
        discovery.add_backend(Box::new(InMemoryBackend::new()));
        
        // 4. Announce our presence
        discovery.announce().await?;
        
        Ok(discovery)
    }
    
    /// ✅ Query by capability, not by name
    pub async fn connect_to_orchestration_service(
        discovery: &PrimalDiscovery
    ) -> Result<()> {
        // Find ANY primal that provides orchestration capability
        let providers = discovery
            .find_capability("orchestration")
            .await?;
        
        if providers.is_empty() {
            // Gracefully handle absence
            println!("No orchestration service available, using local mode");
            return Ok(());
        }
        
        // Use the first available (or implement load balancing)
        let provider = &providers[0];
        println!(
            "Connecting to orchestration service '{}' at {:?}",
            provider.name,
            provider.endpoints
        );
        
        // Connect to discovered endpoint
        if let Some(api_endpoint) = provider.endpoints.get("api") {
            println!("API endpoint: {}", api_endpoint);
            // let client = HttpClient::new();
            // client.connect(api_endpoint).await?;
        }
        
        Ok(())
    }
    
    /// ✅ Handle multiple providers (load balancing, redundancy)
    pub async fn connect_to_ai_service(
        discovery: &PrimalDiscovery
    ) -> Result<()> {
        let ai_providers = discovery
            .find_capability("ai")
            .await?;
        
        match ai_providers.len() {
            0 => {
                println!("No AI service available, using fallback logic");
                Ok(())
            }
            1 => {
                println!("Using AI service: {}", ai_providers[0].name);
                Ok(())
            }
            _ => {
                println!("Multiple AI services available, load balancing:");
                for provider in &ai_providers {
                    println!("  - {} (health: {:?})", provider.name, provider.health);
                }
                // Implement load balancing logic here
                Ok(())
            }
        }
    }
    
    /// ✅ Dynamic discovery - works even if services come/go
    pub async fn periodic_discovery_refresh(
        discovery: &PrimalDiscovery
    ) -> Result<()> {
        loop {
            // Re-discover all primals periodically
            let all = discovery.all_primals().await;
            
            println!("Currently discovered primals:");
            for primal in all {
                println!(
                    "  - {} ({}) with capabilities: {:?}",
                    primal.name,
                    primal.id,
                    primal.capabilities
                );
            }
            
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }
    }
}

/// Complete example showing the evolution
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn example_capability_based_discovery() -> Result<()> {
        // Setup discovery
        let discovery = capability_based_pattern::setup_discovery().await?;
        
        // Simulate other primals announcing themselves
        let orchestrator = SelfKnowledge::builder()
            .with_id("orchestrator")
            .with_name("Universal Orchestrator")
            .with_capability("orchestration")
            .with_endpoint("api", format!("10.0.1.100:{}", crate::constants::network_hardcoded::get_api_port()).parse()?)
            .build()?;
        
        let backend = InMemoryBackend::new();
        backend.announce(&orchestrator).await?;
        
        // Now we can discover by capability
        let providers = discovery
            .find_capability("orchestration")
            .await?;
        
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].id.as_str(), "orchestrator");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn example_multiple_providers() -> Result<()> {
        let discovery = capability_based_pattern::setup_discovery().await?;
        let backend = InMemoryBackend::new();
        
        // Multiple AI services available
        for i in 1..=3 {
            let ai = SelfKnowledge::builder()
                .with_id(format!("ai-{}", i))
                .with_name(format!("AI Service {}", i))
                .with_capability("ai")
                .with_endpoint("api", format!("10.0.1.{}:9000", 100 + i).parse()?)
                .build()?;
            
            backend.announce(&ai).await?;
        }
        
        // Discovery finds all of them
        let ai_services = discovery
            .find_capability("ai")
            .await?;
        
        assert_eq!(ai_services.len(), 3);
        
        // Can implement load balancing, failover, etc.
        for service in ai_services {
            println!("Available AI: {} at {:?}", service.name, service.endpoints);
        }
        
        Ok(())
    }
}

