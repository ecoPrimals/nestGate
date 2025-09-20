/// Service Configuration Domain
///
/// Replaces: ServiceConfig, DiscoveryConfig, AutomationConfig, LifecycleConfig,
/// EcosystemConfig, and 10+ other service config structures
use super::CanonicalDomainConfig;
use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
/// **CANONICAL SERVICE CONFIGURATION**
/// Replaces: ServiceConfig, DiscoveryConfig, AutomationConfig, LifecycleConfig,
/// EcosystemConfig, and 10+ other service config structures
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalServiceConfig {
    /// Service identification
    pub identity: ServiceIdentity,
    /// Service discovery settings
    pub discovery: ServiceDiscovery,
    /// Lifecycle management settings
    pub lifecycle: ServiceLifecycle,
    /// Automation settings
    pub automation: ServiceAutomation,
    /// Health monitoring settings
    pub health: ServiceHealth,
    /// Ecosystem integration settings
    pub ecosystem: ServiceEcosystem,
    /// Environment-specific overrides
    pub environment_overrides: HashMap<String, serde_json::Value>,
}
impl CanonicalDomainConfig for CanonicalServiceConfig {
    fn domain() -> &'static str {
        "service"
    }

    fn validate(&self) -> Result<()> {
        if self.identity.name.is_empty() {
            return Err(NestGateError::config_error(
                "identity.name",
                "cannot be empty",
            ));
        }
        Ok(())
    }

    fn merge(mut self, other: Self) -> Self {
        self.environment_overrides
            .extend(other.environment_overrides);
        self
    }

    fn from_environment() -> Result<Self> {
        Ok(Self::default())
    }

    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "identity": {"type": "object", "description": "Service identification"},
                "discovery": {"type": "object", "description": "Service discovery settings"}
            }
        })
    }
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceIdentity {
    pub name: String,
    pub version: String,
    pub description: String,
    pub tags: Vec<String>,
    pub dependencies: Vec<ServiceDependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscovery {
    pub enabled: bool,
    pub registration_endpoint: String,
    pub heartbeat_interval: Duration,
    pub service_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceLifecycle {
    pub startup_timeout: Duration,
    pub shutdown_timeout: Duration,
    pub restart_policy: String,
    pub graceful_shutdown: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceAutomation {
    pub enable_auto_scaling: bool,
    pub scaling_policies: Vec<ScalingPolicy>,
    pub automation_rules: Vec<AutomationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub health_check_endpoint: String,
    pub health_check_interval: Duration,
    pub health_check_timeout: Duration,
    pub unhealthy_threshold: u32,
    pub healthy_threshold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEcosystem {
    pub ecosystem_participation: bool,
    pub primal_capabilities: Vec<String>,
    pub ecosystem_endpoints: Vec<String>,
    pub federation_settings: FederationSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDependency {
    pub name: String,
    pub version_requirement: String,
    pub dependency_type: DependencyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingPolicy {
    pub metric: String,
    pub threshold: f64,
    pub scale_up_action: ScalingAction,
    pub scale_down_action: ScalingAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingAction {
    pub action_type: ScalingActionType,
    pub amount: u32,
    pub cooldown: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub rule_name: String,
    pub trigger: AutomationTrigger,
    pub actions: Vec<AutomationAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FederationSettings {
    pub enable_federation: bool,
    pub trust_domains: Vec<String>,
    pub federation_protocols: Vec<String>,
    pub cross_ecosystem_auth: bool,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Required,
    Optional,
    Weak,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalingActionType {
    AddInstances,
    RemoveInstances,
    IncreaseResources,
    DecreaseResources,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationTrigger {
    Schedule,
    Event,
    Metric,
    Alert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationAction {
    RestartService,
    ScaleService,
    NotifyOperators,
    RunScript,
}

// Default implementations
impl Default for ServiceIdentity {
    fn default() -> Self {
        Self {
            name: "nestgate-service".to_string(),
            version: "1.0.0".to_string(),
            description: "NestGate Universal Storage Service".to_string(),
            tags: vec!["storage".to_string(), "universal".to_string()],
            dependencies: Vec::new(),
        }
    }
}

impl Default for ServiceDiscovery {
    fn default() -> Self {
        Self {
            enabled: true,
            registration_endpoint: "http://localhost:8500".to_string(),
            heartbeat_interval: Duration::from_secs(30),
            service_tags: Vec::new(),
        }
    }
}

impl Default for ServiceLifecycle {
    fn default() -> Self {
        Self {
            startup_timeout: Duration::from_secs(60),
            shutdown_timeout: Duration::from_secs(30),
            restart_policy: "always".to_string(),
            graceful_shutdown: true,
        }
    }
}

impl Default for ServiceHealth {
    fn default() -> Self {
        Self {
            health_check_endpoint: "/health".to_string(),
            health_check_interval: Duration::from_secs(30),
            health_check_timeout: Duration::from_secs(5),
            unhealthy_threshold: 3,
            healthy_threshold: 2,
        }
    }
}

impl Default for ServiceEcosystem {
    fn default() -> Self {
        Self {
            ecosystem_participation: true,
            primal_capabilities: vec!["storage".to_string()],
            ecosystem_endpoints: Vec::new(),
            federation_settings: FederationSettings::default(),
        }
    }
}
