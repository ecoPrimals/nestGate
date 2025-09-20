//! Memory layout optimizations for zero-cost architecture
//!
//! This module has been refactored into smaller, focused sub-modules.
//! All functionality is re-exported for backward compatibility.

// Re-export all functionality from the new modular structure
pub use crate::memory_layout::*;

// Legacy compatibility - ensure all original exports are available
pub use crate::memory_layout::{
    cache_alignment::{CacheAligned, CachePadded, CACHE_LINE_SIZE},
    memory_pool::{CacheOptimizedMemoryPool, PoolHandle, PoolStats},
}; 