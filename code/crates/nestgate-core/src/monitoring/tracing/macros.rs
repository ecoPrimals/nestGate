// **TRACING MACROS AND UTILITIES**
//! Macros functionality and utilities.
// Structured logging macros and utility functions.
// Extracted from tracing_setup.rs for file size compliance.

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
/// Log an error with structured context
#[macro_export]
macro_rules! log_error {
    ($context:expr, $msg:expr) => {
        $crate::log_with_context!("error", $context, $msg)
    };
}
/// Log a warning with structured context
#[macro_export]
macro_rules! log_warn {
    ($context:expr, $msg:expr) => {
        $crate::log_with_context!("warn", $context, $msg)
    };
}
/// Log info with structured context
#[macro_export]
macro_rules! log_info {
    ($context:expr, $msg:expr) => {
        $crate::log_with_context!("info", $context, $msg)
    };
}
/// Log debug with structured context
#[macro_export]
macro_rules! log_debug {
    ($context:expr, $msg:expr) => {
        $crate::log_with_context!("debug", $context, $msg)
    };
} 