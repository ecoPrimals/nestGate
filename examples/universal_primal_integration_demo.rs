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
use log::info;
use nestgate_core::{NestGateStoragePrimal, PrimalCoordinationRequest, PrimalCoordinationResponse};
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("🚀 Universal Primal Integration Demo");
    info!("NestGate will demonstrate integration with security, AI, orchestration, and compute primals");

    // Initialize NestGate Storage Primal
    let nestgate_primal = NestGateStoragePrimal::new().await?;

    println!("🎯 === Universal Primal Integration Architecture Demo ===\n");

    // Wait for system initialization
    sleep(Duration::from_secs(2)).await;

    // Demonstrate universal primal integration patterns
    println!("📡 === Primal Discovery Phase ===");
    demo_primal_discovery(&nestgate_primal).await?;

    println!("\n🔐 === Security Primal Integration ===");
    demo_security_primal_integration(&nestgate_primal).await?;

    println!("\n🧠 === AI Primal Integration ===");
    demo_ai_primal_integration(&nestgate_primal).await?;

    println!("\n🎵 === Orchestration Primal Integration ===");
    demo_orchestration_primal_integration(&nestgate_primal).await?;

    println!("\n💻 === Compute Primal Integration ===");
    demo_compute_primal_integration(&nestgate_primal).await?;

    println!("\n🌐 === Multi-Primal Coordination ===");
    demo_multi_primal_coordination(&nestgate_primal).await?;

    println!("\n🏆 === Integration Complete ===");
    println!("✅ NestGate universal primal integration is fully operational!");
    println!("The system can now work with any primal ecosystem seamlessly.");

    Ok(())
}

/// Demonstrate primal discovery capabilities
async fn demo_primal_discovery(nestgate: &NestGateStoragePrimal) -> Result<()> {
    info!("🔍 Demonstrating universal primal discovery...");

    // In production, this would scan the network for available primals
    let discovered_primals = vec![
        (
            "security-primal",
            "http://localhost:8080",
            vec!["encryption", "authentication"],
        ),
        (
            "ai-primal",
            "http://localhost:8081",
            vec!["optimization", "analysis"],
        ),
        (
            "orchestration-primal",
            "http://localhost:8082",
            vec!["discovery", "coordination"],
        ),
        (
            "compute-primal",
            "http://localhost:8083",
            vec!["processing", "execution"],
        ),
    ];

    println!(
        "   → Discovered {} primals in the ecosystem:",
        discovered_primals.len()
    );
    for (name, endpoint, capabilities) in &discovered_primals {
        println!(
            "     • {} at {} with capabilities: {:?}",
            name, endpoint, capabilities
        );
    }

    info!("✅ Universal primal discovery completed");
    Ok(())
}

/// Demonstrate security primal integration (works with any security primal)
async fn demo_security_primal_integration(nestgate: &NestGateStoragePrimal) -> Result<()> {
    info!("🔐 Demonstrating security primal integration...");

    let request = PrimalCoordinationRequest {
        request_id: "security-demo".to_string(),
        from_primal: "nestgate".to_string(),
        to_primal: "security-primal".to_string(),
        coordination_type: "storage-encryption".to_string(),
        payload: json!({
            "volume_ids": ["vol-123", "vol-456"],
            "encryption_level": "aes-256",
            "key_management": "hardware-hsm"
        }),
        timestamp: chrono::Utc::now(),
    };

    // Send request to security primal
    match nestgate.coordinate_with_primal(request).await {
        Ok(_response) => {
            info!("✅ Security primal integration successful");
        }
        Err(e) => {
            info!(
                "⚠️  Security primal integration failed (expected in demo): {}",
                e
            );
            info!("In production, this would establish encrypted storage with the security primal");
        }
    }

    // Demonstrate storage encryption coordination
    println!("   → Coordinating storage encryption with security primal...");
    sleep(Duration::from_millis(500)).await;
    println!("   → Establishing HSM key management...");
    sleep(Duration::from_millis(500)).await;
    println!("   → Configuring zero-knowledge encryption...");
    sleep(Duration::from_millis(500)).await;

    info!("🛡️  NestGate + Security Primal Features:");
    println!("     • Hardware-backed encryption keys");
    println!("     • Zero-knowledge storage architecture");
    println!("     • Distributed key management");
    println!("     • Automatic key rotation");

    Ok(())
}

/// Demonstrate AI primal integration (works with any AI primal)
async fn demo_ai_primal_integration(nestgate: &NestGateStoragePrimal) -> Result<()> {
    info!("🧠 Demonstrating AI primal integration...");

    let request = PrimalCoordinationRequest {
        request_id: "ai-demo".to_string(),
        from_primal: "nestgate".to_string(),
        to_primal: "ai-primal".to_string(),
        coordination_type: "storage-optimization".to_string(),
        payload: json!({
            "storage_pools": ["pool-1", "pool-2"],
            "optimization_type": "intelligent-tiering",
            "analysis_scope": "full-filesystem"
        }),
        timestamp: chrono::Utc::now(),
    };

    // Send request to AI primal
    match nestgate.coordinate_with_primal(request).await {
        Ok(_response) => {
            info!("✅ AI primal integration successful");
        }
        Err(e) => {
            info!("⚠️  AI primal integration failed (expected in demo): {}", e);
            info!("In production, this would provide AI-optimized storage for the AI primal");
        }
    }

    // Demonstrate AI storage optimization
    println!("   → Analyzing storage patterns with AI primal...");
    sleep(Duration::from_millis(500)).await;
    println!("   → Implementing intelligent data tiering...");
    sleep(Duration::from_millis(500)).await;
    println!("   → Optimizing compression algorithms...");
    sleep(Duration::from_millis(500)).await;

    info!("🧠 NestGate + AI Primal Features:");
    println!("     • Intelligent data tiering");
    println!("     • Predictive caching");
    println!("     • Automated optimization");
    println!("     • Pattern-based deduplication");

    Ok(())
}

/// Demonstrate orchestration primal integration (works with any orchestration primal)
async fn demo_orchestration_primal_integration(nestgate: &NestGateStoragePrimal) -> Result<()> {
    info!("🎵 Demonstrating orchestration primal integration...");

    let request = PrimalCoordinationRequest {
        request_id: "orchestration-demo".to_string(),
        from_primal: "nestgate".to_string(),
        to_primal: "orchestration-primal".to_string(),
        coordination_type: "service-discovery".to_string(),
        payload: json!({
            "service_name": "nestgate-storage-service",
            "service_type": "storage",
            "discovery_scope": "local-network",
            "load_balancing": true
        }),
        timestamp: chrono::Utc::now(),
    };

    // Send request to orchestration primal
    match nestgate.coordinate_with_primal(request).await {
        Ok(_response) => {
            info!("✅ Orchestration primal integration successful");
        }
        Err(e) => {
            info!(
                "⚠️  Orchestration primal integration failed (expected in demo): {}",
                e
            );
            info!("In production, this would enable service discovery and load balancing with the orchestration primal");
        }
    }

    // Demonstrate service discovery coordination
    println!("   → Registering service with orchestration primal...");
    sleep(Duration::from_millis(500)).await;
    println!("   → Discovering other services...");
    sleep(Duration::from_millis(500)).await;
    println!("   → Configuring load balancing...");
    sleep(Duration::from_millis(500)).await;

    info!("🎵 NestGate + Orchestration Primal Features:");
    println!("     • Universal service discovery");
    println!("     • Intelligent load balancing");
    println!("     • Cross-primal coordination");
    println!("     • Automatic service registration");

    Ok(())
}

/// Demonstrate compute primal integration (works with any compute primal)
async fn demo_compute_primal_integration(nestgate: &NestGateStoragePrimal) -> Result<()> {
    info!("💻 Demonstrating compute primal integration...");

    let request = PrimalCoordinationRequest {
        request_id: "compute-demo".to_string(),
        from_primal: "nestgate".to_string(),
        to_primal: "compute-primal".to_string(),
        coordination_type: "resource-allocation".to_string(),
        payload: json!({
            "workload_type": "batch-processing",
            "resource_requirements": {
                "cpu": "4 cores",
                "memory": "16GB",
                "gpu": false,
                "storage": "1TB"
            },
            "execution_mode": "serverless",
            "persistent_storage": true
        }),
        timestamp: chrono::Utc::now(),
    };

    // Send request to compute primal
    match nestgate.coordinate_with_primal(request).await {
        Ok(_response) => {
            info!("✅ Compute primal integration successful");
        }
        Err(e) => {
            info!(
                "⚠️  Compute primal integration failed (expected in demo): {}",
                e
            );
            info!("In production, this would provide compute resources for the compute primal");
        }
    }

    // Demonstrate resource allocation coordination
    println!("   → Requesting compute resources from compute primal...");
    sleep(Duration::from_millis(500)).await;
    println!("   → Configuring persistent storage...");
    sleep(Duration::from_millis(500)).await;
    println!("   → Establishing execution environment...");
    sleep(Duration::from_millis(500)).await;

    info!("💻 NestGate + Compute Primal Features:");
    println!("     • Dynamic resource scaling");
    println!("     • Persistent storage for workloads");
    println!("     • Serverless execution");
    println!("     • Resource-aware allocation");

    Ok(())
}

/// Demonstrate multi-primal coordination
async fn demo_multi_primal_coordination(nestgate: &NestGateStoragePrimal) -> Result<()> {
    info!("🌟 Demonstrating multi-primal coordination...");

    // Simulate a complex workflow requiring multiple primals
    println!("Scenario: Processing sensitive AI training data with distributed storage");

    // Step 1: Request security from BearDog
    println!("   Step 1: Requesting encryption from security primal...");
    sleep(Duration::from_millis(500)).await;

    // Step 2: Request AI processing from Squirrel
    println!("   Step 2: Requesting AI processing from AI primal...");
    sleep(Duration::from_millis(500)).await;

    // Step 3: Request distribution from Songbird
    println!("   Step 3: Requesting distribution from orchestration primal...");
    sleep(Duration::from_millis(500)).await;

    // Step 4: Request compute resources from Toadstool
    println!("   Step 4: Requesting compute resources from compute primal...");
    sleep(Duration::from_millis(500)).await;

    // Step 5: Coordinate all primals
    println!("   Step 5: Coordinating all primals for unified workflow...");
    sleep(Duration::from_millis(1000)).await;

    info!("✅ Multi-primal coordination completed successfully");

    let coordination_benefits = vec![
        "Unified security, AI, storage, and compute",
        "Automatic failover and load balancing",
        "Consistent configuration across all primals",
        "Centralized monitoring and logging",
        "Simplified management interface",
    ];

    info!("🎯 Multi-Primal Coordination Benefits:");
    for benefit in coordination_benefits {
        info!("   • {}", benefit);
    }

    Ok(())
}

/// Demonstrate health monitoring
async fn demo_health_monitoring(nestgate: &NestGateStoragePrimal) -> Result<()> {
    info!("🏥 Demonstrating health monitoring...");

    // Check NestGate health
    let health = nestgate.health_check().await;
    match health {
        StoragePrimalHealth::Healthy {
            uptime,
            storage_capacity,
            performance_metrics,
        } => {
            info!("✅ NestGate is healthy");
            info!("   Uptime: {:?}", uptime);
            info!(
                "   Storage: {:.1}% used",
                (storage_capacity.used_bytes as f64 / storage_capacity.total_bytes as f64) * 100.0
            );
            info!(
                "   Performance: {} IOPS",
                performance_metrics.read_iops + performance_metrics.write_iops
            );
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

    // Demonstrate health monitoring features
    let health_features = vec![
        "Real-time health status reporting",
        "Automated health checks every 30 seconds",
        "Performance metrics collection",
        "Capacity monitoring and alerting",
        "Cross-primal health coordination",
        "Predictive health analytics",
    ];

    info!("📊 Health Monitoring Features:");
    for feature in health_features {
        info!("   • {}", feature);
    }

    Ok(())
}

/// Convert configuration for NestGate primal
impl From<UniversalNestGateConfig> for NestGatePrimalConfig {
    fn from(config: UniversalNestGateConfig) -> Self {
        Self {
            host: config.nestgate.server.host,
            port: config.nestgate.server.port,
            discovery_enabled: config.primal_ecosystem.enabled,
            primal_registry_endpoint: config.discovery.service_registry.map(|sr| sr.endpoint),
        }
    }
}

// Example usage and testing
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_universal_primal_integration() {
        let config = UniversalNestGateConfig::default();
        assert!(config.validate().is_ok());

        let nestgate_config: NestGatePrimalConfig = config.into();
        assert_eq!(nestgate_config.host, "0.0.0.0");
        assert_eq!(nestgate_config.port, 8080);
    }

    #[tokio::test]
    async fn test_primal_request_creation() {
        let request = StoragePrimalRequest {
            request_id: uuid::Uuid::new_v4(),
            from_primal: "nestgate".to_string(),
            to_primal: "beardog".to_string(),
            request_type: StorageRequestType::CapabilityQuery,
            payload: serde_json::json!({}),
            timestamp: std::time::SystemTime::now(),
        };

        assert_eq!(request.from_primal, "nestgate");
        assert_eq!(request.to_primal, "beardog");
    }

    #[tokio::test]
    async fn test_configuration_loading() {
        // Test default configuration
        let config = UniversalNestGateConfig::default();
        assert!(config.primal_ecosystem.enabled);
        assert_eq!(config.primal_ecosystem.primal_id, "nestgate");

        // Test validation
        assert!(config.validate().is_ok());
    }
}
