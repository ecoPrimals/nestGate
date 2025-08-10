use crate::unified_types::UnifiedConfig;
// **REMOVED**: InterfaceResult import - using unified Result<T> instead

// Remove deprecated imports - use unified types instead
// Note: These modules are being refactored as part of the universal adapter
pub mod adapter;
pub mod config;
pub mod types;

// Remove non-existent modules for now
// pub mod discovery;
// pub mod stats;

// Re-export main types for convenience
pub use adapter::UniversalAdapter;
pub use config::*;
pub use types::*;

/// **UNIFIED**: Use the main Result type from error module
pub use crate::error::Result;

// **REMOVED**: Deprecated UniversalAdapterResult<T> type alias eliminated
// Use unified Result<T> type directly

/// Configuration summary for adapter information
#[derive(Debug, Clone)]
pub struct AdapterConfigurationSummary {
    pub discovery_endpoint: String,
    pub service_name: String,
    pub max_concurrent_requests: u32,
    pub timeout_seconds: u64,
}

impl Default for AdapterConfigurationSummary {
    fn default() -> Self {
        Self {
            discovery_endpoint:
                crate::constants::domain_constants::network::addresses::DEFAULT_DISCOVERY_ENDPOINT
                    .to_string(), // Use centralized constant
            // SOVEREIGNTY FIX: Use capability-based service identification
            service_name: std::env::var("NESTGATE_SERVICE_NAME")
                .unwrap_or_else(|_| format!("universal-adapter-{}", uuid::Uuid::new_v4().simple())),
            max_concurrent_requests:
                crate::constants::domain_constants::network::limits::MAX_CONCURRENT_REQUESTS as u32,
            timeout_seconds: crate::constants::domain_constants::timeouts::REQUEST_TIMEOUT_SECS,
        }
    }
}

/// Universal Adapter System that combines all components
pub struct UniversalAdapterSystem {
    adapter: crate::ecosystem_integration::universal_adapter::adapter::UniversalAdapter,
    config: UnifiedConfig,
}

impl UniversalAdapterSystem {
    pub async fn new(config: UnifiedConfig) -> crate::Result<Self> {
        // Create adapter config from unified config
        let adapter_config = crate::ecosystem_integration::create_default_adapter_config();
        let adapter =
            crate::ecosystem_integration::universal_adapter::adapter::UniversalAdapter::new(
                adapter_config,
            );
        adapter.initialize().await?;

        Ok(Self { adapter, config })
    }

    /// Get the adapter
    pub fn adapter(
        &self,
    ) -> &crate::ecosystem_integration::universal_adapter::adapter::UniversalAdapter {
        &self.adapter
    }

    /// Get the configuration
    pub fn config(&self) -> &UnifiedConfig {
        &self.config
    }
}
