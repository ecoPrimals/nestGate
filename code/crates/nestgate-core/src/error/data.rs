//! **ERROR DATA STRUCTURES**
//! Error handling types and utilities.
//! Rich error context data for different error domains.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// Define the missing types here since they were removed in the refactor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HandlerType {
    Api,
    Middleware,
    Event,
    Error,
    Validation,
    Security,
    Performance,
    Lifecycle,
    ZfsOperation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HandlerPhase {
    Initialization,
    PreProcessing,
    Processing,
    PostProcessing,
    Cleanup,
    ErrorHandling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowState {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
    Retrying,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonitoringType {
    Metrics,
    Logging,
    Tracing,
    Alerting,
    Health,
    Performance,
}

/// Storage-specific error context
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageErrorData {
    pub pool_name: Option<String>,
    pub dataset_name: Option<String>,
    #[serde(default)]
    pub operation_type: String,
    pub filesystem_path: Option<String>,
    pub available_space: Option<u64>,
    pub required_space: Option<u64>,
    pub error_code: Option<i32>,
    #[serde(default)]
    pub retry_count: u32,
    #[serde(default)]
    pub context: HashMap<String, String>,
}

/// Network-specific error context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkErrorData {
    pub endpoint: Option<String>,
    pub port: Option<u16>,
    pub protocol: String,
    pub timeout_duration: Option<Duration>,
    pub retry_count: u32,
    pub response_code: Option<u16>,
    pub context: HashMap<String, String>,
}

impl Default for NetworkErrorData {
    fn default() -> Self {
        Self {
            endpoint: None,
            port: None,
            protocol: "HTTP".to_string(),
            timeout_duration: None,
            retry_count: 0,
            response_code: None,
            context: HashMap::new(),
        }
    }
}

/// Security-specific error context
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityErrorData {
    pub principal: Option<String>,
    #[serde(default)]
    pub operation: String,
    pub resource: Option<String>,
    #[serde(default)]
    pub required_permissions: Vec<String>,
    #[serde(default)]
    pub actual_permissions: Vec<String>,
    pub authentication_method: Option<String>,
    #[serde(default)]
    pub context: HashMap<String, String>,
}

/// Security severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl Default for SecuritySeverity {
    fn default() -> Self {
        Self::Warning
    }
}

/// Automation-specific error context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationErrorData {
    pub workflow_id: Option<String>,
    pub step_name: Option<String>,
    pub automation_type: String,
    pub retry_count: u32,
    pub max_retries: u32,
    pub context: HashMap<String, String>,
}

impl Default for AutomationErrorData {
    fn default() -> Self {
        Self {
            workflow_id: None,
            step_name: None,
            automation_type: String::new(),
            retry_count: 0,
            max_retries: 3,
            context: HashMap::new(),
        }
    }
}
