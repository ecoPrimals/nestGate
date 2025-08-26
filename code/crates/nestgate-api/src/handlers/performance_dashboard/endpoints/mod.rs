//
// This module provides modularized HTTP endpoint handlers for the performance dashboard,
// split from the original 917-line monolithic file for better maintainability.
//
// **MODULAR ORGANIZATION**:
// - `overview.rs` - Dashboard overview and summary endpoints
// - `metrics.rs` - Real-time metrics and monitoring endpoints
// - `pools.rs` - Pool-specific performance endpoints
// - `capacity.rs` - Capacity analysis and forecasting endpoints
// - `io_performance.rs` - I/O performance analysis endpoints
// - `cache.rs` - Cache performance analysis endpoints
// - `events.rs` - Server-sent events and real-time streaming

// ==================== MODULE DECLARATIONS ====================

/// Dashboard overview and summary endpoints
pub mod overview;

/// Real-time metrics and monitoring endpoints
pub mod metrics;

/// Pool-specific performance endpoints
pub mod pools;

/// Capacity analysis and forecasting endpoints
pub mod capacity;

/// I/O performance analysis endpoints
pub mod io_performance;

/// Cache performance analysis endpoints
pub mod cache;

/// Server-sent events and real-time streaming
pub mod events;

// ==================== RE-EXPORTS ====================

pub use overview::*;
pub use metrics::*;
pub use pools::*;
pub use capacity::*;
pub use io_performance::*;
pub use cache::*;
pub use events::*; 