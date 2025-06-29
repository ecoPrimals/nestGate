//! Network protocol definitions and handlers
//!
//! This module provides common protocol definitions and utilities

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// Use nestgate_core for error handling
use nestgate_core::{NestGateError, Result};

/// Supported network protocols
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Protocol {
    /// Network File System
    Nfs,
    /// Server Message Block
    Smb,
    /// File Transfer Protocol
    Ftp,
    /// Secure Shell File Transfer Protocol
    Sftp,
    /// HTTP/HTTPS
    Http,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Nfs => write!(f, "NFS"),
            Protocol::Smb => write!(f, "SMB"),
            Protocol::Ftp => write!(f, "FTP"),
            Protocol::Sftp => write!(f, "SFTP"),
            Protocol::Http => write!(f, "HTTP"),
        }
    }
}

/// Performance preference for protocol selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerformancePreference {
    /// Optimize for speed
    Speed,
    /// Optimize for reliability
    Reliability,
    /// Optimize for compatibility
    Compatibility,
    /// Balanced approach
    Balanced,
}

impl Default for PerformancePreference {
    fn default() -> Self {
        PerformancePreference::Balanced
    }
}

/// Protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolConfig {
    /// Protocol type
    pub protocol: Protocol,
    /// Protocol-specific options
    pub options: HashMap<String, String>,
    /// Performance preference
    pub performance: PerformancePreference,
    /// Enable encryption
    pub encryption: bool,
    /// Connection timeout in seconds
    pub timeout: u32,
    /// Maximum retry attempts
    pub max_retries: u32,
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            protocol: Protocol::Nfs,
            options: HashMap::new(),
            performance: PerformancePreference::default(),
            encryption: true,
            timeout: 30,
            max_retries: 3,
        }
    }
}

/// Mount request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountRequest {
    /// Protocol to use
    pub protocol: Protocol,
    /// Remote server address
    pub server: String,
    /// Remote path or share name
    pub remote_path: String,
    /// Local mount point
    pub mount_point: PathBuf,
    /// Authentication credentials
    pub credentials: Option<Credentials>,
    /// Protocol-specific options
    pub options: HashMap<String, String>,
}

/// Mount response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountResponse {
    /// Mount ID for tracking
    pub mount_id: String,
    /// Success status
    pub success: bool,
    /// Status message
    pub message: String,
    /// Mount point path
    pub mount_point: PathBuf,
}

/// Authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    /// Username
    pub username: String,
    /// Password (should be encrypted in real implementation)
    pub password: String,
    /// Domain (for SMB)
    pub domain: Option<String>,
}

/// Protocol handler trait
#[async_trait]
pub trait ProtocolHandler: Send + Sync + std::fmt::Debug {
    /// Get the protocol type this handler supports
    fn protocol_type(&self) -> Protocol;

    /// Mount a remote resource
    async fn mount(&self, request: MountRequest) -> Result<MountResponse>;

    /// Unmount a resource by mount ID
    async fn unmount(&self, mount_id: &str) -> Result<bool>;

    /// Get status of a mount
    async fn get_status(&self, mount_id: &str) -> Result<MountStatus>;

    /// Test connection to a remote server
    async fn test_connection(
        &self,
        server: &str,
        credentials: Option<&Credentials>,
    ) -> Result<bool>;
}

/// Mount status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountStatus {
    /// Mount ID
    pub mount_id: String,
    /// Is currently mounted
    pub mounted: bool,
    /// Mount point
    pub mount_point: PathBuf,
    /// Protocol used
    pub protocol: Protocol,
    /// Server address
    pub server: String,
    /// Remote path
    pub remote_path: String,
    /// Last access time
    pub last_access: Option<chrono::DateTime<chrono::Utc>>,
    /// Error message if any
    pub error: Option<String>,
}

/// Protocol manager for handling multiple protocols
pub struct ProtocolManager {
    handlers: HashMap<Protocol, Box<dyn ProtocolHandler>>,
}

impl std::fmt::Debug for ProtocolManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProtocolManager")
            .field("handlers", &self.handlers.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl ProtocolManager {
    /// Create a new protocol manager
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Register a protocol handler
    pub fn register_handler(&mut self, handler: Box<dyn ProtocolHandler>) {
        let protocol = handler.protocol_type();
        self.handlers.insert(protocol, handler);
    }

    /// Get a handler for a specific protocol
    pub fn get_handler(&self, protocol: Protocol) -> Option<&dyn ProtocolHandler> {
        self.handlers.get(&protocol).map(|h| h.as_ref())
    }

    /// Mount using the appropriate protocol handler
    pub async fn mount(&self, request: MountRequest) -> Result<MountResponse> {
        let handler = self.get_handler(request.protocol).ok_or_else(|| {
            NestGateError::InvalidInput(format!("No handler for protocol: {}", request.protocol))
        })?;

        handler.mount(request).await
    }

    /// Unmount using the appropriate protocol handler
    pub async fn unmount(&self, protocol: Protocol, mount_id: &str) -> Result<bool> {
        let handler = self.get_handler(protocol).ok_or_else(|| {
            NestGateError::InvalidInput(format!("No handler for protocol: {}", protocol))
        })?;

        handler.unmount(mount_id).await
    }

    /// Get status using the appropriate protocol handler
    pub async fn get_status(&self, protocol: Protocol, mount_id: &str) -> Result<MountStatus> {
        let handler = self.get_handler(protocol).ok_or_else(|| {
            NestGateError::InvalidInput(format!("No handler for protocol: {}", protocol))
        })?;

        handler.get_status(mount_id).await
    }

    /// List all supported protocols
    pub fn supported_protocols(&self) -> Vec<Protocol> {
        self.handlers.keys().copied().collect()
    }
}

impl Default for ProtocolManager {
    fn default() -> Self {
        Self::new()
    }
}
