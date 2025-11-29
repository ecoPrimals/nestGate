// Alert Rules and Suppression Logic

use super::types::{AlertCondition, AlertSeverity};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Alert rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Alertrule
pub struct AlertRule {
    /// Unique rule identifier
    pub id: String,
    /// Human-readable rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Alert condition
    pub condition: AlertCondition,
    /// Alert severity
    pub severity: AlertSeverity,
    /// How long condition must be true before alerting
    pub duration: Duration,
    /// Notification channels to use
    pub channels: Vec<String>,
    /// Whether the rule is enabled
    pub enabled: bool,
    /// Tags for categorization
    pub tags: Vec<String>,
}
/// Alert suppression rule
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Suppressionrule
pub struct SuppressionRule {
    /// Unique suppression rule identifier
    pub id: String,
    /// Pattern to match alert names/rules
    pub pattern: String,
    /// Time window when suppression is active
    pub window: TimeWindow,
}
/// Time window specification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Timewindow
pub struct TimeWindow {
    /// Start time (24-hour format, e.g., "09:00")
    pub start: String,
    /// End time (24-hour format, e.g., "17:00")
    pub end: String,
    /// Days of week (0=Sunday, 6=Saturday)
    pub days: Vec<u8>,
}
