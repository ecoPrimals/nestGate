// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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

/// **NEW SAFE RING BUFFER** - 100% safe, high-performance SPSC buffer
/// Use this instead of unsafe ring buffers - same performance, zero unsafe code!
pub mod safe_ring_buffer;

// Re-export key performance features
#[cfg(feature = "dev-stubs")]
pub use connection_pool::HttpConnectionPool;
pub use connection_pool::{
    ConnectionPoolConfig, ConnectionPoolManager, PoolStats, UniversalConnectionPool,
};

// Re-export advanced optimizations (selective to avoid ambiguous re-exports)
pub use advanced_optimizations::{
    BranchOptimized, CacheAlignedCounter, MemoryPool, PerformanceProfiler, SimdOperations,
};

// Export SAFE implementations as defaults
pub use safe_optimizations::{
    PerformanceConstants, SafeBranchOptimized, SafeCacheAlignedCounter, SafeMemoryArena,
    SafeMemoryPool, SafePerformanceProfiler, SafeRingBuffer, SafeSimdOperations,
};

#[cfg(test)]
mod safe_optimizations_tests;
