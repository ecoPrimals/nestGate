//
// This middleware is currently stubbed out pending the implementation of
// the required AI response types in nestgate-core.

use axum::{middleware::Next, response::Response};
use serde_json::Value;

/// AI-First response wrapper (stubbed)
pub async fn ai_first_middleware(
    request: axum::extract::Request,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    // Simple pass-through middleware for now
    let response = next.run(request).await;
    Ok(response)
}

/// Stub function for AI response creation
pub fn create_ai_first_response(
    _success: bool,
    _data: Value,
    _error_message: Option<String>,
    _status: axum::http::StatusCode,
    _request_id: String,
    _processing_time_ms: u64,
    _confidence_score: f64,
    _suggested_actions: Vec<String>,
) -> serde_json::Value {
    serde_json::json!({
        "success": true,
        "data": _data,
        "ai_enabled": false,
        "status": "stub_implementation"
    })
}
