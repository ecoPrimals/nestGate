/// Contains all configuration related to intelligent tier assignment
/// Extracted from unified_automation_config.rs for better maintainability
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Intelligent tier assignment settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierAssignmentSettings {
    /// Enable intelligent tier assignment
    pub enabled: bool,
    /// Minimum confidence threshold for assignments
    pub min_confidence_threshold: f64,
    /// Assignment algorithms to use
    pub algorithms: Vec<TierAssignmentAlgorithm>,
    /// Tier transition rules
    pub transition_rules: Vec<TierTransitionRule>,
    /// Manual override settings
    pub manual_overrides: TierOverrideSettings,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierAssignmentAlgorithm {
    pub name: String,
    pub algorithm_type: String,
    pub config: HashMap<String, serde_json::Value>,
    pub weight: f64,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierTransitionRule {
    pub from_tier: String,
    pub to_tier: String,
    pub conditions: Vec<TransitionCondition>,
    pub cooldown_period: Duration,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionCondition {
    pub metric: String,
    pub operator: String, // "gt", "lt", "eq", "gte", "lte"
    pub value: f64,
    pub duration: Duration,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierOverrideSettings {
    pub allow_manual_overrides: bool,
    pub override_timeout: Duration,
    pub authorized_users: Vec<String>,
    pub audit_overrides: bool,
}
// Factory methods for different environments
impl TierAssignmentSettings {
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: false, // Disabled for development
            min_confidence_threshold: 0.5,
            algorithms: vec![
                TierAssignmentAlgorithm {
                    name: "basic_load".to_string(),
                    algorithm_type: "load_based".to_string(),
                    config: [("weight".to_string(), serde_json::json!(1.0))]
                        .iter().cloned().collect(),
                    weight: 1.0,
                },
            ],
            transition_rules: Vec::new(),
            manual_overrides: TierOverrideSettings {
                allow_manual_overrides: true,
                override_timeout: Duration::from_secs(3600),
                authorized_users: vec!["developer".to_string()],
                audit_overrides: false }
        }
    }

    pub fn production() -> Self {
        Self {
            enabled: true,
            min_confidence_threshold: 0.8,
            algorithms: vec![
                TierAssignmentAlgorithm {
                    name: "ml_predictor".to_string(),
                    algorithm_type: "machine_learning".to_string(),
                    config: [
                        ("model".to_string(), serde_json::json!("production_model_v2")),
                        ("features".to_string(), serde_json::json!(["load", "latency", "throughput"])),
                    ].iter().cloned().collect(),
                    weight: 0.7,
                },
                TierAssignmentAlgorithm {
                    name: "resource_monitor".to_string(),
                    algorithm_type: "resource_based".to_string(),
                    config: [("cpu_weight".to_string(), serde_json::json!(0.4))]
                        .iter().cloned().collect(),
                    weight: 0.3 }
            ],
            transition_rules: vec![
                TierTransitionRule {
                    from_tier: "tier1".to_string(),
                    to_tier: "tier2".to_string(),
                    conditions: vec![
                        TransitionCondition {
                            metric: "cpu_usage".to_string(),
                            operator: "gt".to_string(),
                            value: 80.0,
                            duration: Duration::from_secs(300),
                        }
                    ],
                    cooldown_period: Duration::from_secs(600),
                }
            ],
            manual_overrides: TierOverrideSettings {
                allow_manual_overrides: true,
                override_timeout: Duration::from_secs(1800),
                authorized_users: vec!["admin".to_string(), "ops".to_string()],
                audit_overrides: true,
            }
        }
    }

    pub fn performance_focused() -> Self {
        Self {
            enabled: true,
            min_confidence_threshold: 0.9, // Higher threshold for performance
            algorithms: vec![
                TierAssignmentAlgorithm {
                    name: "performance_optimizer".to_string(),
                    algorithm_type: "performance_ml".to_string(),
                    config: [
                        ("optimization_target".to_string(), serde_json::json!("latency")),
                        ("aggressive_scaling".to_string(), serde_json::json!(true)),
                    ].iter().cloned().collect(),
                    weight: 0.8,
                },
                TierAssignmentAlgorithm {
                    name: "predictive_scaler".to_string(),
                    algorithm_type: "predictive".to_string(),
                    config: [("horizon".to_string(), serde_json::json!(300))]
                        .iter().cloned().collect(),
                    weight: 0.2 }
            ],
            transition_rules: vec![
                TierTransitionRule {
                    from_tier: "tier1".to_string(),
                    to_tier: "tier2".to_string(),
                    conditions: vec![
                        TransitionCondition {
                            metric: "response_time".to_string(),
                            operator: "gt".to_string(),
                            value: 100.0, // 100ms
                            duration: Duration::from_secs(60), // Faster transitions
                        }
                    ],
                    cooldown_period: Duration::from_secs(120),
                }
            ],
            manual_overrides: TierOverrideSettings {
                allow_manual_overrides: false, // No manual overrides for performance mode
                override_timeout: Duration::from_secs(0),
                authorized_users: Vec::new(),
                audit_overrides: true,
            }
        }
    }

    pub fn reliability_focused() -> Self {
        Self {
            enabled: true,
            min_confidence_threshold: 0.95, // Very conservative
            algorithms: vec![
                TierAssignmentAlgorithm {
                    name: "stability_analyzer".to_string(),
                    algorithm_type: "stability_based".to_string(),
                    config: [
                        ("stability_weight".to_string(), serde_json::json!(0.8)),
                        ("error_rate_threshold".to_string(), serde_json::json!(0.01)),
                    ].iter().cloned().collect(),
                    weight: 0.6,
                },
                TierAssignmentAlgorithm {
                    name: "capacity_planner".to_string(),
                    algorithm_type: "capacity_based".to_string(),
                    config: [("buffer_percentage".to_string(), serde_json::json!(30))]
                        .iter().cloned().collect(),
                    weight: 0.4 }
            ],
            transition_rules: vec![
                TierTransitionRule {
                    from_tier: "tier1".to_string(),
                    to_tier: "tier2".to_string(),
                    conditions: vec![
                        TransitionCondition {
                            metric: "cpu_usage".to_string(),
                            operator: "gt".to_string(),
                            value: 60.0, // Conservative threshold
                            duration: Duration::from_secs(900), // Longer observation period
                        }
                        TransitionCondition {
                            metric: "error_rate".to_string(),
                            operator: "lt".to_string(),
                            value: 0.01, // Low error rate required
                            duration: Duration::from_secs(900),
                        }
                    ],
                    cooldown_period: Duration::from_secs(1800), // Longer cooldown
                }
            ],
            manual_overrides: TierOverrideSettings {
                allow_manual_overrides: true,
                override_timeout: Duration::from_secs(3600),
                authorized_users: vec!["senior_admin".to_string()],
                audit_overrides: true,
            }
        }
    }

    #[must_use]
    pub fn testing() -> Self {
        Self {
            enabled: false,
            min_confidence_threshold: 0.1, // Very low for testing
            algorithms: vec![
                TierAssignmentAlgorithm {
                    name: "test_algorithm".to_string(),
                    algorithm_type: "test".to_string(),
                    config: HashMap::new(),
                    weight: 1.0,
                },
            ],
            transition_rules: Vec::new(),
            manual_overrides: TierOverrideSettings {
                allow_manual_overrides: true,
                override_timeout: Duration::from_secs(60),
                authorized_users: vec!["test_user".to_string()],
                audit_overrides: false }
        }
    }
}

impl Default for TierAssignmentSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            min_confidence_threshold: 0.7,
            algorithms: Vec::new(),
            transition_rules: Vec::new(),
            manual_overrides: TierOverrideSettings::default(),
        }
    }
}

impl Default for TierOverrideSettings {
    fn default() -> Self {
        Self {
            allow_manual_overrides: false,
            override_timeout: Duration::from_secs(3600),
            authorized_users: Vec::new(),
            audit_overrides: true,
        }
    }
}
