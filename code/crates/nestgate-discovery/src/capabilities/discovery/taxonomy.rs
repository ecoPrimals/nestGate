// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Capability taxonomy for structured service discovery
//!
//! Defines the hierarchical organization of capabilities that services can provide.
//! This taxonomy enables fine-grained capability matching while maintaining flexibility.

use serde::{Deserialize, Serialize};

/// Core capability types for service discovery
///
/// Services are discovered and connected based on capabilities they provide,
/// not by their names. This ensures vendor independence and sovereignty.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Capability {
    /// Security-related capabilities
    Security(SecurityCapability),

    /// Networking capabilities
    Networking(NetworkingCapability),

    /// AI and machine learning capabilities
    AI(AICapability),

    /// Orchestration and service management
    Orchestration(OrchestrationCapability),

    /// Storage capabilities
    Storage(StorageCapability),

    /// Custom/extension capabilities
    Custom(String),
}

/// Security service capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecurityCapability {
    /// User authentication
    Authentication,

    /// Access control and authorization
    Authorization,

    /// Data encryption
    Encryption,

    /// Cryptographic key management
    KeyManagement,

    /// Threat detection and analysis
    ThreatDetection,

    /// Security audit logging
    AuditLogging,

    /// Certificate management
    CertificateManagement,
}

/// Networking service capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NetworkingCapability {
    /// TCP protocol support
    TCP,

    /// UDP protocol support
    UDP,

    /// HTTP/HTTPS support
    HTTP,

    /// WebSocket support
    WebSocket,

    /// gRPC support
    GRPC,

    /// Load balancing
    LoadBalancing,

    /// Service mesh integration
    ServiceMesh,

    /// DNS resolution
    DNS,
}

/// AI and machine learning capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AICapability {
    /// Model inference
    Inference,

    /// Model training
    Training,

    /// Model serving and deployment
    ModelServing,

    /// Feature extraction
    FeatureExtraction,

    /// Natural language processing
    NaturalLanguage,

    /// Computer vision
    ComputerVision,

    /// Reinforcement learning
    ReinforcementLearning,
}

/// Orchestration service capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrchestrationCapability {
    /// Container management
    ContainerManagement,

    /// Service scheduling
    ServiceScheduling,

    /// Resource allocation
    ResourceAllocation,

    /// Health monitoring
    HealthMonitoring,

    /// Auto-scaling
    AutoScaling,

    /// Service discovery
    ServiceDiscovery,
}

/// Storage service capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageCapability {
    /// Object storage
    ObjectStorage,

    /// Block storage
    BlockStorage,

    /// File system storage
    FileSystem,

    /// Database storage
    Database,

    /// Cache storage
    Cache,

    /// Snapshot management
    Snapshots,
}

impl Capability {
    /// Get a human-readable description of the capability
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Security(_) => "Security and authentication services",
            Self::Networking(_) => "Network protocol and communication services",
            Self::AI(_) => "Artificial intelligence and machine learning services",
            Self::Orchestration(_) => "Service orchestration and management",
            Self::Storage(_) => "Data storage and persistence services",
            Self::Custom(_) => "Custom capability",
        }
    }

    /// Check if this capability matches another (supports hierarchy)
    #[must_use]
    pub fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            // Exact match
            (a, b) if a == b => true,

            // Wildcard matches (if we implement them later)
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_equality() {
        let cap1 = Capability::Security(SecurityCapability::Authentication);
        let cap2 = Capability::Security(SecurityCapability::Authentication);
        let cap3 = Capability::Security(SecurityCapability::Authorization);

        assert_eq!(cap1, cap2);
        assert_ne!(cap1, cap3);
    }

    #[test]
    fn test_capability_description() {
        let cap = Capability::Security(SecurityCapability::Authentication);
        assert!(cap.description().contains("Security"));
    }

    #[test]
    fn test_capability_matches() {
        let cap1 = Capability::Security(SecurityCapability::Authentication);
        let cap2 = Capability::Security(SecurityCapability::Authentication);

        assert!(cap1.matches(&cap2));
    }

    #[test]
    fn test_capability_serialization() {
        let cap = Capability::AI(AICapability::Inference);
        let json = serde_json::to_string(&cap).unwrap();
        let deserialized: Capability = serde_json::from_str(&json).unwrap();

        assert_eq!(cap, deserialized);
    }
}
