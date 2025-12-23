//! Timeout type tests

use super::super::client::*;
use std::time::Duration;

#[test]
fn test_timeout_new() {
    let timeout = TimeoutMs::new(5000);
    assert_eq!(timeout.as_duration(), Duration::from_millis(5000));
}

#[test]
fn test_timeout_as_duration() {
    let timeout = TimeoutMs::new(1000);
    assert_eq!(timeout.as_duration(), Duration::from_secs(1));
}

#[test]
fn test_timeout_zero() {
    let timeout = TimeoutMs::new(0);
    assert_eq!(timeout.as_duration(), Duration::from_millis(0));
}

#[test]
fn test_timeout_large() {
    let timeout = TimeoutMs::new(60000);
    assert_eq!(timeout.as_duration(), Duration::from_secs(60));
}

#[test]
fn test_timeout_comprehensive() {
    // Common timeout values
    let timeout_1s = TimeoutMs::new(1000);
    assert_eq!(timeout_1s, Duration::from_secs(1));
    
    let timeout_5s = TimeoutMs::new(5000);
    assert_eq!(timeout_5s, Duration::from_secs(5));
    
    let timeout_30s = TimeoutMs::new(30000);
    assert_eq!(timeout_30s, Duration::from_secs(30));
    
    let timeout_1m = TimeoutMs::new(60000);
    assert_eq!(timeout_1m, Duration::from_secs(60));
}

#[test]
fn test_timeout_edge_cases() {
    // Minimum timeout
    let min_timeout = TimeoutMs::new(0);
    assert_eq!(min_timeout, Duration::from_millis(0));
    
    // Very small timeout
    let small_timeout = TimeoutMs::new(1);
    assert_eq!(small_timeout, Duration::from_millis(1));
    
    // Large timeout (1 hour)
    let large_timeout = TimeoutMs::new(3600000);
    assert_eq!(large_timeout, Duration::from_secs(3600));
}

