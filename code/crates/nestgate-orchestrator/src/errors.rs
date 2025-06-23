//! Error types for the orchestrator

use thiserror::Error;

/// Orchestrator error types
#[derive(Debug, Error)]
pub enum OrchestratorError {
    #[error("Service error: {0}")]
    ServiceError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    
    #[error("Service already exists: {0}")]
    ServiceAlreadyExists(String),
    
    #[error("Port allocation error: {0}")]
    PortAllocationError(String),
    
    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),
    
    #[error("Timeout error: {0}")]
    TimeoutError(String),
    
    #[error("Circuit breaker open")]
    CircuitBreakerOpen,
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Security error: {0}")]
    SecurityError(#[from] crate::security::SecurityError),
}

/// Result type for orchestrator operations
pub type Result<T> = std::result::Result<T, OrchestratorError>;

/// Service error types
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Service startup failed: {0}")]
    StartupFailed(String),
    
    #[error("Service shutdown failed: {0}")]
    ShutdownFailed(String),
    
    #[error("Service unhealthy: {0}")]
    Unhealthy(String),
    
    #[error("Service dependency missing: {0}")]
    DependencyMissing(String),
    
    #[error("Service configuration invalid: {0}")]
    InvalidConfiguration(String),
}

/// Communication error types
#[derive(Debug, Error)]
pub enum CommunicationError {
    #[error("WebSocket connection failed: {0}")]
    WebSocketConnectionFailed(String),
    
    #[error("Message serialization failed: {0}")]
    MessageSerializationFailed(String),
    
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    
    #[error("Connection timeout")]
    ConnectionTimeout,
    
    #[error("Authentication failed")]
    AuthenticationFailed,
} 