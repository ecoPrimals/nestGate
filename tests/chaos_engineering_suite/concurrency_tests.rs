// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Concurrent access chaos: many tokio tasks competing under shared fault injection.

use super::common::{ChaosTestConfig, FaultInjector};
use nestgate_core::error::{NestGateError, Result};

#[tokio::test]
async fn test_concurrent_operations_under_stress() -> Result<()> {
    println!("🌪️ Starting chaos test: Concurrent Operations Under Stress");

    let chaos_config = ChaosTestConfig {
        failure_rate: 0.15, // 15% failure rate
        network_latency_ms: 50,
        memory_pressure: true,
        ..Default::default()
    };

    // Spawn multiple concurrent operations
    let mut handles = vec![];

    for i in 0..50 {
        let config = chaos_config.clone();
        let handle = tokio::spawn(async move {
            let fault_injector = FaultInjector::new(config);

            // Simulate concurrent service operation
            fault_injector.simulate_network_latency().await;
            fault_injector.maybe_fail().await?;

            Ok::<String, NestGateError>(format!("Operation {} completed", i))
        });
        handles.push(handle);
    }

    // Wait for all operations to complete
    let mut successful = 0;
    let mut failed = 0;

    for handle in handles {
        match handle.await {
            Ok(Ok(_)) => successful += 1,
            Ok(Err(_)) => failed += 1,
            Err(_) => failed += 1,
        }
    }

    // Should handle concurrent stress gracefully
    assert!(successful > 0, "Some concurrent operations should succeed");

    let success_rate = successful as f64 / (successful + failed) as f64;
    println!(
        "✅ Concurrent stress test: {:.1}% success rate",
        success_rate * 100.0
    );

    Ok(())
}
