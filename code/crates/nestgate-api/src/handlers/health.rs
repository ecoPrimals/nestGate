use axum::response::IntoResponse;

/// Health check handler
pub fn health_check() -> impl IntoResponse {
    "OK"
}
