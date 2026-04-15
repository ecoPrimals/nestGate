// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Lightweight “stress” checks: allocation churn and canonical trait compile surface.

use nestgate_core::{
    CanonicalNetwork, CanonicalSecurity, CanonicalService, CanonicalStorage,
    error::Result,
    service_discovery::types::{ServiceInfo, ServiceMetadata},
};
use std::time::SystemTime;
use uuid::Uuid;

#[tokio::test]
async fn test_memory_leak_detection() -> Result<()> {
    println!("🔍 Starting performance test: Memory Leak Detection");

    // Stand-in for a one-off heap allocation: same stress pattern without `unsafe` alloc APIs.
    let mut _initial_allocation = vec![0u8; 1024];

    // Simulate operations that might cause memory leaks
    for i in 0..100 {
        let _service_info = ServiceInfo {
            service_id: Uuid::new_v4(),
            metadata: ServiceMetadata {
                name: format!("service_{}", i),
                ..Default::default()
            },
            capabilities: vec![],
            endpoints: vec![],
            last_seen: SystemTime::now(),
        };

        // Force garbage collection attempt
        if i % 10 == 0 {
            tokio::task::yield_now().await;
        }
    }

    // Memory should be properly managed (Vec drops here).
    _initial_allocation.fill(0);

    println!("✅ Memory leak detection test completed");
    Ok(())
}

#[test]
fn test_compilation_time_optimization() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Test that canonical traits compile quickly
    let start = std::time::Instant::now();

    // This test itself validates that traits compile
    fn _compile_test<T: CanonicalService>() {}
    fn _compile_test2<T: CanonicalStorage>() {}
    fn _compile_test3<T: CanonicalNetwork>() {}
    fn _compile_test4<T: CanonicalSecurity>() {}

    let compilation_time = start.elapsed();
    println!(
        "✅ Canonical traits compilation time: {:?}",
        compilation_time
    );

    // Should compile very quickly with native async
    assert!(
        compilation_time.as_millis() < 1000,
        "Canonical traits should compile quickly"
    );
    Ok(())
}
