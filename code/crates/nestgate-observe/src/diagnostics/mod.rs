// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// Diagnostics module for NestGate
// This module provides system diagnostics and monitoring functionality
//! for the `NestGate` system. It has been refactored into focused sub-modules:
//! - `types`: Core diagnostic types and enums
//! - `metrics`: System metrics collection and structures
//! - `diagnostic`: Individual diagnostic entries
//! - `manager`: Main diagnostics management logic

/// Individual diagnostic entry types and structures for system health reporting.
pub mod diagnostic;
/// Diagnostics manager coordinating system-wide diagnostic collection and reporting.
pub mod manager;
/// System metrics types for performance and resource monitoring.
pub mod metrics;
/// Core diagnostic types including severity levels and diagnostic categories.
pub mod types;

#[cfg(test)]
mod coverage_tests;

pub use diagnostic::*;
pub use manager::*;
#[allow(ambiguous_glob_reexports)]
pub use metrics::*;
#[allow(ambiguous_glob_reexports)]
pub use types::*;
