// Removed unused error imports
/// Configuration traits for universal service orchestration
use serde::{Deserialize, Serialize};
use crate::config::canonical_master::domains::network::protocols::{HttpConfig, WebSocketConfig};
use crate::Result;

/// Federation configuration for distributed service coordination
/// 
/// This is intentionally a unit type placeholder as federation features
/// are not yet implemented. When federation is added, this should be
/// replaced with a proper configuration struct containing:
/// - Peer discovery settings
/// - Consensus algorithms
/// - Network topology preferences
/// - Replication strategies
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FederationConfig {
    // Reserved for future federation implementation
    _reserved: (),
}

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
/// **⚠️ DEPRECATED**: Use `CanonicalNetworkConfig` from `canonical_master::domains::network`
#[deprecated(
    since = "0.9.0",
    note = "Use canonical_master::domains::network::CanonicalNetworkConfig instead"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub websocket: WebSocketConfig,
    pub http: HttpConfig,
    pub federation: FederationConfig,
}
