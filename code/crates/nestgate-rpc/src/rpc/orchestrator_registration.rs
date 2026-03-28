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
    pub fn builder() -> SelfKnowledgeBuilder {
        SelfKnowledgeBuilder::default()
    }
}

impl SelfKnowledgeBuilder {
    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    pub fn with_capability(mut self, c: &str) -> Self {
        self.capabilities.push(c.to_string());
        self
    }
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
    pub async fn new(self_knowledge: SelfKnowledge) -> Result<Self> {
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

    pub async fn register(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        warn!(
            "orchestrator register: stub (wire nestgate-discovery + nestgate-core); service={}",
            self.self_knowledge.name
        );
        Ok(())
    }

    pub async fn report_health(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        Ok(())
    }

    pub fn start_health_reporting(&self) {
        if !self.enabled {
            return;
        }
        warn!("start_health_reporting: stub until orchestrator wiring");
    }

    pub fn discovery(&self) -> &Arc<dyn DiscoveryMechanism> {
        &self.discovery
    }

    pub fn orchestrator(&self) -> Option<&ServiceInfo> {
        self.orchestrator.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_registration_disabled() {
        std::env::set_var("NESTGATE_DISABLE_ORCHESTRATOR", "true");

        let self_knowledge = SelfKnowledge::builder()
            .with_id("test")
            .with_name("nestgate")
            .with_capability("storage")
            .build()
            .unwrap();

        let registration = OrchestratorRegistration::new(self_knowledge).await.unwrap();
        assert!(!registration.enabled);

        std::env::remove_var("NESTGATE_DISABLE_ORCHESTRATOR");
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
