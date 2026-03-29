// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![cfg(any(feature = "dev-stubs", test))]

//! Orchestrator auto-registration — stub until nestgate-discovery + nestgate-core wiring.

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

/// Payload describing this primal when registering with an orchestrator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistrationData {
    /// Stable service instance identifier.
    pub service_id: String,
    /// Human-readable primal name (for example `nestgate`).
    pub primal_name: String,
    /// Capability strings advertised to the orchestrator.
    pub capabilities: Vec<String>,
    /// Semantic version of the running service.
    pub version: String,
    /// Reachable endpoints keyed by role or transport name.
    pub endpoints: std::collections::HashMap<String, String>,
    /// RFC3339 timestamp when the registration was produced.
    pub registration_time: String,
}

/// Health snapshot reported upstream to an orchestrator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    /// Service instance this report refers to.
    pub service_id: String,
    /// Coarse status label (for example `healthy`, `degraded`).
    pub status: String,
    /// RFC3339 time the sample was taken.
    pub timestamp: String,
    /// Optional structured detail for dashboards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Minimal stand-in for nestgate-core / nestgate-discovery `SelfKnowledge`.
#[derive(Clone, Debug)]
pub struct SelfKnowledge {
    /// Primal id string used in discovery payloads.
    pub id: String,
    /// Display name for this primal.
    pub name: String,
    /// Advertised capability identifiers.
    pub capabilities: Vec<String>,
    /// Version string surfaced to peers.
    pub version: String,
    /// Arbitrary endpoint map until discovery is fully wired.
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
    /// Starts a fluent builder for [`SelfKnowledge`].
    #[must_use]
    pub fn builder() -> SelfKnowledgeBuilder {
        SelfKnowledgeBuilder::default()
    }
}

impl SelfKnowledgeBuilder {
    /// Sets the primal id.
    #[must_use]
    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }
    /// Sets the primal display name.
    #[must_use]
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    /// Appends a single capability string.
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

/// Placeholder until `nestgate-discovery` exposes a real discovery backend.
pub trait DiscoveryMechanism: Send + Sync {
    /// Short label for this mechanism (for example `"stub"`).
    fn mechanism_name(&self) -> &'static str;
}

struct StubDiscovery;

impl DiscoveryMechanism for StubDiscovery {
    fn mechanism_name(&self) -> &'static str {
        "stub"
    }
}

/// Discovered remote service (wire shape preserved for future discovery integration).
#[derive(Debug, Clone)]
pub struct ServiceInfo {
    /// Remote service id.
    pub id: String,
    /// Remote service name.
    pub name: String,
    /// Capabilities reported by the remote.
    pub capabilities: Vec<String>,
    /// Primary connection endpoint.
    pub endpoint: String,
    /// Opaque key/value metadata from discovery.
    pub metadata: HashMap<String, String>,
    /// Optional dedicated health URL, if advertised.
    pub health_endpoint: Option<String>,
}

/// Stub orchestrator client: logs intent until nestgate-discovery + core wiring lands.
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

    /// No-op registration hook (emits tracing until orchestrator RPC exists).
    pub fn register(&self) {
        if !self.enabled {
            return;
        }
        warn!(
            "orchestrator register: stub (wire nestgate-discovery + nestgate-core); service={}",
            self.self_knowledge.name
        );
    }

    /// Placeholder for periodic health push (currently does nothing).
    pub const fn report_health(&self) {}

    /// Starts background health reporting when orchestrator integration is available.
    pub fn start_health_reporting(&self) {
        if !self.enabled {
            return;
        }
        warn!("start_health_reporting: stub until orchestrator wiring");
    }

    /// Returns the active (stub) discovery mechanism handle.
    #[must_use]
    pub fn discovery(&self) -> &Arc<dyn DiscoveryMechanism> {
        &self.discovery
    }

    /// Last known orchestrator record, if any (always `None` in the stub).
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

    #[test]
    fn orchestrator_enabled_register_and_discovery_stub() {
        nestgate_platform::env_process::remove_var("NESTGATE_DISABLE_ORCHESTRATOR");
        let sk = SelfKnowledge::builder()
            .with_name("nestgate")
            .build()
            .expect("knowledge");
        let reg = OrchestratorRegistration::new(sk).expect("reg");
        assert!(reg.enabled);
        reg.register();
        reg.start_health_reporting();
        assert_eq!(reg.discovery().mechanism_name(), "stub");
        assert!(reg.orchestrator().is_none());
    }

    #[test]
    fn health_report_and_service_info_serde() {
        let hr = HealthReport {
            service_id: "s1".to_string(),
            status: "ok".to_string(),
            timestamp: "2026-01-01T00:00:00Z".to_string(),
            details: Some(serde_json::json!({"k": 1})),
        };
        let j = serde_json::to_string(&hr).unwrap();
        assert!(j.contains("s1"));
        let si = ServiceInfo {
            id: "a".to_string(),
            name: "b".to_string(),
            capabilities: vec!["c".to_string()],
            endpoint: "e".to_string(),
            metadata: std::collections::HashMap::new(),
            health_endpoint: None,
        };
        assert_eq!(si.endpoint, "e");
    }

    #[test]
    fn orchestrator_disabled_accepts_true_and_1() {
        for val in ["1", "true", "TRUE"] {
            temp_env::with_var("NESTGATE_DISABLE_ORCHESTRATOR", Some(val), || {
                let sk = SelfKnowledge::builder().build().expect("knowledge");
                let reg = OrchestratorRegistration::new(sk).expect("reg");
                assert!(!reg.enabled, "{val}");
            });
        }
    }

    #[test]
    fn report_health_stub_does_not_panic() {
        let sk = SelfKnowledge::builder().build().expect("knowledge");
        let reg = OrchestratorRegistration::new(sk).expect("reg");
        reg.report_health();
    }
}
