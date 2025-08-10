// Removed unused error imports
/// Configuration traits for universal service orchestration
use async_trait::async_trait;
use futures_util::Stream;
use serde::{Deserialize, Serialize};

use crate::config::federation::FederationConfig;
use crate::config::network::{HttpConfig, WebSocketConfig};
use crate::error::Result;

/// Configuration provider trait
#[async_trait]
pub trait ConfigProvider<T>: Send + Sync
where
    T: serde::de::DeserializeOwned + Clone + Send + Sync,
{
    /// Load configuration from the provider
    async fn load_config(&self) -> Result<T>;

    /// Reload configuration (useful for file-based configs)
    async fn reload_config(&self) -> Result<T>;

    /// Watch for configuration changes
    async fn watch_config(&self) -> impl Stream<Item = Result<T>>;

    /// Validate configuration before loading
    async fn validate_config(&self, config: &T) -> Result<()>;

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
