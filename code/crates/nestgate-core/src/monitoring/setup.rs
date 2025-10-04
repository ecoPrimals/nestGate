// **TRACING SETUP AND CONFIGURATION**
//! Monitoring and observability functionality.
// This module provides tracing system initialization, configuration management,
//! and the main TracingManager for coordinating logging and distributed tracing.

use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info, warn};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

use super::collectors::{LogAggregator, LogEntry};
use super::exporters::{LogDestination, LogRetentionConfig};

// ==================== SECTION ====================

/// Tracing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    /// Enable console logging
    pub console_enabled: bool,
    /// Enable file logging
    pub file_enabled: bool,
    /// Log file path
    /// Enable JSON formatting
    pub json_format: bool,
    /// Enable distributed tracing
    pub distributed_tracing: bool,
    /// Jaeger endpoint for distributed tracing
    pub jaeger_endpoint: Option<String>,
    /// Log aggregation settings
    pub aggregation: LogAggregationConfig,
    /// Custom fields to include in all logs
    pub custom_fields: HashMap<String, String>,
}
impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            console_enabled: true,
            file_enabled: true,
            json_format: true,
            distributed_tracing: false,
            jaeger_endpoint: None,
            aggregation: LogAggregationConfig::default(),
            custom_fields: HashMap::new(),
        }
    }
}

/// Log aggregation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAggregationConfig {
    /// Enable log aggregation
    pub enabled: bool,
    /// Buffer size for log batching
    pub buffer_size: usize,
    /// Flush interval for batched logs
    pub flush_interval: Duration,
    /// External log destinations
    pub destinations: Vec<LogDestination>,
    /// Log retention settings
    pub retention: LogRetentionConfig,
}
impl Default for LogAggregationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            buffer_size: 1000,
            flush_interval: Duration::from_secs(10),
            destinations: vec![],
            retention: LogRetentionConfig::default(),
        }
    }
}

/// Distributed tracing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedTracingConfig {
    /// Enable distributed tracing
    pub enabled: bool,
    /// Service name for tracing
    pub service_name: String,
    /// Jaeger endpoint
    pub jaeger_endpoint: Option<String>,
    /// Sampling rate (0.0 to 1.0)
    pub sampling_rate: f64,
    /// Custom tags to include in all spans
    pub custom_tags: HashMap<String, String>,
}
impl Default for DistributedTracingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            service_name: "nestgate".to_string(),
            jaeger_endpoint: None,
            sampling_rate: 0.1,
            custom_tags: HashMap::new(),
        }
    }
}

/// Tracing context for distributed tracing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingContext {
    /// Trace ID for correlation across services
    pub trace_id: String,
    /// Span ID for current operation
    pub span_id: String,
    /// Parent span ID
    pub parent_span_id: Option<String>,
    /// Service name
    pub service_name: String,
    /// Operation name
    pub operation_name: String,
    /// Custom tags
    pub tags: HashMap<String, String>,
}
impl TracingContext {
    /// Create a new tracing context
    #[must_use]
    pub fn new(service_name: &str, operation_name: &str) -> Self {
        Self {
            trace_id: uuid::Uuid::new_v4().to_string(),
            span_id: uuid::Uuid::new_v4().to_string(),
            parent_span_id: None,
            service_name: service_name.to_string(),
            operation_name: operation_name.to_string(),
            tags: HashMap::new(),
        }
    }

    /// Create a child span from this context
    pub fn create_child_span(&self, operation_name: &str) -> Self {
        Self {
            trace_id: self.trace_id.clone(),
            span_id: uuid::Uuid::new_v4().to_string(),
            parent_span_id: Some(self.span_id.clone()),
            service_name: self.instance_name.clone(),
            operation_name: operation_name.to_string(),
            tags: self.tags.clone(),
        }
    }

    /// Add a tag to the context
    #[must_use]
    pub fn with_tag(mut self, key: &str, value: &str) -> Self {
        self.tags.insert(key.to_string(), value.to_string());
        self
    }
}

// ==================== SECTION ====================

/// Main tracing manager for coordinating all logging and tracing operations
#[derive(Debug)]
pub struct TracingManager {
    /// Configuration
    pub config: TracingConfig,
    /// Log aggregator
    pub aggregator: Option<Arc<LogAggregator>>,
    /// Distributed tracing config
    pub distributed_config: DistributedTracingConfig,
    /// Current tracing context
    pub current_context: Arc<RwLock<Option<TracingContext>>>,
}
impl TracingManager {
    /// Create a new tracing manager
    pub fn new(config: TracingConfig) -> Self {
        let distributed_config = DistributedTracingConfig {
            enabled: config.distributed_tracing,
            jaeger_endpoint: config.jaeger_endpoint.clone(),
            ..Default::default()
        };

        Self {
            config,
            aggregator: None,
            distributed_config,
            current_context: Arc::new(RwLock::new(None)),
        }
    }

    /// Initialize the tracing system
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn initialize(&mut self) -> Result<()>  {
        info!("🚀 Initializing NestGate tracing system");

        // Create log directory if needed
        if let Some(file_path) = &self.config.file_path {
            if let Some(parent) = file_path.parent() {
                tokio::fs::create_dir_all(parent).await.map_err(|e| {
                    NestGateError::Io {
                        message: format!("Failed to create log directory: {e}"),
                        // retryable: true}
                )?;
            }
        }

        // Initialize tracing subscriber
        self.setup_subscriber().await?;

        // Initialize log aggregation if enabled
        if self.config.aggregation.enabled {
            self.setup_aggregation().await?;
        }

        // Initialize distributed tracing if enabled
        if self.distributed_config.enabled {
            self.setup_distributed_tracing().await?;
        }

        info!("✅ Tracing system initialized successfully");
        Ok(())
    }

    /// Setup tracing subscriber with layers
    async fn setup_subscriber(&self) -> Result<()> {
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(&self.config.level));

        let subscriber = tracing_subscriber::registry().with(env_filter);

        // Add console layer if enabled
        let subscriber = if self.config.console_enabled {
            if self.config.json_format {
                subscriber.with(
                    fmt::layer()
                        .json()
                        .with_current_span(true)
                        .with_span_list(true),
                )
            } else {
                subscriber.with(
                    fmt::layer()
                        .pretty()
                        .with_current_span(true)
                        .with_span_list(true),
                )
            }
        } else {
            subscriber
        };

        // Add file layer if enabled
        let subscriber = if self.config.file_enabled {
            if let Some(file_path) = &self.config.file_path {
                let file = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file_path)
                    .map_err(|e| NestGateError::Io {
                        message: format!("Failed to open log file: {e}"),
                        // retryable: true)?;

                if self.config.json_format {
                    subscriber.with(
                        fmt::layer()
                            .json()
                            .with_writer(Arc::new(file))
                            .with_current_span(true),
                    )
                } else {
                    subscriber.with(
                        fmt::layer()
                            .with_writer(Arc::new(file))
                            .with_current_span(true),
                    )
                }
            } else {
                subscriber
            }
        } else {
            subscriber
        };

        subscriber.init();
        debug!("Tracing subscriber initialized");
        Ok(())
    }

    /// Setup log aggregation system
    async fn setup_aggregation(&mut self) -> Result<()> {
        let (sender, receiver) = mpsc::channel(self.config.aggregation.buffer_size);
        
        let aggregator = LogAggregator::new(
            self.config.aggregation.clone(),
            receiver,
        );

        let aggregator_arc = Arc::new(aggregator);
        self.aggregator = Some(aggregator_arc.clone());

        // Start aggregation task
        let aggregator_task = aggregator_arc.clone();
        tokio::spawn(async move {
            if let Err(e) = aggregator_task.start().await {
                error!("Log aggregator failed: {}", e);
            }
        );

        debug!("Log aggregation system initialized");
        Ok(())
    }

    /// Setup distributed tracing
    async fn setup_distributed_tracing(&self) -> Result<()> {
        if let Some(jaeger_endpoint) = &self.distributed_config.jaeger_endpoint {
            debug!("Setting up Jaeger tracing to: {}", jaeger_endpoint);
            // Jaeger setup would go here
            // For now, we'll just log that it's configured
        }

        debug!("Distributed tracing system initialized");
        Ok(())
    }

    /// Get current tracing context
    pub async fn get_context(&self) -> Option<TracingContext> {
        self.current_context.read().await.clone()
    }

    /// Set current tracing context
    pub async fn set_context(&self, context: Option<TracingContext>) {
        *self.current_context.write().await = context;
    }

    /// Create a new tracing context for an operation
    pub async fn start_operation(&self, operation_name: &str) -> TracingContext {
        let context = TracingContext::new(&self.distributed_config.instance_name, operation_name);
        self.set_context(Some(context.clone())).await;
        context
    }

    /// End an operation and clear context
    pub async fn end_operation(&self) {
        self.set_context(None).await;
    }

    /// Send a log entry to the aggregator
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn send_log(&self, entry: LogEntry) -> Result<()>  {
        if let Some(aggregator) = &self.aggregator {
            aggregator.send_log(entry).await?;
        }
        Ok(())
    }

    /// Shutdown the tracing system gracefully
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn shutdown(&self) -> Result<()>  {
        info!("🔄 Shutting down tracing system");

        if let Some(aggregator) = &self.aggregator {
            aggregator.shutdown().await?;
        }

        info!("✅ Tracing system shut down successfully");
        Ok(())
    }

    /// Get tracing statistics
    pub async fn get_statistics(&self) -> TracingStatistics {
        let mut stats = TracingStatistics::default();

        if let Some(aggregator) = &self.aggregator {
            stats.logs_processed = aggregator.get_processed_count().await;
            stats.logs_pending = aggregator.get_pending_count().await;
        }

        stats.distributed_tracing_enabled = self.distributed_config.enabled;
        stats.log_aggregation_enabled = self.config.aggregation.enabled;

        stats
    }
}

/// Tracing system statistics
#[derive(Debug, Clone, Default)]
pub struct TracingStatistics {
    pub logs_processed: u64,
    pub logs_pending: u64,
    pub distributed_tracing_enabled: bool,
    pub log_aggregation_enabled: bool,
    pub active_spans: u32,
    pub completed_traces: u64,
}
// ==================== SECTION ====================

/// Builder for tracing configuration
pub struct TracingConfigBuilder {
    config: TracingConfig,
}
impl TracingConfigBuilder {
    /// Create a new configuration builder
    pub fn new() -> Self {
        Self {
            config: TracingConfig::default(),
        }
    }

    /// Set log level
    #[must_use]
    pub fn level(mut self, level: &str) -> Self {
        self.config.level = level.to_string();
        self
    }

    /// Enable/disable console logging
    #[must_use]
    pub fn console(mut self, enabled: bool) -> Self {
        self.config.console_enabled = enabled;
        self
    }

    /// Enable/disable file logging
    #[must_use]
    pub fn file(mut self, enabled: bool) -> Self {
        self.config.file_enabled = enabled;
        self
    }

    /// Set log file path
        self.config.file_path = Some(path.into());
        self
    }

    /// Enable/disable JSON formatting
    #[must_use]
    pub fn json_format(mut self, enabled: bool) -> Self {
        self.config.json_format = enabled;
        self
    }

    /// Enable/disable distributed tracing
    #[must_use]
    pub fn distributed_tracing(mut self, enabled: bool) -> Self {
        self.config.distributed_tracing = enabled;
        self
    }

    /// Set Jaeger endpoint
    #[must_use]
    pub fn jaeger_endpoint(mut self, endpoint: &str) -> Self {
        self.config.jaeger_endpoint = Some(endpoint.to_string());
        self
    }

    /// Add custom field
    #[must_use]
    pub fn custom_field(mut self, key: &str, value: &str) -> Self {
        self.config.custom_fields.insert(key.to_string(), value.to_string());
        self
    }

    /// Build the configuration
    pub fn build(self) -> TracingConfig {
        self.config
    }
}

impl Default for TracingConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== SECTION ====================

/// Initialize tracing with default configuration
pub async fn init_default_tracing() -> Result<TracingManager> {
    let config = TracingConfig::default();
    let mut manager = TracingManager::new(config);
    manager.initialize().await?;
    Ok(manager)
}
/// Initialize tracing for production environment
pub async fn init_production_tracing() -> Result<TracingManager> {
    let config = TracingConfigBuilder::new()
        .level("info")
        .console(false)
        .file(true)
        .file_path("logs/nestgate-production.log")
        .json_format(true)
        .distributed_tracing(true)
        .build();
    let mut manager = TracingManager::new(config);
    manager.initialize().await?;
    Ok(manager)
}

/// Initialize tracing for development environment
pub async fn init_development_tracing() -> Result<TracingManager> {
    let config = TracingConfigBuilder::new()
        .level("debug")
        .console(true)
        .file(true)
        .json_format(false)
        .distributed_tracing(false)
        .build();
    let mut manager = TracingManager::new(config);
    manager.initialize().await?;
    Ok(manager)
} 