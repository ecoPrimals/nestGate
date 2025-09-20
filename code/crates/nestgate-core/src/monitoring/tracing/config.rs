//! # Tracing Configuration
//! Configuration types and utilities.
// Configuration structures for tracing, logging, and log aggregation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

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