//! **EXTENDED PERFORMANCE VALIDATION TEST SUITE**
//!
//! Comprehensive performance validation for canonical modernization including:
//! - Zero-copy operation validation
//! - Canonical pattern performance
//! - Memory efficiency testing
//! - Concurrent operation scaling
//! - System throughput validation

use crate::canonical_modernization::{UnifiedHealthStatus, UnifiedServiceType};
use nestgate_core::config::unified::types::CanonicalConfig;
use nestgate_core::error::{NestGateError, Result};

use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Test zero-copy configuration operations performance

#[tokio::test]
async fn test_zero_copy_config_performance() -> Result<()> {
    println!("🧪 Testing zero-copy configuration performance...");

    let start = Instant::now();

    // Create a large configuration without unnecessary cloning
    let config = Arc::new(CanonicalConfig::default());

    // Test that Arc sharing is zero-copy
    let config_refs: Vec<Arc<CanonicalConfig>> = (0..10000).map(|_| Arc::clone(&config)).collect();

    let sharing_time = start.elapsed();

    // Verify that sharing is extremely fast (should be sub-millisecond)
    assert!(
        sharing_time < Duration::from_millis(10),
        "Arc sharing should be near-instantaneous"
    );

    // Test concurrent read access
    let start = Instant::now();
    let handles: Vec<_> = config_refs
        .into_iter()
        .take(100)
        .map(|config_ref| {
            tokio::spawn(async move {
                // Read operations should be zero-copy
                let _port = config_ref.network.api.port;
                let _backend = &config_ref.storage.backend_type;
                let _auth = config_ref.security.enable_auth;
            })
        })
        .collect();

    futures::future::join_all(handles).await;
    let concurrent_time = start.elapsed();

    // Concurrent reads should be very fast
    assert!(
        concurrent_time < Duration::from_millis(50),
        "Concurrent reads should be fast"
    );

    println!("✅ Zero-copy configuration performance validated");
    println!("   📊 Arc sharing (10k): {sharing_time:?}");
    println!("   📊 Concurrent reads (100): {concurrent_time:?}");

    Ok(())
}

/// Test canonical pattern memory efficiency
#[tokio::test]
async fn test_canonical_memory_efficiency() -> Result<()> {
    println!("🧪 Testing canonical pattern memory efficiency...");

    // Test that unified types don't waste memory
    let service_types: Vec<UnifiedServiceType> = vec![
        UnifiedServiceType::Storage,
        UnifiedServiceType::Network,
        UnifiedServiceType::Security,
        UnifiedServiceType::Monitoring,
    ];

    // Verify enum size is reasonable (should be small)
    let enum_size = std::mem::size_of::<UnifiedServiceType>();
    assert!(
        enum_size <= 32,
        "UnifiedServiceType should be memory efficient (≤32 bytes)"
    );

    // Test health status efficiency
    let health_size = std::mem::size_of::<UnifiedHealthStatus>();
    assert!(
        health_size <= 32,
        "UnifiedHealthStatus should be memory efficient (≤32 bytes)"
    );

    // Test configuration size is reasonable
    let config_size = std::mem::size_of::<CanonicalConfig>();
    println!("   📊 CanonicalConfig size: {config_size} bytes");

    // Config should be reasonably sized (not bloated)
    assert!(
        config_size < 10000,
        "CanonicalConfig should not be excessively large"
    );

    println!("✅ Canonical pattern memory efficiency validated");
    println!("   📊 UnifiedServiceType: {enum_size} bytes");
    println!("   📊 UnifiedHealthStatus: {health_size} bytes");

    // Remove unused variable warning
    let _service_types = service_types;

    Ok(())
}

/// Test high-throughput operations with canonical patterns
#[tokio::test]
async fn test_high_throughput_operations() -> Result<()> {
    println!("🧪 Testing high-throughput operations...");

    let config = Arc::new(RwLock::new(CanonicalConfig::default()));
    let operations = 10000;

    let start = Instant::now();

    // Test concurrent read operations
    let read_handles: Vec<_> = (0..operations)
        .map(|i| {
            let config = Arc::clone(&config);
            tokio::spawn(async move {
                let config_guard = config.read().await;
                let _port = config_guard.network.api.port;
                let _result = format!("operation_{i}");
            })
        })
        .collect();

    futures::future::join_all(read_handles).await;
    let read_time = start.elapsed();

    // Calculate throughput
    let read_ops_per_sec = operations as f64 / read_time.as_secs_f64();

    // Should handle thousands of operations per second
    assert!(
        read_ops_per_sec > 1000.0,
        "Should handle >1000 read ops/sec"
    );

    println!("✅ High-throughput operations validated");
    println!("   📊 Read throughput: {read_ops_per_sec:.0} ops/sec");

    Ok(())
}

/// Test error handling performance in canonical system
#[tokio::test]
async fn test_canonical_error_performance() -> Result<()> {
    println!("🧪 Testing canonical error handling performance...");

    let start = Instant::now();

    // Create many errors to test performance
    let errors: Vec<NestGateError> = (0..1000)
        .map(|i| NestGateError::Internal {
            message: format!("Test error {i}"),
            location: Some(format!("test_location_{i}")),
            debug_info: None,
            is_bug: false,
        })
        .collect();

    let creation_time = start.elapsed();

    // Test error formatting performance
    let start = Instant::now();
    let _formatted: Vec<String> = errors.iter().map(|error| format!("{error}")).collect();
    let formatting_time = start.elapsed();

    // Error operations should be fast
    assert!(
        creation_time < Duration::from_millis(50),
        "Error creation should be fast"
    );
    assert!(
        formatting_time < Duration::from_millis(100),
        "Error formatting should be fast"
    );

    println!("✅ Canonical error handling performance validated");
    println!("   📊 Error creation (1000): {creation_time:?}");
    println!("   📊 Error formatting (1000): {formatting_time:?}");

    Ok(())
}

/// Test scalability of canonical configuration system
#[tokio::test]
async fn test_canonical_scalability() -> Result<()> {
    println!("🧪 Testing canonical configuration scalability...");

    // Test scaling with increasing load
    for scale in [100, 500, 1000, 2000] {
        let start = Instant::now();

        // Create configurations at scale
        let configs: Vec<CanonicalConfig> = (0..scale)
            .map(|i| CanonicalConfig {
                network: nestgate_core::config::unified::types::NetworkConfig {
                    api: nestgate_core::config::unified::types::ApiServerConfig {
                        port: 8000 + (i % 1000) as u16,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .collect();

        let creation_time = start.elapsed();

        // Test concurrent access at scale
        let start = Instant::now();
        let handles: Vec<_> = configs
            .into_iter()
            .take(std::cmp::min(scale, 100))
            .map(|config| {
                tokio::spawn(async move {
                    let _serialized = serde_json::to_string(&config)?;
                })
            })
            .collect();

        futures::future::join_all(handles).await;
        let access_time = start.elapsed();

        // Performance should scale reasonably
        let creation_rate = scale as f64 / creation_time.as_secs_f64();

        println!("   📊 Scale {scale}: {creation_rate:.0} configs/sec, access: {access_time:?}");

        // Should maintain reasonable performance even at scale
        assert!(
            creation_rate > 1000.0,
            "Should maintain >1000 configs/sec at scale {scale}"
        );
        Ok(())
    }

    println!("✅ Canonical configuration scalability validated");
    Ok(())
}
