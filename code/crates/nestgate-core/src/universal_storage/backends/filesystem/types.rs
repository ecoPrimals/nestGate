// Core types and configuration for the filesystem storage backend

use serde::{Deserialize, Serialize};

// Type aliases for compatibility
pub type DataChunk = Vec<u8>;
/// Type alias for Storageprotocolinfo
pub type StorageProtocolInfo = std::collections::HashMap<String, String>;

/// File metadata structure containing comprehensive file information
#[derive(Debug, Clone)]
/// Filemetadata
pub struct FileMetadata {
    /// Size
    pub size: u64,
    /// Permissions
    pub permissions: String,
    /// Owner
    pub owner: String,
    /// Group
    pub group: String,
    /// Checksum
    pub checksum: Option<String>,
    /// Mime Type
    pub mime_type: Option<String>,
    /// Content Type
    pub content_type: Option<String>,
    /// Custom Metadata
    pub custom_metadata: std::collections::HashMap<String, String>,
    /// Created
    pub created: Option<std::time::SystemTime>,
    /// Modified
    pub modified: Option<std::time::SystemTime>,
    /// Accessed
    pub accessed: Option<std::time::SystemTime>,
    /// Timestamp when this was created
    pub created_at: Option<std::time::SystemTime>,
    /// Modified At
    pub modified_at: Option<std::time::SystemTime>,
    /// Tags
    pub tags: std::collections::HashMap<String, String>,
}
}
/// Response metadata for operations
#[derive(Debug, Clone)]
/// Responsemetadata
pub struct ResponseMetadata {
    /// Status
    pub status: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Request identifier
    pub request_id: String,
}
}
/// File system entry types
#[derive(Debug, Clone, PartialEq)]
/// Types of Entry
pub enum EntryType {
    /// File
    File,
    /// Directory
    Directory,
    /// Symlink
    Symlink,
}
}
/// Filesystem storage backend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Filesystem
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
}
impl Default for FilesystemConfig {
    /// Returns the default instance
    fn default() -> Self { Self {
            root_dir: PathBuf::from("./data"),
            atomic_writes: true,
            track_metadata: true,
            enable_watching: false,
            max_file_size: 0, // Unlimited
         }
         }
}
}
