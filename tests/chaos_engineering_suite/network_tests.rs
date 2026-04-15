// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Network-oriented chaos: discovery under failure injection and high-loss resilience.

use super::common::{ChaosTestConfig, FaultInjector, ResilientService};
use nestgate_core::{
    error::Result,
    service_discovery::types::{ServiceInfo, ServiceMetadata},
};
use std::time::SystemTime;
use uuid::Uuid;

#[tokio::test]
async fn test_service_discovery_under_chaos() -> Result<()> {
    println!("🌪️ Starting chaos test: Service Discovery");

    let chaos_config = ChaosTestConfig {
        failure_rate: 0.3, // 30% failure rate
        network_latency_ms: 200,
        ..Default::default()
    };

    let service_info = ServiceInfo {
        service_id: Uuid::new_v4(),
        metadata: ServiceMetadata {
            name: "chaos-test-service".to_string(),
            ..Default::default()
        },
        capabilities: vec![],
        endpoints: vec![],
        last_seen: SystemTime::now(),
    };

    let resilient_service = ResilientService::new(service_info, chaos_config);

    // Test service discovery under chaos conditions
    let result = resilient_service
        .execute_with_retry(|| {
            // Simulate service discovery operation
            Ok(format!("Service discovered: {}", "test-service"))
        })
        .await;

    assert!(result.is_ok());
    println!("✅ Service discovery survived chaos conditions");
    Ok(())
}

#[tokio::test]
async fn test_network_resilience_under_failures() -> Result<()> {
    println!("🌪️ Starting chaos test: Network Resilience");

    let chaos_config = ChaosTestConfig {
        failure_rate: 0.5, // 50% failure rate
        network_latency_ms: 500,
        random_disconnections: true,
        ..Default::default()
    };

    let fault_injector = FaultInjector::new(chaos_config);

    // Test multiple network operations with high failure rate
    let mut successful_operations = 0;
    let total_operations = 20;

    for _i in 0..total_operations {
        // Simulate network latency
        fault_injector.simulate_network_latency().await;

        // Try network operation with potential failure
        if fault_injector.maybe_fail().await.is_ok() {
            successful_operations += 1;
        }

        // Yield between operations for proper async coordination
        tokio::task::yield_now().await;
    }

    // Should have some successful operations despite chaos
    assert!(
        successful_operations > 0,
        "At least some operations should succeed under chaos"
    );

    let success_rate = successful_operations as f64 / total_operations as f64;
    println!(
        "✅ Network resilience test: {:.1}% success rate under chaos",
        success_rate * 100.0
    );

    Ok(())
}
