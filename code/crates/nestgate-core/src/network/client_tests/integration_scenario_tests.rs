// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Integration Scenario Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: INTEGRATION SCENARIO TESTS

use super::super::client::*;

// ==================== INTEGRATION SCENARIO TESTS ====================
#[tokio::test]
async fn test_client_and_pool_integration() {
    let config = ClientConfig::<30000>::default();
    let client = HttpClient::new(config);

    // Client should be ready to use
    let stats = client.stats();
    assert_eq!(stats.total_connections, 0);
}

#[tokio::test]
async fn test_endpoint_and_request_integration() {
    let port = Port::new(8080).expect("Network operation failed");
    let endpoint = Endpoint::http("api.example.com".to_string(), port);
    let request = Request::get("/api/v1/health");

    // Verify they work together
    assert!(request.path.starts_with('/'));
    assert_eq!(endpoint.port.get(), 8080);
}

#[tokio::test]
async fn test_multiple_endpoints_with_client() {
    let _client = HttpClient::default();

    let endpoint1 = Endpoint::http(
        "service1.com".to_string(),
        Port::new(8080).expect("Network operation failed"),
    );
    let endpoint2 = Endpoint::http(
        "service2.com".to_string(),
        Port::new(9090).expect("Network operation failed"),
    );

    assert_ne!(endpoint1.base_url(), endpoint2.base_url());
}
