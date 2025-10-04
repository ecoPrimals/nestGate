//! Data types and structures for adaptive optimization.

use std::sync::atomic::{AtomicU64, AtomicUsize};
use std::time::{Duration, Instant, SystemTime};
use std::collections::VecDeque;

/// Performance history tracking
pub struct PerformanceHistory {
    snapshots: VecDeque<PerformanceSnapshot>,
    max_history_size: usize,
    analysis_window_size: usize,
}

/// Performance snapshot at a specific point in time
pub struct PerformanceSnapshot {
    timestamp: Instant,
    cpu_utilization: f64,
    memory_utilization: f64,
    network_throughput: u64,
    disk_iops: u64,
    cache_hit_ratio: f64,
    lock_contention_ratio: f64,
    simd_utilization: f64,
    allocation_efficiency: f64,
}

/// Optimization strategy configuration
pub struct OptimizationStrategy {
    target_cpu_utilization: f64,
    target_memory_utilization: f64,
    aggressive_tuning: bool,
    learning_rate: f64,
}

impl Clone for OptimizationStrategy {
    fn clone(&self) -> Self {
        Self {
            target_cpu_utilization: self.target_cpu_utilization,
            target_memory_utilization: self.target_memory_utilization,
            aggressive_tuning: self.aggressive_tuning,
            learning_rate: self.learning_rate,
        }
    }
}

/// Optimization decision made by the engine
pub struct OptimizationDecision {
    parameter_adjustments: Vec<TunableParameter>,
    confidence_score: f64,
    expected_improvement: f64,
    risk_assessment: f64,
}

/// Tunable system parameter
pub struct TunableParameter {
    name: String,
    current_value: f64,
    suggested_value: f64,
    adjustment_confidence: f64,
}

impl Clone for TunableParameter {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            current_value: self.current_value,
            suggested_value: self.suggested_value,
            adjustment_confidence: self.adjustment_confidence,
        }
    }
}

/// Tuning action to be performed
pub struct TuningAction {
    parameter_name: String,
    old_value: f64,
    new_value: f64,
    timestamp: SystemTime,
}

/// Current system metrics
pub struct CurrentMetrics {
    cpu_usage: f64,
    memory_usage: f64,
    network_throughput: u64,
    disk_iops: u64,
    cache_hit_ratio: f64,
    lock_contention: f64,
    simd_utilization: f64,
    allocation_efficiency: f64,
}

/// Optimization prediction result
pub struct OptimizationPrediction {
    predicted_improvement: f64,
    confidence: f64,
}

/// Result of optimization attempt
pub struct OptimizationResult {
    success: bool,
    improvement_achieved: f64,
}

/// Statistics for adaptive performance monitoring
pub struct AdaptivePerformanceStats {
    total_optimizations: AtomicU64,
    successful_optimizations: AtomicU64,
    average_improvement: AtomicU64, // Percentage * 100
    uptime_seconds: AtomicU64,
}

/// Statistics for optimization engine
pub struct OptimizationEngineStats {
    decisions_made: AtomicU64,
    average_confidence: AtomicU64, // Percentage * 100
    learning_iterations: AtomicU64,
}

/// Statistics for auto tuner
pub struct AutoTunerStats {
    parameters_tuned: AtomicU64,
    tuning_actions: AtomicU64,
    rollbacks: AtomicU64,
}

/// Trend analysis results
pub struct TrendAnalysis {
    cpu_trend: f64,
    memory_trend: f64,
    throughput_trend: f64,
    efficiency_trend: f64,
    prediction_window: Duration,
}

impl PerformanceHistory {
    pub fn new(max_size: usize, analysis_window: usize) -> Self {
        Self {
            snapshots: VecDeque::with_capacity(max_size),
            max_history_size: max_size,
            analysis_window_size: analysis_window,
        }
    }

    pub fn add_snapshot(&mut self, snapshot: PerformanceSnapshot) {
        if self.snapshots.len() >= self.max_history_size {
            self.snapshots.pop_front();
        }
        self.snapshots.push_back(snapshot);
    }

    pub fn get_recent_snapshots(&self, count: usize) -> Vec<&PerformanceSnapshot> {
        let start_idx = if self.snapshots.len() > count {
            self.snapshots.len() - count
        } else {
            0
        };
        
        self.snapshots.range(start_idx..).collect()
    }

    pub fn get_analysis_window(&self) -> Vec<&PerformanceSnapshot> {
        self.get_recent_snapshots(self.analysis_window_size)
    }
} 