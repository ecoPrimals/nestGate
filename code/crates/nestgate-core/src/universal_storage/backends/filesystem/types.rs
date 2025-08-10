//! Core types and configuration for the filesystem storage backend

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Type aliases for compatibility
pub type DataChunk = Vec<u8>;
pub type StorageProtocolInfo = std::collections::HashMap<String, String>;

/// File metadata structure containing comprehensive file information
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub path: String,
    pub size: u64,
    pub permissions: String,
    pub owner: String,
    pub group: String,
    pub checksum: Option<String>,
    pub mime_type: Option<String>,
    pub content_type: Option<String>,
    pub custom_metadata: std::collections::HashMap<String, String>,
    pub created: Option<std::time::SystemTime>,
    pub modified: Option<std::time::SystemTime>,
    pub accessed: Option<std::time::SystemTime>,
    pub created_at: Option<std::time::SystemTime>,
    pub modified_at: Option<std::time::SystemTime>,
    pub tags: std::collections::HashMap<String, String>,
}

/// Response metadata for operations
#[derive(Debug, Clone)]
pub struct ResponseMetadata {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub request_id: String,
}

/// File system entry types
#[derive(Debug, Clone, PartialEq)]
pub enum EntryType {
    File,
    Directory,
    Symlink,
}

/// Filesystem storage backend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemConfig {
    /// Root directory for storage
    pub root_dir: PathBuf,
    /// Enable atomic writes using temporary files
    pub atomic_writes: bool,
    /// Enable metadata tracking
    pub track_metadata: bool,
    /// Enable directory watching for change notifications
    pub enable_watching: bool,
    /// Maximum file size (in bytes, 0 = unlimited)
    pub max_file_size: u64,
}

impl Default for FilesystemConfig {
    fn default() -> Self {
        Self {
            root_dir: PathBuf::from("./data"),
            atomic_writes: true,
            track_metadata: true,
            enable_watching: false,
            max_file_size: 0, // Unlimited
        }
    }
}
