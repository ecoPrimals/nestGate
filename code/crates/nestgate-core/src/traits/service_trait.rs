// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Core Service Trait for NestGate Ecosystem
//!
//! **MIGRATED FROM**: `traits_root::service` (November 7, 2025)
//! **CANONICAL**: This is the single source of truth for the Service trait
//! **STATUS**: Production-ready, native async

/// Core service trait for all NestGate services - **ZERO-COST NATIVE ASYNC**
///
/// This trait defines the fundamental lifecycle and operations that all NestGate
/// services must implement. It uses native async (RPITIT) for zero-cost abstractions.
///
/// # Examples
///
/// ```rust
/// use nestgate_core::traits::Service;
///
/// struct MyService;
///
/// impl Service for MyService {
///     fn name(&self) -> &'static str {
///         "my-service"
///     }
///     
///     fn initialize(&self) -> impl std::future::Future<Output = nestgate_core::Result<()>> + Send {
///         async move {
///             // Initialization logic
///             Ok(())
///         }
///     }
///     
///     fn start(&self) -> impl std::future::Future<Output = nestgate_core::Result<()>> + Send {
///         async move {
///             // Start logic
///             Ok(())
///         }
///     }
///     
///     fn stop(&self) -> impl std::future::Future<Output = nestgate_core::Result<()>> + Send {
///         async move {
///             // Stop logic
///             Ok(())
///         }
///     }
///     
///     fn health_check(&self) -> impl std::future::Future<Output = nestgate_core::Result<bool>> + Send {
///         async move {
///             Ok(true)
///         }
///     }
/// }
/// ```
pub trait Service: Send + Sync {
    /// Service name identifier
    fn name(&self) -> &'static str;

    /// Initialize the service
    ///
    /// This is called once during service creation to perform any necessary setup.
    fn initialize(&self) -> impl std::future::Future<Output = crate::Result<()>> + Send;

    /// Start the service
    ///
    /// This is called to begin service operation after initialization.
    fn start(&self) -> impl std::future::Future<Output = crate::Result<()>> + Send;

    /// Stop the service
    ///
    /// This is called to gracefully stop service operation.
    fn stop(&self) -> impl std::future::Future<Output = crate::Result<()>> + Send;

    /// Shutdown the service (alias for stop for backward compatibility)
    ///
    /// **MIGRATION NOTE**: This is an alias for `stop()`. Many implementations
    /// use `shutdown` as their method name. This provides compatibility during
    /// the transition to the canonical trait system.
    fn shutdown(&self) -> impl std::future::Future<Output = crate::Result<()>> + Send {
        self.stop()
    }

    /// Get service health status
    ///
    /// Returns `true` if the service is healthy and operational.
    fn health_check(&self) -> impl std::future::Future<Output = crate::Result<bool>> + Send;
}
