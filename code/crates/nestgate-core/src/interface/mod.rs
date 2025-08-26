use crate::NestGateError;
use std::collections::HashMap;
//
// **CANONICAL MODERNIZATION COMPLETE** - Unified interface definitions
//
// This module provides canonical interface definitions that replace all
// deprecated and fragmented interface patterns.

// **CANONICAL INTERFACES** - Modern unified interface definitions
pub mod core_interfaces;
pub mod event_types;
pub mod health_status;
pub mod storage_types;

// Re-export all types for backward compatibility and ease of use
pub use core_interfaces::{
    ToUnified,
    UniversalConfigInterface,
    UniversalEventInterface,
    UniversalProviderInterface,
    // **REMOVED**: InterfaceResult (use unified Result<T> instead)
    // **REMOVED**: UniversalServiceInterface, UniversalStorageInterface (use canonical traits instead)
};
pub use event_types::{EventHandler, EventPriority, EventSubscription, UnifiedEvent};
pub use health_status::{HealthState, UnifiedHealthStatus, UnifiedServiceMetrics};
pub use storage_types::{StorageMetrics, StorageResource, StorageResourceConfig};

// Import unified configuration types
use crate::traits::{UniversalResponseStatus, UniversalServiceResponse};
use crate::canonical_modernization::UnifiedServiceConfig;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Universal Service Request - re-export from canonical traits
pub use crate::traits::UniversalServiceRequest;

// ==================== ERROR EXTENSION METHODS ====================
// These extend NestGateError with interface-specific convenience methods

/// Helper constructors for interface-specific errors
impl crate::error::NestGateError {
    /// Create a service unavailable error
    pub fn service_unavailable(service: String, reason: String) -> Self {
        Self::System {
            message: format!("Service '{service}' is unavailable: {reason}"),
            resource: crate::error::SystemResource::Network,
            utilization: None,
            recovery: crate::error::RecoveryStrategy::Retry,
        }
    }

    /// Create a configuration error
    pub fn configuration_error(message: String, field: Option<String>) -> Self {
        Self::Configuration {
            message,
            config_source: crate::error::UnifiedConfigSource::Runtime,
            field,
            suggested_fix: Some("Check configuration values and schema".to_string()),
        }
    }

    /// Create an invalid input error
    pub fn invalid_input(field: String, message: String) -> Self {
        Self::Validation {
            field,
            message,
            current_value: None,
            expected: None,
            user_error: true,
        }
    }

    /// Create a system error with resource context (modernized)
    pub fn system_timeout_error(operation: String, timeout_ms: u64) -> Self {
        Self::Timeout {
            operation,
            duration: std::time::Duration::from_millis(timeout_ms),
            retryable: true,
            suggested_timeout: Some(std::time::Duration::from_millis(timeout_ms * 2)),
        }
    }

    /// Create a permission denied error
    pub fn permission_denied_error(resource: String, operation: String) -> Self {
        Self::Security(Box::new(crate::error::SecurityErrorData {
            message: format!("Permission denied for {operation} on resource {resource}"),
            operation: operation.clone(),
            resource: Some(resource),
            principal: None,
            context: None,
        }))
    }

    /// Create a not found error
    pub fn not_found_error(resource_type: String, resource_id: String) -> Self {
        Self::System {
            message: format!("{resource_type} with ID '{resource_id}' not found"),
            resource: crate::error::SystemResource::Memory,
            utilization: None,
            recovery: crate::error::RecoveryStrategy::Continue,
        }
    }

    /// Create an internal error
    pub fn internal_error(message: String, context: String) -> Self {
        Self::Internal {
            message,
            location: Some(context),
            debug_info: Some("Internal system error occurred".to_string()),
            is_bug: true,
        }
    }
}

// ==================== INTERFACE VALIDATION HELPERS ====================

/// Validation helpers for interface components
pub mod validation {
    use super::*;

    /// Validate service info completeness
    pub fn validate_service_info(
        config: &crate::canonical_modernization::UnifiedServiceConfig,
    ) -> crate::error::CanonicalResult<()> {
        if config.name.is_empty() {
            return Err(crate::error::NestGateError::invalid_input(
                "name".to_string(),
                "Service name cannot be empty".to_string(),
            ));
        }

        if config.version.is_empty() {
            return Err(crate::error::NestGateError::invalid_input(
                "version".to_string(),
                "Service version cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    /// Validate request format and structure
    pub fn validate_request(request: &UniversalServiceRequest) -> crate::error::CanonicalResult<()> {
        if request.id.is_empty() {
            return Err(crate::error::NestGateError::invalid_input(
                "request_id".to_string(),
                "Request ID cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    /// Validate storage resource configuration
    pub fn validate_storage_config(config: &StorageResourceConfig) -> crate::error::CanonicalResult<()> {
        if config.resource_type.is_empty() {
            return Err(crate::error::NestGateError::invalid_input(
                "resource_type".to_string(),
                "Resource type cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

// ==================== INTERFACE FACTORIES ====================

/// Factory methods for creating common interface objects
pub mod factories {
    use super::*;

    /// Create a standard service config object
    pub fn create_service_config(
        name: &str,
        version: &str,
        description: &str,
        service_id: &str,
    ) -> crate::canonical_modernization::UnifiedServiceConfig {
        crate::canonical_modernization::UnifiedServiceConfig {
            name: name.to_string(),
            version: version.to_string(),
            port: 8080,
            bind_address: "0.0.0.0".to_string(),
            metadata: {
                let mut metadata = std::collections::HashMap::new();
                metadata.insert("description".to_string(), description.to_string());
                metadata.insert("service_id".to_string(), service_id.to_string());
                metadata.insert("service_type".to_string(), "Generic".to_string());
                metadata
            },
        }
    }

    /// Create a success response
    pub fn success_response(
        request_id: &str,
        data: Option<serde_json::Value>,
    ) -> UniversalServiceResponse {
        UniversalServiceResponse {
            request_id: request_id.to_string(),
            status: UniversalResponseStatus::Success,
            data,
            error: None,
            metadata: std::collections::HashMap::new(),
            processing_time_ms: Some(0),
            headers: std::collections::HashMap::new(),
            body: Vec::new(),
        }
    }

    /// Create an error response
    pub fn error_response(request_id: &str, error_message: &str) -> UniversalServiceResponse {
        UniversalServiceResponse {
            request_id: request_id.to_string(),
            status: UniversalResponseStatus::Error,
            data: None,
            error: Some(error_message.to_string()),
            metadata: std::collections::HashMap::new(),
            processing_time_ms: Some(0),
            headers: std::collections::HashMap::new(),
            body: Vec::new(),
        }
    }

    /// Create a healthy status
    pub fn healthy_status(_message: &str) -> UnifiedHealthStatus {
        UnifiedHealthStatus::Healthy
    }

    /// Create an unhealthy status
    pub fn unhealthy_status(_message: &str) -> UnifiedHealthStatus {
        UnifiedHealthStatus::Unhealthy
    }

    /// Create a default service configuration
    pub fn default_service_config(service_id: &str) -> UnifiedServiceConfig {
        UnifiedServiceConfig {
            name: service_id.to_string(),
            version: "1.0.0".to_string(),
            port: 8080,
            bind_address: "0.0.0.0".to_string(),
            metadata: {
                let mut metadata = std::collections::HashMap::new();
                metadata.insert("service_type".to_string(), "Generic".to_string());
                metadata
            },
        }
    }
}
