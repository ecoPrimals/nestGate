/// Error handling for the automation system
use nestgate_core::error::NestGateError;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Automation-specific error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationError {
    /// Analysis error
    AnalysisError(String),
    /// Prediction error
    PredictionError(String),
    /// Configuration error
    ConfigError(String),
    /// I/O error
    IoError(String),
    /// Core system error
    CoreError(NestGateError),
}

impl fmt::Display for AutomationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AutomationError::AnalysisError(msg) => write!(f, "Analysis error: {msg}"),
            AutomationError::PredictionError(msg) => write!(f, "Prediction error: {msg}"),
            AutomationError::ConfigError(msg) => write!(f, "Configuration error: {msg}"),
            AutomationError::IoError(msg) => write!(f, "I/O error: {msg}"),
            AutomationError::CoreError(err) => write!(f, "Core system error: {err}"),
        }
    }
}

impl std::error::Error for AutomationError {}

impl From<NestGateError> for AutomationError {
    fn from(err: NestGateError) -> Self {
        AutomationError::CoreError(err)
    }
}

impl From<std::io::Error> for AutomationError {
    fn from(err: std::io::Error) -> Self {
        AutomationError::IoError(err.to_string())
    }
}

// Use canonical Result type instead of custom alias
// CANONICAL MODERNIZATION: Use nestgate_core::Result instead of custom AutomationResult
pub use nestgate_core::Result;

/// Helper function to create automation errors with canonical config source
pub fn automation_error(message: String) -> AutomationError {
    AutomationError::CoreError(NestGateError::Configuration {
        message,
        config_source: nestgate_core::error::UnifiedConfigSource::Environment,
        field: None,
        suggested_fix: Some("Check automation configuration".to_string()),
    })
}
