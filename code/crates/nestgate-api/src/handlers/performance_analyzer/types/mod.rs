// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Type definitions for performance analysis (domain-split).

mod analysis;
mod config;
mod report;
mod snapshot_metrics;
mod state;
mod trends;

pub use analysis::{CpuAnalysis, DiskAnalysis, MemoryAnalysis, NetworkAnalysis, ZfsAnalysis};
pub use config::{PerformanceAnalysisConfig, PerformanceAnalysisConfigCanonical};
pub use report::{PerformanceAnalysisReport, PerformanceRecommendation};
pub use snapshot_metrics::{
    CpuMetrics, DiskMetrics, MemoryMetrics, NetworkMetrics, PerformanceSnapshot, ZfsMetrics,
};
pub use state::PerformanceAnalyzerState;
pub use trends::{ComponentAnalysis, PerformanceTrend, PerformanceTrends};

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

// Note: Keep using PerformanceAnalysisConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests;
