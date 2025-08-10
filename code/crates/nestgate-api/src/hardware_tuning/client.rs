//! Universal Compute Service Client
//!
//! ✅ **MODERNIZED**: Capability-based compute service integration
//! ❌ **DEPRECATED**: Legacy primal-specific client implementations

use crate::hardware_tuning::types::*;
use nestgate_core::ecosystem_integration::universal_adapter::UniversalAdapter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Universal compute service client (capability-based)
#[derive(Debug, Clone)]
pub struct UniversalComputeClient {
    adapter: Arc<UniversalAdapter>,
    timeout_seconds: u64,
}

impl UniversalComputeClient {
    /// Create new universal compute client
    pub fn new(adapter: Arc<UniversalAdapter>) -> Self {
        Self {
            adapter,
            timeout_seconds: 30,
        }
    }

    /// Request compute resources through discovered compute capability
    pub async fn request_compute_resources(
        &self,
        request: ComputeRequest,
    ) -> Result<ComputeAllocation, ComputeError> {
        info!("🧮 Requesting compute resources: {:?}", request);

        // ✅ CAPABILITY-BASED: Discover compute service dynamically
        let capability_request =
            nestgate_core::ecosystem_integration::universal_adapter::types::CapabilityRequest {
                request_id: uuid::Uuid::new_v4().to_string(),
                capability_id: "compute".to_string(),
                payload: vec![],
                metadata: std::collections::HashMap::new(),
                performance_requirements: None,
                timeout: Some(std::time::Duration::from_secs(30)),
                priority: 1,
                requires_encryption: false,
            };
        let compute_service = self
            .adapter
            .execute_capability(capability_request)
            .await
            .map_err(|e| {
                ComputeError::DiscoveryError(format!("No compute capability found: {}", e))
            })?;

        debug!("✅ Using compute service: {:?}", compute_service);

        // Make request to discovered compute service
        let client = reqwest::Client::new();
        let response = client
            .post(&format!(
                "{}/api/v1/compute/allocate",
                String::from_utf8_lossy(&compute_service.payload).trim_matches('"')
            ))
            .json(&request)
            .timeout(std::time::Duration::from_secs(self.timeout_seconds))
            .send()
            .await
            .map_err(|e| ComputeError::NetworkError(e.to_string()))?;

        if response.status().is_success() {
            let allocation = response
                .json::<ComputeAllocation>()
                .await
                .map_err(|e| ComputeError::ParseError(e.to_string()))?;

            info!("✅ Compute resources allocated: {:?}", allocation);
            Ok(allocation)
        } else {
            let error_msg = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(ComputeError::ServiceError(error_msg))
        }
    }

    /// Release compute resources
    pub async fn release_compute_resources(&self, allocation_id: &str) -> Result<(), ComputeError> {
        info!("🔄 Releasing compute resources: {}", allocation_id);

        let capability_request =
            nestgate_core::ecosystem_integration::universal_adapter::types::CapabilityRequest {
                request_id: uuid::Uuid::new_v4().to_string(),
                capability_id: "compute".to_string(),
                payload: vec![],
                metadata: std::collections::HashMap::new(),
                performance_requirements: None,
                timeout: Some(std::time::Duration::from_secs(30)),
                priority: 1,
                requires_encryption: false,
            };
        let compute_service = self
            .adapter
            .execute_capability(capability_request)
            .await
            .map_err(|e| {
                ComputeError::DiscoveryError(format!("No compute capability found: {}", e))
            })?;

        let client = reqwest::Client::new();
        let response = client
            .delete(&format!(
                "{}/api/v1/compute/{}",
                String::from_utf8_lossy(&compute_service.payload).trim_matches('"'),
                allocation_id
            ))
            .timeout(std::time::Duration::from_secs(self.timeout_seconds))
            .send()
            .await
            .map_err(|e| ComputeError::NetworkError(e.to_string()))?;

        if response.status().is_success() {
            info!("✅ Compute resources released: {}", allocation_id);
            Ok(())
        } else {
            let error_msg = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(ComputeError::ServiceError(error_msg))
        }
    }

    /// Get compute service status
    pub async fn get_compute_status(&self) -> Result<ComputeStatus, ComputeError> {
        let capability_request =
            nestgate_core::ecosystem_integration::universal_adapter::types::CapabilityRequest {
                request_id: uuid::Uuid::new_v4().to_string(),
                capability_id: "compute".to_string(),
                payload: vec![],
                metadata: std::collections::HashMap::new(),
                performance_requirements: None,
                timeout: Some(std::time::Duration::from_secs(30)),
                priority: 1,
                requires_encryption: false,
            };
        let compute_service = self
            .adapter
            .execute_capability(capability_request)
            .await
            .map_err(|e| {
                ComputeError::DiscoveryError(format!("No compute capability found: {}", e))
            })?;

        let client = reqwest::Client::new();
        let response = client
            .get(&format!(
                "{}/api/v1/compute/status",
                String::from_utf8_lossy(&compute_service.payload).trim_matches('"')
            ))
            .timeout(std::time::Duration::from_secs(self.timeout_seconds))
            .send()
            .await
            .map_err(|e| ComputeError::NetworkError(e.to_string()))?;

        if response.status().is_success() {
            let status = response
                .json::<ComputeStatus>()
                .await
                .map_err(|e| ComputeError::ParseError(e.to_string()))?;
            Ok(status)
        } else {
            let error_msg = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(ComputeError::ServiceError(error_msg))
        }
    }
}

/// Compute error types
#[derive(Debug, thiserror::Error)]
pub enum ComputeError {
    #[error("Discovery error: {0}")]
    DiscoveryError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Service error: {0}")]
    ServiceError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Compute service status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeStatus {
    pub available_resources: ComputeResources,
    pub allocated_resources: ComputeResources,
    pub active_allocations: u32,
    pub service_health: f64,
}
