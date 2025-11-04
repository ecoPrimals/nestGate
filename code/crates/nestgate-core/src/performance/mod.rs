// Performance Optimization Features
//! Module definitions and exports.
// Provides performance enhancements like connection pooling, caching,
//! and request batching for universal data adapters and storage backends.

pub mod connection_pool;
#[cfg(test)]
mod connection_pool_tests;

// **ADVANCED PERFORMANCE OPTIMIZATIONS** - Cutting-edge performance techniques
pub mod advanced_optimizations;
pub mod safe_optimizations;

// Re-export key performance features
pub use connection_pool::{
    ConnectionPoolConfig, ConnectionPoolManager, HttpConnectionPool, PoolStats,
    UniversalConnectionPool,
};

// Re-export advanced optimizations (selective to avoid ambiguous re-exports)
pub use advanced_optimizations::{
    BranchOptimized, CacheAlignedCounter, MemoryPool, PerformanceProfiler, SimdOperations,
};

/// DEPRECATED: Use SafeRingBuffer instead (100% safe, same performance)
#[deprecated(
    since = "0.10.0",
    note = "Use safe_optimizations::SafeRingBuffer instead - zero unsafe code, identical performance"
)]
pub use advanced_optimizations::LockFreeRingBuffer;

// Export SAFE implementations as defaults
pub use safe_optimizations::{
    PerformanceConstants, SafeBranchOptimized, SafeCacheAlignedCounter, SafeMemoryArena,
    SafeMemoryPool, SafePerformanceProfiler, SafeRingBuffer, SafeSimdOperations,
};

#[cfg(test)]
mod safe_optimizations_tests;
