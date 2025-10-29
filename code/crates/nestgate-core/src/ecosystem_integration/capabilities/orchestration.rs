use crate::universal_adapter::{PrimalAgnosticAdapter, CapabilityCategory, CapabilityRequest};
/// Orchestration Capabilities (Orchestration Primal Integration)
///
/// Defines capability interfaces for service coordination, workflow management,
/// and event routing through the Orchestration orchestration primal.
use super::{CapabilityRequest, CapabilityResponse, UniversalCapability};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

// Type aliases to reduce complexity warnings
type OrchestrationResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
type OrchestrationFuture<T> = Pin<Box<dyn Future<Output = OrchestrationResult<T>> + Send>>;

/// Service coordination request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCoordinationRequest {
    pub services: Vec<String>,
    pub coordination_type: String,
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
}

/// Service coordination response  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCoordinationResponse {
    pub success: bool,
    pub results: std::collections::HashMap<String, serde_json::Value>,
}

/// Workflow execution request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRequest {
    pub workflow_id: String,
    pub steps: Vec<WorkflowStep>,
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
}

/// Workflow execution response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResponse {
    pub workflow_id: String,
    pub status: String,
    pub results: std::collections::HashMap<String, serde_json::Value>,
}

/// Individual workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub step_id: String,
    pub action: String,
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
}

/// Orchestration capability trait - simplified return types
pub trait OrchestrationCapability: Send + Sync {
    /// Coordinate multiple services - simplified return type
    fn coordinate_services(
        &self,
        request: ServiceCoordinationRequest,
    ) -> OrchestrationFuture<ServiceCoordinationResponse>;
    
    /// Execute workflow - simplified return type
    fn execute_workflow(
        &self,
        request: WorkflowRequest,
    ) -> OrchestrationFuture<WorkflowResponse>;
}

/// Mock implementation for testing
#[cfg(any(test, feature = "mock-services"))]
pub struct MockOrchestrationCapability {
    enabled: bool,
}
impl MockOrchestrationCapability {
    pub fn new() -> Self {
        Self { enabled: true }
    }
}

impl Default for MockOrchestrationCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl UniversalCapability for MockOrchestrationCapability {
    async fn execute(
        &self,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse, Box<dyn std::error::Error + Send + Sync>> {
        if !self.enabled {
            return Ok(CapabilityResponse::error(
                "Mock orchestration capability is disabled",
            ));
        }

        match request.capability_id.as_str() {
            "orchestration.service_coordination" => {
                let response_data = serde_json::to_value(ServiceCoordinationResponse {
                    success: true,
                    results: HashMap::from([
                        ("coordination_id".to_string(), serde_json::Value::String("mock-coord-123".to_string())),
                        ("status".to_string(), serde_json::Value::String("active".to_string())),
                        ("coordinated_services".to_string(), serde_json::json!([
                            "service1".to_string(),
                            "service2".to_string()
                        ])),
                    ]),
                })?;
                Ok(CapabilityResponse::success(response_data))
            }
            "orchestration.workflow_management" => {
                let response_data = serde_json::to_value(WorkflowResponse {
                    workflow_id: "mock-workflow-456".to_string(),
                    status: "running".to_string(),
                    results: HashMap::from([
                        ("estimated_completion".to_string(), serde_json::Value::String("2024-12-31T12:00:00Z".to_string())),
                    ]),
                })?;
                Ok(CapabilityResponse::success(response_data))
            }
            _ => Ok(CapabilityResponse::error(format!(
                "Unknown capability: {}",
                request.capability_id
            ))),
        }
    }

    fn get_metadata(&self) -> HashMap<String, serde_json::Value> {
        HashMap::from([
            (
                "name".to_string(),
                serde_json::Value::String("Mock Orchestration Capability".to_string()),
            ),
            (
                "version".to_string(),
                serde_json::Value::String("1.0.0".to_string()),
            ),
            (
                "capabilities".to_string(),
                serde_json::json!([
                    "orchestration.service_coordination",
                    "orchestration.workflow_management"
                ]),
            ),
        ])
    }

    async fn health_check(&self) -> bool {
        self.enabled
    }
}

impl OrchestrationCapability for MockOrchestrationCapability {
    fn coordinate_services(
        &self,
        _request: ServiceCoordinationRequest,
    ) -> OrchestrationFuture<ServiceCoordinationResponse> {
        Box::pin(async move {
            Ok(ServiceCoordinationResponse {
                success: true,
                results: HashMap::from([
                    ("coordination_id".to_string(), serde_json::Value::String("mock-coord".to_string())),
                    ("status".to_string(), serde_json::Value::String("active".to_string())),
                    ("coordinated_services".to_string(), serde_json::json!([
                        "mock-service".to_string()
                    ])),
                ]),
            })
        })
    }

    fn execute_workflow(
        &self,
        _request: WorkflowRequest,
    ) -> OrchestrationFuture<WorkflowResponse> {
        Box::pin(async move {
            Ok(WorkflowResponse {
                workflow_id: "mock-workflow".to_string(),
                status: "running".to_string(),
                results: HashMap::new(),
            })
        })
    }
}
