//! Monitoring and Observability
//!
//! Comprehensive monitoring system for NestGate including metrics collection,
//! distributed tracing, health checks, and alerting capabilities.

pub mod alerts;
pub mod dashboards;
pub mod health_checks;
pub mod metrics;
pub mod tracing;

// Re-export key monitoring features
pub use alerts::{AlertChannel, AlertManager, AlertRule, AlertSeverity};
pub use health_checks::{ComponentHealth, HealthCheckManager, HealthStatus, SystemHealth};
pub use metrics::{
    MetricsCollector, MetricsExporter, PerformanceMetrics, ProviderMetrics, StorageMetrics,
    SystemMetrics,
};
pub use tracing::{create_span, init_tracing, TraceContext, TracingConfig};
