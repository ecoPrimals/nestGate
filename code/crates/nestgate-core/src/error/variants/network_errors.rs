//! Network error variants and utilities
//! Network Errors functionality and utilities.
//! This module provides network-specific error types and helper functions.

use super::core_errors::NetworkErrorDetails;

impl NetworkErrorDetails {
    /// Create a network error with just a message
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            operation: None,
            endpoint: None,
            network_data: None,
            context: None,
        }
    }

    /// Create a network error with operation context
    pub fn with_operation(message: impl Into<String>, operation: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            operation: Some(operation.into()),
            endpoint: None,
            network_data: None,
            context: None,
        }
    }

    /// Create a timeout error
    pub fn timeout(endpoint: impl Into<String>) -> Self {
        let endpoint_str = endpoint.into();
        Self {
            message: format!("Network timeout for endpoint: {endpoint_str}"),
            operation: Some("network_request".to_string()),
            endpoint: Some(endpoint_str),
            network_data: None,
            context: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_error_new() {
        let error = NetworkErrorDetails::new("Connection failed");

        assert_eq!(error.message, "Connection failed");
        assert_eq!(error.operation, None);
        assert_eq!(error.endpoint, None);
        assert!(error.network_data.is_none());
        assert!(error.context.is_none());
    }

    #[test]
    fn test_network_error_new_with_string() {
        let error = NetworkErrorDetails::new("Network error".to_string());

        assert_eq!(error.message, "Network error");
        assert_eq!(error.operation, None);
        assert_eq!(error.endpoint, None);
        assert!(error.network_data.is_none());
        assert!(error.context.is_none());
    }

    #[test]
    fn test_network_error_with_operation() {
        let error = NetworkErrorDetails::with_operation("Request failed", "http_get");

        assert_eq!(error.message, "Request failed");
        assert_eq!(error.operation, Some("http_get".to_string()));
        assert_eq!(error.endpoint, None);
        assert!(error.network_data.is_none());
        assert!(error.context.is_none());
    }

    #[test]
    fn test_network_error_with_operation_strings() {
        let error =
            NetworkErrorDetails::with_operation("POST failed".to_string(), "http_post".to_string());

        assert_eq!(error.message, "POST failed");
        assert_eq!(error.operation, Some("http_post".to_string()));
        assert_eq!(error.endpoint, None);
        assert!(error.network_data.is_none());
        assert!(error.context.is_none());
    }

    #[test]
    fn test_network_error_timeout() {
        let error = NetworkErrorDetails::timeout("https://api.example.com/v1/data");

        assert_eq!(
            error.message,
            "Network timeout for endpoint: https://api.example.com/v1/data"
        );
        assert_eq!(error.operation, Some("network_request".to_string()));
        assert_eq!(
            error.endpoint,
            Some("https://api.example.com/v1/data".to_string())
        );
        assert!(error.network_data.is_none());
        assert!(error.context.is_none());
    }

    #[test]
    fn test_network_error_timeout_with_string() {
        let endpoint = "http://localhost:8080/test".to_string();
        let error = NetworkErrorDetails::timeout(endpoint.clone());

        assert_eq!(
            error.message,
            format!("Network timeout for endpoint: {endpoint}")
        );
        assert_eq!(error.operation, Some("network_request".to_string()));
        assert_eq!(error.endpoint, Some(endpoint));
    }

    #[test]
    fn test_network_error_timeout_short_endpoint() {
        let error = NetworkErrorDetails::timeout("localhost");

        assert_eq!(error.message, "Network timeout for endpoint: localhost");
        assert_eq!(error.operation, Some("network_request".to_string()));
        assert_eq!(error.endpoint, Some("localhost".to_string()));
    }

    #[test]
    fn test_network_error_fields_independence() {
        let error1 = NetworkErrorDetails::new("Error 1");
        let error2 = NetworkErrorDetails::with_operation("Error 2", "op");
        let error3 = NetworkErrorDetails::timeout("endpoint");

        // Verify each error has different characteristics
        assert_ne!(error1.message, error2.message);
        assert_ne!(error2.message, error3.message);

        assert_eq!(error1.operation, None);
        assert_eq!(error2.operation, Some("op".to_string()));
        assert_eq!(error3.operation, Some("network_request".to_string()));

        assert_eq!(error1.endpoint, None);
        assert_eq!(error2.endpoint, None);
        assert_eq!(error3.endpoint, Some("endpoint".to_string()));
    }

    #[test]
    fn test_network_error_message_formatting() {
        let endpoint = "api.service.com";
        let error = NetworkErrorDetails::timeout(endpoint);

        // Verify the message format is correct
        let expected_message = format!("Network timeout for endpoint: {endpoint}");
        assert_eq!(error.message, expected_message);

        // Verify the endpoint is stored separately
        assert_eq!(error.endpoint.expect("Network operation failed"), endpoint);
    }
}
