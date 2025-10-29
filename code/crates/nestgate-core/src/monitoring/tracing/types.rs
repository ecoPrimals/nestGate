//! # Tracing Types
//! Type definitions and data structures.
// Core data structures for logging and distributed tracing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

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

/// Generate random trace ID
pub fn generate_trace_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!("{:016x}{:016x}"), rng.gen::<u64>())
}
/// Generate random span ID
pub fn generate_span_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!("{:016x}"))
} 