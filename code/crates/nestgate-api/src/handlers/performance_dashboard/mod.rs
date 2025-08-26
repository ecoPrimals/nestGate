//
// This module provides comprehensive real-time performance monitoring, analytics,
// and visualization for the NestGate storage system. It includes intelligent
// insights, optimization recommendations, and predictive analytics.
//
// ## Key Features
// - **Real-Time Metrics**: Live storage performance monitoring  
// - **Predictive Analytics**: AI-powered performance forecasting
// - **Optimization Insights**: Intelligent recommendations for performance improvement
// - **Resource Monitoring**: Comprehensive system resource tracking
// - **Historical Analysis**: Trend analysis and pattern recognition

pub mod types;
pub mod metrics;
pub mod analyzer;
pub mod optimizer;
pub mod handlers;

// Re-export main types and components
pub use types::*;
pub use metrics::RealTimeMetricsCollector;
pub use analyzer::PerformanceAnalyzer;
pub use optimizer::OptimizationEngineInterface;
pub use handlers::PerformanceDashboard;

// Re-export HTTP handler functions
pub use handlers::{
    dashboard_overview,
    realtime_metrics,
    pool_trends,
    capacity_analysis,
    io_performance,
    cache_performance,
    performance_forecast,
    dashboard_events,
}; 