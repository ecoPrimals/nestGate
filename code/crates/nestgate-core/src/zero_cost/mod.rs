/// **Zero-Cost Architecture Implementation**
/// This module implements universal zero-cost architecture patterns for NestGate,
/// providing high-performance abstractions without runtime overhead.
/// **Key Features**:
/// - Zero-cost abstractions with compile-time optimization
/// - Memory-efficient data structures using const generics
/// - Thread-safe operations without synchronization overhead
/// - Type-safe interfaces with minimal allocation
/// // Compute functionality moved to compute capabilities via universal adapter
pub mod composition;
pub mod network;
pub mod security;
pub mod storage;
pub mod traits;

// Phase 2: Additional zero-cost patterns
pub mod connection_pool;
pub mod memory_pool;
pub mod optimized_traits;
pub mod zfs_operations;

// Phase 3: Native async trait patterns (replacing #[async_trait])
// REMOVED: async_trait_migration - migration utilities no longer needed
pub mod native_async_traits;

pub use composition::*;
pub use optimized_traits::*;
pub use traits::*;
