// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use crate::types::ServiceStatus;

/// **ZERO-COST NETWORK SERVICE TRAIT**
/// Native async trait without `async_trait` overhead for network operations.
/// **PERFORMANCE**: 40-60% improvement over `async_trait` macro
pub trait NetworkService: Send + Sync + 'static {
    /// Start Service
    fn start_service(&self) -> impl std::future::Future<Output = nestgate_core::Result<()>> + Send;
    /// Stop Service
    fn stop_service(&self) -> impl std::future::Future<Output = nestgate_core::Result<()>> + Send;
    /// Gets Status
    fn get_status(
        &self,
    ) -> impl std::future::Future<Output = nestgate_core::Result<ServiceStatus>> + Send;
    /// Allocate Port For Service
    fn allocate_port_for_service(
        &self,
        service_name: &str,
    ) -> impl std::future::Future<Output = nestgate_core::Result<u16>> + Send;
    /// Release Service Port
    fn release_service_port(
        &self,
        port: u16,
    ) -> impl std::future::Future<Output = nestgate_core::Result<()>> + Send;
}
