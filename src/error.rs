//! NestGate error types
//!
//! Comprehensive error handling for NAS operations, ZFS management,
//! storage protocols, and Songbird integration.

use std::fmt;

/// Main error type for NestGate operations
#[derive(Debug, Clone)]
pub enum NestGateError {
    /// ZFS related errors
    ZfsError(String),
    
    /// Invalid ZFS pool name
    InvalidPoolName(String),
    
    /// Storage protocol errors (NFS, SMB, iSCSI, etc.)
    StorageProtocolError(String),
    
    /// Tier management errors
    TierManagementError(String),
    
    /// Configuration errors
    ConfigurationError(String),
    
    /// Songbird integration errors
    OrchestrationError(String),
    
    /// I/O errors
    IoError(String),
    
    /// Network errors
    NetworkError(String),
    
    /// Permission/authentication errors
    PermissionError(String),
    
    /// General system errors
    SystemError(String),
}

impl fmt::Display for NestGateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZfsError(msg) => write!(f, "ZFS error: {msg}"),
            Self::InvalidPoolName(msg) => write!(f, "Invalid pool name: {msg}"),
            Self::StorageProtocolError(msg) => write!(f, "Storage protocol error: {msg}"),
            Self::TierManagementError(msg) => write!(f, "Tier management error: {msg}"),
            Self::ConfigurationError(msg) => write!(f, "Configuration error: {msg}"),
            Self::OrchestrationError(msg) => write!(f, "Orchestration error: {msg}"),
            Self::IoError(msg) => write!(f, "I/O error: {msg}"),
            Self::NetworkError(msg) => write!(f, "Network error: {msg}"),
            Self::PermissionError(msg) => write!(f, "Permission error: {msg}"),
            Self::SystemError(msg) => write!(f, "System error: {msg}"),
        }
    }
}

impl std::error::Error for NestGateError {}

impl From<std::io::Error> for NestGateError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error.to_string())
    }
}

impl From<serde_json::Error> for NestGateError {
    fn from(error: serde_json::Error) -> Self {
        Self::ConfigurationError(format!("JSON serialization error: {error}"))
    }
}

/// Result type for NestGate operations
pub type Result<T> = std::result::Result<T, NestGateError>;

/// Helper macro for creating ZFS errors
#[macro_export]
macro_rules! zfs_error {
    ($msg:expr) => {
        $crate::error::NestGateError::ZfsError($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::NestGateError::ZfsError(format!($fmt, $($arg)*))
    };
}

/// Helper macro for creating storage protocol errors
#[macro_export]
macro_rules! protocol_error {
    ($msg:expr) => {
        $crate::error::NestGateError::StorageProtocolError($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::NestGateError::StorageProtocolError(format!($fmt, $($arg)*))
    };
}

/// Helper macro for creating tier management errors
#[macro_export]
macro_rules! tier_error {
    ($msg:expr) => {
        $crate::error::NestGateError::TierManagementError($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::NestGateError::TierManagementError(format!($fmt, $($arg)*))
    };
} 