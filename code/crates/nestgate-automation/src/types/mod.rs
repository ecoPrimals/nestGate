pub mod config;
pub mod ecosystem;
pub mod optimization;
// **AUTOMATION TYPES MODULE**
// Provides type definitions for the automation system

pub mod prediction;
// Re-export commonly used types
pub use config::*;
pub use ecosystem::*;
pub use optimization::*;
pub use prediction::*;

// ==================== CANONICAL AUTOMATION CONFIGURATION RE-EXPORTS ====================

/// Re-export canonical automation configuration types from nestgate-core
pub use nestgate_core::config::canonical_primary::domains::automation::{
    ActionsConfig, AiAutomationConfig, AnalysisConfig,
    AutomationConfig as CanonicalAutomationConfig, LifecycleConfig, MlPredictionConfig,
    SchedulingConfig, TriggersConfig, WorkflowsConfig,
};

// Note: PredictionConfig and OptimizationConfig not re-exported here to avoid conflicts
// with local modules. Use the canonical ones directly from nestgate_core if needed.

// ==================== BACKWARD COMPATIBILITY ALIASES ====================

/// Backward compatibility alias for UnifiedAutomationConfig
///
/// **Migration Path**: Use `CanonicalAutomationConfig` or the specific sub-configs instead.
pub use CanonicalAutomationConfig as UnifiedAutomationConfig;
