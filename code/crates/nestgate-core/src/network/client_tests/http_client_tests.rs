// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Http Client Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: HTTP CLIENT TESTS

use super::super::client::*;

// ==================== HTTP CLIENT TESTS ====================
#[tokio::test]
async fn test_http_client_default() {
    let client = HttpClient::default();
    let stats = client.stats();
    assert_eq!(stats.total_connections, 0);
}

#[tokio::test]
async fn test_http_client_with_config() {
    let config = ClientConfig::<30000>::default();
    let client = HttpClient::new(config);
    let stats = client.stats();
    assert_eq!(stats.total_connections, 0);
}

#[tokio::test]
async fn test_http_client_stats() {
    let client = HttpClient::default();
    let stats = client.stats();

    assert_eq!(stats.active_requests, 0);
    assert_eq!(stats.total_connections, 0);
    assert_eq!(stats.idle_connections, 0);
}
