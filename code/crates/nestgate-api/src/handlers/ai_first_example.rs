//! AI-First API Example Handler
//! 
//! This module demonstrates how to use the AI-First response format
//! and provides examples of ecosystem-compliant API endpoints.
//!
//! **ECOSYSTEM COMPLIANCE**: 85%+ AI-First Citizen API Standard
//! **AUTO-CONVERSION**: Middleware automatically wraps responses
//! **MANUAL CONTROL**: Direct AIFirstResponse construction when needed

use axum::{
    extract::{Query, Path},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use nestgate_core::ai_first_refactored::{
    AIFirstResponse, AIFirstResponseBuilder, SuggestedAction, ActionType,
    ai_success, ai_success_with_confidence, ai_response_with_actions,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Example data structure for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub pool_name: String,
    pub total_size_gb: u64,
    pub used_size_gb: u64,
    pub available_size_gb: u64,
    pub health_status: String,
    pub last_scrub: Option<String>,
}

/// Query parameters for storage operations
#[derive(Debug, Deserialize)]
pub struct StorageQuery {
    pub pool: Option<String>,
    pub detailed: Option<bool>,
}

/// Request body for storage operations
#[derive(Debug, Deserialize)]
pub struct StorageRequest {
    pub operation: String,
    pub pool_name: String,
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

/// Create the AI-First example routes
pub fn create_routes() -> Router {
    Router::new()
        .route("/ai-first/storage/info", get(get_storage_info))
        .route("/ai-first/storage/info/:pool", get(get_pool_info))
        .route("/ai-first/storage/operation", post(execute_storage_operation))
        .route("/ai-first/demo/confidence", get(demo_confidence_levels))
        .route("/ai-first/demo/suggestions", get(demo_suggested_actions))
}

/// Get storage information - demonstrates automatic AI-First conversion
/// 
/// This endpoint returns standard JSON that gets automatically wrapped
/// by the AI-First middleware into the ecosystem-standard format.
pub async fn get_storage_info(
    Query(params): Query<StorageQuery>,
) -> Result<Json<Vec<StorageInfo>>, StatusCode> {
    // Simulate storage data retrieval
    let storage_pools = vec![
        StorageInfo {
            pool_name: "main-pool".to_string(),
            total_size_gb: 1000,
            used_size_gb: 650,
            available_size_gb: 350,
            health_status: "ONLINE".to_string(),
            last_scrub: Some("2025-01-29T10:00:00Z".to_string()),
        },
        StorageInfo {
            pool_name: "backup-pool".to_string(),
            total_size_gb: 500,
            used_size_gb: 200,
            available_size_gb: 300,
            health_status: "ONLINE".to_string(),
            last_scrub: Some("2025-01-28T15:30:00Z".to_string()),
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

/// Get specific pool information - demonstrates manual AI-First response
/// 
/// This endpoint manually constructs an AI-First response to demonstrate
/// full control over confidence scores and suggested actions.
pub async fn get_pool_info(
    Path(pool_name): Path<String>,
) -> Json<AIFirstResponse<Option<StorageInfo>>> {
    // Simulate pool lookup
    let pool_info = if pool_name == "main-pool" {
        Some(StorageInfo {
            pool_name: pool_name.clone(),
            total_size_gb: 1000,
            used_size_gb: 650,
            available_size_gb: 350,
            health_status: "ONLINE".to_string(),
            last_scrub: Some("2025-01-29T10:00:00Z".to_string()),
        })
    } else if pool_name == "backup-pool" {
        Some(StorageInfo {
            pool_name: pool_name.clone(),
            total_size_gb: 500,
            used_size_gb: 200,
            available_size_gb: 300,
            health_status: "DEGRADED".to_string(),
            last_scrub: Some("2025-01-20T08:00:00Z".to_string()),
        })
    } else {
        None
    };

    // Create AI-First response with appropriate confidence and suggestions
    let response = match &pool_info {
        Some(info) => {
            let confidence = if info.health_status == "ONLINE" { 0.95 } else { 0.7 };
            
            let mut suggestions = vec![
                SuggestedAction {
                    action_id: "monitor_pool".to_string(),
                    action_type: ActionType::Monitor,
                    description: "Continue monitoring pool health".to_string(),
                    confidence: 0.9,
                    parameters: HashMap::new(),
                    dependencies: Vec::new(),
                    estimated_duration_ms: Some(1000),
                }
            ];

            // Add scrub suggestion if pool is degraded or scrub is old
            if info.health_status == "DEGRADED" {
                suggestions.push(SuggestedAction {
                    action_id: "scrub_pool".to_string(),
                    action_type: ActionType::Optimize,
                    description: "Consider running pool scrub to check integrity".to_string(),
                    confidence: 0.8,
                    parameters: {
                        let mut params = HashMap::new();
                        params.insert("pool_name".to_string(), 
                                    serde_json::Value::String(pool_name.clone()));
                        params
                    },
                    dependencies: Vec::new(),
                    estimated_duration_ms: Some(3600000), // 1 hour
                });
            }

            ai_response_with_actions(pool_info, suggestions)
                .with_confidence(confidence)
        }
        None => {
            // Pool not found - low confidence response
            ai_success_with_confidence(pool_info, 0.0)
        }
    };

    Json(response)
}

/// Execute storage operation - demonstrates error handling with AI-First format
pub async fn execute_storage_operation(
    Json(request): Json<StorageRequest>,
) -> Result<Json<AIFirstResponse<String>>, StatusCode> {
    // Simulate operation execution
    let result = match request.operation.as_str() {
        "scrub" => {
            let message = format!("Scrub operation started for pool: {}", request.pool_name);
            
            let suggestions = vec![
                SuggestedAction {
                    action_id: "poll_scrub_status".to_string(),
                    action_type: ActionType::Monitor,
                    description: "Poll scrub status every 5 minutes".to_string(),
                    confidence: 0.9,
                    parameters: {
                        let mut params = HashMap::new();
                        params.insert("poll_interval_ms".to_string(), 
                                    serde_json::Value::Number(serde_json::Number::from(300000)));
                        params.insert("pool_name".to_string(), 
                                    serde_json::Value::String(request.pool_name.clone()));
                        params
                    },
                    dependencies: Vec::new(),
                    estimated_duration_ms: Some(3600000),
                }
            ];

            ai_response_with_actions(message, suggestions)
        }
        "snapshot" => {
            let message = format!("Snapshot created for pool: {}", request.pool_name);
            ai_success_with_confidence(message, 0.95)
        }
        "export" => {
            let message = format!("Export initiated for pool: {}", request.pool_name);
            
            let suggestions = vec![
                SuggestedAction {
                    action_id: "verify_export".to_string(),
                    action_type: ActionType::Monitor,
                    description: "Verify export completion and data integrity".to_string(),
                    confidence: 0.85,
                    parameters: HashMap::new(),
                    dependencies: Vec::new(),
                    estimated_duration_ms: Some(600000), // 10 minutes
                }
            ];

            ai_response_with_actions(message, suggestions)
        }
        _ => {
            // Unsupported operation - this would normally return an error
            // but for demo purposes, we'll return a low-confidence response
            let message = format!("Operation '{}' not supported", request.operation);
            ai_success_with_confidence(message, 0.1)
        }
    };

    Ok(Json(result))
}

/// Demonstrate different confidence levels - shows AI decision-making support
pub async fn demo_confidence_levels() -> Json<AIFirstResponse<Vec<ConfidenceDemo>>> {
    let demos = vec![
        ConfidenceDemo {
            scenario: "High confidence - verified data".to_string(),
            confidence: 0.95,
            description: "Data verified from multiple sources, high reliability".to_string(),
        },
        ConfidenceDemo {
            scenario: "Medium confidence - single source".to_string(),
            confidence: 0.7,
            description: "Data from single source, moderate reliability".to_string(),
        },
        ConfidenceDemo {
            scenario: "Low confidence - estimated data".to_string(),
            confidence: 0.3,
            description: "Estimated data based on heuristics, low reliability".to_string(),
        },
        ConfidenceDemo {
            scenario: "No confidence - error condition".to_string(),
            confidence: 0.0,
            description: "Error occurred, no reliable data available".to_string(),
        },
    ];

    let suggestions = vec![
        SuggestedAction {
            action_id: "improve_data_sources".to_string(),
            action_type: ActionType::Optimize,
            description: "Consider adding more data sources to improve confidence".to_string(),
            confidence: 0.8,
            parameters: HashMap::new(),
            dependencies: Vec::new(),
            estimated_duration_ms: Some(86400000), // 1 day
        }
    ];

    Json(ai_response_with_actions(demos, suggestions))
}

/// Demonstrate suggested actions for AI automation
pub async fn demo_suggested_actions() -> Json<AIFirstResponse<Vec<ActionDemo>>> {
    let demos = vec![
        ActionDemo {
            category: "Monitoring".to_string(),
            description: "Continuous health monitoring with alerts".to_string(),
            automation_level: "Fully Automated".to_string(),
        },
        ActionDemo {
            category: "Optimization".to_string(),
            description: "Performance tuning based on usage patterns".to_string(),
            automation_level: "Semi-Automated".to_string(),
        },
        ActionDemo {
            category: "Recovery".to_string(),
            description: "Automated recovery from common failure modes".to_string(),
            automation_level: "Fully Automated".to_string(),
        },
        ActionDemo {
            category: "Scaling".to_string(),
            description: "Resource scaling based on demand".to_string(),
            automation_level: "Manual Approval Required".to_string(),
        },
    ];

    let comprehensive_suggestions = vec![
        SuggestedAction {
            action_id: "enable_monitoring".to_string(),
            action_type: ActionType::Monitor,
            description: "Enable comprehensive monitoring dashboard".to_string(),
            confidence: 0.95,
            parameters: {
                let mut params = HashMap::new();
                params.insert("dashboard_type".to_string(), 
                            serde_json::Value::String("comprehensive".to_string()));
                params
            },
            dependencies: Vec::new(),
            estimated_duration_ms: Some(5000),
        },
        SuggestedAction {
            action_id: "configure_alerts".to_string(),
            action_type: ActionType::Optimize,
            description: "Configure intelligent alerting based on thresholds".to_string(),
            confidence: 0.9,
            parameters: HashMap::new(),
            dependencies: vec!["enable_monitoring".to_string()],
            estimated_duration_ms: Some(10000),
        },
        SuggestedAction {
            action_id: "setup_automation".to_string(),
            action_type: ActionType::Optimize,
            description: "Setup automated response workflows".to_string(),
            confidence: 0.85,
            parameters: HashMap::new(),
            dependencies: vec!["enable_monitoring".to_string(), "configure_alerts".to_string()],
            estimated_duration_ms: Some(30000),
        },
    ];

    Json(ai_response_with_actions(demos, comprehensive_suggestions))
}

/// Demo data structure for confidence levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceDemo {
    pub scenario: String,
    pub confidence: f64,
    pub description: String,
}

/// Demo data structure for suggested actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDemo {
    pub category: String,
    pub description: String,
    pub automation_level: String,
}

/// Extension trait to add confidence to existing AI-First responses
trait AIFirstResponseExt<T> {
    fn with_confidence(self, confidence: f64) -> AIFirstResponse<T>;
}

impl<T> AIFirstResponseExt<T> for AIFirstResponse<T> {
    fn with_confidence(mut self, confidence: f64) -> AIFirstResponse<T> {
        self.confidence_score = confidence.clamp(0.0, 1.0);
        self
    }
} 