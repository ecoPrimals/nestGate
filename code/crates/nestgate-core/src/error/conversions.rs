/// Error Type Conversions
///
/// This module provides conversion implementations from domain-specific error types
/// to the unified NestGateError system, enabling seamless error handling across
/// all NestGate components.
use crate::error::core::NestGateError;
// Legacy domain_errors import removed - use unified error system instead
use std::collections::HashMap;
use std::io;

// ==================== SECTION ====================

impl From<io::Error> for NestGateError {
    fn from(err: io::Error) -> Self {
        NestGateError::Io {
            operation: "io_operation".to_string(),
            error_message: err.to_string(),
            resource: None,
            retryable: matches!(
                err.kind(),
                io::ErrorKind::TimedOut | io::ErrorKind::Interrupted
            ),
        }
    }
}

impl From<String> for NestGateError {
    fn from(message: String) -> Self {
        NestGateError::Configuration {
            message,
            
            field: None,
            
        }
    }
}

// ==================== SECTION ====================

// Helper function to create ZfsErrorData from string message and operation
pub fn create_zfs_error(message: String, operation: ZfsOperation) -> NestGateError {
    NestGateError::Zfs(Box::new(ZfsErrorData {
        message,
        operation,
        pool: None,
        dataset: None,
        snapshot: None,
        command: None,
        error_code: None,
        recovery_suggestions: Vec::new(),
                    performance_metrics: None,
                    environment: None,
    }))
}

// ==================== SECTION ====================

// Helper function to create PrimalErrorData from string message and operation
pub fn create_primal_error(message: String, operation: PrimalOperation) -> NestGateError {
    NestGateError::Primal(Box::new(PrimalErrorData {
        message,
        operation,
        primal_id: None,
        request_context: None,
        capability: None,
        metadata: HashMap::new(),
    }))
}

// ==================== SECTION ====================

// Helper function to create UniversalZfsErrorData
pub fn create_universal_zfs_error(message: String, operation: String) -> NestGateError {
    NestGateError::UniversalZfs(Box::new(UniversalZfsErrorData {
        message,
        operation,
        backend: None,
        resource: None,
        duration: None,
        circuit_breaker_open: false,
        rate_limit_info: None,
    }))
}

// ==================== SECTION ====================

// Automation errors are already integrated, but we can add helper functions
pub fn create_automation_error(message: String, operation: AutomationOperation) -> NestGateError {
    NestGateError::Automation(Box::new(AutomationErrorData {
        message,
        operation,
        target: None,
        analysis_context: None,
        discovery_context: None,
        cache_context: None,
    }))
}

// ==================== SECTION ====================

// Middleware errors are already integrated, but we can add helper functions
pub fn create_middleware_error(message: String, component: MiddlewareComponent) -> NestGateError {
    NestGateError::Middleware(Box::new(MiddlewareErrorData {
        message,
        component,
        request_context: None,
        validation_context: None,
        handler_context: None,
    }))
}

// ==================== SECTION ====================

/// Create a generic internal error for backward compatibility
pub fn create_internal_error(message: String) -> NestGateError {
    NestGateError::Internal {
        message,
        location: Some(format!("{}:{}", file!(), line!())),
        context: None,
        is_bug: false,
    }
}

/// Create a configuration error for backward compatibility  
pub fn create_config_error(message: String, field: Option<String>) -> NestGateError {
    NestGateError::Configuration {
        message,
        
        field,
        
    }
}

/// Create a validation error for backward compatibility
pub fn create_validation_error(field: String, message: String) -> NestGateError {
    NestGateError::Validation {
        field,
        message,
        current_value: None,
        expected: None,
        user_error: true,
                context: None,
    }
}
