//! Songbird Primal - Universal Cryptographic Spore Integration
//!
//! This example shows how to integrate the Universal Cryptographic Spore system
//! into Songbird's orchestration capabilities while maintaining pure orchestration role.

use crate::universal_spore::{
    UniversalCryptographicSpore, OperationRequest, UserContext, AuthorizationDecision
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use std::collections::HashMap;
use std::time::SystemTime;

/// Songbird with integrated cryptographic spore
pub struct SongbirdOrchestrator {
    /// Orchestration configuration
    pub config: SongbirdConfig,
    
    /// Universal capability routing
    pub capability_router: CapabilityRouter,
    
    /// Universal cryptographic spore for autonomous security
    crypto_spore: Arc<RwLock<UniversalCryptographicSpore>>,
}

impl SongbirdOrchestrator {
    /// Create new Songbird instance with spore integration
    pub async fn new(config: SongbirdConfig) -> Result<Self, SongbirdError> {
        // Initialize capability router
        let capability_router = CapabilityRouter::new(&config).await?;
        
        // Create spore for Songbird
        let mut spore = UniversalCryptographicSpore::new_for_primal("songbird")
            .map_err(|e| SongbirdError::SporeError(e.to_string()))?;
        
        // Initialize BearDog integration if available
        if let Some(beardog_endpoint) = &config.beardog_endpoint {
            spore.initialize_with_beardog(Some(beardog_endpoint.clone())).await
                .map_err(|e| SongbirdError::SporeError(e.to_string()))?;
            info!("🧬 BearDog integration initialized for Songbird");
        } else {
            info!("🧬 Songbird spore operating autonomously");
        }
        
        Ok(Self {
            config,
            capability_router,
            crypto_spore: Arc::new(RwLock::new(spore)),
        })
    }
    
    /// Route capability request with spore authorization
    pub async fn route_capability_request(
        &self,
        capability: &str,
        operation: &str,
        payload: serde_json::Value,
        user_context: UserContext,
    ) -> Result<serde_json::Value, SongbirdError> {
        // Create operation request for spore authorization
        let operation_request = OperationRequest {
            operation_type: format!("capability_request_{}", capability),
            resource_path: format!("/capabilities/{}/{}", capability, operation),
            user_context,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("capability".to_string(), capability.to_string());
                meta.insert("operation".to_string(), operation.to_string());
                meta
            },
            timestamp: SystemTime::now(),
        };

        // Ask spore for authorization
        let spore = self.crypto_spore.read().await;
        let decision = spore.authorize_operation(&operation_request).await
            .map_err(|e| SongbirdError::SporeError(e.to_string()))?;

        match decision {
            AuthorizationDecision::Allow { enhanced_by_beardog, .. } => {
                info!("✅ Capability request authorized: {} -> {} (BearDog: {})", 
                      capability, operation, enhanced_by_beardog);
                
                // Route to appropriate capability provider
                self.capability_router.route_request(capability, operation, payload).await
            },
            
            AuthorizationDecision::RequireLicense { terms, contact, organization_profile } => {
                let message = format!(
                    "Corporate license required for capability '{}' usage by '{}'. \
                     Contact: {} Base rate: ${}/month",
                    capability, organization_profile.organization_name, 
                    contact, terms.base_monthly_rate
                );
                warn!("🏢 {}", message);
                Err(SongbirdError::LicenseRequired { message })
            },
            
            AuthorizationDecision::Deny { reason, remediation, .. } => {
                warn!("❌ Capability request denied: {} -> {} ({})", capability, operation, reason);
                Err(SongbirdError::AccessDenied { reason })
            }
        }
    }
    
    /// Orchestrate service deployment with spore authorization
    pub async fn orchestrate_service_deployment(
        &self,
        service_spec: &ServiceSpec,
        user_context: UserContext,
    ) -> Result<DeploymentResult, SongbirdError> {
        // Create operation request
        let operation_request = OperationRequest {
            operation_type: "service_deployment".to_string(),
            resource_path: format!("/services/{}", service_spec.name),
            user_context,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("service_name".to_string(), service_spec.name.clone());
                meta.insert("service_type".to_string(), service_spec.service_type.clone());
                meta.insert("resource_requirements".to_string(), 
                           format!("cpu:{}, memory:{}", service_spec.cpu_limit, service_spec.memory_limit));
                meta
            },
            timestamp: SystemTime::now(),
        };

        // Authorize deployment
        let spore = self.crypto_spore.read().await;
        let decision = spore.authorize_operation(&operation_request).await
            .map_err(|e| SongbirdError::SporeError(e.to_string()))?;

        match decision {
            AuthorizationDecision::Allow { .. } => {
                info!("✅ Service deployment authorized: {}", service_spec.name);
                
                // Perform orchestration (pure orchestration - no direct implementation)
                self.orchestrate_deployment_internal(service_spec).await
            },
            
            AuthorizationDecision::RequireLicense { terms, contact, organization_profile } => {
                let message = format!(
                    "Corporate license required for service deployment by '{}'. \
                     Service: {} Contact: {}",
                    organization_profile.organization_name, service_spec.name, contact
                );
                Err(SongbirdError::LicenseRequired { message })
            },
            
            AuthorizationDecision::Deny { reason, .. } => {
                Err(SongbirdError::AccessDenied { reason })
            }
        }
    }
    
    /// Create user context from orchestration request
    pub fn create_user_context_from_request(
        &self,
        request: &OrchestrationRequest,
    ) -> UserContext {
        let mut environment_info = HashMap::new();
        environment_info.insert("primal".to_string(), "songbird".to_string());
        environment_info.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
        environment_info.insert("orchestration_type".to_string(), request.orchestration_type.clone());
        
        // Detect corporate patterns in orchestration requests
        if request.automated_deployment || request.batch_size > 10 {
            environment_info.insert("automation_indicator".to_string(), "true".to_string());
        }
        
        if let Some(org_context) = &request.organization_context {
            environment_info.insert("organization".to_string(), org_context.clone());
            environment_info.insert("user_type".to_string(), "corporate".to_string());
        } else {
            environment_info.insert("user_type".to_string(), "individual".to_string());
        }
        
        UserContext {
            user_id: request.user_id.clone(),
            session_id: request.session_id.clone(),
            ip_address: request.source_ip.clone(),
            user_agent: request.user_agent.clone(),
            environment_info,
        }
    }
    
    /// Internal orchestration logic (pure orchestration - delegates to other primals)
    async fn orchestrate_deployment_internal(
        &self,
        service_spec: &ServiceSpec,
    ) -> Result<DeploymentResult, SongbirdError> {
        info!("🎼 Orchestrating deployment: {}", service_spec.name);
        
        // Pure orchestration - route to appropriate primals based on capabilities
        let mut deployment_steps = Vec::new();
        
        // Route to ToadStool for compute resources
        if service_spec.requires_compute {
            let compute_result = self.capability_router
                .route_request("compute", "allocate_resources", 
                              serde_json::json!({
                                  "cpu": service_spec.cpu_limit,
                                  "memory": service_spec.memory_limit
                              }))
                .await?;
            deployment_steps.push(DeploymentStep::ComputeAllocated(compute_result));
        }
        
        // Route to NestGate for storage if needed
        if service_spec.requires_storage {
            let storage_result = self.capability_router
                .route_request("storage", "provision_storage",
                              serde_json::json!({
                                  "size": service_spec.storage_size,
                                  "type": service_spec.storage_type
                              }))
                .await?;
            deployment_steps.push(DeploymentStep::StorageProvisioned(storage_result));
        }
        
        // Route to BearDog for security setup if needed
        if service_spec.requires_security {
            let security_result = self.capability_router
                .route_request("security", "setup_service_security",
                              serde_json::json!({
                                  "service_name": service_spec.name,
                                  "security_level": service_spec.security_level
                              }))
                .await?;
            deployment_steps.push(DeploymentStep::SecurityConfigured(security_result));
        }
        
        Ok(DeploymentResult {
            service_name: service_spec.name.clone(),
            deployment_id: uuid::Uuid::new_v4().to_string(),
            steps: deployment_steps,
            status: DeploymentStatus::Completed,
        })
    }
    
    /// Get spore status for Songbird monitoring
    pub async fn get_spore_status(&self) -> SporeStatus {
        let spore = self.crypto_spore.read().await;
        
        SporeStatus {
            spore_id: spore.spore_id.clone(),
            generation: spore.generation,
            beardog_integrated: spore.beardog_integration.is_some(),
            operations_count: spore.usage_stats.operations_count,
            last_evolution: spore.last_evolution,
            primal_name: "songbird".to_string(),
        }
    }
}

// Supporting types for Songbird integration

#[derive(Debug, Clone)]
pub struct SongbirdConfig {
    pub beardog_endpoint: Option<String>,
    pub capability_providers: HashMap<String, String>,
    pub orchestration_settings: OrchestrationSettings,
}

#[derive(Debug, Clone)]
pub struct OrchestrationSettings {
    pub max_concurrent_deployments: usize,
    pub deployment_timeout: std::time::Duration,
    pub health_check_interval: std::time::Duration,
}

#[derive(Debug, Clone)]
pub struct ServiceSpec {
    pub name: String,
    pub service_type: String,
    pub cpu_limit: f64,
    pub memory_limit: u64,
    pub storage_size: Option<u64>,
    pub storage_type: Option<String>,
    pub security_level: String,
    pub requires_compute: bool,
    pub requires_storage: bool,
    pub requires_security: bool,
}

#[derive(Debug, Clone)]
pub struct OrchestrationRequest {
    pub user_id: Option<String>,
    pub session_id: String,
    pub source_ip: String,
    pub user_agent: Option<String>,
    pub orchestration_type: String,
    pub automated_deployment: bool,
    pub batch_size: usize,
    pub organization_context: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DeploymentResult {
    pub service_name: String,
    pub deployment_id: String,
    pub steps: Vec<DeploymentStep>,
    pub status: DeploymentStatus,
}

#[derive(Debug, Clone)]
pub enum DeploymentStep {
    ComputeAllocated(serde_json::Value),
    StorageProvisioned(serde_json::Value),
    SecurityConfigured(serde_json::Value),
}

#[derive(Debug, Clone)]
pub enum DeploymentStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct SporeStatus {
    pub spore_id: String,
    pub generation: u32,
    pub beardog_integrated: bool,
    pub operations_count: u64,
    pub last_evolution: SystemTime,
    pub primal_name: String,
}

// Capability router (existing Songbird functionality)
pub struct CapabilityRouter {
    providers: HashMap<String, String>,
}

impl CapabilityRouter {
    pub async fn new(_config: &SongbirdConfig) -> Result<Self, SongbirdError> {
        Ok(Self {
            providers: HashMap::new(),
        })
    }
    
    pub async fn route_request(
        &self,
        _capability: &str,
        _operation: &str,
        _payload: serde_json::Value,
    ) -> Result<serde_json::Value, SongbirdError> {
        // Placeholder for actual routing logic
        Ok(serde_json::json!({"status": "success"}))
    }
}

// Error types
#[derive(Debug, thiserror::Error)]
pub enum SongbirdError {
    #[error("Access Denied: {reason}")]
    AccessDenied { reason: String },

    #[error("License Required: {message}")]
    LicenseRequired { message: String },

    #[error("Spore Error: {0}")]
    SporeError(String),

    #[error("Orchestration Error: {0}")]
    OrchestrationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_songbird_spore_integration() {
        let config = SongbirdConfig {
            beardog_endpoint: None,
            capability_providers: HashMap::new(),
            orchestration_settings: OrchestrationSettings {
                max_concurrent_deployments: 10,
                deployment_timeout: std::time::Duration::from_secs(300),
                health_check_interval: std::time::Duration::from_secs(30),
            },
        };

        let songbird = SongbirdOrchestrator::new(config).await.unwrap();

        // Test individual user capability request
        let individual_context = UserContext {
            user_id: Some("alice".to_string()),
            session_id: "session_123".to_string(),
            ip_address: "192.168.1.100".to_string(),
            user_agent: Some("Individual Developer".to_string()),
            environment_info: {
                let mut env = HashMap::new();
                env.insert("user_type".to_string(), "individual".to_string());
                env
            },
        };

        let result = songbird.route_capability_request(
            "compute",
            "get_status",
            serde_json::json!({}),
            individual_context,
        ).await;

        assert!(result.is_ok(), "Individual users should get capability access");

        // Verify spore status
        let status = songbird.get_spore_status().await;
        assert_eq!(status.primal_name, "songbird");
        assert_eq!(status.generation, 0);
    }
} 