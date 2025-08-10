/// **CAPABILITIES MODULE - UNIVERSAL ADAPTER ROUTING**
/// This module replaces all mechanical splits with proper domain-based architecture.
/// Everything routes through the universal adapter using capability discovery.
///
/// **ELIMINATES**: 46 mechanical *_part*.rs files
/// **IMPLEMENTS**: Universal Primal Architecture Standard from parent directory
use crate::ecosystem_integration::universal_adapter::adapter::UniversalAdapter;
use crate::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Re-export the ServiceCapability from universal_adapter for consistency
pub use crate::ecosystem_integration::universal_adapter::types::ServiceCapability;

/// Capability query for searching ecosystem capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapabilityQuery {
    /// Search for capabilities by name pattern
    Search(String),
    /// Get capabilities by exact type
    ByType(String),
    /// Get all available capabilities
    All,
}

// Domain-based modules (not mechanical splits)
pub mod discovery;
pub mod domains;
pub mod routing;

/// Universal capability manager - single point of access
pub struct CapabilityManager {
    adapter: Arc<UniversalAdapter>,
}

impl CapabilityManager {
    /// Create new capability manager with universal adapter
    pub fn new(adapter: Arc<UniversalAdapter>) -> Self {
        Self { adapter }
    }

    /// Discover capability by domain (replaces hardcoded service names)
    pub async fn discover_capability(&self, domain: &str) -> Result<String> {
        use crate::ecosystem_integration::universal_adapter::types::CapabilityQuery;

        let capabilities = self
            .adapter
            .query_capabilities(CapabilityQuery::Search(domain.to_string()))
            .await?;

        capabilities
            .first()
            .map(|cap| cap.name.clone())
            .ok_or_else(|| NestGateError::Configuration {
                message: format!("No capability found for domain: {domain}"),
                config_source: crate::error::UnifiedConfigSource::Runtime,
                field: Some(domain.to_string()),
                suggested_fix: Some(
                    "Ensure required services are running and registered".to_string(),
                ),
            })
    }

    /// Get endpoint for capability (replaces hardcoded endpoints)
    pub async fn get_endpoint(&self, capability: &str) -> Result<String> {
        // Implementation will route through universal adapter
        // This replaces all the hardcoded DEFAULT_*_SERVICE constants
        self.discover_capability(capability).await
    }
}
