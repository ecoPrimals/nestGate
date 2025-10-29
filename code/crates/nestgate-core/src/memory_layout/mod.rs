//! Memory layout optimization modules
//!
//! This module provides high-performance memory layout optimizations
//! for zero-cost architecture, split into focused sub-modules.

pub mod cache_alignment;
pub mod memory_pool;

// Re-export commonly used types
pub use cache_alignment::{CacheAligned, CachePadded, CACHE_LINE_SIZE};
pub use memory_pool::{CacheOptimizedMemoryPool, PoolHandle, PoolStats};
