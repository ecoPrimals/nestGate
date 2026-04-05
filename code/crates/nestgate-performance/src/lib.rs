// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(
    test,
    allow(
        clippy::panic,
        clippy::too_many_lines,
        clippy::cognitive_complexity,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::float_cmp,
        clippy::ip_constant,
        clippy::redundant_clone,
        clippy::manual_range_contains,
        clippy::needless_collect,
        clippy::manual_string_new,
        clippy::type_complexity,
    )
)]
#![expect(
    missing_docs,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::unused_self,
    clippy::option_if_let_else,
    clippy::items_after_statements,
    dead_code
)]

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

pub use zero_copy_networking::{ZeroCopyBuffer, ZeroCopyNetworkInterface, ZeroCopyTxPayload};

// Re-export main types
pub use adaptive_optimization::{
    AdaptivePerformanceMonitor, AutoTuner, MetricsCollector, OptimizationEngine,
    SimpleLearningModel, TrendAnalyzer,
};
