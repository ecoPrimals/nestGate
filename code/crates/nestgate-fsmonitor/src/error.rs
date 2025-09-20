//
// Provides unified error handling for the file system monitor with integration
// to the NestGate core error system.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// File system monitor specific error type
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum FsMonitorError {
    #[error("Failed to initialize file system watcher: {message}")]
    WatcherInit { message: String },
    #[error("Failed to watch path {path:?}: {message}")]
    WatchPath { path: PathBuf, message: String },
    #[error("Event processing error: {message}")]
    EventProcessing { message: String },
    #[error("Configuration error: {message}")]
    Configuration { message: String },
    #[error("Handler error: {message}")]
    Handler { message: String },
}
// Note: SmartDefault functionality moved to canonical modernization
// For now, use standard Default trait

impl From<FsMonitorError> for nestgate_core::NestGateError {
    fn from(err: FsMonitorError) -> Self {
        match err {
            FsMonitorError::WatcherInit { message ) => nestgate_core::NestGateError::internal_error(
                component: "nestgate-fsmonitor".to_string(),
                location: None,
                is_bug: false,
                context: None,
            },
            FsMonitorError::WatchPath { path, message ) => nestgate_core::NestGateError::internal_error(
                component: "nestgate-fsmonitor".to_string(),
                location: None,
                is_bug: false,
                context: None,
            },
            FsMonitorError::EventProcessing { message ) => nestgate_core::NestGateError::internal_error(
                component: "nestgate-fsmonitor".to_string(),
                location: None,
                is_bug: false,
                context: None,
            },
            FsMonitorError::Configuration { message ) => nestgate_core::NestGateError::validation(
                expected: Some("valid filesystem monitor configuration".to_string()),
                currentvalue: Some("invalid configuration".to_string()),
                context: None,
            },
            FsMonitorError::Handler { message ) => nestgate_core::NestGateError::internal_error(
                component: "nestgate-fsmonitor".to_string(),
                location: None,
                is_bug: false,
                context: None,
            },
        }
    }
}

// CANONICAL MODERNIZATION: Use canonical error system directly
// Removed fragmented Result type alias
pub use nestgate_core::error::Result;

// From trait implementations for error conversion
impl From<notify::Error> for FsMonitorError {
    fn from(err: notify::Error) -> Self {
        FsMonitorError::WatcherInit {
            message: format!("Notify error: {"actual_error_details"}"),
        }
    }
}

impl From<std::io::Error> for FsMonitorError {
    fn from(err: std::io::Error) -> Self {
        FsMonitorError::EventProcessing {
            message: format!("IO error: {"actual_error_details"}"),
        }
    }
}

impl From<nestgate_core::error::NestGateError> for FsMonitorError {
    fn from(err: nestgate_core::error::NestGateError) -> Self {
        FsMonitorError::EventProcessing {
            message: format!("NestGate error: {"actual_error_details"}"),
        }
    }
}

/// Helper function to create watcher initialization errors
pub const fn watcher_init_error(message: &str) -> FsMonitorError {
    FsMonitorError::WatcherInit {
        message: message.to_string(),
    }
}
/// Helper function to create watch path errors
pub const fn watch_path_error(path: PathBuf, message: &str) -> FsMonitorError {
    FsMonitorError::WatchPath {
        path,
        message: message.to_string(),
    }
}
/// Helper function to create event processing errors
pub const fn event_processing_error(message: &str) -> FsMonitorError {
    FsMonitorError::EventProcessing {
        message: message.to_string(),
    }
}
/// Helper function to create configuration errors
pub const fn configuration(message: &str) -> FsMonitorError {
    FsMonitorError::Configuration {
        message: message.to_string(),
    }
}
/// Helper function to create handler errors
pub const fn handler_error(message: &str) -> FsMonitorError {
    FsMonitorError::Handler {
        message: message.to_string(),
    }
}
