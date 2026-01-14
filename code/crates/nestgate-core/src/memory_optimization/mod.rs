//! **MEMORY OPTIMIZATION**
//!
//! Advanced memory optimization patterns and utilities for reducing memory
//! usage, preventing leaks, and improving allocation efficiency.
//!
//! ## Architecture
//!
//! This module is organized by optimization concern:
//! - **`stats`**: Memory usage tracking and statistics
//! - **`pooling`**: Object pooling and weak caching
//! - **`pressure`**: Memory pressure detection
//! - **`arena`**: Arena allocation
//! - **`compaction`**: Memory compaction
//! - **`profiler`**: Memory profiling and reporting
//!
//! ## Usage
//!
//! ```rust
//! use nestgate_core::memory_optimization::{MemoryStats, ObjectPool};
//!
//! // Track memory usage
//! let stats = MemoryStats::new();
//! stats.record_allocation(1024);
//!
//! // Use object pooling
//! let pool: ObjectPool<Vec<u8>> = ObjectPool::new(10);
//! ```
//!
//! ## Refactoring Note
//!
//! This module was refactored from a 957-line monolith into focused modules
//! for better maintainability and testability.

// Re-export all optimization modules
pub mod arena;
pub mod compaction;
pub mod pooling;
pub mod pressure;
pub mod profiler;
pub mod stats;

// Re-export commonly used types
pub use arena::MemoryArena;
pub use compaction::MemoryCompactor;
pub use pooling::{ObjectPool, WeakCache};
pub use pressure::MemoryPressureDetector;
pub use profiler::{CategoryReport, MemoryProfiler, MemoryReport};
pub use stats::MemoryStats;
