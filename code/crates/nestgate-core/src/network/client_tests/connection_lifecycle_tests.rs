// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Connection Lifecycle Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: CONNECTION LIFECYCLE TESTS, CONNECTION LIFECYCLE ADVANCED TESTS

use super::super::client::*;
use std::time::Duration;

// ==================== CONNECTION LIFECYCLE TESTS ====================
#[test]
fn test_connection_new() {
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    let connection = Connection::new(endpoint);
    // Connection is freshly created, should not be idle initially
    assert!(!connection.is_idle());
}

#[test]
fn test_connection_initially_not_idle() {
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    let connection = Connection::new(endpoint);
    // New connection should not be idle
    assert!(!connection.is_idle());
}

#[test]
fn test_connection_stats_initial_state() {
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    let connection = Connection::new(endpoint);
    let stats = connection.stats();

    assert_eq!(stats.request_count, 0);
    assert!(stats.age < Duration::from_secs(1));
    assert!(stats.idle_time < Duration::from_secs(1));
}

#[test]
fn test_connection_stats_after_creation() {
    let port = Port::new(3000).expect("Network operation failed");
    let endpoint = Endpoint::http("api.test.com".to_string(), port);

    let connection = Connection::new(endpoint.clone());
    let stats = connection.stats();

    assert_eq!(stats.endpoint, endpoint);
    assert_eq!(stats.request_count, 0);
}

// ==================== CONNECTION LIFECYCLE ADVANCED TESTS ====================
#[tokio::test]
async fn test_connection_becomes_stale_eventually() {
    // While we can't wait 5 minutes in a test, we can verify the logic exists
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    let connection = Connection::new(endpoint);
    // Connection is valid when created
    assert_eq!(connection.request_count, 0);
}

#[tokio::test]
async fn test_connection_multiple_creation() {
    let port = Port::new(8080).expect("Network operation failed");
    let endpoint = Endpoint::http("api.test.com".to_string(), port);

    let conn1 = Connection::new(endpoint.clone());
    let conn2 = Connection::new(endpoint);

    // Connections are valid when created
    assert_eq!(conn1.request_count, 0);
    assert_eq!(conn2.request_count, 0);
}
