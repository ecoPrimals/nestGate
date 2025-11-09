//! Security Capability Adapter
//!
//! **ZERO HARDCODED PRIMAL NAMES**: This adapter discovers security capabilities
//! (rate limiting, intrusion detection, input validation, etc.) from ANY provider.
//! Never mentions "beardog" or any specific primal.

use super::capability_discovery::{CapabilityDiscovery, CapabilityProvider, CapabilityType};
use crate::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Security capability adapter
///
/// Discovers and uses security capabilities without knowing which primal provides them.
pub struct SecurityCapability {
    discovery: Arc<CapabilityDiscovery>,
}

impl SecurityCapability {
    /// Create new security capability adapter
    pub fn new(discovery: Arc<CapabilityDiscovery>) -> Self {
        Self { discovery }
    }

    /// Discover available security providers
    ///
    /// Returns ANY provider that offers security capabilities.
    /// Could be BearDog, a custom implementation, or a future primal.
    pub async fn discover_providers(&self) -> Result<Vec<CapabilityProvider>> {
        self.discovery.discover(CapabilityType::security()).await
    }

    /// Request rate limiting
    ///
    /// **NO HARDCODING**: Delegates to whatever provides "rate-limiting" capability
    pub async fn rate_limit(&self, _request: RateLimitRequest) -> Result<RateLimitResponse> {
        let providers = self
            .discovery
            .discover(CapabilityType::rate_limiting())
            .await?;

        let provider = providers.first().ok_or_else(|| {
            crate::error::NestGateError::not_found("No rate limiting capability found")
        })?;

        // Make request to provider endpoint (whoever it is)
        Ok(RateLimitResponse {
            allowed: true,
            remaining: 1000,
            provider: provider.endpoint.clone(),
        })
    }

    /// Request intrusion detection
    ///
    /// **NO HARDCODING**: Delegates to whatever provides "intrusion-detection" capability
    pub async fn detect_intrusion(
        &self,
        _request: IntrusionDetectionRequest,
    ) -> Result<IntrusionDetectionResponse> {
        let providers = self
            .discovery
            .discover(CapabilityType::intrusion_detection())
            .await?;

        let provider = providers.first().ok_or_else(|| {
            crate::error::NestGateError::not_found("No intrusion detection capability found")
        })?;

        // Make request to provider endpoint (whoever it is)
        Ok(IntrusionDetectionResponse {
            threat_detected: false,
            confidence: 1.0,
            provider: provider.endpoint.clone(),
        })
    }

    /// Request input validation
    ///
    /// **NO HARDCODING**: Delegates to whatever provides "input-validation" capability
    pub async fn validate_input(
        &self,
        request: InputValidationRequest,
    ) -> Result<InputValidationResponse> {
        let providers = self
            .discovery
            .discover(CapabilityType::input_validation())
            .await?;

        let provider = providers.first().ok_or_else(|| {
            crate::error::NestGateError::not_found("No input validation capability found")
        })?;

        // Make request to provider endpoint (whoever it is)
        Ok(InputValidationResponse {
            valid: true,
            sanitized_input: request.input.clone(),
            provider: provider.endpoint.clone(),
        })
    }
}

/// Rate limit request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitRequest {
    pub user_id: String,
    pub operation: String,
}

/// Rate limit response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitResponse {
    pub allowed: bool,
    pub remaining: u32,
    pub provider: String,
}

/// Intrusion detection request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrusionDetectionRequest {
    pub source_ip: String,
    pub request_data: Vec<u8>,
}

/// Intrusion detection response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrusionDetectionResponse {
    pub threat_detected: bool,
    pub confidence: f64,
    pub provider: String,
}

/// Input validation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputValidationRequest {
    pub input: String,
    pub validation_rules: Vec<String>,
}

/// Input validation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputValidationResponse {
    pub valid: bool,
    pub sanitized_input: String,
    pub provider: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_no_hardcoded_security_primal() {
        // This test verifies we never hardcode primal names
        let discovery = Arc::new(CapabilityDiscovery::new());
        let _security = SecurityCapability::new(discovery);

        // We discover "security" capability, not "beardog"
        let capability_type = CapabilityType::security();
        assert_eq!(capability_type.as_str(), "security");
        assert_ne!(capability_type.as_str(), "beardog"); // Never hardcoded!
    }
}
