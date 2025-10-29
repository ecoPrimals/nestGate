//! Canonical Traits for `NestGate`
//!
//! Unified trait system that consolidates all service interfaces across
//! the `NestGate` ecosystem into consistent, canonical patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::Result;

// Use local result type
pub type NestGateResult<T> = Result<T>;

/// Service health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub healthy: bool,
    pub message: String,
    pub details: HashMap<String, String>,
}
/// **CANONICAL SERVICE TRAIT** - Native async implementation
/// Replaces `async_trait` with zero-cost native async patterns
pub trait CanonicalService: Send + Sync + 'static {
    /// Service health status type
    type Health: Clone + Send + Sync + 'static;
    /// Service configuration type
    type Config: Clone + Send + Sync + 'static;

    /// Health check - native async, no boxing overhead
    fn health_check(
        &self,
    ) -> impl std::future::Future<Output = NestGateResult<ServiceHealth>> + Send;

    /// Start the service - native async
    fn start(
        &mut self,
        config: Self::Config,
    ) -> impl std::future::Future<Output = NestGateResult<()>> + Send;

    /// Stop the service - native async
    fn stop(&mut self) -> impl std::future::Future<Output = NestGateResult<()>> + Send;
}

/// **CANONICAL STORAGE TRAIT** - Native implementation
pub trait CanonicalStorage: Send + Sync + 'static {
    /// Read data - native async
    fn read(&self, path: &str)
        -> impl std::future::Future<Output = NestGateResult<Vec<u8>>> + Send;
    /// Write data - native async
    fn write(
        &self,
        path: &str,
        data: &[u8],
    ) -> impl std::future::Future<Output = NestGateResult<()>> + Send;

    /// List directory - native async
    fn list(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = NestGateResult<Vec<String>>> + Send;
}

/// **CANONICAL NETWORK TRAIT** - Native implementation
pub trait CanonicalNetwork: Send + Sync + 'static {
    /// Send data - native async
    fn send(
        &self,
        endpoint: &str,
        data: &[u8],
    ) -> impl std::future::Future<Output = NestGateResult<Vec<u8>>> + Send;
    /// Receive data - native async
    fn receive(&self) -> impl std::future::Future<Output = NestGateResult<Vec<u8>>> + Send;
}

/// **CANONICAL SECURITY TRAIT** - Native implementation
pub trait CanonicalSecurity: Send + Sync + 'static {
    /// Authenticate user - native async
    fn authenticate(
        &self,
        credentials: &str,
    ) -> impl std::future::Future<Output = NestGateResult<bool>> + Send;
    /// Encrypt data - native async
    fn encrypt(
        &self,
        data: &[u8],
    ) -> impl std::future::Future<Output = NestGateResult<Vec<u8>>> + Send;

    /// Decrypt data - native async
    fn decrypt(
        &self,
        data: &[u8],
    ) -> impl std::future::Future<Output = NestGateResult<Vec<u8>>> + Send;
}

/// **CANONICAL AUTOMATION TRAIT** - Native implementation
pub trait CanonicalAutomation: Send + Sync + 'static {
    /// Execute workflow - native async
    fn execute_workflow(
        &self,
        workflow: &str,
    ) -> impl std::future::Future<Output = NestGateResult<String>> + Send;
    /// Schedule task - native async
    fn schedule_task(
        &self,
        task: &str,
        schedule: &str,
    ) -> impl std::future::Future<Output = NestGateResult<String>> + Send;
}

/// **CANONICAL MCP TRAIT** - Native implementation
pub trait CanonicalMcp: Send + Sync + 'static {
    /// Handle MCP message - native async
    fn handle_message(
        &self,
        message: &str,
    ) -> impl std::future::Future<Output = NestGateResult<String>> + Send;
    /// Send MCP response - native async
    fn send_response(
        &self,
        response: &str,
    ) -> impl std::future::Future<Output = NestGateResult<()>> + Send;
}

// ==================== CANONICAL TRAIT IMPLEMENTATIONS ====================

/// Universal Service Implementation
///
/// Provides a default implementation that can be used across all service types
/// MODERNIZED: Zero-cost abstractions with compile-time dispatch
pub struct UniversalServiceImpl {
    pub service_type: crate::types::UnifiedServiceType,
    pub capabilities: Vec<crate::types::CapabilityId>,
    pub health_status: ServiceHealth,
}
impl UniversalServiceImpl {
    #[must_use]
    pub fn new(
        service_type: crate::types::UnifiedServiceType,
        capabilities: Vec<crate::types::CapabilityId>,
        health_status: ServiceHealth,
    ) -> Self {
        Self {
            service_type,
            capabilities,
            health_status: ServiceHealth {
                healthy: health_status.healthy,
                message: health_status.message,
                details: health_status.details,
            },
        }
    }
}

impl CanonicalService for UniversalServiceImpl {
    type Health = ServiceHealth;
    type Config = HashMap<String, String>;

    async fn health_check(&self) -> NestGateResult<ServiceHealth> {
        Ok(ServiceHealth {
            healthy: true,
            message: "Service is operational".to_string(),
            details: HashMap::new(),
        })
    }

    async fn start(&mut self, _config: Self::Config) -> NestGateResult<()> {
        // Implementation for starting the service
        Ok(())
    }

    async fn stop(&mut self) -> NestGateResult<()> {
        // Implementation for stopping the service
        self.health_status = ServiceHealth {
            healthy: false,
            message: "Service stopped".to_string(),
            details: HashMap::new(),
        };
        Ok(())
    }
}

// ==================== CANONICAL TRAIT EXPORTS ====================

// Note: Traits are already public in this module, no need for re-exports
// The traits CanonicalServiceProvider, CanonicalStorageProvider, etc. are
// automatically available when this module is imported
