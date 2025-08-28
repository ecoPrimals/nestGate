/// Safe serialization operations
/// Provides safe alternatives to JSON serialization that might panic
use crate::error::{NestGateError, ErrorSeverity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// **SAFE JSON SERIALIZATION**
/// Replaces crate::safe_operations::safe_to_json()? with proper error handling
pub fn safe_to_json<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value).map_err(|e| NestGateError::Internal {
        message: format!("JSON serialization failed: {e}"),
        component: "safe_serialization".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        is_bug: false,
        context: Some(crate::error::context::ErrorContext {
            error_id: uuid::Uuid::new_v4().to_string(),
            component: "safe_operations".to_string(),
            operation: "serialize".to_string(),
            timestamp: std::time::SystemTime::now(),
            stack_trace: None,
            related_errors: vec![],
            retry_info: None,
            user_context: None,
            request_context: None,
            performance_metrics: None,
            environment: None,
            metadata: std::collections::HashMap::new(),
            severity: ErrorSeverity::Medium,
        }),
    })
}

/// **SAFE JSON DESERIALIZATION**
/// Replaces crate::safe_operations::safe_from_json()? with proper error handling
pub fn safe_from_json<T: for<'de> Deserialize<'de>>(json: &str) -> Result<T> {
    serde_json::from_str(json).map_err(|e| NestGateError::Internal {
        message: format!("JSON deserialization failed: {e}"),
        component: "safe_serialization".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        is_bug: false,
        context: Some(crate::error::context::ErrorContext {
            error_id: uuid::Uuid::new_v4().to_string(),
            component: "safe_operations".to_string(),
            operation: "deserialize".to_string(),
            timestamp: std::time::SystemTime::now(),
            stack_trace: None,
            related_errors: vec![],
            retry_info: None,
            user_context: None,
            request_context: None,
            performance_metrics: None,
            environment: None,
            metadata: std::collections::HashMap::new(),
            severity: ErrorSeverity::Medium,
        }),
    })
}

/// **SAFE JSON PRETTY SERIALIZATION**
/// For configuration and debugging output
pub fn safe_to_json_pretty<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string_pretty(value).map_err(|e| NestGateError::Internal {
        message: format!("JSON pretty serialization failed: {e}"),
        component: "safe_serialization".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        is_bug: false,
        context: Some(crate::error::context::ErrorContext {
            error_id: uuid::Uuid::new_v4().to_string(),
            component: "safe_operations".to_string(),
            operation: "validate".to_string(),
            timestamp: std::time::SystemTime::now(),
            stack_trace: None,
            related_errors: vec![],
            retry_info: None,
            user_context: None,
            request_context: None,
            performance_metrics: None,
            environment: None,
            metadata: std::collections::HashMap::new(),
            severity: ErrorSeverity::Medium,
        }),
    })
}
