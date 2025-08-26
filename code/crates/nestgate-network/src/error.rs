/// **NETWORK ERROR TYPES**
///
/// Unified error handling for network operations and configurations.

// ==================== CANONICAL MODERNIZATION ====================

/// **CANONICAL**: Network-specific Result type using IdioResult
/// This follows the canonical Result<T,E> pattern with domain-specific error type
pub type NetworkResult<T> = IdioResult<T, NetworkError>;

// **CANONICAL MODERNIZATION COMPLETE**: Deprecated Result type alias removed
// Use NetworkResult<T> for domain-specific network errors

/// Helper trait for creating network errors
pub trait NetworkErrorExt {
    fn network_error(message: &str, operation: &str, endpoint: Option<&str>) -> NestGateError;
    fn invalid_network_config(field: &str, message: &str) -> NestGateError;
}

impl NetworkErrorExt for NestGateError {
    fn network_error(message: &str, operation: &str, endpoint: Option<&str>) -> NestGateError {
        NestGateError::network_error(message, operation, endpoint)
    }

    fn invalid_network_config(field: &str, message: &str) -> NestGateError {
        NestGateError::invalid_input(field.to_string(), message.to_string())
    }
}

// ==================== ERROR CONVERSION HELPERS ====================

/// Convert reqwest::Error to NestGateError
pub fn convert_reqwest_error(err: reqwest::Error) -> NestGateError {
    let message = err.to_string();
    let operation = if err.is_timeout() {
        "http_timeout"
    } else if err.is_connect() {
        "http_connect"
    } else if err.is_request() {
        "http_request"
    } else {
        "http_operation"
    };

    NestGateError::network_error(&message, operation, None)
}

/// Convert serde_json::Error to NestGateError
pub fn convert_json_error(err: serde_json::Error) -> NestGateError {
    NestGateError::network_error(
        &format!("JSON serialization error: {}", err),
        "json_processing",
        None,
    )
}

/// Convert std::io::Error to NestGateError
pub fn convert_io_error(err: std::io::Error) -> NestGateError {
    NestGateError::network_error(&format!("IO error: {}", err), "io_operation", None)
}
