//! Memory layout optimization modules
//!
//! This module provides high-performance memory layout optimizations
//! for zero-cost architecture, split into focused sub-modules.

pub mod cache_alignment;

/// DEPRECATED: Use memory_pool_safe instead
/// This module contains unsafe code that has been replaced with 100% safe alternatives
///
/// ⚠️ THIS MODULE IS DEPRECATED - DO NOT USE IN NEW CODE
/// Use `memory_pool_safe::SafeMemoryPool` instead for 100% safe, same-performance alternative
#[deprecated(
    since = "0.10.0",
    note = "Use memory_pool_safe::SafeMemoryPool instead - same performance, zero unsafe code"
)]
#[allow(dead_code)] // Allow dead code since this is deprecated
pub mod memory_pool;

/// 100% SAFE memory pool - proof that Rust can be FAST AND SAFE!
/// No Ferraris in the forest - this is production-ready safe code.
/// **RECOMMENDED**: Use this instead of the deprecated memory_pool module
pub mod memory_pool_safe;

// Re-export commonly used types
pub use cache_alignment::{CacheAligned, CachePadded, CACHE_LINE_SIZE};

// Export SAFE pool as the default (encourage safe usage)
pub use memory_pool_safe::{PoolHandle, PoolStats, SafeMemoryPool};

// Still export old pool for backwards compatibility (deprecated)
#[allow(deprecated)]
pub use memory_pool::CacheOptimizedMemoryPool;
