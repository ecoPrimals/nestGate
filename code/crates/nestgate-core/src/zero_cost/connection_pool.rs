/// Zero-Cost Connection Pool
/// Phase 2: Replace Arc<dyn Fn> connection factory patterns with compile-time specialization.
use crate::Result;
use std::marker::PhantomData;

/// Zero-cost connection factory - replaces Arc<dyn Fn>
pub trait ZeroCostConnectionFactory<T, const POOL_SIZE: usize = 100>
where
    T: Clone + Send + Sync + 'static,
{
    /// Create connection - native function, no boxing
    fn create(&self) -> Result<T>;

    /// Pool capacity at compile-time
    fn pool_size() -> usize {
        POOL_SIZE
    }
    }

/// Zero-cost health check - replaces Arc<dyn Fn>
pub trait ZeroCostHealthCheck<T>
where
    T: Send + Sync + 'static,
{
    /// Check connection health - direct function call
    fn check(&self, connection: &T) -> Result<()>;
    }

/// Zero-cost connection pool with compile-time specialization
pub struct ZeroCostConnectionPool<Connection, Factory, HealthCheck, const POOL_SIZE: usize = 100>
where
    Connection: Clone + Send + Sync + 'static,
    Factory: ZeroCostConnectionFactory<Connection, POOL_SIZE>,
    HealthCheck: ZeroCostHealthCheck<Connection>,
{
    factory: Factory,
    health_check: HealthCheck,
    connections: std::sync::Arc<tokio::sync::RwLock<Vec<Connection>>>,
    _phantom: PhantomData<Connection>,
    }

impl<Connection, Factory, HealthCheck, const POOL_SIZE: usize>
    ZeroCostConnectionPool<Connection, Factory, HealthCheck, POOL_SIZE>
where
    Connection: Clone + Send + Sync + 'static,
    Factory: ZeroCostConnectionFactory<Connection, POOL_SIZE>,
    HealthCheck: ZeroCostHealthCheck<Connection>,
{
    /// Create new pool with compile-time specialization
    pub fn new(factory: Factory, health_check: HealthCheck) -> Self {
        Self {
            factory,
            health_check,
            connections: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
            _phantom: PhantomData,
    }
    }

    /// Get connection from pool - zero-cost dispatch
    pub async fn get_connection(&self) -> Result<Connection> {
        {
            let mut connections = self.connections.write().await;
            if let Some(conn) = connections.pop() {
                // Direct health check - no Arc<dyn> overhead
                if self.health_check.check(&conn).is_ok() {
                    return Ok(conn);
    }
    }
    }

        // Create new connection - compile-time factory
        self.factory.create()
    }

    /// Return connection to pool
    pub async fn return_connection(&self, connection: Connection) -> Result<()> {
        let mut connections = self.connections.write().await;

        if connections.len() < POOL_SIZE {
            // Direct health check before storing
            self.health_check.check(&connection)?;
            connections.push(connection);
        }
        Ok(())
    }

    /// Get pool statistics - compile-time size
    pub async fn get_stats(&self) -> PoolStats {
        let connections = self.connections.read().await;
        PoolStats {
            active_connections: connections.len(),
            max_connections: POOL_SIZE,
            utilization: connections.len() as f64 / POOL_SIZE as f64,
    }
    }
    }

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub active_connections: usize,
    pub max_connections: usize,
    pub utilization: f64,
    }

/// Example connection type
#[derive(Debug, Clone)]
pub struct DatabaseConnection {
    pub id: String,
    pub connected: bool,
    }

/// Production database connection factory
pub struct ProductionDbFactory;

impl ZeroCostConnectionFactory<DatabaseConnection, 1000> for ProductionDbFactory {
    fn create(&self) -> Result<DatabaseConnection> {
        Ok(DatabaseConnection {
            id: format!("prod_conn_{}", std::process::id()),
            connected: true,
        })
    }
    }

/// Development database connection factory
pub struct DevelopmentDbFactory;

impl ZeroCostConnectionFactory<DatabaseConnection, 100> for DevelopmentDbFactory {
    fn create(&self) -> Result<DatabaseConnection> {
        Ok(DatabaseConnection {
            id: format!("dev_conn_{}", std::process::id()),
            connected: true,
        })
    }
    }

/// Production health check
pub struct ProductionHealthCheck;

impl ZeroCostHealthCheck<DatabaseConnection> for ProductionHealthCheck {
    fn check(&self, connection: &DatabaseConnection) -> Result<()> {
        if connection.connected {
            Ok(())
        } else {
            Err(crate::NestGateError::Internal {
                message: "Connection health check failed".to_string(),
                component: "zero_cost_connection_pool".to_string(),
                location: Some("ConnectionPool".to_string()),
                context: None,
                is_bug: false,
            })
        }
    }
    }

/// Development health check - always passes
pub struct DevelopmentHealthCheck;

impl ZeroCostHealthCheck<DatabaseConnection> for DevelopmentHealthCheck {
    fn check(&self, _connection: &DatabaseConnection) -> Result<()> {
        Ok(()) // Development always passes
    }
    }

/// Production connection pool type
pub type ProductionConnectionPool = ZeroCostConnectionPool<
    DatabaseConnection,
    ProductionDbFactory,
    ProductionHealthCheck,
    1000, // Pool size
>;

/// Development connection pool type
pub type DevelopmentConnectionPool = ZeroCostConnectionPool<
    DatabaseConnection,
    DevelopmentDbFactory,
    DevelopmentHealthCheck,
    100, // Pool size
>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_zero_cost_connection_pool() {
        // Create production pool with compile-time specialization
        let pool = ProductionConnectionPool::new(ProductionDbFactory, ProductionHealthCheck);

        // Test zero-cost connection creation
        let conn1 = pool.get_connection().await;
        assert!(conn1.is_ok());
        assert!(conn1.as_ref().unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
}).id.contains("prod_conn"));

        // Test connection return
        let return_result = crate::safe_operations::safe_connection_pool_return(
            pool.return_connection(conn1),
            "connection_pool",
        )
        .await?;
        assert!(return_result.is_ok());

        // Test pool statistics
        let stats = pool.get_stats().await;
        assert_eq!(stats.max_connections, 1000); // Compile-time size
        assert!(stats.utilization >= 0.0);

        println!("✅ Zero-cost connection pool validation successful!");
    }

    #[tokio::test]
    async fn test_development_pool_specialization() {
        // Test development specialization
        let dev_pool = DevelopmentConnectionPool::new(DevelopmentDbFactory, DevelopmentHealthCheck);

        let conn = dev_pool.get_connection().await;
        assert!(conn.is_ok());
        assert!(conn.as_ref().unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
}).id.contains("dev_conn"));

        let stats = dev_pool.get_stats().await;
        assert_eq!(stats.max_connections, 100); // Development size

        println!("✅ Development pool specialization working!");
    }
    }
