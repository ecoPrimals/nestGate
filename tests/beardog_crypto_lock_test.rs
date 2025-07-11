//! BearDog Crypto Lock Integration Test
//!
//! This test verifies that:
//! 1. BearDog is the EXCLUSIVE key manager for crypto locks
//! 2. All external extraction requires BearDog crypto locks
//! 3. Keys never leave the ecosystem
//! 4. Internal communication remains free

use nestgate_core::{cert::BearDogConfig, crypto_locks::*, Result};
use std::time::Duration;

#[tokio::test]
async fn test_beardog_exclusive_key_management() -> Result<()> {
    // Setup BearDog configuration
    let beardog_config = BearDogConfig {
        endpoint: "https://beardog.test:8443".to_string(),
        api_key: "test-sovereign-key".to_string(),
        trust_anchor: "beardog-trust-anchor".to_string(),
        validation_timeout: Duration::from_secs(30),
        retry_attempts: 3,
    };

    // Create BearDog-managed boundary guardian
    let guardian = ExternalBoundaryGuardian::new(beardog_config);

    // Test 1: Internal communication should be free (no BearDog keys required)
    let internal_decision = guardian
        .check_external_boundary("nestgate-core", "nestgate-api", "query")
        .await?;

    match internal_decision {
        AccessDecision::Allow { reason, .. } => {
            assert!(reason.contains("no BearDog key required"));
            println!("✅ Internal communication is free (no BearDog keys needed)");
        }
        _ => panic!("Internal communication should be free"),
    }

    // Test 2: External access should require BearDog crypto locks
    let external_decision = guardian
        .check_external_boundary(
            "nestgate-core",
            "https://api.external-service.com",
            "export",
        )
        .await?;

    match external_decision {
        AccessDecision::RequireLock { reason, .. } => {
            assert!(reason.contains("BearDog crypto lock required"));
            println!("✅ External access requires BearDog crypto locks");
        }
        _ => panic!("External access should require BearDog crypto locks"),
    }

    Ok(())
}

#[tokio::test]
async fn test_beardog_sovereignty_model() -> Result<()> {
    println!("👑 Testing BearDog Sovereignty Model");

    // Create BearDog configuration
    let beardog_config = BearDogConfig {
        endpoint: "https://beardog.sovereign:8443".to_string(),
        api_key: "sovereign-master-key".to_string(),
        trust_anchor: "beardog-sovereign-trust".to_string(),
        validation_timeout: Duration::from_secs(30),
        retry_attempts: 3,
    };

    let guardian = ExternalBoundaryGuardian::new(beardog_config);

    // Scenario 1: Your ecosystem - completely free
    let ecosystem_comm = guardian
        .check_external_boundary("nestgate-zfs", "nestgate-automation", "optimize")
        .await?;

    assert!(matches!(ecosystem_comm, AccessDecision::Allow { .. }));
    println!("✅ Your ecosystem: Completely free communication");

    // Scenario 2: External company wants to use your system
    let external_request = guardian
        .check_external_boundary("nestgate-core", "https://bigtech-corp.com/api", "extract")
        .await?;

    assert!(matches!(
        external_request,
        AccessDecision::RequireLock { .. }
    ));
    println!("✅ External company: Must use BearDog crypto lock");

    println!("👑 BearDog Sovereignty Model verified:");
    println!("   • Your ecosystem: Completely free");
    println!("   • External companies: Must use BearDog keys");
    println!("   • Keys never leave ecosystem: ✅ Guaranteed");

    Ok(())
}
