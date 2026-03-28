// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// Success response types and builders
// Provides unified success response structures for API endpoints

//! Success Response module

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified success response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Success operation
pub struct SuccessResponse {
    /// Success message
    pub message: String,

    /// Response data
    pub data: Option<HashMap<String, serde_json::Value>>,

    /// Response metadata
    pub metadata: HashMap<String, serde_json::Value>,

    /// Timestamp of response
    pub timestamp: String,

    /// Request correlation ID
    pub correlation_id: Option<String>,
}
impl SuccessResponse {
    /// Create a new success response
    #[must_use]
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            data: None,
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            correlation_id: None,
        }
    }

    /// Add data to the response
    #[must_use]
    pub fn add_data(mut self, key: &str, value: serde_json::Value) -> Self {
        if self.data.is_none() {
            self.data = Some(HashMap::new());
        }
        if let Some(ref mut data) = self.data {
            data.insert(key.to_string(), value);
        }
        self
    }

    /// Add metadata to the response
    #[must_use]
    pub fn add_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        self.metadata.insert(key.to_string(), value);
        self
    }

    /// Set correlation ID
    #[must_use]
    pub fn with_correlation_id(mut self, correlation_id: String) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    /// Set multiple data fields at once
    #[must_use]
    pub fn with_data(mut self, data: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(data);
        self
    }

    /// Set multiple metadata fields at once
    #[must_use]
    pub fn with_metadata(mut self, metadata: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = metadata;
        self
    }
}

impl IntoResponse for SuccessResponse {
    /// Into Response
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Success response factory for common success types
pub struct SuccessResponseFactory;
impl SuccessResponseFactory {
    /// Create a "created" success response
    #[must_use]
    pub fn created(path: &str, id: &str) -> SuccessResponse {
        SuccessResponse::new(&format!("{path} created successfully"))
            .add_data("resource_type", serde_json::json!(path))
            .add_data("resource_id", serde_json::json!(id))
    }

    /// Create an "updated" success response
    #[must_use]
    pub fn updated(resource: &str, id: &str) -> SuccessResponse {
        SuccessResponse::new(&format!("{resource} updated successfully"))
            .add_data("resource_type", serde_json::json!(resource))
            .add_data("resource_id", serde_json::json!(id))
    }

    /// Create a "deleted" success response
    #[must_use]
    pub fn deleted(resource: &str) -> SuccessResponse {
        SuccessResponse::new(&format!("{resource} deleted successfully"))
            .add_data("resource_type", serde_json::json!(resource))
    }

    /// Create a "retrieved" success response
    #[must_use]
    pub fn retrieved(resource_type: &str, count: u64) -> SuccessResponse {
        SuccessResponse::new(&format!("{resource_type} retrieved successfully"))
            .add_data("resource_type", serde_json::json!(resource_type))
            .add_data("count", serde_json::json!(count))
    }

    /// Create a generic operation success response
    #[must_use]
    pub fn operation_success(operation: &str, resource: &str) -> SuccessResponse {
        SuccessResponse::new(&format!("{operation} completed successfully"))
            .add_data("operation", serde_json::json!(operation))
            .add_data("resource", serde_json::json!(resource))
    }

    /// Create a validation success response
    #[must_use]
    pub fn validation_success(message: &str) -> SuccessResponse {
        SuccessResponse::new(message).add_metadata("type", serde_json::json!("validation"))
    }

    /// Create a processing success response
    #[must_use]
    pub fn processing_started(job_id: &str, operation: &str) -> SuccessResponse {
        SuccessResponse::new("Processing started successfully")
            .add_data("job_id", serde_json::json!(job_id))
            .add_data("operation", serde_json::json!(operation))
            .add_metadata("status", serde_json::json!("processing"))
    }

    /// Create a processing completed response
    #[must_use]
    pub fn processing_completed(job_id: &str, result: serde_json::Value) -> SuccessResponse {
        SuccessResponse::new("Processing completed successfully")
            .add_data("job_id", serde_json::json!(job_id))
            .add_data("result", result)
            .add_metadata("status", serde_json::json!("completed"))
    }

    /// Create a health check success response
    #[must_use]
    pub fn health_check(service: &str, status: &str) -> SuccessResponse {
        SuccessResponse::new(&format!("{service} is healthy"))
            .add_data("service", serde_json::json!(service))
            .add_data("status", serde_json::json!(status))
            .add_metadata("check_type", serde_json::json!("health"))
    }

    /// Create a configuration update success response
    #[must_use]
    pub fn config_updated(component: &str, changes: u32) -> SuccessResponse {
        SuccessResponse::new(&format!("{component} configuration updated"))
            .add_data("component", serde_json::json!(component))
            .add_data("changes_applied", serde_json::json!(changes))
            .add_metadata("type", serde_json::json!("configuration"))
    }
}

/// Legacy success response for backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for LegacySuccess operation
pub struct LegacySuccessResponse {
    /// Success
    pub success: bool,
    /// Message
    pub message: String,
    /// Data
    pub data: Option<serde_json::Value>,
    /// Timestamp
    pub timestamp: String,
}
impl From<SuccessResponse> for LegacySuccessResponse {
    /// From
    fn from(unified: SuccessResponse) -> Self {
        Self {
            success: true,
            message: unified.message,
            data: unified
                .data
                .map(|d| serde_json::to_value(d).unwrap_or_default()),
            timestamp: unified.timestamp,
        }
    }
}

impl IntoResponse for LegacySuccessResponse {
    /// Into Response
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response_creation() {
        let response = SuccessResponse::new("Test success");
        assert_eq!(response.message, "Test success");
        assert!(response.data.is_none());
        assert!(response.metadata.is_empty());
    }

    #[test]
    fn test_success_response_with_data() {
        let response = SuccessResponse::new("Test").add_data("key", serde_json::json!("value"));

        assert!(response.data.is_some());
        let data = response.data.expect("Operation failed");
        assert_eq!(data["key"], serde_json::json!("value"));
    }

    #[test]
    fn test_success_factory_methods() {
        let created = SuccessResponseFactory::created("user", "123");
        assert!(created.message.contains("created successfully"));
        assert!(created.data.is_some());

        let updated = SuccessResponseFactory::updated("user", "123");
        assert!(updated.message.contains("updated successfully"));

        let deleted = SuccessResponseFactory::deleted("user");
        assert!(deleted.message.contains("deleted successfully"));
    }

    #[test]
    fn test_legacy_conversion() {
        let unified = SuccessResponse::new("Test success");
        let legacy: LegacySuccessResponse = unified.into();
        assert!(legacy.success);
        assert_eq!(legacy.message, "Test success");
    }

    #[test]
    fn test_processing_responses() {
        let started = SuccessResponseFactory::processing_started("job123", "data_migration");
        assert!(started.data.is_some());

        let completed = SuccessResponseFactory::processing_completed(
            "job123",
            serde_json::json!({"result": "success"}),
        );
        assert!(completed.data.is_some());
    }
}
