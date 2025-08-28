/// Main Connection Pool Implementation
/// Core connection pool logic with health monitoring and connection lifecycle management.
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock, Semaphore};
use tracing::{debug, info, warn};

use super::{ConnectionFactory, ConnectionGuard, HealthCheckFn, PoolStats};
use crate::config::canonical_master::NestGateCanonicalConfig;
use crate::{NestGateError, Result};

/// Wrapper for pooled connections with metadata
pub(super) struct PooledConnection<T> {
    pub(super) connection: T,
    #[allow(dead_code)]
    pub(super) created_at: Instant,
    pub(super) last_used: Instant,
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
    config: NestGateCanonicalConfig,
    /// Semaphore to limit concurrent connections
    semaphore: Arc<Semaphore>,
    /// Pool statistics
    stats: Arc<RwLock<PoolStats>>,
}

impl<T> ConnectionPool<T>
where
    T: Send + 'static,
{
    /// Create a new connection pool
    pub fn new(
        config: NestGateCanonicalConfig,
        factory: ConnectionFactory<T>,
        health_check: Option<HealthCheckFn<T>>,
    ) -> Result<Self> {
        config.validate()?;

        let health_check = health_check.unwrap_or_else(|| {
            Arc::new(|_| Ok(())) // Default health check always passes
        });

        let semaphore = Arc::new(Semaphore::new(config.network.max_connections));

        let pool = Self {
            pool: Arc::new(Mutex::new(VecDeque::new())),
            factory,
            health_check,
            config: config.clone(),
            semaphore,
            stats: Arc::new(RwLock::new(PoolStats::default())),
        };

        // Initialize minimum connections
        tokio::spawn(Self::initialize_pool(
            Arc::clone(&pool.pool),
            Arc::clone(&pool.factory),
            Arc::clone(&pool.stats),
            config.min_connections,
        ));

        // Start health check task
        tokio::spawn(Self::health_check_loop(
            Arc::clone(&pool.pool),
            Arc::clone(&pool.health_check),
            Arc::clone(&pool.stats),
            config,
        ));

        Ok(pool)
    }

    /// Acquire a connection from the pool
    pub async fn acquire(&self) -> Result<ConnectionGuard<T>> {
        let start_time = Instant::now();

        // Wait for available slot with timeout
        let permit = tokio::time::timeout(
            self.config.network.timeouts.connection_timeout,
            self.semaphore.acquire(),
        )
        .await
        .map_err(|_| NestGateError::Internal {
            message: "Connection pool acquisition timeout".to_string(),
            location: Some(file!().to_string()),
            context: None,
            is_bug: false,
        })?;

        let _permit = permit.map_err(|_| NestGateError::Internal {
            message: "Connection pool semaphore error".to_string(),
            location: Some(file!().to_string()),
            context: None,
            is_bug: false,
        })?;

        // Try to get existing connection from pool
        let connection = {
            let mut pool = self.pool.lock().await;

            // Remove stale connections
            let now = Instant::now();
            while let Some(conn) = pool.front() {
                if now.duration_since(conn.last_used) > Duration::from_secs(300) {
                    // 5 minute idle timeout
                    pool.pop_front();
                    if let Ok(mut stats) = self.stats.try_write() {
                        stats.idle_connections = stats.idle_connections.saturating_sub(1);
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
            stats.active_connections += 1;
        }

        debug!(
            acquisition_time_ms = start_time.elapsed().as_millis(),
            "Connection acquired from pool"
        );

        Ok(ConnectionGuard::new(connection, Arc::clone(&self.pool)))
    }

    /// Get current pool statistics
    pub async fn stats(&self) -> PoolStats {
        self.stats.read().await.clone()
    }

    /// Initialize minimum connections in the pool
    async fn initialize_pool(
        pool: Arc<Mutex<VecDeque<PooledConnection<T>>>>,
        factory: ConnectionFactory<T>,
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
                    pool_stats.idle_connections += 1;
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
        _config: NestGateCanonicalConfig,
    ) {
        let mut interval = tokio::time::interval(Duration::from_secs(30)); // 30 second health check interval

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
                pool_stats.failed_acquisitions += failures;
                pool_stats.idle_connections = pool_guard.len();
            }

            if failures > 0 {
                warn!("Removed {} unhealthy connections from pool", failures);
            }
        }
    }
}
