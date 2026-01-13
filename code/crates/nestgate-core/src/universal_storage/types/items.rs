//! Storage items and metadata
//!
//! Types for representing storage items (files, directories) and their metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Storage item (file or directory)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageItem {
    /// Item name
    pub name: String,
    /// Item type (file, directory, etc.)
    pub item_type: StorageItemType,
    /// Size in bytes
    pub size: u64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,
    /// Last access timestamp
    pub accessed_at: Option<DateTime<Utc>>,
    /// MIME type
    pub mime_type: Option<String>,
    /// Checksum (e.g., SHA-256)
    pub checksum: Option<String>,
    /// Extended metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Storage item types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageItemType {
    /// Regular file
    File,
    /// Directory
    Directory,
    /// Symbolic link
    Symlink,
    /// Block device
    BlockDevice,
    /// Character device
    CharDevice,
    /// Named pipe (FIFO)
    Pipe,
    /// Unix domain socket
    Socket,
    /// Unknown or unsupported type
    Unknown,
}

/// Storage metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetadata {
    /// Content type (MIME type)
    pub content_type: Option<String>,
    /// Content encoding (e.g., "gzip", "br")
    pub content_encoding: Option<String>,
    /// Content language (e.g., "en-US")
    pub content_language: Option<String>,
    /// Cache control directives
    pub cache_control: Option<String>,
    /// Entity tag for cache validation
    pub etag: Option<String>,
    /// Custom user-defined metadata
    pub custom: HashMap<String, String>,
    /// System-generated metadata
    pub system: HashMap<String, serde_json::Value>,
}
