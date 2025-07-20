//! Network layer errors

use thiserror::Error;

/// Network layer errors
#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Internal network error: {0}")]
    Internal(String),

    #[error("Connection timeout")]
    Timeout,

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Service discovery failed: {0}")]
    DiscoveryFailed(String),

    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("NestGate error: {0}")]
    NestGate(#[from] nestgate_core::NestGateError),
}

/// Network layer result type
pub type Result<T> = std::result::Result<T, NetworkError>;
