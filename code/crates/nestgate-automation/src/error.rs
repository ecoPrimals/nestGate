// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// Error handling for the automation system
use nestgate_core::error::NestGateError;
use serde::{Deserialize, Serialize};
use std::fmt;
/// Automation-specific error types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Errors that can occur during Automation operations
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
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AnalysisError(msg) => write!(f, "Analysis error: {msg}"),
            Self::PredictionError(msg) => write!(f, "Prediction error: {msg}"),
            Self::ConfigError(msg) => write!(f, "Configuration error: {msg}"),
            Self::IoError(msg) => write!(f, "I/O error: {msg}"),
            Self::CoreError(err) => write!(f, "Core system error: {err}"),
        }
    }
}

impl std::error::Error for AutomationError {}

impl From<NestGateError> for AutomationError {
    /// From
    fn from(err: NestGateError) -> Self {
        Self::CoreError(err)
    }
}

impl From<std::io::Error> for AutomationError {
    /// From
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}

// Use canonical Result type instead of custom alias
// CANONICAL MODERNIZATION: Use nestgate_core::Result instead of custom AutomationResult
pub use nestgate_core::Result;

/// Helper function to create automation errors with canonical config source
#[must_use]
pub fn automation_error(message: String) -> AutomationError {
    AutomationError::CoreError(NestGateError::automation(&message))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automation_error_variants() {
        let variants = [
            AutomationError::AnalysisError("test".to_string()),
            AutomationError::PredictionError("test".to_string()),
            AutomationError::ConfigError("test".to_string()),
            AutomationError::IoError("test".to_string()),
        ];
        assert_eq!(variants.len(), 4);
    }

    #[test]
    fn test_analysis_error_display() {
        let err = AutomationError::AnalysisError("failed to analyze".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Analysis error"));
        assert!(display.contains("failed to analyze"));
    }

    #[test]
    fn test_prediction_error_display() {
        let err = AutomationError::PredictionError("prediction failed".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Prediction error"));
        assert!(display.contains("prediction failed"));
    }

    #[test]
    fn test_config_error_display() {
        let err = AutomationError::ConfigError("invalid config".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Configuration error"));
        assert!(display.contains("invalid config"));
    }

    #[test]
    fn test_io_error_display() {
        let err = AutomationError::IoError("file not found".to_string());
        let display = format!("{}", err);
        assert!(display.contains("I/O error"));
        assert!(display.contains("file not found"));
    }

    #[test]
    fn test_error_clone() {
        let err = AutomationError::AnalysisError("test".to_string());
        let cloned = err.clone();
        assert_eq!(format!("{}", err), format!("{}", cloned));
    }

    #[test]
    fn test_error_debug() {
        let err = AutomationError::PredictionError("test".to_string());
        let debug = format!("{:?}", err);
        assert!(debug.contains("PredictionError"));
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let auto_err: AutomationError = io_err.into();
        match auto_err {
            AutomationError::IoError(_) => (),
            _ => panic!("Expected IoError variant"),
        }
    }

    #[test]
    fn test_error_is_send_sync() {
        /// Assert Send Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AutomationError>();
    }

    #[test]
    fn test_analysis_error_creation() {
        let err = AutomationError::AnalysisError("test analysis".to_string());
        if let AutomationError::AnalysisError(msg) = err {
            assert_eq!(msg, "test analysis");
        } else {
            panic!("Expected AnalysisError");
        }
    }

    #[test]
    fn test_prediction_error_creation() {
        let err = AutomationError::PredictionError("test prediction".to_string());
        if let AutomationError::PredictionError(msg) = err {
            assert_eq!(msg, "test prediction");
        } else {
            panic!("Expected PredictionError");
        }
    }

    #[test]
    fn test_config_error_creation() {
        let err = AutomationError::ConfigError("test config".to_string());
        if let AutomationError::ConfigError(msg) = err {
            assert_eq!(msg, "test config");
        } else {
            panic!("Expected ConfigError");
        }
    }

    #[test]
    fn test_io_error_creation() {
        let err = AutomationError::IoError("test io".to_string());
        if let AutomationError::IoError(msg) = err {
            assert_eq!(msg, "test io");
        } else {
            panic!("Expected IoError");
        }
    }

    #[test]
    fn test_error_trait_implementation() {
        let err = AutomationError::AnalysisError("test".to_string());
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_empty_error_messages() {
        let errors = [
            AutomationError::AnalysisError(String::new()),
            AutomationError::PredictionError(String::new()),
            AutomationError::ConfigError(String::new()),
            AutomationError::IoError(String::new()),
        ];
        assert_eq!(errors.len(), 4);
    }

    #[test]
    fn test_long_error_messages() {
        let long_msg = "a".repeat(1000);
        let err = AutomationError::AnalysisError(long_msg.clone());
        let display = format!("{}", err);
        assert!(display.contains(&long_msg));
    }

    #[test]
    fn test_special_characters_in_errors() {
        let msg = "error with special chars: !@#$%^&*()";
        let err = AutomationError::AnalysisError(msg.to_string());
        let display = format!("{}", err);
        assert!(display.contains("!@#$%^&*()"));
    }

    #[test]
    fn test_unicode_in_errors() {
        let msg = "错误消息 🚀";
        let err = AutomationError::ConfigError(msg.to_string());
        let display = format!("{}", err);
        assert!(display.contains("错误消息"));
        assert!(display.contains("🚀"));
    }

    #[test]
    fn test_serialization() {
        let err = AutomationError::AnalysisError("test".to_string());
        let serialized = serde_json::to_string(&err).expect("String operation failed");
        assert!(serialized.contains("AnalysisError"));
        assert!(serialized.contains("test"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"AnalysisError":"test message"}"#;
        let err: AutomationError =
            serde_json::from_str(json).expect("Failed to convert from string");
        if let AutomationError::AnalysisError(msg) = err {
            assert_eq!(msg, "test message");
        } else {
            panic!("Expected AnalysisError");
        }
    }

    #[test]
    fn test_round_trip_serialization() {
        let original = AutomationError::PredictionError("round trip test".to_string());
        let json = serde_json::to_string(&original).expect("String operation failed");
        let deserialized: AutomationError =
            serde_json::from_str(&json).expect("Failed to convert from string");
        assert_eq!(format!("{}", original), format!("{}", deserialized));
    }

    #[test]
    fn test_multiple_error_types() {
        let errors = [
            AutomationError::AnalysisError("error1".to_string()),
            AutomationError::PredictionError("error2".to_string()),
            AutomationError::ConfigError("error3".to_string()),
        ];

        for (i, err) in errors.iter().enumerate() {
            let display = format!("{}", err);
            assert!(display.contains(&format!("error{}", i + 1)));
        }
    }

    #[test]
    fn test_error_equality_after_clone() {
        let err1 = AutomationError::AnalysisError("test".to_string());
        let err2 = err1.clone();
        assert_eq!(format!("{}", err1), format!("{}", err2));
        assert_eq!(format!("{:?}", err1), format!("{:?}", err2));
    }

    #[test]
    fn test_nested_error_display() {
        let nestgate_err = NestGateError::configuration_error("field", "base error");
        let auto_err = AutomationError::CoreError(nestgate_err);
        let display = format!("{}", auto_err);
        assert!(display.contains("Core system error"));
    }

    #[test]
    fn test_from_nestgate_error() {
        let nestgate_err = NestGateError::configuration_error("field", "test error");
        let auto_err: AutomationError = nestgate_err.into();
        match auto_err {
            AutomationError::CoreError(_) => (),
            _ => panic!("Expected CoreError variant"),
        }
    }

    #[test]
    fn test_error_size() {
        use std::mem::size_of;
        // Ensure error type is reasonably sized
        assert!(size_of::<AutomationError>() < 1024);
    }

    #[test]
    fn test_all_variants_displayable() {
        let variants = [
            AutomationError::AnalysisError("a".to_string()),
            AutomationError::PredictionError("p".to_string()),
            AutomationError::ConfigError("c".to_string()),
            AutomationError::IoError("i".to_string()),
        ];

        for variant in variants {
            let display = format!("{}", variant);
            assert!(!display.is_empty());
            assert!(display.contains("error") || display.contains("Error"));
        }
    }

    #[test]
    fn test_all_variants_debuggable() {
        let variants = [
            AutomationError::AnalysisError("a".to_string()),
            AutomationError::PredictionError("p".to_string()),
            AutomationError::ConfigError("c".to_string()),
            AutomationError::IoError("i".to_string()),
        ];

        for variant in variants {
            let debug = format!("{:?}", variant);
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn round5_automation_error_from_nestgate_core_impl() {
        let core = NestGateError::internal_error("inner", "automation");
        let e: AutomationError = core.into();
        assert!(matches!(e, AutomationError::CoreError(_)));
        assert!(e.to_string().contains("Core system error"));
    }

    #[test]
    fn round5_automation_error_core_display_contains_nested() {
        let core = NestGateError::internal_error("nested", "mod");
        let e = AutomationError::CoreError(core);
        assert!(e.to_string().contains("nested"));
    }

    #[test]
    fn round5_automation_error_serde_roundtrip_prediction() {
        let e = AutomationError::PredictionError("p".to_string());
        let json = serde_json::to_string(&e).unwrap();
        let back: AutomationError = serde_json::from_str(&json).unwrap();
        assert_eq!(format!("{e:?}"), format!("{back:?}"));
    }

    #[test]
    fn round5_automation_error_serde_roundtrip_config() {
        let e = AutomationError::ConfigError("cfg".to_string());
        let json = serde_json::to_string(&e).unwrap();
        let back: AutomationError = serde_json::from_str(&json).unwrap();
        assert_eq!(format!("{e:?}"), format!("{back:?}"));
    }

    #[test]
    fn round5_automation_error_serde_roundtrip_io() {
        let e = AutomationError::IoError("io".to_string());
        let json = serde_json::to_string(&e).unwrap();
        let back: AutomationError = serde_json::from_str(&json).unwrap();
        assert_eq!(format!("{e:?}"), format!("{back:?}"));
    }

    #[test]
    fn round5_automation_error_serde_roundtrip_analysis() {
        let e = AutomationError::AnalysisError("a".to_string());
        let json = serde_json::to_string(&e).unwrap();
        let back: AutomationError = serde_json::from_str(&json).unwrap();
        assert_eq!(format!("{e:?}"), format!("{back:?}"));
    }
}
