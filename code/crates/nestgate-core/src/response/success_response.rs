/// Success Response Module
/// Success response types and utilities
/// **PROBLEM SOLVED**: Standardized success response patterns
use axum::response::{IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Success response structure for specific use cases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResponse {
    /// Success message
    pub message: String,
    /// Success timestamp
    pub timestamp: DateTime<Utc>,
    /// Additional success data
    pub data: Option<HashMap<String, serde_json::Value>>,
    /// Success metadata
    pub metadata: Option<HashMap<String, String>>,
}

impl SuccessResponse {
    /// Create a new success response with message
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            timestamp: chrono::Utc::now(),
            data: None,
            metadata: None,
        }
    }

    /// Create a success response with data
    pub fn with_data(message: &str, data: HashMap<String, serde_json::Value>) -> Self {
        Self {
            message: message.to_string(),
            timestamp: chrono::Utc::now(),
            data: Some(data),
            metadata: None,
        }
    }

    /// Create a success response with metadata
    pub fn with_metadata(message: &str, metadata: HashMap<String, String>) -> Self {
        Self {
            message: message.to_string(),
            timestamp: chrono::Utc::now(),
            data: None,
            metadata: Some(metadata),
        }
    }

    /// Add data to the success response
    pub fn add_data(mut self, key: &str, value: serde_json::Value) -> Self {
        let mut data = self.data.unwrap_or_default();
        data.insert(key.to_string(), value);
        self.data = Some(data);
        self
    }

    /// Add metadata to the success response
    pub fn add_metadata(mut self, key: &str, value: &str) -> Self {
        let mut metadata = self.metadata.unwrap_or_default();
        metadata.insert(key.to_string(), value.to_string());
        self.metadata = Some(metadata);
        self
    }

    /// Convert to JSON for HTTP responses
    pub fn to_json(self) -> Json<Self> {
        Json(self)
    }
}

impl IntoResponse for SuccessResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, Json(self)).into_response()
    }
}

impl Default for SuccessResponse {
    fn default() -> Self {
        Self::new("Operation completed successfully")
    }
}

/// Success response factory for common success patterns
pub struct SuccessResponseFactory;

impl SuccessResponseFactory {
    /// Create a "created" success response
    pub fn created(resource: &str, id: &str) -> SuccessResponse {
        SuccessResponse::new(&format!("{resource} created successfully"))
            .add_data("resource_type", serde_json::json!(resource))
            .add_data("resource_id", serde_json::json!(id))
    }

    /// Create an "updated" success response
    pub fn updated(resource: &str, id: &str) -> SuccessResponse {
        SuccessResponse::new(&format!("{resource} updated successfully"))
            .add_data("resource_type", serde_json::json!(resource))
            .add_data("resource_id", serde_json::json!(id))
    }

    /// Create a "deleted" success response
    pub fn deleted(resource: &str, id: &str) -> SuccessResponse {
        SuccessResponse::new(&format!("{resource} deleted successfully"))
            .add_data("resource_type", serde_json::json!(resource))
            .add_data("resource_id", serde_json::json!(id))
    }

    /// Create a "started" success response for service operations
    pub fn started(service: &str) -> SuccessResponse {
        SuccessResponse::new(&format!("{service} started successfully"))
            .add_data("service", serde_json::json!(service))
            .add_data("status", serde_json::json!("started"))
    }

    /// Create a "stopped" success response for service operations
    pub fn stopped(service: &str) -> SuccessResponse {
        SuccessResponse::new(&format!("{service} stopped successfully"))
            .add_data("service", serde_json::json!(service))
            .add_data("status", serde_json::json!("stopped"))
    }

    /// Create a "completed" success response for operations
    pub fn completed(operation: &str) -> SuccessResponse {
        SuccessResponse::new(&format!("{operation} completed successfully"))
            .add_data("operation", serde_json::json!(operation))
            .add_data("status", serde_json::json!("completed"))
    }

    /// Create a "validated" success response
    pub fn validated(resource: &str) -> SuccessResponse {
        SuccessResponse::new(&format!("{resource} validation successful"))
            .add_data("resource", serde_json::json!(resource))
            .add_data("status", serde_json::json!("valid"))
    }

    /// Create a generic "ok" success response
    pub fn ok() -> SuccessResponse {
        SuccessResponse::new("OK")
    }

    /// Create a "health check" success response
    pub fn health_check(service: &str, status: &str) -> SuccessResponse {
        SuccessResponse::new(&format!("{service} health check passed"))
            .add_data("service", serde_json::json!(service))
            .add_data("health_status", serde_json::json!(status))
    }

    /// Create a "configuration updated" success response
    pub fn config_updated(service: &str) -> SuccessResponse {
        SuccessResponse::with_metadata(&format!("{service} configuration updated successfully"), {
            let mut metadata = HashMap::new();
            metadata.insert("service".to_string(), service.to_string());
            metadata.insert("operation".to_string(), "config_update".to_string());
            metadata
        })
    }
}
