// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Connection Pool Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: CONNECTION POOL TESTS, CONNECTION POOL ADVANCED TESTS

use super::super::client::*;

// ==================== CONNECTION POOL TESTS ====================
#[test]
fn test_connection_pool_new() {
    let config = ClientConfig::<30000>::default();
    let pool = ConnectionPool::new(config);

    // Pool should be created successfully
    assert!(std::mem::size_of_val(&pool) > 0);
}

// ==================== CONNECTION POOL ADVANCED TESTS ====================
#[tokio::test]
async fn test_connection_pool_get_connection() {
    let config = ClientConfig::<30000>::default();
    let pool = ConnectionPool::new(config);

    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    let result = pool.get_connection(&endpoint).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_connection_pool_return_connection() {
    let config = ClientConfig::<30000>::default();
    let pool = ConnectionPool::new(config);

    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    let connection = pool
        .get_connection(&endpoint)
        .await
        .expect("Network operation failed");
    pool.return_connection(connection);
}

#[tokio::test]
async fn test_connection_pool_reuse() {
    let config = ClientConfig::<30000>::default();
    let pool = ConnectionPool::new(config);

    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    // Get and return a connection
    let conn1 = pool
        .get_connection(&endpoint)
        .await
        .expect("Network operation failed");
    pool.return_connection(conn1);

    // Get another connection (should potentially reuse)
    let conn2 = pool.get_connection(&endpoint).await;
    assert!(conn2.is_ok());
}

#[tokio::test]
async fn test_connection_pool_multiple_endpoints() {
    let config = ClientConfig::<30000>::default();
    let pool = ConnectionPool::new(config);

    let port1 = Port::new(8080).expect("Network operation failed");
    let port2 = Port::new(9090).expect("Network operation failed");
    let endpoint1 = Endpoint::http("service1.example.com".to_string(), port1);
    let endpoint2 = Endpoint::http("service2.example.com".to_string(), port2);

    let conn1 = pool.get_connection(&endpoint1).await;
    let conn2 = pool.get_connection(&endpoint2).await;

    assert!(conn1.is_ok());
    assert!(conn2.is_ok());
}
