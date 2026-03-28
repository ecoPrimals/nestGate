// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use std::future::Future;

use super::canonical_service::CanonicalService;

/// **THE** canonical provider trait for service provisioning
///
/// This trait provides a generic way to provision services of type `T`.
/// It extends `CanonicalService` and adds provisioning capabilities.
///
/// **Type Parameter**: `T` - The service type being provided
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::traits::canonical_hierarchy::{CanonicalProvider, CanonicalStorage};
///
/// pub struct StorageProvider {
///     config: ProviderConfig,
/// }
///
/// impl CanonicalService for StorageProvider {
///     // ... implement CanonicalService
/// }
///
/// impl CanonicalProvider<Box<dyn CanonicalStorage>> for StorageProvider {
///     type Metadata = ProviderMetadata;
///
///     async fn provide(&self) -> Result<Box<dyn CanonicalStorage>, Self::Error> {
///         // Create storage instance
///         Ok(Box::new(MyStorage::new()))
///     }
///
///     async fn provide_with_config(
///         &self,
///         config: Self::Config,
///     ) -> Result<Box<dyn CanonicalStorage>, Self::Error> {
///         // Create storage with config
///         Ok(Box::new(MyStorage::with_config(config)))
///     }
///
///     async fn metadata(&self) -> Result<Self::Metadata, Self::Error> {
///         Ok(ProviderMetadata::default())
///     }
///
///     async fn from_config(config: Self::Config) -> Result<Self, Self::Error> {
///         Ok(Self { config })
///     }
/// }
/// ```
#[deprecated(
    since = "0.11.2",
    note = "Use crate::traits::canonical::CanonicalProvider instead. \
            This alternative definition extends CanonicalService and is less flexible. \
            The canonical version is standalone with customizable Config/Error types. \
            Migration: Replace canonical_hierarchy::CanonicalProvider with \
            canonical::CanonicalProvider and add explicit Config/Error types. \
            Target removal: v0.12.0 (May 2026). \
            See: CANONICAL_PROVIDER_COMPARISON.md for detailed migration guide."
)]
/// CanonicalProvider trait
pub trait CanonicalProvider<T>: CanonicalService {
    /// Provider-specific metadata
    type Metadata: Clone + Send + Sync + 'static;

    // ==================== PROVISIONING ====================

    /// Provide a service instance
    fn provide(&self) -> impl Future<Output = Result<T, Self::Error>> + Send;

    /// Provide a service with specific configuration
    fn provide_with_config(
        &self,
        config: Self::Config,
    ) -> impl Future<Output = Result<T, Self::Error>> + Send;

    // ==================== CAPABILITY DISCOVERY ====================

    /// Get provider metadata
    fn metadata(&self) -> impl Future<Output = Result<Self::Metadata, Self::Error>> + Send;

    /// Check if provider can provide the requested service
    ///
    /// Default implementation returns `true`.
    /// Override for capability-based filtering.
    fn can_provide(&self) -> impl Future<Output = bool> + Send {
        async { true }
    }

    // ==================== FACTORY METHODS ====================

    /// Create provider from configuration
    fn from_config(config: Self::Config) -> impl Future<Output = Result<Self, Self::Error>> + Send
    where
        Self: Sized;
}
