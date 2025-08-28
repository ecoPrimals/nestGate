// Removed unused error imports
/// Configuration traits for universal service orchestration
use futures_util::Stream;
use serde::{Deserialize, Serialize};

use crate::config::federation::FederationConfig;
use crate::config::network::{HttpConfig, WebSocketConfig};
use crate::Result;

/// Configuration provider trait
pub trait ConfigProvider<T>: Send + Sync
where
    T: serde::de::DeserializeOwned + Clone + Send + Sync,
{
    /// Load configuration from the provider
    fn load_config(&self) -> impl std::future::Future<Output = Result<T>> + Send;

    /// Reload configuration (useful for file-based configs)
    fn reload_config(&self) -> impl std::future::Future<Output = Result<T>> + Send;

    /// Watch for configuration changes
    fn watch_config(&self) -> impl std::future::Future<Output = Result<T>> + Send;

    /// Validate configuration before loading
    fn validate_config(&self, config: &T) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Get provider information
    fn provider_info(&self) -> ConfigProviderInfo;
}

/// Information about a configuration provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProviderInfo {
    pub name: String,
    pub version: String,
    pub supports_reload: bool,
    pub supports_watch: bool,
}

/// Configuration metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMetadata {
    pub source: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub checksum: String,
    pub version: u64,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub websocket: WebSocketConfig,
    pub http: HttpConfig,
    pub federation: FederationConfig,
}
