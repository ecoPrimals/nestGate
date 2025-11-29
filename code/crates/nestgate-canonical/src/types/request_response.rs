//! Request and Response Type Definitions
//!
//! Canonical request/response types for service communication.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::health_types::ServiceMetrics;
use super::service_types::{CapabilityId, UnifiedServiceType};

/// Canonical Request Type
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Canonical operation
pub struct CanonicalRequest {
    /// Unique identifier
    pub id: String,
    /// Service Type
    pub service_type: UnifiedServiceType,
    /// Capability
    pub capability: CapabilityId,
    /// Payload
    pub payload: HashMap<String, serde_json::Value>,
    /// Timeout
    pub timeout: Option<Duration>,
}

/// Canonical Response Type
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Canonical operation
pub struct CanonicalResponse {
    /// Request identifier
    pub request_id: String,
    /// Success
    pub success: bool,
    /// Payload
    pub payload: Option<serde_json::Value>,
    /// Error
    pub error: Option<String>,
    /// Metrics
    pub metrics: Option<ServiceMetrics>,
}

impl Default for CanonicalResponse {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            request_id: "unknown".to_string(),
            success: false,
            payload: None,
            error: None,
            metrics: None,
        }
    }
}
