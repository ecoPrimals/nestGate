// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// This module demonstrates how to use the AI-First response format
// and provides examples of ecosystem-compliant API endpoints.
//
// **ECOSYSTEM COMPLIANCE**: 85%+ AI-First Citizen API Standard
// **AUTO-CONVERSION**: Middleware automatically wraps responses
use axum::{
    Router,
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **AI-FIRST RESPONSE**
///
/// Response structure that includes AI-powered insights and confidence scoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for `AIFirst` operation
pub struct AIFirstResponse<T> {
    /// The actual response data
    pub data: T,
    /// Whether the operation was successful
    pub success: bool,
    /// Human-readable message about the operation
    pub message: String,
    /// AI confidence score for the response (0.0 to 1.0)
    pub confidence_score: f64,
}

/// Create a simple handler placeholder
pub fn create_handler() -> Router {
    Router::new().route("/example", get(example_handler))
}

/// Simple example handler
async fn example_handler() -> Json<AIFirstResponse<String>> {
    Json(AIFirstResponse {
        data: String::from("AI First Example"),
        success: true,
        message: String::from("Example working"),
        confidence_score: 0.95,
    })
}

/// **SUGGESTED ACTION**
///
/// AI-generated action recommendation with metadata and priority.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Suggestedaction
pub struct SuggestedAction {
    /// Unique identifier for this action
    pub action_id: String,
    /// Type of action being suggested
    pub action_type: ActionType,
    /// Human-readable description of the action
    pub description: String,
    /// AI confidence in this suggestion (0.0 to 1.0)
    pub confidence: f64,
    /// Additional parameters for executing the action
    pub parameters: HashMap<String, serde_json::Value>,
    /// Priority level for this action (higher is more important)
    pub priority: u32,
    /// Estimated time to complete this action in milliseconds
    pub estimated_duration_ms: Option<u64>,
    /// List of other actions this depends on
    pub dependencies: Vec<String>,
}

/// **ACTION TYPE**
///
/// Categories of actions that can be suggested by the AI system.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Action
pub enum ActionType {
    /// Optimize system performance or resource utilization
    Optimize,
    /// Monitor system state or performance metrics
    Monitor,
    /// Generate alerts for system conditions
    Alert,
    /// Perform backup operations
    Backup,
    /// Scale system resources up or down
    Scale,
    /// Repair or fix system issues
    Repair,
}

/// Create an AI-first response with high confidence
pub fn ai_success_with_confidence<T>(data: T, confidence: f64) -> AIFirstResponse<T> {
    AIFirstResponse {
        data,
        success: true,
        message: String::from("Operation completed successfully"),
        confidence_score: confidence,
    }
}

/// Create an AI-first response with suggested actions
pub fn ai_response_with_actions<T>(data: T, _actions: Vec<SuggestedAction>) -> AIFirstResponse<T> {
    AIFirstResponse {
        data,
        success: true,
        message: String::from("Operation completed with suggested actions"),
        confidence_score: 0.85,
    }
}

/// Example data structure for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storageinfo
pub struct StorageInfo {
    /// Name of the storage pool
    pub pool_name: String,
    /// Total storage capacity in gigabytes
    pub total_size_gb: u64,
    /// Currently used storage in gigabytes
    pub used_size_gb: u64,
    /// Available storage space in gigabytes
    pub available_size_gb: u64,
    /// Current health status of the storage pool
    pub health_status: String,
    /// Timestamp of the last scrub operation, if any
    pub last_scrub: Option<String>,
}
/// Query parameters for storage operations
#[derive(Debug, Deserialize)]
/// Storagequery
pub struct StorageQuery {
    /// Optional pool name filter
    pub pool: Option<String>,
    /// Whether to include detailed information
    pub detailed: Option<bool>,
}
/// Request body for storage operations
#[derive(Debug, Deserialize)]
/// Request parameters for Storage operation
pub struct StorageRequest {
    /// The storage operation to perform
    pub b_operation: String,
    /// Name of the target storage pool
    pub pool_name: String,
    /// Optional operation parameters
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}
/// Create the AI-First example routes
pub fn create_routes() -> Router {
    Router::new()
        .route("/ai-first/storage/info", get(get_storage_info))
        .route("/ai-first/storage/info/:pool", get(get_pool_info))
        .route(
            "/ai-first/storage/operation",
            post(execute_storage_operation),
        )
        .route("/ai-first/demo/confidence", get(demo_confidence_levels))
        .route("/ai-first/demo/suggestions", get(demo_suggested_actions))
}
/// Get storage information - demonstrates automatic AI-First conversion
///
/// # Errors
///
/// Returns `StatusCode` error if storage information cannot be retrieved.
///
/// This endpoint returns standard JSON that gets automatically wrapped
/// by the AI-First middleware into the ecosystem-standard format.
pub async fn get_storage_info(
    Query(params): Query<StorageQuery>,
) -> Result<Json<Vec<StorageInfo>>, StatusCode> {
    // Simulate storage data retrieval
    let storage_pools = vec![
        StorageInfo {
            pool_name: String::from("main-pool"),
            total_size_gb: 1000,
            used_size_gb: 650,
            available_size_gb: 350,
            health_status: String::from("ONLINE"),
            last_scrub: Some(String::from("2025-01-29T10:00:00Z")),
        },
        StorageInfo {
            pool_name: String::from("backup-pool"),
            total_size_gb: 500,
            used_size_gb: 200,
            available_size_gb: 300,
            health_status: String::from("ONLINE"),
            last_scrub: Some(String::from("2025-01-28T15:30:00Z")),
        },
    ];
    // Filter by pool if specified
    let filtered_pools = if let Some(pool_filter) = params.pool {
        storage_pools
            .into_iter()
            .filter(|p| p.pool_name.contains(&pool_filter))
            .collect()
    } else {
        storage_pools
    };

    // Return standard JSON - middleware will wrap in AI-First format
    Ok(Json(filtered_pools))
}

/// **POOL INFORMATION**
///
/// Comprehensive information about a storage pool.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Poolinfo
pub struct PoolInfo {
    /// Name of the storage pool
    pub pool_name: String,
    /// Total storage capacity in gigabytes
    pub total_size_gb: u64,
    /// Currently used storage in gigabytes
    pub used_size_gb: u64,
    /// Available storage space in gigabytes
    pub available_size_gb: u64,
    /// Current health status of the pool
    pub health_status: String,
    /// Timestamp of the last scrub operation
    pub last_scrub: Option<String>,
}

/// **POOL QUERY PARAMETERS**
///
/// Query parameters for pool information requests.
#[derive(Debug, Deserialize)]
/// Poolquery
pub struct PoolQuery {
    /// Optional pool name filter
    pub pool: Option<String>,
    /// Whether to include detailed information
    pub detailed: Option<bool>,
}

/// **POOL OPERATION REQUEST**
///
/// Request structure for pool operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for `PoolOperation` operation
pub struct PoolOperationRequest {
    /// Operation to perform on the pool
    pub b_operation: String,
    /// Name of the target pool
    pub pool_name: String,
    /// Optional operation parameters
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

/// Get specific pool information - demonstrates manual AI-First response
///
/// This endpoint manually constructs an AI-First response to demonstrate
/// full control over confidence scores and suggested actions.
pub async fn get_pool_info(
    Path(pool_name): Path<String>,
) -> Json<AIFirstResponse<Option<PoolInfo>>> {
    // Simulate pool lookup
    let pool_info = if pool_name == "main-pool" {
        Some(PoolInfo {
            pool_name: pool_name.clone(),
            total_size_gb: 1000,
            used_size_gb: 650,
            available_size_gb: 350,
            health_status: String::from("ONLINE"),
            last_scrub: Some(String::from("2025-01-29T10:00:00Z")),
        })
    } else if pool_name == "backup-pool" {
        Some(PoolInfo {
            pool_name: pool_name.clone(),
            total_size_gb: 500,
            used_size_gb: 200,
            available_size_gb: 300,
            health_status: String::from("DEGRADED"),
            last_scrub: Some(String::from("2025-01-20T08:00:00Z")),
        })
    } else {
        None
    };
    // Create AI-First response with appropriate confidence and suggestions
    let response = match &pool_info {
        Some(info) => {
            let confidence = if info.health_status == "ONLINE" {
                0.95
            } else {
                0.7
            };

            let mut suggestions = vec![SuggestedAction {
                action_id: String::from("monitor_pool"),
                action_type: ActionType::Monitor,
                description: String::from("Continue monitoring pool health"),
                confidence: 0.9,
                parameters: HashMap::new(),
                priority: 2, // Medium priority
                dependencies: Vec::new(),
                estimated_duration_ms: Some(1000),
            }];

            // Add scrub suggestion if pool is degraded or scrub is old
            if info.health_status == "DEGRADED" {
                suggestions.push(SuggestedAction {
                    action_id: String::from("scrub_pool"),
                    action_type: ActionType::Optimize,
                    description: String::from("Consider running pool scrub to check integrity"),
                    confidence: 0.8,
                    parameters: {
                        let mut params = HashMap::new();
                        params.insert(
                            String::from("pool_name"),
                            serde_json::Value::String(pool_name),
                        );
                        params
                    },
                    priority: 1, // High priority for degraded pools
                    dependencies: Vec::new(),
                    estimated_duration_ms: Some(3_600_000), // 1 hour
                });
            }

            ai_response_with_actions(pool_info, suggestions).with_confidence(confidence)
        }
        None => {
            // Pool not found - low confidence response
            ai_success_with_confidence(pool_info, 0.0)
        }
    };

    Json(response)
}

/// Execute storage operation - demonstrates error handling with AI-First format
///
/// # Errors
///
/// Returns `StatusCode` error if the storage operation cannot be executed.
pub async fn execute_storage_operation(
    Json(request): Json<PoolOperationRequest>,
) -> Result<Json<AIFirstResponse<String>>, StatusCode> {
    // Simulate operation execution
    let result = match request.b_operation.as_str() {
        "scrub" => {
            let message = String::from("Scrub operation started for pool");

            let suggestions = vec![SuggestedAction {
                action_id: String::from("poll_scrub_status"),
                action_type: ActionType::Monitor,
                description: String::from("Poll scrub status every 5 minutes"),
                confidence: 0.9,
                parameters: {
                    let mut params = HashMap::new();
                    params.insert(
                        String::from("poll_interval_ms"),
                        serde_json::Value::Number(serde_json::Number::from(300_000)),
                    );
                    params.insert(
                        String::from("pool_name"),
                        serde_json::Value::String(request.pool_name.clone()),
                    );
                    params
                },
                priority: 1, // High priority
                dependencies: Vec::new(),
                estimated_duration_ms: Some(3_600_000),
            }];
            ai_response_with_actions(message, suggestions)
        }
        "snapshot" => {
            let message = format!("Snapshot created for pool: {}", request.pool_name);
            ai_success_with_confidence(message, 0.95)
        }
        "export" => {
            let message = format!("Export initiated for pool: {}", request.pool_name);

            let suggestions = vec![SuggestedAction {
                action_id: String::from("verify_export"),
                action_type: ActionType::Monitor,
                description: String::from("Verify export completion and data integrity"),
                confidence: 0.85,
                parameters: HashMap::new(),
                priority: 2, // Medium priority
                dependencies: Vec::new(),
                estimated_duration_ms: Some(600_000), // 10 minutes
            }];

            ai_response_with_actions(message, suggestions)
        }
        _ => {
            // Unsupported operation - this would normally return an error
            // but for demo purposes, we'll return a low-confidence response
            let message = format!("Operation '{}' not supported", request.b_operation);
            ai_success_with_confidence(message, 0.1)
        }
    };

    Ok(Json(result))
}

/// Demonstrate different confidence levels - shows AI decision-making support
pub async fn demo_confidence_levels() -> Json<AIFirstResponse<Vec<OptimizationScenario>>> {
    let demos = vec![
        OptimizationScenario {
            scenario: String::from("High confidence - verified data"),
            confidence: 0.95,
            description: String::from("Data verified from multiple sources, high reliability"),
        },
        OptimizationScenario {
            scenario: String::from("Medium confidence - single source"),
            confidence: 0.7,
            description: String::from("Data from single source, moderate reliability"),
        },
        OptimizationScenario {
            scenario: String::from("Low confidence - estimated data"),
            confidence: 0.3,
            description: String::from("Estimated data based on heuristics, low reliability"),
        },
        OptimizationScenario {
            scenario: String::from("No confidence - error condition"),
            confidence: 0.0,
            description: String::from("Error occurred, no reliable data available"),
        },
    ];
    let suggestions = vec![SuggestedAction {
        action_id: String::from("improve_data_sources"),
        action_type: ActionType::Optimize,
        description: String::from("Consider adding more data sources to improve confidence"),
        confidence: 0.8,
        parameters: HashMap::new(),
        priority: 3, // Low priority
        dependencies: Vec::new(),
        estimated_duration_ms: Some(86_400_000), // 1 day
    }];

    Json(ai_response_with_actions(demos, suggestions))
}

/// Demonstrate suggested actions for AI automation
pub async fn demo_suggested_actions() -> Json<AIFirstResponse<Vec<AutomationCapability>>> {
    let demos = vec![
        AutomationCapability {
            category: String::from("Monitoring"),
            description: String::from("Continuous health monitoring with alerts"),
            automation_level: String::from("Fully Automated"),
        },
        AutomationCapability {
            category: String::from("Optimization"),
            description: String::from("Performance tuning based on usage patterns"),
            automation_level: String::from("Semi-Automated"),
        },
        AutomationCapability {
            category: String::from("Recovery"),
            description: String::from("Automated recovery from common failure modes"),
            automation_level: String::from("Fully Automated"),
        },
        AutomationCapability {
            category: String::from("Scaling"),
            description: String::from("Resource scaling based on demand"),
            automation_level: String::from("Manual Approval Required"),
        },
    ];
    let comprehensive_suggestions = vec![
        SuggestedAction {
            action_id: String::from("enable_monitoring"),
            action_type: ActionType::Monitor,
            description: String::from("Enable comprehensive monitoring dashboard"),
            confidence: 0.95,
            parameters: {
                let mut params = HashMap::new();
                params.insert(
                    String::from("dashboard_type"),
                    serde_json::Value::String(String::from("comprehensive")),
                );
                params
            },
            priority: 1, // High priority
            dependencies: Vec::new(),
            estimated_duration_ms: Some(5000),
        },
        SuggestedAction {
            action_id: String::from("configure_alerts"),
            action_type: ActionType::Optimize,
            description: String::from("Configure intelligent alerting based on thresholds"),
            confidence: 0.9,
            parameters: HashMap::new(),
            priority: 2, // Medium priority
            dependencies: vec![String::from("enable_monitoring")],
            estimated_duration_ms: Some(10_000),
        },
        SuggestedAction {
            action_id: String::from("setup_automation"),
            action_type: ActionType::Optimize,
            description: String::from("Setup automated response workflows"),
            confidence: 0.85,
            parameters: HashMap::new(),
            priority: 3, // Lower priority (depends on others)
            dependencies: vec![
                String::from("enable_monitoring"),
                String::from("configure_alerts"),
            ],
            estimated_duration_ms: Some(30000),
        },
    ];

    Json(ai_response_with_actions(demos, comprehensive_suggestions))
}

/// **OPTIMIZATION SCENARIO**
///
/// AI-generated optimization scenario with confidence scoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Optimizationscenario
pub struct OptimizationScenario {
    /// Name of the optimization scenario
    pub scenario: String,
    /// AI confidence in this scenario (0.0 to 1.0)
    pub confidence: f64,
    /// Detailed description of the optimization
    pub description: String,
}

/// **AUTOMATION CAPABILITY**
///
/// Describes an automation capability with AI assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Automationcapability
pub struct AutomationCapability {
    /// Category of automation capability
    pub category: String,
    /// Description of what can be automated
    pub description: String,
    /// Level of automation available
    pub automation_level: String,
}
/// Extension trait to add confidence to existing AI-First responses
trait AIFirstResponseExt<T> {
    /// Builder method to set Confidence
    fn with_confidence(self, confidence: f64) -> AIFirstResponse<T>;
}
impl<T> AIFirstResponseExt<T> for AIFirstResponse<T> {
    /// Builder method to set Confidence
    fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence_score = confidence.clamp(0.0, 1.0);
        self
    }
}

/// **AI POOL OPERATION HANDLER**
///
/// Handle AI-powered pool operations with intelligent recommendations.
pub fn ai_pool_operation(
    Json(request): Json<PoolOperationRequest>,
) -> Json<AIFirstResponse<String>> {
    // AI-enhanced pool operation with confidence scoring
    let operation_result = format!(
        "Executed {} on pool {}",
        request.b_operation, request.pool_name
    );

    Json(ai_success_with_confidence(operation_result, 0.92))
}

/// **AI POOL STATUS HANDLER**
///
/// Get pool status with AI-powered health assessment.
pub fn ai_pool_status(Path(pool_name): Path<String>) -> Json<AIFirstResponse<PoolInfo>> {
    // AI-enhanced pool status with health analysis
    let pool_info = PoolInfo {
        pool_name,
        total_size_gb: 1000,
        used_size_gb: 400,
        available_size_gb: 600,
        health_status: String::from("Healthy"),
        last_scrub: Some(String::from("2024-01-15T10:30:00Z")),
    };

    Json(ai_success_with_confidence(pool_info, 0.95))
}

// Tests moved to ai_first_example_tests.rs for file size compliance

// Tests moved to ai_first_example_tests.rs for file size compliance
// (703 lines of tests separated from 571 lines of implementation)
