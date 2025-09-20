//! Smart refactoring of 1,265-line monolith into focused, maintainable modules.
//! Each module handles a specific automation concern with clear boundaries.
use nestgate_core::config::defaults::NetworkPortDefaults as StandardDomainConfig;
use nestgate_core::smart_abstractions::prelude::SmartDefault;
use serde::{Deserialize, Serialize};

// Import all module components
pub mod lifecycle;
pub mod ml_prediction;
pub mod optimization;
pub mod scheduling;
pub mod workflows;

// Re-export all public types for seamless migration
pub use lifecycle::*;
pub use ml_prediction::*;
pub use optimization::*;
pub use scheduling::*;
pub use workflows::*;

//! **UNIFIED AUTOMATION EXTENSIONS**
//! Main configuration structure that composes all specialized modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAutomationExtensions {
    /// Lifecycle management settings
    pub lifecycle: LifecycleSettings,
    /// ML prediction configuration
    pub ml_prediction: MlPredictionSettings,
    /// Workflow engine settings
    pub workflows: WorkflowSettings,
    /// Optimization parameters
    pub optimization: OptimizationSettings,
    /// Scheduling configuration
    pub scheduling: SchedulingSettings,
}
//! **UNIFIED AUTOMATION CONFIGURATION**
//! The main configuration type following StandardDomainConfig pattern
//! CANONICAL MODERNIZATION: Simplified type alias without type parameters
pub type UnifiedAutomationConfig = StandardDomainConfig;
impl SmartDefault for UnifiedAutomationExtensions {
    fn smart_default() -> Self { Self {
            lifecycle: LifecycleSettings::smart_default(),
            ml_prediction: MlPredictionSettings::smart_default(),
            workflows: WorkflowSettings::smart_default(),
            optimization: OptimizationSettings::smart_default(),
            scheduling: SchedulingSettings::smart_default(),
         }
}

impl Default for UnifiedAutomationExtensions {
    fn default() -> Self {
        Self::smart_default()
    }
}
