//! File System Monitor Error Types
//!
//! Provides unified error handling for the file system monitor with integration
//! to the NestGate core error system.

use nestgate_core::smart_abstractions::prelude::*;
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

impl SmartDefault for FsMonitorError {
    fn smart_default() -> Self {
        Self::Configuration {
            message: "Default configuration error".to_string(),
        }
    }
}

impl From<FsMonitorError> for nestgate_core::NestGateError {
    fn from(err: FsMonitorError) -> Self {
        match err {
            FsMonitorError::WatcherInit { message } => nestgate_core::NestGateError::Internal {
                message: format!("FS Monitor watcher init: {message}"),
                location: Some("nestgate-fsmonitor".to_string()),
                debug_info: None,
                is_bug: false,
            },
            FsMonitorError::WatchPath { path, message } => nestgate_core::NestGateError::Internal {
                message: format!("FS Monitor watch path {path:?}: {message}"),
                location: Some("nestgate-fsmonitor".to_string()),
                debug_info: Some(format!("path: {path:?}")),
                is_bug: false,
            },
            FsMonitorError::EventProcessing { message } => nestgate_core::NestGateError::Internal {
                message: format!("FS Monitor event processing: {message}"),
                location: Some("nestgate-fsmonitor".to_string()),
                debug_info: None,
                is_bug: false,
            },
            FsMonitorError::Configuration { message } => nestgate_core::NestGateError::Validation {
                field: "fsmonitor_config".to_string(),
                message: format!("FS Monitor config: {message}"),
                expected: Some("valid filesystem monitor configuration".to_string()),
                current_value: Some("invalid configuration".to_string()),
                user_error: true,
            },
            FsMonitorError::Handler { message } => nestgate_core::NestGateError::Internal {
                message: format!("FS Monitor handler: {message}"),
                location: Some("nestgate-fsmonitor".to_string()),
                debug_info: None,
                is_bug: false,
            },
        }
    }
}

/// Result type for file system monitor operations
pub type Result<T> = std::result::Result<T, FsMonitorError>;

// From trait implementations for error conversion
impl From<notify::Error> for FsMonitorError {
    fn from(err: notify::Error) -> Self {
        FsMonitorError::WatcherInit {
            message: format!("Notify error: {}", err),
        }
    }
}

impl From<std::io::Error> for FsMonitorError {
    fn from(err: std::io::Error) -> Self {
        FsMonitorError::EventProcessing {
            message: format!("IO error: {}", err),
        }
    }
}

impl From<nestgate_core::error::NestGateError> for FsMonitorError {
    fn from(err: nestgate_core::error::NestGateError) -> Self {
        FsMonitorError::EventProcessing {
            message: format!("NestGate error: {}", err),
        }
    }
}

/// Helper function to create watcher initialization errors
pub fn watcher_init_error(message: &str) -> FsMonitorError {
    FsMonitorError::WatcherInit {
        message: message.to_string(),
    }
}

/// Helper function to create watch path errors
pub fn watch_path_error(path: PathBuf, message: &str) -> FsMonitorError {
    FsMonitorError::WatchPath {
        path,
        message: message.to_string(),
    }
}

/// Helper function to create event processing errors
pub fn event_processing_error(message: &str) -> FsMonitorError {
    FsMonitorError::EventProcessing {
        message: message.to_string(),
    }
}

/// Helper function to create configuration errors
pub fn configuration_error(message: &str) -> FsMonitorError {
    FsMonitorError::Configuration {
        message: message.to_string(),
    }
}

/// Helper function to create handler errors
pub fn handler_error(message: &str) -> FsMonitorError {
    FsMonitorError::Handler {
        message: message.to_string(),
    }
}
