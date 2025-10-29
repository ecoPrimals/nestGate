//! SIMD optimizations for zero-cost architecture
//!
//! This module has been refactored into smaller, focused sub-modules.
//! All functionality is re-exported for backward compatibility.
//!
//! ## Performance Claims
//!
//! SIMD optimizations provide:
//! - 4-16x improvement for vectorizable operations
//! - 2-8x improvement for bulk cryptographic operations
//! - 8-16x improvement for pattern matching and search
//! - Cache-optimized memory operations
//!
//! ## Migration
//!
//! The original large implementation has been split into:
//! - `simd::types` - Error types, capabilities, and statistics
//! - `simd::batch_processor` - High-performance batch processing

use std::arch::x86_64::*;

// Re-export all functionality from the new modular structure
pub use crate::simd::*;

// Legacy compatibility - ensure all original exports are available
pub use crate::simd::{
    batch_processor::SimdBatchProcessor,
    types::{SimdCapabilities, SimdError, SimdStats},
};

// Legacy type aliases for backward compatibility
pub type SimdBatchProcessor32 = SimdBatchProcessor<32>;
pub type SimdBatchProcessor64 = SimdBatchProcessor<64>;
pub type SimdBatchProcessor128 = SimdBatchProcessor<128>;

// Legacy constants
pub const SIMD_BATCH_SIZE: usize = DEFAULT_BATCH_SIZE;
pub const SIMD_MAX_BATCH_SIZE: usize = MAX_BATCH_SIZE; 