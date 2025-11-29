//! Zero Cost Data Sources module

use crate::error::NestGateError;
use std::collections::HashMap;
//
// Modern, compile-time data source system that eliminates dynamic dispatch
// and provides zero-cost abstractions for data streaming operations.

use crate::{Result};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

// ==================== SECTION ====================

/// Zero-cost data capability trait using compile-time generics
#[allow(async_fn_in_trait)]
pub trait ZeroCostDataCapability {
    /// Type alias for Config
    type Config: Clone + Send + Sync;
    /// Type alias for Stream
    type Stream: ZeroCostDataStream;
    /// Type alias for Error
    type Error: Into<NestGateError>;
    /// Get the capability name at compile time
    const CAPABILITY_NAME: &'static str;

    /// Create a new data stream with the given configuration
    async fn create_stream(
        &self,
        config: Self::Config,
    ) -> std::result::Result<Self::Stream, Self::Error>;

    /// Validate configuration at compile time
    fn validate_config(config: &Self::Config) -> std::result::Result<(), Self::Error>;

    /// Get supported data formats
    fn supported_formats() -> &'static [&'static str];
}

/// Zero-cost data stream trait
#[allow(async_fn_in_trait)]
pub trait ZeroCostDataStream {
    /// Type alias for Item
    type Item: Send + Sync;
    /// Type alias for Error
    type Error: Into<NestGateError>;
    /// Read data from the stream
    async fn read(&mut self, buffer: &mut [u8]) -> std::result::Result<usize, Self::Error>;

    /// Seek to position in the stream
    async fn seek(&mut self, position: u64) -> std::result::Result<(), Self::Error>;

    /// Get stream metadata
    fn metadata(&self) -> StreamMetadata;
}

// ==================== SECTION ====================

/// Universal data configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::config::DataConfig;
/// 
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::DataConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for Data
pub struct DataConfig {
    /// Source Type
    pub source_type: String,
    /// Endpoint
    pub endpoint: String,
    /// Timeout Seconds
    pub timeout_seconds: u64,
    /// Retry Attempts
    pub retry_attempts: u32,
    /// Headers
    pub headers: std::collections::HashMap<String, String>,
}
impl Default for DataConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            source_type: "generic".to_string(),
            endpoint: String::new(),
            timeout_seconds: 30,
            retry_attempts: 3,
            headers: std::collections::HashMap::new(),
        }
    }
}

/// Stream metadata
#[derive(Debug, Clone)]
/// Streammetadata
pub struct StreamMetadata {
    /// Size
    pub size: Option<u64>,
    /// Content Type
    pub content_type: String,
    /// Encoding
    pub encoding: Option<String>,
    /// Last Modified
    pub last_modified: Option<std::time::SystemTime>,
}
// ==================== SECTION ====================

/// HTTP data provider with zero-cost abstractions
pub struct HttpDataProvider {
    client: reqwest::Client,
    base_url: String,
}
/// HTTP data stream
pub struct HttpDataStream {
    response: Option<reqwest::Response>,
    metadata: StreamMetadata,
    position: u64,
}
/// HTTP data errors
#[derive(Debug, thiserror::Error)]
/// Errors that can occur during HttpData operations
pub enum HttpDataError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Stream error: {0}")]
    StreamError(String),
}
impl From<HttpDataError> for NestGateError {
    /// From
    fn from(error: HttpDataError) -> Self {
        NestGateError::internal_error(
    }
}

impl HttpDataProvider {
    /// Creates a new instance
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }
}

impl ZeroCostDataCapability for HttpDataProvider {
    /// Type alias for Config
    type Config = DataConfig;
    /// Type alias for Stream
    type Stream = HttpDataStream;
    /// Type alias for Error
    type Error = HttpDataError;

    /// Capability Name
    const CAPABILITY_NAME: &'static str = "http";

    /// Creates  Stream
    async fn create_stream(
        &self,
        config: Self::Config,
    ) -> std::result::Result<Self::Stream, Self::Error> {
        let url = format!("{}/{}", self.base_url, config.endpoint);
        let response = self
            .client
            .get(&url)
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .send()
            .await
            .map_err(HttpDataError::RequestFailed)?;

        let content_length = response.content_length();
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();

        let metadata = StreamMetadata {
            size: content_length,
            content_type,
            encoding: None,
            last_modified: None,
        };

        Ok(HttpDataStream {
            response: Some(response),
            metadata,
            position: 0,
        })
    }

    /// Validates  Config
    fn validate_config(config: &Self::Config) -> std::result::Result<(), Self::Error> {
        if config.endpoint.is_empty() {
            return Err(HttpDataError::InvalidConfig(
                "endpoint cannot be empty".to_string(),
            ));
        }
        if config.timeout_seconds == 0 {
            return Err(HttpDataError::InvalidConfig(
                "timeout must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }

    /// Supported Formats
    fn supported_formats() -> &'static [&'static str] {
        &["json", "xml", "text", "binary"]
    }
}

impl ZeroCostDataStream for HttpDataStream {
    /// Type alias for Item
    type Item = Vec<u8>;
    /// Type alias for Error
    type Error = HttpDataError;

    /// Read
    async fn read(&mut self, buffer: &mut [u8]) -> std::result::Result<usize, Self::Error> {
        if let Some(response) = &mut self.response {
            let chunk = response
                .chunk()
                .await
                .map_err(HttpDataError::RequestFailed)?;

            if let Some(chunk) = chunk {
                let len = std::cmp::min(chunk.len(), buffer.len());
                buffer[..len].copy_from_slice(&chunk[..len]);
                self.position += len as u64;
                Ok(len)
            } else {
                Ok(0) // EOF
            }
        } else {
            Ok(0)
        }
    }

    /// Seek
    async fn seek(&mut self, _position: u64) -> std::result::Result<(), Self::Error> {
        Err(HttpDataError::StreamError(
            "HTTP streams do not support seeking".to_string(),
        ))
    }

    /// Metadata
    fn metadata(&self) -> StreamMetadata {
        self.metadata.clone()
    }
}

// ==================== SECTION ====================

/// File data provider with zero-cost abstractions
pub struct FileDataProvider {
}
/// File data stream
pub struct FileDataStream {
    file: Option<tokio::fs::File>,
    metadata: StreamMetadata,
    position: u64,
}
/// File data errors
#[derive(Debug, thiserror::Error)]
/// Errors that can occur during FileData operations
pub enum FileDataError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}
impl From<FileDataError> for NestGateError {
    /// From
    fn from(error: FileDataError) -> Self {
        NestGateError::internal_error(
    }
}

impl FileDataProvider {
        Self {
        }
    }
}

impl ZeroCostDataCapability for FileDataProvider {
    /// Type alias for Config
    type Config = DataConfig;
    /// Type alias for Stream
    type Stream = FileDataStream;
    /// Type alias for Error
    type Error = FileDataError;

    /// Capability Name
    const CAPABILITY_NAME: &'static str = "file";

    /// Creates  Stream
    async fn create_stream(
        &self,
        config: Self::Config,
    ) -> std::result::Result<Self::Stream, Self::Error> {
        let file_path = self.base_path.join(&config.endpoint);

        // Validate file exists and is readable
        let metadata = fs::metadata(&file_path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                FileDataError::FileNotFound(file_path.display().to_string())
            } else if e.kind() == std::io::ErrorKind::PermissionDenied {
                FileDataError::PermissionDenied(file_path.display().to_string())
            } else {
                FileDataError::IoError(e)
            }
        )?;

        let file = fs::File::open(&file_path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                FileDataError::PermissionDenied(file_path.display().to_string())
            } else {
                FileDataError::IoError(e)
            }
        )?;

        // Determine content type from file extension
        let content_type = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| match ext.to_lowercase().as_str() {
                "json" => "application/json",
                "xml" => "application/xml",
                "txt" | "md" => "text/plain",
                "csv" => "text/csv",
                "yaml" | "yml" => "application/yaml",
                _ => "application/octet-stream",
            })
            .unwrap_or("application/octet-stream")
            .to_string();

        let stream_metadata = StreamMetadata {
            size: Some(metadata.len()),
            content_type,
            encoding: None,
            last_modified: metadata.modified().ok(),
        };

        Ok(FileDataStream {
            file: Some(file),
            metadata: stream_metadata,
            position: 0,
        })
    }

    /// Validates  Config
    fn validate_config(config: &Self::Config) -> std::result::Result<(), Self::Error> {
        if config.endpoint.is_empty() {
            return Err(FileDataError::InvalidConfig(
                "file path cannot be empty".to_string(),
            ));
        }

        // Validate path doesn't contain dangerous patterns
        if config.endpoint.contains("..") {
            return Err(FileDataError::InvalidConfig(
                "path traversal not allowed".to_string(),
            ));
        }

        Ok(())
    }

    /// Supported Formats
    fn supported_formats() -> &'static [&'static str] {
        &["json", "xml", "txt", "csv", "yaml", "binary"]
    }
}

impl ZeroCostDataStream for FileDataStream {
    /// Type alias for Item
    type Item = Vec<u8>;
    /// Type alias for Error
    type Error = FileDataError;

    /// Read
    async fn read(&mut self, buffer: &mut [u8]) -> std::result::Result<usize, Self::Error> {
        if let Some(file) = &mut self.file {
            let bytes_read = file.read(buffer).await?;
            self.position += bytes_read as u64;
            Ok(bytes_read)
        } else {
            Ok(0)
        }
    }

    /// Seek
    async fn seek(&mut self, position: u64) -> std::result::Result<(), Self::Error> {
        if let Some(file) = &mut self.file {
            file.seek(std::io::SeekFrom::Start(position)).await?;
            self.position = position;
            Ok(())
        } else {
            Err(FileDataError::InvalidConfig(
                "file stream not initialized".to_string(),
            ))
        }
    }

    /// Metadata
    fn metadata(&self) -> StreamMetadata {
        self.metadata.clone()
    }
}

// ==================== SECTION ====================

/// Zero-cost data manager with compile-time provider selection
pub struct ZeroCostDataManager<T: ZeroCostDataCapability> {
    provider: T,
    config: T::Config,
}
impl<T: ZeroCostDataCapability> ZeroCostDataManager<T> {
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub fn new(provider: T, config: T::Config) -> Result<Self>  {
        T::validate_config(&config).map_err(|e| e.into())?;
        Ok(Self { provider, config })
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn create_stream(&self) -> Result<T::Stream>  {
        self.provider
            .create_stream(self.config.clone())
            .await
            .map_err(|e| e.into())
    }

    /// Capability Name
    pub fn capability_name() -> &'static str {
        T::CAPABILITY_NAME
    }

    /// Supported Formats
    pub fn supported_formats() -> &'static [&'static str] {
        T::supported_formats()
    }
}

// ==================== SECTION ====================

/// Zero-cost data factory for compile-time provider selection
pub struct ZeroCostDataFactory;
impl ZeroCostDataFactory {
    /// Create HTTP data manager
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn create_http_manager(
        base_url: String,
        config: DataConfig,
    ) -> Result<ZeroCostDataManager<HttpDataProvider>>  {
        let provider = HttpDataProvider::new(base_url);
        ZeroCostDataManager::new(provider, config)
    }

    /// Create file data manager
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn create_file_manager<P: AsRef<Path>>(
        config: DataConfig,
    ) -> Result<ZeroCostDataManager<FileDataProvider>>  {
        let provider = FileDataProvider::new(base_path);
        ZeroCostDataManager::new(provider, config)
    }
}

// ==================== SECTION ====================


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Dataconfigcanonical
pub type DataConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using DataConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_config_default() {
        let config = DataConfig::default();
        assert_eq!(config.source_type, "generic");
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.retry_attempts, 3);
        assert!(config.headers.is_empty());
    }

    #[test]
    fn test_http_provider_validation() {
        let mut config = DataConfig::default();
        config.endpoint = "test".to_string();
        assert!(HttpDataProvider::validate_config(&config).is_ok());

        config.endpoint = String::new();
        assert!(HttpDataProvider::validate_config(&config).is_err());

        config.endpoint = "test".to_string();
        config.timeout_seconds = 0;
        assert!(HttpDataProvider::validate_config(&config).is_err());
    }

    #[test]
    fn test_file_provider_validation() {
        let mut config = DataConfig::default();
        config.endpoint = "test.txt".to_string();
        assert!(FileDataProvider::validate_config(&config).is_ok());

        config.endpoint = String::new();
        assert!(FileDataProvider::validate_config(&config).is_err());

        config.endpoint = "../../../etc/passwd".to_string();
        assert!(FileDataProvider::validate_config(&config).is_err());
    }

    #[tokio::test]
    async fn test_file_provider_creation() {
        let temp_dir = std::env::temp_dir();
        let provider = FileDataProvider::new(&temp_dir);

        let config = DataConfig {
            endpoint: "nonexistent.txt".to_string(),
            ..Default::default()
        };

        // Should fail for nonexistent file
        let result = provider.create_stream(config).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_supported_formats() {
        let http_formats = HttpDataProvider::supported_formats();
        assert!(http_formats.contains(&"json"));
        assert!(http_formats.contains(&"xml"));

        let file_formats = FileDataProvider::supported_formats();
        assert!(file_formats.contains(&"json"));
        assert!(file_formats.contains(&"yaml"));
    }

    #[test]
    fn test_capability_names() {
        assert_eq!(HttpDataProvider::CAPABILITY_NAME, "http");
        assert_eq!(FileDataProvider::CAPABILITY_NAME, "file");
    }
}
