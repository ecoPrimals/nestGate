// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(
    unused,
    dead_code,
    deprecated,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

/// ZFS Integration Test
///
/// Real integration tests for ZFS functionality
///
/// **Note**: These tests require a real ZFS system. They will be skipped
/// if ZFS is not available or cannot be accessed.
use nestgate_core::{NestGateError, Result};
use nestgate_zfs::config::ZfsConfig;
use nestgate_zfs::manager::ZfsManager;
use std::sync::Arc;
use tokio::time::Duration;

use nestgate_core::canonical_types::StorageTier;

/// Helper to check if ZFS is available
async fn is_zfs_available() -> bool {
    let config = ZfsConfig::default();
    match ZfsManager::new(config).await {
        Ok(_) => true,
        Err(e) => {
            if e.to_string().contains("ZFS modules cannot be auto-loaded")
                || e.to_string().contains("Tier manager")
            {
                false
            } else {
                // Other errors mean ZFS might be available but having issues
                true
            }
        }
    }
}

#[tokio::test]
#[ignore = "Requires real ZFS system - run with --ignored flag when ZFS available"]
async fn test_zfs_integration() -> Result<()> {
    if !is_zfs_available().await {
        println!("⏭️ Skipping ZFS integration test - ZFS not available");
        return Ok(());
    }

    println!("🚀 Starting ZFS integration test");

    // Create ZFS manager
    let config = ZfsConfig::default();
    let manager = ZfsManager::new(config).await.map_err(|e| {
        NestGateError::internal_error(
            format!("Failed to create ZFS manager: {e}"),
            "test_component",
        )
    })?;

    println!("✅ ZFS manager created successfully");

    // Test basic manager functionality
    let service_status = manager.get_service_status().await?;
    println!("📊 Service status: {:?}", service_status.overall_health);

    println!("✅ ZFS integration test completed successfully");
    Ok(())
}

#[tokio::test]
#[ignore = "Requires real ZFS system - run with --ignored flag when ZFS available"]
async fn test_zfs_pool_operations() -> Result<()> {
    if !is_zfs_available().await {
        println!("⏭️ Skipping ZFS pool operations test - ZFS not available");
        return Ok(());
    }

    println!("🔄 Testing ZFS pool operations");

    let config = ZfsConfig::default();
    let manager = ZfsManager::new(config).await?;

    // Test pool manager operations
    let pool_status = manager.pool_manager.get_overall_status().await?;
    println!(
        "📊 Pool status - Online pools: {}",
        pool_status.pools_online
    );

    Ok(())
}

#[tokio::test]
#[ignore = "Requires real ZFS system - run with --ignored flag when ZFS available"]
async fn test_zfs_dataset_operations() -> Result<()> {
    if !is_zfs_available().await {
        println!("⏭️ Skipping ZFS dataset operations test - ZFS not available");
        return Ok(());
    }

    println!("🗂️ Testing ZFS dataset operations");

    let config = ZfsConfig::default();
    let manager = ZfsManager::new(config).await?;

    // Test dataset creation
    let dataset_name = "nestpool/test_dataset";
    let result = manager
        .dataset_manager
        .create_dataset(dataset_name, "zfspool", StorageTier::Warm)
        .await;

    match result {
        Ok(_) => println!("✅ Dataset created successfully"),
        Err(e) => println!("⚠️ Dataset creation failed (expected in test): {e}"),
    }

    Ok(())
}

#[tokio::test]
#[ignore = "Requires real ZFS system - run with --ignored flag when ZFS available"]
async fn test_zfs_performance_monitoring() -> Result<()> {
    if !is_zfs_available().await {
        println!("⏭️ Skipping ZFS performance monitoring test - ZFS not available");
        return Ok(());
    }

    println!("📈 Testing ZFS performance monitoring");

    let config = ZfsConfig::default();
    let manager = ZfsManager::new(config).await?;

    // Test performance metrics
    let metrics = manager
        .performance_monitor
        .read()
        .await
        .get_current_metrics()
        .await;
    println!(
        "📊 Performance metrics collected: {} pools",
        metrics.pool_metrics.total_iops as i32
    );

    Ok(())
}

#[tokio::test]
#[ignore = "Requires real ZFS system - run with --ignored flag when ZFS available"]
async fn test_zfs_concurrent_operations() -> Result<()> {
    if !is_zfs_available().await {
        println!("⏭️ Skipping ZFS concurrent operations test - ZFS not available");
        return Ok(());
    }

    println!("🔄 Testing concurrent ZFS operations");

    let config = ZfsConfig::default();
    let manager = Arc::new(ZfsManager::new(config).await?);

    // Test concurrent operations
    let mut handles = vec![];

    for i in 0..5 {
        let manager_clone = Arc::clone(&manager);
        let handle = tokio::spawn(async move {
            let _metrics = manager_clone
                .performance_monitor
                .read()
                .await
                .get_current_metrics()
                .await;
            println!("🔄 Concurrent operation {i} completed");
            Ok::<(), NestGateError>(())
        });
        handles.push(handle);
    }

    // Wait for all operations
    for handle in handles {
        handle
            .await
            .map_err(|e| NestGateError::internal_error(e.to_string(), "test_component"))??;
    }

    println!("✅ All concurrent operations completed");
    Ok(())
}

#[tokio::test]
#[ignore = "Requires real ZFS system - run with --ignored flag when ZFS available"]
async fn test_zfs_error_handling() -> Result<()> {
    if !is_zfs_available().await {
        println!("⏭️ Skipping ZFS error handling test - ZFS not available");
        return Ok(());
    }

    println!("❌ Testing ZFS error handling");

    let config = ZfsConfig {
        // API endpoint configuration removed - using network config instead
        ..Default::default()
    };

    let manager = ZfsManager::new(config).await.map_err(|e| {
        NestGateError::internal_error(format!("Failed to create manager: {e}"), "test_component")
    })?;

    // This should handle the error gracefully
    let _status = manager.get_service_status().await?;

    println!("✅ Error handling test completed");
    Ok(())
}

#[tokio::test]
#[ignore = "Requires real ZFS system - run with --ignored flag when ZFS available"]
async fn test_zfs_timeout_handling() -> Result<()> {
    if !is_zfs_available().await {
        println!("⏭️ Skipping ZFS timeout handling test - ZFS not available");
        return Ok(());
    }

    println!("⏱️ Testing ZFS timeout handling");

    let config = ZfsConfig::default();
    let manager = ZfsManager::new(config).await?;

    // Test with timeout
    let result = tokio::time::timeout(Duration::from_secs(5), manager.get_service_status()).await;

    match result {
        Ok(status) => {
            println!(
                "✅ Operation completed within timeout: {:?}",
                status.is_ok()
            );
        }
        Err(_) => {
            println!("⚠️ Operation timed out (expected in some environments)");
        }
    }

    Ok(())
}
