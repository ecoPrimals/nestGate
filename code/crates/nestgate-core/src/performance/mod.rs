// Performance Optimization Features
//! Module definitions and exports.
// Provides performance enhancements like connection pooling, caching,
//! and request batching for universal data adapters and storage backends.

pub mod connection_pool;

// **ADVANCED PERFORMANCE OPTIMIZATIONS** - Cutting-edge performance techniques
pub mod advanced_optimizations;
// Re-export key performance features
pub use connection_pool::{
    ConnectionPoolConfig, ConnectionPoolManager, HttpConnectionPool, PoolStats,
    UniversalConnectionPool,
};

// Re-export advanced optimizations
pub use advanced_optimizations::*;
