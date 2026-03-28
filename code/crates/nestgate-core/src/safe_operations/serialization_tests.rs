// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Tests for safe serialization operations
//! Validates JSON/MessagePack safety, error handling, and edge cases

use super::serialization::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TestData {
    name: String,
    value: i32,
    optional: Option<String>,
}

#[test]
fn test_safe_serialize_json_simple() {
    let data = TestData {
        name: "test".to_string(),
        value: 42,
        optional: None,
    };
    
    let result = safe_serialize_json(&data);
    assert!(result.is_ok());
    
    let json = result.unwrap();
    assert!(json.contains("test"));
    assert!(json.contains("42"));
}

#[test]
fn test_safe_deserialize_json_simple() {
    let json = r#"{"name":"test","value":42,"optional":null}"#;
    
    let result: Result<TestData, _> = safe_deserialize_json(json);
    assert!(result.is_ok());
    
    let data = result.unwrap();
    assert_eq!(data.name, "test");
    assert_eq!(data.value, 42);
    assert_eq!(data.optional, None);
}

#[test]
fn test_safe_deserialize_json_invalid() {
    let json = r#"{"invalid json"#;
    
    let result: Result<TestData, _> = safe_deserialize_json(json);
    assert!(result.is_err());
}

#[test]
fn test_safe_deserialize_json_wrong_type() {
    let json = r#"{"name":"test","value":"not a number","optional":null}"#;
    
    let result: Result<TestData, _> = safe_deserialize_json(json);
    assert!(result.is_err());
}

#[test]
fn test_safe_serialize_json_empty() {
    let data = TestData {
        name: String::new(),
        value: 0,
        optional: None,
    };
    
    let result = safe_serialize_json(&data);
    assert!(result.is_ok());
}

#[test]
fn test_safe_serialize_json_special_chars() {
    let data = TestData {
        name: "test\n\t\"special\"".to_string(),
        value: 42,
        optional: Some("unicode: 世界 🌍".to_string()),
    };
    
    let result = safe_serialize_json(&data);
    assert!(result.is_ok());
    
    // Verify round-trip
    let json = result.unwrap();
    let result2: Result<TestData, _> = safe_deserialize_json(&json);
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), data);
}

#[test]
fn test_safe_json_round_trip() {
    let original = TestData {
        name: "round trip test".to_string(),
        value: 123,
        optional: Some("data".to_string()),
    };
    
    let json = safe_serialize_json(&original).unwrap();
    let deserialized: TestData = safe_deserialize_json(&json).unwrap();
    
    assert_eq!(original, deserialized);
}

#[test]
fn test_safe_serialize_nested_data() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Nested {
        inner: TestData,
        list: Vec<i32>,
    }
    
    let data = Nested {
        inner: TestData {
            name: "nested".to_string(),
            value: 1,
            optional: None,
        },
        list: vec![1, 2, 3, 4, 5],
    };
    
    let json = safe_serialize_json(&data).unwrap();
    let deserialized: Nested = safe_deserialize_json(&json).unwrap();
    
    assert_eq!(data, deserialized);
}

#[test]
fn test_safe_serialize_large_data() {
    #[derive(Serialize, Deserialize)]
    struct LargeData {
        items: Vec<i32>,
    }
    
    let data = LargeData {
        items: (0..10_000).collect(),
    };
    
    let result = safe_serialize_json(&data);
    assert!(result.is_ok());
}

#[test]
fn test_safe_deserialize_empty_string() {
    let json = "";
    
    let result: Result<TestData, _> = safe_deserialize_json(json);
    assert!(result.is_err());
}

#[test]
fn test_safe_serialize_pretty() {
    let data = TestData {
        name: "pretty".to_string(),
        value: 42,
        optional: None,
    };
    
    let result = safe_serialize_json_pretty(&data);
    assert!(result.is_ok());
    
    let json = result.unwrap();
    assert!(json.contains('\n')); // Pretty format includes newlines
}

#[test]
fn test_safe_serialize_msgpack() {
    let data = TestData {
        name: "msgpack test".to_string(),
        value: 42,
        optional: Some("data".to_string()),
    };
    
    let result = safe_serialize_msgpack(&data);
    assert!(result.is_ok());
}

#[test]
fn test_safe_deserialize_msgpack() {
    let data = TestData {
        name: "msgpack".to_string(),
        value: 123,
        optional: None,
    };
    
    let bytes = safe_serialize_msgpack(&data).unwrap();
    let result: Result<TestData, _> = safe_deserialize_msgpack(&bytes);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), data);
}

#[test]
fn test_safe_msgpack_round_trip() {
    let original = TestData {
        name: "msgpack round trip".to_string(),
        value: 456,
        optional: Some("optional data".to_string()),
    };
    
    let bytes = safe_serialize_msgpack(&original).unwrap();
    let deserialized: TestData = safe_deserialize_msgpack(&bytes).unwrap();
    
    assert_eq!(original, deserialized);
}

#[test]
fn test_safe_deserialize_msgpack_invalid() {
    let invalid_bytes = vec![0xFF, 0xFF, 0xFF, 0xFF];
    
    let result: Result<TestData, _> = safe_deserialize_msgpack(&invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_serialization_error_messages() {
    let invalid_json = "{invalid}";
    
    let result: Result<TestData, _> = safe_deserialize_json(invalid_json);
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert!(!error.to_string().is_empty());
}

#[test]
fn test_concurrent_serialization() {
    use std::thread;
    
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                let data = TestData {
                    name: format!("thread-{}", i),
                    value: i,
                    optional: None,
                };
                safe_serialize_json(&data)
            })
        })
        .collect();
    
    for handle in handles {
        assert!(handle.join().unwrap().is_ok());
    }
}

#[test]
fn test_json_vs_msgpack_size() {
    let data = TestData {
        name: "comparison".to_string(),
        value: 42,
        optional: Some("data".to_string()),
    };
    
    let json = safe_serialize_json(&data).unwrap();
    let msgpack = safe_serialize_msgpack(&data).unwrap();
    
    // MessagePack should generally be more compact
    println!("JSON size: {}, MessagePack size: {}", json.len(), msgpack.len());
}

