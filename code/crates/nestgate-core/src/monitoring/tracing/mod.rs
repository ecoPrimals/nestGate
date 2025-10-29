//! # Tracing and Logging Setup
//! Module definitions and exports.
// Comprehensive tracing and logging system for NestGate including structured
//! logging, log aggregation, distributed tracing, and integration with external
//! logging systems like ELK stack, Loki, and Jaeger.

// Module declarations
pub mod aggregator;
pub mod config;
pub mod retention;
pub mod setup;
pub mod types;

// Re-export all public items for backward compatibility
pub use aggregator::LogAggregator;
pub use config::{
    ElasticsearchAuth, LogAggregationConfig, LogDestination, LogRetentionConfig, LokiAuth,
    TracingConfig,
};
pub use retention::{LogRetentionManager, LogStats};
pub use setup::{create_span, initialize_tracing};
pub use types::{generate_span_id, generate_trace_id, LogEntry, TraceContext}; 