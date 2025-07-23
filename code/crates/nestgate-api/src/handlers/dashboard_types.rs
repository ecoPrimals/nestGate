//! Dashboard Type Definitions
//!
//! Core types and structures used by the performance dashboard system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{ SystemTime};
use tokio::sync::RwLock;
use std::time::Duration;

/// Time range configuration for performance queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: SystemTime,
    pub end: SystemTime,
    pub granularity: Duration,
}

impl TimeRange {
    /// Create a new time range
    pub fn new(start: SystemTime, end: SystemTime, granularity: Duration) -> Self {
        Self {
            start,
            end,
            granularity,
        }
    }

    /// Create a time range for the last N hours
    pub fn last_hours(hours: u64) -> Self {
        let end = SystemTime::now();
        let start = end - Duration::from_secs(hours * 3600);
        Self {
            start,
            end,
            granularity: Duration::from_secs(300), // 5 minutes
        }
    }

    /// Create a time range for the last N days
    pub fn last_days(days: u64) -> Self {
        let end = SystemTime::now();
        let start = end - Duration::from_secs(days * 24 * 3600);
        Self {
            start,
            end,
            granularity: Duration::from_secs(3600), // 1 hour
        }
    }

    /// Get duration of this time range
    pub fn duration(&self) -> Duration {
        self.end.duration_since(self.start).unwrap_or(Duration::ZERO)
    }

    /// Check if this time range is valid
    pub fn is_valid(&self) -> bool {
        self.start < self.end && !self.granularity.is_zero()
    }

    /// Get the number of data points in this range
    pub fn data_points(&self) -> usize {
        if !self.is_valid() {
            return 0;
        }
        let duration = self.duration();
        (duration.as_secs() / self.granularity.as_secs()).max(1) as usize
    }

    /// Split time range into intervals based on granularity
    pub fn intervals(&self) -> Vec<(SystemTime, SystemTime)> {
        let mut intervals = Vec::new();
        let mut current = self.start;
        
        while current < self.end {
            let next = current + self.granularity;
            let interval_end = if next > self.end { self.end } else { next };
            intervals.push((current, interval_end));
            current = next;
        }
        
        intervals
    }
}

/// Dashboard state for real-time updates
#[derive(Debug)]
pub struct DashboardState {
    /// Current active connections
    pub active_connections: u32,
    /// Last update timestamp
    pub last_update: SystemTime,
    /// Cached metrics for quick access
    pub cached_metrics: HashMap<String, serde_json::Value>,
    /// Performance alerts
    pub active_alerts: Vec<PerformanceAlert>,
}

impl DashboardState {
    pub fn new() -> Self {
        Self {
            active_connections: 0,
            last_update: SystemTime::now(),
            cached_metrics: HashMap::new(),
            active_alerts: Vec::new(),
        }
    }

    pub fn update_metrics(&mut self, key: String, value: serde_json::Value) {
        self.cached_metrics.insert(key, value);
        self.last_update = SystemTime::now();
    }

    pub fn add_alert(&mut self, alert: PerformanceAlert) {
        self.active_alerts.push(alert);
    }

    pub fn clear_resolved_alerts(&mut self) {
        self.active_alerts.retain(|alert| !alert.resolved);
    }
}

impl Default for DashboardState {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance alert definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub id: String,
    pub severity: AlertSeverity,
    pub title: String,
    pub message: String,
    pub timestamp: SystemTime,
    pub resolved: bool,
    pub metric_name: String,
    pub current_value: f64,
    pub threshold: f64,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

/// Dashboard event for real-time updates
#[derive(Debug, Clone, Serialize)]
pub struct DashboardEvent {
    pub event_type: String,
    pub data: serde_json::Value,
    pub timestamp: SystemTime,
}

/// Dashboard configuration
#[derive(Debug, Clone)]
pub struct DashboardConfig {
    /// Enable real-time updates
    pub enable_real_time: bool,
    /// Update interval for real-time data
    pub update_interval: Duration,
    /// Maximum history to keep in memory
    pub max_history_points: usize,
    /// Enable predictive analytics
    pub enable_predictions: bool,
    /// Alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        let mut alert_thresholds = HashMap::new();
        alert_thresholds.insert("cpu_usage".to_string(), 80.0);
        alert_thresholds.insert("memory_usage".to_string(), 85.0);
        alert_thresholds.insert("disk_usage".to_string(), 90.0);
        alert_thresholds.insert("latency_ms".to_string(), 1000.0);
        alert_thresholds.insert("error_rate".to_string(), 5.0);

        Self {
            enable_real_time: true,
            update_interval: Duration::from_secs(1),
            max_history_points: 1000,
            enable_predictions: true,
            alert_thresholds,
        }
    }
} 