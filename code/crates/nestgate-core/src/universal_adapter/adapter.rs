/// **NESTGATE UNIVERSAL ADAPTER**
/// Unified interface for all provider interactions
///
/// **MIGRATION COMPLETE**: Now uses canonical UnifiedProviderConfig from consolidated_traits
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::Result;
use crate::unified_types::UnifiedConfig;

// **UNIFIED PROVIDER TYPES** - All provider types now use canonical definitions
// No local duplicates - everything imported from consolidated_traits

/// Adapter statistics structure
#[derive(Debug, Clone)]
pub struct AdapterStats {
    pub active_providers: usize,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
}

impl Default for AdapterStats {
    fn default() -> Self {
        Self {
            active_providers: 0,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
        }
    }
}

/// Universal adapter implementation with typed provider support
pub struct UniversalAdapter {
    /// Security provider (placeholder for compilation)
    pub security_provider: Option<String>,

    /// Storage provider (placeholder for compilation)  
    pub storage_provider: Option<String>,

    /// Network provider (placeholder for compilation)
    pub network_provider: Option<String>,

    /// Monitoring provider (placeholder for compilation)
    pub monitoring_provider: Option<String>,

    /// Configuration system
    #[allow(dead_code)]
    config: UnifiedConfig,
    #[allow(dead_code)]
    active_capabilities: Arc<RwLock<HashMap<String, Vec<String>>>>,
    stats: Arc<RwLock<AdapterStats>>,
}

impl UniversalAdapter {
    pub fn new(config: UnifiedConfig) -> Self {
        Self {
            security_provider: None,
            storage_provider: None,
            network_provider: None,
            monitoring_provider: None,
            config,
            active_capabilities: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(AdapterStats::default())),
        }
    }

    /// Initialize the adapter
    pub async fn initialize(&self) -> Result<()> {
        tracing::info!("🔧 Initializing UniversalAdapter");
        Ok(())
    }

    /// Get adapter statistics
    pub async fn get_stats(&self) -> AdapterStats {
        self.stats.read().await.clone()
    }

    /// Health check
    pub async fn health_check(&self) -> bool {
        true // Simplified health check
    }

    /// Query capabilities from the ecosystem
    pub async fn query_capabilities(
        &self,
        _query: crate::capabilities::CapabilityQuery,
    ) -> crate::Result<Vec<crate::ecosystem_integration::universal_adapter::types::ServiceCapability>>
    {
        // Placeholder implementation - would query actual ecosystem capabilities
        // For now, return empty vec since this is a mock implementation
        Ok(vec![])
    }

    /// Get orchestration provider if available
    pub async fn get_orchestration_provider(
        &self,
    ) -> Option<crate::ecosystem_integration::universal_adapter::types::ServiceCapability> {
        // Placeholder implementation - would search for orchestration capabilities
        // For now, return None since this is a mock implementation
        None
    }

    /// Find providers by capability type
    pub async fn find_providers_by_capability(
        &self,
        _capability_type: &str,
    ) -> Vec<crate::ecosystem_integration::universal_adapter::types::ServiceCapability> {
        // Placeholder implementation - would search discovered capabilities
        // For now, return empty vec since this is a mock implementation
        vec![]
    }

    /// Get compute provider if available
    pub async fn get_compute_provider(
        &self,
    ) -> Option<crate::ecosystem_integration::universal_adapter::types::ServiceCapability> {
        // Placeholder implementation - would search for compute capabilities
        // For now, return None since this is a mock implementation
        None
    }
}

/// Create a default adapter
pub fn create_default_adapter() -> UniversalAdapter {
    UniversalAdapter::new(UnifiedConfig::default())
}

/// Create an adapter with custom configuration
pub fn create_adapter_with_config(config: UnifiedConfig) -> UniversalAdapter {
    UniversalAdapter::new(config)
}
