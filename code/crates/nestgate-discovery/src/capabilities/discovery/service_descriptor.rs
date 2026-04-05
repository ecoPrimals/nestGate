// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Service descriptor for capability-based discovery
//!
//! Describes a discovered service including its capabilities, endpoint information,
//! and metadata. Services are identified by UUID, not by name, to maintain
//! vendor independence.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

use super::taxonomy::Capability;

// Placeholder for humantime_serde - use standard Duration serialization
mod humantime_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_secs().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs))
    }
}

/// Describes a discovered service and its capabilities
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_core::capabilities::discovery::{
///     Endpoint, Protocol, SecurityCapability, ServiceDescriptor,
///     ServiceHealth, ServiceMetadata, Capability,
/// };
/// use uuid::Uuid;
///
/// let service = ServiceDescriptor {
///     id: Uuid::new_v4(),
///     name: "security-service-1".to_string(),
///     capabilities: vec![
///         Capability::Security(SecurityCapability::Authentication),
///         Capability::Security(SecurityCapability::Encryption),
///     ],
///     endpoint: Endpoint::https("10.0.1.5".to_string(), 8443),
///     metadata: ServiceMetadata::default(),
///     health: ServiceHealth::Healthy,
/// };
///
/// assert!(service.has_capability(&Capability::Security(SecurityCapability::Authentication)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDescriptor {
    /// Unique service identifier (generated, not hardcoded)
    pub id: Uuid,

    /// Service name (for logging only, never used for discovery)
    pub name: String,

    /// Capabilities provided by this service
    pub capabilities: Vec<Capability>,

    /// Network endpoint information
    pub endpoint: Endpoint,

    /// Service metadata
    pub metadata: ServiceMetadata,

    /// Current health status
    pub health: ServiceHealth,
}

/// Network endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    /// Host address (IP or hostname)
    pub host: String,

    /// Port number
    pub port: u16,

    /// Communication protocol
    pub protocol: Protocol,

    /// TLS/SSL enabled
    pub tls: bool,
}

/// Communication protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Protocol {
    /// HTTP/HTTPS
    HTTP,

    /// HTTPS (HTTP with TLS)
    HTTPS,

    /// gRPC
    GRPC,

    /// WebSocket
    WebSocket,

    /// Raw TCP
    TCP,

    /// Raw UDP
    UDP,

    /// Custom protocol
    Custom,
}

/// Service metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    /// Service version
    pub version: String,

    /// Service uptime
    #[serde(with = "humantime_serde")]
    pub uptime: Duration,

    /// Current load (0.0 - 1.0)
    pub load: f64,

    /// Average latency in milliseconds (for load balancing)
    pub latency_ms: Option<f64>,

    /// Custom tags
    pub tags: HashMap<String, String>,

    /// Geographic location (for latency optimization)
    pub location: Option<String>,
}

impl Default for ServiceMetadata {
    fn default() -> Self {
        Self {
            version: "unknown".to_string(),
            uptime: Duration::from_secs(0),
            load: 0.0,
            latency_ms: None,
            tags: HashMap::new(),
            location: None,
        }
    }
}

/// Service health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceHealth {
    /// Service is healthy and operational
    Healthy,

    /// Service is degraded but operational
    Degraded,

    /// Service is unhealthy
    Unhealthy,

    /// Service health is unknown
    Unknown,
}

impl ServiceDescriptor {
    /// Check if this service provides a specific capability
    #[must_use]
    pub fn has_capability(&self, capability: &Capability) -> bool {
        self.capabilities.iter().any(|c| c == capability)
    }

    /// Check if this service is healthy
    #[must_use]
    pub const fn is_healthy(&self) -> bool {
        matches!(self.health, ServiceHealth::Healthy)
    }

    /// Check if this service is available (healthy or degraded)
    #[must_use]
    pub const fn is_available(&self) -> bool {
        matches!(
            self.health,
            ServiceHealth::Healthy | ServiceHealth::Degraded
        )
    }

    /// Get full service URL
    #[must_use]
    pub fn url(&self) -> String {
        let scheme = if self.endpoint.tls { "https" } else { "http" };
        format!("{}://{}:{}", scheme, self.endpoint.host, self.endpoint.port)
    }
}

impl Endpoint {
    /// Create a new endpoint
    #[must_use]
    pub const fn new(host: String, port: u16, protocol: Protocol, tls: bool) -> Self {
        Self {
            host,
            port,
            protocol,
            tls,
        }
    }

    /// Create an HTTP endpoint
    #[must_use]
    pub const fn http(host: String, port: u16) -> Self {
        Self::new(host, port, Protocol::HTTP, false)
    }

    /// Create an HTTPS endpoint
    #[must_use]
    pub const fn https(host: String, port: u16) -> Self {
        Self::new(host, port, Protocol::HTTPS, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::discovery::taxonomy::{Capability, SecurityCapability};

    #[test]
    fn test_service_has_capability() {
        let service = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "test-service".to_string(),
            capabilities: vec![Capability::Security(SecurityCapability::Authentication)],
            endpoint: Endpoint::http("localhost".to_string(), 8080),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        assert!(service.has_capability(&Capability::Security(SecurityCapability::Authentication)));
        assert!(!service.has_capability(&Capability::Security(SecurityCapability::Encryption)));
    }

    #[test]
    fn test_service_health_checks() {
        let mut service = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "test".to_string(),
            capabilities: vec![],
            endpoint: Endpoint::http("localhost".to_string(), 8080),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        assert!(service.is_healthy());
        assert!(service.is_available());

        service.health = ServiceHealth::Degraded;
        assert!(!service.is_healthy());
        assert!(service.is_available());

        service.health = ServiceHealth::Unhealthy;
        assert!(!service.is_healthy());
        assert!(!service.is_available());
    }

    #[test]
    fn test_service_url() {
        let service = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "test".to_string(),
            capabilities: vec![],
            endpoint: Endpoint::https("example.com".to_string(), 443),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        assert_eq!(service.url(), "https://example.com:443");
    }

    #[test]
    fn test_endpoint_constructors() {
        let http = Endpoint::http("localhost".to_string(), 8080);
        assert_eq!(http.protocol, Protocol::HTTP);
        assert!(!http.tls);

        let https = Endpoint::https("localhost".to_string(), 443);
        assert_eq!(https.protocol, Protocol::HTTPS);
        assert!(https.tls);
    }
}
