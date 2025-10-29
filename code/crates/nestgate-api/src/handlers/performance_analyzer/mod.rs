//! **REAL PERFORMANCE ANALYZER**
//!
//! Live system performance monitoring and analysis that replaces all mock implementations
//! with real ZFS metrics, system monitoring, and performance data collection.
//!
//! This module has been refactored into a modular structure for better maintainability.

pub mod analyzer;
pub mod collectors;
pub mod metrics;
pub mod reports;
pub mod types;

// Re-export the new modular implementation
pub use analyzer::{
    AnalysisResult, AnalyzerConfig, ComponentAnalysis, PerformanceAnalyzer, PerformanceStatus,
};
pub use collectors::{BatchCollector, DataCollector, MetricsSnapshot};
pub use metrics::{
    DiskIOMetrics, MetricsError, NetworkMetrics, SystemMetrics, SystemMetricsCollector,
};
pub use reports::{MultiFormatReport, PerformanceReport, ReportConfig, ReportGenerator};

// Re-export legacy types for backward compatibility
pub use types::*;
