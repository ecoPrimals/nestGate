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

#[cfg(test)]
mod tests {
    use super::*;

    // Handler Type tests
    #[test]
    fn test_handler_type_variants() {
        let api = HandlerType::Api;
        let middleware = HandlerType::Middleware;
        let event = HandlerType::Event;

        // Test Debug formatting
        assert!(format!("{api:?}").contains("Api"));
        assert!(format!("{middleware:?}").contains("Middleware"));

        // Test Clone
        let cloned = api.clone();
        assert!(format!("{cloned:?}").contains("Api"));
    }

    // Handler Phase tests
    #[test]
    fn test_handler_phase_variants() {
        let init = HandlerPhase::Initialization;
        let processing = HandlerPhase::Processing;
        let cleanup = HandlerPhase::Cleanup;

        assert!(format!("{init:?}").contains("Initialization"));
        assert!(format!("{processing:?}").contains("Processing"));
        assert!(format!("{cleanup:?}").contains("Cleanup"));
    }

    // Workflow State tests
    #[test]
    fn test_workflow_state_variants() {
        let pending = WorkflowState::Pending;
        let running = WorkflowState::Running;
        let completed = WorkflowState::Completed;
        let failed = WorkflowState::Failed;

        assert!(format!("{pending:?}").contains("Pending"));
        assert!(format!("{running:?}").contains("Running"));
        assert!(format!("{completed:?}").contains("Completed"));
        assert!(format!("{failed:?}").contains("Failed"));
    }

    // Monitoring Type tests
    #[test]
    fn test_monitoring_type_variants() {
        let metrics = MonitoringType::Metrics;
        let logging = MonitoringType::Logging;
        let health = MonitoringType::Health;

        assert!(format!("{metrics:?}").contains("Metrics"));
        assert!(format!("{logging:?}").contains("Logging"));
        assert!(format!("{health:?}").contains("Health"));
    }

    // Storage Error Data tests
    #[test]
    fn test_storage_error_data_default() {
        let data = StorageErrorData::default();

        assert_eq!(data.pool_name, None);
        assert_eq!(data.dataset_name, None);
        assert_eq!(data.operation_type, "");
        assert_eq!(data.filesystem_path, None);
        assert_eq!(data.retry_count, 0);
        assert!(data.context.is_empty());
    }

    #[test]
    fn test_storage_error_data_with_values() {
        let mut data = StorageErrorData::default();
        data.pool_name = Some("test-pool".to_string());
        data.dataset_name = Some("test-dataset".to_string());
        data.operation_type = "create".to_string();
        data.available_space = Some(1024);
        data.required_space = Some(2048);
        data.error_code = Some(42);
        data.retry_count = 3;

        assert_eq!(data.pool_name, Some("test-pool".to_string()));
        assert_eq!(data.dataset_name, Some("test-dataset".to_string()));
        assert_eq!(data.operation_type, "create");
        assert_eq!(data.available_space, Some(1024));
        assert_eq!(data.required_space, Some(2048));
        assert_eq!(data.error_code, Some(42));
        assert_eq!(data.retry_count, 3);
    }

    #[test]
    fn test_storage_error_data_clone() {
        let mut data = StorageErrorData::default();
        data.pool_name = Some("test-pool".to_string());

        let cloned = data.clone();
        assert_eq!(cloned.pool_name, Some("test-pool".to_string()));
    }

    // Network Error Data tests
    #[test]
    fn test_network_error_data_default() {
        let data = NetworkErrorData::default();

        assert_eq!(data.endpoint, None);
        assert_eq!(data.port, None);
        assert_eq!(data.protocol, "HTTP");
        assert_eq!(data.timeout_duration, None);
        assert_eq!(data.retry_count, 0);
        assert_eq!(data.response_code, None);
        assert!(data.context.is_empty());
    }

    #[test]
    fn test_network_error_data_with_values() {
        let mut data = NetworkErrorData::default();
        data.endpoint = Some("http://example.com".to_string());
        data.port = Some(8080);
        data.protocol = "HTTPS".to_string();
        data.timeout_duration = Some(Duration::from_secs(30));
        data.retry_count = 2;
        data.response_code = Some(404);

        assert_eq!(data.endpoint, Some("http://example.com".to_string()));
        assert_eq!(data.port, Some(8080));
        assert_eq!(data.protocol, "HTTPS");
        assert_eq!(data.timeout_duration, Some(Duration::from_secs(30)));
        assert_eq!(data.retry_count, 2);
        assert_eq!(data.response_code, Some(404));
    }

    #[test]
    fn test_network_error_data_clone() {
        let mut data = NetworkErrorData::default();
        data.endpoint = Some("http://test.com".to_string());

        let cloned = data.clone();
        assert_eq!(cloned.endpoint, Some("http://test.com".to_string()));
    }

    // Security Error Data tests
    #[test]
    fn test_security_error_data_default() {
        let data = SecurityErrorData::default();

        assert_eq!(data.principal, None);
        assert_eq!(data.operation, "");
        assert_eq!(data.resource, None);
        assert!(data.required_permissions.is_empty());
        assert!(data.actual_permissions.is_empty());
        assert_eq!(data.authentication_method, None);
        assert!(data.context.is_empty());
    }

    #[test]
    fn test_security_error_data_with_values() {
        let mut data = SecurityErrorData::default();
        data.principal = Some("user@example.com".to_string());
        data.operation = "read".to_string();
        data.resource = Some("/api/data".to_string());
        data.required_permissions = vec!["read".to_string(), "write".to_string()];
        data.actual_permissions = vec!["read".to_string()];
        data.authentication_method = Some("jwt".to_string());

        assert_eq!(data.principal, Some("user@example.com".to_string()));
        assert_eq!(data.operation, "read");
        assert_eq!(data.resource, Some("/api/data".to_string()));
        assert_eq!(data.required_permissions.len(), 2);
        assert_eq!(data.actual_permissions.len(), 1);
        assert_eq!(data.authentication_method, Some("jwt".to_string()));
    }

    #[test]
    fn test_security_error_data_clone() {
        let mut data = SecurityErrorData::default();
        data.principal = Some("test@example.com".to_string());

        let cloned = data.clone();
        assert_eq!(cloned.principal, Some("test@example.com".to_string()));
    }

    // Security Severity tests
    #[test]
    fn test_security_severity_variants() {
        let info = SecuritySeverity::Info;
        let warning = SecuritySeverity::Warning;
        let error = SecuritySeverity::Error;
        let critical = SecuritySeverity::Critical;

        assert_eq!(info, SecuritySeverity::Info);
        assert_eq!(warning, SecuritySeverity::Warning);
        assert_eq!(error, SecuritySeverity::Error);
        assert_eq!(critical, SecuritySeverity::Critical);
    }

    #[test]
    fn test_security_severity_default() {
        let severity = SecuritySeverity::default();
        assert_eq!(severity, SecuritySeverity::Warning);
    }

    #[test]
    fn test_security_severity_ordering() {
        let info = SecuritySeverity::Info;
        let warning = SecuritySeverity::Warning;

        assert_ne!(info, warning);
    }

    // Automation Error Data tests
    #[test]
    fn test_automation_error_data_default() {
        let data = AutomationErrorData::default();

        assert_eq!(data.workflow_id, None);
        assert_eq!(data.step_name, None);
        assert_eq!(data.automation_type, "");
        assert_eq!(data.retry_count, 0);
        assert_eq!(data.max_retries, 3);
        assert!(data.context.is_empty());
    }

    #[test]
    fn test_automation_error_data_with_values() {
        let mut data = AutomationErrorData::default();
        data.workflow_id = Some("workflow-123".to_string());
        data.step_name = Some("step-1".to_string());
        data.automation_type = "deployment".to_string();
        data.retry_count = 2;
        data.max_retries = 5;
        data.context.insert("key".to_string(), "value".to_string());

        assert_eq!(data.workflow_id, Some("workflow-123".to_string()));
        assert_eq!(data.step_name, Some("step-1".to_string()));
        assert_eq!(data.automation_type, "deployment");
        assert_eq!(data.retry_count, 2);
        assert_eq!(data.max_retries, 5);
        assert_eq!(data.context.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_automation_error_data_clone() {
        let mut data = AutomationErrorData::default();
        data.workflow_id = Some("test-workflow".to_string());

        let cloned = data.clone();
        assert_eq!(cloned.workflow_id, Some("test-workflow".to_string()));
    }

    #[test]
    fn test_automation_error_data_retry_logic() {
        let data = AutomationErrorData {
            workflow_id: Some("test".to_string()),
            step_name: None,
            automation_type: "test".to_string(),
            retry_count: 2,
            max_retries: 3,
            context: HashMap::new(),
        };

        assert!(data.retry_count < data.max_retries);
    }

    // Context HashMap tests
    #[test]
    fn test_storage_error_context_usage() {
        let mut data = StorageErrorData::default();
        data.context.insert("pool".to_string(), "tank".to_string());
        data.context
            .insert("operation".to_string(), "snapshot".to_string());

        assert_eq!(data.context.len(), 2);
        assert_eq!(data.context.get("pool"), Some(&"tank".to_string()));
    }

    #[test]
    fn test_network_error_context_usage() {
        let mut data = NetworkErrorData::default();
        data.context
            .insert("request_id".to_string(), "12345".to_string());

        assert_eq!(data.context.len(), 1);
        assert!(data.context.contains_key("request_id"));
    }

    #[test]
    fn test_security_error_context_usage() {
        let mut data = SecurityErrorData::default();
        data.context
            .insert("ip".to_string(), "192.168.1.1".to_string());
        data.context
            .insert("user_agent".to_string(), "Mozilla".to_string());

        assert_eq!(data.context.len(), 2);
    }
}
