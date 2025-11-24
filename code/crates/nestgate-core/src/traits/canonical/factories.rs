//! **Canonical Factory Traits**
//!
//! Factory and auxiliary trait definitions.
//!
//! **Extracted**: November 19, 2025 - From canonical_unified_traits.rs
//! **Lines**: ~25 (from original 1,100-line file)

use super::provider::CanonicalProvider;
use super::service::CanonicalService;
use std::future::Future;

// ==================== ADDITIONAL CANONICAL TRAITS ====================

/// MCP protocol trait
pub trait CanonicalMcp: CanonicalService {}
/// Automation trait
pub trait CanonicalAutomation: CanonicalService {}
/// Zero-cost service marker
pub trait ZeroCostService: CanonicalService {}
/// Service factory
pub trait CanonicalServiceFactory<T: CanonicalService> {
    fn create_service(
        &self,
        config: T::Config,
    ) -> impl Future<Output = std::result::Result<T, crate::NestGateError>> + Send;
}
/// Provider factory
pub trait CanonicalProviderFactory<T, P: CanonicalProvider<T>> {
    fn create_provider(
        &self,
        config: P::Config,
    ) -> impl Future<Output = std::result::Result<P, crate::NestGateError>> + Send;
}
