// **ERROR HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandlerConfig {
    pub response: ErrorResponseConfig,
    pub logging: ErrorLoggingConfig,
    pub recovery: ErrorRecoveryConfig,
    pub notification: ErrorNotificationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponseConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLoggingConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRecoveryConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorNotificationConfig {
    pub enabled: bool,
}

impl Default for ErrorHandlerConfig {
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
    #[must_use]
    pub const fn production_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn high_performance() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
    pub const fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
