//! Universal Primal Integration Demo
//! This demo showcases how NestGate integrates with any primal ecosystem:
//! - Security Primals (like BearDog)
//! - AI Primals (like Squirrel)
//! - Orchestration Primals (like Songbird)
//! - Compute Primals (like Toadstool)
//!
//! The universal architecture allows seamless integration with any primal,
//! regardless of the specific implementation or vendor.

use anyhow::Result;
use nestgate_api::universal_primal::{NestGateStoragePrimal, StoragePrimalRequest, StoragePrimalHealth, NestGatePrimalConfig, StorageRequestType, StoragePrimalProvider};
use serde_json::json;
use std::time::SystemTime;
use tracing::{info, warn, error};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Initialize NestGate Storage Primal
    let config = NestGatePrimalConfig {
        host: "localhost".to_string(),
        port: 8080,
        discovery_enabled: false,
        primal_registry_endpoint: None,
    };
    let nestgate_primal = NestGateStoragePrimal::new(config).await?;

    println!("🎯 === Universal Primal Integration Architecture Demo ===\n");

    // Demo 1: Basic Primal Discovery
    info!("📡 Demo 1: Basic Primal Discovery");
    demo_primal_discovery(&nestgate_primal).await?;

    // Demo 2: Health Monitoring
    info!("❤️ Demo 2: Health Monitoring");
    demo_health_monitoring(&nestgate_primal).await?;

    // Demo 3: Storage Request Creation
    info!("💾 Demo 3: Storage Request Creation");
    demo_storage_request_creation().await?;

    println!("\n🎉 Demo completed successfully!");
    Ok(())
}

async fn demo_primal_discovery(_nestgate: &NestGateStoragePrimal) -> Result<()> {
    info!("🔍 Demonstrating primal discovery...");

    // Simulate primal discovery
    info!("✅ Discovered security primal at endpoint: https://security-primal.example.com");
    info!("✅ Discovered AI primal at endpoint: https://ai-primal.example.com");
    info!("✅ Discovered orchestration primal at endpoint: https://orchestration-primal.example.com");
    info!("✅ Discovered compute primal at endpoint: https://compute-primal.example.com");

    Ok(())
}

async fn demo_health_monitoring(nestgate: &NestGateStoragePrimal) -> Result<()> {
    info!("🏥 Demonstrating health monitoring...");

    // Check health using the trait
    let health = nestgate.health_check().await;
    
    match health {
        StoragePrimalHealth::Healthy {
            uptime,
            storage_capacity,
            performance_metrics,
        } => {
            info!("✅ NestGate is healthy");
            info!("   Uptime: {:?}", uptime);
            info!("   Storage: {:.1}% used", (storage_capacity.used_bytes as f64 / storage_capacity.total_bytes as f64) * 100.0);
            info!("   Performance metrics: {:?}", performance_metrics);
        }
        StoragePrimalHealth::Degraded {
            issues,
            capacity_remaining,
        } => {
            warn!("⚠️  NestGate is degraded");
            warn!("   Issues: {:?}", issues);
            warn!("   Capacity remaining: {:.1}%", capacity_remaining * 100.0);
        }
        StoragePrimalHealth::Unhealthy {
            reason,
            last_healthy,
        } => {
            error!("❌ NestGate is unhealthy");
            error!("   Reason: {}", reason);
            error!("   Last healthy: {:?}", last_healthy);
        }
    }

    Ok(())
}

async fn demo_storage_request_creation() -> Result<()> {
    info!("🔧 Demonstrating storage request creation...");
    
    // Create a sample storage request
        let request = StoragePrimalRequest {
        request_id: Uuid::new_v4(),
            from_primal: "nestgate".to_string(),
        to_primal: "security-primal".to_string(),
            request_type: StorageRequestType::CapabilityQuery,
        payload: json!({
            "query_type": "encryption_capabilities",
            "volume_ids": ["vol-123", "vol-456"],
            "requirements": {
                "encryption": "AES-256",
                "key_management": "hsm",
                "compliance": "fips-140-2"
            }
        }),
        timestamp: SystemTime::now(),
        };

    info!("✅ Created storage request: {}", request.request_id);
    info!("   From: {} -> To: {}", request.from_primal, request.to_primal);
    info!("   Type: {:?}", request.request_type);
    info!("   Payload: {}", request.payload);
    
    Ok(())
}
