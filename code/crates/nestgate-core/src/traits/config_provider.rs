//! Configuration Provider Trait for Dynamic Configuration Management
//!
//! **MIGRATED FROM**: `traits::config_provider` (November 7, 2025)
//! **CANONICAL**: This is the single source of truth for configuration providers
//! **STATUS**: Production-ready, native async

use crate::Result;
use serde::{Deserialize, Serialize};

/// Configuration provider trait for loading and managing configurations
///
/// This trait provides the interface for configuration providers that can
/// load, reload, watch, and validate configurations from various sources
/// (files, environment variables, remote config servers, etc.).
///
/// # Type Parameters
///
/// * `T` - The configuration type, must be deserializable, cloneable, and thread-safe
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::traits::ConfigProvider;
///
/// struct FileConfigProvider {
///     path: PathBuf,
/// }
///
/// impl ConfigProvider<MyConfig> for FileConfigProvider {
///     async fn load_config(&self) -> nestgate_core::Result<MyConfig> {
///         // Load from file
///         todo!()
///     }
///     
///     // ... implement other methods
/// }
/// ```
pub trait ConfigProvider<T>: Send + Sync
where
    T: serde::de::DeserializeOwned + Clone + Send + Sync,
{
    /// Load configuration from the provider
    fn load_config(&self) -> impl std::future::Future<Output = Result<T>> + Send;

    /// Reload configuration (useful for file-based configs)
    fn reload_config(&self) -> impl std::future::Future<Output = Result<T>> + Send;

    /// Watch for configuration changes
    ///
    /// This method returns the updated configuration when changes are detected.
    fn watch_config(&self) -> impl std::future::Future<Output = Result<T>> + Send;

    /// Validate configuration before loading
    fn validate_config(&self, config: &T) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Get provider information
    fn provider_info(&self) -> ConfigProviderInfo;
}

/// Information about a configuration provider
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configproviderinfo
pub struct ConfigProviderInfo {
    /// Name
    pub name: String,
    /// Version
    pub version: String,
    /// Supports Reload
    pub supports_reload: bool,
    /// Supports Watch
    pub supports_watch: bool,
}

/// Configuration metadata for tracking config changes
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configmetadata
pub struct ConfigMetadata {
    /// Source
    pub source: String,
    /// Last Modified
    pub last_modified: chrono::DateTime<chrono::Utc>,
    /// Checksum
    pub checksum: String,
    /// Version
    pub version: u64,
}

/// Federation configuration for distributed service coordination
///
/// **NOTE**: This is currently a placeholder. When federation is implemented,
/// this should contain:
/// - Peer discovery settings
/// - Consensus algorithms
/// - Network topology preferences
/// - Replication strategies
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for Federation
pub struct FederationConfig {
    // Reserved for future federation implementation
    _reserved: (),
}
