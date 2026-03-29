// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Orchestrator auto-registration — stub until nestgate-discovery + nestgate-core wiring.

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

/// Service registration data for orchestrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistrationData {
    pub service_id: String,
    pub primal_name: String,
    pub capabilities: Vec<String>,
    pub version: String,
    pub endpoints: std::collections::HashMap<String, String>,
    pub registration_time: String,
}

/// Health status report for orchestrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    pub service_id: String,
    pub status: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Minimal stand-in for nestgate-core / nestgate-discovery `SelfKnowledge`.
#[derive(Clone, Debug)]
pub struct SelfKnowledge {
    pub id: String,
    pub name: String,
    pub capabilities: Vec<String>,
    pub version: String,
    pub endpoints: HashMap<String, String>,
}

/// Builder for [`SelfKnowledge`] (tests and docs).
#[derive(Default)]
pub struct SelfKnowledgeBuilder {
    id: Option<String>,
    name: Option<String>,
    capabilities: Vec<String>,
}

impl SelfKnowledge {
    #[must_use]
    pub fn builder() -> SelfKnowledgeBuilder {
        SelfKnowledgeBuilder::default()
    }
}

impl SelfKnowledgeBuilder {
    #[must_use]
    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }
    #[must_use]
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    #[must_use]
    pub fn with_capability(mut self, c: &str) -> Self {
        self.capabilities.push(c.to_string());
        self
    }
    /// # Errors
    ///
    /// Returns `NestGateError` if required builder fields cannot be satisfied (currently always
    /// returns `Ok`).
    pub fn build(self) -> Result<SelfKnowledge> {
        Ok(SelfKnowledge {
            id: self.id.unwrap_or_else(|| "nestgate".to_string()),
            name: self.name.unwrap_or_else(|| "nestgate".to_string()),
            capabilities: self.capabilities,
            version: "1.0.0".to_string(),
            endpoints: HashMap::new(),
        })
    }
}

/// Placeholder until `nestgate-discovery` exposes `DiscoveryMechanism`.
pub trait DiscoveryMechanism: Send + Sync {
    fn mechanism_name(&self) -> &'static str;
}

struct StubDiscovery;

impl DiscoveryMechanism for StubDiscovery {
    fn mechanism_name(&self) -> &'static str {
        "stub"
    }
}

/// Discovered service (shape preserved for callers).
#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub id: String,
    pub name: String,
    pub capabilities: Vec<String>,
    pub endpoint: String,
    pub metadata: HashMap<String, String>,
    pub health_endpoint: Option<String>,
}

pub struct OrchestratorRegistration {
    self_knowledge: SelfKnowledge,
    discovery: Arc<dyn DiscoveryMechanism>,
    orchestrator: Option<ServiceInfo>,
    enabled: bool,
}

impl OrchestratorRegistration {
    /// # Errors
    ///
    /// Returns `NestGateError` if the registration context cannot be constructed (currently always
    /// returns `Ok`).
    pub fn new(self_knowledge: SelfKnowledge) -> Result<Self> {
        let enabled = std::env::var("NESTGATE_DISABLE_ORCHESTRATOR")
            .map(|v| v != "1" && v.to_lowercase() != "true")
            .unwrap_or(true);

        if !enabled {
            info!("🎵 Orchestrator auto-registration disabled by environment");
        }

        Ok(Self {
            self_knowledge,
            discovery: Arc::new(StubDiscovery),
            orchestrator: None,
            enabled,
        })
    }

    pub fn register(&self) {
        if !self.enabled {
            return;
        }
        warn!(
            "orchestrator register: stub (wire nestgate-discovery + nestgate-core); service={}",
            self.self_knowledge.name
        );
    }

    pub const fn report_health(&self) {}

    pub fn start_health_reporting(&self) {
        if !self.enabled {
            return;
        }
        warn!("start_health_reporting: stub until orchestrator wiring");
    }

    #[must_use]
    pub fn discovery(&self) -> &Arc<dyn DiscoveryMechanism> {
        &self.discovery
    }

    #[must_use]
    pub const fn orchestrator(&self) -> Option<&ServiceInfo> {
        self.orchestrator.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_registration_disabled() {
        // SAFETY: single-threaded test — no concurrent env readers.
        nestgate_platform::env_process::set_var("NESTGATE_DISABLE_ORCHESTRATOR", "true");

        let self_knowledge = SelfKnowledge::builder()
            .with_id("test")
            .with_name("nestgate")
            .with_capability("storage")
            .build()
            .unwrap();

        let registration = OrchestratorRegistration::new(self_knowledge).unwrap();
        assert!(!registration.enabled);

        // SAFETY: single-threaded test — no concurrent env readers.
        nestgate_platform::env_process::remove_var("NESTGATE_DISABLE_ORCHESTRATOR");
    }

    #[tokio::test]
    async fn test_registration_data_serialization() {
        let data = ServiceRegistrationData {
            service_id: "test-123".to_string(),
            primal_name: "nestgate".to_string(),
            capabilities: vec!["storage".to_string()],
            version: "1.0.0".to_string(),
            endpoints: std::collections::HashMap::new(),
            registration_time: "2026-01-13T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("test-123"));
        assert!(json.contains("storage"));
    }
}
