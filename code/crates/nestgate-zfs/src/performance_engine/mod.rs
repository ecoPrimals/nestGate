// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// This module provides comprehensive ZFS performance monitoring and optimization
// capabilities, organized into logical sub-modules for maintainability.

//! Performance Engine module

pub mod engine;
pub mod monitoring;
/// Performance engine type definitions
pub mod types;

#[cfg(test)]
mod engine_tests;
#[cfg(test)]
mod tests_monitoring;
#[cfg(test)]
mod tests_types;

// Re-export commonly used types
pub use engine::PerformanceOptimizationEngine;
pub use monitoring::RealTimePerformanceMonitor;
pub use types::*;
