//! Tracing and Logging Setup
//!
//! Comprehensive tracing and logging system for NestGate including structured
//! logging, log aggregation, distributed tracing, and integration with external
//! logging systems like ELK stack, Loki, and Jaeger.

use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info, warn};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

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
    pub file_path: Option<PathBuf>,
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
            file_path: Some(PathBuf::from("logs/nestgate.log")),
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

/// External log destination configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogDestination {
    /// Elasticsearch destination
    Elasticsearch {
        url: String,
        index: String,
        auth: Option<ElasticsearchAuth>,
    },
    /// Loki destination
    Loki {
        url: String,
        labels: HashMap<String, String>,
        auth: Option<LokiAuth>,
    },
    /// Syslog destination
    Syslog {
        host: String,
        port: u16,
        facility: String,
    },
    /// HTTP webhook destination
    Webhook {
        url: String,
        method: String,
        headers: HashMap<String, String>,
    },
}

/// Elasticsearch authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElasticsearchAuth {
    Basic { username: String, password: String },
    ApiKey { key: String },
}

/// Loki authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LokiAuth {
    Basic { username: String, password: String },
    Bearer { token: String },
}

/// Log retention configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRetentionConfig {
    /// Maximum age of log files
    pub max_age: Duration,
    /// Maximum size of log files (in bytes)
    pub max_size: u64,
    /// Maximum number of log files to keep
    pub max_files: usize,
    /// Enable log compression
    pub compress: bool,
}

impl Default for LogRetentionConfig {
    fn default() -> Self {
        Self {
            max_age: Duration::from_secs(30 * 24 * 3600), // 30 days
            max_size: 100 * 1024 * 1024,                  // 100 MB
            max_files: 10,
            compress: true,
        }
    }
}

/// Structured log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Log timestamp
    pub timestamp: SystemTime,
    /// Log level
    pub level: String,
    /// Log message
    pub message: String,
    /// Source module/file
    pub module: Option<String>,
    /// Source line number
    pub line: Option<u32>,
    /// Trace ID for distributed tracing
    pub trace_id: Option<String>,
    /// Span ID for distributed tracing
    pub span_id: Option<String>,
    /// Custom fields
    pub fields: HashMap<String, serde_json::Value>,
    /// Service name
    pub service: String,
    /// Service version
    pub version: String,
    /// Host/instance information
    pub host: String,
}

/// Trace context for distributed tracing
#[derive(Debug, Clone)]
pub struct TraceContext {
    /// Trace ID
    pub trace_id: String,
    /// Span ID
    pub span_id: String,
    /// Parent span ID
    pub parent_span_id: Option<String>,
    /// Trace flags
    pub flags: u8,
}

impl Default for TraceContext {
    fn default() -> Self {
        Self::new()
    }
}

impl TraceContext {
    /// Create new trace context
    pub fn new() -> Self {
        Self {
            trace_id: generate_trace_id(),
            span_id: generate_span_id(),
            parent_span_id: None,
            flags: 0,
        }
    }

    /// Create child span context
    pub fn child(&self) -> Self {
        Self {
            trace_id: self.trace_id.clone(),
            span_id: generate_span_id(),
            parent_span_id: Some(self.span_id.clone()),
            flags: self.flags,
        }
    }
}

/// Log aggregator for batching and forwarding logs
pub struct LogAggregator {
    /// Configuration
    config: LogAggregationConfig,
    /// Log buffer
    buffer: Arc<RwLock<Vec<LogEntry>>>,
    /// HTTP client for external destinations
    client: reqwest::Client,
    /// Shutdown channel
    shutdown_tx: Option<mpsc::Sender<()>>,
}

impl LogAggregator {
    /// Create new log aggregator
    pub fn new(config: LogAggregationConfig) -> Self {
        info!(
            "📋 Initializing log aggregator with {} destinations",
            config.destinations.len()
        );

        Self {
            config,
            buffer: Arc::new(RwLock::new(Vec::new())),
            client: reqwest::Client::new(),
            shutdown_tx: None,
        }
    }

    /// Start log aggregation background task
    pub async fn start(&mut self) -> Result<()> {
        if !self.config.enabled {
            debug!("Log aggregation disabled");
            return Ok(());
        }

        let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        let buffer = self.buffer.clone();
        let config = self.config.clone();
        let client = self.client.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(config.flush_interval);

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if let Err(e) = Self::flush_logs(&buffer, &config, &client).await {
                            error!("Failed to flush logs: {}", e);
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        info!("Log aggregator shutting down");
                        // Final flush
                        if let Err(e) = Self::flush_logs(&buffer, &config, &client).await {
                            error!("Failed to flush logs on shutdown: {}", e);
                        }
                        break;
                    }
                }
            }
        });

        info!("✅ Log aggregator started");
        Ok(())
    }

    /// Add log entry to buffer
    pub async fn add_log(&self, entry: LogEntry) {
        if !self.config.enabled {
            return;
        }

        let mut buffer = self.buffer.write().await;
        buffer.push(entry);

        // Flush if buffer is full
        if buffer.len() >= self.config.buffer_size {
            drop(buffer); // Release write lock
            if let Err(e) = Self::flush_logs(&self.buffer, &self.config, &self.client).await {
                error!("Failed to flush full buffer: {}", e);
            }
        }
    }

    /// Flush logs to external destinations
    async fn flush_logs(
        buffer: &Arc<RwLock<Vec<LogEntry>>>,
        config: &LogAggregationConfig,
        client: &reqwest::Client,
    ) -> Result<()> {
        let logs = {
            let mut buffer_guard = buffer.write().await;
            if buffer_guard.is_empty() {
                return Ok(());
            }
            let logs = buffer_guard.clone();
            buffer_guard.clear();
            logs
        };

        debug!(
            "Flushing {} log entries to {} destinations",
            logs.len(),
            config.destinations.len()
        );

        for destination in &config.destinations {
            if let Err(e) = Self::send_to_destination(&logs, destination, client).await {
                error!("Failed to send logs to destination: {}", e);
            }
        }

        Ok(())
    }

    /// Send logs to specific destination
    async fn send_to_destination(
        logs: &[LogEntry],
        destination: &LogDestination,
        client: &reqwest::Client,
    ) -> Result<()> {
        match destination {
            LogDestination::Elasticsearch { url, index, auth } => {
                Self::send_to_elasticsearch(logs, url, index, auth, client).await
            }
            LogDestination::Loki { url, labels, auth } => {
                Self::send_to_loki(logs, url, labels, auth, client).await
            }
            LogDestination::Syslog {
                host,
                port,
                facility,
            } => Self::send_to_syslog(logs, host, *port, facility).await,
            LogDestination::Webhook {
                url,
                method,
                headers,
            } => Self::send_to_webhook(logs, url, method, headers, client).await,
        }
    }

    /// Send logs to Elasticsearch
    async fn send_to_elasticsearch(
        logs: &[LogEntry],
        url: &str,
        index: &str,
        auth: &Option<ElasticsearchAuth>,
        client: &reqwest::Client,
    ) -> Result<()> {
        let bulk_url = format!("{url}/_bulk");
        let mut bulk_body = String::new();

        for log in logs {
            let index_line = serde_json::json!({
                "index": {
                    "_index": index,
                    "_type": "_doc"
                }
            });
            if let Ok(index_json) = serde_json::to_string(&index_line) {
                bulk_body.push_str(&index_json);
                bulk_body.push('\n');
                if let Ok(log_json) = serde_json::to_string(log) {
                    bulk_body.push_str(&log_json);
                } else {
                    tracing::warn!("Failed to serialize log entry, skipping");
                    continue;
                }
            } else {
                tracing::warn!("Failed to serialize index line, skipping log entry");
                continue;
            }
            bulk_body.push('\n');
        }

        let mut request = client
            .post(&bulk_url)
            .header("Content-Type", "application/x-ndjson")
            .body(bulk_body);

        if let Some(auth) = auth {
            request = match auth {
                ElasticsearchAuth::Basic { username, password } => {
                    request.basic_auth(username, Some(password))
                }
                ElasticsearchAuth::ApiKey { key } => {
                    request.header("Authorization", format!("ApiKey {key}"))
                }
            };
        }

        let response = request.send().await.map_err(|e| NestGateError::Internal {
            message: format!("Failed to send to Elasticsearch: {e}"),
            location: Some(file!().to_string()),
            debug_info: None,
            is_bug: false,
        })?;

        if response.status().is_success() {
            debug!("✅ Sent {} logs to Elasticsearch", logs.len());
        } else {
            warn!(
                "Failed to send logs to Elasticsearch: {}",
                response.status()
            );
        }

        Ok(())
    }

    /// Send logs to Loki
    async fn send_to_loki(
        logs: &[LogEntry],
        url: &str,
        labels: &HashMap<String, String>,
        auth: &Option<LokiAuth>,
        client: &reqwest::Client,
    ) -> Result<()> {
        let push_url = format!("{url}/loki/api/v1/push");

        let streams: Vec<serde_json::Value> = logs
            .iter()
            .map(|log| {
                let mut stream_labels = labels.clone();
                stream_labels.insert("level".to_string(), log.level.clone());
                stream_labels.insert("service".to_string(), log.service.clone());

                let _label_string = stream_labels
                    .iter()
                    .map(|(k, v)| format!("{k}=\"{v}\""))
                    .collect::<Vec<_>>()
                    .join(",");

                let timestamp_ns = log
                    .timestamp
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos()
                    .to_string();

                serde_json::json!({
                    "stream": stream_labels,
                    "values": [[timestamp_ns, log.message]]
                })
            })
            .collect();

        let payload = serde_json::json!({
            "streams": streams
        });

        let mut request = client
            .post(&push_url)
            .header("Content-Type", "application/json")
            .json(&payload);

        if let Some(auth) = auth {
            request = match auth {
                LokiAuth::Basic { username, password } => {
                    request.basic_auth(username, Some(password))
                }
                LokiAuth::Bearer { token } => {
                    request.header("Authorization", format!("Bearer {token}"))
                }
            };
        }

        let response = request.send().await.map_err(|e| NestGateError::Internal {
            message: format!("Failed to send to Loki: {e}"),
            location: Some(file!().to_string()),
            debug_info: None,
            is_bug: false,
        })?;

        if response.status().is_success() {
            debug!("✅ Sent {} logs to Loki", logs.len());
        } else {
            warn!("Failed to send logs to Loki: {}", response.status());
        }

        Ok(())
    }

    /// Send logs to Syslog
    async fn send_to_syslog(
        logs: &[LogEntry],
        host: &str,
        port: u16,
        facility: &str,
    ) -> Result<()> {
        // Syslog integration implementation
        debug!(
            "Sending {} logs to syslog at {}:{} (facility: {})",
            logs.len(),
            host,
            port,
            facility
        );

        // Format logs for syslog RFC 5424 format
        for log in logs {
            let syslog_message = format!(
                "<{}>{} {} {} {} {} - {}",
                Self::get_syslog_priority(&log.level, facility),
                chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ"),
                std::env::var("HOSTNAME").unwrap_or_else(|_| "nestgate".to_string()),
                "nestgate",
                std::process::id(),
                log.message.as_str(),
                log.message
            );

            // In a real implementation, this would send via UDP/TCP to syslog daemon
            // For now, we log the formatted message
            tracing::info!(target: "syslog", "{}", syslog_message);
        }

        Ok(())
    }

    /// Get syslog priority value (facility * 8 + severity)
    fn get_syslog_priority(level: &str, facility: &str) -> u8 {
        let facility_code = match facility {
            "user" => 1,
            "daemon" => 3,
            "local0" => 16,
            "local1" => 17,
            _ => 16, // default to local0
        };

        let severity = match level {
            "ERROR" => 3,
            "WARN" => 4,
            "INFO" => 6,
            "DEBUG" => 7,
            _ => 6, // default to info
        };

        facility_code * 8 + severity
    }

    /// Send logs to webhook
    async fn send_to_webhook(
        logs: &[LogEntry],
        url: &str,
        method: &str,
        headers: &HashMap<String, String>,
        client: &reqwest::Client,
    ) -> Result<()> {
        let payload = serde_json::json!({
            "logs": logs,
            "count": logs.len(),
            "timestamp": SystemTime::now()
        });

        let mut request = match method.to_uppercase().as_str() {
            "POST" => client.post(url).json(&payload),
            "PUT" => client.put(url).json(&payload),
            _ => {
                return Err(NestGateError::Internal {
                    message: format!("Unsupported HTTP method for webhook: {method}"),
                    location: Some(file!().to_string()),
                    debug_info: None,
                    is_bug: false,
                })
            }
        };

        for (key, value) in headers {
            request = request.header(key, value);
        }

        let response = request.send().await.map_err(|e| NestGateError::Internal {
            message: format!("Failed to send to webhook: {e}"),
            location: Some(file!().to_string()),
            debug_info: None,
            is_bug: false,
        })?;

        if response.status().is_success() {
            debug!("✅ Sent {} logs to webhook", logs.len());
        } else {
            warn!("Failed to send logs to webhook: {}", response.status());
        }

        Ok(())
    }

    /// Shutdown log aggregator
    pub async fn shutdown(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(()).await;
        }
    }
}

/// Initialize tracing with comprehensive configuration
pub fn init_tracing(config: TracingConfig) -> Result<LogAggregator> {
    let env_filter_str = std::env::var("RUST_LOG").unwrap_or_else(|_| config.level.clone());

    let mut layers = Vec::new();

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
                std::fs::create_dir_all(parent).map_err(|e| NestGateError::Internal {
                    message: format!("Failed to create log directory: {e}"),
                    location: Some(file!().to_string()),
                    debug_info: None,
                    is_bug: false,
                })?;
            }

            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)
                .map_err(|e| NestGateError::Internal {
                    message: format!("Failed to open log file: {e}"),
                    location: Some(file!().to_string()),
                    debug_info: None,
                    is_bug: false,
                })?;

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

/// Generate random trace ID
fn generate_trace_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!("{:016x}{:016x}", rng.gen::<u64>(), rng.gen::<u64>())
}

/// Generate random span ID
fn generate_span_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!("{:016x}", rng.gen::<u64>())
}

/// Log retention manager
pub struct LogRetentionManager {
    config: LogRetentionConfig,
}

impl LogRetentionManager {
    /// Create new retention manager
    pub fn new(config: LogRetentionConfig) -> Self {
        Self { config }
    }

    /// Clean up old log files
    pub async fn cleanup_logs(&self, log_dir: &PathBuf) -> Result<()> {
        if !log_dir.exists() {
            return Ok(());
        }

        let mut files_to_remove = Vec::new();
        let mut total_size = 0u64;

        // Scan log files
        let mut entries =
            tokio::fs::read_dir(log_dir)
                .await
                .map_err(|e| NestGateError::Internal {
                    message: format!("Failed to read log directory: {e}"),
                    location: Some(file!().to_string()),
                    debug_info: None,
                    is_bug: false,
                })?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| NestGateError::Internal {
                message: format!("Failed to read directory entry: {e}"),
                location: Some(file!().to_string()),
                debug_info: None,
                is_bug: false,
            })?
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let metadata = entry
                .metadata()
                .await
                .map_err(|e| NestGateError::Internal {
                    message: format!("Failed to read file metadata: {e}"),
                    location: Some(file!().to_string()),
                    debug_info: None,
                    is_bug: false,
                })?;

            let modified = metadata.modified().map_err(|e| NestGateError::Internal {
                message: format!("Failed to read file modification time: {e}"),
                location: Some(file!().to_string()),
                debug_info: None,
                is_bug: false,
            })?;

            let age = SystemTime::now()
                .duration_since(modified)
                .unwrap_or_default();

            // Check if file is too old
            if age > self.config.max_age {
                files_to_remove.push(path);
                continue;
            }

            // Check if file is too large
            if metadata.len() > self.config.max_size {
                files_to_remove.push(path);
                continue;
            }

            total_size += metadata.len();
        }

        // Remove old files
        for file_path in files_to_remove {
            if let Err(e) = tokio::fs::remove_file(&file_path).await {
                warn!("Failed to remove old log file {:?}: {}", file_path, e);
            } else {
                debug!("Removed old log file: {:?}", file_path);
            }
        }

        debug!("Log cleanup completed, total size: {} bytes", total_size);
        Ok(())
    }

    /// Start log retention background task
    pub fn start_retention_task(&self, log_dir: PathBuf) -> tokio::task::JoinHandle<()> {
        let manager = self.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Check every hour

            loop {
                interval.tick().await;

                if let Err(e) = manager.cleanup_logs(&log_dir).await {
                    error!("Failed to cleanup logs: {}", e);
                }
            }
        })
    }
}

impl Clone for LogRetentionManager {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
        }
    }
}

/// Structured logging macros
#[macro_export]
macro_rules! log_with_context {
    ($level:expr, $context:expr, $msg:expr) => {
        match $level {
            "error" => tracing::error!(
                trace_id = $context.as_ref().map(|c| c.trace_id.as_str()).unwrap_or(""),
                span_id = $context.as_ref().map(|c| c.span_id.as_str()).unwrap_or(""),
                "{}",
                $msg
            ),
            "warn" => tracing::warn!(
                trace_id = $context.as_ref().map(|c| c.trace_id.as_str()).unwrap_or(""),
                span_id = $context.as_ref().map(|c| c.span_id.as_str()).unwrap_or(""),
                "{}",
                $msg
            ),
            "info" => tracing::info!(
                trace_id = $context.as_ref().map(|c| c.trace_id.as_str()).unwrap_or(""),
                span_id = $context.as_ref().map(|c| c.span_id.as_str()).unwrap_or(""),
                "{}",
                $msg
            ),
            "debug" => tracing::debug!(
                trace_id = $context.as_ref().map(|c| c.trace_id.as_str()).unwrap_or(""),
                span_id = $context.as_ref().map(|c| c.span_id.as_str()).unwrap_or(""),
                "{}",
                $msg
            ),
            _ => tracing::trace!(
                trace_id = $context.as_ref().map(|c| c.trace_id.as_str()).unwrap_or(""),
                span_id = $context.as_ref().map(|c| c.span_id.as_str()).unwrap_or(""),
                "{}",
                $msg
            ),
        }
    };
}
