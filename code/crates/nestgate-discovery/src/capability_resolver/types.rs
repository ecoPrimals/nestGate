// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! [`ResolvedService`] and the [`CapabilityResolver`] trait.

use crate::unified_capabilities::UnifiedCapability;
use nestgate_types::error::Result;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Unified service endpoint result from capability resolution
#[derive(Debug, Clone)]
pub struct ResolvedService {
    /// Service identifier
    pub id: Arc<str>,
    /// Host address
    pub host: String,
    /// Port number
    pub port: u16,
    /// Protocol (http, https, grpc, etc.)
    pub protocol: Arc<str>,
    /// Capabilities this service provides
    pub capabilities: Vec<UnifiedCapability>,
    /// Service health indicator
    pub is_healthy: bool,
}

impl ResolvedService {
    /// Get full URL for this service
    #[must_use]
    pub fn url(&self) -> String {
        format!("{}://{}:{}", &*self.protocol, self.host, self.port)
    }

    /// Get endpoint without protocol
    #[must_use]
    pub fn endpoint(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Unified capability resolver trait - implemented by all registry types
///
/// This trait provides a common interface for capability-based service discovery
/// regardless of the underlying registry implementation.
///
/// **Object-Safe**: Uses boxed futures to enable dynamic dispatch
pub trait CapabilityResolver: Send + Sync {
    /// Find a service by unified capability
    fn resolve_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<ResolvedService>> + Send + '_>>;

    /// Find all services that provide a capability
    fn resolve_capability_all(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ResolvedService>>> + Send + '_>>;

    /// Check if a capability is available
    fn has_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + '_>>;
}
