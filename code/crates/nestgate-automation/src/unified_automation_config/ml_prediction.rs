/// **ML PREDICTION MODULE**
/// Machine learning and prediction configuration - extracted from monolithic config
use serde::{Deserialize, Serialize};
use std::time::Duration;
/// ML prediction settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Mlpredictionsettings
pub struct MlPredictionSettings {
    /// Enable ML predictions
    pub enabled: bool,
    /// Model configuration
    pub model_path: String,
    /// Prediction interval
    pub prediction_interval: Duration,
    /// Confidence threshold
    pub confidence_threshold: f64,
}
impl SmartDefault for MlPredictionSettings {
    /// Smart Default
    fn smart_default() -> Self {
        Self {
            enabled: false,
            model_path: "/etc/nestgate/models/default.onnx".to_string(),
            prediction_interval: Duration::from_secs(300),
            confidence_threshold: 0.8,
        }
    }
}

impl Default for MlPredictionSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self::smart_default()
    }
}
