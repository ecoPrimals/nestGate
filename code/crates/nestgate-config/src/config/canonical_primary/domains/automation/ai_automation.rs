// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **AI AUTOMATION CONFIGURATION**
///
/// AI-powered automation settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `AiAutomation`
pub struct AiAutomationConfig {
    /// Enable AI automation
    pub enabled: bool,

    /// Enable predictive scaling
    pub predictive_scaling: bool,

    /// Enable auto-optimization
    pub auto_optimization: bool,

    /// Enable learning mode
    pub learning_mode: bool,

    /// AI model configuration string
    pub model_config: String,

    /// Monitoring interval
    pub monitoring_interval: Duration,

    /// Confidence threshold for AI decisions
    pub confidence_threshold: f64,
}

impl Default for AiAutomationConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development()
    }
}

impl AiAutomationConfig {
    /// Creates a development-optimized configuration for AI automation
    ///
    /// Returns an `AiAutomationConfig` with AI features disabled but learning mode enabled,
    /// suitable for development and model training.
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: false,
            predictive_scaling: false,
            auto_optimization: false,
            learning_mode: true,
            model_config: "default".to_string(),
            monitoring_interval: Duration::from_secs(300),
            confidence_threshold: 0.6,
        }
    }

    /// Creates a production-hardened configuration for AI automation
    ///
    /// Returns an `AiAutomationConfig` with all AI features enabled, predictive scaling,
    /// auto-optimization, and strict confidence thresholds for production workloads.
    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            predictive_scaling: true,
            auto_optimization: true,
            learning_mode: false, // Fixed models in production
            model_config: "production".to_string(),
            monitoring_interval: Duration::from_secs(60),
            confidence_threshold: 0.85,
        }
    }
}
