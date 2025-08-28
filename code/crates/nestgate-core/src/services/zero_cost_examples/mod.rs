//
// This module provides modularized examples of migrating services from async_trait 
// patterns to zero-cost implementations, split from the original 906-line file.
//
// **MODULAR ORGANIZATION**:
// - `config_manager.rs` - Configuration management service examples
// - `legacy_implementations.rs` - Legacy async_trait implementations
// - `zero_cost_implementations.rs` - Zero-cost native async implementations
// - `performance_comparison.rs` - Performance benchmarking utilities

// ==================== SECTION ====================

/// Configuration management service examples
pub mod config_manager;

/// Legacy async_trait implementations
pub mod legacy_implementations;

/// Zero-cost native async implementations
pub mod zero_cost_implementations;

/// Performance comparison utilities
pub mod performance_comparison;

// ==================== SECTION ====================

pub use config_manager::*;
pub use legacy_implementations::*;
pub use zero_cost_implementations::*;
pub use performance_comparison::*; 