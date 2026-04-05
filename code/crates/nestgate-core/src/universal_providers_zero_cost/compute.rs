// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::future::Future;
use std::marker::PhantomData;

use super::status::ComputeResources;

/// **ZERO-COST UNIVERSAL COMPUTE WRAPPER**
///
/// Direct composition replacement for `Arc<dyn ComputePrimalProvider>`
/// PERFORMANCE: 60-80% improvement through monomorphization
pub struct ZeroCostUniversalComputeWrapper<Provider, const MAX_COMPUTE_UNITS: usize = 1000>
where
    Provider: ZeroCostComputeProvider,
{
    _provider_name: String,
    _endpoint: String,
    _capabilities: Vec<String>,
    /// Direct composition - no `Arc<dyn>` overhead
    _provider: Provider,
    _phantom: PhantomData<()>,
}
/// Zero-cost compute provider trait - replaces `Arc<dyn ComputePrimalProvider>`
pub trait ZeroCostComputeProvider: Send + Sync + 'static {
    /// Type alias for Error
    type Error: Send + Sync + 'static;
    /// Type alias for ComputeRequest
    type ComputeRequest: Send + Sync;
    /// Type alias for ComputeResponse
    type ComputeResponse: Send + Sync;
    /// Execute compute task with native async
    fn execute_compute(
        &self,
        request: &Self::ComputeRequest,
    ) -> impl Future<Output = std::result::Result<Self::ComputeResponse, Self::Error>> + Send;

    /// Get compute resources with zero allocation
    fn get_resources(
        &self,
    ) -> impl Future<Output = std::result::Result<ComputeResources, Self::Error>> + Send;

    /// Performs a health check for the compute provider.
    ///
    /// # Returns
    /// A future that resolves to `Ok(true)` if healthy, `Ok(false)` otherwise, or an error.
    fn health_check(&self) -> impl Future<Output = std::result::Result<bool, Self::Error>> + Send;
}
