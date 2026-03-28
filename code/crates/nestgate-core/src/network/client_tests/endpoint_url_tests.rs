// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Endpoint Url Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: ENDPOINT URL TESTS

use super::super::client::*;

// ==================== ENDPOINT URL TESTS ====================
#[test]
fn test_endpoint_url_construction() {
    let port = Port::new(8443).expect("Network operation failed");
    let endpoint = Endpoint::https("secure.api.com".to_string(), port);

    assert_eq!(endpoint.base_url(), "https://secure.api.com:8443");
}

#[test]
fn test_endpoint_with_various_ports() {
    let ports_and_urls = vec![
        (80, "http://example.com:80"),
        (443, "http://example.com:443"),
        (8080, "http://example.com:8080"),
        (3000, "http://example.com:3000"),
    ];

    for (port_num, expected_url) in ports_and_urls {
        let port = Port::new(port_num).expect("Network operation failed");
        let endpoint = Endpoint::http("example.com".to_string(), port);
        assert_eq!(endpoint.base_url(), expected_url);
    }
}
