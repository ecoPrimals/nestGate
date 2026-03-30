// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **ANALYSIS CONFIGURATION**
///
/// Configuration for automated file and system analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Analysis
pub struct AnalysisConfig {
    /// Enable analysis
    pub enabled: bool,

    /// Scan interval
    pub scan_interval: Duration,

    /// Maximum file size to analyze (bytes)
    pub max_file_size: u64,

    /// File extensions to include in analysis
    pub include_extensions: Vec<String>,

    /// File extensions to exclude from analysis
    pub exclude_extensions: Vec<String>,

    /// Enable deep content analysis
    pub deep_analysis_enabled: bool,

    /// Parallel analysis workers
    pub parallel_workers: usize,
}

impl Default for AnalysisConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development()
    }
}

impl AnalysisConfig {
    /// Creates a development-optimized configuration for automated analysis
    ///
    /// Returns an `AnalysisConfig` with relaxed scanning intervals and smaller file size limits
    /// suitable for local development environments.
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: true,
            scan_interval: Duration::from_secs(3600), // 1 hour
            max_file_size: 1024 * 1024 * 1024,        // 1GB
            include_extensions: vec!["*".to_string()],
            exclude_extensions: vec![".tmp".to_string(), ".log".to_string()],
            deep_analysis_enabled: false,
            parallel_workers: 4,
        }
    }

    /// Creates a production-hardened configuration for data quality analysis
    ///
    /// Returns an `AnalysisConfig` with aggressive scanning, deep analysis enabled,
    /// and increased parallelism for production workloads.
    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            scan_interval: Duration::from_secs(1800), // 30 minutes
            max_file_size: 10 * 1024 * 1024 * 1024,   // 10GB
            include_extensions: vec!["*".to_string()],
            exclude_extensions: vec![".tmp".to_string(), ".log".to_string(), ".cache".to_string()],
            deep_analysis_enabled: true,
            parallel_workers: 8,
        }
    }
}
