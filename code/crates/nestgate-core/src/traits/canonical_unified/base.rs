//! **BASE CANONICAL TRAITS**
//!
//! Core service and provider traits that form the foundation of the canonical trait system.

#![allow(clippy::type_complexity)]

use serde::{Deserialize, Serialize};
use std::future::Future;
use std::time::SystemTime;
use crate::unified_enums::service_types::UnifiedServiceType;
use super::types::{ServiceCapabilities, ProviderHealth, ProviderCapabilities};

// ==================== THE CANONICAL SERVICE TRAIT ====================

/// **THE** canonical service trait that replaces ALL service traits
/// This is the single source of truth for all `NestGate` services
pub trait CanonicalService: Send + Sync + 'static {
    /// Service configuration type
    type Config: Clone + Send + Sync + 'static;

    /// Service health status type
    type Health: Clone + Send + Sync + 'static;

    /// Service metrics type
    type Metrics: Clone + Send + Sync + 'static;

    /// Service error type
    type Error: Send + Sync + std::error::Error + 'static;
    // ==================== CORE SERVICE OPERATIONS ====================

    /// Start the service - native async
    fn start(&self) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Stop the service - native async
    fn stop(&self) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Check service health - native async
    fn is_healthy(
        &self,
    ) -> impl Future<Output = std::result::Result<Self::Health, Self::Error>> + Send;

    /// Get service metrics - native async
    fn get_metrics(
        &self,
    ) -> impl Future<Output = std::result::Result<Self::Metrics, Self::Error>> + Send;

    /// Get service capabilities - native async
    fn capabilities(
        &self,
    ) -> impl Future<Output = std::result::Result<ServiceCapabilities, Self::Error>> + Send;

    /// Validate configuration - native async
    fn validate_config(
        &self,
        config: &Self::Config,
    ) -> impl Future<Output = std::result::Result<Vec<String>, Self::Error>> + Send;

    // ==================== ADDITIONAL SERVICE METHODS ====================

    /// Get service identifier - PEDANTIC ADDITION
    fn service_id(&self) -> &str {
        "unknown"
    }

    /// Get service type - PEDANTIC ADDITION
    fn service_type(&self) -> UnifiedServiceType {
        UnifiedServiceType::Generic
    }

    /// Initialize service with config - PEDANTIC ADDITION
    fn initialize(
        &self,
        config: Self::Config,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send {
        async move {
            let _ = config; // Use config parameter
            Ok(())
        }
    }

    /// Health check method - PEDANTIC ADDITION
    fn health_check(
        &self,
    ) -> impl Future<Output = std::result::Result<Self::Health, Self::Error>> + Send {
        async move {
            // PEDANTIC: Use is_healthy method instead of default()
            self.is_healthy().await
        }
    }

    /// Shutdown method - PEDANTIC ADDITION  
    fn shutdown(&self) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send {
        async move {
            // Default graceful shutdown
            Ok(())
        }
    }

    /// Restart method - PEDANTIC ADDITION
    fn restart(&self) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send {
        async move {
            // Default restart implementation
            Ok(())
        }
    }

    /// Update configuration method - PEDANTIC ADDITION
    fn update_config(
        &self,
        _config: Self::Config,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send {
        async move {
            // Default config update implementation
            Ok(())
        }
    }
}

// ==================== THE CANONICAL PROVIDER TRAIT ====================

/// **THE** canonical provider trait that replaces ALL provider traits
/// This is the single source of truth for all `NestGate` providers
pub trait CanonicalProvider<T>: Send + Sync + 'static {
    /// Provider configuration type
    type Config: Clone + Send + Sync + 'static;

    /// Provider error type
    type Error: Send + Sync + std::error::Error + 'static;

    /// Provider metadata type
    type Metadata: Clone + Send + Sync + 'static;
    // ==================== CORE PROVIDER OPERATIONS ====================

    /// Provide service instance - native async
    fn provide(
        &self,
        config: Self::Config,
    ) -> impl Future<Output = std::result::Result<T, Self::Error>> + Send;

    /// Configure provider - native async
    fn configure(
        &mut self,
        config: Self::Config,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Get provider metadata - native async
    fn metadata(
        &self,
    ) -> impl Future<Output = std::result::Result<Self::Metadata, Self::Error>> + Send;

    /// Health check - native async
    fn health_check(
        &self,
    ) -> impl Future<Output = std::result::Result<ProviderHealth, Self::Error>> + Send;

    /// Get provider capabilities - native async
    fn capabilities(
        &self,
    ) -> impl Future<Output = std::result::Result<ProviderCapabilities, Self::Error>> + Send;
}

