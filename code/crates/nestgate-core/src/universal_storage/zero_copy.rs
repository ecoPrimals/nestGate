//! Zero-Copy Memory Management for NestGate Storage
//!
//! This module implements zero-copy memory management for NestGate's canonical storage system,
//! providing significant performance improvements by eliminating unnecessary data copying.

// Re-export everything from the zero_copy module for backward compatibility
pub use self::zero_copy::*;

/// Zero-copy submodule containing the refactored implementation
pub mod zero_copy; 