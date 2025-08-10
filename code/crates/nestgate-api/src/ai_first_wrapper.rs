//! AI-First API endpoint wrappers
//! Converts existing APIs to AI-First format
//!
//! This module provides conversion utilities to transform traditional API responses
//! into AI-First format that complies with the ecoPrimals ecosystem standards.

use nestgate_core::ai_first::*;
use nestgate_core::NestGateError;
use std::collections::HashMap;
use std::time::Instant;
use uuid::Uuid;

/// Wrapper to convert any result to AI-First format
/// 
/// This is the primary function for converting traditional Rust Results
/// into AI-First responses that AI agents can easily consume.
pub fn to_ai_first_response<T, E>(
    result: Result<T, E>,
    operation_type: &str,
    start_time: Instant,
    request_id: Uuid,
) -> AIFirstResponse<T>
where
    T: serde::Serialize + Clone,
    E: std::fmt::Display + std::fmt::Debug,
{
    let processing_time_ms = start_time.elapsed().as_millis() as u64;
    
    match result {
        Ok(data) => {
            let confidence_score = calculate_confidence_score(operation_type, true);
            let suggested_actions = generate_success_actions(operation_type);
            let ai_metadata = create_success_metadata(operation_type);
            
            AIFirstResponse::success(data, request_id, processing_time_ms, confidence_score)
                .with_suggested_actions(suggested_actions)
                .with_ai_metadata(ai_metadata)
        }
        Err(error) => {
            let ai_error = convert_to_ai_first_error(error, operation_type);
            let confidence_score = calculate_confidence_score(operation_type, false);
            let suggested_actions = generate_error_actions(operation_type);
            let ai_metadata = create_error_metadata(operation_type);
            
            // Use serde_json::Value as placeholder for failed operations
            let null_data: T = serde_json::from_value(serde_json::Value::Null)
                .unwrap_or_else(|_| return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    "Cannot create null value for type".to_string()
).into()));
            
            AIFirstResponse::error(ai_error, request_id, processing_time_ms, confidence_score)
                .with_suggested_actions(suggested_actions)
                .with_ai_metadata(ai_metadata)
        }
    }
}

/// Calculate confidence score based on operation type and success
fn calculate_confidence_score(operation_type: &str, success: bool) -> f64 {
    match (operation_type, success) {
        ("zfs_pool_creation", true) => 0.95,     // High confidence in ZFS pool ops
        ("zfs_dataset_creation", true) => 0.93,  // High confidence in dataset ops
        ("zfs_snapshot", true) => 0.92,          // High confidence in snapshots
        ("network_discovery", true) => 0.85,     // Medium-high confidence
        ("service_registration", true) => 0.88,  // Good confidence in service ops
        ("ai_prediction", true) => 0.75,         // Medium confidence for AI
        ("system_health_check", true) => 0.90,   // High confidence in health checks
        (_, false) => 0.3,                       // Low confidence on errors
        _ => 0.8,                                // Default confidence
    }
}

/// Generate suggested actions for successful operations
fn generate_success_actions(operation_type: &str) -> Vec<SuggestedAction> {
    match operation_type {
        "zfs_pool_creation" => vec![
            SuggestedAction {
                action_type: "create_datasets".to_string(),
                description: "Create initial datasets for organization".to_string(),
                parameters: create_dataset_parameters(),
                confidence: 0.9,
                estimated_duration_ms: 5000,
            },
            SuggestedAction {
                action_type: "enable_compression".to_string(),
                description: "Enable compression for space efficiency".to_string(),
                parameters: create_compression_parameters(),
                confidence: 0.85,
                estimated_duration_ms: 2000,
            },
        ],
        "zfs_dataset_creation" => vec![
            SuggestedAction {
                action_type: "set_quotas".to_string(),
                description: "Set appropriate quotas to prevent overuse".to_string(),
                parameters: create_quota_parameters(),
                confidence: 0.8,
                estimated_duration_ms: 1000,
            },
        ],
        "zfs_snapshot" => vec![
            SuggestedAction {
                action_type: "schedule_cleanup".to_string(),
                description: "Schedule automatic snapshot cleanup".to_string(),
                parameters: create_cleanup_parameters(),
                confidence: 0.75,
                estimated_duration_ms: 3000,
            },
        ],
        "storage_optimization" => vec![
            SuggestedAction {
                action_type: "schedule_scrub".to_string(),
                description: "Schedule regular pool scrubbing".to_string(),
                parameters: create_scrub_parameters(),
                confidence: 0.85,
                estimated_duration_ms: 2000,
            },
        ],
        "network_discovery" => vec![
            SuggestedAction {
                action_type: "validate_services".to_string(),
                description: "Validate discovered services are healthy".to_string(),
                parameters: HashMap::new(),
                confidence: 0.82,
                estimated_duration_ms: 4000,
            },
        ],
        _ => vec![]
    }
}

/// Generate suggested actions for error recovery
fn generate_error_actions(operation_type: &str) -> Vec<SuggestedAction> {
    match operation_type {
        "zfs_pool_creation" => vec![
            SuggestedAction {
                action_type: "check_disk_space".to_string(),
                description: "Verify sufficient disk space is available".to_string(),
                parameters: HashMap::new(),
                confidence: 0.9,
                estimated_duration_ms: 1000,
            },
            SuggestedAction {
                action_type: "validate_devices".to_string(),
                description: "Check that all specified devices exist".to_string(),
                parameters: HashMap::new(),
                confidence: 0.95,
                estimated_duration_ms: 2000,
            },
        ],
        "network_discovery" => vec![
            SuggestedAction {
                action_type: "retry_discovery".to_string(),
                description: "Retry service discovery with backoff".to_string(),
                parameters: HashMap::from([
                    ("delay_seconds".to_string(), serde_json::Value::Number(serde_json::Number::from(5))),
                ]),
                confidence: 0.7,
                estimated_duration_ms: 8000,
            },
        ],
        _ => vec![
            SuggestedAction {
                action_type: "check_logs".to_string(),
                description: "Review system logs for more details".to_string(),
                parameters: HashMap::new(),
                confidence: 0.6,
                estimated_duration_ms: 2000,
            },
        ]
    }
}

/// Convert NestGateError to AI-First format
fn convert_to_ai_first_error<E: std::fmt::Display + std::fmt::Debug>(
    error: E,
    operation_type: &str
) -> AIFirstError {
    let error_str = format!("{}", error);
    let error_debug = format!("{:?}", error);
    
    // Analyze error content to determine category and response
    if error_str.contains("timeout") || error_str.contains("network") {
        AIFirstError::transient(
            generate_error_code(&error_str, operation_type),
            error_str,
        )
    } else if error_str.contains("config") || error_str.contains("permission") {
        AIFirstError::configuration(
            generate_error_code(&error_str, operation_type),
            error_str,
        )
    } else if error_str.contains("critical") || error_str.contains("fatal") {
        AIFirstError::critical(
            generate_error_code(&error_str, operation_type),
            error_str,
        )
    } else {
        // Default to transient for unknown errors
        let mut error = AIFirstError::transient(
            generate_error_code(&error_str, operation_type),
            error_str,
        );
        
        // Add debug information to context
        error.context.insert(
            "debug_info".to_string(),
            serde_json::Value::String(error_debug),
        );
        error.context.insert(
            "operation_type".to_string(),
            serde_json::Value::String(operation_type.to_string()),
        );
        
        error
    }
}

/// Generate machine-readable error codes
fn generate_error_code(error_message: &str, operation_type: &str) -> String {
    let operation_prefix = operation_type.to_uppercase().replace(' ', "_");
    
    if error_message.contains("timeout") {
        format!("{}_TIMEOUT", operation_prefix)
    } else if error_message.contains("permission") {
        format!("{}_PERMISSION_DENIED", operation_prefix)
    } else if error_message.contains("not found") {
        format!("{}_NOT_FOUND", operation_prefix)
    } else if error_message.contains("already exists") {
        format!("{}_ALREADY_EXISTS", operation_prefix)
    } else if error_message.contains("insufficient") || error_message.contains("space") {
        format!("{}_INSUFFICIENT_RESOURCES", operation_prefix)
    } else if error_message.contains("network") {
        format!("{}_NETWORK_ERROR", operation_prefix)
    } else if error_message.contains("config") {
        format!("{}_CONFIGURATION_ERROR", operation_prefix)
    } else {
        format!("{}_UNKNOWN_ERROR", operation_prefix)
    }
}

/// Create success metadata for AI optimization
fn create_success_metadata(operation_type: &str) -> AIResponseMetadata {
    AIResponseMetadata {
        operation_type: operation_type.to_string(),
        complexity_score: calculate_complexity_score(operation_type),
        resource_usage: estimate_resource_usage(operation_type, true),
        performance_hints: generate_performance_hints(operation_type, true),
        optimization_opportunities: generate_optimization_opportunities(operation_type),
    }
}

/// Create error metadata for AI analysis
fn create_error_metadata(operation_type: &str) -> AIResponseMetadata {
    AIResponseMetadata {
        operation_type: operation_type.to_string(),
        complexity_score: calculate_complexity_score(operation_type),
        resource_usage: estimate_resource_usage(operation_type, false),
        performance_hints: generate_performance_hints(operation_type, false),
        optimization_opportunities: vec![
            "Implement error recovery patterns".to_string(),
            "Add monitoring and alerting".to_string(),
        ],
    }
}

/// Calculate operation complexity for AI planning
fn calculate_complexity_score(operation_type: &str) -> f64 {
    match operation_type {
        "zfs_pool_creation" => 0.8,      // High complexity
        "zfs_snapshot" => 0.4,           // Medium-low complexity
        "network_discovery" => 0.6,      // Medium complexity
        "service_registration" => 0.5,   // Medium complexity
        "system_health_check" => 0.3,    // Low complexity
        _ => 0.5,                        // Default medium
    }
}

/// Estimate resource usage for AI optimization
fn estimate_resource_usage(operation_type: &str, success: bool) -> ResourceUsage {
    let base_multiplier = if success { 1.0 } else { 0.3 }; // Errors use fewer resources
    
    match operation_type {
        "zfs_pool_creation" => ResourceUsage {
            cpu_time_ms: (5000.0 * base_multiplier) as u64,
            memory_bytes: (50_000_000.0 * base_multiplier) as u64,
            disk_io_bytes: (100_000_000.0 * base_multiplier) as u64,
            network_io_bytes: 0,
        },
        "zfs_snapshot" => ResourceUsage {
            cpu_time_ms: (1000.0 * base_multiplier) as u64,
            memory_bytes: (10_000_000.0 * base_multiplier) as u64,
            disk_io_bytes: (20_000_000.0 * base_multiplier) as u64,
            network_io_bytes: 0,
        },
        "network_discovery" => ResourceUsage {
            cpu_time_ms: (2000.0 * base_multiplier) as u64,
            memory_bytes: (5_000_000.0 * base_multiplier) as u64,
            disk_io_bytes: 0,
            network_io_bytes: (1_000_000.0 * base_multiplier) as u64,
        },
        _ => ResourceUsage::default(),
    }
}

/// Generate performance hints for AI optimization
fn generate_performance_hints(operation_type: &str, success: bool) -> Vec<String> {
    let mut hints = vec![];
    
    match operation_type {
        "zfs_pool_creation" => {
            hints.push("Consider enabling compression for better space utilization".to_string());
            hints.push("Use RAID-Z for redundancy in production".to_string());
            if success {
                hints.push("Pool created successfully, consider setting up monitoring".to_string());
            }
        },
        "zfs_snapshot" => {
            hints.push("Snapshots are instant and space-efficient".to_string());
            hints.push("Consider automated snapshot scheduling".to_string());
        },
        "network_discovery" => {
            hints.push("Discovery can be cached to improve performance".to_string());
            hints.push("Consider service mesh for better service discovery".to_string());
        },
        _ => {
            hints.push("Monitor resource usage for optimization opportunities".to_string());
        }
    }
    
    if !success {
        hints.push("Error occurred - consider retry strategies".to_string());
        hints.push("Check system logs for detailed error information".to_string());
    }
    
    hints
}

/// Generate optimization opportunities for AI learning
fn generate_optimization_opportunities(operation_type: &str) -> Vec<String> {
    match operation_type {
        "zfs_pool_creation" => vec![
            "Enable background compression for space savings".to_string(),
            "Configure automatic scrubbing schedule".to_string(),
            "Set up ZFS monitoring and alerting".to_string(),
        ],
        "zfs_snapshot" => vec![
            "Implement automated snapshot retention policies".to_string(),
            "Consider incremental backups to remote storage".to_string(),
        ],
        "network_discovery" => vec![
            "Implement service discovery caching".to_string(),
            "Add health check integration".to_string(),
        ],
        _ => vec![
            "Add comprehensive monitoring".to_string(),
            "Implement automated recovery procedures".to_string(),
        ],
    }
}

// Helper functions for creating action parameters

fn create_dataset_parameters() -> HashMap<String, serde_json::Value> {
    HashMap::from([
        ("compression".to_string(), serde_json::Value::String("lz4".to_string())),
        ("recordsize".to_string(), serde_json::Value::String("128K".to_string())),
    ])
}

fn create_compression_parameters() -> HashMap<String, serde_json::Value> {
    HashMap::from([
        ("algorithm".to_string(), serde_json::Value::String("lz4".to_string())),
        ("level".to_string(), serde_json::Value::Number(serde_json::Number::from(1))),
    ])
}

fn create_quota_parameters() -> HashMap<String, serde_json::Value> {
    HashMap::from([
        ("default_quota".to_string(), serde_json::Value::String("10G".to_string())),
        ("warn_at".to_string(), serde_json::Value::String("8G".to_string())),
    ])
}

fn create_cleanup_parameters() -> HashMap<String, serde_json::Value> {
    HashMap::from([
        ("retention_days".to_string(), serde_json::Value::Number(serde_json::Number::from(30))),
        ("frequency".to_string(), serde_json::Value::String("daily".to_string())),
    ])
}

fn create_scrub_parameters() -> HashMap<String, serde_json::Value> {
    HashMap::from([
        ("frequency".to_string(), serde_json::Value::String("monthly".to_string())),
        ("priority".to_string(), serde_json::Value::String("low".to_string())),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_successful_response_conversion() {
        let start_time = Instant::now();
        let request_id = Uuid::new_v4();
        let result: Result<String, &str> = Ok("success".to_string());
        
        let response = to_ai_first_response(
            result,
            "zfs_pool_creation",
            start_time,
            request_id,
        );
        
        assert_eq!(response.success, true);
        assert_eq!(response.data, "success");
        assert!(response.confidence_score >= 0.9);
        assert!(!response.suggested_actions.is_empty());
    }
    
    #[test]
    fn test_error_response_conversion() {
        let start_time = Instant::now();
        let request_id = Uuid::new_v4();
        let result: Result<String, &str> = Err("timeout occurred");
        
        let response = to_ai_first_response(
            result,
            "network_discovery", 
            start_time,
            request_id,
        );
        
        assert_eq!(response.success, false);
        assert!(response.error.is_some());
        assert_eq!(response.confidence_score, 0.3);
        
        let error = response.error.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())
});
        assert_eq!(error.code, "NETWORK_DISCOVERY_TIMEOUT");
        assert!(matches!(error.category, AIErrorCategory::Transient));
    }
    
    #[test]
    fn test_confidence_score_calculation() {
        assert_eq!(calculate_confidence_score("zfs_pool_creation", true), 0.95);
        assert_eq!(calculate_confidence_score("any_operation", false), 0.3);
        assert_eq!(calculate_confidence_score("unknown_operation", true), 0.8);
    }
    
    #[test]
    fn test_error_code_generation() {
        assert_eq!(
            generate_error_code("timeout occurred", "zfs_operation"),
            "ZFS_OPERATION_TIMEOUT"
        );
        assert_eq!(
            generate_error_code("permission denied", "file_access"),
            "FILE_ACCESS_PERMISSION_DENIED"
        );
    }
} 