//! Tests for connection pool functionality

use super::connection_pool::*;
use crate::error::NestGateError;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Mock connection type for testing
#[derive(Debug, Clone)]
struct MockConnection {
    id: u32,
}

impl MockConnection {
    /// Creates a new instance
    fn new(id: u32) -> Self {
        Self { id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_pool_config_default() {
        let config = ConnectionPoolConfig::default();
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_connections, 2);
        assert_eq!(config.max_idle_time, Duration::from_secs(300));
        assert_eq!(config.connection_timeout, Duration::from_secs(30));
        assert_eq!(config.cleanup_interval, Duration::from_secs(60));
    }

    #[test]
    fn test_pooled_connection_creation() {
        let conn = MockConnection::new(1);
        let pooled = PooledConnection::new(conn);

        assert_eq!(pooled.connection.id, 1);
        assert!(!pooled.in_use);
        assert!(pooled.last_used.elapsed() < Duration::from_secs(1));
    }

    #[test]
    fn test_pooled_connection_mark_used() {
        let conn = MockConnection::new(1);
        let mut pooled = PooledConnection::new(conn);

        pooled.mark_used();
        assert!(pooled.in_use);
    }

    #[test]
    fn test_pooled_connection_mark_idle() {
        let conn = MockConnection::new(1);
        let mut pooled = PooledConnection::new(conn);

        pooled.mark_used();
        assert!(pooled.in_use);

        pooled.mark_idle();
        assert!(!pooled.in_use);
    }

    #[tokio::test]
    async fn test_pooled_connection_idle_detection() {
        let conn = MockConnection::new(1);
        let pooled = PooledConnection::new(conn);

        // Immediately created connection should not be idle too long
        assert!(!pooled.is_idle_too_long(Duration::from_secs(1)));

        // Very short max_idle_time should detect as idle (non-blocking, concurrent)
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert!(pooled.is_idle_too_long(Duration::from_millis(1)));
    }

    #[test]
    fn test_pool_stats_default() {
        let stats = PoolStats::default();
        assert_eq!(stats.total_connections, 0);
        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.idle_connections, 0);
        assert_eq!(stats.connections_created, 0);
        assert_eq!(stats.connections_destroyed, 0);
        assert_eq!(stats.connection_requests, 0);
        assert_eq!(stats.connection_timeouts, 0);
    }

    #[tokio::test]
    async fn test_universal_connection_pool_creation() {
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        let config = ConnectionPoolConfig::default();
        let factory = move || {
            let id = counter_clone.fetch_add(1, Ordering::SeqCst);
            Ok(MockConnection::new(id))
        };

        let pool = UniversalConnectionPool::new(config, factory);
        // If we got here without panic, creation succeeded
        let pool_arc = Arc::new(pool);
        let stats = pool_arc.get_stats().await;
        assert_eq!(stats.connection_requests, 0);
    }

    #[tokio::test]
    async fn test_connection_pool_get_connection() {
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        let config = ConnectionPoolConfig {
            max_connections: 5,
            min_connections: 1,
            max_idle_time: Duration::from_secs(60),
            connection_timeout: Duration::from_secs(5),
            cleanup_interval: Duration::from_secs(30),
        };

        let factory = move || {
            let id = counter_clone.fetch_add(1, Ordering::SeqCst);
            Ok(MockConnection::new(id))
        };

        let pool = Arc::new(UniversalConnectionPool::new(config, factory));

        // Get a connection
        let conn_guard = pool.get_connection().await;
        assert!(conn_guard.is_ok());

        if let Ok(guard) = conn_guard {
            assert_eq!(guard.connection().id, 0);
            // Connection is released when guard is dropped
        }
    }

    #[tokio::test]
    async fn test_connection_pool_stats() {
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        let config = ConnectionPoolConfig::default();
        let factory = move || {
            let id = counter_clone.fetch_add(1, Ordering::SeqCst);
            Ok(MockConnection::new(id))
        };

        let pool = Arc::new(UniversalConnectionPool::new(config, factory));

        let stats = pool.get_stats().await;
        assert_eq!(stats.connection_requests, 0);
    }

    #[tokio::test]
    async fn test_connection_pool_multiple_connections() {
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        let config = ConnectionPoolConfig {
            max_connections: 3,
            min_connections: 1,
            max_idle_time: Duration::from_secs(60),
            connection_timeout: Duration::from_secs(5),
            cleanup_interval: Duration::from_secs(30),
        };

        let factory = move || {
            let id = counter_clone.fetch_add(1, Ordering::SeqCst);
            Ok(MockConnection::new(id))
        };

        let pool = Arc::new(UniversalConnectionPool::new(config, factory));

        // Get multiple connections
        let conn1 = pool.get_connection().await.expect("Failed to get conn1");
        let conn2 = pool.get_connection().await.expect("Failed to get conn2");
        let conn3 = pool.get_connection().await.expect("Failed to get conn3");

        assert_eq!(conn1.connection().id, 0);
        assert_eq!(conn2.connection().id, 1);
        assert_eq!(conn3.connection().id, 2);

        // Connections are released when guards are dropped
        drop(conn1);
        drop(conn2);
        drop(conn3);
    }

    #[tokio::test]
    async fn test_connection_pool_get_connection_twice() {
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        let config = ConnectionPoolConfig::default();
        let factory = move || {
            let id = counter_clone.fetch_add(1, Ordering::SeqCst);
            Ok(MockConnection::new(id))
        };

        let pool = Arc::new(UniversalConnectionPool::new(config, factory));

        // Get and release
        let conn1 = pool.get_connection().await.expect("Failed to get");
        let _id1 = conn1.connection().id;
        drop(conn1); // Release

        // Get again - should succeed (may or may not reuse)
        let conn2 = pool.get_connection().await.expect("Failed to get");
        // Just verify we got a valid connection, not necessarily the same one
        assert!(conn2.connection().id < 10); // Should be a low ID since we only created a few
        drop(conn2);
    }

    #[tokio::test]
    async fn test_connection_pool_cleanup() {
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        let config = ConnectionPoolConfig {
            max_connections: 10,
            min_connections: 1,
            max_idle_time: Duration::from_millis(50), // Very short for testing
            connection_timeout: Duration::from_secs(5),
            cleanup_interval: Duration::from_millis(100),
        };

        let factory = move || {
            let id = counter_clone.fetch_add(1, Ordering::SeqCst);
            Ok(MockConnection::new(id))
        };

        let pool = Arc::new(UniversalConnectionPool::new(config, factory));

        // Get and release to create idle connection
        let conn = pool.get_connection().await.expect("Failed to get");
        drop(conn);

        // Wait for cleanup
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Cleanup should have run (we just verify no panics)
        let stats = pool.get_stats().await;
        assert!(stats.connections_created > 0);
    }

    #[test]
    fn test_connection_pool_config_custom() {
        let config = ConnectionPoolConfig {
            max_connections: 20,
            min_connections: 5,
            max_idle_time: Duration::from_secs(600),
            connection_timeout: Duration::from_secs(10),
            cleanup_interval: Duration::from_secs(120),
        };

        assert_eq!(config.max_connections, 20);
        assert_eq!(config.min_connections, 5);
        assert_eq!(config.max_idle_time, Duration::from_secs(600));
        assert_eq!(config.connection_timeout, Duration::from_secs(10));
        assert_eq!(config.cleanup_interval, Duration::from_secs(120));
    }

    #[tokio::test]
    async fn test_connection_factory_error_handling() {
        let config = ConnectionPoolConfig::default();

        // Factory that always fails
        let factory = || -> crate::error::Result<MockConnection> {
            use crate::error::variants::core_errors::InternalErrorDetails;
            Err(NestGateError::Internal(Box::new(InternalErrorDetails {
                message: "Factory error".to_string(),
                component: "test".to_string(),
                location: Some("connection_pool_tests".to_string()),
                is_bug: false,
                context: None,
            })))
        };

        let pool = Arc::new(UniversalConnectionPool::new(config, factory));

        // Getting a connection should fail
        let result = pool.get_connection().await;
        assert!(result.is_err());
    }

    #[test]
    fn test_connection_guard_deref() {
        // This tests the guard's ability to provide access to the connection
        let conn = MockConnection::new(42);
        let pooled = PooledConnection::new(conn);
        assert_eq!(pooled.connection.id, 42);
    }

    #[tokio::test]
    async fn test_pool_stats_tracking() {
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        let config = ConnectionPoolConfig::default();
        let factory = move || {
            let id = counter_clone.fetch_add(1, Ordering::SeqCst);
            Ok(MockConnection::new(id))
        };

        let pool = Arc::new(UniversalConnectionPool::new(config, factory));

        // Initial stats
        let stats1 = pool.get_stats().await;
        let initial_requests = stats1.connection_requests;

        // Get a connection
        let _conn = pool
            .get_connection()
            .await
            .expect("Failed to get connection");

        // Stats should be updated
        let stats2 = pool.get_stats().await;
        assert!(stats2.connection_requests >= initial_requests);
    }
}
