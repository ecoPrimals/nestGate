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

//! Example: Migrating from Hardcoded Values to Capability-Based Discovery
//!
//! This example demonstrates how to evolve from hardcoded constants
//! to modern, capability-based service discovery.

use nestgate_core::config::capability_based::{
    CapabilityConfigBuilder, FallbackMode, PrimalCapability,
};
use nestgate_core::error::Result;

/// ❌ OLD APPROACH: Hardcoded constants (ANTI-PATTERN)
mod old_hardcoded_approach {
    // ❌ BAD: Hardcoded primal URLs (These are examples of what NOT to do)
    #[allow(dead_code)]
    pub const BEARDOG_URL: &str = "http://localhost:3000";
    #[allow(dead_code)]
    pub const SONGBIRD_URL: &str = "http://localhost:8080";
    #[allow(dead_code)]
    pub const STORAGE_PORT: u16 = 9000;
    #[allow(dead_code)]
    pub const API_PORT: u16 = 8080;

    #[allow(dead_code)]
    pub async fn connect_to_security() -> Result<(), String> {
        // ❌ BAD: Directly using hardcoded URL
        let _client = format!("Connecting to {}", BEARDOG_URL);
        Ok(())
    }
}

/// ✅ NEW APPROACH: Capability-Based Discovery (CORRECT PATTERN)
mod new_capability_approach {
    use super::*;

    pub async fn connect_to_security() -> Result<()> {
        // ✅ GOOD: Discover security service by capability
        let config = CapabilityConfigBuilder::new()
            .with_fallback_mode(FallbackMode::GracefulDegradation)
            .build()?;

        // Discover authentication service (could be BearDog, or any other primal)
        let auth_service = config.discover(PrimalCapability::Security).await?;

        println!("✅ Discovered auth service at: {}", auth_service.endpoint);
        Ok(())
    }

    pub async fn connect_to_storage() -> Result<()> {
        // ✅ GOOD: Discover storage by capability
        let config = CapabilityConfigBuilder::new().build()?;

        let storage_service = config.discover(PrimalCapability::Storage).await?;

        println!(
            "✅ Discovered storage service at: {}",
            storage_service.endpoint
        );
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔄 Hardcoding Migration Example\n");

    // Set up environment for demonstration
    nestgate_core::env_process::set_var(
        "NESTGATE_CAPABILITY_AUTHENTICATION_ENDPOINT",
        "127.0.0.1:3000",
    );
    nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", "127.0.0.1:9000");

    println!("❌ OLD APPROACH: Hardcoded values");
    println!(
        "   - Hardcoded URL: {}",
        old_hardcoded_approach::BEARDOG_URL
    );
    println!(
        "   - Hardcoded Port: {}",
        old_hardcoded_approach::STORAGE_PORT
    );
    println!("   - Violates sovereignty (knows other primals at compile-time)");
    println!("   - Can't adapt to different deployments\n");

    println!("✅ NEW APPROACH: Capability-based discovery");
    new_capability_approach::connect_to_security().await?;
    new_capability_approach::connect_to_storage().await?;
    println!("   - No hardcoded knowledge of other primals");
    println!("   - Services discovered at runtime");
    println!("   - Sovereignty-compliant");
    println!("   - Adapts to any deployment\n");

    println!("📋 Migration Steps:");
    println!("   1. Replace const BEARDOG_URL with capability discovery");
    println!("   2. Set NESTGATE_CAPABILITY_AUTHENTICATION_ENDPOINT env var");
    println!("   3. Use config.discover(PrimalCapability::Authentication)");
    println!("   4. Service endpoint resolved at runtime!");

    Ok(())
}
