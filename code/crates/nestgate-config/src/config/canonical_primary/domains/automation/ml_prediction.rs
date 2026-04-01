// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};

/// **ML PREDICTION CONFIGURATION**
///
/// Machine learning-specific prediction settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `MlPrediction`
pub struct MlPredictionConfig {
    /// Enable ML predictions
    pub enabled: bool,

    /// ML model path
    pub model_path: String,

    /// Model update interval in hours
    pub model_update_interval_hours: u32,

    /// Training data size
    pub training_data_size: usize,

    /// Enable model auto-retraining
    pub auto_retrain: bool,

    /// Prediction confidence threshold
    pub confidence_threshold: f64,
}

impl Default for MlPredictionConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development()
    }
}

impl MlPredictionConfig {
    /// Creates a development-optimized configuration for ML prediction
    ///
    /// Returns an `MlPredictionConfig` with ML features disabled by default and smaller
    /// training datasets suitable for development environments.
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: false, // Disabled in dev by default
            model_path: "/opt/nestgate/models/default".to_string(),
            model_update_interval_hours: 24,
            training_data_size: 1000,
            auto_retrain: false,
            confidence_threshold: 0.6,
        }
    }

    /// Creates a production-hardened configuration for ML prediction
    ///
    /// Returns an `MlPredictionConfig` with ML enabled, auto-retraining, larger datasets,
    /// and strict confidence thresholds for production workloads.
    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            model_path: "/opt/nestgate/models/production".to_string(),
            model_update_interval_hours: 168, // Weekly
            training_data_size: 10000,
            auto_retrain: true,
            confidence_threshold: 0.8,
        }
    }
}
