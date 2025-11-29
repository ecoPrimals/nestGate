//! Memory Optimization Module - Use Rust Standard Library
//!
//! **ARCHITECTURE NOTE**: NestGate uses Rust's standard memory management.
//! We are a **storage primal**, not a memory management framework.
//!
//! # Deleted Stubs
//!
//! **Previously had these stub implementations** (DELETED):
//! - `allocators.rs` - Custom allocators (use stdlib)
//! - `leak_detection.rs` - Leak detection (use valgrind/heaptrack)
//! - `profiling.rs` - Memory profiling (use external tools)
//! - `stats.rs` - Memory statistics (use external tools)
//! - `structures.rs` - Memory structures (use stdlib)
//! - `zero_copy.rs` - Generic zero-copy (implement per-module as needed)
//!
//! # Modern Approach
//!
//! ## Use Rust's Standard Allocator
//!
//! Rust's default allocator is excellent. Don't reinvent the wheel.
//!
//! ```rust
//! // Just use standard collections
//! use std::collections::{HashMap, Vec};
//!
//! let mut cache: HashMap<String, Vec<u8>> = HashMap::new();
//! cache.insert("key".to_string(), vec![1, 2, 3]);
//! ```
//!
//! ## Memory Pools (Where Needed)
//!
//! For hot paths that need object pooling, use existing crates:
//! - `object-pool` crate - Simple object pooling
//! - `lockfree` crate - Lock-free data structures
//! - `crossbeam` crate - Concurrent collections
//!
//! ## Zero-Copy (Where Applicable)
//!
//! Implement zero-copy patterns in specific modules:
//! - Use `&[u8]` instead of `Vec<u8>` when possible
//! - Use `Cow<'_, [u8]>` for flexibility
//! - Use `bytes::Bytes` for reference-counted buffers
//! - Avoid cloning large data structures
//!
//! ```rust
//! use std::borrow::Cow;
//!
//! // Zero-copy when possible, owned when needed
//! pub fn process_data(data: Cow<'_, [u8]>) -> Result<Output> {
//!     // Works with both borrowed and owned data
//! }
//! ```
//!
//! ## Memory Profiling Tools
//!
//! Use external tools for memory analysis:
//! - **Valgrind** - Memory leak detection
//! - **Heaptrack** - Heap profiling
//! - **Massif** - Heap profiler
//! - **jemalloc** - Alternative allocator with profiling
//! - **cargo flamegraph** - CPU and memory profiling
//!
//! ## Benchmarking
//!
//! Use Criterion for memory benchmarks:
//! ```rust
//! use criterion::{black_box, criterion_group, criterion_main, Criterion};
//!
//! fn bench_allocation(c: &mut Criterion) {
//!     c.bench_function("allocate_vec", |b| {
//!         b.iter(|| {
//!             let v: Vec<u8> = black_box(vec![0; 1024]);
//!             v
//!         });
//!     });
//! }
//! ```

// Keep the memory pool if it's actually used (check usage first)
pub mod pools;

// Re-export commonly used types
pub use std::sync::Arc;
pub use std::rc::Rc;
pub use std::borrow::Cow;

/// Memory statistics (simple wrapper around stdlib)
#[derive(Debug, Clone, Copy)]
/// Memorystats
pub struct MemoryStats {
    /// Allocated Bytes
    pub allocated_bytes: usize,
    /// Deallocated Bytes
    pub deallocated_bytes: usize,
}

impl MemoryStats {
    /// Get current memory usage from the system
    ///
    /// Note: Use external tools like `sysinfo` crate for real stats
    pub fn current() -> Self {
        // This is a placeholder - use sysinfo crate for real implementation
        Self {
            allocated_bytes: 0,
            deallocated_bytes: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_standard_collections() {
        use std::collections::HashMap;
        
        let mut map: HashMap<String, Vec<u8>> = HashMap::new();
        map.insert("test".to_string(), vec![1, 2, 3]);
        
        assert_eq!(map.get("test"), Some(&vec![1, 2, 3]));
    }
    
    #[test]
    fn test_zero_copy_pattern() {
        use std::borrow::Cow;
        
        /// Processes data
        fn process(data: Cow<'_, [u8]>) -> usize {
            data.len()
        }
        
        // Borrowed
        let borrowed_data = &[1, 2, 3][..];
        assert_eq!(process(Cow::Borrowed(borrowed_data)), 3);
        
        // Owned
        let owned_data = vec![1, 2, 3, 4];
        assert_eq!(process(Cow::Owned(owned_data)), 4);
    }
}
