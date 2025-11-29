/// **⚠️ DEPRECATED MODULE**: This entire module is a facade wrapper around `crate::universal_adapter`
/// 
/// **Migration**: Use `nestgate_core::universal_adapter` directly instead
/// - `ecosystem_integration::UniversalAdapter` → `universal_adapter::UniversalAdapter`
/// - `ecosystem_integration::universal_adapter::config` → `universal_adapter::config`
/// - `ecosystem_integration::create_default_adapter_config()` → `universal_adapter::config::UniversalAdapterConfig::default()`
/// 
/// **Removed**: November 10, 2025 - No remaining imports found
#[deprecated(since = "0.11.2", note = "Use crate::universal_adapter directly. This facade module will be removed in v0.12.0 (May 2026)")]
use crate::config::canonical_primary::NestGateCanonicalConfig;
// **REMOVED**: InterfaceResult import - using unified Result<T> instead

// Remove deprecated imports - use unified types instead
// Note: These modules are being refactored as part of the universal adapter
#[deprecated(since = "0.11.2", note = "Use crate::universal_adapter instead")]
pub mod adapter;
#[deprecated(since = "0.11.2", note = "Use crate::universal_adapter::config instead")]
pub mod config;
#[deprecated(since = "0.11.2", note = "Use crate::universal_adapter::types instead")]
pub mod types;

// Remove non-existent modules for now
// pub mod discovery;
// pub mod stats;

// Re-export main types for convenience
pub use adapter::UniversalAdapter;
pub use config::*;
pub use types::*;

// **UNIFIED**: Use the main Result type from error module
pub use crate::Result;
// **REMOVED**: Deprecated UniversalAdapterResult<T> type alias eliminated
// Use unified Result<T> type directly

// Configuration summary for adapter information
#[derive(Debug, Clone)]
/// Adapterconfigurationsummary
pub struct AdapterConfigurationSummary {
    /// Discovery Endpoint
    pub discovery_endpoint: String,
    /// Service name
    pub service_name: String,
    /// Max Concurrent Requests
    pub max_concurrent_requests: u32,
    /// Timeout Seconds
    pub timeout_seconds: u64,
}
impl Default for AdapterConfigurationSummary {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            discovery_endpoint:
                crate::constants::network::DEFAULT_DISCOVERY_ENDPOINT
                    .to_string(), // Use centralized constant
            // SOVEREIGNTY FIX: Use capability-based service identification
            service_name: std::env::var("NESTGATE_SERVICE_NAME")
                .unwrap_or_else(|_| format!("universal-adapter-{uuid::Uuid::new_v4(}").simple())),
            max_concurrent_requests:
                crate::constants::canonical_defaults::network::limits::MAX_CONCURRENT_REQUESTS,
            timeout_seconds: crate::constants::domain_constants::timeouts::REQUEST_TIMEOUT_SECS,
        }
    }
}

// Universal Adapter System that combines all components
pub struct UniversalAdapterSystem {
    adapter: crate::universal_adapter::PrimalAgnosticAdapter,
    config: NestGateCanonicalConfig,
}
impl UniversalAdapterSystem {
    /// Creates a new instance
    pub async fn new(config: NestGateCanonicalConfig) -> crate::Result<Self> {
        // Create adapter config from unified config
        let adapter_config = crate::ecosystem_integration::create_default_adapter_config();
        let adapter =
            crate::universal_adapter::PrimalAgnosticAdapter::new(
                adapter_config,
            );
        adapter.initialize().await?;

        Ok(Self { adapter, config })
    }

    /// Get the adapter
    pub fn adapter(
        &self,
    ) -> &crate::universal_adapter::PrimalAgnosticAdapter {
        &self.adapter
    }

    /// Get the configuration
    pub fn config(&self) -> &NestGateCanonicalConfig {
        &self.config
    }
}
