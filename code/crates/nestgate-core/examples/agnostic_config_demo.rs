//! Agnostic Configuration Demo
//!
//! Demonstrates how to use capability-based discovery and agnostic configuration.
//!
//! **Run**: `cargo run --example agnostic_config_demo`

use nestgate_core::config::agnostic_config::{migrate_endpoint, migrate_port};
use nestgate_core::config::capability_discovery::{discover_service, discover_with_fallback};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Agnostic Configuration Demo");
    println!("================================\n");

    // Example 1: Migrate a port
    println!("1️⃣  Migrating API Port:");
    println!("   Checking: Capability → Env (API_PORT) → Default (8080)");

    match migrate_port("api", 8080).await {
        Ok(port) => println!("   ✅ API Port: {}\n", port),
        Err(e) => println!("   ❌ Error: {}\n", e),
    }

    // Example 2: Migrate an endpoint
    println!("2️⃣  Migrating Storage Endpoint:");
    println!("   Checking: Capability → Env (STORAGE_URL) → Default");

    match migrate_endpoint("storage", "http://localhost:5000").await {
        Ok(endpoint) => println!("   ✅ Storage: {}\n", endpoint),
        Err(e) => println!("   ❌ Error: {}\n", e),
    }

    // Example 3: Direct capability discovery
    println!("3️⃣  Direct Capability Discovery:");
    println!("   Looking for 'database' service...");

    match discover_service("database").await {
        Ok(endpoint) => {
            println!("   ✅ Found: {}", endpoint.endpoint);
            println!("   📍 Source: {:?}\n", endpoint.source);
        }
        Err(e) => println!("   ℹ️  Not found: {} (expected in dev)\n", e),
    }

    // Example 4: Fallback chain
    println!("4️⃣  Complete Fallback Chain:");
    println!("   Priority: 1) Capability 2) ENV 3) Default");

    match discover_with_fallback("cache", "CACHE_URL", "redis://localhost:6379").await {
        Ok(endpoint) => {
            println!("   ✅ Cache: {}", endpoint.endpoint);
            println!("   📍 Source: {:?}\n", endpoint.source);
        }
        Err(e) => println!("   ❌ Error: {}\n", e),
    }

    // Example 5: Environment variable override
    println!("5️⃣  Environment Variable Override:");
    if let Ok(custom_port) = std::env::var("CUSTOM_PORT") {
        println!("   ✅ CUSTOM_PORT detected: {}", custom_port);
    } else {
        println!("   ℹ️  Set CUSTOM_PORT env var to override");
        println!("   Example: CUSTOM_PORT=9000 cargo run --example agnostic_config_demo");
    }

    println!("\n✨ Demo Complete!");
    println!("\n📖 Key Principles:");
    println!("   • Capability discovery first (runtime)");
    println!("   • Environment variables second (deployment)");
    println!("   • Safe defaults last (development)");
    println!("   • Zero hardcoding in production code");
    println!("   • Self-knowledge only (no primal assumptions)");

    Ok(())
}
