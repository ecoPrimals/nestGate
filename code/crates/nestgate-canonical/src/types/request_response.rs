// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn canonical_response_default() {
        let r = CanonicalResponse::default();
        assert_eq!(r.request_id, "unknown");
        assert!(!r.success);
    }

    #[test]
    fn canonical_request_and_response_serde_roundtrip() {
        let req = CanonicalRequest {
            id: "1".to_string(),
            service_type: UnifiedServiceType::Storage,
            capability: CapabilityId {
                domain: "d".to_string(),
                capability: "c".to_string(),
                version: "v1".to_string(),
            },
            payload: HashMap::new(),
            timeout: Some(Duration::from_secs(5)),
        };
        let js = serde_json::to_string(&req).expect("req serde");
        let _: CanonicalRequest = serde_json::from_str(&js).expect("req de");

        let res = CanonicalResponse {
            request_id: "1".to_string(),
            success: true,
            payload: Some(serde_json::json!({ "k": 1 })),
            error: None,
            metrics: Some(ServiceMetrics::default()),
        };
        let js2 = serde_json::to_string(&res).expect("res serde");
        let back: CanonicalResponse = serde_json::from_str(&js2).expect("res de");
        assert!(back.success);
    }
}
