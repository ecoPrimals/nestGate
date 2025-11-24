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
pub struct CanonicalRequest {
    pub id: String,
    pub service_type: UnifiedServiceType,
    pub capability: CapabilityId,
    pub payload: HashMap<String, serde_json::Value>,
    pub timeout: Option<Duration>,
}

/// Canonical Response Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalResponse {
    pub request_id: String,
    pub success: bool,
    pub payload: Option<serde_json::Value>,
    pub error: Option<String>,
    pub metrics: Option<ServiceMetrics>,
}

impl Default for CanonicalResponse {
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
