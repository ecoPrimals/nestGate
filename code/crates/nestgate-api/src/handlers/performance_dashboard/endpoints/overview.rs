// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// HTTP endpoints for dashboard overview and summary information.

//! Overview module

use axum::{
    extract::Query,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::handlers::performance_dashboard::{
    types::{ApiResponse, DashboardOverview, TimeRange},
    services::PerformanceDashboard,
    config::DashboardConfig,
};

/// Query parameters for dashboard overview
#[derive(Debug, Deserialize)]
/// Dashboardoverviewquery
pub struct DashboardOverviewQuery {
    /// Time range for metrics (optional)
    pub time_range: Option<String>,
    /// Include detailed metrics (optional)
    pub detailed: Option<bool>,
}
/// GET /dashboard/overview
pub async fn dashboard_overview(
    Query(_params): Query<DashboardOverviewQuery>,
) -> Result<Json<ApiResponse<DashboardOverview>>, StatusCode> {
    let dashboard = PerformanceDashboard::new(DashboardConfig::default());
    let time_range = TimeRange::last_hour(); // Default time range
    
    match dashboard.get_dashboard_overview(time_range).await {
        Ok(overview) => {
            debug!("Dashboard overview retrieved successfully");
            Ok(Json(ApiResponse::success(overview)))
        }
        Err(e) => {
            error!("Dashboard overview error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
} 