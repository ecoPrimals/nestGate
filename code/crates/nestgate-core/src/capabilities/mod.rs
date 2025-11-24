//! 🌱 **CAPABILITIES MODULE**
//!
//! Capability-based discovery system for the ecoPrimals ecosystem.
//! Replaces ALL hardcoded primal, vendor, and service names.

/// Capability discovery mechanisms for ecosystem services
pub mod discovery;
/// Capability-based routing for dynamic service resolution
pub mod routing;

#[cfg(test)]
mod integration_tests; // Nov 23, 2025 - P1 test expansion
/// Capability taxonomy and classification system
pub mod taxonomy;

pub use taxonomy::{Capability, CapabilityCategory, CapabilityType};
