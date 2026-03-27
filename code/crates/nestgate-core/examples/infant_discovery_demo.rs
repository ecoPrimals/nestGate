//! # Infant Discovery Pattern Demo
//!
//! Demonstrates how a primal starts with **zero knowledge** and discovers its ecosystem.
//!
//! ## Philosophy
//!
//! Each primal is like an "infant" - born with only self-awareness, no knowledge of
//! other primals or infrastructure. It learns about its ecosystem through discovery.
//!
//! ## Run
//!
//! ```bash
//! cargo run --example infant_discovery_demo
//! ```

use anyhow::{Context, Result};
use nestgate_core::discovery_mechanism::DiscoveryBuilder;
use nestgate_core::self_knowledge::SelfKnowledge;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🐣 INFANT DISCOVERY PATTERN DEMO\n");
    println!("═══════════════════════════════════════════════════════════\n");

    // STEP 1: SELF-AWARENESS (Birth)
    // ════════════════════════════════════════════════════════════
    println!("1️⃣  SELF-AWARENESS (Birth)");
    println!("   \"I am born. Who am I? What can I do?\"\n");

    let self_knowledge = SelfKnowledge::builder()
        .with_id("nestgate-001")
        .with_name("nestgate")
        .with_version("0.12.0")
        .with_capability("storage")
        .with_capability("zfs_management")
        .with_endpoint("api", "0.0.0.0:8080".parse::<SocketAddr>()?)
        .with_endpoint("metrics", "0.0.0.0:9090".parse::<SocketAddr>()?)
        .build()
        .context("Failed to build self-knowledge")?;

    println!("   ✅ I am: {}", self_knowledge.name);
    println!("   ✅ I provide capabilities:");
    for cap in &self_knowledge.capabilities {
        println!("      • {}", cap);
    }
    println!("   ✅ I am accessible at:");
    for (name, addr) in &self_knowledge.endpoints {
        println!("      • {}: {}", name, addr);
    }
    println!();

    // STEP 2: DISCOVER MECHANISM (Learning to see)
    // ════════════════════════════════════════════════════════════
    println!("2️⃣  DISCOVERY MECHANISM (Learning to see)");
    println!("   \"How do I find others? What discovery is available?\"\n");

    let discovery = DiscoveryBuilder::new()
        .with_timeout(std::time::Duration::from_secs(5))
        .with_cache_duration(std::time::Duration::from_secs(60))
        .detect()
        .await
        .context("Failed to detect discovery mechanism")?;

    println!("   ✅ Detected mechanism: {}", discovery.mechanism_name());
    println!("   ✅ Ready to discover ecosystem\n");

    // STEP 3: ANNOUNCE SELF (Hello world!)
    // ════════════════════════════════════════════════════════════
    println!("3️⃣  ANNOUNCE SELF (Hello world!)");
    println!("   \"Here I am! I provide storage and ZFS management.\"\n");

    discovery
        .announce(&self_knowledge)
        .await
        .context("Failed to announce self")?;

    println!("   ✅ Announced to ecosystem");
    println!("   ✅ Other primals can now discover me by capability\n");

    // STEP 4: DISCOVER OTHERS (Finding siblings)
    // ════════════════════════════════════════════════════════════
    println!("4️⃣  DISCOVER OTHERS (Finding siblings)");
    println!("   \"Who else is here? What can they do?\"\n");

    // Look for orchestration capability (NOT by name "songbird"!)
    println!("   🔍 Looking for 'orchestration' capability...");
    let orchestrators = discovery
        .find_by_capability("orchestration".to_string())
        .await
        .context("Failed to find orchestrators")?;

    if orchestrators.is_empty() {
        println!("      ℹ️  No orchestrators found (this is expected in demo)");
    } else {
        println!("      ✅ Found {} orchestrator(s):", orchestrators.len());
        for orch in &orchestrators {
            println!("         • {} at {}", orch.name, orch.endpoint);
        }
    }
    println!();

    // Look for security capability (NOT by name "beardog"!)
    println!("   🔍 Looking for 'authentication' capability...");
    let auth_services = discovery
        .find_by_capability("authentication".to_string())
        .await
        .context("Failed to find auth services")?;

    if auth_services.is_empty() {
        println!("      ℹ️  No auth services found (this is expected in demo)");
    } else {
        println!("      ✅ Found {} auth service(s):", auth_services.len());
        for auth in &auth_services {
            println!("         • {} at {}", auth.name, auth.endpoint);
        }
    }
    println!();

    // Look for AI capability (NOT by name "squirrel"!)
    println!("   🔍 Looking for 'ai' capability...");
    let ai_services = discovery
        .find_by_capability("ai".to_string())
        .await
        .context("Failed to find AI services")?;

    if ai_services.is_empty() {
        println!("      ℹ️  No AI services found (this is expected in demo)");
    } else {
        println!("      ✅ Found {} AI service(s):", ai_services.len());
        for ai in &ai_services {
            println!("         • {} at {}", ai.name, ai.endpoint);
        }
    }
    println!();

    // STEP 5: DEMONSTRATE PATTERN SUCCESS
    // ════════════════════════════════════════════════════════════
    println!("5️⃣  PATTERN SUCCESS");
    println!();
    println!("   ✅ Started with ZERO knowledge of:");
    println!("      • Other primals (no hardcoded names)");
    println!("      • Infrastructure (no hardcoded k8s/consul)");
    println!("      • Network topology (no hardcoded URLs)");
    println!();
    println!("   ✅ Discovered through:");
    println!("      • Self-awareness (know only myself)");
    println!("      • Capability queries (NOT name queries)");
    println!("      • Auto-detected mechanism (portable)");
    println!();
    println!("   ✅ Results:");
    println!("      • Sovereignty preserved");
    println!("      • Vendor agnostic");
    println!("      • Infrastructure portable");
    println!("      • Zero compile-time coupling");
    println!();

    // STEP 6: GRACEFUL SHUTDOWN
    // ════════════════════════════════════════════════════════════
    println!("6️⃣  GRACEFUL SHUTDOWN");
    println!("   \"Goodbye! Deregistering...\"\n");

    discovery
        .deregister(self_knowledge.id.as_str())
        .await
        .context("Failed to deregister")?;

    println!("   ✅ Deregistered from ecosystem");
    println!();

    println!("═══════════════════════════════════════════════════════════");
    println!("🎉 INFANT DISCOVERY PATTERN COMPLETE!");
    println!();
    println!("Key Principles Demonstrated:");
    println!("  1. Self-awareness only (no external knowledge at birth)");
    println!("  2. Runtime discovery (not compile-time)");
    println!("  3. Capability-based (not name-based)");
    println!("  4. Vendor agnostic (works anywhere)");
    println!("  5. Sovereignty preserving (no privileged knowledge)");
    println!("═══════════════════════════════════════════════════════════");

    Ok(())
}
