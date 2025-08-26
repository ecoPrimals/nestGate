//
// **CANONICAL MODERNIZATION**: Replaces hardcoded BiomeOS integration with universal
// capability-based ecosystem discovery and integration.
//
// **ELIMINATES**:
// - Hardcoded biomeOS endpoints and service calls
// - System-specific integration code
// - Violation of Universal Adapter Architecture principles
//
// **PROVIDES**:
// - Capability-based ecosystem discovery
// - Universal adapter compliance
// - Support for any ecosystem (BiomeOS, k8s, Docker, etc.)
// - Graceful fallbacks and sovereignty compliance

use nestgate_core::{
    universal_adapter::{UniversalAdapter, CanonicalCapabilityRequest},
    canonical_modernization::{CanonicalEcosystemConfig},
    error::{Result, NestGateError},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

/// Universal ecosystem integration service
/// Replaces hardcoded BiomeOS integration with capability-based discovery
pub struct UniversalEcosystemIntegration {
    /// Universal adapter for capability-based communication
    adapter: Arc<UniversalAdapter>,
    /// Ecosystem configuration
    config: CanonicalEcosystemConfig,
    /// Discovered ecosystem information
    discovered_ecosystems: tokio::sync::RwLock<Vec<EcosystemInfo>>,
}

impl UniversalEcosystemIntegration {
    /// Create new universal ecosystem integration
    pub fn new(adapter: Arc<UniversalAdapter>, config: CanonicalEcosystemConfig) -> Self {
        Self {
            adapter,
            config,
            discovered_ecosystems: tokio::sync::RwLock::new(Vec::new()),
        }
    }

    /// Discover any ecosystem (BiomeOS, k8s, docker, etc.) using capability-based discovery
    pub async fn discover_ecosystem(&self) -> Result<Vec<EcosystemInfo>> {
        // Use universal capability discovery instead of hardcoded endpoints
        let discovery_capability = "ecosystem_discovery_v1".to_string();
        
        let mut request = CanonicalCapabilityRequest {
            capability: discovery_capability,
            parameters: HashMap::new(),
            metadata: HashMap::new(),
            request_id: "ecosystem_discovery".to_string(),
            target_service: None,
            timeout: None,
        };
        request.parameters.insert("discovery_timeout".to_string(), serde_json::json!(30));
        request.parameters.insert("include_health_status".to_string(), serde_json::json!(true));

        let response = self.adapter.execute_capability(request).await?;
        
        // Parse discovered ecosystems
        let ecosystems: Vec<EcosystemInfo> = serde_json::from_value(response.data.unwrap_or_default())
            .map_err(|e| NestGateError::Configuration { 
                message: format!("Failed to parse ecosystem discovery response: {}", e),
                config_source: nestgate_core::error::UnifiedConfigSource::File("ecosystem_integration".to_string()),
                field: Some("discovery_response".to_string()),
                suggested_fix: Some("Check ecosystem response format".to_string()),
            })?;

        // Cache discovered ecosystems
        let mut cache = self.discovered_ecosystems.write().await;
        *cache = ecosystems.clone();

        Ok(ecosystems)
    }

    /// Register NestGate with discovered ecosystem using universal patterns
    pub async fn register_with_ecosystem(&self, ecosystem_id: &str) -> Result<RegistrationResult> {
        let registration_capability = "ecosystem_registration_v1".to_string();
        
        let service_info = ServiceRegistrationInfo {
            service_name: "nestgate".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: self.get_nestgate_capabilities().await,
            endpoints: self.get_nestgate_endpoints().await,
            health_check_endpoint: "/health".to_string(),
            metadata: self.get_service_metadata().await,
        };

        let mut request = CanonicalCapabilityRequest {
            capability: registration_capability,
            parameters: HashMap::new(),
            metadata: HashMap::new(),
            request_id: "ecosystem_registration".to_string(),
            target_service: None,
            timeout: None,
        };
        request.parameters.insert("ecosystem_id".to_string(), serde_json::json!(ecosystem_id));
        request.parameters.insert("service_info".to_string(), serde_json::to_value(service_info)?);

        let response = self.adapter.execute_capability(request).await?;
        
        serde_json::from_value(response.data.unwrap_or_default())
            .map_err(|e| NestGateError::Configuration {
                message: format!("Failed to parse registration response: {}", e),
                config_source: nestgate_core::error::UnifiedConfigSource::File("ecosystem_integration".to_string()),
                field: Some("registration_response".to_string()),
                suggested_fix: Some("Check registration response format".to_string()),
            })
    }

    /// Handle ecosystem events using universal event handling
    pub async fn handle_ecosystem_event(&self, event: UniversalEcosystemEvent) -> Result<EventResponse> {
        let event_capability = "ecosystem_event_handling_v1".to_string();
        
        let mut request = CanonicalCapabilityRequest {
            capability: event_capability,
            parameters: HashMap::new(),
            metadata: HashMap::new(),
            request_id: "ecosystem_event_handling".to_string(),
            target_service: None,
            timeout: None,
        };
        request.parameters.insert("event_type".to_string(), serde_json::json!(event.event_type));
        request.parameters.insert("event_data".to_string(), serde_json::to_value(event.data)?);

        let response = self.adapter.execute_capability(request).await?;
        
        serde_json::from_value(response.data.unwrap_or_default())
            .map_err(|e| NestGateError::Configuration {
                message: format!("Failed to parse event response: {}", e),
                config_source: nestgate_core::error::UnifiedConfigSource::File("ecosystem_integration".to_string()),
                field: Some("event_response".to_string()),
                suggested_fix: Some("Check event response format".to_string()),
            })
    }

    /// Get ecosystem compatibility status
    pub async fn get_compatibility_status(&self) -> Result<CompatibilityStatus> {
        let status_capability = "ecosystem_status_v1".to_string();
        
        let request = CanonicalCapabilityRequest {
            capability: status_capability,
            parameters: HashMap::new(),
            metadata: HashMap::new(),
            request_id: "ecosystem_status".to_string(),
            target_service: None,
            timeout: None,
        };
        let response = self.adapter.execute_capability(request).await?;
        
        serde_json::from_value(response.data.unwrap_or_default())
            .map_err(|e| NestGateError::Configuration {
                message: format!("Failed to parse status response: {}", e),
                config_source: nestgate_core::error::UnifiedConfigSource::File("ecosystem_integration".to_string()),
                field: Some("status_response".to_string()),
                suggested_fix: Some("Check status response format".to_string()),
            })
    }

    /// Get NestGate capabilities for registration
    async fn get_nestgate_capabilities(&self) -> Vec<String> {
        vec![
            "storage.zfs.management".to_string(),
            "storage.nas.protocols".to_string(),
            "api.rest.management".to_string(),
            "monitoring.metrics.collection".to_string(),
            "security.authentication.oauth".to_string(),
        ]
    }

    /// Get NestGate endpoints for registration
    async fn get_nestgate_endpoints(&self) -> Vec<ServiceEndpoint> {
        vec![
            ServiceEndpoint {
                name: "api".to_string(),
                url: format!("http://{}:{}", 
                    "0.0.0.0".to_string(),
                    8080
                ),
                protocol: "http".to_string(),
                health_check: Some("/health".to_string()),
            },
            ServiceEndpoint {
                name: "metrics".to_string(),
                url: format!("http://{}:{}/metrics", 
                    "0.0.0.0".to_string(),
                    9090
                ),
                protocol: "http".to_string(),
                health_check: None,
            },
        ]
    }

    /// Get service metadata for registration
    async fn get_service_metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("service_type".to_string(), "storage_orchestration".to_string());
        metadata.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
        metadata.insert("architecture".to_string(), "universal_adapter".to_string());
        metadata.insert("capabilities".to_string(), "zfs,nas,api,monitoring".to_string());
        metadata
    }
}

/// Universal capability ID for ecosystem operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityId {
    pub domain: String,
    pub capability: String,
    pub version: String,
}

impl CapabilityId {
    pub fn new(domain: &str, capability: &str, version: &str) -> Self {
        Self {
            domain: domain.to_string(),
            capability: capability.to_string(),
            version: version.to_string(),
        }
    }
}



/// Discovered ecosystem information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemInfo {
    pub ecosystem_id: String,
    pub ecosystem_type: EcosystemType,
    pub version: String,
    pub capabilities: Vec<String>,
    pub endpoints: Vec<String>,
    pub health_status: EcosystemHealthStatus,
    pub discovery_method: String,
    pub discovered_at: SystemTime,
}

/// Types of ecosystems that can be discovered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EcosystemType {
    BiomeOS,
    Kubernetes,
    Docker,
    Nomad,
    Standalone,
    Unknown(String),
}

/// Ecosystem health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EcosystemHealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Service registration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistrationInfo {
    pub service_name: String,
    pub service_version: String,
    pub capabilities: Vec<String>,
    pub endpoints: Vec<ServiceEndpoint>,
    pub health_check_endpoint: String,
    pub metadata: HashMap<String, String>,
}

/// Service endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub name: String,
    pub url: String,
    pub protocol: String,
    pub health_check: Option<String>,
}

/// Registration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationResult {
    pub registration_id: String,
    pub status: RegistrationStatus,
    pub assigned_endpoints: Vec<String>,
    pub ttl: Duration,
    pub renewal_token: Option<String>,
}

/// Registration status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistrationStatus {
    Success,
    Partial,
    Failed,
    Pending,
}

/// Universal ecosystem event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalEcosystemEvent {
    pub event_id: String,
    pub event_type: String,
    pub source_ecosystem: String,
    pub timestamp: SystemTime,
    pub data: serde_json::Value,
}

/// Event handling response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventResponse {
    pub handled: bool,
    pub response_data: Option<serde_json::Value>,
    pub next_actions: Vec<String>,
}

/// Ecosystem compatibility status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityStatus {
    pub compatible: bool,
    pub supported_features: Vec<String>,
    pub unsupported_features: Vec<String>,
    pub recommendations: Vec<String>,
    pub version_compatibility: HashMap<String, String>,
} 