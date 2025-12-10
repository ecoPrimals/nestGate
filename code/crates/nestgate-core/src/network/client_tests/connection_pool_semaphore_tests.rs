//! Connection Pool Semaphore Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: CONNECTION POOL SEMAPHORE TESTS

use super::super::client::*;

// ==================== CONNECTION POOL SEMAPHORE TESTS ====================
#[tokio::test]
async fn test_connection_pool_respects_max_connections() {
    let mut config = ClientConfig::<30000>::default();
    config.max_connections = 2; // Very low limit for testing
    let pool = ConnectionPool::new(config);

    let port = Port::new(8080).expect("Network operation failed");
    let endpoint = Endpoint::http("test.example.com".to_string(), port);

    // Get first connection
    let conn1 = pool.get_connection(&endpoint).await;
    assert!(conn1.is_ok());

    // Get second connection
    let conn2 = pool.get_connection(&endpoint).await;
    assert!(conn2.is_ok());
}

#[tokio::test]
async fn test_connection_pool_per_host_limit() {
    let config = ClientConfig::<30000>::default();
    assert_eq!(config.max_connections_per_host, 10);
}
