//! **CANONICAL AUTOMATION CONFIGURATION MODULE**
//!
//! The single source of truth for all automation configuration across NestGate.
//! Consolidates AutomationConfig, UnifiedAutomationConfig, and automation domain configs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **CANONICAL AUTOMATION CONFIGURATION**
///
/// Consolidates all automation configuration patterns into a single comprehensive struct.
///
/// **Replaces**:
/// - `AutomationConfig` (nestgate-automation/src/types/config.rs)
/// - `AutomationConfig` (canonical_primary/supporting_types.rs)
/// - `AutomationConfig` (canonical_primary/detailed_configs.rs)
/// - `AutomationDomainConfig` (unified_final_config/domain_configs/automation.rs)
/// - `UnifiedAutomationExtensions` (unified_automation_config/mod.rs)
/// - `DatasetAutomationConfig` (nestgate-zfs/src/config/automation.rs - kept separate in storage domain)
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

/// **ANALYSIS CONFIGURATION**
///
/// Configuration for automated file and system analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// Enable analysis
    pub enabled: bool,

    /// Scan interval
    pub scan_interval: Duration,

    /// Maximum file size to analyze (bytes)
    pub max_file_size: u64,

    /// File extensions to include in analysis
    pub include_extensions: Vec<String>,

    /// File extensions to exclude from analysis
    pub exclude_extensions: Vec<String>,

    /// Enable deep content analysis
    pub deep_analysis_enabled: bool,

    /// Parallel analysis workers
    pub parallel_workers: usize,
}

/// **PREDICTION CONFIGURATION**
///
/// Configuration for predictive analytics and forecasting.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// **ML PREDICTION CONFIGURATION**
///
/// Machine learning-specific prediction settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// **AI AUTOMATION CONFIGURATION**
///
/// AI-powered automation settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// **LIFECYCLE CONFIGURATION**
///
/// Data lifecycle management settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleConfig {
    /// Enable lifecycle management
    pub enabled: bool,

    /// Hot tier retention in days
    pub hot_retention_days: u32,

    /// Warm tier retention in days
    pub warm_retention_days: u32,

    /// Cold tier retention in days
    pub cold_retention_days: u32,

    /// Archive retention in days
    pub archive_retention_days: u32,

    /// Enable automatic tiering
    pub auto_tiering: bool,

    /// Tier transition rules
    pub transition_rules: Vec<String>,
}

/// **OPTIMIZATION CONFIGURATION**
///
/// System and resource optimization settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Enable optimization
    pub enabled: bool,

    /// Optimization frequency in hours
    pub frequency_hours: u32,

    /// Enable storage optimization
    pub storage_optimization: bool,

    /// Enable performance optimization
    pub performance_optimization: bool,

    /// Enable resource balancing
    pub resource_balancing: bool,

    /// Optimization target (e.g., "performance", "efficiency", "balance")
    pub target: String,

    /// Optimization aggressiveness (0-100)
    pub aggressiveness: u8,
}

/// **WORKFLOWS CONFIGURATION**
///
/// Automated workflow engine settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowsConfig {
    /// Enable workflows
    pub enabled: bool,

    /// Maximum concurrent workflows
    pub max_concurrent_workflows: usize,

    /// Workflow timeout
    pub workflow_timeout: Duration,

    /// Enable workflow scheduling
    pub scheduling_enabled: bool,

    /// Workflow definitions directory
    pub definitions_dir: String,

    /// Enable workflow versioning
    pub versioning_enabled: bool,
}

/// **SCHEDULING CONFIGURATION**
///
/// Task scheduling and execution settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingConfig {
    /// Enable scheduling
    pub enabled: bool,

    /// Schedule check interval
    pub check_interval: Duration,

    /// Enable cron-style scheduling
    pub cron_enabled: bool,

    /// Maintenance windows (cron format)
    pub maintenance_windows: Vec<String>,

    /// Enable distributed scheduling
    pub distributed: bool,

    /// Maximum scheduled tasks
    pub max_scheduled_tasks: usize,
}

/// **TRIGGERS CONFIGURATION**
///
/// Event trigger settings for automation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggersConfig {
    /// Enable triggers
    pub enabled: bool,

    /// File system triggers enabled
    pub filesystem_triggers: bool,

    /// Performance triggers enabled
    pub performance_triggers: bool,

    /// Time-based triggers enabled
    pub time_triggers: bool,

    /// Custom trigger definitions
    pub custom_triggers: HashMap<String, serde_json::Value>,

    /// Trigger evaluation interval
    pub evaluation_interval: Duration,
}

/// **ACTIONS CONFIGURATION**
///
/// Automated action execution settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionsConfig {
    /// Enable actions
    pub enabled: bool,

    /// Enable storage actions (move, copy, delete)
    pub storage_actions: bool,

    /// Enable notification actions
    pub notification_actions: bool,

    /// Enable script execution
    pub script_execution: bool,

    /// Maximum action retries
    pub max_retries: u32,

    /// Action timeout
    pub action_timeout: Duration,

    /// Allowed action types
    pub allowed_actions: Vec<String>,
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
    fn default() -> Self {
        Self::development()
    }
}

// ==================== ANALYSIS CONFIG IMPLEMENTATIONS ====================

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self::development()
    }
}

impl AnalysisConfig {
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: true,
            scan_interval: Duration::from_secs(3600), // 1 hour
            max_file_size: 1024 * 1024 * 1024,        // 1GB
            include_extensions: vec!["*".to_string()],
            exclude_extensions: vec![".tmp".to_string(), ".log".to_string()],
            deep_analysis_enabled: false,
            parallel_workers: 4,
        }
    }

    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            scan_interval: Duration::from_secs(1800), // 30 minutes
            max_file_size: 10 * 1024 * 1024 * 1024,   // 10GB
            include_extensions: vec!["*".to_string()],
            exclude_extensions: vec![".tmp".to_string(), ".log".to_string(), ".cache".to_string()],
            deep_analysis_enabled: true,
            parallel_workers: 8,
        }
    }
}

// ==================== PREDICTION CONFIG IMPLEMENTATIONS ====================

impl Default for PredictionConfig {
    fn default() -> Self {
        Self::development()
    }
}

impl PredictionConfig {
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

// ==================== ML PREDICTION CONFIG IMPLEMENTATIONS ====================

impl Default for MlPredictionConfig {
    fn default() -> Self {
        Self::development()
    }
}

impl MlPredictionConfig {
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

// ==================== AI AUTOMATION CONFIG IMPLEMENTATIONS ====================

impl Default for AiAutomationConfig {
    fn default() -> Self {
        Self::development()
    }
}

impl AiAutomationConfig {
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

// ==================== LIFECYCLE CONFIG IMPLEMENTATIONS ====================

impl Default for LifecycleConfig {
    fn default() -> Self {
        Self::development()
    }
}

impl LifecycleConfig {
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: false, // Disabled in dev
            hot_retention_days: 7,
            warm_retention_days: 30,
            cold_retention_days: 90,
            archive_retention_days: 365,
            auto_tiering: false,
            transition_rules: vec![],
        }
    }

    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            hot_retention_days: 30,
            warm_retention_days: 90,
            cold_retention_days: 365,
            archive_retention_days: 1825, // 5 years
            auto_tiering: true,
            transition_rules: vec![
                "hot_to_warm: age > 30d".to_string(),
                "warm_to_cold: age > 90d".to_string(),
                "cold_to_archive: age > 365d".to_string(),
            ],
        }
    }
}

// ==================== OPTIMIZATION CONFIG IMPLEMENTATIONS ====================

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self::development()
    }
}

impl OptimizationConfig {
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: false,
            frequency_hours: 24,
            storage_optimization: false,
            performance_optimization: false,
            resource_balancing: false,
            target: "balance".to_string(),
            aggressiveness: 30,
        }
    }

    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            frequency_hours: 6,
            storage_optimization: true,
            performance_optimization: true,
            resource_balancing: true,
            target: "efficiency".to_string(),
            aggressiveness: 70,
        }
    }
}

// ==================== WORKFLOWS CONFIG IMPLEMENTATIONS ====================

impl Default for WorkflowsConfig {
    fn default() -> Self {
        Self::development()
    }
}

impl WorkflowsConfig {
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: false,
            max_concurrent_workflows: 5,
            workflow_timeout: Duration::from_secs(600),
            scheduling_enabled: false,
            definitions_dir: "./workflows".to_string(),
            versioning_enabled: false,
        }
    }

    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            max_concurrent_workflows: 20,
            workflow_timeout: Duration::from_secs(1800),
            scheduling_enabled: true,
            definitions_dir: "/etc/nestgate/workflows".to_string(),
            versioning_enabled: true,
        }
    }
}

// ==================== SCHEDULING CONFIG IMPLEMENTATIONS ====================

impl Default for SchedulingConfig {
    fn default() -> Self {
        Self::development()
    }
}

impl SchedulingConfig {
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: false,
            check_interval: Duration::from_secs(60),
            cron_enabled: false,
            maintenance_windows: vec![],
            distributed: false,
            max_scheduled_tasks: 50,
        }
    }

    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            check_interval: Duration::from_secs(30),
            cron_enabled: true,
            maintenance_windows: vec![
                "0 2 * * *".to_string(),  // 2 AM daily
                "0 14 * * 0".to_string(), // 2 PM Sundays
            ],
            distributed: true,
            max_scheduled_tasks: 500,
        }
    }
}

// ==================== TRIGGERS CONFIG IMPLEMENTATIONS ====================

impl Default for TriggersConfig {
    fn default() -> Self {
        Self::development()
    }
}

impl TriggersConfig {
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: false,
            filesystem_triggers: false,
            performance_triggers: false,
            time_triggers: false,
            custom_triggers: HashMap::new(),
            evaluation_interval: Duration::from_secs(60),
        }
    }

    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            filesystem_triggers: true,
            performance_triggers: true,
            time_triggers: true,
            custom_triggers: HashMap::new(),
            evaluation_interval: Duration::from_secs(10),
        }
    }
}

// ==================== ACTIONS CONFIG IMPLEMENTATIONS ====================

impl Default for ActionsConfig {
    fn default() -> Self {
        Self::development()
    }
}

impl ActionsConfig {
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: false,
            storage_actions: false,
            notification_actions: true,
            script_execution: false,
            max_retries: 3,
            action_timeout: Duration::from_secs(60),
            allowed_actions: vec!["notify".to_string(), "log".to_string()],
        }
    }

    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            storage_actions: true,
            notification_actions: true,
            script_execution: true,
            max_retries: 5,
            action_timeout: Duration::from_secs(300),
            allowed_actions: vec![
                "notify".to_string(),
                "log".to_string(),
                "move".to_string(),
                "copy".to_string(),
                "tier".to_string(),
                "execute_script".to_string(),
            ],
        }
    }
}
