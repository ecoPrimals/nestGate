// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Builder for constructing SelfKnowledge instances
//!
//! Provides a fluent API for building self-knowledge with validation.

use super::{HealthStatus, PrimalId, ResourceInfo, SelfKnowledge};
use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::SystemTime;

/// Builder for constructing [`SelfKnowledge`]
///
/// ## Example
///
/// ```rust,ignore
/// // Uses get_api_port/get_metrics_port - see tests for full example
/// use nestgate_core::self_knowledge::SelfKnowledgeBuilder;
/// let knowledge = SelfKnowledgeBuilder::new()
///     .with_id("nestgate")
///     .with_name("NestGate Storage")
///     .with_capability("storage")
///     .build()
///     .expect("valid config");
/// ```
#[derive(Debug, Default)]
pub struct SelfKnowledgeBuilder {
    id: Option<String>,
    name: Option<String>,
    version: Option<String>,
    capabilities: Vec<String>,
    endpoints: HashMap<String, SocketAddr>,
    resources: Option<ResourceInfo>,
    health: Option<HealthStatus>,
}

impl SelfKnowledgeBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the primal's unique identifier
    ///
    /// **Required**: This must be set before calling [`build()`](Self::build).
    ///
    /// **Convention**: Use lowercase primal name (e.g., "nestgate", "orchestrator", "beardog")
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the primal's human-readable name
    ///
    /// **Optional**: Defaults to titlecase of ID if not set.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the primal's version
    ///
    /// **Optional**: Defaults to "0.0.0" if not set.
    ///
    /// **Tip**: Use `env!("CARGO_PKG_VERSION")` to automatically use Cargo.toml version
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Add a capability this primal provides
    ///
    /// Can be called multiple times to add multiple capabilities.
    ///
    /// **Examples**: "storage", "orchestration", "ai", "security", "zfs"
    pub fn with_capability(mut self, capability: impl Into<String>) -> Self {
        self.capabilities.push(capability.into());
        self
    }

    /// Add multiple capabilities at once
    pub fn with_capabilities(
        mut self,
        capabilities: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.capabilities
            .extend(capabilities.into_iter().map(|c| c.into()));
        self
    }

    /// Add an endpoint where this primal is accessible
    ///
    /// **Parameters**:
    /// - `name`: Endpoint identifier ("api", "metrics", "websocket", etc.)
    /// - `addr`: Socket address where the endpoint listens
    ///
    /// Can be called multiple times to add multiple endpoints.
    pub fn with_endpoint(mut self, name: impl Into<String>, addr: SocketAddr) -> Self {
        self.endpoints.insert(name.into(), addr);
        self
    }

    /// Set resource information
    ///
    /// **Optional**: Defaults to reasonable values if not set.
    pub fn with_resources(mut self, resources: ResourceInfo) -> Self {
        self.resources = Some(resources);
        self
    }

    /// Set initial health status
    ///
    /// **Optional**: Defaults to [`HealthStatus::Starting`] if not set.
    pub fn with_health(mut self, health: HealthStatus) -> Self {
        self.health = Some(health);
        self
    }

    /// Build the SelfKnowledge instance
    ///
    /// ## Errors
    ///
    /// Returns an error if:
    /// - ID is not set (required)
    /// - ID is empty or contains invalid characters
    pub fn build(self) -> Result<SelfKnowledge> {
        // Validate required fields
        let id_str = self.id.context("Primal ID is required")?;

        if id_str.is_empty() {
            bail!("Primal ID cannot be empty");
        }

        if id_str.contains(char::is_whitespace) {
            bail!("Primal ID cannot contain whitespace");
        }

        let id = PrimalId::new(id_str.clone());

        // Generate defaults for optional fields
        let name = self.name.unwrap_or_else(|| {
            // Titlecase the ID as default name
            let mut chars = id_str.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        });

        let version = self.version.unwrap_or_else(|| "0.0.0".to_string());
        let resources = self.resources.unwrap_or_default();
        let health = self.health.unwrap_or_default();

        Ok(SelfKnowledge {
            id,
            name,
            version,
            capabilities: self.capabilities,
            endpoints: self.endpoints,
            resources,
            health,
            last_updated: SystemTime::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_builder() {
        let knowledge = SelfKnowledgeBuilder::new()
            .with_id("test")
            .build()
            .expect("Should build with just ID");

        assert_eq!(knowledge.id.as_str(), "test");
        assert_eq!(knowledge.name, "Test"); // Auto-generated from ID
        assert_eq!(knowledge.version, "0.0.0"); // Default version
    }

    #[test]
    fn test_full_builder() {
        let knowledge = SelfKnowledgeBuilder::new()
            .with_id("nestgate")
            .with_name("NestGate Storage")
            .with_version("1.0.0")
            .with_capability("storage")
            .with_capability("zfs")
            .with_endpoint(
                "api",
                format!(
                    "127.0.0.1:{}",
                    nestgate_config::constants::network_hardcoded::get_api_port()
                )
                .parse()
                .unwrap(),
            )
            .with_health(HealthStatus::Healthy)
            .build()
            .expect("Should build successfully");

        assert_eq!(knowledge.id.as_str(), "nestgate");
        assert_eq!(knowledge.name, "NestGate Storage");
        assert_eq!(knowledge.version, "1.0.0");
        assert_eq!(knowledge.capabilities.len(), 2);
        assert_eq!(knowledge.endpoints.len(), 1);
        assert_eq!(knowledge.health, HealthStatus::Healthy);
    }

    #[test]
    fn test_builder_requires_id() {
        let result = SelfKnowledgeBuilder::new().with_name("Test").build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("required"));
    }

    #[test]
    fn test_builder_rejects_empty_id() {
        let result = SelfKnowledgeBuilder::new().with_id("").build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[test]
    fn test_builder_rejects_whitespace_in_id() {
        let result = SelfKnowledgeBuilder::new().with_id("nest gate").build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("whitespace"));
    }

    #[test]
    fn test_multiple_capabilities() {
        let knowledge = SelfKnowledgeBuilder::new()
            .with_id("test")
            .with_capabilities(vec!["cap1", "cap2", "cap3"])
            .build()
            .unwrap();

        assert_eq!(knowledge.capabilities.len(), 3);
    }

    #[test]
    fn test_multiple_endpoints() {
        let knowledge = SelfKnowledgeBuilder::new()
            .with_id("test")
            .with_endpoint(
                "api",
                format!(
                    "127.0.0.1:{}",
                    nestgate_config::constants::network_hardcoded::get_api_port()
                )
                .parse()
                .unwrap(),
            )
            .with_endpoint(
                "metrics",
                format!(
                    "127.0.0.1:{}",
                    nestgate_config::constants::network_hardcoded::get_metrics_port()
                )
                .parse()
                .unwrap(),
            )
            .with_endpoint(
                "websocket",
                format!(
                    "127.0.0.1:{}",
                    nestgate_config::constants::network_hardcoded::get_websocket_port()
                )
                .parse()
                .unwrap(),
            )
            .build()
            .unwrap();

        assert_eq!(knowledge.endpoints.len(), 3);
        assert!(knowledge.get_endpoint("api").is_some());
        assert!(knowledge.get_endpoint("metrics").is_some());
        assert!(knowledge.get_endpoint("websocket").is_some());
    }
}
