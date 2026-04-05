// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Discovered Protocol
//!
//! Aggregates all discovered information about a storage endpoint.

use super::authentication::AuthenticationPattern;
use super::features::{FeatureSet, StorageFeature};
use super::operations::StorageOperationPattern;
use super::transport::TransportProtocol;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Discovered protocol information for a storage endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredProtocol {
    /// Transport mechanism
    pub transport: TransportProtocol,

    /// Storage operation pattern
    pub operation_pattern: StorageOperationPattern,

    /// Authentication mechanism
    pub authentication: AuthenticationPattern,

    /// Discovered features
    pub features: FeatureSet,

    /// API version/dialect information
    pub api_info: ApiInfo,

    /// Performance characteristics (if known)
    pub performance: Option<PerformanceInfo>,
}

impl DiscoveredProtocol {
    /// Create a new discovered protocol
    #[must_use]
    pub fn new(
        transport: TransportProtocol,
        operation_pattern: StorageOperationPattern,
        authentication: AuthenticationPattern,
    ) -> Self {
        Self {
            transport,
            operation_pattern,
            authentication,
            features: FeatureSet::new(),
            api_info: ApiInfo::default(),
            performance: None,
        }
    }

    /// Add a feature
    pub fn add_feature(&mut self, feature: StorageFeature) {
        self.features.add(feature);
    }

    /// Check if feature is supported
    #[must_use]
    pub fn has_feature(&self, feature: &StorageFeature) -> bool {
        self.features.has(feature)
    }

    /// Get a human-readable description
    #[must_use]
    pub fn description(&self) -> String {
        format!(
            "{} / {} / {} ({} features)",
            self.transport.description(),
            self.operation_pattern_name(),
            self.authentication.description(),
            self.features.len()
        )
    }

    /// Get operation pattern name
    const fn operation_pattern_name(&self) -> &str {
        match &self.operation_pattern {
            StorageOperationPattern::ObjectStore { .. } => "Object Store",
            StorageOperationPattern::BlockStore { .. } => "Block Store",
            StorageOperationPattern::FileSystem { .. } => "File System",
            StorageOperationPattern::KeyValue { .. } => "Key-Value",
            StorageOperationPattern::Document { .. } => "Document Store",
            StorageOperationPattern::Stream { .. } => "Stream",
        }
    }

    /// Is this protocol secure?
    #[must_use]
    pub const fn is_secure(&self) -> bool {
        self.transport.is_secure() && self.authentication.is_secure()
    }
}

/// API information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiInfo {
    /// API specification this follows (if any)
    ///
    /// Examples: "`OpenAPI` 3.0", "REST", "GraphQL", etc.
    pub specification: Option<String>,

    /// API version identifier
    ///
    /// Examples: "2006-03-01" (S3), "2020-12-06" (Azure), "v1", etc.
    pub version: Option<String>,

    /// Server identification
    pub server_info: Option<String>,

    /// Custom metadata about the API
    pub metadata: HashMap<String, String>,
}

impl ApiInfo {
    /// Create with specification and version
    pub fn new(specification: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            specification: Some(specification.into()),
            version: Some(version.into()),
            server_info: None,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata
    #[must_use]
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Performance characteristics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceInfo {
    /// Typical read latency
    pub read_latency_ms: Option<u64>,

    /// Typical write latency
    pub write_latency_ms: Option<u64>,

    /// Maximum throughput (bytes/sec)
    pub max_throughput_bytes_per_sec: Option<u64>,

    /// Maximum operations per second
    pub max_ops_per_sec: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::universal_storage::universal::{HttpVersion, ObjectAddressing, ObjectOrganization};

    #[test]
    fn test_discovered_protocol_creation() {
        let protocol = DiscoveredProtocol::new(
            TransportProtocol::Http {
                version: HttpVersion::Http1_1,
                tls: None,
            },
            StorageOperationPattern::ObjectStore {
                addressing: ObjectAddressing::PathBased,
                organization: ObjectOrganization::Hierarchical { separator: '/' },
            },
            AuthenticationPattern::None,
        );

        assert!(protocol.description().contains("HTTP/1.1"));
        assert!(protocol.description().contains("Object Store"));
    }

    #[test]
    fn test_api_info_builder() {
        let info = ApiInfo::new("REST", "v1")
            .with_metadata("vendor", "example")
            .with_metadata("region", "us-east-1");

        assert_eq!(info.specification, Some("REST".to_string()));
        assert_eq!(info.version, Some("v1".to_string()));
        assert_eq!(info.metadata.get("vendor"), Some(&"example".to_string()));
    }

    #[test]
    fn discovered_protocol_all_operation_pattern_names() {
        use crate::universal_storage::universal::authentication::AuthenticationPattern;
        use crate::universal_storage::universal::operations::{
            BlockAddressing, KeyFormat, QueryCapabilities, StorageOperationPattern, StreamOrdering,
        };
        use crate::universal_storage::universal::transport::{HttpVersion, TransportProtocol};

        let transport = TransportProtocol::Http {
            version: HttpVersion::Http1_1,
            tls: None,
        };
        let auth = AuthenticationPattern::None;

        for op in [
            StorageOperationPattern::ObjectStore {
                addressing: ObjectAddressing::PathBased,
                organization: ObjectOrganization::Flat,
            },
            StorageOperationPattern::BlockStore {
                block_size: 512,
                addressing: BlockAddressing::Sequential,
            },
            StorageOperationPattern::FileSystem {
                path_separator: '/',
                case_sensitive: false,
            },
            StorageOperationPattern::KeyValue {
                key_format: KeyFormat::default(),
            },
            StorageOperationPattern::Document {
                query_capabilities: QueryCapabilities::default(),
            },
            StorageOperationPattern::Stream {
                ordering: StreamOrdering::Unordered,
            },
        ] {
            let p = DiscoveredProtocol::new(transport.clone(), op, auth.clone());
            assert!(!p.description().is_empty());
            assert!(!p.is_secure());
        }
    }
}
