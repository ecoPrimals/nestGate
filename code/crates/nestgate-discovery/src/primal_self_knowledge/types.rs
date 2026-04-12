// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Data types for primal identity, capabilities, endpoints, and discovery.

use serde::{Deserialize, Serialize};

/// Our identity as a primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalIdentity {
    /// Unique identifier (generated at runtime)
    pub id: String,

    /// Primal type (e.g., "nestgate")
    pub primal_type: String,

    /// Semantic version
    pub version: String,

    /// When we started (birth time)
    pub started_at: std::time::SystemTime,
}

/// A capability we provide
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    /// Capability name (e.g., "storage", "zfs", "encryption")
    pub name: String,

    /// Description of what this capability does
    pub description: String,

    /// Endpoint where this capability is accessible
    pub endpoint: String,

    /// Metadata about this capability
    pub metadata: std::collections::HashMap<String, String>,
}

/// An endpoint where we can be reached
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    /// Protocol (http, grpc, websocket, etc.)
    pub protocol: String,

    /// Network address
    pub address: String,

    /// Port number (from environment or discovery, never hardcoded)
    pub port: u16,

    /// URL path (if applicable)
    pub path: Option<String>,

    /// Health check path
    pub health_path: Option<String>,
}

impl Endpoint {
    /// Get the full URL for this endpoint
    #[must_use]
    pub fn url(&self) -> String {
        let path = self.path.as_deref().unwrap_or("");
        format!("{}://{}:{}{}", self.protocol, self.address, self.port, path)
    }

    /// Get the health check URL
    #[must_use]
    pub fn health_url(&self) -> Option<String> {
        self.health_path.as_ref().map(|health_path| {
            format!(
                "{}://{}:{}{}",
                self.protocol, self.address, self.port, health_path
            )
        })
    }
}

/// Discovery mechanism for finding other primals
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiscoveryMechanism {
    /// Environment variables (explicit configuration)
    Environment,

    /// mDNS/Bonjour/Zeroconf
    MDns,

    /// DNS Service Discovery (RFC 6763)
    DnsSd,

    /// `HashiCorp` Consul
    Consul,

    /// Kubernetes service discovery
    Kubernetes,

    /// File-based configuration
    FileConfig,
}

/// A discovered primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPrimal {
    /// Primal identity
    pub identity: PrimalIdentity,

    /// Capabilities this primal provides
    pub capabilities: Vec<Capability>,

    /// Primary endpoint for this primal
    pub primary_endpoint: Endpoint,

    /// When we discovered this primal
    pub discovered_at: std::time::SystemTime,

    /// How we discovered it
    pub discovery_method: DiscoveryMechanism,
}

impl DiscoveredPrimal {
    /// Get the primary endpoint URL
    #[must_use]
    pub fn primary_endpoint(&self) -> String {
        self.primary_endpoint.url()
    }

    /// Check if this primal provides a specific capability
    #[must_use]
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities
            .iter()
            .any(|c| c.name.eq_ignore_ascii_case(capability))
    }
}
