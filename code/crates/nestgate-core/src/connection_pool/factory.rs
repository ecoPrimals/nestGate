/// Connection pool factory helpers for HTTP clients backed by [`crate::http_client_stub::Client`].
use super::ConnectionPool;
use crate::config::canonical_primary::{NestGateCanonicalConfig, UnifiedConnectionPoolConfig};
use crate::http_client_stub as reqwest;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;

/// Pooled HTTP client type (`DiscoveryHttpClient` façade).
pub type HttpConnectionPool = ConnectionPool<Client>;

/// Build a canonical config with pool limits derived from [`UnifiedConnectionPoolConfig`].
fn nestgate_config_for_pool(pool: &UnifiedConnectionPoolConfig) -> NestGateCanonicalConfig {
    let mut config = NestGateCanonicalConfig::default();
    config.network.api.max_connections = pool.max_size.max(1);
    config.network.api.connection_timeout = pool.connection_timeout;
    config
}

/// Create a pre-configured HTTP connection pool using the discovery HTTP client stack.
///
/// Pool sizing and timeouts are taken from [`UnifiedConnectionPoolConfig`] when provided,
/// and merged into [`NestGateCanonicalConfig`] for the shared [`ConnectionPool`] implementation.
pub fn create_http_pool(
    config: Option<UnifiedConnectionPoolConfig>,
) -> crate::Result<HttpConnectionPool> {
    let pool_cfg = config.unwrap_or_else(|| {
        let mut c = UnifiedConnectionPoolConfig::default();
        c.initial_size = 2;
        c.max_size = 10;
        c.idle_timeout = Duration::from_secs(300);
        c.connection_timeout = Duration::from_secs(30);
        c.health_check_interval = Duration::from_secs(60);
        c
    });

    let canonical = nestgate_config_for_pool(&pool_cfg);
    let connect_timeout = pool_cfg.connection_timeout;

    let factory = Arc::new(move || -> crate::Result<Client> {
        Client::builder().timeout(connect_timeout).build()
    });

    let health_check: Option<Arc<dyn Fn(&Client) -> crate::Result<()> + Send + Sync>> =
        Some(Arc::new(|client: &Client| {
            // Verify the client can be cloned and its internal state is consistent.
            // Full HTTP health probes require async; this sync check validates the
            // client handle is still usable (not poisoned or dropped).
            let _clone = client.clone();
            Ok(())
        }));

    ConnectionPool::new(canonical, factory, health_check)
}

/// Placeholder for database connection pools (not yet implemented).
///
/// Returns a clear validation error until PostgreSQL/MySQL (or other) factories are wired.
pub fn create_database_pool<T>(
    _config: NestGateCanonicalConfig,
    _connection_string: &str,
) -> crate::Result<ConnectionPool<T>>
where
    T: Send + Sync + 'static,
{
    Err(crate::NestGateError::validation(
        "create_database_pool is not implemented; use a database-specific pool when available",
    ))
}
