#![allow(
    unused,
    dead_code,
    deprecated,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

//! Example: Ecosystem Integration with Capability Discovery
//!
//! This example demonstrates how NestGate integrates with other ecoPrimals
//! using capability-based runtime discovery (no hardcoded dependencies).

use nestgate_core::Result;
use nestgate_core::primal_discovery::{PrimalDiscovery, SelfKnowledge};

/// Example: NestGate announces its storage capabilities
#[allow(dead_code)]
async fn announce_nestgate_capabilities() -> Result<()> {
    // 1. Define self-knowledge (what NestGate provides)
    let self_knowledge = SelfKnowledge::builder()
        .name("nestgate")
        .capability("zfs_storage")
        .capability("snapshot_management")
        .capability("data_deduplication")
        .capability("compression")
        .capability("object_storage_bridge")
        .endpoint_http(8080)
        .metadata("version", "0.10.0")
        .metadata("max_pools", "unlimited")
        .build();

    // 2. Create discovery service and announce
    let discovery = PrimalDiscovery::new(self_knowledge);
    discovery.announce().await?;

    println!("✅ NestGate capabilities announced to ecosystem");
    Ok(())
}

/// Example: Discover and use ToadStool compute service
#[allow(dead_code)]
async fn use_toadstool_for_compute() -> Result<()> {
    // No hardcoded knowledge of ToadStool!
    // Discover it by capability at runtime

    let self_knowledge = SelfKnowledge::builder().name("nestgate").build();

    let discovery = PrimalDiscovery::new(self_knowledge);

    // Discover compute capability (could be ToadStool, or any other primal)
    let compute_service = discovery.discover_capability("compute_execution").await?;

    println!(
        "✅ Discovered compute service at: {}",
        compute_service
            .primary_endpoint()
            .as_deref()
            .unwrap_or("unknown")
    );

    // Now we can use it (via HTTP, gRPC, etc.)
    // let result = compute_service.execute_job(job_spec).await?;

    Ok(())
}

/// Example: Discover and integrate with Songbird orchestration
#[allow(dead_code)]
async fn integrate_with_songbird() -> Result<()> {
    let self_knowledge = SelfKnowledge::builder()
        .name("nestgate")
        .capability("zfs_storage")
        .build();

    let discovery = PrimalDiscovery::new(self_knowledge);

    // Discover orchestration capability
    let orchestrator = discovery
        .discover_capability("workflow_orchestration")
        .await?;

    println!(
        "✅ Discovered orchestrator: {}",
        orchestrator
            .primary_endpoint()
            .as_deref()
            .unwrap_or("unknown")
    );

    // Register with orchestrator for workflow participation
    // orchestrator.register_service("nestgate", capabilities).await?;

    Ok(())
}

/// Example: Discover and use BearDog for encryption
#[allow(dead_code)]
async fn use_beardog_for_encryption() -> Result<()> {
    let self_knowledge = SelfKnowledge::builder().name("nestgate").build();

    let discovery = PrimalDiscovery::new(self_knowledge);

    // Discover cryptographic capability
    let crypto_service = discovery
        .discover_capability("cryptographic_operations")
        .await?;

    println!(
        "✅ Discovered crypto service: {}",
        crypto_service
            .primary_endpoint()
            .as_deref()
            .unwrap_or("unknown")
    );

    // Use for dataset encryption
    // let encrypted = crypto_service.encrypt(dataset_key, data).await?;

    Ok(())
}

/// Example: Complete ecosystem workflow
///
/// This shows how NestGate participates in a multi-primal workflow
/// without any hardcoded knowledge of other services.
#[allow(dead_code)]
async fn ecosystem_workflow_example() -> Result<()> {
    println!("🌍 Starting ecosystem integration workflow...\n");

    // 1. Announce our capabilities
    println!("1️⃣ Announcing NestGate capabilities...");
    announce_nestgate_capabilities().await?;

    // 2. Discover other services
    println!("\n2️⃣ Discovering ecosystem services...");

    let self_knowledge = SelfKnowledge::builder()
        .name("nestgate")
        .capability("zfs_storage")
        .build();

    let discovery = PrimalDiscovery::new(self_knowledge);

    // Try to discover each service (graceful if not found)
    if let Ok(compute) = discovery.discover_capability("compute_execution").await {
        println!(
            "   ✅ Found compute service: {}",
            compute.primary_endpoint().as_deref().unwrap_or("unknown")
        );
    } else {
        println!("   ℹ️  No compute service available");
    }

    if let Ok(orchestrator) = discovery
        .discover_capability("workflow_orchestration")
        .await
    {
        println!(
            "   ✅ Found orchestrator: {}",
            orchestrator
                .primary_endpoint()
                .as_deref()
                .unwrap_or("unknown")
        );
    } else {
        println!("   ℹ️  No orchestrator available");
    }

    if let Ok(crypto) = discovery
        .discover_capability("cryptographic_operations")
        .await
    {
        println!(
            "   ✅ Found crypto service: {}",
            crypto.primary_endpoint().as_deref().unwrap_or("unknown")
        );
    } else {
        println!("   ℹ️  No crypto service available");
    }

    println!("\n✅ Ecosystem integration complete!");
    println!("   NestGate is ready to work with any discovered services");

    Ok(())
}

/// Main function - demonstrates ecosystem integration
#[tokio::main]
async fn main() -> Result<()> {
    println!("🌍 NestGate Ecosystem Integration Example\n");
    println!("============================================\n");

    println!("This example demonstrates:");
    println!("  1. Self-knowledge announcement");
    println!("  2. Runtime service discovery");
    println!("  3. Capability-based integration");
    println!("  4. Zero hardcoded dependencies\n");

    // Run the complete workflow
    ecosystem_workflow_example().await?;

    println!("\n💡 Key Principles Demonstrated:");
    println!("  ✅ Primals have only self-knowledge");
    println!("  ✅ Services discovered at runtime");
    println!("  ✅ Connect by capability, not by name");
    println!("  ✅ Graceful degradation if service unavailable");
    println!("  ✅ Zero vendor lock-in (primal sovereignty)");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_core::primal_discovery::Endpoint;

    #[tokio::test]
    async fn test_self_knowledge_building() {
        let knowledge = SelfKnowledge::builder()
            .name("nestgate")
            .capability("storage")
            .endpoint_http(8080)
            .build();

        assert_eq!(knowledge.name, "nestgate");
        assert_eq!(knowledge.capabilities.len(), 1);
        assert_eq!(knowledge.endpoints.len(), 1);
    }

    #[test]
    fn test_endpoint_url_generation() {
        let endpoint = Endpoint::http(8080);
        let url = endpoint.url();

        assert!(url.contains("http://"));
        assert!(url.contains("8080"));
    }
}
