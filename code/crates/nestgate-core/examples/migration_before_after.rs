// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Migration Before/After Example
//!
//! Shows side-by-side comparison of hardcoded vs agnostic patterns.
//!
//! **Run**: `cargo run --example migration_before_after`

#![allow(dead_code)]
#![allow(unused_imports)]

// ============================================================================
// ❌ BEFORE: Hardcoded Pattern
// ============================================================================

mod before_hardcoded {
    pub struct ServerConfig {
        pub api_port: u16,
        pub health_port: u16,
    }

    impl ServerConfig {
        pub fn new() -> Self {
            Self {
                api_port: 8080,    // ❌ Hardcoded
                health_port: 8443, // ❌ Hardcoded
            }
        }

        pub fn start_server(&self) -> String {
            format!("Starting server on port {}", self.api_port)
        }
    }
}

// ============================================================================
// ✅ AFTER: Agnostic Pattern
// ============================================================================

mod after_agnostic {
    use nestgate_core::config::agnostic_config::migrate_port;
    use nestgate_core::Result;

    pub struct ServerConfig {
        pub api_port: u16,
        pub health_port: u16,
    }

    impl ServerConfig {
        pub async fn discover() -> Result<Self> {
            Ok(Self {
                // ✅ Capability → Env → Default
                api_port: migrate_port("api", 8080).await?,
                health_port: migrate_port("health", 8443).await?,
            })
        }

        pub fn start_server(&self) -> String {
            format!("Starting server on port {} (discovered)", self.api_port)
        }
    }
}

// ============================================================================
// Demo Runner
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Migration Before/After Example");
    println!("===================================\n");

    // Before: Hardcoded
    println!("❌ BEFORE (Hardcoded):");
    let old_config = before_hardcoded::ServerConfig::new();
    println!("   API Port: {} (hardcoded)", old_config.api_port);
    println!("   Health Port: {} (hardcoded)", old_config.health_port);
    println!("   Problem: No flexibility, vendor-coupled\n");

    // After: Agnostic
    println!("✅ AFTER (Agnostic):");
    let new_config = after_agnostic::ServerConfig::discover().await?;
    println!("   API Port: {} (discovered)", new_config.api_port);
    println!("   Health Port: {} (discovered)", new_config.health_port);
    println!("   Benefit: Capability-based, environment-driven\n");

    // Show usage
    println!("📖 Usage:");
    println!("   Default:     cargo run --example migration_before_after");
    println!("   Override:    API_PORT=9000 cargo run --example migration_before_after");
    println!("   Production:  Uses capability discovery automatically\n");

    println!("✨ Migration principles:");
    println!("   1. Keep same fallback values (8080, 8443)");
    println!("   2. Add capability discovery first");
    println!("   3. Environment variables second");
    println!("   4. Safe defaults last");
    println!("   5. Maintain backward compatibility");

    Ok(())
}
