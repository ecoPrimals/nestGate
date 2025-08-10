/// High-Performance Connection Pool System
/// Provides optimized connection pooling for database and network operations
/// to handle high-concurrency scenarios efficiently.
/// ## Performance Impact
/// - **Connection Reuse**: Eliminates connection establishment overhead
/// - **Configurable Limits**: Prevents resource exhaustion under load
/// - **Health Monitoring**: Automatic cleanup of stale connections
/// - **Load Balancing**: Distributes requests across available connections
pub mod config;
pub mod factory;
pub mod guard;
pub mod pool;
pub mod stats;

// Re-export the PooledConnection type for internal use

pub use config::PoolConfig;
pub use factory::create_http_pool;
pub use guard::ConnectionGuard;
pub use pool::ConnectionPool;
pub use stats::PoolStats;

// Type aliases for connection factory and health check functions
use crate::Result;
use std::sync::Arc;

/// Type alias for connection factory function
pub type ConnectionFactory<T> = Arc<dyn Fn() -> Result<T> + Send + Sync>;

/// Type alias for health check function
pub type HealthCheckFn<T> = Arc<dyn Fn(&T) -> Result<()> + Send + Sync>;
