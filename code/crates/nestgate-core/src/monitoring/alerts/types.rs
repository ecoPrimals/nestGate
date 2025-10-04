// Core Alert Types and Enums

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}
impl AlertSeverity {
    /// Get numeric value for comparison
    pub fn level(&self) -> u8 {
        match self {
            Self::Info => 0,
            Self::Warning => 1,
            Self::Error => 2,
            Self::Critical => 3,
        }
    }

    /// Get color for UI display
    pub fn color(&self) -> &'static str {
        match self {
            AlertSeverity::Info => "blue",
            AlertSeverity::Warning => "yellow",
            AlertSeverity::Error => "orange",
            AlertSeverity::Critical => "red",
        }
    }
}

/// Alert rule condition types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCondition {
    /// Threshold-based condition (value > threshold, value < threshold, etc.)
    Threshold {
        metric: String,
        operator: ThresholdOperator,
        value: f64,
    },
    /// Rate-based condition (change over time)
    Rate {
        metric: String,
        operator: ThresholdOperator,
        rate: f64,
        duration: Duration,
    },
    /// Availability condition
    Availability {
        component: String,
        min_uptime_percent: f64,
        duration: Duration,
    },
    /// Custom condition with expression
    Custom { expression: String },
}
/// Threshold comparison operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThresholdOperator {
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equal,
    NotEqual,
}
/// Active alert instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Unique alert identifier
    pub id: String,
    /// Rule that triggered this alert
    pub rule_id: String,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert title/summary
    pub title: String,
    /// Detailed alert message
    pub message: String,
    /// When the alert was first triggered
    pub triggered_at: SystemTime,
    /// When the alert was last updated
    pub updated_at: SystemTime,
    /// Current alert status
    pub status: AlertStatus,
    /// Additional context data
    pub context: std::collections::HashMap<String, String>,
    /// Number of times this alert has been triggered
    pub trigger_count: u32,
}
/// Alert status states
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertStatus {
    /// Alert is active and firing
    Active,
    /// Alert condition resolved
    Resolved,
    /// Alert has been acknowledged
    Acknowledged,
    /// Alert has been suppressed
    Suppressed,
}
