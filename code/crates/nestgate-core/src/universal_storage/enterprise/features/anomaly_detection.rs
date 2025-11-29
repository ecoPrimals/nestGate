// **STORAGE ANOMALY DETECTION**
//! Anomaly Detection functionality and utilities.
// Real-time anomaly detection for storage systems.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Storage anomaly detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storageanomaly
pub struct StorageAnomaly {
    /// Anomaly identifier
    pub anomaly_id: String,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Anomaly Type
    pub anomaly_type: AnomalyType,
    /// Severity
    pub severity: AnomalySeverity,
    /// Human-readable description
    pub description: String,
    /// Affected Resources
    pub affected_resources: Vec<String>,
    /// Confidence Score
    pub confidence_score: f64,
    /// Recommended Actions
    pub recommended_actions: Vec<String>,
}
/// Types of storage anomalies
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Anomaly
pub enum AnomalyType {
    /// Performancedegradation
    PerformanceDegradation,
    /// Unusualcapacitygrowth
    UnusualCapacityGrowth,
    /// Unexpectedaccesspatterns
    UnexpectedAccessPatterns,
    /// Errorratespike
    ErrorRateSpike,
    /// Latencyanomaly
    LatencyAnomaly,
    /// Throughputanomaly
    ThroughputAnomaly,
    /// Securityanomaly
    SecurityAnomaly,
    /// Configurationanomaly
    ConfigurationAnomaly,
    /// Hardwareanomaly
    HardwareAnomaly,
    /// Networkanomaly
    NetworkAnomaly,
}
/// Severity levels for anomalies
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Anomalyseverity
pub enum AnomalySeverity {
    /// Critical
    Critical,
    /// High
    High,
    /// Medium
    Medium,
    /// Low
    Low,
    /// Info
    Info,
} 