// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **REQUEST/RESPONSE TYPES** — API communication patterns

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for  operation
pub struct Request {
    /// Unique request identifier
    pub id: String,
    /// HTTP method (GET, POST, etc.)
    pub method: String,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Optional request body
    pub body: Option<serde_json::Value>,
    /// Timestamp when the request was received
    pub timestamp: SystemTime,
}

/// Response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for  operation
pub struct Response {
    /// Associated request identifier
    pub request_id: String,
    /// HTTP status code
    pub status_code: u16,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Optional response body
    pub body: Option<serde_json::Value>,
    /// Timestamp when the response was generated
    pub timestamp: SystemTime,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

/// API error structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Error type for Api operations
pub struct ApiError {
    /// Error code for categorization
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Optional additional error details
    pub details: Option<serde_json::Value>,
    /// Timestamp when the error occurred
    pub timestamp: SystemTime,
}
