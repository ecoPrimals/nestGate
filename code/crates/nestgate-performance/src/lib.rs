// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Performance optimization and monitoring for `NestGate`
//!
//! Provides adaptive performance optimization, SIMD operations, **safe concurrent structures**,
//! and zero-copy networking capabilities.
//!
//! ## 🚀 Safe Concurrent Structures
//!
//! This crate provides **100% safe** concurrent data structures with zero unsafe code:
//! - `SafeConcurrentQueue<T>`: Lock-free multi-producer multi-consumer queue
//! - `SafeConcurrentHashMap<K, V>`: High-performance concurrent hash map
//!
//! These safe abstractions **replace** the old unsafe lock-free implementations,
//! providing **equal or better performance** with **complete memory safety**.

pub mod adaptive_optimization;
// ✅ ELIMINATED: custom_allocators (14 unsafe blocks) - Use nestgate_core::memory_pool instead
// ✅ ELIMINATED: lock_free_structures (20 unsafe blocks) - Use safe_concurrent instead
pub mod safe_concurrent; // ✅ 100% SAFE - Zero unsafe code
pub mod simd;
pub mod zero_copy_networking;

// Re-export main types
pub use adaptive_optimization::{
    AdaptivePerformanceMonitor, AutoTuner, MetricsCollector, OptimizationEngine,
    SimpleLearningModel, TrendAnalyzer,
};
