use std::collections::HashMap;
//
// **CANONICAL MODERNIZATION**: Service metadata types and definitions
// that were previously scattered across different modules.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// **SERVICE CAPABILITY**
/// 
/// Represents a specific capability that a service provides
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Servicecapability
pub struct ServiceCapability {
    /// Capability name
    pub name: String,
    /// Capability version
    pub version: String,
    /// Capability description
    pub description: String,
    /// Capability metadata
    pub metadata: HashMap<String, String>,
    /// Whether this capability is enabled
    pub enabled: bool,
}
impl Default for ServiceCapability {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            name: "generic".to_string(),
            version: "1.0.0".to_string(),
            description: "Generic service capability".to_string(),
            metadata: HashMap::new(),
            enabled: true,
        }
    }
}

/// **UNIVERSAL SERVICE METADATA**
/// 
/// Comprehensive metadata for services in the NestGate ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Universalservicemetadata
pub struct UniversalServiceMetadata {
    /// Service identifier
    pub service_id: String,
    /// Service name
    pub service_name: String,
    /// Service version
    pub service_version: String,
    /// Service description
    pub description: String,
    /// Service capabilities
    pub capabilities: Vec<ServiceCapability>,
    /// Service endpoints
    pub endpoints: Vec<ServiceEndpoint>,
    /// Service dependencies
    pub dependencies: Vec<ServiceDependency>,
    /// Service configuration
    pub configuration: HashMap<String, String>,
    /// Service tags
    pub tags: Vec<String>,
    /// Service creation time
    pub created_at: SystemTime,
    /// Service last update time
    pub updated_at: SystemTime,
    /// Service status
    pub status: ServiceStatus,
}
impl Default for UniversalServiceMetadata {
    /// Returns the default instance
    fn default() -> Self {
        let now = SystemTime::now();
        Self {
            service_id: "unknown".to_string(),
            service_name: "Unknown Service".to_string(),
            service_version: "1.0.0".to_string(),
            description: "Unknown service".to_string(),
            capabilities: Vec::new(),
            endpoints: Vec::new(),
            dependencies: Vec::new(),
            configuration: HashMap::new(),
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            status: ServiceStatus::Unknown,
        }
    }
}

/// **SERVICE ENDPOINT**
/// 
/// Represents a service endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serviceendpoint
pub struct ServiceEndpoint {
    /// Endpoint name
    pub name: String,
    /// Endpoint URL or address
    pub endpoint: String,
    /// Endpoint port
    pub port: u16,
    /// Endpoint protocol
    pub protocol: String,
    /// Whether endpoint is secure
    pub secure: bool,
    /// Endpoint metadata
    pub metadata: HashMap<String, String>,
}
impl Default for ServiceEndpoint {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            endpoint: "localhost".to_string(),
            port: 8080,
            protocol: "http".to_string(),
            secure: false,
            metadata: HashMap::new(),
        }
    }
}

/// **SERVICE DEPENDENCY**
/// 
/// Represents a service dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicedependency
pub struct ServiceDependency {
    /// Dependency service name
    pub service_name: String,
    /// Required version
    pub version_requirement: String,
    /// Whether dependency is optional
    pub optional: bool,
    /// Dependency metadata
    pub metadata: HashMap<String, String>,
}
impl Default for ServiceDependency {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            service_name: "unknown".to_string(),
            version_requirement: "*".to_string(),
            optional: false,
            metadata: HashMap::new(),
        }
    }
}

/// **SERVICE STATUS**
/// 
/// Represents the current status of a service
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Status values for Service
pub enum ServiceStatus {
    /// Service is running
    Running,
    /// Service is stopped
    Stopped,
    /// Service is starting
    Starting,
    /// Service is stopping
    Stopping,
    /// Service is in maintenance
    Maintenance,
    /// Service has an error
    Error,
    /// Service status is unknown
    Unknown,
}
impl Default for ServiceStatus {
    /// Returns the default instance
    fn default() -> Self {
        Self::Unknown
    }
} 