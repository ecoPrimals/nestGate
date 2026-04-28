// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Data types and structures for adaptive optimization.
#![expect(
    dead_code,
    reason = "Optimization types are staged for the adaptive engine; unit tests construct them"
)]

use std::collections::VecDeque;
use std::sync::atomic::AtomicU64;
use std::time::{Duration, Instant, SystemTime};

/// Performance history tracking
pub struct PerformanceHistory {
    snapshots: VecDeque<PerformanceSnapshot>,
    max_history_size: usize,
    analysis_window_size: usize,
}

/// Performance snapshot at a specific point in time
pub struct PerformanceSnapshot {
    /// Timestamp
    pub timestamp: Instant,
    /// Cpu Utilization
    pub cpu_utilization: f64,
    /// Memory Utilization
    pub memory_utilization: f64,
    /// Network Throughput
    pub network_throughput: u64,
    /// Disk Iops
    pub disk_iops: u64,
    /// Cache Hit Ratio
    pub cache_hit_ratio: f64,
    /// Lock Contention Ratio
    pub lock_contention_ratio: f64,
    /// Simd Utilization
    pub simd_utilization: f64,
    /// Allocation Efficiency
    pub allocation_efficiency: f64,
}

/// Optimization strategy configuration
#[derive(Clone, Copy)]
pub struct OptimizationStrategy {
    target_cpu_utilization: f64,
    target_memory_utilization: f64,
    aggressive_tuning: bool,
    learning_rate: f64,
}

/// Optimization decision made by the engine
pub struct OptimizationDecision {
    /// Parameter Adjustments
    pub parameter_adjustments: Vec<TunableParameter>,
    /// Confidence Score
    pub confidence_score: f64,
    /// Expected Improvement
    pub expected_improvement: f64,
    /// Risk Assessment
    pub risk_assessment: f64,
}

/// Tunable system parameter
#[derive(Clone)]
pub struct TunableParameter {
    name: String,
    current_value: f64,
    suggested_value: f64,
    adjustment_confidence: f64,
}

/// Tuning action to be performed
/// Tuningaction
pub struct TuningAction {
    parameter_name: String,
    old_value: f64,
    new_value: f64,
    timestamp: SystemTime,
}

/// Current system metrics
pub struct CurrentMetrics {
    /// Cpu Usage
    pub cpu_usage: f64,
    /// Memory Usage
    pub memory_usage: f64,
    /// Network Throughput
    pub network_throughput: u64,
    /// Disk Iops
    pub disk_iops: u64,
    /// Cache Hit Ratio
    pub cache_hit_ratio: f64,
    /// Lock Contention
    pub lock_contention: f64,
    /// Simd Utilization
    pub simd_utilization: f64,
    /// Allocation Efficiency
    pub allocation_efficiency: f64,
}

/// Optimization prediction result
pub struct OptimizationPrediction {
    /// Predicted Improvement
    pub predicted_improvement: f64,
    /// Confidence
    pub confidence: f64,
}

/// Result of optimization attempt
pub struct OptimizationResult {
    /// Success
    pub success: bool,
    /// Improvement Achieved
    pub improvement_achieved: f64,
}

/// Statistics for adaptive performance monitoring
/// Adaptiveperformancestats
pub struct AdaptivePerformanceStats {
    total_optimizations: AtomicU64,
    successful_optimizations: AtomicU64,
    average_improvement: AtomicU64, // Percentage * 100
    uptime_seconds: AtomicU64,
}

/// Statistics for optimization engine
/// Optimizationenginestats
pub struct OptimizationEngineStats {
    decisions_made: AtomicU64,
    average_confidence: AtomicU64, // Percentage * 100
    learning_iterations: AtomicU64,
}

/// Statistics for auto tuner
/// Autotunerstats
pub struct AutoTunerStats {
    parameters_tuned: AtomicU64,
    tuning_actions: AtomicU64,
    rollbacks: AtomicU64,
}

/// Trend analysis results
pub struct TrendAnalysis {
    /// Cpu Trend
    pub cpu_trend: f64,
    /// Memory Trend
    pub memory_trend: f64,
    /// Throughput Trend
    pub throughput_trend: f64,
    /// Efficiency Trend
    pub efficiency_trend: f64,
    /// Prediction Window
    pub prediction_window: Duration,
}

impl PerformanceHistory {
    #[must_use]
    pub fn new(max_size: usize, analysis_window: usize) -> Self {
        Self {
            snapshots: VecDeque::with_capacity(max_size),
            max_history_size: max_size,
            analysis_window_size: analysis_window,
        }
    }

    /// Add Snapshot
    pub fn add_snapshot(&mut self, snapshot: PerformanceSnapshot) {
        if self.snapshots.len() >= self.max_history_size {
            self.snapshots.pop_front();
        }
        self.snapshots.push_back(snapshot);
    }

    #[must_use]
    pub fn get_recent_snapshots(&self, count: usize) -> Vec<&PerformanceSnapshot> {
        let start_idx = if self.snapshots.len() > count {
            self.snapshots.len() - count
        } else {
            0
        };

        self.snapshots.range(start_idx..).collect()
    }

    #[must_use]
    pub fn get_analysis_window(&self) -> Vec<&PerformanceSnapshot> {
        self.get_recent_snapshots(self.analysis_window_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_history_creation() {
        let history = PerformanceHistory::new(100, 10);
        assert_eq!(history.max_history_size, 100);
        assert_eq!(history.analysis_window_size, 10);
        assert_eq!(history.snapshots.len(), 0);
    }

    #[test]
    fn test_performance_history_add_snapshot() {
        let mut history = PerformanceHistory::new(100, 10);
        let snapshot = PerformanceSnapshot {
            timestamp: Instant::now(),
            cpu_utilization: 0.5,
            memory_utilization: 0.6,
            network_throughput: 1000,
            disk_iops: 500,
            cache_hit_ratio: 0.9,
            lock_contention_ratio: 0.1,
            simd_utilization: 0.8,
            allocation_efficiency: 0.95,
        };
        history.add_snapshot(snapshot);
        assert_eq!(history.snapshots.len(), 1);
    }

    #[test]
    fn test_performance_history_max_size() {
        let mut history = PerformanceHistory::new(5, 3);
        for _ in 0..10 {
            let snapshot = PerformanceSnapshot {
                timestamp: Instant::now(),
                cpu_utilization: 0.5,
                memory_utilization: 0.6,
                network_throughput: 1000,
                disk_iops: 500,
                cache_hit_ratio: 0.9,
                lock_contention_ratio: 0.1,
                simd_utilization: 0.8,
                allocation_efficiency: 0.95,
            };
            history.add_snapshot(snapshot);
        }
        assert_eq!(history.snapshots.len(), 5);
    }

    #[test]
    fn test_get_recent_snapshots() {
        let mut history = PerformanceHistory::new(100, 10);
        for _ in 0..15 {
            let snapshot = PerformanceSnapshot {
                timestamp: Instant::now(),
                cpu_utilization: 0.5,
                memory_utilization: 0.6,
                network_throughput: 1000,
                disk_iops: 500,
                cache_hit_ratio: 0.9,
                lock_contention_ratio: 0.1,
                simd_utilization: 0.8,
                allocation_efficiency: 0.95,
            };
            history.add_snapshot(snapshot);
        }
        let recent = history.get_recent_snapshots(5);
        assert_eq!(recent.len(), 5);
    }

    #[test]
    fn test_get_analysis_window() {
        let mut history = PerformanceHistory::new(100, 10);
        for _ in 0..20 {
            let snapshot = PerformanceSnapshot {
                timestamp: Instant::now(),
                cpu_utilization: 0.5,
                memory_utilization: 0.6,
                network_throughput: 1000,
                disk_iops: 500,
                cache_hit_ratio: 0.9,
                lock_contention_ratio: 0.1,
                simd_utilization: 0.8,
                allocation_efficiency: 0.95,
            };
            history.add_snapshot(snapshot);
        }
        let window = history.get_analysis_window();
        assert_eq!(window.len(), 10);
    }

    #[test]
    fn test_optimization_strategy_copy() {
        let strategy = OptimizationStrategy {
            target_cpu_utilization: 0.7,
            target_memory_utilization: 0.8,
            aggressive_tuning: true,
            learning_rate: 0.01,
        };
        let copied = strategy;
        assert_eq!(copied.target_cpu_utilization, 0.7);
        assert_eq!(copied.target_memory_utilization, 0.8);
        assert!(copied.aggressive_tuning);
        assert_eq!(copied.learning_rate, 0.01);
        let _ = strategy; // verify original still usable (Copy semantics)
    }

    #[test]
    fn test_tunable_parameter_clone() {
        let param = TunableParameter {
            name: "cache_size".to_string(),
            current_value: 100.0,
            suggested_value: 150.0,
            adjustment_confidence: 0.9,
        };
        let cloned = param.clone();
        assert_eq!(cloned.name, "cache_size");
        assert_eq!(cloned.current_value, 100.0);
        assert_eq!(cloned.suggested_value, 150.0);
        assert_eq!(cloned.adjustment_confidence, 0.9);
    }

    #[test]
    fn test_optimization_result_success() {
        let result = OptimizationResult {
            success: true,
            improvement_achieved: 0.25,
        };
        assert!(result.success);
        assert_eq!(result.improvement_achieved, 0.25);
    }

    #[test]
    fn test_optimization_result_failure() {
        let result = OptimizationResult {
            success: false,
            improvement_achieved: -0.05,
        };
        assert!(!result.success);
        assert_eq!(result.improvement_achieved, -0.05);
    }

    #[test]
    fn test_adaptive_performance_stats_creation() {
        let stats = AdaptivePerformanceStats {
            total_optimizations: AtomicU64::new(100),
            successful_optimizations: AtomicU64::new(85),
            average_improvement: AtomicU64::new(2000), // 20%
            uptime_seconds: AtomicU64::new(3600),
        };
        assert_eq!(
            stats
                .total_optimizations
                .load(std::sync::atomic::Ordering::Relaxed),
            100
        );
        assert_eq!(
            stats
                .successful_optimizations
                .load(std::sync::atomic::Ordering::Relaxed),
            85
        );
    }

    #[test]
    fn test_optimization_engine_stats() {
        let stats = OptimizationEngineStats {
            decisions_made: AtomicU64::new(50),
            average_confidence: AtomicU64::new(8500), // 85%
            learning_iterations: AtomicU64::new(1000),
        };
        assert_eq!(
            stats
                .decisions_made
                .load(std::sync::atomic::Ordering::Relaxed),
            50
        );
        assert_eq!(
            stats
                .average_confidence
                .load(std::sync::atomic::Ordering::Relaxed),
            8500
        );
    }

    #[test]
    fn test_auto_tuner_stats() {
        let stats = AutoTunerStats {
            parameters_tuned: AtomicU64::new(25),
            tuning_actions: AtomicU64::new(100),
            rollbacks: AtomicU64::new(5),
        };
        assert_eq!(
            stats
                .parameters_tuned
                .load(std::sync::atomic::Ordering::Relaxed),
            25
        );
        assert_eq!(
            stats.rollbacks.load(std::sync::atomic::Ordering::Relaxed),
            5
        );
    }

    #[test]
    fn test_trend_analysis_creation() {
        let analysis = TrendAnalysis {
            cpu_trend: 0.05,
            memory_trend: 0.03,
            throughput_trend: 100.0,
            efficiency_trend: 0.02,
            prediction_window: Duration::from_secs(300),
        };
        assert_eq!(analysis.cpu_trend, 0.05);
        assert_eq!(analysis.memory_trend, 0.03);
        assert_eq!(analysis.throughput_trend, 100.0);
    }

    #[test]
    fn test_performance_snapshot_values() {
        let snapshot = PerformanceSnapshot {
            timestamp: Instant::now(),
            cpu_utilization: 0.75,
            memory_utilization: 0.65,
            network_throughput: 5000,
            disk_iops: 1500,
            cache_hit_ratio: 0.92,
            lock_contention_ratio: 0.08,
            simd_utilization: 0.85,
            allocation_efficiency: 0.97,
        };
        assert_eq!(snapshot.cpu_utilization, 0.75);
        assert_eq!(snapshot.memory_utilization, 0.65);
        assert_eq!(snapshot.network_throughput, 5000);
    }

    #[test]
    fn test_optimization_decision_creation() {
        let decision = OptimizationDecision {
            parameter_adjustments: vec![],
            confidence_score: 0.85,
            expected_improvement: 0.15,
            risk_assessment: 0.1,
        };
        assert_eq!(decision.confidence_score, 0.85);
        assert_eq!(decision.expected_improvement, 0.15);
        assert_eq!(decision.risk_assessment, 0.1);
    }

    #[test]
    fn test_tuning_action_creation() {
        let action = TuningAction {
            parameter_name: "thread_pool_size".to_string(),
            old_value: 4.0,
            new_value: 8.0,
            timestamp: SystemTime::now(),
        };
        assert_eq!(action.parameter_name, "thread_pool_size");
        assert_eq!(action.old_value, 4.0);
        assert_eq!(action.new_value, 8.0);
    }

    #[test]
    fn test_current_metrics_creation() {
        let metrics = CurrentMetrics {
            cpu_usage: 0.6,
            memory_usage: 0.7,
            network_throughput: 3000,
            disk_iops: 800,
            cache_hit_ratio: 0.88,
            lock_contention: 0.12,
            simd_utilization: 0.75,
            allocation_efficiency: 0.93,
        };
        assert_eq!(metrics.cpu_usage, 0.6);
        assert_eq!(metrics.memory_usage, 0.7);
        assert_eq!(metrics.cache_hit_ratio, 0.88);
    }

    #[test]
    fn test_optimization_prediction() {
        let prediction = OptimizationPrediction {
            predicted_improvement: 0.2,
            confidence: 0.9,
        };
        assert_eq!(prediction.predicted_improvement, 0.2);
        assert_eq!(prediction.confidence, 0.9);
    }

    #[test]
    fn test_empty_performance_history() {
        let history = PerformanceHistory::new(100, 10);
        let recent = history.get_recent_snapshots(5);
        assert_eq!(recent.len(), 0);
    }

    #[test]
    fn test_performance_history_with_few_snapshots() {
        let mut history = PerformanceHistory::new(100, 10);
        for _ in 0..3 {
            let snapshot = PerformanceSnapshot {
                timestamp: Instant::now(),
                cpu_utilization: 0.5,
                memory_utilization: 0.6,
                network_throughput: 1000,
                disk_iops: 500,
                cache_hit_ratio: 0.9,
                lock_contention_ratio: 0.1,
                simd_utilization: 0.8,
                allocation_efficiency: 0.95,
            };
            history.add_snapshot(snapshot);
        }
        let recent = history.get_recent_snapshots(10);
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_optimization_strategy_default_values() {
        let strategy = OptimizationStrategy {
            target_cpu_utilization: 0.8,
            target_memory_utilization: 0.8,
            aggressive_tuning: false,
            learning_rate: 0.01,
        };
        assert!(!strategy.aggressive_tuning);
        assert_eq!(strategy.learning_rate, 0.01);
    }

    #[test]
    fn test_tunable_parameter_adjustment() {
        let param = TunableParameter {
            name: "buffer_size".to_string(),
            current_value: 1024.0,
            suggested_value: 2048.0,
            adjustment_confidence: 0.95,
        };
        assert!(param.suggested_value > param.current_value);
        assert!(param.adjustment_confidence > 0.9);
    }

    #[test]
    fn test_multiple_tunable_parameters() {
        let params = [
            TunableParameter {
                name: "param1".to_string(),
                current_value: 10.0,
                suggested_value: 15.0,
                adjustment_confidence: 0.8,
            },
            TunableParameter {
                name: "param2".to_string(),
                current_value: 20.0,
                suggested_value: 25.0,
                adjustment_confidence: 0.9,
            },
        ];
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, "param1");
        assert_eq!(params[1].name, "param2");
    }

    #[test]
    fn test_optimization_decision_with_parameters() {
        let params = [TunableParameter {
            name: "cache_size".to_string(),
            current_value: 100.0,
            suggested_value: 150.0,
            adjustment_confidence: 0.9,
        }];
        let decision = OptimizationDecision {
            parameter_adjustments: params.to_vec(),
            confidence_score: 0.88,
            expected_improvement: 0.12,
            risk_assessment: 0.05,
        };
        assert_eq!(decision.parameter_adjustments.len(), 1);
        assert!(decision.risk_assessment < 0.1);
    }

    #[test]
    fn test_atomic_stats_thread_safety() {
        use std::sync::atomic::Ordering;
        let stats = AdaptivePerformanceStats {
            total_optimizations: AtomicU64::new(0),
            successful_optimizations: AtomicU64::new(0),
            average_improvement: AtomicU64::new(0),
            uptime_seconds: AtomicU64::new(0),
        };

        stats.total_optimizations.fetch_add(1, Ordering::Relaxed);
        assert_eq!(stats.total_optimizations.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_trend_analysis_positive_trends() {
        let analysis = TrendAnalysis {
            cpu_trend: 0.05,
            memory_trend: 0.03,
            throughput_trend: 150.0,
            efficiency_trend: 0.02,
            prediction_window: Duration::from_secs(600),
        };
        // Use epsilon for positive trend checks
        assert!(analysis.cpu_trend > 1e-9);
        assert!(analysis.throughput_trend > 1e-9);
    }

    #[test]
    fn test_trend_analysis_negative_trends() {
        let analysis = TrendAnalysis {
            cpu_trend: -0.05,
            memory_trend: -0.02,
            throughput_trend: -50.0,
            efficiency_trend: -0.01,
            prediction_window: Duration::from_secs(300),
        };
        // Use epsilon for negative trend checks
        assert!(analysis.cpu_trend < -1e-9);
        assert!(analysis.throughput_trend < -1e-9);
    }

    #[test]
    fn test_performance_history_capacity() {
        let history = PerformanceHistory::new(1000, 100);
        assert_eq!(history.snapshots.capacity(), 1000);
    }

    #[test]
    fn test_tuning_action_timestamp_is_recent() {
        let action = TuningAction {
            parameter_name: "test_param".to_string(),
            old_value: 1.0,
            new_value: 2.0,
            timestamp: SystemTime::now(),
        };
        let elapsed = action.timestamp.elapsed().expect("Operation failed");
        assert!(elapsed.as_secs() < 1);
    }

    #[test]
    fn test_optimization_result_zero_improvement() {
        let result = OptimizationResult {
            success: true,
            improvement_achieved: 0.0,
        };
        assert!(result.success);
        assert_eq!(result.improvement_achieved, 0.0);
    }

    #[test]
    fn test_optimization_result_large_improvement() {
        let result = OptimizationResult {
            success: true,
            improvement_achieved: 0.95,
        };
        assert!(result.success);
        assert!(result.improvement_achieved > 0.5);
    }

    #[test]
    fn test_current_metrics_all_fields() {
        let metrics = CurrentMetrics {
            cpu_usage: 0.5,
            memory_usage: 0.6,
            network_throughput: 2000,
            disk_iops: 1000,
            cache_hit_ratio: 0.85,
            lock_contention: 0.15,
            simd_utilization: 0.7,
            allocation_efficiency: 0.9,
        };
        // Verify all fields are accessible
        let _ = metrics.cpu_usage;
        let _ = metrics.memory_usage;
        let _ = metrics.network_throughput;
        let _ = metrics.disk_iops;
        let _ = metrics.cache_hit_ratio;
        let _ = metrics.lock_contention;
        let _ = metrics.simd_utilization;
        let _ = metrics.allocation_efficiency;
    }

    #[test]
    fn test_performance_snapshot_timestamp() {
        let snapshot = PerformanceSnapshot {
            timestamp: Instant::now(),
            cpu_utilization: 0.5,
            memory_utilization: 0.5,
            network_throughput: 1000,
            disk_iops: 500,
            cache_hit_ratio: 0.9,
            lock_contention_ratio: 0.1,
            simd_utilization: 0.8,
            allocation_efficiency: 0.95,
        };
        let elapsed = snapshot.timestamp.elapsed();
        assert!(elapsed.as_millis() < 100);
    }
}
