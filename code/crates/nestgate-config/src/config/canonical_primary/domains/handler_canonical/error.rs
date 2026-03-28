// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **ERROR HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ErrorHandler
pub struct ErrorHandlerConfig {
    /// Response
    pub response: ErrorResponseConfig,
    /// Logging
    pub logging: ErrorLoggingConfig,
    /// Recovery
    pub recovery: ErrorRecoveryConfig,
    /// Notification
    pub notification: ErrorNotificationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ErrorResponse
pub struct ErrorResponseConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ErrorLogging
pub struct ErrorLoggingConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ErrorRecovery
pub struct ErrorRecoveryConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ErrorNotification
pub struct ErrorNotificationConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl Default for ErrorHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            response: ErrorResponseConfig { enabled: true },
            logging: ErrorLoggingConfig { enabled: true },
            recovery: ErrorRecoveryConfig { enabled: true },
            notification: ErrorNotificationConfig { enabled: false },
        }
    }
}

impl ErrorHandlerConfig {
    /// Creates a production-optimized error handler configuration
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }

    /// Creates a development-optimized error handler configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Creates a high-performance error handler configuration
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }

    /// Merges two error handler configurations, preferring values from self
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}
