//! # Tracing Setup
//! Setup functionality and utilities.
// Tracing and logging system initialization

use crate::{NestGateError, Result};
use std::fs::OpenOptions;
use std::sync::Arc;
use tracing::{info, debug};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

use super::aggregator::LogAggregator;
use super::config::TracingConfig;
use super::types::TraceContext;

/// Initialize tracing system
pub fn initialize_tracing(config: TracingConfig) -> Result<LogAggregator> {
    let env_filter_str = format!("nestgate={config.level}");
    
    let mut layers: Vec<Box<dyn Layer<_> + Send + Sync>> = Vec::new();
    // Console layer
    if config.console_enabled {
        let console_filter = EnvFilter::new(&env_filter_str);
        if config.json_format {
            let console_layer = fmt::layer()
                .json()
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_filter(console_filter);
            layers.push(console_layer.boxed());
        } else {
            let console_layer = fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_filter(console_filter);
            layers.push(console_layer.boxed());
        }
    }

    // File layer
    if config.file_enabled {
        if let Some(file_path) = &config.file_path {
            // Ensure log directory exists
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| NestGateError::internal_error(
                    debug_info: None,
                )?;
            }

            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)
                .map_err(|e| NestGateError::internal_error(
                    debug_info: None,
                )?;

            let file_filter = EnvFilter::new(&env_filter_str);
            if config.json_format {
                let file_layer = fmt::layer()
                    .json()
                    .with_writer(Arc::new(file))
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_filter(file_filter);
                layers.push(file_layer.boxed());
            } else {
                let file_layer = fmt::layer()
                    .with_writer(Arc::new(file))
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_filter(file_filter);
                layers.push(file_layer.boxed());
            }
        }
    }

    // Initialize subscriber
    tracing_subscriber::registry().with(layers).init();

    info!("✅ Tracing initialized with level: {}", config.level);

    // Create log aggregator
    let aggregator = LogAggregator::new(config.aggregation);

    Ok(aggregator)
}

/// Create a new span with trace context
pub fn create_span(name: &str, context: Option<&TraceContext>) -> tracing::Span {
    let span = tracing::info_span!(
        "operation",
        operation = name,
        trace_id = context.map(|c| c.trace_id.as_str()).unwrap_or(""),
        span_id = context.map(|c| c.span_id.as_str()).unwrap_or(""),
        parent_span_id = context
            .and_then(|c| c.parent_span_id.as_deref())
            .unwrap_or("")
    );
    if let Some(ctx) = context {
        debug!("Created span: {} with trace_id: {}", name, ctx.trace_id);
    }

    span
} 