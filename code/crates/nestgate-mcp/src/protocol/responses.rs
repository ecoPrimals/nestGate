//! **RESPONSE TYPES**
//!
//! MCP response structures and status types.

use crate::types::{MountInfo, ProviderCapabilities, VolumeInfo};
use nestgate_core::diagnostics::SystemMetrics;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::errors::ErrorPayload;
use super::federation::FederationStatus;
use super::orchestrator::LoadBalancingInfo;
use super::services::{HealthStatus, ServiceInfo};

/// Enhanced MCP Response with advanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// Unique identifier
    pub id: String,
    /// Request identifier
    pub request_id: String,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
    /// Status
    pub status: ResponseStatus,
    /// Payload
    pub payload: Option<ResponsePayload>,
    /// Error
    pub error: Option<ErrorPayload>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

impl Response {
    /// Create success response
    #[must_use]
    pub fn success(request_id: String, payload: ResponsePayload) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            request_id,
            timestamp: std::time::SystemTime::now(),
            status: ResponseStatus::Success,
            payload: Some(payload),
            error: None,
            metadata: HashMap::new(),
        }
    }

    /// Create error response
    #[must_use]
    pub fn error(request_id: String, error: ErrorPayload) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            request_id,
            timestamp: std::time::SystemTime::now(),
            status: ResponseStatus::Error,
            payload: None,
            error: Some(error),
            metadata: HashMap::new(),
        }
    }
}

/// Response status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseStatus {
    Success,
    Error,
    Pending,
    Timeout,
}

/// Response payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponsePayload {
    CapabilityResponse(ProviderCapabilities),
    VolumeInfo(VolumeInfo),
    VolumeList(Vec<VolumeInfo>),
    MountInfo(MountInfo),
    MetricsReport(SystemMetrics),
    HealthStatus(HealthStatus),
    FederationStatus(FederationStatus),
    ServiceList(Vec<ServiceInfo>),
    LoadBalancingInfo(LoadBalancingInfo),
    Empty,
}
