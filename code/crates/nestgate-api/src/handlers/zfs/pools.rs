// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// 
// This module provides REST API endpoints for ZFS pool operations with
// AI-First response formatting that enables seamless integration with AI agents.

//! Pools module

use crate::ai_first_wrapper::to_ai_first_response;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use nestgate_core::ai_first::{AIFirstResponse, SuggestedAction};

// Production: Use real ZFS types (aliased for compatibility)
#[cfg(not(feature = "dev-stubs"))]
use nestgate_zfs::{
    types::PoolInfo,
    ProductionZfsManager as ZfsManager,
};
// Note: ZfsConfidenceCalculator is stub-only, not needed in production

// Development: Use stub types
#[cfg(feature = "dev-stubs")]
use crate::dev_stubs::zfs::{PoolInfo, ZfsConfidenceCalculator, ZfsManager};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;
use nestgate_core::{get_or_create_uuid};

/// ZFS pool creation request
#[derive(Debug, Deserialize)]
/// Request parameters for CreatePool operation
pub struct CreatePoolRequest {
    /// Name
    pub name: String,
    ///  Devices
    pub _devices: Vec<String>,
    /// Pool Type
    pub pool_type: String,
    /// Options
    pub options: Option<HashMap<String, String>>,
}
/// ZFS pool information response
#[derive(Debug, Serialize, Clone)]
/// Response data for Pool operation
pub struct PoolResponse {
    /// Name
    pub name: String,
    /// Health
    pub health: String,
    /// Capacity
    pub capacity: Option<PoolCapacityResponse>,
    /// Ai Recommendations
    pub ai_recommendations: Vec<String>,
}
/// Pool capacity information
#[derive(Debug, Serialize, Clone)]
/// Response data for PoolCapacity operation
pub struct PoolCapacityResponse {
    /// Total Bytes
    pub total_bytes: u64,
    /// Free Bytes
    pub free_bytes: u64,
    /// Utilization Percent
    pub utilization_percent: f64,
}
/// Pool operation parameters
#[derive(Debug, Deserialize)]
/// Pooloperationparams
pub struct PoolOperationParams {
    /// Force
    pub force: Option<bool>,
    /// Dry Run
    pub dry_run: Option<bool>,
}
/// Application state containing ZFS manager
#[derive(Clone)]
/// Appstate
pub struct AppState {
    /// Zfs Manager
    pub zfs_manager: Arc<ZfsManager>,
}
impl Default for PoolResponse {
    /// Returns the default instance
    fn default() -> Self { Self {
            name: String::new(),
            health: "unknown".to_string(),
            capacity: None,
            ai_recommendations: vec![],
         }
}

/// Create ZFS pool routes
pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/pools", post(create_pool))
        .route("/pools", get(list_pools))
        .route("/pools/:name", get(get_pool))
        .route("/pools/:name", delete(destroy_pool))
        .route("/pools/:name/scrub", post(start_scrub))
        .route("/pools/:name/export", post(export_pool))
        .route("/pools/:name/import", post(import_pool))
}
/// Create a new ZFS pool with AI-First response format
#[axum::debug_handler]
pub async fn create_pool(
    State(state): State<AppState>,
    Json(request): Json<CreatePoolRequest>,
) -> Result<Json<AIFirstResponse<PoolResponse>>, StatusCode> {
    let request_id = *get_or_create_uuid("pool_create_request");
    let start_time = Instant::now();
    
    // Create pool using ZFS manager
    let result = state.zfs_manager
        .create_pool(&request.name, &request._devices, &request.pool_type)
        .await;
    
    // Convert result to AI-First format
    let ai_result = result.map(|pool_info| {
        let mut response = PoolResponse {
            name: pool_info.name.clone(),
            health: format!("{:?}", pool_info.health),
            capacity: pool_info.capacity.map(|cap| PoolCapacityResponse {
                total_bytes: cap.total_bytes,
                free_bytes: cap.free_bytes,
                utilization_percent: cap.utilization_percent,
            }),
            ai_recommendations: generate_pool_recommendations(&pool_info, "create"),
        };
        response
    });
    
    let response = to_ai_first_response(
        ai_result,
        "zfs_pool_creation",
        start_time,
        request_id,
    );
    
    // Enhance with ZFS-specific confidence scoring
    let enhanced_response = enhance_with_zfs_confidence(
        response,
        "create",
        None, // No existing pool for creation
    );
    
    Ok(Json(enhanced_response))
}
/// List all ZFS pools with AI-First response format
#[axum::debug_handler]
pub async fn list_pools(
    State(state): State<AppState>,
) -> Result<Json<AIFirstResponse<Vec<PoolResponse>>>, StatusCode> {
    let request_id = *get_or_create_uuid("pool_list_request");
    let start_time = Instant::now();
    
    let result = state.zfs_manager.list_pools().await;
    
    let ai_result = result.map(|pools| {
        pools.into_iter().map(|pool_info| {
            PoolResponse {
                name: pool_info.name.clone(),
                health: format!("{:?}", pool_info.health),
                capacity: pool_info.capacity.map(|cap| PoolCapacityResponse {
                    total_bytes: cap.total_bytes,
                    free_bytes: cap.free_bytes,
                    utilization_percent: cap.utilization_percent,
                }),
                ai_recommendations: generate_pool_recommendations(&pool_info, "list"),
            }
        }).collect()
    });
    
    let response = to_ai_first_response(
        ai_result,
        "zfs_pool_listing",
        start_time,
        request_id,
    );
    
    Ok(Json(response))
}
/// Get specific ZFS pool information
#[axum::debug_handler]
pub async fn get_pool(
    Path(pool_name): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<AIFirstResponse<PoolResponse>>, StatusCode> {
    let request_id = *get_or_create_uuid("pool_status_request");
    let start_time = Instant::now();
    
    let result = state.zfs_manager.get_pool_info(&pool_name).await;
    
    let ai_result = result.map(|pool_info| {
        PoolResponse {
            name: pool_info.name.clone(),
            health: format!("{:?}", pool_info.health),
            capacity: pool_info.capacity.as_ref().map(|cap| PoolCapacityResponse {
                total_bytes: cap.total_bytes,
                free_bytes: cap.free_bytes,
                utilization_percent: cap.utilization_percent,
            }),
            ai_recommendations: generate_pool_recommendations(&pool_info, "status"),
        }
    });
    
    let response = to_ai_first_response(
        ai_result,
        "zfs_pool_status",
        start_time,
        request_id,
    );
    
    Ok(Json(response))
}
/// Destroy a ZFS pool
#[axum::debug_handler]
pub async fn destroy_pool(
    Path(pool_name): Path<String>,
    Query(_params): Query<PoolOperationParams>,
    State(state): State<AppState>,
) -> Result<Json<AIFirstResponse<String>>, StatusCode> {
    let request_id = *get_or_create_uuid("pool_destroy_request");
    let start_time = Instant::now();
    
    // Get pool info first for confidence calculation
    let pool_info = state.zfs_manager.get_pool_info(&pool_name).await.ok();
    
    let result = if _params.dry_run.unwrap_or(false) {
        Ok(format!("Would destroy pool '{}' (dry run)", pool_name))
    } else {
        state
            .zfs_manager
            .destroy_pool(&pool_name)
            .await
            .map(|_| format!("Successfully destroyed pool '{}'", pool_name))
    };
    
    let response = to_ai_first_response(
        result,
        "zfs_pool_destruction",
        start_time,
        request_id,
    );
    
    // Enhance with ZFS-specific confidence scoring
    let enhanced_response = enhance_with_zfs_confidence(
        response,
        "destroy",
        pool_info.as_ref(),
    );
    
    Ok(Json(enhanced_response))
}
/// Start pool scrub operation
#[axum::debug_handler]
pub async fn start_scrub(
    Path(pool_name): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<AIFirstResponse<String>>, StatusCode> {
    let request_id = *get_or_create_uuid("pool_scrub_request");
    let start_time = Instant::now();
    
    // Get pool info for confidence calculation
    let pool_info = state.zfs_manager.get_pool_info(&pool_name).await.ok();
    
    let result = state.zfs_manager.scrub_pool(&pool_name).await
        .map(|_| format!("Started scrub for pool '{}'", pool_name));
    
    let response = to_ai_first_response(
        result,
        "zfs_pool_scrub",
        start_time,
        request_id,
    );
    
    // Enhance with ZFS-specific confidence and performance impact
    let enhanced_response = enhance_with_zfs_confidence(
        response,
        "scrub",
        pool_info.as_ref(),
    );
    
    Ok(Json(enhanced_response))
}
/// Export ZFS pool
#[axum::debug_handler]
pub async fn export_pool(
    Path(pool_name): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<AIFirstResponse<String>>, StatusCode> {
    let request_id = *get_or_create_uuid("pool_export_request");
    let start_time = Instant::now();
    
    let pool_info = state.zfs_manager.get_pool_info(&pool_name).await.ok();
    
    let result = state.zfs_manager.export_pool(&pool_name).await
        .map(|_| format!("Successfully exported pool '{}'", pool_name));
    
    let response = to_ai_first_response(
        result,
        "zfs_pool_export",
        start_time,
        request_id,
    );
    
    let enhanced_response = enhance_with_zfs_confidence(
        response,
        "export",
        pool_info.as_ref(),
    );
    
    Ok(Json(enhanced_response))
}
/// Import ZFS pool
#[axum::debug_handler]
pub async fn import_pool(
    Path(pool_name): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<AIFirstResponse<String>>, StatusCode> {
    let request_id = *get_or_create_uuid("pool_import_request");
    let start_time = Instant::now();
    
    let result = state.zfs_manager.import_pool(&pool_name).await
        .map(|_| format!("Successfully imported pool '{}'", pool_name));
    
    let response = to_ai_first_response(
        result,
        "zfs_pool_import",
        start_time,
        request_id,
    );
    
    let enhanced_response = enhance_with_zfs_confidence(
        response,
        "import",
        None, // Pool info not available before import
    );
    
    Ok(Json(enhanced_response))
}
/// Enhance AI-First response with ZFS-specific confidence scoring
fn enhance_with_zfs_confidence<T>(
    mut response: AIFirstResponse<T>,
    operation: &str,
    pool_info: Option<&PoolInfo>,
) -> AIFirstResponse<T> {
    // Calculate ZFS-specific confidence score
    let zfs_confidence = ZfsConfidenceCalculator::pool_operation_confidence(operation, pool_info);
    
    // Use the higher of the two confidence scores
    response.confidence_score = response.confidence_score.max(zfs_confidence);
    
    // Add ZFS-specific suggested actions
    let zfs_actions = generate_zfs_suggested_actions(operation, pool_info);
    response.suggested_actions.extend(zfs_actions);
    
    // Add ZFS performance impact information
    if let Some(info) = pool_info {
        let _performance_impact = ZfsConfidenceCalculator::calculate_performance_impact(
            operation, Some(info), None
        );
        let util = info.capacity.utilization_percent;

        response.ai_metadata.optimization_opportunities.extend(vec![
            format!("Operation CPU impact: {:.1}%", (util * 0.2_f64).min(100.0)),
            format!("Operation I/O impact: {:.1}%", (util * 0.25_f64).min(100.0)),
            format!(
                "Estimated duration: {:.0} minutes",
                (util / 15.0_f64).max(1.0).min(120.0)
            ),
            format!(
                "Recommended scheduling: {}",
                if util > 85.0 {
                    "off-peak hours (high utilization)"
                } else {
                    "standard maintenance window"
                }
            ),
        ]);
    }
    
    response
}
/// Generate ZFS-specific suggested actions for AI agents
fn generate_zfs_suggested_actions(operation: &str, pool_info: Option<&PoolInfo>) -> Vec<SuggestedAction> {
    match operation {
        "scrub" => vec![
            SuggestedAction {
                action_type: "monitor_progress".to_string(),
                description: "Monitor scrub progress with pool status checks".to_string(),
                parameters: HashMap::from([
                    (
                        "command".to_string(),
                        serde_json::Value::String("zpool status".to_string()),
                    ),
                    (
                        "interval_seconds".to_string(),
                        serde_json::Value::Number(serde_json::Number::from(300)),
                    ),
                ]),
                confidence: 0.95,
                estimated_duration_ms: 60000, // 1 minute monitoring intervals
            },
            SuggestedAction {
                action_type: "schedule_next_scrub".to_string(),
                description: "Schedule next scrub based on pool size and usage".to_string(),
                parameters: HashMap::from([(
                    "frequency".to_string(),
                    serde_json::Value::String("monthly".to_string()),
                )]),
                confidence: 0.8,
                estimated_duration_ms: 5000,
            },
        ],
        "create" => vec![
            SuggestedAction {
                action_type: "enable_compression".to_string(),
                description: "Enable LZ4 compression for space efficiency".to_string(),
                parameters: HashMap::from([(
                    "compression".to_string(),
                    serde_json::Value::String("lz4".to_string()),
                )]),
                confidence: 0.9,
                estimated_duration_ms: 2000,
            },
            SuggestedAction {
                action_type: "create_initial_datasets".to_string(),
                description: "Create organizational dataset structure".to_string(),
                parameters: HashMap::new(),
                confidence: 0.85,
                estimated_duration_ms: 10_000,
            },
        ],
        "destroy" => {
            if let Some(info) = pool_info {
                let capacity = &info.capacity;
                if capacity.utilization_percent > 50.0 {
                    return vec![SuggestedAction {
                        action_type: "backup_data".to_string(),
                        description: "Backup important data before pool destruction".to_string(),
                        parameters: HashMap::from([(
                            "data_size_gb".to_string(),
                            serde_json::Value::Number(serde_json::Number::from(
                                (capacity.total_bytes - capacity.available_bytes) / 1_000_000_000,
                            )),
                        )]),
                        confidence: 0.99,
                        estimated_duration_ms: 300000, // 5 minutes
                    }];
                }
            }
            vec![]
        }
        _ => vec![],
    }
}
/// Generate operation-specific recommendations for pools
fn generate_pool_recommendations(pool_info: &PoolInfo, operation: &str) -> Vec<String> {
    let mut recommendations = vec![];
    
    match operation {
        "create" => {
            recommendations.push("Consider enabling compression for space efficiency".to_string());
            recommendations.push("Set up regular scrub schedule for data integrity".to_string());
            recommendations.push("Configure appropriate recordsize for your workload".to_string());
        }
        "status" | "list" => {
            // Add health-based recommendations
            match pool_info.health {
                nestgate_zfs::PoolHealth::Healthy => {
                    recommendations.push("Pool is healthy - consider regular maintenance".to_string());
                }
                nestgate_zfs::PoolHealth::Degraded => {
                    recommendations.push("Pool is degraded - consider resilver or device replacement".to_string());
                    recommendations.push("Check pool status for specific issues".to_string());
                }
                nestgate_zfs::PoolHealth::Faulted => {
                    recommendations.push("Pool is faulted - immediate attention required".to_string());
                    recommendations.push("Review pool status and replace failed _devices".to_string());
                }
                _ => {
                    recommendations.push("Monitor pool health regularly".to_string());
                }
            }
            
            // Add capacity-based recommendations
            if let Some(capacity) = &pool_info.capacity {
                if capacity.utilization_percent > 80.0 {
                    recommendations.push("Pool utilization high - consider adding storage".to_string());
                } else if capacity.utilization_percent > 90.0 {
                    recommendations.push("Pool utilization critical - add storage immediately".to_string());
                }
            }
        }
        _ => {
            recommendations.push("Monitor operation progress and system performance".to_string());
        }
    }
    
    recommendations
}
#[cfg(all(test, feature = "dev-stubs"))]
mod tests {
    use super::*;
    use crate::dev_stubs::zfs::{PoolCapacity, PoolHealth};
    
    #[test]
    fn test_pool_response_creation() {
        let pool_info = PoolInfo {
            name: "testpool".to_string(),
            health: PoolHealth::Healthy,
            capacity: Some(PoolCapacity {
                total_bytes: 1_000_000_000_000, // 1TB
                free_bytes: 500_000_000_000,    // 500GB
                utilization_percent: 50.0,
            }),
        };
        
        let response = PoolResponse {
            name: pool_info.name.clone(),
            health: format!("{:?}", pool_info.health),
            capacity: pool_info.capacity.map(|cap| PoolCapacityResponse {
                total_bytes: cap.total_bytes,
                free_bytes: cap.free_bytes,
                utilization_percent: cap.utilization_percent,
            }),
            ai_recommendations: generate_pool_recommendations(&pool_info, "status"),
        };
        
        assert_eq!(response.name, "testpool");
        assert_eq!(response.health, "Healthy");
        assert!(response.capacity.is_some());
        assert!(!response.ai_recommendations.is_empty());
    }
    
    #[test]
    fn test_zfs_confidence_enhancement() {
        let pool_info = PoolInfo {
            name: "testpool".to_string(),
            health: PoolHealth::Healthy,
            capacity: None,
        };
        
        let confidence = ZfsConfidenceCalculator::pool_operation_confidence("scrub", Some(&pool_info));
        assert_eq!(confidence, 0.95); // Healthy pool should have high scrub confidence
        
        let degraded_pool = PoolInfo {
            name: "degraded".to_string(),
            health: PoolHealth::Degraded,
            capacity: None,
        };
        
        let confidence = ZfsConfidenceCalculator::pool_operation_confidence("scrub", Some(&degraded_pool));
        assert_eq!(confidence, 0.8); // Degraded pool should have lower confidence
    }
} 