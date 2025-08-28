/// Connection Guard for Safe Connection Management
/// Provides RAII (Resource Acquisition Is Initialization) pattern for connection management,
/// ensuring connections are properly returned to the pool when no longer needed.
use crate::Result;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

// Import shared types from pool module
use super::pool::PooledConnection;

/// RAII guard for safe connection access
/// This guard ensures that connections are automatically returned to the pool
/// when the guard is dropped, preventing connection leaks.
pub struct ConnectionGuard<T: Send + 'static> {
    connection: Option<T>,
    pool: Option<Arc<Mutex<VecDeque<PooledConnection<T>>>>>,
}

impl<T: Send + 'static> ConnectionGuard<T> {
    /// Create a new connection guard
    pub(in crate::connection_pool) fn new(
        connection: T,
        pool: Arc<Mutex<VecDeque<PooledConnection<T>>>>,
    ) -> Self {
        Self {
            connection: Some(connection),
            pool: Some(pool),
        }
    }

    /// Get an immutable reference to the connection
    pub fn connection(&self) -> Result<&T> {
        self.connection
            .as_ref()
            .ok_or_else(|| crate::NestGateError::Internal {
                message: "Connection has been consumed".to_string(),
                location: Some(file!().to_string()),
                context: None,
                is_bug: false,
            })
    }

    /// Get a mutable reference to the connection
    pub fn connection_mut(&mut self) -> Result<&mut T> {
        self.connection
            .as_mut()
            .ok_or_else(|| crate::NestGateError::Internal {
                message: "Connection has been consumed".to_string(),
                location: Some(file!().to_string()),
                context: None,
                is_bug: false,
            })
    }

    /// Take ownership of the connection, consuming the guard
    ///
    /// This removes the connection from the pool permanently.
    /// Use with caution as it can lead to connection leaks if not managed properly.
    pub fn take(mut self) -> Result<T> {
        self.connection
            .take()
            .ok_or_else(|| crate::NestGateError::Internal {
                message: "Connection has already been taken".to_string(),
                location: Some(file!().to_string()),
                context: None,
                is_bug: false,
            })
    }

    /// Release the connection back to the pool manually
    ///
    /// This is normally handled automatically by the Drop implementation,
    /// but can be called explicitly for early release.
    pub async fn release(mut self) -> Result<()> {
        if let (Some(connection), Some(pool)) = (self.connection.take(), self.pool.take()) {
            let mut pool_guard = pool.lock().await;
            let now = Instant::now();
            pool_guard.push_back(PooledConnection {
                connection,
                created_at: now,
                last_used: now,
            });
        }
        Ok(())
    }
}

impl<T: Send + 'static> Drop for ConnectionGuard<T> {
    fn drop(&mut self) {
        if let (Some(connection), Some(pool)) = (self.connection.take(), self.pool.clone()) {
            // Return connection to pool asynchronously
            // Note: We can't await in Drop, so we spawn a task
            let pool_clone = pool.clone();
            tokio::spawn(async move {
                let mut pool_guard = pool_clone.lock().await;
                let now = Instant::now();
                pool_guard.push_back(PooledConnection {
                    connection,
                    created_at: now, // Use current time as creation time
                    last_used: now,
                });
            });
        }
    }
}
