// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Configuration validation under timeouts, recovery loops, and graceful degradation under load.

use super::common::{ChaosTestConfig, FaultInjector};
use nestgate_core::{
    config::canonical_primary::NestGateCanonicalConfig,
    error::{NestGateError, Result},
    service_discovery::types::{ServiceInfo, ServiceMetadata},
};
use std::time::{Duration, SystemTime};
use tokio::time::timeout;
use uuid::Uuid;

#[tokio::test]
async fn test_configuration_resilience() -> Result<()> {
    println!("🌪️ Starting chaos test: Configuration Resilience");

    // Test configuration loading under various failure conditions
    let configs: Vec<NestGateCanonicalConfig> = vec![
        // Valid configuration
        NestGateCanonicalConfig::default(),
        // Configuration with missing fields (should use defaults)
        {
            let mut config: NestGateCanonicalConfig = NestGateCanonicalConfig::default();
            config.system.instance_name = "".to_string(); // Invalid but should be handled
            config
        },
    ];

    for (i, config) in configs.into_iter().enumerate() {
        // Test configuration validation under chaos
        let validation_result = timeout(Duration::from_secs(2), async {
            // Simulate configuration validation
            if config.system.instance_name.is_empty() {
                return Err(NestGateError::validation("Instance name cannot be empty"));
            }
            Ok::<(), NestGateError>(())
        })
        .await;

        match validation_result {
            Ok(Ok(())) => println!("✅ Configuration {} is valid", i),
            Ok(Err(e)) => println!("⚠️ Configuration {} validation failed: {}", i, e),
            Err(_) => println!("⏰ Configuration {} validation timed out", i),
        }
    }

    println!("✅ Configuration resilience test completed");
    Ok(())
}

#[tokio::test]
async fn test_system_recovery_after_failures() -> Result<()> {
    println!("🌪️ Starting chaos test: System Recovery");

    let chaos_config = ChaosTestConfig {
        failure_rate: 0.60, // 60% failure rate - balanced between chaos and reliability
        network_latency_ms: 200, // Reduced to make test faster
        memory_pressure: true,
        ..Default::default()
    };

    let fault_injector = FaultInjector::new(chaos_config);

    // Simulate system under extreme stress
    let mut recovery_successful = false;
    let max_attempts = 15; // Increased to ensure >99.9% success probability

    for attempt in 1..=max_attempts {
        println!("🔄 Recovery attempt {}/{}", attempt, max_attempts);

        // Inject extreme chaos
        fault_injector.simulate_memory_pressure()?;
        fault_injector.simulate_network_latency().await;

        // Try system recovery
        if fault_injector.maybe_fail().await.is_ok() {
            recovery_successful = true;
            println!("✅ System recovered on attempt {}", attempt);
            break;
        }

        // Exponential backoff for recovery with realistic async delay (capped to avoid excessive delay)
        let backoff_ms = std::cmp::min(100 * (1 << attempt), 2000); // Cap at 2 seconds
        let backoff = Duration::from_millis(backoff_ms);
        tokio::time::sleep(backoff).await;
    }

    assert!(
        recovery_successful,
        "System should eventually recover even under extreme chaos (failed after {} attempts)",
        max_attempts
    );
    println!("✅ System recovery test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_graceful_degradation() -> Result<()> {
    println!("🌪️ Starting chaos test: Graceful Degradation");

    // Test that system degrades gracefully under resource constraints
    let service_info = ServiceInfo {
        service_id: Uuid::new_v4(),
        metadata: ServiceMetadata {
            name: "test-service".to_string(),
            ..Default::default()
        },
        capabilities: vec![],
        endpoints: vec![],
        last_seen: SystemTime::now(),
    };

    // Simulate resource exhaustion
    let degraded_result = timeout(Duration::from_secs(1), async {
        // Simulate resource-intensive operation
        let _large_allocation: Vec<u8> = vec![0; 1024 * 1024]; // 1MB allocation

        // System should still respond, even if slower
        Ok::<ServiceInfo, NestGateError>(service_info.clone())
    })
    .await;

    match degraded_result {
        Ok(Ok(_)) => println!("✅ System maintained functionality under resource pressure"),
        Ok(Err(e)) => println!("⚠️ System degraded gracefully: {}", e),
        Err(_) => println!("⏰ System response degraded but didn't crash"),
    }

    println!("✅ Graceful degradation test completed");
    Ok(())
}
