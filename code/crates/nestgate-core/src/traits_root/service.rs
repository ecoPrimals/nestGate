//
// Core service trait definitions for the NestGate ecosystem

// Removed unused Future import - using native async


/// Core service trait for all NestGate services - **ZERO-COST NATIVE ASYNC**
/// 
/// **CANONICAL**: This trait is unified and used across the entire codebase
pub trait Service: Send + Sync {
    /// Service name identifier
    fn name(&self) -> &str;
    /// Initialize the service
    fn initialize(&self) -> impl std::future::Future<Output = crate::Result<()>> + Send;

    /// Start the service
    fn start(&self) -> impl std::future::Future<Output = crate::Result<()>> + Send;

    /// Stop the service
    fn stop(&self) -> impl std::future::Future<Output = crate::Result<()>> + Send;

    /// Shutdown the service (alias for stop for backward compatibility)
    /// 
    /// **MIGRATION NOTE**: This is an alias for `stop()`. Many implementations
    /// use `shutdown` as their method name. This provides compatibility during
    /// the transition to the canonical trait system.
    fn shutdown(&self) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.stop()
    }

    /// Get service health status
    fn health_check(&self) -> impl std::future::Future<Output = crate::Result<bool>> + Send;
}
