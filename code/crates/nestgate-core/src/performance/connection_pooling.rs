use crate::error::NestGateError;
//
// Intelligent connection pooling with adaptive scaling for high-throughput scenarios.

use crate::error::CanonicalResult as Result;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore};

// Type aliases for connection pooling
type PooledConnectionQueue<T> = Arc<RwLock<VecDeque<PooledConnection<T>>>>;

/// Advanced connection pool with intelligent scaling
pub struct IntelligentConnectionPool<T> {
    connections: PooledConnectionQueue<T>,
    #[allow(dead_code)] // Configuration stored for future use
    config: ConnectionPoolConfig,
    metrics: Arc<PoolMetrics>,
    semaphore: Arc<Semaphore>,
}
/// Connection pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
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
    pub min_connections: usize,
    pub max_connections: usize,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
    pub health_check_interval: Duration,
    pub scale_up_threshold: f64,
    pub scale_down_threshold: f64,
}
impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 5,
            max_connections: 100,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
            health_check_interval: Duration::from_secs(60),
            scale_up_threshold: 0.8,
            scale_down_threshold: 0.3,
        }
    }
}

/// Pool metrics for monitoring and optimization
#[derive(Debug, Default)]
pub struct PoolMetrics {
    pub total_connections: AtomicUsize,
    pub active_connections: AtomicUsize,
    pub idle_connections: AtomicUsize,
    pub connection_requests: AtomicU64,
    pub connection_failures: AtomicU64,
    pub average_wait_time: AtomicU64,
}
/// Pooled connection wrapper
pub struct PooledConnection<T> {
    pub connection: T,
    pub created_at: Instant,
    pub last_used: Instant,
    pub use_count: u64,
    pub is_healthy: bool,
}
impl<T> PooledConnection<T> {
    pub fn new(connection: T) -> Self {
        let now = Instant::now();
        Self {
            connection,
            created_at: now,
            last_used: now,
            use_count: 0,
            is_healthy: true,
        }
    }

    pub fn mark_used(&mut self) {
        self.last_used = Instant::now();
        self.use_count += 1;
    }

    pub fn is_idle(&self, idle_timeout: Duration) -> bool {
        self.last_used.elapsed() > idle_timeout
    }
}

impl<T: Send + Sync + 'static> IntelligentConnectionPool<T> {
    pub fn new(config: ConnectionPoolConfig) -> Self {
        Self {
            connections: Arc::new(RwLock::new(VecDeque::new())),
            semaphore: Arc::new(Semaphore::new(config.max_connections)),
            metrics: Arc::new(PoolMetrics::default()),
            config,
        }
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn get_connection(&self) -> Result<PooledConnection<T>>  {
        // Acquire semaphore permit
        let _permit = self.semaphore.acquire().await?;

        // Try to get an existing connection
        let mut connections = self.connections.write().await;

        if let Some(mut conn) = connections.pop_front() {
            conn.mark_used();
            self.metrics
                .active_connections
                .fetch_add(1, Ordering::Relaxed);
            Ok(conn)
        } else {
            // Would create new connection in real implementation
            Err(crate::error::NestGateError::network_error(
                "Connection pool exhausted",
                "connection_pool",
                None,
            ))
        }
    }

    pub async fn return_connection(&self, connection: PooledConnection<T>) {
        let mut connections = self.connections.write().await;
        connections.push_back(connection);
        self.metrics
            .active_connections
            .fetch_sub(1, Ordering::Relaxed);
    }

    pub fn get_metrics(&self) -> PoolMetrics {
        PoolMetrics {
            total_connections: AtomicUsize::new(
                self.metrics.total_connections.load(Ordering::Relaxed),
            ),
            active_connections: AtomicUsize::new(
                self.metrics.active_connections.load(Ordering::Relaxed),
            ),
            idle_connections: AtomicUsize::new(
                self.metrics.idle_connections.load(Ordering::Relaxed),
            ),
            connection_requests: AtomicU64::new(
                self.metrics.connection_requests.load(Ordering::Relaxed),
            ),
            connection_failures: AtomicU64::new(
                self.metrics.connection_failures.load(Ordering::Relaxed),
            ),
            average_wait_time: AtomicU64::new(
                self.metrics.average_wait_time.load(Ordering::Relaxed),
            ),
        }
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

