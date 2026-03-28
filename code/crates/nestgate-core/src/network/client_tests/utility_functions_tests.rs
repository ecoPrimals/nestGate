// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Utility Functions Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: UTILITY FUNCTIONS TESTS

use super::super::client::*;

// ==================== UTILITY FUNCTIONS TESTS ====================
#[test]
fn test_create_client() {
    let client = create_client();
    // Should not panic, client should be created
    assert!(std::mem::size_of_val(&client) > 0);
}

#[tokio::test]
async fn test_https_endpoint_helper() {
    let result = https_endpoint("secure.example.com", 443).await;
    assert!(result.is_ok());

    let endpoint = result.expect("Network operation failed");
    assert_eq!(endpoint.scheme, Scheme::Https);
    assert_eq!(endpoint.host, "secure.example.com");
    assert_eq!(endpoint.port.get(), 443);
}

#[tokio::test]
async fn test_http_endpoint_helper() {
    let result = http_endpoint("api.example.com", 8080).await;
    assert!(result.is_ok());

    let endpoint = result.expect("Network operation failed");
    assert_eq!(endpoint.scheme, Scheme::Http);
    assert_eq!(endpoint.host, "api.example.com");
    assert_eq!(endpoint.port.get(), 8080);
}

#[tokio::test]
async fn test_https_endpoint_invalid_port() {
    let result = https_endpoint("example.com", 0).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_http_endpoint_invalid_port() {
    let result = http_endpoint("example.com", 0).await;
    assert!(result.is_err());
}
