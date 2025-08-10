/// Safe serialization operations
/// Provides safe alternatives to JSON serialization that might panic
use crate::NestGateError;
use serde::{Deserialize, Serialize};

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// **SAFE JSON SERIALIZATION**
/// Replaces crate::safe_operations::safe_to_json()? with proper error handling
pub fn safe_to_json<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value).map_err(|e| NestGateError::Internal {
        message: format!("JSON serialization failed: {e}"),
        location: Some(format!("{}:{}", file!(), line!())),
        debug_info: Some(format!("Serializing type: {}", std::any::type_name::<T>())),
        is_bug: false,
    })
}

/// **SAFE JSON DESERIALIZATION**
/// Replaces crate::safe_operations::safe_from_json()? with proper error handling
pub fn safe_from_json<T: for<'de> Deserialize<'de>>(json: &str) -> Result<T> {
    serde_json::from_str(json).map_err(|e| NestGateError::Internal {
        message: format!("JSON deserialization failed: {e}"),
        location: Some(format!("{}:{}", file!(), line!())),
        debug_info: Some(format!(
            "Deserializing to type: {}",
            std::any::type_name::<T>()
        )),
        is_bug: false,
    })
}

/// **SAFE JSON PRETTY SERIALIZATION**
/// For configuration and debugging output
pub fn safe_to_json_pretty<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string_pretty(value).map_err(|e| NestGateError::Internal {
        message: format!("JSON pretty serialization failed: {e}"),
        location: Some(format!("{}:{}", file!(), line!())),
        debug_info: Some(format!("Serializing type: {}", std::any::type_name::<T>())),
        is_bug: false,
    })
}
