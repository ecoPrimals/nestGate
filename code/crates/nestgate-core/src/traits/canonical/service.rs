// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **Canonical Service Trait**
//!
//! Core service trait that defines the interface for all NestGate services.
//!
//! **Extracted**: November 19, 2025 - From canonical_unified_traits.rs
//! **Lines**: ~150 (from original 1,100-line file)

use super::types::ServiceCapabilities;
use crate::unified_enums::service_types::UnifiedServiceType;
use std::future::Future;

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
    #[expect(clippy::unnecessary_literal_bound)] // Default is static; impls may return `&self`-backed ids
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
