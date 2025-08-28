/// Safe serialization operations
/// Provides safe alternatives to JSON serialization that might panic
use crate::error::NestGateError;
use serde::{Deserialize, Serialize};

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
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
            error_id: uuid::Uuid::new_v4().to_string(),
            operation: "safe_to_json".to_string(),
            component: "safe_serialization".to_string(),
            timestamp: std::time::SystemTime::now(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                map.insert("serializing_type".to_string(), std::any::type_name::<T>().to_string());
                map.insert("error".to_string(), e.to_string());
                map
            },
            stack_trace: None,
            related_errors: vec![],
            retry_info: None,
            recovery_suggestions: vec!["Check that the type implements Serialize correctly".to_string()],
                    performance_metrics: None,
                    environment: None,
            performance_metrics: None,
            environment: None,
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
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
            error_id: uuid::Uuid::new_v4().to_string(),
            operation: "safe_from_json".to_string(),
            component: "safe_serialization".to_string(),
            timestamp: std::time::SystemTime::now(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                map.insert("deserializing_to_type".to_string(), std::any::type_name::<T>().to_string());
                map.insert("json_sample".to_string(), json.chars().take(100).collect::<String>());
                map.insert("error".to_string(), e.to_string());
                map
            },
            stack_trace: None,
            related_errors: vec![],
            retry_info: None,
            recovery_suggestions: vec!["Check JSON format and type compatibility".to_string()],
                    performance_metrics: None,
                    environment: None,
            performance_metrics: None,
            environment: None,
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
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
            error_id: uuid::Uuid::new_v4().to_string(),
            operation: "safe_to_json_pretty".to_string(),
            component: "safe_serialization".to_string(),
            timestamp: std::time::SystemTime::now(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                map.insert("serializing_type".to_string(), std::any::type_name::<T>().to_string());
                map.insert("error".to_string(), e.to_string());
                map
            },
            stack_trace: None,
            related_errors: vec![],
            retry_info: None,
            recovery_suggestions: vec!["Check that the type implements Serialize correctly".to_string()],
                    performance_metrics: None,
                    environment: None,
            performance_metrics: None,
            environment: None,
        }),
    })
}
