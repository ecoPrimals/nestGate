// **STORAGE ANOMALY DETECTION**
//! Anomaly Detection functionality and utilities.
// Real-time anomaly detection for storage systems.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Storage anomaly detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAnomaly {
    pub anomaly_id: String,
    pub timestamp: SystemTime,
    pub anomaly_type: AnomalyType,
    pub severity: AnomalySeverity,
    pub description: String,
    pub affected_resources: Vec<String>,
    pub confidence_score: f64,
    pub recommended_actions: Vec<String>,
}
/// Types of storage anomalies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyType {
    PerformanceDegradation,
    UnusualCapacityGrowth,
    UnexpectedAccessPatterns,
    ErrorRateSpike,
    LatencyAnomaly,
    ThroughputAnomaly,
    SecurityAnomaly,
    ConfigurationAnomaly,
    HardwareAnomaly,
    NetworkAnomaly,
}
/// Severity levels for anomalies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalySeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
} 