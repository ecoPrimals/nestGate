/// **WORKFLOWS MODULE**
/// Workflow engine configuration - extracted from monolithic config
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
/// Workflow settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Workflowsettings
pub struct WorkflowSettings {
    /// Enable workflows
    pub enabled: bool,
    /// Workflow definitions
    pub workflows: HashMap<String, WorkflowDefinition>,
    /// Default timeout
    pub default_timeout: Duration,
    /// Max concurrent workflows
    pub max_concurrent: u32,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
/// Workflowdefinition
pub struct WorkflowDefinition {
    /// Workflow name
    pub name: String,
    /// Workflow enabled
    pub enabled: bool,
    /// Workflow steps
    pub steps: Vec<String>,
    /// Workflow timeout
    pub timeout: Duration,
}
impl SmartDefault for WorkflowSettings {
    /// Smart Default
    fn smart_default() -> Self {
        Self {
            enabled: true,
            workflows: HashMap::default(),
            default_timeout: Duration::from_secs(300),
            max_concurrent: 10,
        }
    }
}

impl Default for WorkflowSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self::smart_default()
    }
}
