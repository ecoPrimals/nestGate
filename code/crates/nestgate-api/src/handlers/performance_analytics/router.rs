//! Performance Analytics Router
//!
//! Router configuration for performance analytics endpoints.

use axum::{
    routing::{get, post},
    Router,
};

use super::handlers::*;

/// Create performance analytics router
pub fn create_performance_router() -> Router<crate::routes::AppState> {
    Router::new()
        .route("/metrics/current", get(get_current_metrics))
        .route("/metrics/historical", get(get_historical_metrics))
        .route("/alerts", get(get_alerts))
        .route("/alerts/:alert_id/acknowledge", post(acknowledge_alert))
        .route("/recommendations", get(get_recommendations))
        .route("/recommendations/:rec_id/apply", post(apply_recommendation))
        .route("/config", get(get_config))
        .route("/config", post(update_config))
        .route("/dashboard", get(get_dashboard))
} 