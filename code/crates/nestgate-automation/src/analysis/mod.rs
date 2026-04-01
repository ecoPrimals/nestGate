// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! Analysis module: file characteristics, access patterns, and dataset summaries.

mod batch_analysis;
mod dataset_analyzer;
mod file_analyzer;
mod pattern_analyzer;
mod types;

#[cfg(test)]
mod tests;

pub use batch_analysis::analyze_datasets_with_patterns;
pub use dataset_analyzer::DatasetAnalyzer;
pub use file_analyzer::FileAnalyzer;
pub use pattern_analyzer::PatternAnalyzer;
pub use types::{DatasetAnalysis, DatasetSummary, FileCharacteristics};
