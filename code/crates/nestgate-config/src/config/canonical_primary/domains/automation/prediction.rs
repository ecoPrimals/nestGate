// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **PREDICTION CONFIGURATION**
///
/// Configuration for predictive analytics and forecasting.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Prediction
pub struct PredictionConfig {
    /// Enable prediction
    pub enabled: bool,

    /// Prediction window in days
    pub prediction_window_days: u32,

    /// Minimum confidence threshold (0.0-1.0)
    pub min_confidence: f64,

    /// Model parameters (key-value pairs)
    pub model_params: HashMap<String, f64>,

    /// Historical data retention in days
    pub history_retention_days: u32,

    /// Enable real-time predictions
    pub realtime_enabled: bool,
}

impl Default for PredictionConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development()
    }
}

impl PredictionConfig {
    /// Creates a development-optimized configuration for predictive analytics
    ///
    /// Returns a `PredictionConfig` with shorter prediction windows and lower confidence
    /// thresholds suitable for development and testing.
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: true,
            prediction_window_days: 7,
            min_confidence: 0.5,
            model_params: HashMap::new(),
            history_retention_days: 30,
            realtime_enabled: false,
        }
    }

    /// Creates a production-hardened configuration for predictive analytics
    ///
    /// Returns a `PredictionConfig` with longer prediction windows, higher confidence
    /// thresholds, and real-time predictions enabled for production workloads.
    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            prediction_window_days: 30,
            min_confidence: 0.7,
            model_params: HashMap::new(),
            history_retention_days: 90,
            realtime_enabled: true,
        }
    }
}
