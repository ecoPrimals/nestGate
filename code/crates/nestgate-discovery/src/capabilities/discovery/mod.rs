// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Capability-based service discovery
//!
//! This module implements the capability-based discovery system that replaces
//! hardcoded primal name references throughout the codebase.
//!
//! # Architecture
//!
//! The discovery system is built on five core components:
//!
//! 1. **`CapabilityRegistry`** - Central registry of service capabilities
//! 2. **`ServiceDetector`** - Auto-discovers services and their capabilities
//! 3. **`ServiceResolver`** - Resolves capabilities to connections
//! 4. **`CapabilityTaxonomy`** - Hierarchical capability organization
//! 5. **`ServiceDescriptor`** - Metadata about discovered services
//!
//! # Example
//!
//! ```rust,ignore
//! use nestgate_core::capabilities::discovery::{CapabilityRegistry, Capability, SecurityCapability};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create registry
//! let registry = CapabilityRegistry::new();
//!
//! // Find services by capability (NOT by name!)
//! let security_services = registry
//!     .find_providers(&Capability::Security(SecurityCapability::Authentication))
//!     .await;
//!
//! // Use any service providing the capability
//! if let Some(service) = security_services.first() {
//!     println!("Security service at: {}", service.url());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Zero Hardcoding Principle
//!
//! This implementation follows the "Zero Hardcoding" principle from
//! `PRIMAL_ECOSYSTEM_INTEGRATION_SPEC.md`. Services are discovered by
//! **capability**, not by name. This ensures:
//!
//! - ✅ Vendor independence
//! - ✅ Dynamic service substitution
//! - ✅ Load balancing across providers
//! - ✅ Sovereignty compliance
//!
//! # Migration from Hardcoded Names
//!
//! Old code with hardcoded primal names:
//!
//! ```rust,ignore
//! // ❌ VIOLATES SPEC - hardcoded name
//! if service_name == "beardog" {
//!     // security operations
//! }
//! ```
//!
//! New code with capability-based discovery:
//!
//! ```rust,ignore
//! // ✅ SPECIFICATION COMPLIANT
//! if service.has_capability(&Capability::Security) {
//!     // security operations
//! }
//! ```

pub mod detector;
#[deprecated(
    since = "0.3.0",
    note = "Service registry and orchestration discovery are orchestration-provider concerns. NestGate retains only capability-based peer lookup via env and JSON-RPC IPC."
)]
/// Orchestration capability taxonomy and discovery helpers; orchestration classification belongs with the ecosystem orchestration layer.
pub mod orchestration;
pub mod registry;
pub mod resolver;
pub mod service_descriptor;
pub mod taxonomy;

pub use detector::ServiceDetector;
pub use registry::CapabilityRegistry;
pub use resolver::ServiceResolver;
pub use service_descriptor::{
    Endpoint, Protocol, ServiceDescriptor, ServiceHealth, ServiceMetadata,
};
pub use taxonomy::{AICapability, Capability, NetworkingCapability, SecurityCapability};

/// Result type for capability operations
pub type CapabilityResult<T> = Result<T, CapabilityError>;

/// Alias for backward compatibility during migration phase.
///
/// Alias for compatibility during migration to `CapabilityRegistry`.
pub type CapabilityDiscovery = CapabilityRegistry;

/// Alias for backward compatibility.
pub type DiscoveryManager = CapabilityRegistry;

/// Errors that can occur during capability operations
#[derive(Debug, thiserror::Error)]
pub enum CapabilityError {
    /// No service found providing the requested capability
    #[error("No service found providing capability: {0:?}")]
    NoProvider(Capability),

    /// Service detection failed
    #[error("Service detection failed: {0}")]
    DetectionFailed(String),

    /// Connection to service failed
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// Invalid capability specification
    #[error("Invalid capability: {0}")]
    InvalidCapability(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_error_display() {
        let err =
            CapabilityError::NoProvider(Capability::Security(SecurityCapability::Authentication));
        assert!(err.to_string().contains("No service found"));
    }

    #[test]
    fn test_capability_error_no_provider() {
        let err =
            CapabilityError::NoProvider(Capability::Security(SecurityCapability::Authentication));
        assert!(matches!(err, CapabilityError::NoProvider(_)));
        assert!(err.to_string().contains("No service found"));
        assert!(err.to_string().contains("Authentication"));
    }

    #[test]
    fn test_capability_error_detection_failed() {
        let err = CapabilityError::DetectionFailed("Network timeout".to_string());
        assert!(err.to_string().contains("Service detection failed"));
        assert!(err.to_string().contains("Network timeout"));
    }

    #[test]
    fn test_capability_error_connection_failed() {
        let err = CapabilityError::ConnectionFailed("Connection refused".to_string());
        assert!(err.to_string().contains("Connection failed"));
        assert!(err.to_string().contains("Connection refused"));
    }

    #[test]
    fn test_capability_error_invalid_capability() {
        let err = CapabilityError::InvalidCapability("Unknown capability".to_string());
        assert!(err.to_string().contains("Invalid capability"));
        assert!(err.to_string().contains("Unknown capability"));
    }

    #[test]
    fn test_discovery_manager_type_alias() {
        // Verify type alias exists for backward compatibility
        // This ensures existing code continues to work during migration
        let _type_check: Option<DiscoveryManager> = None;
        // If this compiles, the type alias is working correctly
    }
}
