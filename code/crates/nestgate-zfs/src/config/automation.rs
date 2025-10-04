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
    /// Enable AI-powered optimizations
    pub ai_enabled: bool,
    /// Tier prediction model configuration
    pub model_config: String,
    /// Performance monitoring interval
    pub monitoring_interval_seconds: u64,
    /// AI confidence threshold for recommendations
    pub confidence_threshold: f64,
}
impl Default for DatasetAutomationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            scan_interval_seconds: 300, // 5 minutes
            learning_period_days: 7,
            default_policy: "balanced_performance".to_string(),
            ai_settings: AiAutomationSettings {
                ai_enabled: false,
                model_config: "default".to_string(),
                monitoring_interval_seconds: 300, // 5 minutes
                confidence_threshold: 0.8,
            },
        }
    }
}

impl Default for AiAutomationSettings {
    fn default() -> Self {
        Self {
            ai_enabled: false,
            model_config: "default".to_string(),
            monitoring_interval_seconds: 300, // 5 minutes
            confidence_threshold: 0.8,
        }
    }
}

impl AiAutomationSettings {
    /// Create production-optimized AI automation settings
    pub fn production() -> Self {
        Self {
            ai_enabled: true,
            model_config: "production-optimized".to_string(),
            monitoring_interval_seconds: 120, // 2 minutes
            confidence_threshold: 0.9,
        }
    }
}
