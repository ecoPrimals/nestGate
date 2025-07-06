//! # Hardware Tuning API Routes
//! 
//! **REST API endpoints for hardware-agnostic tuning**
//! 
//! These routes provide external access to hardware tuning capabilities
//! while enforcing crypto lock protection for commercial extraction.

use std::sync::Arc;
use axum::{
    routing::{get, post},
    Router, Json, Path,
    extract::{State, Query},
    response::Json as ResponseJson,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::handlers::{
    HardwareTuningService, HardwareTuningRequest, HardwareTuningResponse,
    BenchmarkResult, TuningProfile, ExtractionLock,
};

/// Hardware tuning API state
#[derive(Clone)]
pub struct HardwareTuningState {
    pub service: Arc<HardwareTuningService>,
}

/// Query parameters for tuning session
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionQuery {
    pub session_id: Uuid,
}

/// Query parameters for benchmark
#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkQuery {
    pub name: String,
}

/// Crypto lock installation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoLockRequest {
    pub source: String,
    pub destination: String,
    pub lock: ExtractionLock,
}

/// Crypto lock installation response
#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoLockResponse {
    pub success: bool,
    pub message: String,
}

/// Create hardware tuning routes
pub fn create_routes(state: HardwareTuningState) -> Router {
    Router::new()
        .route("/hardware/tune", post(start_tuning_session))
        .route("/hardware/session/:session_id", get(get_session_status))
        .route("/hardware/benchmark", post(run_benchmark))
        .route("/hardware/profiles", get(list_tuning_profiles))
        .route("/hardware/crypto-lock", post(install_crypto_lock))
        .route("/hardware/health", get(health_check))
        .with_state(state)
}

/// Start a new hardware tuning session
/// 
/// **POST /api/hardware/tune**
/// 
/// Starts automatic hardware detection and tuning with external extraction protection.
/// 
/// **External Access Protection:**
/// - Internal primal communication: FREE
/// - External system access: CRYPTO LOCK REQUIRED
/// - Commercial extraction: COPYLEFT ENFORCEMENT
/// 
/// **Request Body:**
/// ```json
/// {
///   "mode": "Auto" | "Performance" | "Balanced" | "Efficiency",
///   "target_profile": "optional_profile_name",
///   "custom_params": { "key": "value" },
///   "external_access": {
///     "external_systems": ["https://api.example.com"],
///     "operations": ["read", "write"],
///     "crypto_lock": { ... }
///   }
/// }
/// ```
/// 
/// **Response:**
/// ```json
/// {
///   "session_id": "uuid",
///   "status": "Completed",
///   "hardware_config": { ... },
///   "result": { ... },
///   "performance_improvement": 40.0,
///   "recommendations": ["..."],
///   "warnings": ["..."]
/// }
/// ```
async fn start_tuning_session(
    State(state): State<HardwareTuningState>,
    Json(request): Json<HardwareTuningRequest>,
) -> Result<ResponseJson<HardwareTuningResponse>, StatusCode> {
    match state.service.start_tuning_session(request).await {
        Ok(response) => Ok(ResponseJson(response)),
        Err(e) => {
            eprintln!("Tuning session error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get hardware tuning session status
/// 
/// **GET /api/hardware/session/{session_id}**
/// 
/// Retrieves the current status of a hardware tuning session.
/// 
/// **Path Parameters:**
/// - `session_id`: UUID of the tuning session
/// 
/// **Response:**
/// ```json
/// {
///   "session_id": "uuid",
///   "status": "Completed",
///   "hardware_config": { ... },
///   "result": { ... },
///   "performance_improvement": 40.0
/// }
/// ```
async fn get_session_status(
    State(state): State<HardwareTuningState>,
    Path(session_id): Path<Uuid>,
) -> Result<ResponseJson<HardwareTuningResponse>, StatusCode> {
    match state.service.get_session_status(session_id).await {
        Ok(response) => Ok(ResponseJson(response)),
        Err(e) => {
            eprintln!("Session status error: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Run performance benchmark
/// 
/// **POST /api/hardware/benchmark**
/// 
/// Runs hardware performance benchmarks for CPU, memory, storage, or network.
/// 
/// **Query Parameters:**
/// - `name`: Benchmark name ("cpu", "memory", "storage", "network", "overall")
/// 
/// **Response:**
/// ```json
/// {
///   "name": "cpu",
///   "timestamp": "2024-01-01T00:00:00Z",
///   "hardware_config": { ... },
///   "metrics": {
///     "cpu_score": 85.0,
///     "memory_score": 90.0,
///     "storage_score": 95.0,
///     "network_score": 80.0,
///     "overall_score": 87.5,
///     "latency_ms": 1.0,
///     "throughput_mbps": 5000.0,
///     "iops": 500000
///   }
/// }
/// ```
async fn run_benchmark(
    State(state): State<HardwareTuningState>,
    Query(query): Query<BenchmarkQuery>,
) -> Result<ResponseJson<BenchmarkResult>, StatusCode> {
    match state.service.run_benchmark(&query.name).await {
        Ok(result) => Ok(ResponseJson(result)),
        Err(e) => {
            eprintln!("Benchmark error: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

/// List available tuning profiles
/// 
/// **GET /api/hardware/profiles**
/// 
/// Lists all available hardware tuning profiles.
/// 
/// **Response:**
/// ```json
/// [
///   {
///     "name": "High Performance",
///     "cpu_optimizations": ["enable_turbo", "set_affinity"],
///     "memory_optimizations": ["huge_pages", "numa_aware"],
///     "storage_optimizations": ["io_scheduler", "readahead"],
///     "network_optimizations": ["tcp_tuning", "buffer_sizes"],
///     "estimated_performance_gain": 40.0
///   }
/// ]
/// ```
async fn list_tuning_profiles(
    State(state): State<HardwareTuningState>,
) -> Result<ResponseJson<Vec<TuningProfile>>, StatusCode> {
    match state.service.list_tuning_profiles().await {
        Ok(profiles) => Ok(ResponseJson(profiles)),
        Err(e) => {
            eprintln!("Profile list error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Install crypto lock for external system access
/// 
/// **POST /api/hardware/crypto-lock**
/// 
/// Installs a crypto lock for accessing external systems, enforcing copyleft
/// requirements and preventing commercial extraction.
/// 
/// **Request Body:**
/// ```json
/// {
///   "source": "nestgate-api",
///   "destination": "https://api.example.com",
///   "lock": {
///     "lock_id": "uuid",
///     "lock_type": "SovereignExternal",
///     "proof": {
///       "public_key": "...",
///       "signature": "...",
///       "timestamp": "2024-01-01T00:00:00Z",
///       "nonce": "...",
///       "proof_hash": "...",
///       "ecosystem_fingerprint": "..."
///     },
///     "expires_at": null,
///     "allowed_operations": ["read", "write"],
///     "restrictions": {
///       "max_data_volume": 1000000,
///       "max_api_calls": 1000,
///       "geographic_limits": ["US", "EU"],
///       "purpose_restrictions": ["research", "development"]
///     },
///     "copyleft_requirements": {
///       "require_source_disclosure": true,
///       "require_attribution": true,
///       "require_share_alike": true,
///       "require_modification_disclosure": true,
///       "compatible_licenses": ["GPL-3.0", "AGPL-3.0"]
///     }
///   }
/// }
/// ```
/// 
/// **Response:**
/// ```json
/// {
///   "success": true,
///   "message": "Crypto lock installed successfully"
/// }
/// ```
async fn install_crypto_lock(
    State(state): State<HardwareTuningState>,
    Json(request): Json<CryptoLockRequest>,
) -> Result<ResponseJson<CryptoLockResponse>, StatusCode> {
    match state.service.install_crypto_lock(&request.source, &request.destination, request.lock).await {
        Ok(()) => Ok(ResponseJson(CryptoLockResponse {
            success: true,
            message: "Crypto lock installed successfully".to_string(),
        })),
        Err(e) => {
            eprintln!("Crypto lock installation error: {}", e);
            Ok(ResponseJson(CryptoLockResponse {
                success: false,
                message: format!("Failed to install crypto lock: {}", e),
            }))
        }
    }
}

/// Hardware tuning health check
/// 
/// **GET /api/hardware/health**
/// 
/// Provides health status of the hardware tuning service.
/// 
/// **Response:**
/// ```json
/// {
///   "status": "healthy",
///   "timestamp": "2024-01-01T00:00:00Z",
///   "version": "1.0.0",
///   "features": {
///     "hardware_detection": true,
///     "auto_tuning": true,
///     "crypto_locks": true,
///     "external_protection": true
///   }
/// }
/// ```
async fn health_check(
    State(_state): State<HardwareTuningState>,
) -> ResponseJson<serde_json::Value> {
    ResponseJson(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "version": "1.0.0",
        "features": {
            "hardware_detection": true,
            "auto_tuning": true,
            "crypto_locks": true,
            "external_protection": true,
            "agnostic_setup": true,
            "internal_communication_free": true,
            "copyleft_enforcement": true
        },
        "supported_hardware": {
            "cpu": ["x86_64", "arm64", "riscv64"],
            "memory": ["DDR4", "DDR5", "HBM"],
            "storage": ["HDD", "SSD", "NVMe", "Optane", "Tape"],
            "network": ["Ethernet", "WiFi", "Infiniband", "Fiber"],
            "accelerators": ["GPU", "TPU", "FPGA", "ASIC"]
        },
        "external_boundary_protection": {
            "internal_primal_communication": "FREE",
            "external_system_access": "CRYPTO_LOCK_REQUIRED",
            "commercial_extraction": "COPYLEFT_ENFORCEMENT"
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, Method};
    use tower::ServiceExt;
    
    #[tokio::test]
    async fn test_health_check_endpoint() {
        let service = Arc::new(HardwareTuningService::new());
        let state = HardwareTuningState { service };
        let app = create_routes(state);
        
        let request = Request::builder()
            .method(Method::GET)
            .uri("/hardware/health")
            .body(Body::empty())
            .unwrap();
        
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
    
    #[tokio::test]
    async fn test_list_profiles_endpoint() {
        let service = Arc::new(HardwareTuningService::new());
        let state = HardwareTuningState { service };
        let app = create_routes(state);
        
        let request = Request::builder()
            .method(Method::GET)
            .uri("/hardware/profiles")
            .body(Body::empty())
            .unwrap();
        
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
} 