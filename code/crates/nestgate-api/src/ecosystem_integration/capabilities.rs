//! **CAPABILITIES MANAGEMENT**
//!
//! Capability management and matching for ecosystem services.

use std::collections::HashMap;
use super::types::{ServiceCapability, ServiceCategory};

/// Capability matcher for service discovery
pub struct CapabilityMatcher;
impl CapabilityMatcher {
    /// Check if a service capability matches requirements
    pub fn matches_requirement(
        capability: &ServiceCapability,
        requirement: &ServiceCapability,
    ) -> bool {
        match (capability, requirement) {
            (ServiceCapability::Storage { types: cap_types, .. }, 
             ServiceCapability::Storage { types: req_types, .. }) => {
                req_types.iter().all(|req_type| cap_types.contains(req_type))
            }
            (ServiceCapability::Compute { architectures: cap_archs, .. },
             ServiceCapability::Compute { architectures: req_archs, .. }) => {
                req_archs.iter().all(|req_arch| cap_archs.contains(req_arch))
            }
            (ServiceCapability::Custom { name: cap_name, .. },
             ServiceCapability::Custom { name: req_name, .. }) => {
                cap_name == req_name
            }
            _ => false,
        }
    }

    /// Score capability match quality (0.0 to 1.0)
    pub fn match_score(
        capability: &ServiceCapability,
        requirement: &ServiceCapability,
    ) -> f64 {
        if Self::matches_requirement(capability, requirement) {
            // Simple scoring - could be more sophisticated
            0.8
        } else {
            0.0
        }
    }
} 