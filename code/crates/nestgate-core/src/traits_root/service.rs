//
// Core service trait definitions for the NestGate ecosystem

// Removed unused Future import - using native async


/// Core service trait for all NestGate services - **ZERO-COST NATIVE ASYNC**
pub trait Service: Send + Sync {
    /// Service name identifier
    fn name(&self) -> &str;

    /// Initialize the service
    fn initialize(&self) -> impl std::future::Future<Output = crate::Result<()>> + Send;

    /// Start the service
    fn start(&self) -> impl std::future::Future<Output = crate::Result<()>> + Send;

    /// Stop the service
    fn stop(&self) -> impl std::future::Future<Output = crate::Result<()>> + Send;

    /// Get service health status
    fn health_check(&self) -> impl std::future::Future<Output = crate::Result<bool>> + Send;
}
