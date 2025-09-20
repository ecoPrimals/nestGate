use axum::response::IntoResponse;

/// Health check handler
pub const fn health_check() -> impl IntoResponse {
    "OK"
}
