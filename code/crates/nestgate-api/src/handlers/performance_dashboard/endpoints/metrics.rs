//
// HTTP endpoints for real-time metrics and monitoring.

use axum::{http::StatusCode, Json};
use tracing::{debug, error};

use crate::handlers::performance_dashboard::{
    types::{ApiResponse, RealTimeMetrics},
    services::RealTimeMetricsCollector,
};

/// GET /dashboard/metrics/realtime
pub async fn realtime_metrics() -> Result<Json<ApiResponse<RealTimeMetrics>>, StatusCode> {
    let collector = RealTimeMetricsCollector::new();
    
    match collector.get_current_metrics().await {
        Ok(metrics) => {
            debug!("Real-time metrics retrieved successfully");
            Ok(Json(ApiResponse::success(metrics)))
        },
        Err(e) => {
            error!("Realtime metrics error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
} 