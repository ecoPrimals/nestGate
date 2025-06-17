/*!
 * Error types for the Port Manager
 */

use std::io;
use thiserror::Error;

/// Custom result type for the Port Manager
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error enum for the Port Manager
#[derive(Error, Debug)]
pub enum Error {
    /// Error during port allocation
    #[error("Port allocation error: {0}")]
    PortAllocation(String),
    
    /// No available ports in range
    #[error("No available ports in range {start}-{end}")]
    NoAvailablePorts { start: u16, end: u16 },
    
    /// Service not found
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    
    /// Service already exists
    #[error("Service already registered: {0}")]
    ServiceAlreadyExists(String),
    
    /// Process error
    #[error("Process error: {0}")]
    Process(String),
    
    /// Process termination error
    #[error("Process termination error: {0}")]
    ProcessTermination(String),
    
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// YAML serialization/deserialization error
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),
    
    /// API error
    #[error("API error: {0}")]
    Api(String),
    
    /// Dependency cycle detected
    #[error("Dependency cycle detected: {0}")]
    DependencyCycle(String),
    
    /// Service startup error
    #[error("Service startup error: {0}")]
    ServiceStartup(String),
    
    /// Service shutdown error
    #[error("Service shutdown error: {0}")]
    ServiceShutdown(String),
    
    /// Health check error
    #[error("Health check error: {0}")]
    HealthCheck(String),
    
    /// Permission error
    #[error("Permission error: {0}")]
    Permission(String),
    
    /// General error
    #[error("{0}")]
    General(String),
} 