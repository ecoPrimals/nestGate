///
/// This module handles statistics collection, metrics tracking, and performance
/// monitoring for the zero-cost orchestration client.
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

/// Orchestration operation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostOrchestrationStats {
    /// Total service registrations performed
    pub total_registrations: u64,
    /// Total service discoveries performed
    pub total_discoveries: u64,
    /// Total port allocations performed
    pub total_port_allocations: u64,
    /// Total health checks performed
    pub total_health_checks: u64,
    /// Total failed operations
    pub failed_operations: u64,
    /// Total response time in milliseconds
    pub total_response_time_ms: f64,
    /// Last health check timestamp
    pub last_health_check: Option<SystemTime>,
    /// Average response time
    pub average_response_time_ms: f64,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Operations per second
    pub ops_per_second: f64,
    /// Peak operations per second
    pub peak_ops_per_second: f64,
    /// Total successful operations
    pub successful_operations: u64,
    /// Statistics collection start time
    pub stats_start_time: SystemTime,
}

impl Default for ZeroCostOrchestrationStats {
    fn default() -> Self {
        Self::new()
    }
}

impl ZeroCostOrchestrationStats {
    /// Create new statistics instance
    pub fn new() -> Self {
        Self {
            stats_start_time: SystemTime::now(),
            ..Default::default()
        }
    }

    /// Calculate derived metrics
    pub fn calculate_derived_metrics(&mut self) {
        let total_ops = self.successful_operations + self.failed_operations;

        // Calculate success rate
        if total_ops > 0 {
            self.success_rate = self.successful_operations as f64 / total_ops as f64;
        }

        // Calculate average response time
        if self.successful_operations > 0 {
            self.average_response_time_ms =
                self.total_response_time_ms / self.successful_operations as f64;
        }

        // Calculate operations per second
        if let Ok(elapsed) = self.stats_start_time.elapsed() {
            let seconds = elapsed.as_secs_f64();
            if seconds > 0.0 {
                self.ops_per_second = total_ops as f64 / seconds;
                if self.ops_per_second > self.peak_ops_per_second {
                    self.peak_ops_per_second = self.ops_per_second;
                }
            }
        }
    }

    /// Reset all statistics
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Get uptime duration
    pub fn uptime(&self) -> Duration {
        self.stats_start_time.elapsed().unwrap_or_default()
    }
}

/// Statistics collector for orchestration operations
pub struct StatsCollector {
    stats: Arc<RwLock<ZeroCostOrchestrationStats>>,
}

impl StatsCollector {
    /// Create new stats collector
    pub fn new(stats: Arc<RwLock<ZeroCostOrchestrationStats>>) -> Self {
        Self { stats }
    }

    /// Record a successful service registration
    pub fn record_registration_success(&self, response_time_ms: f64) {
        if let Ok(mut stats) = self.stats.write() {
            stats.total_registrations += 1;
            stats.successful_operations += 1;
            stats.total_response_time_ms += response_time_ms;
            stats.calculate_derived_metrics();
        }
    }

    /// Record a failed service registration
    pub fn record_registration_failure(&self) {
        if let Ok(mut stats) = self.stats.write() {
            stats.total_registrations += 1;
            stats.failed_operations += 1;
            stats.calculate_derived_metrics();
        }
    }

    /// Record a successful service discovery
    pub fn record_discovery_success(&self, response_time_ms: f64) {
        if let Ok(mut stats) = self.stats.write() {
            stats.total_discoveries += 1;
            stats.successful_operations += 1;
            stats.total_response_time_ms += response_time_ms;
            stats.calculate_derived_metrics();
        }
    }

    /// Record a failed service discovery
    pub fn record_discovery_failure(&self) {
        if let Ok(mut stats) = self.stats.write() {
            stats.total_discoveries += 1;
            stats.failed_operations += 1;
            stats.calculate_derived_metrics();
        }
    }

    /// Record a successful port allocation
    pub fn record_port_allocation_success(&self, response_time_ms: f64) {
        if let Ok(mut stats) = self.stats.write() {
            stats.total_port_allocations += 1;
            stats.successful_operations += 1;
            stats.total_response_time_ms += response_time_ms;
            stats.calculate_derived_metrics();
        }
    }

    /// Record a failed port allocation
    pub fn record_port_allocation_failure(&self) {
        if let Ok(mut stats) = self.stats.write() {
            stats.total_port_allocations += 1;
            stats.failed_operations += 1;
            stats.calculate_derived_metrics();
        }
    }

    /// Record a successful health check
    pub fn record_health_check_success(&self, response_time_ms: f64) {
        if let Ok(mut stats) = self.stats.write() {
            stats.total_health_checks += 1;
            stats.successful_operations += 1;
            stats.total_response_time_ms += response_time_ms;
            stats.last_health_check = Some(SystemTime::now());
            stats.calculate_derived_metrics();
        }
    }

    /// Record a failed health check
    pub fn record_health_check_failure(&self) {
        if let Ok(mut stats) = self.stats.write() {
            stats.total_health_checks += 1;
            stats.failed_operations += 1;
            stats.last_health_check = Some(SystemTime::now());
            stats.calculate_derived_metrics();
        }
    }

    /// Get current statistics snapshot
    pub fn get_stats(&self) -> ZeroCostOrchestrationStats {
        match self.stats.read() {
            Ok(stats) => stats.clone(),
            Err(e) => {
                tracing::error!("RwLock read poisoned - returning default stats: {:?}", e);
                // Return default stats rather than panic
                ZeroCostOrchestrationStats::default()
            }
        }
    }

    /// Reset all statistics
    pub fn reset_stats(&self) {
        if let Ok(mut stats) = self.stats.write() {
            stats.reset();
        }
    }

    /// Get performance summary
    pub fn get_performance_summary(&self) -> PerformanceSummary {
        let stats = self.get_stats();
        PerformanceSummary {
            total_operations: stats.successful_operations + stats.failed_operations,
            success_rate: stats.success_rate,
            average_response_time_ms: stats.average_response_time_ms,
            ops_per_second: stats.ops_per_second,
            peak_ops_per_second: stats.peak_ops_per_second,
            uptime: stats.uptime(),
        }
    }
}

/// Performance summary for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    /// Total operations performed
    pub total_operations: u64,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Average response time in milliseconds
    pub average_response_time_ms: f64,
    /// Current operations per second
    pub ops_per_second: f64,
    /// Peak operations per second
    pub peak_ops_per_second: f64,
    /// Total uptime
    pub uptime: Duration,
}

impl PerformanceSummary {
    /// Check if performance is healthy
    pub fn is_healthy(&self) -> bool {
        self.success_rate >= 0.95 && self.average_response_time_ms < 1000.0
    }

    /// Get performance grade (A-F)
    pub fn performance_grade(&self) -> char {
        if self.success_rate >= 0.99 && self.average_response_time_ms < 100.0 {
            'A'
        } else if self.success_rate >= 0.95 && self.average_response_time_ms < 500.0 {
            'B'
        } else if self.success_rate >= 0.90 && self.average_response_time_ms < 1000.0 {
            'C'
        } else if self.success_rate >= 0.80 && self.average_response_time_ms < 2000.0 {
            'D'
        } else {
            'F'
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_calculation() {
        let mut stats = ZeroCostOrchestrationStats::new();
        stats.successful_operations = 90;
        stats.failed_operations = 10;
        stats.total_response_time_ms = 4500.0;

        stats.calculate_derived_metrics();

        assert_eq!(stats.success_rate, 0.9);
        assert_eq!(stats.average_response_time_ms, 50.0);
    }

    #[test]
    fn test_performance_summary() {
        let summary = PerformanceSummary {
            total_operations: 1000,
            success_rate: 0.99,
            average_response_time_ms: 50.0,
            ops_per_second: 100.0,
            peak_ops_per_second: 150.0,
            uptime: Duration::from_secs(3600),
        };

        assert!(summary.is_healthy());
        assert_eq!(summary.performance_grade(), 'A');
    }
}
