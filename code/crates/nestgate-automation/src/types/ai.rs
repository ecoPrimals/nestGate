//! AI-related types for machine learning and AI integration

use serde::{Deserialize, Serialize};

/// AI integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub enable_tier_prediction: bool,
    pub enable_optimization: bool,
    pub enable_anomaly_detection: bool,
    pub model_cache_size: usize,
    pub prediction_timeout_ms: u64,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            enable_tier_prediction: true,
            enable_optimization: true,
            enable_anomaly_detection: false,
            model_cache_size: 100,
            prediction_timeout_ms: 5000,
        }
    }
}

/// AI model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub model_id: String,
    pub model_type: ModelType,
    pub version: String,
    pub accuracy: f64,
    pub last_trained: Option<u64>,
}

/// Types of AI models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    TierPredictor,
    AccessPatternAnalyzer,
    PerformanceOptimizer,
    AnomalyDetector,
}

/// AI training metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingMetrics {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub training_samples: u64,
    pub validation_samples: u64,
} 