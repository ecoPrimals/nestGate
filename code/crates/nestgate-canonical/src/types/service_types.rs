// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Service Type Definitions
//!
//! Core service type classifications and capability identifiers.

use serde::{Deserialize, Serialize};

/// Canonical Service Type Classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of `UnifiedService`
pub enum UnifiedServiceType {
    /// Storage
    Storage,
    /// Network
    Network,
    /// Compute
    Compute,
    /// Security
    Security,
    /// Intelligence
    Intelligence,
    /// Orchestration
    Orchestration,
}

impl std::fmt::Display for UnifiedServiceType {
    /// Fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Storage => write!(f, "Storage"),
            Self::Network => write!(f, "Network"),
            Self::Compute => write!(f, "Compute"),
            Self::Security => write!(f, "Security"),
            Self::Intelligence => write!(f, "Intelligence"),
            Self::Orchestration => write!(f, "Orchestration"),
        }
    }
}

/// Canonical Capability Identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Capabilityid
pub struct CapabilityId {
    /// Domain
    pub domain: String,
    /// Capability
    pub capability: String,
    /// Version
    pub version: String,
}

impl CapabilityId {
    /// Create a new capability ID with pedantic validation
    #[must_use]
    pub const fn new(domain: String, capability: String, version: String) -> Self {
        Self {
            domain,
            capability,
            version,
        }
    }

    /// Get the capability domain
    #[must_use]
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Get the capability name
    #[must_use]
    pub fn capability(&self) -> &str {
        &self.capability
    }

    /// Get the capability version
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_service_type_display() {
        assert_eq!(UnifiedServiceType::Storage.to_string(), "Storage");
        assert_eq!(UnifiedServiceType::Network.to_string(), "Network");
        assert_eq!(UnifiedServiceType::Compute.to_string(), "Compute");
        assert_eq!(UnifiedServiceType::Security.to_string(), "Security");
        assert_eq!(UnifiedServiceType::Intelligence.to_string(), "Intelligence");
        assert_eq!(
            UnifiedServiceType::Orchestration.to_string(),
            "Orchestration"
        );
    }

    #[test]
    fn test_unified_service_type_equality() {
        assert_eq!(UnifiedServiceType::Storage, UnifiedServiceType::Storage);
        assert_ne!(UnifiedServiceType::Storage, UnifiedServiceType::Network);
    }

    #[test]
    fn test_unified_service_type_serialization() {
        let service_type = UnifiedServiceType::Storage;
        let json = serde_json::to_string(&service_type).expect("Failed to serialize");
        let deserialized: UnifiedServiceType =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(service_type, deserialized);
    }

    #[test]
    fn test_unified_service_type_all_variants() {
        let variants = vec![
            UnifiedServiceType::Storage,
            UnifiedServiceType::Network,
            UnifiedServiceType::Compute,
            UnifiedServiceType::Security,
            UnifiedServiceType::Intelligence,
            UnifiedServiceType::Orchestration,
        ];

        for variant in variants {
            let json = serde_json::to_string(&variant).expect("Failed to serialize");
            let deserialized: UnifiedServiceType =
                serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(variant, deserialized);
        }
    }

    #[test]
    fn test_capability_id_creation() {
        let cap_id = CapabilityId::new(
            "storage".to_string(),
            "zfs".to_string(),
            "1.0.0".to_string(),
        );

        assert_eq!(cap_id.domain(), "storage");
        assert_eq!(cap_id.capability(), "zfs");
        assert_eq!(cap_id.version(), "1.0.0");
    }

    #[test]
    fn test_capability_id_getters() {
        let cap_id = CapabilityId {
            domain: "network".to_string(),
            capability: "routing".to_string(),
            version: "2.1.0".to_string(),
        };

        assert_eq!(cap_id.domain(), "network");
        assert_eq!(cap_id.capability(), "routing");
        assert_eq!(cap_id.version(), "2.1.0");
    }

    #[test]
    fn test_capability_id_equality() {
        let cap_id1 = CapabilityId::new(
            "storage".to_string(),
            "zfs".to_string(),
            "1.0.0".to_string(),
        );

        let cap_id2 = CapabilityId::new(
            "storage".to_string(),
            "zfs".to_string(),
            "1.0.0".to_string(),
        );

        let cap_id3 = CapabilityId::new(
            "network".to_string(),
            "routing".to_string(),
            "1.0.0".to_string(),
        );

        assert_eq!(cap_id1, cap_id2);
        assert_ne!(cap_id1, cap_id3);
    }

    #[test]
    fn test_capability_id_serialization() {
        let cap_id = CapabilityId::new(
            "security".to_string(),
            "authentication".to_string(),
            "3.0.0".to_string(),
        );

        let json = serde_json::to_string(&cap_id).expect("Failed to serialize");
        let deserialized: CapabilityId =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(cap_id, deserialized);
        assert_eq!(cap_id.domain(), deserialized.domain());
        assert_eq!(cap_id.capability(), deserialized.capability());
        assert_eq!(cap_id.version(), deserialized.version());
    }

    #[test]
    fn test_capability_id_clone() {
        let cap_id = CapabilityId::new(
            "compute".to_string(),
            "processing".to_string(),
            "1.5.0".to_string(),
        );

        let cloned = cap_id.clone();
        assert_eq!(cap_id, cloned);
    }

    #[test]
    fn test_capability_id_debug() {
        let cap_id = CapabilityId::new(
            "intelligence".to_string(),
            "ml".to_string(),
            "0.9.0".to_string(),
        );

        let debug_str = format!("{:?}", cap_id);
        assert!(debug_str.contains("intelligence"));
        assert!(debug_str.contains("ml"));
        assert!(debug_str.contains("0.9.0"));
    }
}
