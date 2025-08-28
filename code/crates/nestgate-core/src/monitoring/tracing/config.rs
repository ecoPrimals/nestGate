//! **TRACING CONFIGURATION**
//!
//! Configuration types and structures for the tracing system.
//! Extracted from tracing_setup.rs for file size compliance.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

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
    /// Create a new trace context with random IDs
    pub fn new() -> Self {
        Self {
            trace_id: uuid::Uuid::new_v4().to_string(),
            span_id: uuid::Uuid::new_v4().to_string(),
            parent_span_id: None,
            flags: 0,
        }
    }

    /// Create a child trace context
    pub fn child(&self) -> Self {
        Self {
            trace_id: self.trace_id.clone(),
            span_id: uuid::Uuid::new_v4().to_string(),
            parent_span_id: Some(self.span_id.clone()),
            flags: self.flags,
        }
    }
} 