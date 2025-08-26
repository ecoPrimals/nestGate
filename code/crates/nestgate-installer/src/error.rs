use thiserror::Error;

/// Installer-specific error types
#[derive(Debug, Error)]
pub enum InstallerError {
    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Installation failed: {0}")]
    Installation(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("System error: {0}")]
    System(String),

    #[error("NestGate core error: {0}")]
    Core(#[from] NestGateError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for installer operations
pub type InstallerResult<T> = std::result::Result<T, InstallerError>;
