// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Connection Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: CONNECTION TESTS

use super::super::client::*;
use std::time::Duration;

// ==================== CONNECTION TESTS ====================
#[tokio::test]
async fn test_connection_is_alive_new() {
    use crate::constants::hardcoding::{addresses, runtime_fallback_ports};
    let port = Port::new(runtime_fallback_ports::HTTP).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);
    let connection = Connection::new(endpoint);

    // Connection is valid when created
    assert_eq!(connection.request_count, 0);
}

#[tokio::test]
async fn test_connection_stats() {
    use crate::constants::hardcoding::{addresses, runtime_fallback_ports};
    let port = Port::new(runtime_fallback_ports::HTTP).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);
    let connection = Connection::new(endpoint);

    let stats = connection.stats();
    assert_eq!(stats.request_count, 0);
    assert!(stats.age < Duration::from_secs(1));
}
