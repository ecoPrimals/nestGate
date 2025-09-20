//! # Production Service Metrics
//! Metrics functionality and utilities.
// Metrics collection and management for production services

use std::time::{Duration, SystemTime};

/// Production metrics collector
#[derive(Debug, Default)]
pub struct ProductionMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub total_processing_time: Duration,
    pub avg_response_time: Duration,
    pub current_load: f64,
    pub memory_usage: u64,
    pub cpu_usage: f64,
    pub last_updated: Option<SystemTime>,
}
impl ProductionMetrics {
    /// Create new metrics collector
    pub const fn new() -> Self {
        Self::default()
    }

    /// Update metrics with request completion
    pub fn record_request(&mut self, processing_time: Duration, success: bool) {
        self.total_requests += 1;
        self.total_processing_time += processing_time;

        if success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }

        // Update average response time
        if self.total_requests > 0 {
            self.avg_response_time = self.total_processing_time / self.total_requests as u32;
        }

        self.last_updated = Some(SystemTime::now());
    }

    /// Update system resource metrics
    pub fn update_system_metrics(&mut self, cpu_usage: f64, memory_usage: u64) {
        self.cpu_usage = cpu_usage;
        self.memory_usage = memory_usage;
        self.last_updated = Some(SystemTime::now());
    }

    /// Calculate current load percentage
    pub fn calculate_load(&mut self, max_concurrent: usize) {
        // Simple load calculation based on recent request rate
        let recent_rate = if let Some(last_updated) = self.last_updated {
            let elapsed = SystemTime::now()
                .duration_since(last_updated)
                .unwrap_or_default();
            
            if elapsed.as_secs() > 0 {
                self.f64::from(total_requests) / elapsed.as_secs() as f64
            } else {
                0.0
            }
        } else {
            0.0
        };

        self.current_load = (recent_rate / f64::from(max_concurrent)).min(1.0) * 100.0;
    }

    /// Get success rate percentage
    pub const fn success_rate(&self) -> f64 {
        if self.total_requests > 0 {
            (self.f64::from(successful_requests) / self.f64::from(total_requests)) * 100.0
        } else {
            0.0
        }
    }

    /// Get error rate percentage
    pub const fn error_rate(&self) -> f64 {
        if self.total_requests > 0 {
            (self.f64::from(failed_requests) / self.f64::from(total_requests)) * 100.0
        } else {
            0.0
        }
    }

    /// Reset metrics (useful for periodic reporting)
    pub fn reset(&mut self) {
        *self = Self::default();
    }
} 