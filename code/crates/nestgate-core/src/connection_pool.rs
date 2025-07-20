//! High-Performance Connection Pool System
//!
//! Provides optimized connection pooling for database and network operations
//! to handle high-concurrency scenarios efficiently.
//!
//! ## Performance Impact
//! - **Connection Reuse**: Eliminates connection establishment overhead
//! - **Configurable Limits**: Prevents resource exhaustion under load
//! - **Health Monitoring**: Automatic cleanup of stale connections
//! - **Load Balancing**: Distributes requests across available connections

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock, Semaphore};
use tracing::{debug, info, warn};

/// Type alias for connection factory function
type ConnectionFactory<T> = Arc<dyn Fn() -> Result<T> + Send + Sync>;
/// Type alias for health check function  
type HealthCheckFn<T> = Arc<dyn Fn(&T) -> Result<()> + Send + Sync>;

use crate::{NestGateError, Result};

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Minimum number of connections to maintain
    pub min_connections: usize,
    /// Maximum number of connections allowed
    pub max_connections: usize,
    /// Maximum time a connection can remain idle
    pub max_idle_time: Duration,
    /// Timeout for acquiring a connection from pool
    pub acquire_timeout: Duration,
    /// Interval for health check operations
    pub health_check_interval: Duration,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 2,
            max_connections: 20,
            max_idle_time: Duration::from_secs(300), // 5 minutes
            acquire_timeout: Duration::from_secs(10),
            health_check_interval: Duration::from_secs(30),
        }
    }
}

/// Generic connection pool for any connection type
pub struct ConnectionPool<T>
where
    T: Send + 'static,
{
    /// Available connections
    pool: Arc<Mutex<VecDeque<PooledConnection<T>>>>,
    /// Connection factory function
    factory: ConnectionFactory<T>,
    /// Health check function
    health_check: HealthCheckFn<T>,
    /// Pool configuration
    config: PoolConfig,
    /// Semaphore to limit concurrent connections
    semaphore: Arc<Semaphore>,
    /// Pool statistics
    stats: Arc<RwLock<PoolStats>>,
}

/// Wrapper for pooled connections with metadata
struct PooledConnection<T> {
    connection: T,
    #[allow(dead_code)]
    created_at: Instant,
    last_used: Instant,
}

/// Connection pool statistics
#[derive(Debug, Default, Clone)]
pub struct PoolStats {
    pub total_created: u64,
    pub total_acquired: u64,
    pub total_returned: u64,
    pub current_size: usize,
    pub active_connections: usize,
    pub failed_acquisitions: u64,
    pub health_check_failures: u64,
}

/// Guard that automatically returns connection to pool when dropped
pub struct ConnectionGuard<T>
where
    T: Send + 'static,
{
    connection: Option<T>,
    pool: Arc<Mutex<VecDeque<PooledConnection<T>>>>,
    stats: Arc<RwLock<PoolStats>>,
    semaphore: Arc<Semaphore>,
    acquired_at: Instant,
}

impl<T> ConnectionPool<T>
where
    T: Send + 'static,
{
    /// Create a new connection pool
    pub fn new<F, H>(factory: F, health_check: H, config: PoolConfig) -> Self
    where
        F: Fn() -> Result<T> + Send + Sync + 'static,
        H: Fn(&T) -> Result<()> + Send + Sync + 'static,
    {
        let semaphore = Arc::new(Semaphore::new(config.max_connections));

        let pool = Self {
            pool: Arc::new(Mutex::new(VecDeque::new())),
            factory: Arc::new(factory),
            health_check: Arc::new(health_check),
            config,
            semaphore,
            stats: Arc::new(RwLock::new(PoolStats::default())),
        };

        // Initialize minimum connections
        tokio::spawn(Self::initialize_pool(
            Arc::clone(&pool.pool),
            Arc::clone(&pool.factory),
            Arc::clone(&pool.stats),
            pool.config.min_connections,
        ));

        // Start health check task
        tokio::spawn(Self::health_check_loop(
            Arc::clone(&pool.pool),
            Arc::clone(&pool.health_check),
            Arc::clone(&pool.stats),
            pool.config.clone(),
        ));

        pool
    }

    /// Acquire a connection from the pool
    pub async fn acquire(&self) -> Result<ConnectionGuard<T>> {
        let start_time = Instant::now();

        // Wait for available slot with timeout
        let permit = tokio::time::timeout(self.config.acquire_timeout, self.semaphore.acquire())
            .await
            .map_err(|_| {
                NestGateError::Internal("Connection pool acquisition timeout".to_string())
            })?;

        let _permit = permit
            .map_err(|_| NestGateError::Internal("Connection pool semaphore error".to_string()))?;

        // Try to get existing connection from pool
        let connection = {
            let mut pool = self.pool.lock().await;

            // Remove stale connections
            let now = Instant::now();
            while let Some(conn) = pool.front() {
                if now.duration_since(conn.last_used) > self.config.max_idle_time {
                    pool.pop_front();
                    if let Ok(mut stats) = self.stats.try_write() {
                        stats.current_size = stats.current_size.saturating_sub(1);
                    }
                } else {
                    break;
                }
            }

            pool.pop_front()
        };

        let connection = match connection {
            Some(mut pooled_conn) => {
                // Health check existing connection
                if (self.health_check)(&pooled_conn.connection).is_err() {
                    // Connection unhealthy, create new one
                    (self.factory)()?
                } else {
                    pooled_conn.last_used = Instant::now();
                    pooled_conn.connection
                }
            }
            None => {
                // Create new connection
                let conn = (self.factory)()?;
                if let Ok(mut stats) = self.stats.try_write() {
                    stats.total_created += 1;
                }
                conn
            }
        };

        // Update statistics
        if let Ok(mut stats) = self.stats.try_write() {
            stats.total_acquired += 1;
            stats.active_connections += 1;
        }

        debug!(
            acquisition_time_ms = start_time.elapsed().as_millis(),
            "Connection acquired from pool"
        );

        Ok(ConnectionGuard {
            connection: Some(connection),
            pool: Arc::clone(&self.pool),
            stats: Arc::clone(&self.stats),
            semaphore: Arc::clone(&self.semaphore),
            acquired_at: start_time,
        })
    }

    /// Get current pool statistics
    pub async fn stats(&self) -> PoolStats {
        self.stats.read().await.clone()
    }

    /// Initialize minimum connections in the pool
    async fn initialize_pool(
        pool: Arc<Mutex<VecDeque<PooledConnection<T>>>>,
        factory: Arc<dyn Fn() -> Result<T> + Send + Sync>,
        stats: Arc<RwLock<PoolStats>>,
        min_connections: usize,
    ) {
        let mut pool_guard = pool.lock().await;

        for _ in 0..min_connections {
            if let Ok(connection) = factory() {
                let now = Instant::now();
                pool_guard.push_back(PooledConnection {
                    connection,
                    created_at: now,
                    last_used: now,
                });

                if let Ok(mut pool_stats) = stats.try_write() {
                    pool_stats.total_created += 1;
                    pool_stats.current_size += 1;
                }
            }
        }

        info!(
            "Initialized connection pool with {} connections",
            pool_guard.len()
        );
    }

    /// Health check loop for monitoring connection health
    async fn health_check_loop(
        pool: Arc<Mutex<VecDeque<PooledConnection<T>>>>,
        health_check: HealthCheckFn<T>,
        stats: Arc<RwLock<PoolStats>>,
        config: PoolConfig,
    ) {
        let mut interval = tokio::time::interval(config.health_check_interval);

        loop {
            interval.tick().await;

            let mut pool_guard = pool.lock().await;
            let mut healthy_connections = VecDeque::new();
            let mut failures = 0u64;

            // Check health of all connections
            while let Some(pooled_conn) = pool_guard.pop_front() {
                if health_check(&pooled_conn.connection).is_ok() {
                    healthy_connections.push_back(pooled_conn);
                } else {
                    failures += 1;
                }
            }

            // Replace pool contents with healthy connections
            *pool_guard = healthy_connections;

            // Update statistics
            if let Ok(mut pool_stats) = stats.try_write() {
                pool_stats.health_check_failures += failures;
                pool_stats.current_size = pool_guard.len();
            }

            if failures > 0 {
                warn!("Removed {} unhealthy connections from pool", failures);
            }
        }
    }
}

impl<T> ConnectionGuard<T>
where
    T: Send + 'static,
{
    /// Get reference to the connection
    pub fn connection(&self) -> Result<&T> {
        self.connection.as_ref().ok_or_else(|| {
            crate::NestGateError::Configuration("Connection has been consumed".to_string())
        })
    }

    /// Get mutable reference to the connection
    pub fn connection_mut(&mut self) -> Result<&mut T> {
        self.connection.as_mut().ok_or_else(|| {
            crate::NestGateError::Configuration("Connection has been consumed".to_string())
        })
    }
}

impl<T> Drop for ConnectionGuard<T>
where
    T: Send + 'static,
{
    fn drop(&mut self) {
        if let Some(connection) = self.connection.take() {
            let pool = Arc::clone(&self.pool);
            let stats = Arc::clone(&self.stats);
            let semaphore = Arc::clone(&self.semaphore);
            let acquired_at = self.acquired_at;

            tokio::spawn(async move {
                // Return connection to pool
                let now = Instant::now();
                let mut pool_guard = pool.lock().await;

                pool_guard.push_back(PooledConnection {
                    connection,
                    created_at: acquired_at, // Keep original creation time
                    last_used: now,
                });

                // Update statistics
                if let Ok(mut pool_stats) = stats.try_write() {
                    pool_stats.total_returned += 1;
                    pool_stats.active_connections = pool_stats.active_connections.saturating_sub(1);
                }

                // Release semaphore permit
                semaphore.add_permits(1);

                debug!(
                    usage_time_ms = now.duration_since(acquired_at).as_millis(),
                    "Connection returned to pool"
                );
            });
        }
    }
}

// Specialized connection pools for common use cases

/// HTTP client connection pool
pub type HttpConnectionPool = ConnectionPool<reqwest::Client>;

/// Database connection pool (generic for different DB types)
pub type DatabaseConnectionPool<T> = ConnectionPool<T>;

/// Create an HTTP client connection pool with sensible defaults
pub fn create_http_pool(config: Option<PoolConfig>) -> HttpConnectionPool {
    let config = config.unwrap_or_default();

    ConnectionPool::new(
        || {
            let client = reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .map_err(|e| NestGateError::Network(format!("HTTP client creation failed: {e}")))?;
            Ok(client)
        },
        |client| {
            // Simple health check - verify client is still valid
            if client.get("http://127.0.0.1").build().is_ok() {
                Ok(())
            } else {
                Err(NestGateError::Network(
                    "HTTP client health check failed".to_string(),
                ))
            }
        },
        config,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockConnection {
        id: usize,
        healthy: std::sync::Arc<std::sync::atomic::AtomicBool>,
    }

    #[tokio::test]
    async fn test_connection_pool_basic_functionality() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        let pool = ConnectionPool::new(
            move || {
                let id = counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(MockConnection {
                    id,
                    healthy: Arc::new(std::sync::atomic::AtomicBool::new(true)),
                })
            },
            |conn| {
                if conn.healthy.load(Ordering::SeqCst) {
                    Ok(())
                } else {
                    Err(NestGateError::Internal("Unhealthy connection".to_string()))
                }
            },
            PoolConfig::default(),
        );

        // Test connection acquisition and return
        {
            let conn = pool.acquire().await.unwrap();
            assert_eq!(conn.connection().unwrap().id, 0);
        }

        // Connection should be returned to pool
        let stats = pool.stats().await;
        assert!(stats.total_acquired > 0);
        assert!(stats.total_returned > 0);
    }

    #[tokio::test]
    async fn test_connection_pool_concurrency() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        let pool = Arc::new(ConnectionPool::new(
            move || {
                let id = counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(MockConnection {
                    id,
                    healthy: Arc::new(std::sync::atomic::AtomicBool::new(true)),
                })
            },
            |conn| {
                if conn.healthy.load(Ordering::SeqCst) {
                    Ok(())
                } else {
                    Err(NestGateError::Internal("Unhealthy connection".to_string()))
                }
            },
            PoolConfig {
                max_connections: 5,
                ..Default::default()
            },
        ));

        // Test concurrent access
        let mut handles = vec![];
        for _ in 0..10 {
            let pool_clone = Arc::clone(&pool);
            handles.push(tokio::spawn(async move {
                let _conn = pool_clone.acquire().await.unwrap();
                tokio::time::sleep(Duration::from_millis(10)).await;
            }));
        }

        // Wait for all tasks
        for handle in handles {
            handle.await.unwrap();
        }

        let stats = pool.stats().await;
        assert!(stats.total_acquired >= 10);
    }
}
