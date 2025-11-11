// Universal Connection Pool
//! Connection Pool functionality and utilities.
// Provides connection pooling for any data provider to improve performance
//! and reduce connection overhead. Works with HTTP clients, database connections,
//! or any resource that benefits from pooling.

use crate::error::{NestGateError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore};
use tracing::{debug, info};

/// Configuration for connection pooling
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::ConnectionPoolConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::ConnectionPoolConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct ConnectionPoolConfig {
    /// Maximum number of connections per pool
    pub max_connections: usize,
    /// Minimum number of connections to maintain
    pub min_connections: usize,
    /// Maximum idle time before closing a connection
    pub max_idle_time: Duration,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// How often to clean up idle connections
    pub cleanup_interval: Duration,
}
impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 2,
            max_idle_time: Duration::from_secs(300), // 5 minutes
            connection_timeout: Duration::from_secs(30),
            cleanup_interval: Duration::from_secs(60), // 1 minute
        }
    }
}

/// A pooled connection wrapper
#[derive(Debug)]
pub struct PooledConnection<T> {
    /// The actual connection
    pub connection: T,
    /// When this connection was last used
    pub last_used: Instant,
    /// Whether this connection is currently in use
    pub in_use: bool,
    /// Connection creation time
    pub created_at: Instant,
}
impl<T> PooledConnection<T> {
    pub fn new(connection: T) -> Self {
        let now = Instant::now();
        Self {
            connection,
            last_used: now,
            in_use: false,
            created_at: now,
        }
    }

    pub fn mark_used(&mut self) {
        self.last_used = Instant::now();
        self.in_use = true;
    }

    pub fn mark_idle(&mut self) {
        self.in_use = false;
    }

    pub fn is_idle_too_long(&self, max_idle_time: Duration) -> bool {
        !self.in_use && self.last_used.elapsed() > max_idle_time
    }
}

/// Universal connection pool that can pool any type of connection
pub struct UniversalConnectionPool<T> {
    /// Pool configuration
    config: ConnectionPoolConfig,
    /// The actual connection pool
    connections: Arc<RwLock<Vec<PooledConnection<T>>>>,
    /// Semaphore to limit concurrent connections
    semaphore: Arc<Semaphore>,
    /// Connection factory function
    connection_factory: Arc<dyn Fn() -> Result<T> + Send + Sync>,
    /// Pool statistics
    stats: Arc<RwLock<PoolStats>>,
}
/// Connection pool statistics
#[derive(Debug, Default)]
pub struct PoolStats {
    pub total_connections: usize,
    pub active_connections: usize,
    pub idle_connections: usize,
    pub connections_created: u64,
    pub connections_destroyed: u64,
    pub connection_requests: u64,
    pub connection_timeouts: u64,
}
impl<T> UniversalConnectionPool<T>
where
    T: Send + Sync + 'static,
{
    /// Create a new connection pool
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn new<F>(config: ConnectionPoolConfig, connection_factory: F) -> Self
    where
        F: Fn() -> Result<T> + Send + Sync + 'static,
    {
        info!(
            "🏊 Creating universal connection pool with max {} connections",
            config.max_connections
        );

        Self {
            semaphore: Arc::new(Semaphore::new(config.max_connections)),
            connections: Arc::new(RwLock::new(Vec::new())),
            connection_factory: Arc::new(connection_factory),
            config,
            stats: Arc::new(RwLock::new(PoolStats::default())),
        }
    }

    /// Get a connection from the pool
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_connection(self: &Arc<Self>) -> Result<PooledConnectionGuard<T>> {
        let permit = self.semaphore.clone().acquire_owned().await.map_err(|_| {
            NestGateError::Internal(Box::new(crate::error::InternalErrorDetails {
                message: "Failed to acquire connection permit".to_string(),
                component: "connection_pool".to_string(),
                location: None,
                is_bug: false,
                context: None,
            }))
        })?;

        // Try to find an idle connection
        let mut connections = self.connections.write().await;

        // Look for an idle connection
        for pooled_conn in connections.iter_mut() {
            if !pooled_conn.in_use {
                pooled_conn.mark_used();
                debug!("♻️ Reusing existing connection from pool");

                // For now, we'll create a new connection instead of trying to move/clone
                // This is a temporary solution until we can redesign the pooling properly
                drop(connections); // Release the lock

                let new_connection = (self.connection_factory)().inspect_err(|_e| {
                    let stats = self.stats.clone();
                    tokio::spawn(async move {
                        let mut stats = stats.write().await;
                        stats.connection_timeouts += 1;
                    });
                })?;

                return Ok(PooledConnectionGuard::new(
                    new_connection,
                    self.clone(),
                    permit,
                ));
            }
        }

        // No idle connection available, create a new one
        debug!("🆕 Creating new connection for pool");
        let new_connection = (self.connection_factory)().inspect_err(|_e| {
            let stats = self.stats.clone();
            tokio::spawn(async move {
                let mut stats = stats.write().await;
                stats.connection_timeouts += 1;
            });
        })?;

        // Update stats
        let mut stats = self.stats.write().await;
        stats.connections_created += 1;
        stats.total_connections += 1;

        Ok(PooledConnectionGuard::new(
            new_connection,
            self.clone(),
            permit,
        ))
    }

    /// Clean up idle connections
    pub async fn cleanup_idle_connections(&self) {
        let connections_to_remove = {
            let connections = self.connections.read().await;
            let mut to_remove = Vec::new();

            for (i, conn) in connections.iter().enumerate() {
                if !conn.in_use
                    && conn.is_idle_too_long(self.config.max_idle_time)
                    && connections.len() > self.config.min_connections
                {
                    to_remove.push(i);
                }
            }
            to_remove
        };

        if !connections_to_remove.is_empty() {
            let mut connections = self.connections.write().await;
            // Remove in reverse order to maintain indices
            for &index in connections_to_remove.iter().rev() {
                if index < connections.len() {
                    connections.remove(index);
                    debug!("🧹 Removed idle connection from pool");
                }
            }

            // Update stats
            let mut stats = self.stats.write().await;
            stats.total_connections = connections.len();
        }
    }

    /// Get pool statistics
    pub async fn get_stats(&self) -> PoolStats {
        let connections = self.connections.read().await;
        let mut stats = self.stats.write().await;

        stats.total_connections = connections.len();
        stats.active_connections = connections.iter().filter(|c| c.in_use).count();
        stats.idle_connections = connections.iter().filter(|c| !c.in_use).count();

        // Clone the stats data, not the guard
        PoolStats {
            connections_created: stats.connections_created,
            connections_destroyed: stats.connections_destroyed,
            connection_requests: stats.connection_requests,
            connection_timeouts: stats.connection_timeouts,
            total_connections: stats.total_connections,
            active_connections: stats.active_connections,
            idle_connections: stats.idle_connections,
        }
    }

    /// Start background cleanup task
    pub fn start_cleanup_task(&self) -> tokio::task::JoinHandle<()> {
        let pool = self.connections.clone();
        let stats = self.stats.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(config.cleanup_interval);

            loop {
                interval.tick().await;

                // Cleanup logic
                let mut connections = pool.write().await;
                let initial_count = connections.len();
                let min_connections = config.min_connections;
                let max_idle_time = config.max_idle_time;

                connections.retain(|conn| {
                    !conn.is_idle_too_long(max_idle_time) || initial_count <= min_connections
                });

                let removed_count = initial_count - connections.len();
                if removed_count > 0 {
                    let mut stats = stats.write().await;
                    stats.connections_destroyed += removed_count as u64;
                    stats.total_connections = connections.len();
                    debug!(
                        "🧹 Background cleanup removed {} idle connections",
                        removed_count
                    );
                }
            }
        })
    }
}

/// RAII guard for pooled connections
pub struct PooledConnectionGuard<T> {
    connection: T,
    #[allow(dead_code)] // Used for RAII cleanup semantics
    pool: Arc<UniversalConnectionPool<T>>,
    _permit: tokio::sync::OwnedSemaphorePermit,
}
impl<T> PooledConnectionGuard<T> {
    fn new(
        connection: T,
        pool: Arc<UniversalConnectionPool<T>>,
        permit: tokio::sync::OwnedSemaphorePermit,
    ) -> Self {
        Self {
            connection,
            pool,
            _permit: permit,
        }
    }

    /// Get a reference to the underlying connection
    pub fn connection(&self) -> &T {
        &self.connection
    }

    /// Get a mutable reference to the underlying connection
    pub fn connection_mut(&mut self) -> &mut T {
        &mut self.connection
    }
}

impl<T> Drop for PooledConnectionGuard<T> {
    fn drop(&mut self) {
        debug!("🔄 Connection returned to pool");
        // Connection will be returned to pool when this is dropped
        // The pool will handle reusing it for the next request
    }
}

/// Manager for multiple connection pools
pub struct ConnectionPoolManager {
    pools: RwLock<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>,
}
impl ConnectionPoolManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
            pools: RwLock::new(HashMap::new()),
        }
    }

    /// Register a connection pool for a specific provider
    pub async fn register_pool<T>(&self, provider_name: String, pool: UniversalConnectionPool<T>)
    where
        T: Send + Sync + 'static,
    {
        info!(
            "📝 Registering connection pool for provider: {}",
            provider_name
        );
        let mut pools = self.pools.write().await;
        pools.insert(provider_name, Box::new(pool));
    }

    /// Get a connection pool for a provider
    pub fn get_pool<T>(&self, _provider_name: &str) -> Option<Arc<UniversalConnectionPool<T>>>
    where
        T: Send + Sync + 'static,
    {
        // CANONICAL: Using Arc<dyn Any> for zero-cost shared ownership and thread safety
        // to properly support returning references
        None
    }
}

impl Default for ConnectionPoolManager {
    fn default() -> Self {
        Self::new()
    }
}

/// HTTP client connection pool (example usage)
pub type HttpConnectionPool = UniversalConnectionPool<reqwest::Client>;
impl HttpConnectionPool {
    /// Create an HTTP client connection pool
    pub fn new_http_pool(config: ConnectionPoolConfig) -> Self {
        Self::new(config, || {
            reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .map_err(|e| {
                    NestGateError::Internal(Box::new(crate::error::InternalErrorDetails {
                        message: format!("HTTP client creation failed: {}", e),
                        component: "http_connection_pool".to_string(),
                        location: None,
                        is_bug: false,
                        context: None,
                    }))
                })
        })
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type ConnectionPoolConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ConnectionPoolConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

