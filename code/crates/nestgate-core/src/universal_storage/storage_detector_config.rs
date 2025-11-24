//! Configuration for Storage Detection
//!
//! Provides immutable configuration for storage detection, eliminating
//! runtime environment variable access for credential checking.

use std::sync::Arc;

/// Immutable configuration for storage detector AWS credentials
#[derive(Debug, Clone)]
pub struct StorageDetectorConfig {
    /// AWS access key ID (if available)
    aws_access_key: Option<String>,

    /// AWS secret access key (if available)
    aws_secret_key: Option<String>,
}

/// Type alias for shared immutable configuration
pub type SharedStorageDetectorConfig = Arc<StorageDetectorConfig>;

impl StorageDetectorConfig {
    /// Create a new empty configuration
    pub fn new() -> Self {
        Self {
            aws_access_key: None,
            aws_secret_key: None,
        }
    }

    /// Load configuration from environment variables
    ///
    /// Reads AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY once at startup.
    pub fn from_env() -> Self {
        Self {
            aws_access_key: std::env::var("AWS_ACCESS_KEY_ID").ok(),
            aws_secret_key: std::env::var("AWS_SECRET_ACCESS_KEY").ok(),
        }
    }

    /// Set AWS credentials explicitly (for testing)
    pub fn with_aws_credentials(
        mut self,
        access_key: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> Self {
        self.aws_access_key = Some(access_key.into());
        self.aws_secret_key = Some(secret_key.into());
        self
    }

    /// Check if AWS credentials are available
    pub fn has_aws_credentials(&self) -> bool {
        self.aws_access_key.is_some() && self.aws_secret_key.is_some()
    }

    /// Get AWS access key (if available)
    pub fn aws_access_key(&self) -> Option<&str> {
        self.aws_access_key.as_deref()
    }

    /// Get AWS secret key (if available)
    pub fn aws_secret_key(&self) -> Option<&str> {
        self.aws_secret_key.as_deref()
    }
}

impl Default for StorageDetectorConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = StorageDetectorConfig::new();
        assert!(!config.has_aws_credentials());
        assert!(config.aws_access_key().is_none());
        assert!(config.aws_secret_key().is_none());
    }

    #[test]
    fn test_with_aws_credentials() {
        let config = StorageDetectorConfig::new().with_aws_credentials("test_key", "test_secret");

        assert!(config.has_aws_credentials());
        assert_eq!(config.aws_access_key(), Some("test_key"));
        assert_eq!(config.aws_secret_key(), Some("test_secret"));
    }

    #[test]
    fn test_has_aws_credentials_partial() {
        let mut config = StorageDetectorConfig::new();
        config.aws_access_key = Some("key".to_string());
        // Missing secret key
        assert!(!config.has_aws_credentials());
    }

    #[test]
    fn test_config_shared() {
        let config = Arc::new(StorageDetectorConfig::new().with_aws_credentials("key", "secret"));
        let config2 = Arc::clone(&config);

        assert!(config.has_aws_credentials());
        assert!(config2.has_aws_credentials());
        assert_eq!(Arc::strong_count(&config), 2);
    }

    #[test]
    fn test_from_env() {
        // Test runs without AWS env vars, should return empty config
        let config = StorageDetectorConfig::from_env();
        // Result depends on actual environment, just verify it doesn't crash
        let _ = config.has_aws_credentials();
    }

    #[test]
    fn test_default() {
        let config1 = StorageDetectorConfig::default();
        let config2 = StorageDetectorConfig::new();

        assert_eq!(config1.has_aws_credentials(), config2.has_aws_credentials());
    }
}
