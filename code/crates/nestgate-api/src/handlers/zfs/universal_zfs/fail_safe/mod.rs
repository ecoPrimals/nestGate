// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// This module contains the fail-safe ZFS service implementation split into logical submodules:
// - circuit_breaker: Circuit breaker implementation
// - retry_executor: Retry logic implementation
// - core: Main service structure and core methods
// - pool_operations: Pool fail-safe operations
// - dataset_operations: Dataset fail-safe operations
// - snapshot_operations: Snapshot fail-safe operations
// - optimization: Optimization and configuration operations

//! Fail Safe module

mod circuit_breaker;
/// Core fail-safe service implementation
pub mod core;
pub(crate) mod dataset_operations;
pub(crate) mod optimization;
pub(crate) mod pool_operations;
mod retry_executor;
pub(crate) mod snapshot_operations;

// Re-export the main service
pub use circuit_breaker::{CircuitBreaker, CircuitBreakerState};
pub use core::FailSafeZfsService;
pub use retry_executor::RetryExecutor;
