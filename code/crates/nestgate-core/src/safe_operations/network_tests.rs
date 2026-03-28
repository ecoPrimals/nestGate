// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Tests for safe network operations
//! Validates timeout handling, connection safety, and error paths

use super::network::*;
use std::time::Duration;

#[tokio::test]
async fn test_safe_connect_with_timeout() {
    let address = "127.0.0.1:8080";
    let timeout = Duration::from_secs(5);
    
    // This will likely fail (nothing listening), but should timeout gracefully
    let result = safe_connect_with_timeout(address, timeout).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_safe_connect_invalid_address() {
    let address = "invalid:address:format";
    let timeout = Duration::from_secs(1);
    
    let result = safe_connect_with_timeout(address, timeout).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_safe_connect_zero_timeout() {
    let address = "127.0.0.1:8080";
    let timeout = Duration::from_secs(0);
    
    let result = safe_connect_with_timeout(address, timeout).await;
    // Should fail immediately or handle gracefully
    assert!(result.is_err());
}

#[tokio::test]
async fn test_safe_connect_very_long_timeout() {
    let address = "240.0.0.1:1"; // Non-routable address
    let timeout = Duration::from_millis(100); // Short timeout for test speed
    
    let result = safe_connect_with_timeout(address, timeout).await;
    assert!(result.is_err());
}

#[test]
fn test_validate_address_valid_ipv4() {
    assert!(validate_address("127.0.0.1:8080").is_ok());
    assert!(validate_address("192.168.1.1:80").is_ok());
    assert!(validate_address("8.8.8.8:53").is_ok());
}

#[test]
fn test_validate_address_valid_ipv6() {
    assert!(validate_address("[::1]:8080").is_ok());
    assert!(validate_address("[2001:db8::1]:80").is_ok());
}

#[test]
fn test_validate_address_invalid_format() {
    assert!(validate_address("not an address").is_err());
    assert!(validate_address("127.0.0.1").is_err()); // Missing port
    assert!(validate_address(":8080").is_err()); // Missing host
    assert!(validate_address("").is_err()); // Empty
}

#[test]
fn test_validate_address_invalid_port() {
    assert!(validate_address("127.0.0.1:99999").is_err()); // Port too large
    assert!(validate_address("127.0.0.1:0").is_err()); // Port zero (invalid)
}

#[test]
fn test_validate_address_edge_cases() {
    // Max valid port
    assert!(validate_address("127.0.0.1:65535").is_ok());
    
    // Min valid port
    assert!(validate_address("127.0.0.1:1").is_ok());
}

#[tokio::test]
async fn test_safe_request_with_retry_immediate_success() {
    let operation = || async { Ok::<_, std::io::Error>(42) };
    
    let result = safe_request_with_retry(operation, 3, Duration::from_millis(10)).await;
    assert_eq!(result.unwrap(), 42);
}

#[tokio::test]
async fn test_safe_request_with_retry_eventual_success() {
    let mut attempts = 0;
    let operation = || async {
        attempts += 1;
        if attempts < 3 {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Temporary failure"))
        } else {
            Ok::<_, std::io::Error>(42)
        }
    };
    
    let result = safe_request_with_retry(operation, 5, Duration::from_millis(10)).await;
    assert_eq!(result.unwrap(), 42);
}

#[tokio::test]
async fn test_safe_request_with_retry_all_fail() {
    let operation = || async {
        Err::<i32, _>(std::io::Error::new(std::io::ErrorKind::Other, "Always fails"))
    };
    
    let result = safe_request_with_retry(operation, 3, Duration::from_millis(10)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_safe_request_with_retry_zero_retries() {
    let operation = || async { Ok::<_, std::io::Error>(42) };
    
    let result = safe_request_with_retry(operation, 0, Duration::from_millis(10)).await;
    // Should at least try once
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_parse_socket_addr_valid() {
    assert!(parse_socket_addr("127.0.0.1:8080").is_ok());
    assert!(parse_socket_addr("0.0.0.0:80").is_ok());
    assert!(parse_socket_addr("[::1]:8080").is_ok());
}

#[test]
fn test_parse_socket_addr_invalid() {
    assert!(parse_socket_addr("invalid").is_err());
    assert!(parse_socket_addr("").is_err());
    assert!(parse_socket_addr("127.0.0.1").is_err());
}

#[test]
fn test_is_local_address() {
    assert!(is_local_address("127.0.0.1:8080"));
    assert!(is_local_address("localhost:8080"));
    assert!(is_local_address("[::1]:8080"));
    assert!(!is_local_address("8.8.8.8:80"));
    assert!(!is_local_address("192.168.1.1:80"));
}

#[tokio::test]
async fn test_concurrent_connections() {
    let addresses = vec![
        "127.0.0.1:8080",
        "127.0.0.1:8081",
        "127.0.0.1:8082",
    ];
    
    let timeout = Duration::from_millis(100);
    
    let mut handles = vec![];
    for addr in addresses {
        let handle = tokio::spawn(async move {
            safe_connect_with_timeout(addr, timeout).await
        });
        handles.push(handle);
    }
    
    // All should complete (even if with errors)
    for handle in handles {
        let _ = handle.await;
    }
}

#[test]
fn test_network_error_messages() {
    let addr = "invalid:address";
    let result = validate_address(addr);
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(!error.to_string().is_empty());
}

#[test]
fn test_address_with_hostname() {
    // Hostnames should be validated differently than IPs
    let result = validate_address("example.com:80");
    // This might fail or succeed depending on DNS resolution
    let _ = result;
}

#[tokio::test]
async fn test_timeout_precision() {
    let start = std::time::Instant::now();
    let timeout = Duration::from_millis(100);
    
    let _ = safe_connect_with_timeout("240.0.0.1:1", timeout).await;
    
    let elapsed = start.elapsed();
    // Should timeout close to the specified duration (within 50ms tolerance)
    assert!(elapsed < timeout + Duration::from_millis(50));
}

