//! Core traits for EcoPrimal SDK
//!
//! This module defines the fundamental traits that all primals must implement.

use super::config::*;
use super::errors::*;
use super::types::*;
use async_trait::async_trait;

/// EcoPrimal trait - MUST be implemented by all primals for biomeOS integration
#[async_trait]
pub trait EcoPrimal: Send + Sync {
    /// Get primal metadata information
    fn metadata(&self) -> &PrimalMetadata;

    /// Get list of capabilities this primal provides
    fn capabilities(&self) -> &[PrimalCapability];

    /// Initialize the primal with configuration
    async fn initialize(&self, config: &PrimalConfig) -> Result<(), PrimalError>;

    /// Handle a generic primal request
    async fn handle_request(&self, request: PrimalRequest) -> Result<PrimalResponse, PrimalError>;

    /// Get current health status
    async fn health_check(&self) -> PrimalHealth;

    /// Gracefully shutdown the primal
    async fn shutdown(&self) -> Result<(), PrimalError>;

    /// Get primal performance metrics
    async fn get_metrics(&self) -> Result<PrimalMetrics, PrimalError>;

    /// Update primal configuration at runtime
    async fn update_config(&self, config: &PrimalConfig) -> Result<(), PrimalError>;

    /// Get supported API versions
    fn supported_api_versions(&self) -> Vec<String>;

    /// Check if primal supports a specific capability
    fn supports_capability(&self, capability: &PrimalCapability) -> bool;
}

/// Advanced EcoPrimal trait for enhanced functionality
#[async_trait]
pub trait AdvancedEcoPrimal: EcoPrimal {
    /// Placeholder for advanced functionality
    async fn advanced_operation(&self) -> Result<(), PrimalError> {
        // Stub implementation
        Ok(())
    }
}
