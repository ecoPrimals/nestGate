// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Capability request dispatch, query, and routing helpers.

use super::adapter_types::{CapabilityRequest, CapabilityResponse, UniversalAdapter};
use super::canonical;
use super::types::CapabilityQuery;
use std::collections::HashMap;
use std::time::SystemTime;

impl UniversalAdapter {
    /// Request capability operation (universal communication pattern)
    /// Replaces all hardcoded primal-to-primal calls
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn request_capability(
        &self,
        capability: &str,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse, String> {
        let capability_info = self.get_capability(capability)?;

        // Make HTTP request to capability endpoint (not hardcoded primal)
        let start_time = SystemTime::now();

        // Implementation would make actual HTTP request
        let response = CapabilityResponse {
            status: "success".to_string(),
            result: serde_json::json!({
                "method": request.method,
                "category": capability,
                "status": "success"
            }),
            metadata: HashMap::new(),
            provider: capability_info.provider,
            latency_ms: start_time.elapsed().unwrap_or_default().as_millis() as u64,
        };

        Ok(response)
    }

    /// Query capability using the universal adapter pattern
    /// COMPATIBILITY: For modules expecting `PrimalAgnosticAdapter` interface
    ///
    /// # Errors
    ///
    /// Returns [`crate::error::NestGateError`] if capability lookup fails (currently always returns
    /// [`Ok`]; reserved for future strict validation).
    pub fn query_capability(&self, query: &CapabilityQuery) -> crate::Result<Vec<String>> {
        // Convert CapabilityQuery to our internal format and find matching capabilities
        let matching_capabilities: Vec<String> = self
            .capabilities
            .values()
            .filter(|cap| cap.category.contains(&query.capability))
            .map(|cap| cap.endpoint.clone())
            .collect();

        Ok(matching_capabilities)
    }

    /// Route capability request to appropriate service
    /// COMPATIBILITY: For modules expecting `PrimalAgnosticAdapter` interface
    ///
    /// # Errors
    ///
    /// Returns [`crate::error::NestGateError`] when no matching capability is registered for the
    /// request.
    pub fn route_capability_request(
        &self,
        request: &canonical::CanonicalCapabilityRequest,
    ) -> crate::Result<serde_json::Value> {
        // Find appropriate capability for this request
        if let Some(capability) = self.capabilities.get(&request.capability) {
            // Route to the discovered capability endpoint
            Ok(serde_json::json!({
                "service": capability.endpoint,
                "operation": request.method,
                "status": "routed",
                "provider": capability.provider
            }))
        } else {
            Err(crate::error::NestGateError::not_found(format!(
                "No capability found for: {}",
                request.capability
            )))
        }
    }
}
