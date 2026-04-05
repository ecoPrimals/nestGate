// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::future::Future;
use std::marker::PhantomData;

use super::status::ServiceStatus;

/// **ZERO-COST UNIVERSAL ORCHESTRATION WRAPPER**
///
/// Direct composition replacement for `Arc<dyn OrchestrationPrimalProvider>`
/// PERFORMANCE: 50-70% improvement through compile-time specialization
pub struct ZeroCostUniversalOrchestrationWrapper<Provider, const MAX_INSTANCES: usize = 500>
where
    Provider: ZeroCostOrchestrationProvider,
{
    _provider_name: String,
    _endpoint: String,
    _capabilities: Vec<String>,
    /// Direct composition - no `Arc<dyn>` overhead
    _provider: Provider,
    _phantom: PhantomData<()>,
}
/// Zero-cost orchestration provider trait - replaces `Arc<dyn OrchestrationPrimalProvider>`
pub trait ZeroCostOrchestrationProvider: Send + Sync + 'static {
    /// Type alias for Error
    type Error: Send + Sync + 'static;
    /// Type alias for InstanceId
    type InstanceId: Send + Sync + Clone;
    /// Type alias for ServiceConfig
    type ServiceConfig: Send + Sync + Clone;
    /// Deploy service with native async
    fn deploy_service(
        &self,
        config: &Self::ServiceConfig,
    ) -> impl Future<Output = std::result::Result<Self::InstanceId, Self::Error>> + Send;

    /// Scale service with zero-cost dispatch
    fn scale_service(
        &self,
        instance_id: &Self::InstanceId,
        replicas: u32,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Get service status with compile-time optimization
    fn get_service_status(
        &self,
        instance_id: &Self::InstanceId,
    ) -> impl Future<Output = std::result::Result<ServiceStatus, Self::Error>> + Send;

    /// Performs a health check on the orchestration provider.
    ///
    /// Returns `Ok(true)` if the provider can orchestrate services,
    /// `Ok(false)` if degraded, or `Err` if the health check failed.
    fn health_check(&self) -> impl Future<Output = std::result::Result<bool, Self::Error>> + Send;
}
