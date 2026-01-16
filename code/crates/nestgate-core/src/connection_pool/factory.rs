/// Connection Pool Factory Functions
/// Provides convenient factory functions for creating commonly used connection pools.
use crate::http_client_stub as reqwest;
use super::ConnectionPool;
use crate::config::canonical_primary::NestGateCanonicalConfig;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
/// Type alias for HTTP connection pool
pub type HttpConnectionPool = ConnectionPool<Client>;
/// Create a pre-configured HTTP connection pool
/// This factory function creates a connection pool optimized for HTTP clients,
/// with reasonable defaults for web service communication.
pub fn create_http_pool(
    config: Option<crate::config::canonical_primary::UnifiedConnectionPoolConfig>,
) -> crate::Result<HttpConnectionPool> {
    let unified_config = config.unwrap_or_else(|| {
        let mut config = crate::config::canonical_primary::UnifiedConnectionPoolConfig::default();
        config.min_connections = 2;
        config.max_connections = 10;
        config.max_idle_time_seconds = 300;
        config.acquire_timeout_seconds = 30;
        config.health_check_interval_seconds = 60;
        config.enable_validation = true;
        config.retry_attempts = 3;
        config.retry_delay_seconds = 2;
        config.enable_metrics = true;
        config.pool_name = "http-client-pool".to_string();
        config
    );
    let pool_config = unified_config;

    // Create HTTP client factory
    let factory = Arc::new(|| -> crate::Result<Client> {
        Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| crate::NestGateError::internal_error(
                location: Some(format!("{})
                location: Some(format!("Error: {e}"))})
    );

    // Create health check function for HTTP clients
    let health_check: Option<Arc<dyn Fn(&Client) -> crate::Result<()> + Send + Sync>> =
        Some(Arc::new(|_client: &Client| -> crate::Result<()> {
            // Simple health check - just return Ok for HTTP clients
            Ok(())
        }));

    // Create NestGateCanonicalConfig with connection pool configuration
            let mut unified_config = crate::config::canonical_primary::NestGateCanonicalConfig::default();
    unified_config.connection_pool = pool_config;

    HttpConnectionPool::new(unified_config, factory, health_check)
}

/// Create a database connection pool
/// This is a placeholder for database-specific connection pools.
/// In a real implementation, this would create pools for PostgreSQL, MySQL, etc.
pub fn create_database_pool<T>(
    _config: NestGateCanonicalConfig,
    connection_string: &str,
) -> crate::Result<ConnectionPool<T>>
where
    T: Send + Sync + 'static,
{
    // This is a placeholder implementation
    // Real implementation would parse connection_string and create appropriate connections
    Err(crate::NestGateError::validation(
}
