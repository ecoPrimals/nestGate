// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CANONICAL AUTOMATION CONFIGURATION MODULE**
//!
//! The single source of truth for all automation configuration across `NestGate`.
//! Consolidates `AutomationConfig`, `UnifiedAutomationConfig`, and automation domain configs.

mod actions;
mod ai_automation;
mod analysis;
mod config;
mod lifecycle;
mod ml_prediction;
mod optimization;
mod prediction;
mod scheduling;
mod triggers;
mod workflows;

pub use actions::ActionsConfig;
pub use ai_automation::AiAutomationConfig;
pub use analysis::AnalysisConfig;
pub use config::AutomationConfig;
pub use lifecycle::LifecycleConfig;
pub use ml_prediction::MlPredictionConfig;
pub use optimization::OptimizationConfig;
pub use prediction::PredictionConfig;
pub use scheduling::SchedulingConfig;
pub use triggers::TriggersConfig;
pub use workflows::WorkflowsConfig;

#[cfg(test)]
mod automation_domain_round3_tests {
    use super::*;

    #[test]
    fn automation_config_default_matches_development() {
        let a = AutomationConfig::default();
        let d = AutomationConfig::development();
        assert_eq!(a.enabled, d.enabled);
        assert_eq!(a.max_concurrent_tasks, d.max_concurrent_tasks);
    }

    #[test]
    fn automation_config_production_serde_roundtrip() {
        let p = AutomationConfig::production();
        let json = serde_json::to_string(&p).expect("ser");
        let back: AutomationConfig = serde_json::from_str(&json).expect("de");
        assert!(back.enabled);
        assert!(back.min_confidence_threshold >= 0.7);
    }

    #[test]
    fn analysis_config_production_has_deep_analysis() {
        let a = AnalysisConfig::production();
        assert!(a.deep_analysis_enabled);
        assert!(a.parallel_workers >= 8);
    }
}
