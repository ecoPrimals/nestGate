// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Capability categories, service advertisements, and request/response payloads.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

// ==================== CAPABILITY CATEGORIES ====================

/// Universal capability categories that any primal can provide
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Capabilitycategory
pub enum CapabilityCategory {
    /// Storage capabilities (NestGate's primary domain)
    Storage,
    /// Orchestration capabilities (service mesh, workflow management)
    Orchestration,
    /// Compute capabilities (processing, containers, functions)
    Compute,
    /// Security capabilities (auth, encryption, access control)
    Security,
    /// Intelligence capabilities (AI, ML, analytics)
    Intelligence,
    /// Management capabilities (deployment, monitoring, configuration)
    Management,
    /// Network capabilities (routing, load balancing, discovery)
    Network,
    /// Data capabilities (databases, caching, streaming)
    Data,
}

impl CapabilityCategory {
    /// Convert to PrimalCapability for service registry discovery
    #[must_use]
    pub fn to_primal_capability(
        &self,
    ) -> crate::universal_primal_discovery::capability_based_discovery::PrimalCapability {
        use crate::universal_primal_discovery::capability_based_discovery::PrimalCapability;
        match self {
            Self::Storage => PrimalCapability::ZfsStorage,
            Self::Orchestration => PrimalCapability::Custom("orchestration".to_string()),
            Self::Compute => PrimalCapability::Custom("compute".to_string()),
            Self::Security => PrimalCapability::Authentication,
            Self::Intelligence => PrimalCapability::Custom("intelligence".to_string()),
            Self::Management => PrimalCapability::Custom("management".to_string()),
            Self::Network => PrimalCapability::Custom("network".to_string()),
            Self::Data => PrimalCapability::DataSync,
        }
    }
}

/// Specific capability that a service provides
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicecapability
pub struct ServiceCapability {
    /// Unique capability identifier
    pub id: Uuid,
    /// Category of capability
    pub category: CapabilityCategory,
    /// Specific operation name
    pub operation: String,
    /// Human-readable description
    pub description: String,
    /// Version of this capability
    pub version: String,
    /// Required parameters for this capability
    pub required_parameters: Vec<String>,
    /// Optional parameters
    pub optional_parameters: Vec<String>,
    /// Expected response format
    pub response_format: String,
}

impl ServiceCapability {
    /// Create a new service capability
    #[must_use]
    pub fn new(category: CapabilityCategory, operation: &str, description: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            category,
            operation: operation.to_string(),
            description: description.to_string(),
            version: "1.0.0".to_string(),
            required_parameters: Vec::new(),
            optional_parameters: Vec::new(),
            response_format: "json".to_string(),
        }
    }

    /// Create a storage capability (NestGate's domain)
    #[must_use]
    pub fn storage(operation: &str, description: &str) -> Self {
        Self::new(CapabilityCategory::Storage, operation, description)
    }
}

// ==================== CAPABILITY REQUESTS ====================

/// Request for a specific capability
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Capability operation
pub struct CapabilityRequest {
    /// Request ID for tracking
    pub request_id: Uuid,
    /// Category of capability needed
    pub category: CapabilityCategory,
    /// Specific operation requested
    pub operation: String,
    /// Parameters for the operation
    pub parameters: HashMap<String, serde_json::Value>,
    /// Timeout for the request
    pub timeout_seconds: u64,
    /// Whether this request is required or optional
    pub required: bool,
}

impl CapabilityRequest {
    /// Create a new capability request
    #[must_use]
    pub fn new(category: CapabilityCategory, operation: &str) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            category,
            operation: operation.to_string(),
            parameters: HashMap::new(),
            timeout_seconds: 30,
            required: true,
        }
    }

    /// Add a parameter to the request
    #[must_use]
    pub fn with_parameter(mut self, key: &str, value: serde_json::Value) -> Self {
        self.parameters.insert(key.to_string(), value);
        self
    }

    /// Make this request optional
    #[must_use]
    pub const fn optional(mut self) -> Self {
        self.required = false;
        self
    }

    /// Set timeout for this request
    #[must_use]
    pub const fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }
}

// ==================== CAPABILITY RESPONSES ====================

/// Response from a capability provider
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Capability operation
pub struct CapabilityResponse {
    /// Request ID this responds to
    pub request_id: Uuid,
    /// Whether the operation was successful
    pub success: bool,
    /// Response data
    pub data: serde_json::Value,
    /// Error message if unsuccessful
    pub error: Option<String>,
    /// Metadata about the response
    pub metadata: HashMap<String, String>,
    /// Time the operation took
    pub execution_time_ms: u64,
}

impl CapabilityResponse {
    /// Create a successful response
    #[must_use]
    pub fn success(request_id: Uuid, data: serde_json::Value) -> Self {
        Self {
            request_id,
            success: true,
            data,
            error: None,
            metadata: HashMap::new(),
            execution_time_ms: 0,
        }
    }

    /// Create an error response
    #[must_use]
    pub fn error(request_id: Uuid, error: String) -> Self {
        Self {
            request_id,
            success: false,
            data: serde_json::Value::Null,
            error: Some(error),
            metadata: HashMap::new(),
            execution_time_ms: 0,
        }
    }
}

// ==================== DISCOVERED SERVICE ====================

/// Information about a discovered service
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Service implementation for Discovered
pub struct DiscoveredService {
    /// Service identifier
    pub service_id: Uuid,
    /// Service name (generic, not primal-specific)
    pub name: String,
    /// Service type (generic description)
    pub service_type: String,
    /// Endpoint for communication
    pub endpoint: String,
    /// Capabilities this service provides
    pub capabilities: Vec<ServiceCapability>,
    /// Last time this service was seen
    pub last_seen: SystemTime,
    /// Health status
    pub healthy: bool,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl DiscoveredService {
    /// Create a new discovered service
    #[must_use]
    pub fn new(name: &str, service_type: &str, endpoint: &str) -> Self {
        Self {
            service_id: Uuid::new_v4(),
            name: name.to_string(),
            service_type: service_type.to_string(),
            endpoint: endpoint.to_string(),
            capabilities: Vec::new(),
            last_seen: SystemTime::now(),
            healthy: true,
            metadata: HashMap::new(),
        }
    }

    /// Add a capability to this service
    #[must_use]
    pub fn with_capability(mut self, capability: ServiceCapability) -> Self {
        self.capabilities.push(capability);
        self
    }

    /// Check if this service provides a specific capability
    #[must_use]
    pub fn provides_capability(&self, category: &CapabilityCategory, operation: &str) -> bool {
        self.capabilities
            .iter()
            .any(|cap| cap.category == *category && cap.operation == operation)
    }
}
