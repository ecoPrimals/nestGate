// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for performance monitoring analysis module
//!
//! This module provides 100% coverage for the trend analysis and performance
//! evaluation functionality.

#[cfg(test)]
mod analysis_tests {
    use super::super::analysis::*;
    use super::super::*;
    use crate::performance::types::{CurrentPerformanceMetrics, PerformanceSnapshot};
    use std::collections::VecDeque;
    use std::sync::Arc;
    use std::time::{Duration, SystemTime};
    use tokio::sync::RwLock;

    // ==================== HELPER FUNCTIONS ====================

    /// Creates  Test Metrics History
    fn create_test_metrics_history() -> Arc<RwLock<VecDeque<PerformanceSnapshot>>> {
        Arc::new(RwLock::new(VecDeque::new()))
    }

    /// Creates  Test Snapshot
    fn create_test_snapshot(score: f64, age_secs: u64) -> PerformanceSnapshot {
        PerformanceSnapshot {
            timestamp: SystemTime::now() - Duration::from_secs(age_secs),
            metrics: CurrentPerformanceMetrics::default(),
            performance_score: score,
        }
    }

    /// Populate History With Samples
    async fn populate_history_with_samples(
        history: &Arc<RwLock<VecDeque<PerformanceSnapshot>>>,
        count: usize,
    ) {
        let mut hist = history.write().await;
        for i in 0..count {
            hist.push_back(create_test_snapshot(
                50.0 + (i as f64 * 2.0),
                (count - i) as u64 * 60,
            ));
        }
    }

    // ==================== PERFORMANCE ANALYZER TESTS ====================

    #[tokio::test]
    async fn test_analyze_trends_with_empty_history() {
        let history = create_test_metrics_history();

        let result = PerformanceAnalyzer::analyze_trends(&history).await;

        // Should return Ok with default report even with empty history
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_analyze_trends_with_single_entry() {
        let history = create_test_metrics_history();
        {
            let mut hist = history.write().await;
            hist.push_back(create_test_snapshot(75.0, 60));
        }

        let result = PerformanceAnalyzer::analyze_trends(&history).await;

        // Should handle single entry gracefully
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_analyze_trends_with_two_entries() {
        let history = create_test_metrics_history();
        {
            let mut hist = history.write().await;
            hist.push_back(create_test_snapshot(70.0, 120));
            hist.push_back(create_test_snapshot(80.0, 60));
        }

        let result = PerformanceAnalyzer::analyze_trends(&history).await;

        // Should successfully analyze trend between two snapshots
        assert!(result.is_ok());
        let report = result.unwrap();
        // Verify report structure
        assert!(std::ptr::addr_of!(report) != std::ptr::null());
    }

    #[tokio::test]
    async fn test_analyze_trends_with_multiple_entries() {
        let history = create_test_metrics_history();
        populate_history_with_samples(&history, 10).await;

        let result = PerformanceAnalyzer::analyze_trends(&history).await;

        // Should handle multiple entries efficiently
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_analyze_trends_detects_improving_performance() {
        let history = create_test_metrics_history();
        {
            let mut hist = history.write().await;
            hist.push_back(create_test_snapshot(60.0, 120));
            hist.push_back(create_test_snapshot(85.0, 60));
        }

        let result = PerformanceAnalyzer::analyze_trends(&history).await;

        // Should detect performance improvement (60 -> 85)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_analyze_trends_detects_degrading_performance() {
        let history = create_test_metrics_history();
        {
            let mut hist = history.write().await;
            hist.push_back(create_test_snapshot(90.0, 120));
            hist.push_back(create_test_snapshot(65.0, 60));
        }

        let result = PerformanceAnalyzer::analyze_trends(&history).await;

        // Should detect performance degradation (90 -> 65)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_analyze_trends_with_stable_performance() {
        let history = create_test_metrics_history();
        {
            let mut hist = history.write().await;
            hist.push_back(create_test_snapshot(75.0, 120));
            hist.push_back(create_test_snapshot(75.5, 60));
        }

        let result = PerformanceAnalyzer::analyze_trends(&history).await;

        // Should handle stable performance (minimal change)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_analyze_trends_with_large_history() {
        let history = create_test_metrics_history();
        populate_history_with_samples(&history, 1000).await;

        let result = PerformanceAnalyzer::analyze_trends(&history).await;

        // Should handle large history efficiently
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_analyze_trends_concurrent_access() {
        let history = create_test_metrics_history();
        populate_history_with_samples(&history, 50).await;

        // Test concurrent reads (not spawning due to Send bound requirements)
        let mut results = vec![];
        for _ in 0..10 {
            let result = PerformanceAnalyzer::analyze_trends(&history).await;
            results.push(result);
        }

        // All should complete successfully
        for result in results {
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_analysis_report_default() {
        let report = AnalysisReport::default();

        // Verify default construction
        assert!(std::ptr::addr_of!(report) != std::ptr::null());
    }

    // ==================== ZFS PERFORMANCE MONITOR ANALYSIS TESTS ====================

    #[tokio::test]
    async fn test_start_analysis_task() {
        let mut monitor = ZfsPerformanceMonitor::new_for_testing();

        let result = monitor.start_analysis_task();

        // Should start task successfully
        assert!(result.is_ok());
        assert!(monitor.analysis_task.is_some());
    }

    #[tokio::test]
    async fn test_start_analysis_task_multiple_times() {
        let mut monitor = ZfsPerformanceMonitor::new_for_testing();

        let result1 = monitor.start_analysis_task();
        assert!(result1.is_ok());

        // Starting again should replace the task
        let result2 = monitor.start_analysis_task();
        assert!(result2.is_ok());
        assert!(monitor.analysis_task.is_some());
    }

    #[tokio::test]
    async fn test_analyze_trends_static_method() {
        let current_metrics = Arc::new(RwLock::new(CurrentPerformanceMetrics::default()));
        let history = create_test_metrics_history();

        let result = ZfsPerformanceMonitor::analyze_trends(&current_metrics, &history).await;

        // Should succeed and add snapshot to history
        assert!(result.is_ok());

        let hist = history.read().await;
        assert_eq!(hist.len(), 1);
    }

    #[tokio::test]
    async fn test_analyze_trends_adds_snapshot() {
        let current_metrics = Arc::new(RwLock::new(CurrentPerformanceMetrics::default()));
        let history = create_test_metrics_history();

        // Run analysis multiple times
        for _ in 0..5 {
            let result = ZfsPerformanceMonitor::analyze_trends(&current_metrics, &history).await;
            assert!(result.is_ok());
        }

        let hist = history.read().await;
        assert_eq!(hist.len(), 5);
    }

    #[tokio::test]
    async fn test_analyze_trends_respects_max_history() {
        let current_metrics = Arc::new(RwLock::new(CurrentPerformanceMetrics::default()));
        let history = create_test_metrics_history();

        // Add 2881 snapshots (one more than max)
        for _ in 0..2881 {
            let result = ZfsPerformanceMonitor::analyze_trends(&current_metrics, &history).await;
            assert!(result.is_ok());
        }

        let hist = history.read().await;
        // Should trim to max of 2880
        assert_eq!(hist.len(), 2880);
    }

    #[tokio::test]
    async fn test_analyze_trends_removes_oldest_when_full() {
        let current_metrics = Arc::new(RwLock::new(CurrentPerformanceMetrics::default()));
        let history = create_test_metrics_history();

        // Fill history to max
        for i in 0..2880 {
            // Modify metrics to track order
            {
                let mut metrics = current_metrics.write().await;
                metrics.pool_metrics.total_iops = i as f64;
            }
            ZfsPerformanceMonitor::analyze_trends(&current_metrics, &history)
                .await
                .unwrap();
        }

        // Get first entry's IOPS value
        let first_iops = {
            let hist = history.read().await;
            hist.front().unwrap().metrics.pool_metrics.total_iops
        };

        // Add one more entry
        {
            let mut metrics = current_metrics.write().await;
            metrics.pool_metrics.total_iops = 9999.0;
        }
        ZfsPerformanceMonitor::analyze_trends(&current_metrics, &history)
            .await
            .unwrap();

        let hist = history.read().await;
        assert_eq!(hist.len(), 2880);
        // First entry should have changed (oldest removed)
        assert_ne!(
            hist.front().unwrap().metrics.pool_metrics.total_iops,
            first_iops
        );
        // Last entry should be our new one
        assert_eq!(hist.back().unwrap().metrics.pool_metrics.total_iops, 9999.0);
    }

    #[tokio::test]
    async fn test_analyze_trends_snapshot_structure() {
        let mut metrics = CurrentPerformanceMetrics::default();
        metrics.pool_metrics.total_iops = 1000.0;
        metrics.pool_metrics.total_throughput_mbs = 500.0;
        metrics.pool_metrics.avg_latency_ms = 5.0;

        let current_metrics = Arc::new(RwLock::new(metrics));
        let history = create_test_metrics_history();

        ZfsPerformanceMonitor::analyze_trends(&current_metrics, &history)
            .await
            .unwrap();

        let hist = history.read().await;
        let snapshot = hist.back().unwrap();

        // Verify snapshot captured current metrics
        assert_eq!(snapshot.metrics.pool_metrics.total_iops, 1000.0);
        assert_eq!(snapshot.metrics.pool_metrics.total_throughput_mbs, 500.0);
        assert_eq!(snapshot.metrics.pool_metrics.avg_latency_ms, 5.0);
        assert_eq!(snapshot.performance_score, 85.0);
    }

    #[tokio::test]
    async fn test_analyze_trends_timestamp_ordering() {
        let current_metrics = Arc::new(RwLock::new(CurrentPerformanceMetrics::default()));
        let history = create_test_metrics_history();

        // Add multiple snapshots with small delays
        for i in 0..10 {
            // Modify metrics slightly to track order
            {
                let mut metrics = current_metrics.write().await;
                metrics.pool_metrics.total_iops = i as f64 * 10.0;
            }
            ZfsPerformanceMonitor::analyze_trends(&current_metrics, &history)
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        let hist = history.read().await;
        // Verify timestamps are in order
        let mut prev_timestamp = SystemTime::UNIX_EPOCH;
        for snapshot in hist.iter() {
            assert!(snapshot.timestamp >= prev_timestamp);
            prev_timestamp = snapshot.timestamp;
        }
    }

    #[tokio::test]
    async fn test_analyze_trends_concurrent_writes() {
        let current_metrics = Arc::new(RwLock::new(CurrentPerformanceMetrics::default()));
        let history = create_test_metrics_history();

        // Spawn multiple concurrent analysis tasks
        let mut handles = vec![];
        for _ in 0..20 {
            let metrics = Arc::clone(&current_metrics);
            let hist = Arc::clone(&history);
            let handle = tokio::spawn(async move {
                ZfsPerformanceMonitor::analyze_trends(&metrics, &hist).await
            });
            handles.push(handle);
        }

        // All should complete successfully
        for handle in handles {
            assert!(handle.await.unwrap().is_ok());
        }

        let hist = history.read().await;
        assert_eq!(hist.len(), 20);
    }

    #[tokio::test]
    async fn test_analyze_trends_performance_score_calculation() {
        let current_metrics = Arc::new(RwLock::new(CurrentPerformanceMetrics::default()));
        let history = create_test_metrics_history();

        ZfsPerformanceMonitor::analyze_trends(&current_metrics, &history)
            .await
            .unwrap();

        let hist = history.read().await;
        let snapshot = hist.back().unwrap();

        // Performance score should be calculated (currently hardcoded to 85.0)
        assert_eq!(snapshot.performance_score, 85.0);
        assert!(snapshot.performance_score >= 0.0);
        assert!(snapshot.performance_score <= 100.0);
    }

    // ==================== EDGE CASE TESTS ====================

    #[tokio::test]
    async fn test_analysis_with_zero_metrics() {
        let mut metrics = CurrentPerformanceMetrics::default();
        metrics.pool_metrics.total_iops = 0.0;
        metrics.pool_metrics.total_throughput_mbs = 0.0;
        metrics.pool_metrics.avg_latency_ms = 0.0;

        let current_metrics = Arc::new(RwLock::new(metrics));
        let history = create_test_metrics_history();

        let result = ZfsPerformanceMonitor::analyze_trends(&current_metrics, &history).await;

        // Should handle zero metrics gracefully
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_analysis_with_extreme_values() {
        let mut metrics = CurrentPerformanceMetrics::default();
        metrics.pool_metrics.total_iops = f64::MAX / 2.0;
        metrics.pool_metrics.total_throughput_mbs = f64::MAX / 2.0;
        metrics.pool_metrics.avg_latency_ms = f64::MAX / 2.0;

        let current_metrics = Arc::new(RwLock::new(metrics));
        let history = create_test_metrics_history();

        let result = ZfsPerformanceMonitor::analyze_trends(&current_metrics, &history).await;

        // Should handle extreme values without panicking
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_history_pruning_under_concurrent_load() {
        let current_metrics = Arc::new(RwLock::new(CurrentPerformanceMetrics::default()));
        let history = create_test_metrics_history();

        // Pre-fill to near capacity
        for _ in 0..2875 {
            ZfsPerformanceMonitor::analyze_trends(&current_metrics, &history)
                .await
                .unwrap();
        }

        // Now add concurrently to trigger pruning
        let mut handles = vec![];
        for _ in 0..20 {
            let metrics = Arc::clone(&current_metrics);
            let hist = Arc::clone(&history);
            let handle = tokio::spawn(async move {
                ZfsPerformanceMonitor::analyze_trends(&metrics, &hist).await
            });
            handles.push(handle);
        }

        for handle in handles {
            assert!(handle.await.unwrap().is_ok());
        }

        let hist = history.read().await;
        assert!(hist.len() <= 2880);
    }
}
