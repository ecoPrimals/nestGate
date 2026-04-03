// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CACHE MATH INTEGRATION TESTS**
//!
//! Tests for cache-related mathematical calculations and algorithms

use crate::common::*;
use nestgate_core::cache_math::*;

/// Test cache size calculations
#[test]
fn test_cache_size_calculations() -> Result<(), Box<dyn std::error::Error>> {
    // Test basic cache size calculations
    let cache_size = calculate_optimal_cache_size(1024 * 1024 * 1024); // 1GB memory
    assert!(cache_size > 0);
    assert!(cache_size <= 1024 * 1024 * 1024); // Should not exceed available memory
    
    // Test cache size for different memory amounts
    let small_cache = calculate_optimal_cache_size(128 * 1024 * 1024); // 128MB
    let large_cache = calculate_optimal_cache_size(8 * 1024 * 1024 * 1024); // 8GB
    
    assert!(small_cache < large_cache); // Larger memory should allow larger cache
    assert!(small_cache > 0);
    assert!(large_cache > 0);
    Ok(())
}

/// Test cache hit ratio calculations
#[test]
fn test_cache_hit_ratio() -> Result<(), Box<dyn std::error::Error>> {
    // Test perfect hit ratio
    assert_eq!(calculate_hit_ratio(100, 0), 1.0);
    
    // Test no hits
    assert_eq!(calculate_hit_ratio(0, 100), 0.0);
    
    // Test mixed scenarios
    assert_eq!(calculate_hit_ratio(75, 25), 0.75);
    assert_eq!(calculate_hit_ratio(50, 50), 0.5);
    
    // Test edge cases
    assert_eq!(calculate_hit_ratio(0, 0), 0.0); // No requests
    Ok(())
}

/// Test cache eviction algorithms
#[test]
fn test_cache_eviction_scoring() -> Result<(), Box<dyn std::error::Error>> {
    // Test LRU scoring
    let now = std::time::SystemTime::now();
    let old_time = now - std::time::Duration::from_secs(3600); // 1 hour ago
    let recent_time = now - std::time::Duration::from_secs(60); // 1 minute ago
    
    let old_score = calculate_lru_score(old_time, now);
    let recent_score = calculate_lru_score(recent_time, now);
    
    // More recent items should have higher scores (less likely to be evicted)
    assert!(recent_score > old_score);
    assert!(old_score >= 0.0);
    assert!(recent_score >= 0.0);
    Ok(())
}

/// Test cache memory efficiency calculations
#[test]
fn test_memory_efficiency() -> Result<(), Box<dyn std::error::Error>> {
    // Test memory overhead calculations
    let data_size = 1024; // 1KB of actual data
    let overhead = calculate_memory_overhead(data_size);
    
    assert!(overhead >= 0);
    assert!(overhead < data_size); // Overhead should be reasonable
    
    // Test total memory usage
    let total_usage = calculate_total_memory_usage(data_size, overhead);
    assert_eq!(total_usage, data_size + overhead);
    Ok(())
}

/// Test cache performance predictions
#[test]
fn test_performance_predictions() -> Result<(), Box<dyn std::error::Error>> {
    // Test access time predictions
    let cache_access_time = predict_cache_access_time(1000); // 1000 entries
    let large_cache_time = predict_cache_access_time(100000); // 100k entries
    
    assert!(cache_access_time > 0.0);
    assert!(large_cache_time > 0.0);
    
    // Larger caches might have slightly longer access times
    // but should still be reasonable
    assert!(large_cache_time >= cache_access_time);
    Ok(())
}

/// Test cache optimization algorithms
#[test]
fn test_cache_optimization() -> Result<(), Box<dyn std::error::Error>> {
    // Test optimal cache tier distribution
    let total_size = 1024 * 1024 * 1024; // 1GB
    let tier_distribution = calculate_optimal_tier_distribution(total_size);
    
    // Should have reasonable distribution across tiers
    assert!(tier_distribution.hot_tier_size > 0);
    assert!(tier_distribution.warm_tier_size > 0);
    assert!(tier_distribution.cold_tier_size >= 0);
    
    // Total should not exceed available size
    let total_distributed = tier_distribution.hot_tier_size + 
                           tier_distribution.warm_tier_size + 
                           tier_distribution.cold_tier_size;
    assert!(total_distributed <= total_size);
    Ok(())
}

/// Test mathematical utility functions
#[test]
fn test_math_utilities() -> Result<(), Box<dyn std::error::Error>> {
    // Test power of 2 calculations
    assert!(is_power_of_two(1024));
    assert!(is_power_of_two(2048));
    assert!(!is_power_of_two(1000));
    assert!(!is_power_of_two(0));
    
    // Test next power of 2
    assert_eq!(next_power_of_two(1000), 1024);
    assert_eq!(next_power_of_two(1024), 1024);
    assert_eq!(next_power_of_two(1025), 2048);
    
    // Test logarithm calculations
    assert_eq!(log2_floor(1024), 10);
    assert_eq!(log2_floor(1000), 9);
    assert_eq!(log2_floor(1), 0);
    Ok(())
}

/// Test statistical calculations for cache performance
#[test]
fn test_cache_statistics() -> Result<(), Box<dyn std::error::Error>> {
    let access_times = vec![0.1, 0.2, 0.15, 0.3, 0.25, 0.18, 0.22];
    
    // Test average calculation
    let average = calculate_average(&access_times);
    assert!(average > 0.0);
    assert!(average < 1.0); // Should be reasonable
    
    // Test median calculation
    let median = calculate_median(&access_times);
    assert!(median > 0.0);
    assert!(median < 1.0);
    
    // Test standard deviation
    let std_dev = calculate_standard_deviation(&access_times);
    assert!(std_dev >= 0.0);
    assert!(std_dev < average); // Should be reasonable relative to average
    Ok(())
}

/// Test cache load balancing calculations
#[test]
fn test_load_balancing() -> Result<(), Box<dyn std::error::Error>> {
    let cache_loads = vec![0.8, 0.6, 0.9, 0.4, 0.7]; // Cache utilization ratios
    
    // Test load balancing score
    let balance_score = calculate_load_balance_score(&cache_loads);
    assert!(balance_score >= 0.0);
    assert!(balance_score <= 1.0);
    
    // Test perfectly balanced scenario
    let balanced_loads = vec![0.5, 0.5, 0.5, 0.5];
    let perfect_score = calculate_load_balance_score(&balanced_loads);
    assert!(perfect_score > balance_score); // Should be better balanced
    Ok(())
}

/// Test cache replacement algorithms
#[test]
fn test_replacement_algorithms() -> Result<(), Box<dyn std::error::Error>> {
    // Test LFU (Least Frequently Used) scoring
    let access_counts = vec![10, 5, 15, 3, 8];
    let lfu_scores = calculate_lfu_scores(&access_counts);
    
    assert_eq!(lfu_scores.len(), access_counts.len());
    
    // Items with fewer accesses should have lower scores (more likely to be evicted)
    let min_access_idx = access_counts.iter().position(|&x| x == 3)?;
    let max_access_idx = access_counts.iter().position(|&x| x == 15)?;
    
    assert!(lfu_scores[min_access_idx] < lfu_scores[max_access_idx]);
    Ok(())
}

/// Comprehensive cache math integration test
#[test]
fn test_comprehensive_cache_math() -> Result<(), Box<dyn std::error::Error>> {
    // Simulate a realistic cache scenario
    let total_memory = 2 * 1024 * 1024 * 1024; // 2GB
    let optimal_cache_size = calculate_optimal_cache_size(total_memory);
    
    // Calculate tier distribution
    let distribution = calculate_optimal_tier_distribution(optimal_cache_size);
    
    // Simulate cache performance over time
    let mut hit_counts = Vec::new();
    let mut miss_counts = Vec::new();
    
    for hour in 0..24 {
        // Simulate different access patterns throughout the day
        let base_hits = 1000;
        let base_misses = 200;
        
        // Peak hours (9-17) have better hit ratios
        let hits = if hour >= 9 && hour <= 17 {
            base_hits + (hour - 9) * 50
        } else {
            base_hits - 100
        };
        
        let misses = base_misses - (hits - base_hits) / 4;
        
        hit_counts.push(hits.max(0));
        miss_counts.push(misses.max(0));
    Ok(())
    }
    
    // Calculate performance metrics
    let total_hits: u64 = hit_counts.iter().sum();
    let total_misses: u64 = miss_counts.iter().sum();
    let overall_hit_ratio = calculate_hit_ratio(total_hits, total_misses);
    
    // Verify reasonable performance
    assert!(overall_hit_ratio > 0.7); // Should have good hit ratio
    assert!(overall_hit_ratio < 1.0);
    
    // Test that our calculations are consistent
    assert!(optimal_cache_size > 0);
    assert!(optimal_cache_size <= total_memory);
    assert!(distribution.hot_tier_size + distribution.warm_tier_size + distribution.cold_tier_size <= optimal_cache_size);
    Ok(())
} 