// **MONITORING CONFIGURATION - CANONICAL MODERNIZED**
//! Configuration types and utilities.
// Configuration types and structures for the monitoring system.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Log aggregation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for LogAggregation
pub struct LogAggregationConfig {
    /// Enable log aggregation
    pub enabled: bool,
    /// Batch size for log shipping
    pub batch_size: usize,
    /// Flush interval for batched logs
    pub flush_interval_seconds: u64,
    /// Log destinations
    pub destinations: Vec<LogDestination>,
    /// Buffer size for log entries
    pub buffer_size: usize,
}
}
impl Default for LogAggregationConfig {
    /// Returns the default instance
    fn default() -> Self { Self {
            enabled: false,
            batch_size: 100,
            flush_interval_seconds: 10,
            destinations: vec![],
            buffer_size: 1000,
         }
         }
}
}

/// Log destination configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Logdestination
pub enum LogDestination {
    /// Local file destination
    ,
/// Elasticsearch destination
    Elasticsearch {
        url: String,
        index: String,
        auth: Option<ElasticsearchAuth>,
    }
    }
    ,
/// Loki destination
    Loki {
        url: String,
        auth: Option<LokiAuth>,
        labels: std::collections::HashMap<String, String>,
    }
    }
    ,
/// Syslog destination
    Syslog {
        host: String,
        port: u16,
        facility: String,
    }
    }
    ,
/// Custom HTTP endpoint
    Http {
        url: String,
        headers: std::collections::HashMap<String, String>,
        timeout_seconds: Option<u64>,
    }
    }
}
}
/// Elasticsearch authentication methods
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Elasticsearchauth
pub enum ElasticsearchAuth {
    /// Basic
    Basic { username: String, password: String  }
    /// Apikey
    ApiKey { b_key: String }
}
}
/// Loki authentication methods
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Lokiauth
pub enum LokiAuth {
    /// Basic
    Basic { username: String, password: String  }
    /// Bearertoken
    BearerToken { token: String }
}
}
/// Log retention configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for LogRetention
pub struct LogRetentionConfig {
    /// Enable log retention management
    pub enabled: bool,
    /// Maximum age of log files in days
    pub max_age_days: u32,
    /// Maximum size of log directory in MB
    pub max_size_mb: u64,
    /// Cleanup interval in hours
    pub cleanup_interval_hours: u64,
}
}
impl Default for LogRetentionConfig {
    /// Returns the default instance
    fn default() -> Self { Self {
            enabled: true,
            max_age_days: 30,
            max_size_mb: 1000, // 1GB
            cleanup_interval_hours: 24,
         }
         }
}
}