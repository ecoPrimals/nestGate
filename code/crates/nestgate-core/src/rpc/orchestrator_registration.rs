//! # 🎵 Orchestrator Auto-Registration (Capability-Based)
//!
//! **Automatic Service Registration with ANY Orchestrator Primal**
//!
//! Implements automatic service registration and health reporting
//! using capability-based discovery. Works with ANY primal that provides
//! the "orchestration" capability - not just Songbird!
//!
//! ## Philosophy
//! - **Infant Discovery**: Start with zero knowledge
//! - **Capability-Based**: Find "orchestration", not "songbird"
//! - **Vendor Agnostic**: Works with any orchestrator
//! - **Self-Knowledge**: Register own capabilities only
//! - **Graceful Fallback**: Continues without orchestrator if unavailable
//! - **Modern Async**: Native async/await patterns
//!
//! ## Environment Variables
//! - `NESTGATE_FAMILY_ID` (required): Own family identifier
//! - `NESTGATE_DISABLE_ORCHESTRATOR` (optional): Disable auto-registration
//! - Discovery mechanism uses standard discovery env vars (see `DiscoveryMechanism`)
//!
//! ## Usage
//! ```no_run
//! use nestgate_core::rpc::orchestrator_registration::OrchestratorRegistration;
//! use nestgate_core::self_knowledge::SelfKnowledge;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // 1. Build self-knowledge
//! use nestgate_core::constants::ports;
//! let self_knowledge = SelfKnowledge::builder()
//!     .with_id("nestgate-001")
//!     .with_name("nestgate")
//!     .with_capability("storage")
//!     .with_endpoint("api", ports::get_api_server_addr().parse()?)
//!     .build()?;
//!
//! // 2. Create registration (auto-discovers orchestrator)
//! let registration = OrchestratorRegistration::new(self_knowledge).await?;
//!
//! // 3. Register with discovered orchestrator
//! registration.register().await?;
//!
//! // 4. Start health reporting
//! registration.start_health_reporting();
//! # Ok(())
//! # }
//! ```

use crate::discovery_mechanism::{DiscoveryBuilder, DiscoveryMechanism, ServiceInfo};
use crate::error::{NestGateError, Result};
use crate::self_knowledge::SelfKnowledge;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, warn};

/// Service registration data for orchestrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistrationData {
    /// Service identifier (from SelfKnowledge)
    pub service_id: String,
    /// Primal name (from SelfKnowledge)
    pub primal_name: String,
    /// Service capabilities
    pub capabilities: Vec<String>,
    /// Service version (from SelfKnowledge)
    pub version: String,
    /// Service endpoints
    pub endpoints: std::collections::HashMap<String, String>,
    /// Registration timestamp (ISO 8601)
    pub registration_time: String,
}

/// Health status report for orchestrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    /// Service identifier
    pub service_id: String,
    /// Health status: "healthy", "degraded", "unhealthy"
    pub status: String,
    /// Timestamp (ISO 8601)
    pub timestamp: String,
    /// Optional health details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Orchestrator auto-registration manager (capability-based)
pub struct OrchestratorRegistration {
    /// Self-knowledge
    self_knowledge: SelfKnowledge,
    /// Discovery mechanism
    discovery: Arc<dyn DiscoveryMechanism>,
    /// Discovered orchestrator (if any)
    orchestrator: Option<ServiceInfo>,
    /// Registration enabled flag
    enabled: bool,
}

impl OrchestratorRegistration {
    /// Create new orchestrator registration manager
    ///
    /// # Infant Discovery Principle
    /// - Uses only self-knowledge for registration
    /// - Discovers orchestrator via capability ("orchestration")
    /// - No hardcoded primal names
    /// - Infrastructure agnostic (mDNS, Consul, k8s)
    ///
    /// # Arguments
    /// - `self_knowledge`: This primal's self-knowledge
    ///
    /// # Errors
    /// - Returns error if configuration is invalid
    pub async fn new(self_knowledge: SelfKnowledge) -> Result<Self> {
        // Check if registration is disabled
        let enabled = std::env::var("NESTGATE_DISABLE_ORCHESTRATOR")
            .map(|v| v != "1" && v.to_lowercase() != "true")
            .unwrap_or(true);

        if !enabled {
            info!("🎵 Orchestrator auto-registration disabled by environment");

            let discovery_box = DiscoveryBuilder::new()
                .detect()
                .await
                .map_err(|e| NestGateError::configuration_error("discovery", &e.to_string()))?;
            let discovery: Arc<dyn DiscoveryMechanism> = Arc::from(discovery_box);

            return Ok(Self {
                self_knowledge,
                discovery,
                orchestrator: None,
                enabled: false,
            });
        }

        // Create discovery mechanism (auto-detects infrastructure)
        let discovery_box = DiscoveryBuilder::new()
            .detect()
            .await
            .map_err(|e| NestGateError::configuration_error("discovery", &e.to_string()))?;

        let discovery: Arc<dyn DiscoveryMechanism> = Arc::from(discovery_box);

        info!(
            "🔍 Using discovery mechanism: {}",
            discovery.mechanism_name()
        );

        // Discover orchestrator via capability
        let orchestrator = Self::discover_orchestrator(&discovery).await;

        // Modern pattern: explicit Some/None handling (no unwrap!)
        if let Some(ref orch) = orchestrator {
            info!("✅ Discovered orchestrator: {}", orch.name);
        } else {
            warn!("🎵 No orchestrator discovered - continuing without orchestration");
            info!(
                "   Orchestrators can be discovered via {}",
                discovery.mechanism_name()
            );
        }

        Ok(Self {
            self_knowledge,
            discovery,
            orchestrator,
            enabled,
        })
    }

    /// Discover orchestrator by capability
    ///
    /// # Infant Discovery Principle
    /// - Searches for "orchestration" capability (not "songbird"!)
    /// - Uses auto-detected discovery mechanism
    /// - Returns first available orchestrator
    async fn discover_orchestrator(discovery: &Arc<dyn DiscoveryMechanism>) -> Option<ServiceInfo> {
        debug!("🔍 Searching for orchestration capability...");

        match discovery
            .find_by_capability("orchestration".to_string())
            .await
        {
            Ok(orchestrators) => {
                if orchestrators.is_empty() {
                    debug!("No orchestrators found");
                    None
                } else {
                    let orch = &orchestrators[0];
                    info!("✅ Found orchestrator: {} ({})", orch.name, orch.id);
                    Some(orch.clone())
                }
            }
            Err(e) => {
                warn!("Failed to discover orchestrator: {}", e);
                None
            }
        }
    }

    /// Register with discovered orchestrator
    ///
    /// Announces this service to the discovered orchestrator.
    /// Returns Ok(()) whether registration succeeds or not (graceful fallback).
    ///
    /// # Errors
    /// - Never returns error (logs failures and continues)
    pub async fn register(&self) -> Result<()> {
        if !self.enabled {
            debug!("Orchestrator registration disabled");
            return Ok(());
        }

        // First, announce self via discovery mechanism
        debug!("📢 Announcing self via discovery...");
        if let Err(e) = self.discovery.announce(&self.self_knowledge).await {
            warn!("Failed to announce via discovery: {}", e);
        } else {
            info!(
                "✅ Announced via {} as '{}'",
                self.discovery.mechanism_name(),
                self.self_knowledge.name
            );
        }

        // Then, register with orchestrator if available
        let orchestrator = match &self.orchestrator {
            Some(orch) => orch,
            None => {
                debug!("No orchestrator available - skipping orchestrator registration");
                return Ok(());
            }
        };

        info!("🎵 Registering with orchestrator: {}", orchestrator.name);

        // Build registration data from self-knowledge
        let registration = ServiceRegistrationData {
            service_id: self.self_knowledge.id.as_str().to_string(),
            primal_name: self.self_knowledge.name.clone(),
            capabilities: self.self_knowledge.capabilities.clone(),
            version: self.self_knowledge.version.clone(),
            endpoints: self
                .self_knowledge
                .endpoints
                .iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect(),
            registration_time: chrono::Utc::now().to_rfc3339(),
        };

        // Attempt registration via orchestrator endpoint
        match Self::send_registration(&orchestrator.endpoint, &registration).await {
            Ok(()) => {
                info!("✅ Successfully registered with orchestrator");
                info!("   Service ID: {}", self.self_knowledge.id);
                info!("   Capabilities: {:?}", self.self_knowledge.capabilities);
            }
            Err(e) => {
                warn!("⚠️  Failed to register with orchestrator: {}", e);
                warn!("   Continuing without orchestration (graceful fallback)");
            }
        }

        Ok(())
    }

    /// Send registration data to orchestrator
    async fn send_registration(
        endpoint: &str,
        registration: &ServiceRegistrationData,
    ) -> Result<()> {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
        use tokio::net::UnixStream;

        // ⚠️ DEPRECATED: Direct Unix socket connection
        // TODO: Migrate to Songbird's universal IPC (Phase 3)
        // See: UNIVERSAL_IPC_EVOLUTION_PLAN_JAN_19_2026.md

        // Try Unix socket first, fall back to HTTP
        if endpoint.starts_with('/') || endpoint.starts_with("unix://") {
            let socket_path = endpoint.trim_start_matches("unix://");

            // Connect to orchestrator
            // ⚠️ DEPRECATED: Use songbird::ipc::connect() instead
            let stream = UnixStream::connect(socket_path).await.map_err(|e| {
                NestGateError::configuration_error(
                    "orchestrator_connect",
                    &format!("Failed to connect to orchestrator socket: {}", e),
                )
            })?;

            let (reader, mut writer) = stream.into_split();
            let mut reader = BufReader::new(reader);

            // Build JSON-RPC request
            let request = serde_json::json!({
                "jsonrpc": "2.0",
                "method": "register_service",
                "params": registration,
                "id": 1
            });

            let request_json = serde_json::to_string(&request)
                .map_err(|e| NestGateError::api(format!("Failed to serialize request: {}", e)))?;

            // Send request
            writer
                .write_all(request_json.as_bytes())
                .await
                .map_err(|e| NestGateError::io_error(format!("Failed to send request: {}", e)))?;
            writer
                .write_all(b"\n")
                .await
                .map_err(|e| NestGateError::io_error(format!("Failed to send newline: {}", e)))?;

            // Read response
            let mut response_line = String::new();
            reader
                .read_line(&mut response_line)
                .await
                .map_err(|e| NestGateError::io_error(format!("Failed to read response: {}", e)))?;

            // Parse response
            let response: serde_json::Value = serde_json::from_str(&response_line)
                .map_err(|e| NestGateError::api(format!("Failed to parse response: {}", e)))?;

            // Check for errors
            if let Some(error) = response.get("error") {
                return Err(NestGateError::api(format!(
                    "Orchestrator registration failed: {}",
                    error
                )));
            }

            debug!("Received registration response: {:?}", response);
            Ok(())
        } else {
            // HTTP endpoint
            Err(NestGateError::not_implemented(
                "HTTP orchestrator registration not yet implemented",
            ))
        }
    }

    /// Send periodic health report to orchestrator
    ///
    /// Should be called periodically (e.g., every 30 seconds) to maintain
    /// service registration and report health status.
    ///
    /// # Errors
    /// - Never returns error (logs failures and continues)
    pub async fn report_health(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let orchestrator = match &self.orchestrator {
            Some(orch) => orch,
            None => return Ok(()),
        };

        let health = HealthReport {
            service_id: self.self_knowledge.id.as_str().to_string(),
            status: "healthy".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            details: None,
        };

        match Self::send_health_report(&orchestrator.endpoint, &health).await {
            Ok(()) => {
                debug!("Health report sent to orchestrator");
            }
            Err(e) => {
                warn!("Failed to send health report: {}", e);
            }
        }

        Ok(())
    }

    /// Send health report to orchestrator
    async fn send_health_report(endpoint: &str, health: &HealthReport) -> Result<()> {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
        use tokio::net::UnixStream;

        if endpoint.starts_with('/') || endpoint.starts_with("unix://") {
            let socket_path = endpoint.trim_start_matches("unix://");

            let stream = UnixStream::connect(socket_path).await.map_err(|e| {
                NestGateError::configuration_error(
                    "orchestrator_connect",
                    &format!("Failed to connect to orchestrator: {}", e),
                )
            })?;

            let (reader, mut writer) = stream.into_split();
            let mut reader = BufReader::new(reader);

            let request = serde_json::json!({
                "jsonrpc": "2.0",
                "method": "report_health",
                "params": health,
                "id": 1
            });

            let request_json = serde_json::to_string(&request)
                .map_err(|e| NestGateError::api(format!("Failed to serialize request: {}", e)))?;

            writer
                .write_all(request_json.as_bytes())
                .await
                .map_err(|e| NestGateError::io_error(format!("Failed to send request: {}", e)))?;
            writer
                .write_all(b"\n")
                .await
                .map_err(|e| NestGateError::io_error(format!("Failed to send newline: {}", e)))?;

            let mut response_line = String::new();
            reader
                .read_line(&mut response_line)
                .await
                .map_err(|e| NestGateError::io_error(format!("Failed to read response: {}", e)))?;

            Ok(())
        } else {
            Err(NestGateError::not_implemented(
                "HTTP health reporting not yet implemented",
            ))
        }
    }

    /// Start periodic health reporting
    ///
    /// Spawns a background task that sends health reports every 30 seconds.
    /// The task runs indefinitely until the program exits.
    pub fn start_health_reporting(&self) {
        if !self.enabled || self.orchestrator.is_none() {
            return;
        }

        let self_knowledge = self.self_knowledge.clone();
        let discovery = Arc::clone(&self.discovery);
        let orchestrator = self.orchestrator.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;

                // Update discovery health check
                if let Err(e) = discovery.announce(&self_knowledge).await {
                    error!("Failed to update discovery announcement: {}", e);
                }

                // Send health to orchestrator
                if let Some(ref orch) = orchestrator {
                    let health = HealthReport {
                        service_id: self_knowledge.id.as_str().to_string(),
                        status: "healthy".to_string(),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                        details: None,
                    };

                    if let Err(e) = Self::send_health_report(&orch.endpoint, &health).await {
                        warn!("Health reporting error: {}", e);
                    }
                }
            }
        });

        info!("🎵 Started periodic health reporting (30s interval)");
    }

    /// Get discovery mechanism
    pub fn discovery(&self) -> &Arc<dyn DiscoveryMechanism> {
        &self.discovery
    }

    /// Get discovered orchestrator info
    pub fn orchestrator(&self) -> Option<&ServiceInfo> {
        self.orchestrator.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery_mechanism::testing::{MockDiscovery, TestServiceBuilder};

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

    #[tokio::test]
    async fn test_discovers_orchestrator_by_capability() {
        // Create mock discovery with orchestrator
        let mut mock = MockDiscovery::new();

        let orchestrator = TestServiceBuilder::new("orch-1")
            .name("TestOrchestrator")
            .capability("orchestration")
            .endpoint("/tmp/orch.sock")
            .build();

        mock.add_service(orchestrator);
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Test discovery
        let discovery: Arc<dyn DiscoveryMechanism> = Arc::new(mock);
        let found = OrchestratorRegistration::discover_orchestrator(&discovery).await;

        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.name, "TestOrchestrator");
        assert!(found.capabilities.contains(&"orchestration".to_string()));
    }
}
