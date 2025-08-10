//! Performance Optimization Features
//!
//! Provides performance enhancements like connection pooling, caching,
//! and request batching for universal data adapters and storage backends.

pub mod connection_pool;

// Re-export key performance features
pub use connection_pool::{
    ConnectionPoolConfig, ConnectionPoolManager, HttpConnectionPool, PoolStats,
    UniversalConnectionPool,
};
