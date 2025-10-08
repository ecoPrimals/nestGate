//! 🌱 **CAPABILITIES MODULE**
//!
//! Capability-based discovery system for the ecoPrimals ecosystem.
//! Replaces ALL hardcoded primal, vendor, and service names.

pub mod discovery;
pub mod routing;
pub mod taxonomy;

pub use taxonomy::{Capability, CapabilityCategory, CapabilityType};
