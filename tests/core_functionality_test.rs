//! Core functionality tests for NestGate
//! Tests the core functionality that compiles and works correctly

use nestgate_core::{
    canonical_modernization::unified_enums::{UnifiedHealthStatus, UnifiedServiceType},
    error::{NestGateError, Result},
    traits::CanonicalConfig,
    traits::{UniversalResponseStatus, UniversalServiceRequest, UniversalServiceResponse},
};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tokio::time::sleep;

#[tokio::test]
async fn test_canonical_config_creation() -> Result<()> {
    println!("🧪 Testing canonical configuration creation");

    // Test default configuration
    let config = CanonicalConfig::default();
    println!("✅ Created default canonical config");

    Ok(())
}

#[tokio::test]
async fn test_unified_enums() -> Result<()> {
    println!("🧪 Testing unified enum system");

    // Test service types
    let storage_service = UnifiedServiceType::Storage;
    let network_service = UnifiedServiceType::Network;
    let security_service = UnifiedServiceType::Security;

    println!("✅ Service types created successfully");

    // Test health status
    let healthy = UnifiedHealthStatus::Healthy;
    let unhealthy = UnifiedHealthStatus::Unhealthy;

    println!("✅ Health status enums created successfully");

    Ok(())
}

#[tokio::test]
async fn test_error_system() -> Result<()> {
    println!("🧪 Testing unified error system");

    // Test creating different types of errors
    let internal_error = NestGateError::internal_error(
        "Test internal error".to_string(),
        "test_context".to_string(),
    );

    println!("✅ Created internal error");

    // Test error conversion
    let result: Result<String> = Err(internal_error);
    match result {
        Ok(_) => unreachable!(),
        Err(_) => {
            println!("✅ Error handling works correctly");
    Ok(())
        }
    Ok(())
    }

    Ok(())
}

#[tokio::test]
async fn test_service_request_response() -> Result<()> {
    println!("🧪 Testing service request/response system");

    // Test creating service request
    let mut parameters = HashMap::new();
    parameters.insert(
        "operation".to_string(),
        serde_json::Value::String("test".to_string()),
    );

    let mut metadata = HashMap::new();
    metadata.insert("source".to_string(), "test_client".to_string());

    let request = UniversalServiceRequest {
        request_id: "test_request_123".to_string(),
        operation: "test_operation".to_string(),
        parameters,
        metadata: metadata.clone(),
    };

    println!("✅ Created service request: {}", request.request_id);

    // Test creating service response
    let response = UniversalServiceResponse {
        request_id: request.request_id.clone(),
        status: UniversalResponseStatus::Success,
        data: Some(serde_json::Value::String("test_result".to_string())),
        error: None,
        metadata,
    };

    println!(
        "✅ Created service response with status: {:?}",
        response.status
    );

    Ok(())
}

#[tokio::test]
async fn test_async_operations() -> Result<()> {
    println!("🧪 Testing async operations");

    // Test async sleep
    let start = SystemTime::now();
    sleep(Duration::from_millis(50)).await;
    let elapsed = start.elapsed()?;

    assert!(
        elapsed >= Duration::from_millis(40),
        "Sleep should take at least 40ms"
    );
    println!("✅ Async sleep worked: {elapsed:?}");

    // Test async error handling
    async fn async_operation_that_fails() -> Result<String> {
        Err(NestGateError::internal_error(
            "Async operation failed".to_string(),
            "test".to_string(),
        ))
    }

    match async_operation_that_fails().await {
        Ok(_) => unreachable!(),
        Err(_) => {
            println!("✅ Async error handling works");
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_concurrent_operations() -> Result<()> {
    println!("🧪 Testing concurrent operations");

    let mut handles = Vec::new();

    // Spawn multiple concurrent tasks
    for i in 0..5 {
        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(10 + i * 5)).await;
            format!("Task {i} completed")
        });
        handles.push(handle);
        Ok(())
    }

    // Wait for all tasks to complete
    let mut results = Vec::new();
    for handle in handles {
        let result = handle.await.map_err(|e| {
            NestGateError::internal_error(
                format!("Task join failed: {e}"),
                "test_concurrent_operations".to_string(),
            )
        })?;
        results.push(result);
        Ok(())
    }

    assert_eq!(results.len(), 5, "All tasks should complete");
    println!(
        "✅ Concurrent operations completed: {} tasks",
        results.len()
    );

    Ok(())
}

#[tokio::test]
async fn test_json_serialization() -> Result<()> {
    println!("🧪 Testing JSON serialization");

    // Test service request serialization
    let mut parameters = HashMap::new();
    parameters.insert(
        "test_param".to_string(),
        serde_json::Value::String("test_value".to_string()),
    );

    let mut metadata = HashMap::new();
    metadata.insert("client".to_string(), "test_client".to_string());

    let request = UniversalServiceRequest {
        request_id: "test_123".to_string(),
        operation: "test_op".to_string(),
        parameters,
        metadata,
    };

    // Serialize to JSON
    let json_str = serde_json::to_string(&request).map_err(|e| {
        NestGateError::internal_error(
            format!("JSON serialization failed: {e}"),
            "test_json_serialization".to_string(),
        )
    })?;

    println!("✅ Request serialized to JSON: {} bytes", json_str.len());

    // Deserialize from JSON
    let deserialized: UniversalServiceRequest = serde_json::from_str(&json_str).map_err(|e| {
        NestGateError::internal_error(
            format!("JSON deserialization failed: {e}"),
            "test_json_serialization".to_string(),
        )
    })?;

    assert_eq!(deserialized.request_id, "test_123");
    assert_eq!(deserialized.operation, "test_op");

    println!("✅ Request deserialized successfully");

    Ok(())
}

#[tokio::test]
async fn test_memory_operations() -> Result<()> {
    println!("🧪 Testing memory operations");

    // Test creating and manipulating data structures
    let mut data_map: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..100 {
        let key = format!("key_{i}");
        let values = vec![
            format!("value_{}_a", i),
            format!("value_{}_b", i),
            format!("value_{}_c", i),
        ];
        data_map.insert(key, values);
        Ok(())
    }

    assert_eq!(data_map.len(), 100);

    // Test accessing data
    let key_50 = "key_50".to_string();
    if let Some(values) = data_map.get(&key_50) {
        assert_eq!(values.len(), 3);
        assert_eq!(values[0], "value_50_a");
    } else {
        return Err(NestGateError::internal_error(
            "Expected key not found".to_string(),
            "test_memory_operations".to_string(),
        ));
        Ok(())
    }

    println!("✅ Memory operations completed successfully");

    Ok(())
}
