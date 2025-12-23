//! HTTP Client Error Types
//!
//! Domain-specific errors for HTTP client operations with
//! automatic conversion to NestGateError.

use std::time::Duration;
use thiserror::Error;

use crate::error::NestGateError;

// ==================== HTTP CLIENT ERROR ====================

/// HTTP client specific errors
///
/// Domain-specific errors that automatically convert to NestGateError
/// for unified error handling across the system.
#[derive(Debug, Error)]
pub enum HttpClientError {
    /// Connection to server failed
    #[error("Connection failed: {message}")]
    ConnectionFailed {
        /// Error message describing the connection failure
        message: String,
    },

    /// Request timed out
    #[error("Request timeout after {timeout:?}")]
    Timeout {
        /// The timeout duration that was exceeded
        timeout: Duration,
    },

    /// Server returned an invalid response
    #[error("Invalid response: {message}")]
    InvalidResponse {
        /// Error message describing why the response is invalid
        message: String,
    },

    /// Too many HTTP redirects were encountered
    #[error("Too many redirects: {count}")]
    TooManyRedirects {
        /// The number of redirects that were attempted
        count: usize,
    },

    /// Request was cancelled
    #[error("Request cancelled")]
    Cancelled,

    /// Invalid URL provided
    #[error("Invalid URL: {url}")]
    InvalidUrl {
        /// The URL that was invalid
        url: String,
    },

    /// TLS/SSL error
    #[error("TLS error: {message}")]
    TlsError {
        /// Error message describing the TLS failure
        message: String,
    },
}

// ==================== CONVERSIONS ====================

impl From<HttpClientError> for NestGateError {
    fn from(err: HttpClientError) -> Self {
        match err {
            HttpClientError::ConnectionFailed { message } => NestGateError::network_error(&message),
            HttpClientError::Timeout { timeout } => {
                NestGateError::timeout_error("HTTP request", timeout)
            }
            HttpClientError::InvalidResponse { message } => {
                NestGateError::validation_error(&format!("Invalid HTTP response: {}", message))
            }
            HttpClientError::TooManyRedirects { count } => {
                NestGateError::network_error(&format!("Too many redirects: {}", count))
            }
            HttpClientError::Cancelled => NestGateError::network_error("HTTP request cancelled"),
            HttpClientError::InvalidUrl { url } => {
                NestGateError::validation_error(&format!("Invalid URL: {}", url))
            }
            HttpClientError::TlsError { message } => {
                NestGateError::network_error(&format!("TLS error: {}", message))
            }
        }
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_failed_error() {
        let err = HttpClientError::ConnectionFailed {
            message: "Connection refused".to_string(),
        };
        assert_eq!(err.to_string(), "Connection failed: Connection refused");
    }

    #[test]
    fn test_timeout_error() {
        let err = HttpClientError::Timeout {
            timeout: Duration::from_secs(30),
        };
        assert!(err.to_string().contains("timeout"));
    }

    #[test]
    fn test_invalid_response_error() {
        let err = HttpClientError::InvalidResponse {
            message: "Malformed JSON".to_string(),
        };
        assert_eq!(err.to_string(), "Invalid response: Malformed JSON");
    }

    #[test]
    fn test_error_conversion() {
        let client_err = HttpClientError::ConnectionFailed {
            message: "Test".to_string(),
        };
        let _nestgate_err: NestGateError = client_err.into();
        // Conversion should succeed
    }
}
