//! Capability struct and related implementations
//!
//! This module defines the Capability struct which represents a discovered capability
//! with its metadata, endpoint, and confidence information.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types::CapabilityType;

/// Discovered capability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    /// Type of capability
    pub capability_type: CapabilityType,
    /// Endpoint URL (discovered at runtime)
    pub endpoint: String,
    /// Provider name (discovered, not assumed)
    pub provider: Option<String>,
    /// Version information
    pub version: Option<String>,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Capability {
    /// Create a new capability
    #[must_use]
    pub fn new(capability_type: CapabilityType, endpoint: String) -> Self {
        Self {
            capability_type,
            endpoint,
            provider: None,
            version: None,
            confidence: 1.0,
            metadata: HashMap::new(),
        }
    }

    /// Set provider information
    #[must_use]
    pub fn with_provider(mut self, provider: String) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Set version information
    #[must_use]
    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    /// Set confidence level
    #[must_use]
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Add metadata
    #[must_use]
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_new() {
        let cap = Capability::new(
            CapabilityType::DataStorage,
            "http://storage:9000".to_string(),
        );

        assert_eq!(cap.capability_type, CapabilityType::DataStorage);
        assert_eq!(cap.endpoint, "http://storage:9000");
        assert_eq!(cap.provider, None);
        assert_eq!(cap.version, None);
        assert_eq!(cap.confidence, 1.0);
        assert!(cap.metadata.is_empty());
    }

    #[test]
    fn test_capability_builder() {
        let cap = Capability::new(
            CapabilityType::Orchestration,
            "http://discovered-service:8080".to_string(),
        )
        .with_provider("discovered-provider".to_string())
        .with_version("1.0.0".to_string())
        .with_confidence(0.95)
        .with_metadata("region".to_string(), "us-west".to_string());

        assert_eq!(cap.capability_type, CapabilityType::Orchestration);
        assert_eq!(cap.endpoint, "http://discovered-service:8080");
        assert_eq!(cap.provider, Some("discovered-provider".to_string()));
        assert_eq!(cap.confidence, 0.95);
        assert_eq!(cap.metadata.get("region"), Some(&"us-west".to_string()));
    }

    #[test]
    fn test_capability_with_provider() {
        let cap = Capability::new(CapabilityType::Security, "http://security:8443".to_string())
            .with_provider("secure-provider".to_string());

        assert_eq!(cap.provider, Some("secure-provider".to_string()));
    }

    #[test]
    fn test_capability_with_version() {
        let cap = Capability::new(CapabilityType::Compute, "http://compute:5000".to_string())
            .with_version("2.1.0".to_string());

        assert_eq!(cap.version, Some("2.1.0".to_string()));
    }

    #[test]
    fn test_capability_with_confidence() {
        let cap = Capability::new(
            CapabilityType::ArtificialIntelligence,
            "http://ai:3000".to_string(),
        )
        .with_confidence(0.85);

        assert_eq!(cap.confidence, 0.85);
    }

    #[test]
    fn test_capability_confidence_bounds() {
        let cap_low = Capability::new(
            CapabilityType::DataStorage,
            "http://storage:9000".to_string(),
        )
        .with_confidence(0.0);

        let cap_high = Capability::new(
            CapabilityType::DataStorage,
            "http://storage:9000".to_string(),
        )
        .with_confidence(1.0);

        assert_eq!(cap_low.confidence, 0.0);
        assert_eq!(cap_high.confidence, 1.0);
    }

    #[test]
    fn test_capability_with_multiple_metadata() {
        let cap = Capability::new(
            CapabilityType::MessageQueue,
            "http://queue:5672".to_string(),
        )
        .with_metadata("region".to_string(), "eu-central".to_string())
        .with_metadata("env".to_string(), "production".to_string())
        .with_metadata("tier".to_string(), "premium".to_string());

        assert_eq!(cap.metadata.len(), 3);
        assert_eq!(cap.metadata.get("region"), Some(&"eu-central".to_string()));
        assert_eq!(cap.metadata.get("env"), Some(&"production".to_string()));
        assert_eq!(cap.metadata.get("tier"), Some(&"premium".to_string()));
    }

    #[test]
    fn test_capability_serialization() {
        let cap = Capability::new(CapabilityType::Security, "http://security:8080".to_string())
            .with_provider("auth-service".to_string())
            .with_version("1.2.3".to_string());

        let serialized = serde_json::to_string(&cap).expect("String operation failed");
        let deserialized: Capability =
            serde_json::from_str(&serialized).expect("Failed to convert from string");

        assert_eq!(cap.capability_type, deserialized.capability_type);
        assert_eq!(cap.endpoint, deserialized.endpoint);
        assert_eq!(cap.provider, deserialized.provider);
        assert_eq!(cap.version, deserialized.version);
    }

    #[test]
    fn test_capability_clone() {
        let cap = Capability::new(
            CapabilityType::ArtificialIntelligence,
            "http://ai:3000".to_string(),
        )
        .with_provider("ml-service".to_string());

        let cloned = cap.clone();

        assert_eq!(cap.capability_type, cloned.capability_type);
        assert_eq!(cap.endpoint, cloned.endpoint);
        assert_eq!(cap.provider, cloned.provider);
    }

    #[test]
    fn test_capability_empty_metadata() {
        let cap = Capability::new(CapabilityType::Management, "http://mgmt:8080".to_string());
        assert!(cap.metadata.is_empty());
    }

    #[test]
    fn test_empty_endpoint() {
        let cap = Capability::new(CapabilityType::Security, String::new());
        assert_eq!(cap.endpoint, "");
    }

    #[test]
    fn test_capability_metadata_immutability() {
        let cap1 = Capability::new(CapabilityType::Compute, "http://compute:8080".to_string());
        let mut cap2 = cap1.clone();

        cap2.metadata.insert("key".to_string(), "value".to_string());

        // Original should not be affected
        assert!(cap1.metadata.is_empty());
        assert_eq!(cap2.metadata.len(), 1);
    }

    #[test]
    fn test_capability_endpoint_protocols() {
        let protocols = vec![
            ("http", "http://service:80"),
            ("https", "https://service:443"),
            ("tcp", "tcp://service:9000"),
            ("grpc", "grpc://service:50051"),
            ("ws", "ws://service:8080"),
            ("wss", "wss://service:8443"),
        ];

        for (protocol, endpoint) in protocols {
            let cap = Capability::new(CapabilityType::DataStorage, endpoint.to_string());
            assert!(
                cap.endpoint.starts_with(protocol),
                "Endpoint should start with protocol: {}",
                protocol
            );
        }
    }

    #[test]
    fn test_capability_equality() {
        let cap1 = Capability::new(CapabilityType::Compute, "http://compute:9000".to_string());
        let cap2 = Capability::new(CapabilityType::Compute, "http://compute:9000".to_string());
        let cap3 = Capability::new(CapabilityType::Compute, "http://different:9000".to_string());

        assert_eq!(cap1.capability_type, cap2.capability_type);
        assert_eq!(cap1.endpoint, cap2.endpoint);
        assert_ne!(cap1.endpoint, cap3.endpoint);
    }

    #[test]
    fn test_capability_metadata() {
        let mut cap = Capability::new(CapabilityType::Security, "http://auth:8080".to_string());

        cap.metadata
            .insert("version".to_string(), "1.0.0".to_string());
        cap.metadata
            .insert("protocol".to_string(), "oauth2".to_string());

        assert_eq!(cap.metadata.get("version"), Some(&"1.0.0".to_string()));
        assert_eq!(cap.metadata.get("protocol"), Some(&"oauth2".to_string()));
        assert_eq!(cap.metadata.len(), 2);
    }

    #[test]
    fn test_multiple_endpoint_types() {
        let endpoints = vec![
            "http://service:8080",
            "https://secure-service:8443",
            "tcp://message-queue:5672",
            "grpc://api-service:9090",
            "internal://nestgate/storage",
        ];

        for endpoint in endpoints {
            let cap = Capability::new(CapabilityType::DataStorage, endpoint.to_string());
            assert_eq!(cap.endpoint, endpoint);
        }
    }

    #[test]
    fn test_all_storage_types() {
        let storage_types = vec![
            CapabilityType::DataStorage,
            CapabilityType::KeyValueStorage,
            CapabilityType::RelationalStorage,
            CapabilityType::DocumentStorage,
            CapabilityType::TimeSeriesStorage,
            CapabilityType::ObjectStorage,
        ];

        for storage_type in storage_types {
            let cap = Capability::new(storage_type.clone(), "http://storage:5000".to_string());
            assert!(matches!(
                cap.capability_type,
                CapabilityType::DataStorage
                    | CapabilityType::KeyValueStorage
                    | CapabilityType::RelationalStorage
                    | CapabilityType::DocumentStorage
                    | CapabilityType::TimeSeriesStorage
                    | CapabilityType::ObjectStorage
            ));
        }
    }
}
