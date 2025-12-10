//! Client Stats Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: CLIENT STATS TESTS

use super::super::client::*;

// ==================== CLIENT STATS TESTS ====================
#[test]
fn test_client_stats_serialization() {
    let stats = ClientStats {
        total_connections: 10,
        active_requests: 5,
        total_requests: 100,
        failed_requests: 2,
    };

    let json = serde_json::to_string(&stats);
    assert!(json.is_ok());
}

#[test]
fn test_client_stats_default_values() {
    let stats = ClientStats {
        total_connections: 0,
        active_requests: 0,
        total_requests: 0,
        failed_requests: 0,
    };

    assert_eq!(stats.total_connections, 0);
    assert_eq!(stats.active_requests, 0);
    assert_eq!(stats.total_requests, 0);
    assert_eq!(stats.failed_requests, 0);
}
