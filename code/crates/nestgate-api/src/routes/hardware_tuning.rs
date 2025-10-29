//
// **REST API endpoints for hardware-agnostic tuning**
//
// These routes provide external access to hardware tuning capabilities
// while enforcing crypto lock protection for commercial extraction.

use crate::handlers::hardware_tuning::{
use tracing::info;
use tracing::error;
    HardwareTuningRequest, HardwareTuningResponse,
    ExternalAccessStatus, TuningRecommendations,
};
use crate::routes::AppState;
use axum::response::IntoResponse;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::Json as ResponseJson,
};
use nestgate_core::ExtractionLock;
// Removed unused tracing import
use uuid::Uuid;

/// Auto-tune hardware endpoint with live Compute integration
pub async fn auto_tune(
    State(state): State<AppState>,
    Json(_request): Json<HardwareTuningRequest>,
) -> std::result::Result<ResponseJson<HardwareTuningResponse>, StatusCode> {
    let service = &state.hardware_tuning_service;
    info!("🚀 Auto-tuning hardware with live compute integration");

    match service.auto_tune().await {
        Ok(result) => {
            info!(
                "✅ Auto-tuning completed: {} optimizations applied",
                result.applied_settings.len()
            );

            let performance_gain = result.performance_improvement;

            Ok(ResponseJson(HardwareTuningResponse {
                session_id: Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                status: crate::handlers::hardware_tuning::SessionStatus::Completed,
                hardware_config: None,
                result: Some(result),
                performance_improvement: Some(performance_gain),
                external_access_status: ExternalAccessStatus {
                    access_granted: true,
                    access_level: "standard".to_string(),
                    restrictions: vec![],
                    expires_at: None,
                }
                recommendations: TuningRecommendations {
                    cpu_recommendations: vec![
                        "Hardware tuning completed with live compute data".to_string(),
                    ],
                    memory_recommendations: vec![
                        "Consider monitoring performance metrics".to_string(),
                    ],
                    storage_recommendations: vec![],
                    network_recommendations: vec![],
                }
                warnings: vec![],
            }))
        }
        Err(e) => {
            error!("❌ Auto-tuning failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get hardware configuration with live data
pub async fn get_config(
    State(state): State<AppState>,
) -> std::result::Result<ResponseJson<serde_json::Value>, StatusCode> {
    let service = &state.hardware_tuning_service;
    info!("📊 Getting hardware configuration with live compute data");

    match service.get_config().await {
        Ok(config) => {
            info!("✅ Hardware configuration retrieved with live metrics");
            Ok(ResponseJson(config))
        }
    Err(e) => {
            error!("❌ Failed to get configuration: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get tuning profiles from Compute
pub async fn get_profiles(
    State(state): State<AppState>,
) -> std::result::Result<ResponseJson<serde_json::Value>, StatusCode> {
    let service = &state.hardware_tuning_service;
    info!("📋 Getting tuning profiles from compute service");

    match service.get_profiles().await {
        Ok(profiles) => {
            info!(
                "✅ Retrieved {} tuning profiles from compute service",
                profiles.len()
            );
            Ok(ResponseJson(serde_json::json!({
                "profiles": profiles,
                "source": "compute_service_data",
                "timestamp": chrono::Utc::now()
            })))
        }
        Err(e) => {
            error!("❌ Failed to get profiles: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Run live benchmark with Compute compute resources
pub fn run_benchmark(
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> std::result::Result<ResponseJson<serde_json::Value>, StatusCode> {
    let service = &state.hardware_tuning_service;
    let benchmark_name = request
        .get("benchmark_name")
        .and_then(|v| v.as_str())
        .unwrap_or("overall");
    info!(
        "🏁 Running live benchmark '{}' with compute resources",
        benchmark_name
    );

    match service.benchmark(benchmark_name).await {
        Ok(result) => {
            info!(
                "✅ Live benchmark '{}' completed (score: {})",
                benchmark_name, result.score
            );

            Ok(ResponseJson(serde_json::json!({
                "benchmark_result": result,
                "source": "compute_service_live",
                "timestamp": chrono::Utc::now()
            })))
        }
        Err(e) => {
            error!("❌ Benchmark '{}' failed: {}", benchmark_name, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Generate extraction lock with Security integration
pub fn generate_extraction_lock(
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> std::result::Result<ResponseJson<ExtractionLock>, StatusCode> {
    let service = &state.hardware_tuning_service;
    let source = request
        .get("source")
        .and_then(|v| v.as_str())
        .unwrap_or(nestgate_core::constants::canonical_defaults::network::LOCALHOST)
        .to_string();
    let destination = request
        .get("destination")
        .and_then(|v| v.as_str())
        .unwrap_or("remote")
        .to_string();

    info!(
        "🔐 Generating extraction lock: {} -> {}",
        source, destination
    );

    match service.generate_extraction_lock(source, destination).await {
        Ok(lock) => {
            info!("✅ Extraction lock generated with cryptographic proof");
            Ok(ResponseJson(lock))
        }
    Err(e) => {
            error!("❌ Failed to generate extraction lock: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Verify extraction lock with Security validation
pub fn verify_extraction_lock(
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> std::result::Result<ResponseJson<serde_json::Value>, StatusCode> {
    let service = &state.hardware_tuning_service;
    let lock_id = request
        .get("lock_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .unwrap_or_else(Uuid::new_v4);

    info!("🔓 Verifying extraction lock: {}", lock_id);

    match service.verify_extraction_lock(lock_id).await {
        Ok(valid) => {
            info!(
                "✅ Extraction lock verification: {}",
                if valid { "VALID" } else { "INVALID" }
            );

            Ok(ResponseJson(serde_json::json!({
                "valid": valid,
                "lock_id": lock_id,
                "verified_by": "cryptographic_proof",
                "timestamp": chrono::Utc::now()
            })))
        }
        Err(e) => {
            error!("❌ Failed to verify extraction lock: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get session status
pub fn get_session_status(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    info!("🔍 Getting session status for: {}", session_id);
    // Get session from hardware tuning service
    let session = state
        .hardware_tuning_service
        .get_session_status(session_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(session))
}
