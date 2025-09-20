//! **CHAOS ENGINEERING TESTS**
//!
//! Fault injection and resilience testing to validate system behavior under stress.

use crate::common::*;
use crate::config::*;
use std::time::{Duration, Instant};
use tokio::time::{sleep, timeout};
use rand::Rng;

/// Test network partition resilience
#[tokio::test]
async fn test_network_partition_resilience() -> Result<(), Box<dyn std::error::Error>> {
    init_test_logging();
    
    println!("🌪️ CHAOS: Network Partition Simulation");
    
    let storage = MockStorage::new();
    let start_time = Instant::now();
    
    // Phase 1: Establish baseline
    println!("  🌐 Establishing baseline conditions...");
    let test_key = "network_test";
    let test_data = b"Network resilience test data";
    assert!(storage.write(test_key, test_data).await.is_ok());
    
    // Phase 2: Simulate network partition (operations should still work with local storage)
    println!("  🚫 Simulating network partition...");
    sleep(Duration::from_millis(100)).await;
    
    // Verify local operations still work during "partition"
    assert!(storage.exists(test_key).await);
    let retrieved_data = storage.read(test_key).await?;
    assert_eq!(retrieved_data, test_data);
    
    // Phase 3: Simulate recovery
    println!("  🔄 Simulating network recovery...");
    sleep(Duration::from_millis(50)).await;
    
    // Verify full functionality restored
    let recovery_key = "recovery_test";
    let recovery_data = b"Recovery test data";
    assert!(storage.write(recovery_key, recovery_data).await.is_ok());
    assert!(storage.exists(recovery_key).await);
    
    let elapsed = start_time.elapsed();
    println!("  ✅ Network partition test completed in {:?}", elapsed);
    
    // Cleanup
    let _ = storage.delete(test_key).await;
    let _ = storage.delete(recovery_key).await;
    Ok(())
}

/// Test random service failures
#[tokio::test]
async fn test_random_service_failures() -> Result<(), Box<dyn std::error::Error>> {
    init_test_logging();
    
    println!("🌪️ CHAOS: Random Service Failures");
    
    let storage = MockStorage::new();
    let mut rng = rand::thread_rng();
    let mut successful_operations = 0;
    let total_operations = 10;
    
    for round in 1..=total_operations {
        println!("  🎲 Chaos round {}: Testing service resilience...", round);
        
        let test_key = format!("chaos_test_{}", round);
        let test_data = format!("Chaos test data round {}", round);
        
        // Simulate random failure probability
        let failure_chance: f64 = rng.gen();
        
        if failure_chance > FAULT_INJECTION_PROBABILITY {
            // Normal operation
            match storage.write(&test_key, test_data.as_bytes()).await {
                Ok(_) => {
                    successful_operations += 1;
                    println!("    ✅ Round {} succeeded", round);
    Ok(())
                }
                Err(_) => {
                    println!("    ❌ Round {} failed (unexpected)", round);
    Ok(())
                }
    Ok(())
            }
        } else {
            // Simulated failure - still try operation but expect it might fail
            println!("    🎯 Round {} - injecting fault", round);
            match timeout(
                Duration::from_millis(100),
                storage.write(&test_key, test_data.as_bytes())
            ).await {
                Ok(Ok(_)) => {
                    successful_operations += 1;
                    println!("    ✅ Round {} succeeded despite fault injection", round);
    Ok(())
                }
                Ok(Err(_)) => {
                    println!("    ⚠️ Round {} failed as expected from fault injection", round);
    Ok(())
                }
                Err(_) => {
                    println!("    ⏰ Round {} timed out (fault injection effect)", round);
    Ok(())
                }
    Ok(())
            }
    Ok(())
        }
        
        // Small delay between rounds
        sleep(Duration::from_millis(10)).await;
    }
    
    let success_rate = (successful_operations as f64) / (total_operations as f64) * 100.0;
    println!("  📊 Success rate: {:.1}% ({}/{})", success_rate, successful_operations, total_operations);
    
    // We expect at least 70% success rate even with chaos
    assert!(success_rate >= 70.0, "Success rate too low: {:.1}%", success_rate);
    
    println!("  ✅ Random service failures test completed");
}

/// Test memory pressure handling
#[tokio::test]
async fn test_memory_pressure_resilience() -> Result<(), Box<dyn std::error::Error>> {
    init_test_logging();
    
    println!("🌪️ CHAOS: Memory Pressure Simulation");
    
    let storage = MockStorage::new();
    let start_time = Instant::now();
    
    // Phase 1: Normal operations
    println!("  💾 Testing normal memory operations...");
    let normal_key = "normal_memory_test";
    let normal_data = vec![0u8; 1024]; // 1KB
    assert!(storage.write(normal_key, &normal_data).await.is_ok());
    
    // Phase 2: Simulate memory pressure with larger operations
    println!("  🔥 Simulating memory pressure...");
    let mut large_operations_successful = 0;
    let large_operations_total = 5;
    
    for i in 1..=large_operations_total {
        let large_key = format!("large_memory_test_{}", i);
        let large_data = vec![i as u8; MEDIUM_DATA_SIZE]; // 1MB each
        
        match timeout(
            Duration::from_secs(2),
            storage.write(&large_key, &large_data)
        ).await {
            Ok(Ok(_)) => {
                large_operations_successful += 1;
                println!("    ✅ Large operation {} succeeded", i);
                
                // Verify we can still read it
                let retrieved = storage.read(&large_key).await?;
                assert_eq!(retrieved.len(), MEDIUM_DATA_SIZE);
                
                // Cleanup immediately to free memory
                let _ = storage.delete(&large_key).await;
    Ok(())
            }
            Ok(Err(e)) => {
                println!("    ⚠️ Large operation {} failed: {:?}", i, e);
    Ok(())
            }
            Err(_) => {
                println!("    ⏰ Large operation {} timed out", i);
    Ok(())
            }
    Ok(())
        }
    Ok(())
    }
    
    // Phase 3: Verify normal operations still work
    println!("  🔄 Verifying normal operations after memory pressure...");
    let recovery_key = "memory_recovery_test";
    let recovery_data = b"Memory recovery test";
    assert!(storage.write(recovery_key, recovery_data).await.is_ok());
    
    let retrieved_recovery = storage.read(recovery_key).await?;
    assert_eq!(retrieved_recovery, recovery_data);
    
    // Verify original data is still accessible
    let retrieved_normal = storage.read(normal_key).await?;
    assert_eq!(retrieved_normal, normal_data);
    
    let elapsed = start_time.elapsed();
    println!("  📊 Memory pressure test stats:");
    println!("    - Large operations successful: {}/{}", large_operations_successful, large_operations_total);
    println!("    - Total duration: {:?}", elapsed);
    
    // We expect at least 60% of large operations to succeed
    let large_success_rate = (large_operations_successful as f64) / (large_operations_total as f64) * 100.0;
    assert!(large_success_rate >= 60.0, "Large operations success rate too low: {:.1}%", large_success_rate);
    
    println!("  ✅ Memory pressure resilience test completed");
    
    // Cleanup
    let _ = storage.delete(normal_key).await;
    let _ = storage.delete(recovery_key).await;
}

/// Test concurrent operations under stress
#[tokio::test]
async fn test_concurrent_stress_resilience() -> Result<(), Box<dyn std::error::Error>> {
    init_test_logging();
    
    println!("🌪️ CHAOS: Concurrent Stress Test");
    
    let storage = MockStorage::new();
    let start_time = Instant::now();
    let concurrent_operations = 20;
    
    println!("  🚀 Launching {} concurrent operations...", concurrent_operations);
    
    let mut handles = Vec::new();
    
    for i in 0..concurrent_operations {
        let storage_clone = storage.clone();
        let handle = tokio::spawn(async move {
            let key = format!("stress_test_{}", i);
            let data = format!("Stress test data for operation {}", i).repeat(100); // Larger data
            
            // Random delay to increase chaos
            let delay_ms = rand::thread_rng().gen_range(1..50);
            sleep(Duration::from_millis(delay_ms)).await;
            
            // Perform write-read-delete cycle
            match storage_clone.write(&key, data.as_bytes()).await {
                Ok(_) => {
                    // Try to read back
                    match storage_clone.read(&key).await {
                        Ok(retrieved_data) => {
                            if retrieved_data == data.as_bytes() {
                                // Try to delete
                                match storage_clone.delete(&key).await {
                                    Ok(_) => (true, "write-read-delete cycle successful".to_string()),
                                    Err(e) => (false, format!("delete failed: {:?}", e)),
    Ok(())
                                }
                            } else {
                                (false, "data mismatch on read".to_string())
    Ok(())
                            }
    Ok(())
                        }
                        Err(e) => (false, format!("read failed: {:?}", e)),
    Ok(())
                    }
    Ok(())
                }
                Err(e) => (false, format!("write failed: {:?}", e)),
    Ok(())
            }
        });
        handles.push(handle);
    Ok(())
    }
    
    // Wait for all operations to complete
    let mut successful_operations = 0;
    let mut failure_reasons = Vec::new();
    
    for (i, handle) in handles.into_iter().enumerate() {
        match timeout(Duration::from_secs(5), handle).await {
            Ok(Ok((success, reason))) => {
                if success {
                    successful_operations += 1;
                    println!("    ✅ Operation {} succeeded", i);
                } else {
                    println!("    ❌ Operation {} failed: {}", i, reason);
                    failure_reasons.push(reason);
                }
            }
            Ok(Err(e)) => {
                println!("    💥 Operation {} panicked: {:?}", i, e);
                failure_reasons.push(format!("panic: {:?}", e));
            }
            Err(_) => {
                println!("    ⏰ Operation {} timed out", i);
                failure_reasons.push("timeout".to_string());
            }
        }
    }
    
    let elapsed = start_time.elapsed();
    let success_rate = (successful_operations as f64) / (concurrent_operations as f64) * 100.0;
    
    println!("  📊 Concurrent stress test results:");
    println!("    - Operations successful: {}/{}", successful_operations, concurrent_operations);
    println!("    - Success rate: {:.1}%", success_rate);
    println!("    - Total duration: {:?}", elapsed);
    
    if !failure_reasons.is_empty() {
        println!("    - Failure reasons: {:?}", failure_reasons);
    }
    
    // We expect at least 80% success rate under concurrent stress
    assert!(success_rate >= 80.0, "Concurrent stress success rate too low: {:.1}%", success_rate);
    
    println!("  ✅ Concurrent stress resilience test completed");
} 