
use axum::response::IntoResponse;

/// Health check handler
pub async fn health_check() -> impl IntoResponse {
    "OK"
}
