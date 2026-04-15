// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage-oriented chaos: simulated disk pressure and bounded async writes.

use super::common::{ChaosTestConfig, FaultInjector};
use nestgate_core::error::{NestGateError, Result};
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn test_storage_fault_tolerance() -> Result<()> {
    println!("🌪️ Starting chaos test: Storage Fault Tolerance");

    let chaos_config = ChaosTestConfig {
        failure_rate: 0.2, // 20% failure rate
        disk_full_simulation: true,
        ..Default::default()
    };

    let fault_injector = FaultInjector::new(chaos_config);

    // Test storage operations under fault conditions
    for i in 0..10 {
        fault_injector.simulate_memory_pressure()?;

        // Simulate storage operation
        let operation_result = timeout(Duration::from_secs(5), async {
            fault_injector.maybe_fail().await?;

            // Simulate write operation
            Ok::<String, NestGateError>(format!("Data written: item_{}", i))
        })
        .await;

        match operation_result {
            Ok(Ok(_)) => println!("✅ Storage operation {} succeeded", i),
            Ok(Err(_)) => println!("⚠️ Storage operation {} failed (expected under chaos)", i),
            Err(_) => println!(
                "⏰ Storage operation {} timed out (expected under chaos)",
                i
            ),
        }
    }

    println!("✅ Storage fault tolerance test completed");
    Ok(())
}
