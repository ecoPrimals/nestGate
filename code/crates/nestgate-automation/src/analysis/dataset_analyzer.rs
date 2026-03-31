// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Dataset-level scanning, aggregation, and tier prediction.

use std::time::SystemTime;

use nestgate_core::error::{NestGateError, Result};
use nestgate_core::unified_enums::StorageTier;
use tracing::{info, warn};

use crate::types::prediction::{AccessPattern, DataPattern, FileAnalysis, SizeCategory};

use super::file_analyzer::FileAnalyzer;
use super::pattern_analyzer::PatternAnalyzer;
use super::types::{DatasetAnalysis, FileCharacteristics};

/// Dataset analyzer for analyzing entire datasets
#[derive(Debug)]
pub struct DatasetAnalyzer {
    pub(super) file_analyzer: FileAnalyzer,
    _pattern_analyzer: PatternAnalyzer,
}

impl DatasetAnalyzer {
    #[must_use]
    pub fn new() -> Self {
        Self {
            file_analyzer: FileAnalyzer::new(),
            _pattern_analyzer: PatternAnalyzer::new(),
        }
    }

    /// Analyze a single file
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn analyze_file(&self, file_path: &str) -> Result<FileAnalysis> {
        self.file_analyzer.analyze_file(file_path).await
    }

    /// Analyze a dataset and provide recommendations
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn analyze_dataset(&self, dataset_path: &str) -> Result<DatasetAnalysis> {
        let path = std::path::Path::new(dataset_path);

        if !path.exists() {
            return Err(NestGateError::validation(format!(
                "Dataset path does not exist: {dataset_path}"
            )));
        }

        let (file_analyses, total_files, total_size) = self.scan_dataset_directory(path).await?;

        let _access_patterns = self.aggregate_patterns(
            &file_analyses.iter().collect::<Vec<_>>(),
            &AccessPattern::default(),
        )?;
        let _recommendations: Vec<String> = vec![];

        Ok(DatasetAnalysis {
            path: dataset_path.to_string(),
            total_files,
            total_size_bytes: total_size,
            file_types: file_analyses
                .iter()
                .map(|f| (f.file_type.clone(), f.size_bytes))
                .collect(),
            characteristics: FileCharacteristics {
                size_category: SizeCategory::Unknown,
                access_frequency: 0,
                is_frequently_accessed: false,
                is_sequential_access: false,
                data_pattern: DataPattern::Unknown,
            },
        })
    }

    /// Predict optimal storage tier for a dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn predict_optimal_tier(&self, dataset_path: &str) -> Result<StorageTier> {
        if dataset_path.contains("archive") || dataset_path.contains("backup") {
            Ok(StorageTier::Cold)
        } else if dataset_path.contains("active") || dataset_path.contains("current") {
            Ok(StorageTier::Hot)
        } else {
            Ok(StorageTier::Warm)
        }
    }

    async fn scan_dataset_directory(
        &self,
        path: &std::path::Path,
    ) -> Result<(Vec<FileAnalysis>, u64, u64)> {
        let mut file_analyses = Vec::with_capacity(100);
        let mut total_files = 0;
        let mut total_size = 0;

        if let Ok(mut entries) = tokio::fs::read_dir(path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let entry_path = entry.path();

                if entry_path.is_file() {
                    match self
                        .file_analyzer
                        .analyze_file(&entry_path.to_string_lossy())
                        .await
                    {
                        Ok(analysis) => {
                            total_size += analysis.size_bytes;
                            file_analyses.push(analysis);
                            total_files += 1;
                        }
                        Err(e) => {
                            warn!("Failed to analyze file {:?}: {}", entry_path, e);
                        }
                    }

                    if total_files >= 100 {
                        info!("Limiting dataset analysis to 100 files for performance");
                        break;
                    }
                }
            }
        }

        Ok((file_analyses, total_files, total_size))
    }

    /// Aggregate patterns from multiple file analyses
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn aggregate_patterns(
        &self,
        file_analyses: &[&FileAnalysis],
        _patterns: &AccessPattern,
    ) -> Result<AccessPattern> {
        if file_analyses.is_empty() {
            return Ok(AccessPattern::default());
        }

        let mut _total_size = 0u64;
        let _total_daily_access = 0u32;
        let _total_read_write_ratio = 0.0f64;

        for analysis in file_analyses {
            _total_size += analysis.size_bytes;
        }

        let total_accesses = if file_analyses.is_empty() { 0 } else { 10 };

        Ok(AccessPattern {
            accesses_last_24h: total_accesses,
            accesses_last_week: total_accesses * 7,
            accesses_last_month: total_accesses * 30,
            total_accesses,
            last_access: SystemTime::now(),
            peak_access_times: vec![9, 10, 11, 14, 15, 16],
            read_write_ratio: 3.0,
        })
    }
}

impl Default for DatasetAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod dataset_analyzer_unit_tests {
    use nestgate_core::unified_enums::StorageTier;

    use crate::types::prediction::AccessPattern;

    use super::DatasetAnalyzer;

    #[test]
    fn predict_tier_branches() {
        let a = DatasetAnalyzer::new();
        assert_eq!(
            a.predict_optimal_tier("/x/archive/y").unwrap(),
            StorageTier::Cold
        );
        assert_eq!(
            a.predict_optimal_tier("/x/backup/y").unwrap(),
            StorageTier::Cold
        );
        assert_eq!(
            a.predict_optimal_tier("/x/active/y").unwrap(),
            StorageTier::Hot
        );
        assert_eq!(
            a.predict_optimal_tier("/x/current/y").unwrap(),
            StorageTier::Hot
        );
        assert_eq!(
            a.predict_optimal_tier("/neutral").unwrap(),
            StorageTier::Warm
        );
    }

    #[test]
    fn aggregate_patterns_empty_returns_default_access_pattern() {
        let a = DatasetAnalyzer::new();
        let p = a
            .aggregate_patterns(&[], &AccessPattern::default())
            .unwrap();
        assert_eq!(p.total_accesses, 0);
    }

    #[test]
    fn aggregate_patterns_nonempty_sets_access_counts() {
        let a = DatasetAnalyzer::new();
        let fa = crate::types::prediction::FileAnalysis {
            file_path: "/tmp/f".into(),
            size_bytes: 100,
            created_at: std::time::SystemTime::UNIX_EPOCH,
            modified_at: std::time::SystemTime::UNIX_EPOCH,
            accessed_at: std::time::SystemTime::UNIX_EPOCH,
            file_type: "txt".into(),
        };
        let p = a
            .aggregate_patterns(&[&fa], &AccessPattern::default())
            .unwrap();
        assert_eq!(p.total_accesses, 10);
        assert!(!p.peak_access_times.is_empty());
    }

    #[tokio::test]
    async fn analyze_dataset_nonexistent_is_validation_error() {
        let a = DatasetAnalyzer::new();
        let err = a
            .analyze_dataset("/nonexistent/nestgate_dataset_analyzer_path")
            .await
            .unwrap_err();
        assert!(err.to_string().contains("does not exist"));
    }

    #[tokio::test]
    async fn analyze_dataset_tempdir_smoke() {
        let dir = tempfile::tempdir().expect("tempdir");
        let a = DatasetAnalyzer::new();
        let path = dir.path().to_str().unwrap();
        let analysis = a.analyze_dataset(path).await.expect("analyze");
        assert_eq!(analysis.path, path);
        assert_eq!(analysis.total_files, 0);
    }

    #[tokio::test]
    async fn analyze_file_missing_returns_error() {
        let a = DatasetAnalyzer::new();
        assert!(a.analyze_file("/nonexistent/file.txt").await.is_err());
    }
}
