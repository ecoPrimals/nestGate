//! Automation Configuration Module
//!
//! Configuration for dataset automation and AI-driven optimization.

use serde::{Deserialize, Serialize};

/// Dataset automation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetAutomationConfig {
    /// Enable dataset automation
    pub enabled: bool,
    /// Automation scan interval (seconds)
    pub scan_interval_seconds: u64,
    /// Learning period for new datasets (days)
    pub learning_period_days: u32,
    /// Default automation policy
    pub default_policy: String,
    /// AI integration settings
    pub ai_settings: AiAutomationSettings,
}

/// AI-powered automation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAutomationSettings {
    /// Enable AI-driven tier predictions
    pub enable_ai_predictions: bool,
    /// Confidence threshold for AI decisions
    pub ai_confidence_threshold: f64,
    /// Learning rate for access pattern prediction
    pub learning_rate: f64,
    /// Historical data window for learning (days)
    pub learning_window_days: u32,
}

impl Default for DatasetAutomationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            scan_interval_seconds: 300, // 5 minutes
            learning_period_days: 7,
            default_policy: "balanced_performance".to_string(),
            ai_settings: AiAutomationSettings {
                enable_ai_predictions: true,
                ai_confidence_threshold: 0.8,
                learning_rate: 0.1,
                learning_window_days: 30,
            },
        }
    }
}
