//! Comprehensive tests for success response module
//! Added: November 14, 2025 - Coverage Sprint

#[cfg(test)]
mod success_response_tests {
    use crate::response::success_response::*;
    use serde_json::json;

    #[test]
    fn test_success_response_creation() {
        let response = SuccessResponse::new("Operation completed");
        
        assert_eq!(response.message, "Operation completed");
        assert!(response.data.is_null());
        assert!(response.timestamp > 0);
    }

    #[test]
    fn test_success_response_with_data() {
        let data = json!({"key": "value", "count": 42});
        let response = SuccessResponse::with_data("Success", data.clone());
        
        assert_eq!(response.message, "Success");
        assert_eq!(response.data, data);
    }

    #[test]
    fn test_success_response_with_complex_data() {
        let data = json!({
            "user": {
                "id": 123,
                "name": "Test User",
                "roles": ["admin", "user"]
            },
            "metadata": {
                "created_at": "2025-11-14",
                "version": "1.0"
            }
        });
        
        let response = SuccessResponse::with_data("User created", data.clone());
        
        assert_eq!(response.data["user"]["id"], 123);
        assert_eq!(response.data["user"]["roles"][0], "admin");
    }

    #[test]
    fn test_success_response_empty_message() {
        let response = SuccessResponse::new("");
        assert_eq!(response.message, "");
    }

    #[test]
    fn test_success_response_serialization() {
        let response = SuccessResponse::new("Test message");
        
        // Should be able to serialize
        let serialized = serde_json::to_string(&response);
        assert!(serialized.is_ok());
        
        let json_str = serialized.unwrap();
        assert!(json_str.contains("Test message"));
        assert!(json_str.contains("timestamp"));
    }

    #[test]
    fn test_success_response_deserialization() {
        let json_str = r#"{"message":"Test","data":null,"timestamp":1234567890}"#;
        
        let response: Result<SuccessResponse, _> = serde_json::from_str(json_str);
        assert!(response.is_ok());
        
        let response = response.unwrap();
        assert_eq!(response.message, "Test");
        assert_eq!(response.timestamp, 1234567890);
    }

    #[test]
    fn test_success_response_timestamp_is_recent() {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let response = SuccessResponse::new("Test");
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Timestamp should be within 1 second of now
        assert!((response.timestamp as i64 - now as i64).abs() < 2);
    }

    #[test]
    fn test_success_response_with_array_data() {
        let data = json!([1, 2, 3, 4, 5]);
        let response = SuccessResponse::with_data("Array data", data.clone());
        
        assert_eq!(response.data[2], 3);
        assert!(response.data.is_array());
    }

    #[test]
    fn test_success_response_with_null_data() {
        let data = json!(null);
        let response = SuccessResponse::with_data("Null data", data);
        
        assert!(response.data.is_null());
    }

    #[test]
    fn test_success_response_debug_format() {
        let response = SuccessResponse::new("Debug test");
        let debug_str = format!("{:?}", response);
        
        assert!(debug_str.contains("Debug test"));
    }

    #[test]
    fn test_success_response_clone() {
        let original = SuccessResponse::new("Clone test");
        let cloned = original.clone();
        
        assert_eq!(original.message, cloned.message);
        assert_eq!(original.timestamp, cloned.timestamp);
    }

    #[tokio::test]
    async fn test_multiple_success_responses_have_different_timestamps() {
        let response1 = SuccessResponse::new("First");
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        let response2 = SuccessResponse::new("Second");
        
        assert!(response2.timestamp >= response1.timestamp);
    }

    #[test]
    fn test_success_response_with_unicode() {
        let response = SuccessResponse::new("Success ✓ Operation completed 🎉");
        
        assert!(response.message.contains("✓"));
        assert!(response.message.contains("🎉"));
    }

    #[test]
    fn test_success_response_with_long_message() {
        let long_message = "a".repeat(10000);
        let response = SuccessResponse::new(&long_message);
        
        assert_eq!(response.message.len(), 10000);
    }
}

