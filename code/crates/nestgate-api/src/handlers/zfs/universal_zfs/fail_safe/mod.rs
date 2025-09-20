//
// This module contains the fail-safe ZFS service implementation split into logical submodules:
// - circuit_breaker: Circuit breaker implementation
// - retry_executor: Retry logic implementation
// - core: Main service structure and core methods
// - pool_operations: Pool fail-safe operations
// - dataset_operations: Dataset fail-safe operations
// - snapshot_operations: Snapshot fail-safe operations
// - optimization: Optimization and configuration operations

mod circuit_breaker;
pub mod core;
mod dataset_operations;
mod optimization;
mod pool_operations;
mod retry_executor;
mod snapshot_operations;

// Re-export the main service
pub use circuit_breaker::{CircuitBreaker, CircuitBreakerState};
pub use core::FailSafeZfsService;
pub use retry_executor::RetryExecutor;
