// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Connection Stats Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: CONNECTION STATS TESTS

use super::super::client::*;
use std::time::Duration;

// ==================== CONNECTION STATS TESTS ====================
#[test]
fn test_connection_stats_serialization() {
    let port = Port::new(8080).expect("Network operation failed");
    let endpoint = Endpoint::http("test.com".to_string(), port);

    let stats = ConnectionStats {
        endpoint,
        age: Duration::from_secs(10),
        idle_time: Duration::from_secs(5),
        request_count: 42,
    };

    let json = serde_json::to_string(&stats);
    assert!(json.is_ok());
}
