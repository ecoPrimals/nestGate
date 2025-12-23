//! **PERFORMANCE DASHBOARD MODULE**
//!
//! Comprehensive performance monitoring and analytics dashboard.

pub mod handlers;
/// Real-time metrics collection and analysis
pub mod metrics;
/// Performance optimization algorithms and recommendations
pub mod optimizer;
pub mod types;

// Re-export main types
pub use types::*;

// Re-export simplified handler functions
pub use handlers::{
    get_dashboard_overview, get_performance_analysis, stream_dashboard_metrics, DashboardQuery,
    PerformanceDashboard,
};

// TEMP_DISABLED: #[cfg(test)]
// TEMP_DISABLED: mod handlers_tests;

#[cfg(test)]
mod optimizer_tests;

#[cfg(test)]
mod handlers_tests;

#[cfg(test)]
mod metrics_tests;
