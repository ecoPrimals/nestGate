//! Comprehensive tests for error utilities
//! Added: November 21, 2025 - Coverage Expansion
//!
//! Target: Complete coverage of error utility functions

#[cfg(test)]
mod error_utilities_tests {
    use super::super::utilities::*;
    use super::super::Result;
    use std::path::Path;
    use std::sync::{mpsc, Mutex};

    // ==================== Safe String Conversion Tests ====================

    #[test]
    fn test_safe_to_string_integer() {
        assert_eq!(safe_to_string(42), "42");
        assert_eq!(safe_to_string(0), "0");
        assert_eq!(safe_to_string(-123), "-123");
    }

    #[test]
    fn test_safe_to_string_float() {
        assert_eq!(safe_to_string(3.15), "3.15");
        assert_eq!(safe_to_string(0.0), "0");
    }

    #[test]
    fn test_safe_to_string_string() {
        assert_eq!(safe_to_string("hello"), "hello");
        assert_eq!(safe_to_string(""), "");
    }

    #[test]
    fn test_safe_to_string_bool() {
        assert_eq!(safe_to_string(true), "true");
        assert_eq!(safe_to_string(false), "false");
    }

    #[test]
    fn test_safe_to_string_char() {
        assert_eq!(safe_to_string('a'), "a");
        assert_eq!(safe_to_string('🚀'), "🚀");
    }

    // ==================== Environment Variable Tests ====================

    #[test]
    fn test_safe_env_var_existing() {
        std::env::set_var("TEST_VAR_UTIL_123", "test_value");
        let result = safe_env_var("TEST_VAR_UTIL_123");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_value");
        std::env::remove_var("TEST_VAR_UTIL_123");
    }

    #[test]
    fn test_safe_env_var_missing() {
        let result = safe_env_var("NONEXISTENT_VAR_UTIL_XYZ_999");
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_env_var_error_message() {
        let result = safe_env_var("MISSING_VAR_ERROR");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let debug_str = format!("{:?}", err);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_safe_env_var_or_default_existing() {
        std::env::set_var("TEST_VAR_DEFAULT_UTIL", "actual");
        let result = safe_env_var_or_default("TEST_VAR_DEFAULT_UTIL", "default");
        assert_eq!(result, "actual");
        std::env::remove_var("TEST_VAR_DEFAULT_UTIL");
    }

    #[test]
    fn test_safe_env_var_or_default_missing() {
        let result = safe_env_var_or_default("MISSING_VAR_UTIL_XYZ", "fallback");
        assert_eq!(result, "fallback");
    }

    #[test]
    fn test_safe_env_var_or_default_empty_default() {
        let result = safe_env_var_or_default("NONEXISTENT_EMPTY_UTIL", "");
        assert_eq!(result, "");
    }

    #[test]
    fn test_safe_env_var_or_default_special_chars() {
        let result = safe_env_var_or_default("MISSING_SPECIAL", "default with spaces");
        assert_eq!(result, "default with spaces");
    }

    // ==================== File Operation Tests ====================

    #[test]
    fn test_safe_read_to_string_nonexistent() {
        let result = safe_read_to_string(Path::new("/nonexistent/file/util/xyz123.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_read_to_string_existing() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_util_read.txt");
        let test_content = "test content for safe read";

        std::fs::write(&test_file, test_content).expect("Failed to write test file");

        let result = safe_read_to_string(&test_file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_content);

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn test_safe_read_to_string_empty() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_util_empty.txt");

        std::fs::write(&test_file, "").expect("Failed to write test file");

        let result = safe_read_to_string(&test_file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn test_safe_read_to_string_multiline() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_util_multiline.txt");
        let content = "line 1\nline 2\nline 3";

        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = safe_read_to_string(&test_file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);

        std::fs::remove_file(&test_file).ok();
    }

    // ==================== JSON Operation Tests ====================

    #[test]
    fn test_safe_json_parse_simple() {
        let json_str = r#"{"key": "value"}"#;
        let result: Result<serde_json::Value> = safe_json_parse(json_str);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["key"], "value");
    }

    #[test]
    fn test_safe_json_parse_array() {
        let json_str = "[1, 2, 3, 4, 5]";
        let result: Result<serde_json::Value> = safe_json_parse(json_str);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value[0], 1);
        assert_eq!(value[4], 5);
    }

    #[test]
    fn test_safe_json_parse_nested() {
        let json_str = r#"{"outer": {"inner": "value"}}"#;
        let result: Result<serde_json::Value> = safe_json_parse(json_str);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["outer"]["inner"], "value");
    }

    #[test]
    fn test_safe_json_parse_invalid() {
        let invalid_json = "{ invalid json }";
        let result: Result<serde_json::Value> = safe_json_parse(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_json_parse_empty() {
        let empty_json = "";
        let result: Result<serde_json::Value> = safe_json_parse(empty_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_json_parse_malformed() {
        let malformed = r#"{"key": value}"#; // Missing quotes around value
        let result: Result<serde_json::Value> = safe_json_parse(malformed);
        assert!(result.is_err());
    }

    // ==================== Mutex Operation Tests ====================

    #[test]
    fn test_safe_lock_success() {
        let mutex = Mutex::new(42);
        let result = safe_lock(&mutex);
        assert!(result.is_ok());
        assert_eq!(*result.unwrap(), 42);
    }

    #[test]
    fn test_safe_lock_multiple() {
        let mutex = Mutex::new(vec![1, 2, 3]);
        {
            let result = safe_lock(&mutex);
            assert!(result.is_ok());
            let mut guard = result.unwrap();
            guard.push(4);
        }
        {
            let result = safe_lock(&mutex);
            assert!(result.is_ok());
            let guard = result.unwrap();
            assert_eq!(guard.len(), 4);
        }
    }

    #[test]
    fn test_safe_lock_string() {
        let mutex = Mutex::new(String::from("test"));
        let result = safe_lock(&mutex);
        assert!(result.is_ok());
        assert_eq!(*result.unwrap(), "test");
    }

    // ==================== Channel Operation Tests ====================

    #[test]
    fn test_safe_send_success() {
        let (tx, rx) = mpsc::channel();
        let result = safe_send(&tx, 42);
        assert!(result.is_ok());
        assert_eq!(rx.recv().unwrap(), 42);
    }

    #[test]
    fn test_safe_send_string() {
        let (tx, rx) = mpsc::channel();
        let result = safe_send(&tx, String::from("message"));
        assert!(result.is_ok());
        assert_eq!(rx.recv().unwrap(), "message");
    }

    #[test]
    fn test_safe_send_disconnected() {
        let (tx, rx) = mpsc::channel();
        drop(rx); // Disconnect receiver
        let result = safe_send(&tx, 42);
        assert!(result.is_err());
    }

    // ==================== Error Constructor Tests ====================

    #[test]
    fn test_storage_error_constructor() {
        let err = storage_error("Storage operation failed");
        assert!(format!("{:?}", err).contains("Storage"));
    }

    #[test]
    fn test_configuration_error_constructor() {
        let err = configuration_error("Invalid configuration");
        assert!(format!("{:?}", err).contains("Configuration"));
    }

    #[test]
    fn test_validation_error_constructor() {
        let err = validation_error("Validation failed");
        assert!(format!("{:?}", err).contains("Validation"));
    }

    #[test]
    fn test_internal_error_constructor() {
        let err = internal("Critical failure", "auth_service");
        assert!(format!("{:?}", err).contains("Internal"));
    }

    #[test]
    fn test_error_constructor_empty_message() {
        let err = storage_error("");
        let debug_str = format!("{:?}", err);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_error_constructor_long_message() {
        let long_msg = "a".repeat(1000);
        let err = validation_error(&long_msg);
        let debug_str = format!("{:?}", err);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_error_constructor_special_characters() {
        let err = configuration_error("Error with \n newlines \t and \r special chars");
        let debug_str = format!("{:?}", err);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_internal_error_with_component() {
        let err = internal("Failed", "test_component");
        let debug_str = format!("{:?}", err);
        assert!(!debug_str.is_empty());
    }

    // ==================== Integration Tests ====================

    #[test]
    fn test_error_chain_with_constructors() {
        /// Operation
        fn operation() -> Result<String> {
            Err(storage_error("Initial error"))
        }

        let result = operation();
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_error_types() {
        let errors = vec![
            storage_error("storage"),
            configuration_error("config"),
            validation_error("validation"),
            internal("internal", "component"),
        ];

        for err in errors {
            assert!(!format!("{:?}", err).is_empty());
        }
    }

    #[test]
    fn test_file_and_json_integration() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_util_json.json");
        let json_content = r#"{"name": "test", "value": 42}"#;

        std::fs::write(&test_file, json_content).expect("Failed to write");

        let read_result = safe_read_to_string(&test_file);
        assert!(read_result.is_ok());

        let json_str = read_result.unwrap();
        let parse_result: Result<serde_json::Value> = safe_json_parse(&json_str);
        assert!(parse_result.is_ok());

        let value = parse_result.unwrap();
        assert_eq!(value["name"], "test");
        assert_eq!(value["value"], 42);

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn test_env_var_and_to_string_integration() {
        std::env::set_var("TEST_INT_VAR", "12345");

        let env_result = safe_env_var("TEST_INT_VAR");
        assert!(env_result.is_ok());

        let value = env_result.unwrap();
        assert_eq!(value, "12345");

        std::env::remove_var("TEST_INT_VAR");
    }

    #[test]
    fn test_mutex_with_error_recovery() {
        let mutex = Mutex::new(0);

        {
            let lock_result = safe_lock(&mutex);
            assert!(lock_result.is_ok());
            let mut guard = lock_result.unwrap();
            *guard = 42;
        }

        {
            let lock_result = safe_lock(&mutex);
            assert!(lock_result.is_ok());
            assert_eq!(*lock_result.unwrap(), 42);
        }
    }
}
