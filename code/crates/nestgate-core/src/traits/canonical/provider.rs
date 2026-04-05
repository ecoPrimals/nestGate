// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **Canonical Provider Trait**
//!
//! Core provider trait for service provisioning.
//!
//! **Extracted**: November 19, 2025 - From canonical_unified_traits.rs
//! **Lines**: ~45 (from original 1,100-line file)

use super::types::{ProviderCapabilities, ProviderHealth};
use std::future::Future;

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
