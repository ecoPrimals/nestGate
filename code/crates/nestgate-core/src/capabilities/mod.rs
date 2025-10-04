// **CAPABILITIES MODULE - UNIVERSAL ADAPTER ROUTING**
// This module replaces all mechanical splits with proper domain-based architecture.
// Everything routes through the universal adapter using capability discovery.

/// Capabilities module for universal adapter routing.
///
/// This module provides capability-based service discovery and routing through the universal
/// adapter pattern, eliminating hardcoded service dependencies and enabling true sovereignty.
//
// **ELIMINATES**: 46 mechanical *_part*.rs files
// **IMPLEMENTS**: Universal Primal Architecture Standard from parent directory
use crate::universal_adapter::PrimalAgnosticAdapter;
use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
// Re-export the ServiceCapability from universal_adapter for consistency
// pub use crate::universal_adapter::ServiceCapability; // Commented out until available

// Capability query for searching ecosystem capabilities
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

// Universal capability manager - single point of access
pub struct CapabilityManager {
    adapter: Arc<PrimalAgnosticAdapter>,
}
impl CapabilityManager {
    /// Create new capability manager with universal adapter
    #[must_use]
    pub fn new(adapter: Arc<PrimalAgnosticAdapter>) -> Self {
        Self { adapter }
    }

    /// Discover capability by domain (replaces hardcoded service names)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn discover_capability(&self, domain: &str) -> Result<String> {
        // use crate::universal_adapter::CapabilityRequest; // Commented out until available

        let capabilities = self.adapter.query_capability(
            &crate::universal_adapter::types::CapabilityQuery::search(domain.to_string()),
        )?;

        capabilities
            .first()
            .cloned() // String type doesn't have name field
            .ok_or_else(|| {
                NestGateError::configuration_error(
                    domain,
                    &format!("No capability found for domain: {domain}"),
                )
            })
    }

    /// Get endpoint for capability (replaces hardcoded endpoints)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_endpoint(&self, capability: &str) -> Result<String> {
        // Implementation will route through universal adapter
        // This replaces all the hardcoded DEFAULT_*_SERVICE constants
        self.discover_capability(capability).await
    }
}
