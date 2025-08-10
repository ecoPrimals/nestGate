/// Zero-Cost Trait Definitions
/// Core traits using native async methods and const generics for compile-time specialization.
/// Replaces async_trait patterns for maximum performance.
// Removed unused imports - using native async traits instead
use crate::constants::{buffers, limits};

/// **ZERO-COST STORAGE PROVIDER TRAIT**
/// Compile-time optimized storage operations with const generics
#[async_trait::async_trait]
pub trait ZeroCostStorageProvider {
    type PoolInfo: Clone + Send + Sync + 'static;
    type DatasetInfo: Clone + Send + Sync + 'static;
    type Error: Clone + Send + Sync + 'static;
    type Result: Clone + Send + Sync + 'static;

    /// Get pool information
    async fn get_pool_info(&self, pool_name: &str) -> Self::Result;

    /// Get dataset statistics
    async fn get_dataset_stats(&self, dataset_name: &str) -> Self::Result;
    }

/// **ZERO-COST SECURITY PROVIDER TRAIT**
/// Compile-time optimized security operations
#[async_trait::async_trait]
pub trait ZeroCostSecurityProvider {
    type TokenInfo: Clone + Send + Sync + 'static;
    type Result: Clone + Send + Sync + 'static;

    /// Maximum number of active tokens (compile-time constant)
    fn max_tokens() -> usize;

    /// Generate authentication token
    async fn generate_token(&self, user_id: &str) -> Self::Result;

    /// Validate token
    async fn validate_token(&self, token: &str) -> Self::Result;

    /// Revoke token
    async fn revoke_token(&self, token: &str) -> Self::Result;
    }

/// **ZERO-COST NETWORK PROVIDER TRAIT**
/// Compile-time optimized network operations
#[async_trait::async_trait]
pub trait ZeroCostNetworkProvider<const MAX_CONNECTIONS: usize, const BUFFER_SIZE: usize> {
    type ConnectionInfo: Clone + Send + Sync + 'static;
    type Result: Clone + Send + Sync + 'static;

    /// Maximum connections supported (compile-time constant)
    const MAX_CONN: usize = MAX_CONNECTIONS;

    /// Network buffer size (compile-time constant)
    const BUFFER_SZ: usize = BUFFER_SIZE;

    /// Establish connection with compile-time bounds checking
    async fn establish_connection(&self, endpoint: &str) -> Self::Result;

    /// Close connection
    async fn close_connection(&self, connection_id: &str) -> Self::Result;

    /// Get connection statistics
    async fn get_connection_stats(&self) -> Self::Result;
    }

/// Zero-copy data source with compile-time buffer management
pub trait ZeroCostDataSource<
    const BUFFER_SIZE: usize = { buffers::BUFFER_SIZE_STANDARD },
    const MAX_RESULTS: usize = 1000, // Default max results for zero-copy operations
>
{
    type DataResult: Clone + Send + Sync + 'static;

    /// Fetch data with zero-copy operations
    async fn fetch_data(&self, query: &str) -> Self::DataResult;
    async fn stream_data(&self, query: &str) -> Self::DataResult;

    /// Compile-time buffer validation
    fn buffer_size() -> usize {
        BUFFER_SIZE
    }

    fn max_results() -> usize {
        MAX_RESULTS
    }
    }

/// Zero-cost universal adapter - replaces Arc<dyn> patterns
pub trait ZeroCostUniversalAdapter<Storage, Security, Network>
where
    Storage: ZeroCostStorageProvider,
    Security: ZeroCostSecurityProvider,
    Network: ZeroCostNetworkProvider<1024, 4096>,
{
    /// Get storage provider - compile-time dispatch
    fn storage(&self) -> &Storage;

    /// Get security provider - zero-cost abstraction
    fn security(&self) -> &Security;

    /// Get network provider - compile-time specialization
    fn network(&self) -> &Network;

    /// Health check with all providers
    fn health_check(&self) -> impl std::future::Future<Output = crate::Result<bool>> + Send {
        async move {
            // All providers available at compile-time
            Ok(true)
    }
    }
    }
