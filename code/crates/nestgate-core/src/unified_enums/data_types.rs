/// Data Classification Enums
/// This module contains enums related to data types, content types,
/// and data processing classifications.
use serde::{Deserialize, Serialize};
use std::fmt;

// ==================== DATA CLASSIFICATION ====================

/// **THE** DataType - unified across all modules
/// Replaces 4+ fragmented DataType definitions across temporal_storage, ecosystem_integration, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnifiedDataType {
    // Core data types
    /// Binary data files
    Binary,
    /// Text and document data
    Text,
    /// Structured data (JSON, XML, etc.)
    Structured,
    /// Multimedia content (images, audio, video)
    Multimedia,

    // Scientific and research data
    /// Genomic data and sequences
    Genomic,
    /// Experimental and research data
    Scientific,
    /// AI/ML models and datasets
    Model,
    /// Training datasets
    Dataset,

    // System data
    /// Configuration files
    Configuration,
    /// System logs and telemetry
    Logs,
    /// Backup and archive data
    Archive,
    /// Temporary or cache data
    Temporary,

    /// Unknown or unclassified data
    Unknown,
    /// Custom data type with description
    Custom(String),
}

impl Default for UnifiedDataType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for UnifiedDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binary => write!(f, "binary"),
            Self::Text => write!(f, "text"),
            Self::Structured => write!(f, "structured"),
            Self::Multimedia => write!(f, "multimedia"),
            Self::Genomic => write!(f, "genomic"),
            Self::Scientific => write!(f, "scientific"),
            Self::Model => write!(f, "model"),
            Self::Dataset => write!(f, "dataset"),
            Self::Configuration => write!(f, "configuration"),
            Self::Logs => write!(f, "logs"),
            Self::Archive => write!(f, "archive"),
            Self::Temporary => write!(f, "temporary"),
            Self::Unknown => write!(f, "unknown"),
            Self::Custom(desc) => write!(f, "{desc}"),
        }
    }
}

// ==================== CONTENT TYPE CLASSIFICATION ====================

/// **THE** ContentType - unified across all modules
/// Replaces ContentType definitions in API handlers and HTTP responses
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnifiedContentType {
    /// JSON content
    Json,
    /// XML content
    Xml,
    /// Plain text content
    Text,
    /// HTML content
    Html,
    /// Binary content
    Binary,
    /// YAML content
    Yaml,
    /// TOML content
    Toml,
    /// CSV content
    Csv,
    /// Markdown content
    Markdown,
    /// PDF content
    Pdf,
    /// Image content (generic)
    Image,
    /// Audio content (generic)
    Audio,
    /// Video content (generic)
    Video,
    /// Database content
    Database,
    /// Time series data
    TimeSeries,
    /// Compressed data
    Compressed,
    /// Encrypted data
    Encrypted,
    /// Geospatial data
    Geospatial,
    /// Graph data
    Graph,
    /// Custom content type
    Custom(String),
}

impl Default for UnifiedContentType {
    fn default() -> Self {
        Self::Text
    }
}

impl fmt::Display for UnifiedContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json => write!(f, "application/json"),
            Self::Xml => write!(f, "application/xml"),
            Self::Text => write!(f, "text/plain"),
            Self::Html => write!(f, "text/html"),
            Self::Binary => write!(f, "application/octet-stream"),
            Self::Yaml => write!(f, "application/yaml"),
            Self::Toml => write!(f, "application/toml"),
            Self::Csv => write!(f, "text/csv"),
            Self::Markdown => write!(f, "text/markdown"),
            Self::Pdf => write!(f, "application/pdf"),
            Self::Image => write!(f, "image/*"),
            Self::Audio => write!(f, "audio/*"),
            Self::Video => write!(f, "video/*"),
            Self::Custom(mime) => write!(f, "{mime}"),
            Self::Database => write!(f, "application/x-database"),
            Self::TimeSeries => write!(f, "application/x-timeseries"),
            Self::Compressed => write!(f, "application/x-compressed"),
            Self::Encrypted => write!(f, "application/x-encrypted"),
            Self::Geospatial => write!(f, "application/x-geospatial"),
            Self::Graph => write!(f, "application/x-graph"),
        }
    }
}

// ==================== FILE TYPE CLASSIFICATION ====================

/// **THE** FileType - unified across all modules
/// Replaces FileType definitions in automation and ZFS modules
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnifiedFileType {
    /// Regular file
    Regular,
    /// Directory
    Directory,
    /// Symbolic link
    Symlink,
    /// Hard link
    Hardlink,
    /// Device file
    Device,
    /// FIFO/pipe
    Fifo,
    /// Socket
    Socket,
    /// Unknown file type
    Unknown,
    /// Custom file type
    Custom(String),
}

impl Default for UnifiedFileType {
    fn default() -> Self {
        Self::Regular
    }
}

impl fmt::Display for UnifiedFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regular => write!(f, "regular"),
            Self::Directory => write!(f, "directory"),
            Self::Symlink => write!(f, "symlink"),
            Self::Hardlink => write!(f, "hardlink"),
            Self::Device => write!(f, "device"),
            Self::Fifo => write!(f, "fifo"),
            Self::Socket => write!(f, "socket"),
            Self::Unknown => write!(f, "unknown"),
            Self::Custom(file_type) => write!(f, "{file_type}"),
        }
    }
}
