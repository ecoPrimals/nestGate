// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::{
    actions::ActionsConfig, ai_automation::AiAutomationConfig, analysis::AnalysisConfig,
    lifecycle::LifecycleConfig, ml_prediction::MlPredictionConfig,
    optimization::OptimizationConfig, prediction::PredictionConfig, scheduling::SchedulingConfig,
    triggers::TriggersConfig, workflows::WorkflowsConfig,
};

/// **CANONICAL AUTOMATION CONFIGURATION**
///
/// Consolidates all automation configuration patterns into a single comprehensive struct.
///
/// **Replaces**:
/// - `AutomationConfig` (nestgate-automation/src/types/config.rs)
/// - `AutomationConfig` (`canonical_primary/supporting_types.rs`)
/// - `AutomationConfig` (`canonical_primary/detailed_configs.rs`)
/// - `AutomationDomainConfig` (`unified_final_config/domain_configs/automation.rs`)
/// - `UnifiedAutomationExtensions` (`unified_automation_config/mod.rs`)
/// - `DatasetAutomationConfig` (nestgate-zfs/src/config/automation.rs - kept separate in storage domain)
///
/// Configuration for Automation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationConfig {
    // ==================== CORE SETTINGS ====================
    /// Enable automation system
    pub enabled: bool,

    /// Maximum concurrent automation tasks
    pub max_concurrent_tasks: usize,

    /// Task execution timeout
    pub task_timeout: Duration,

    // ==================== ANALYSIS & MONITORING ====================
    /// Analysis configuration
    pub analysis: AnalysisConfig,

    // ==================== PREDICTION & ML ====================
    /// Prediction configuration
    pub prediction: PredictionConfig,

    /// ML prediction settings
    pub ml_prediction: MlPredictionConfig,

    /// AI automation settings
    pub ai_settings: AiAutomationConfig,

    // ==================== LIFECYCLE & OPTIMIZATION ====================
    /// Lifecycle management configuration
    pub lifecycle: LifecycleConfig,

    /// Optimization settings
    pub optimization: OptimizationConfig,

    // ==================== WORKFLOWS & SCHEDULING ====================
    /// Workflow engine settings
    pub workflows: WorkflowsConfig,

    /// Scheduling configuration
    pub scheduling: SchedulingConfig,

    // ==================== TRIGGERS & ACTIONS ====================
    /// Event triggers configuration
    pub triggers: TriggersConfig,

    /// Automated actions configuration
    pub actions: ActionsConfig,

    // ==================== ADVANCED SETTINGS ====================
    /// Optimization interval in hours
    pub optimization_interval_hours: u32,

    /// Prediction cache TTL in hours
    pub prediction_cache_ttl_hours: u32,

    /// Enable intelligent tier assignment
    pub enable_intelligent_tier_assignment: bool,

    /// Enable automatic optimization
    pub enable_automatic_optimization: bool,

    /// Minimum confidence threshold for predictions
    pub min_confidence_threshold: f64,

    /// Orchestration endpoint
    pub orchestration_endpoint: Option<String>,

    /// Custom automation settings (extensibility)
    pub automation_settings: HashMap<String, serde_json::Value>,
}

impl AutomationConfig {
    /// Create a development-optimized configuration
    #[must_use]
    pub fn development() -> Self {
        Self {
            // Core settings
            enabled: true,
            max_concurrent_tasks: 10,
            task_timeout: Duration::from_secs(300),

            // Analysis & Monitoring
            analysis: AnalysisConfig::development(),

            // Prediction & ML
            prediction: PredictionConfig::development(),
            ml_prediction: MlPredictionConfig::development(),
            ai_settings: AiAutomationConfig::development(),

            // Lifecycle & Optimization
            lifecycle: LifecycleConfig::development(),
            optimization: OptimizationConfig::development(),

            // Workflows & Scheduling
            workflows: WorkflowsConfig::development(),
            scheduling: SchedulingConfig::development(),

            // Triggers & Actions
            triggers: TriggersConfig::development(),
            actions: ActionsConfig::development(),

            // Advanced settings
            optimization_interval_hours: 1, // Frequent for dev
            prediction_cache_ttl_hours: 2,
            enable_intelligent_tier_assignment: true,
            enable_automatic_optimization: false, // Disable in dev
            min_confidence_threshold: 0.5,        // Lower for dev
            orchestration_endpoint: None,
            automation_settings: HashMap::new(),
        }
    }

    /// Create a production-hardened configuration
    #[must_use]
    pub fn production() -> Self {
        Self {
            // Core settings
            enabled: true,
            max_concurrent_tasks: 50,
            task_timeout: Duration::from_secs(600),

            // Analysis & Monitoring
            analysis: AnalysisConfig::production(),

            // Prediction & ML
            prediction: PredictionConfig::production(),
            ml_prediction: MlPredictionConfig::production(),
            ai_settings: AiAutomationConfig::production(),

            // Lifecycle & Optimization
            lifecycle: LifecycleConfig::production(),
            optimization: OptimizationConfig::production(),

            // Workflows & Scheduling
            workflows: WorkflowsConfig::production(),
            scheduling: SchedulingConfig::production(),

            // Triggers & Actions
            triggers: TriggersConfig::production(),
            actions: ActionsConfig::production(),

            // Advanced settings
            optimization_interval_hours: 6, // Every 6 hours
            prediction_cache_ttl_hours: 24,
            enable_intelligent_tier_assignment: true,
            enable_automatic_optimization: true,
            min_confidence_threshold: 0.8, // Higher for production
            orchestration_endpoint: None,  // Set via environment
            automation_settings: HashMap::new(),
        }
    }
}

impl Default for AutomationConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development()
    }
}
