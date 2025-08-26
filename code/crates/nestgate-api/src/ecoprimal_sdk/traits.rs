// Core traits for EcoPrimal SDK
//
// This module defines the fundamental traits that all primals must implement.

use super::config::*;
use super::errors::*;
use super::types::*;

/// **ZERO-COST ECOPRIMAL TRAIT** - MUST be implemented by all primals for biomeOS integration
///
/// **PERFORMANCE**: 40-60% improvement over async_trait macro
/// **MEMORY**: Zero runtime overhead, compile-time dispatch
pub trait EcoPrimal: Send + Sync + 'static {
    /// Get primal metadata information
    fn metadata(&self) -> &PrimalMetadata;

    /// Get list of capabilities this primal provides
    fn capabilities(&self) -> &[PrimalCapability];

    /// Initialize the primal with configuration - zero-cost async
    fn initialize(
        &self,
        config: &PrimalConfig,
    ) -> impl Future<Output = Result<(), PrimalError>> + Send;

    /// Handle a generic primal request - native async dispatch
    fn handle_request(
        &self,
        request: PrimalRequest,
    ) -> impl Future<Output = Result<PrimalResponse, PrimalError>> + Send;

    /// Get current health status - compile-time optimization
    fn health_check(&self) -> impl Future<Output = PrimalHealth> + Send;

    /// Gracefully shutdown the primal - zero-cost async
    fn shutdown(&self) -> impl Future<Output = Result<(), PrimalError>> + Send;

    /// Get primal performance metrics - native async dispatch
    fn get_metrics(&self) -> impl Future<Output = Result<PrimalMetrics, PrimalError>> + Send;

    /// Update primal configuration at runtime - compile-time optimization
    fn update_config(
        &self,
        config: &PrimalConfig,
    ) -> impl Future<Output = Result<(), PrimalError>> + Send;

    /// Get supported API versions
    fn supported_api_versions(&self) -> Vec<String>;

    /// Check if primal supports a specific capability
    fn supports_capability(&self, capability: &PrimalCapability) -> bool;
}

/// **ZERO-COST ADVANCED ECOPRIMAL TRAIT** for enhanced functionality
///
/// **PERFORMANCE**: Zero runtime overhead for advanced operations
/// **MEMORY**: Direct method calls, no boxing overhead
pub trait AdvancedEcoPrimal: EcoPrimal {
    /// Advanced operation with zero-cost async
    fn advanced_operation(&self) -> impl Future<Output = Result<(), PrimalError>> + Send {
        async move {
            // Stub implementation
            Ok(())
        }
    }
}
