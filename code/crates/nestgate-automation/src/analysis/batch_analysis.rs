// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Multi-dataset analysis entry points.

use nestgate_core::error::Result;
use tracing::warn;

use crate::types::prediction::DataPattern;

use super::dataset_analyzer::DatasetAnalyzer;
use super::types::DatasetAnalysis;

/// Utility function to analyze multiple datasets with machine learning patterns
pub async fn analyze_datasets_with_patterns(
    datasets: &[String],
    _access_patterns: &[DataPattern],
) -> Result<Vec<DatasetAnalysis>> {
    let analyzer = DatasetAnalyzer::new();
    let mut results = Vec::with_capacity(datasets.len());
    for dataset_path in datasets {
        match analyzer.analyze_dataset(dataset_path).await {
            Ok(analysis) => {
                results.push(analysis);
            }
            Err(e) => {
                warn!("Failed to analyze dataset {}: {}", dataset_path, e);
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use crate::types::prediction::DataPattern;

    use super::analyze_datasets_with_patterns;

    #[tokio::test]
    async fn analyze_datasets_empty_returns_empty() {
        let out = analyze_datasets_with_patterns(&[], &[]).await.unwrap();
        assert!(out.is_empty());
    }

    #[tokio::test]
    async fn analyze_datasets_missing_paths_ok_with_no_results() {
        let out =
            analyze_datasets_with_patterns(&["/nonexistent/nestgate_batch_test_path".into()], &[])
                .await
                .unwrap();
        assert!(out.is_empty());
    }

    #[tokio::test]
    async fn analyze_datasets_passes_patterns_slice() {
        let patterns = [DataPattern::Sequential];
        let out = analyze_datasets_with_patterns(&[], &patterns)
            .await
            .unwrap();
        assert!(out.is_empty());
    }
}
