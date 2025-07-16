//! Universal Model API Handlers
//!
//! RESTful endpoints for the universal model system that provides agnostic
//! integration with HuggingFace, OpenAI, Claude, local models, and future providers.

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

use nestgate_core::universal_model_api::{
    InferenceInput, InferenceRequest, InferenceResponse, ModelCapability, ModelHandle, ModelInfo, ModelLoadRequest,
    ModelParameters, OutputFormat, ProviderConfig, RegistryHealth, UniversalModelProviderFactory, UniversalModelRegistry,
};

use crate::models::{ErrorResponse, Response};

/// Universal Model API State
#[derive(Clone)]
pub struct UniversalModelApiState {
    pub registry: Arc<UniversalModelRegistry>,
}

/// Model listing query parameters
#[derive(Debug, Deserialize)]
pub struct ModelQuery {
    pub provider: Option<String>,
    pub model_type: Option<String>,
    pub capability: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Provider registration request
#[derive(Debug, Deserialize)]
pub struct RegisterProviderRequest {
    pub provider_config: ProviderConfig,
}

/// Model load request body
#[derive(Debug, Deserialize)]
pub struct LoadModelRequest {
    pub model_id: String,
    pub configuration: Option<serde_json::Value>,
    pub resource_requirements: Option<serde_json::Value>,
    pub optimization_settings: Option<serde_json::Value>,
}

/// Inference request body
#[derive(Debug, Deserialize)]
pub struct InferenceApiRequest {
    pub model_handle_id: Uuid,
    pub input: serde_json::Value,
    pub parameters: Option<serde_json::Value>,
    pub streaming: Option<bool>,
}

/// Create router for universal model API
pub fn create_universal_model_router() -> Router<UniversalModelApiState> {
    Router::new()
        // Provider management
        .route("/providers", get(list_providers))
        .route("/providers", post(register_provider))
        .route("/providers/:provider_id", get(get_provider_info))
        .route("/providers/:provider_id/health", get(get_provider_health))

        // Model management
        .route("/models", get(list_models))
        .route("/models/:model_id", get(get_model_info))
        .route("/models/:model_id/load", post(load_model))
        .route("/models/:model_id/unload", post(unload_model))
        .route("/models/search", get(search_models))

        // Inference
        .route("/inference", post(run_inference))
        .route("/inference/stream", post(stream_inference))

        // Registry management
        .route("/registry/health", get(get_registry_health))
        .route("/registry/metrics", get(get_registry_metrics))

        // Capability queries
        .route("/capabilities", get(list_capabilities))
        .route("/capabilities/:capability", get(find_models_by_capability))
}

/// List all registered providers
pub async fn list_providers(
    State(state): State<UniversalModelApiState>,
) -> Json<Response<Vec<String>>> {
    info!("📋 Listing all registered model providers");

    let providers = state.registry.list_providers().await;
    let total_providers = providers.len();

    Json(Response {
        data: providers,
        metadata: Some(serde_json::json!({
            "total_providers": total_providers,
            "timestamp": chrono::Utc::now()
        })),
    })
}

/// Register a new model provider
pub async fn register_provider(
    State(state): State<UniversalModelApiState>,
    Json(request): Json<RegisterProviderRequest>,
) -> Result<Json<Response<String>>, Json<ErrorResponse>> {
    info!(
        "🔌 Registering new model provider: {}",
        request.provider_config.provider_type
    );

    match UniversalModelProviderFactory::create_provider(request.provider_config.clone()) {
        Ok(provider) => match state.registry.register_provider(provider).await {
            Ok(_) => {
                info!(
                    "✅ Provider {} registered successfully",
                    request.provider_config.provider_type
                );
                Ok(Json(Response {
                    data: request.provider_config.provider_type,
                    metadata: Some(serde_json::json!({
                        "status": "registered",
                        "timestamp": chrono::Utc::now()
                    })),
                }))
            }
            Err(e) => {
                warn!("❌ Failed to register provider: {}", e);
                Err(Json(ErrorResponse {
                    message: format!("Failed to register provider: {}", e),
                    code: Some("PROVIDER_REGISTRATION_FAILED".to_string()),
                    details: None,
                }))
            }
        },
        Err(e) => {
            warn!("❌ Failed to create provider: {}", e);
            Err(Json(ErrorResponse {
                message: format!("Failed to create provider: {}", e),
                code: Some("PROVIDER_CREATION_FAILED".to_string()),
                details: None,
            }))
        }
    }
}

/// Get provider information
pub async fn get_provider_info(
    State(state): State<UniversalModelApiState>,
    Path(provider_id): Path<String>,
) -> Result<Json<Response<serde_json::Value>>, Json<ErrorResponse>> {
    info!("🔍 Getting provider info for: {}", provider_id);

    match state.registry.get_provider(&provider_id).await {
        Some(provider) => {
            let info = serde_json::json!({
                "provider_id": provider.provider_id(),
                "capabilities": provider.capabilities(),
                "auth_requirements": provider.auth_requirements(),
            });

            Ok(Json(Response {
                data: info,
                metadata: Some(serde_json::json!({
                    "timestamp": chrono::Utc::now()
                })),
            }))
        }
        None => Err(Json(ErrorResponse {
            message: format!("Provider {} not found", provider_id),
            code: Some("PROVIDER_NOT_FOUND".to_string()),
            details: None,
        })),
    }
}

/// Get provider health
pub async fn get_provider_health(
    State(state): State<UniversalModelApiState>,
    Path(provider_id): Path<String>,
) -> Result<Json<Response<serde_json::Value>>, Json<ErrorResponse>> {
    info!("🏥 Checking health for provider: {}", provider_id);

    match state.registry.get_provider(&provider_id).await {
        Some(provider) => match provider.health_check().await {
            Ok(health) => Ok(Json(Response {
                data: serde_json::to_value(health).unwrap_or_default(),
                metadata: Some(serde_json::json!({
                    "timestamp": chrono::Utc::now()
                })),
            })),
            Err(e) => {
                warn!("❌ Health check failed for provider {}: {}", provider_id, e);
                Err(Json(ErrorResponse {
                    message: format!("Health check failed: {}", e),
                    code: Some("HEALTH_CHECK_FAILED".to_string()),
                    details: None,
                }))
            }
        },
        None => Err(Json(ErrorResponse {
            message: format!("Provider {} not found", provider_id),
            code: Some("PROVIDER_NOT_FOUND".to_string()),
            details: None,
        })),
    }
}

/// List all available models
pub async fn list_models(
    State(state): State<UniversalModelApiState>,
    Query(query): Query<ModelQuery>,
) -> Result<Json<Response<Vec<ModelInfo>>>, Json<ErrorResponse>> {
    info!("📋 Listing models with query: {:?}", query);

    match state.registry.list_all_models().await {
        Ok(mut models) => {
            // Apply filters
            if let Some(provider) = &query.provider {
                models.retain(|m| m.provider == *provider);
            }

            if let Some(model_type) = &query.model_type {
                models.retain(|m| match serde_json::to_string(&m.model_type) {
                    Ok(type_str) => type_str.contains(model_type),
                    Err(_) => false,
                });
            }

            // Apply pagination
            let total = models.len();
            let offset = query.offset.unwrap_or(0) as usize;
            let limit = query.limit.unwrap_or(100) as usize;

            if offset < models.len() {
                models = models.into_iter().skip(offset).take(limit).collect();
            } else {
                models.clear();
            }

            Ok(Json(Response {
                data: models,
                metadata: Some(serde_json::json!({
                    "total": total,
                    "offset": offset,
                    "limit": limit,
                    "timestamp": chrono::Utc::now()
                })),
            }))
        }
        Err(e) => {
            warn!("❌ Failed to list models: {}", e);
            Err(Json(ErrorResponse {
                message: format!("Failed to list models: {}", e),
                code: Some("MODEL_LIST_FAILED".to_string()),
                details: None,
            }))
        }
    }
}

/// Get specific model information
pub async fn get_model_info(
    State(state): State<UniversalModelApiState>,
    Path(model_id): Path<String>,
) -> Result<Json<Response<ModelInfo>>, Json<ErrorResponse>> {
    info!("🔍 Getting model info for: {}", model_id);

    // Find the model across all providers
    match state.registry.list_all_models().await {
        Ok(models) => match models.into_iter().find(|m| m.id == model_id) {
            Some(model) => Ok(Json(Response {
                data: model,
                metadata: Some(serde_json::json!({
                    "timestamp": chrono::Utc::now()
                })),
            })),
            None => Err(Json(ErrorResponse {
                message: format!("Model {} not found", model_id),
                code: Some("MODEL_NOT_FOUND".to_string()),
                details: None,
            })),
        },
        Err(e) => {
            warn!("❌ Failed to search for model: {}", e);
            Err(Json(ErrorResponse {
                message: format!("Failed to search for model: {}", e),
                code: Some("MODEL_SEARCH_FAILED".to_string()),
                details: None,
            }))
        }
    }
}

/// Load a model for inference
pub async fn load_model(
    State(state): State<UniversalModelApiState>,
    Path(model_id): Path<String>,
    Json(request): Json<LoadModelRequest>,
) -> Result<Json<Response<ModelHandle>>, Json<ErrorResponse>> {
    info!("🚀 Loading model: {}", model_id);

    // Create a model load request with defaults
    let load_request = ModelLoadRequest {
        model_id: model_id.clone(),
        configuration: request
            .configuration
            .and_then(|c| serde_json::from_value(c).ok())
            .unwrap_or_default(),
        resource_requirements: request
            .resource_requirements
            .and_then(|r| serde_json::from_value(r).ok())
            .unwrap_or_default(),
        optimization_settings: request
            .optimization_settings
            .and_then(|o| serde_json::from_value(o).ok())
            .unwrap_or_default(),
    };

    match state.registry.load_model(load_request).await {
        Ok(handle) => {
            info!(
                "✅ Model {} loaded successfully with handle {}",
                model_id, handle.handle_id
            );
            Ok(Json(Response {
                data: handle,
                metadata: Some(serde_json::json!({
                    "timestamp": chrono::Utc::now()
                })),
            }))
        }
        Err(e) => {
            warn!("❌ Failed to load model {}: {}", model_id, e);
            Err(Json(ErrorResponse {
                message: format!("Failed to load model: {}", e),
                code: Some("MODEL_LOAD_FAILED".to_string()),
                details: None,
            }))
        }
    }
}

/// Unload a model
pub async fn unload_model(
    State(state): State<UniversalModelApiState>,
    Path(model_id): Path<String>,
) -> Result<Json<Response<String>>, Json<ErrorResponse>> {
    info!("🔻 Unloading model: {}", model_id);

    match state.registry.unload_model(&model_id).await {
        Ok(()) => {
            info!("✅ Model {} unloaded successfully", model_id);
            Ok(Json(Response {
                data: format!("Model {} unloaded successfully", model_id),
                metadata: Some(serde_json::json!({
                    "timestamp": chrono::Utc::now()
                })),
            }))
        }
        Err(e) => {
            warn!("❌ Failed to unload model {}: {}", model_id, e);
            Err(Json(ErrorResponse {
                message: format!("Failed to unload model {}: {}", model_id, e),
                code: Some("UNLOAD_FAILED".to_string()),
                details: None,
            }))
        }
    }
}

/// Search models by various criteria
pub async fn search_models(
    State(state): State<UniversalModelApiState>,
    Query(query): Query<ModelQuery>,
) -> Result<Json<Response<Vec<ModelInfo>>>, Json<ErrorResponse>> {
    info!("🔍 Searching models with query: {:?}", query);

    // This is similar to list_models but with more advanced search capabilities
    list_models(State(state), Query(query)).await
}

/// Run inference on a model
pub async fn run_inference(
    State(state): State<UniversalModelApiState>,
    Json(request): Json<InferenceApiRequest>,
) -> Result<Json<Response<InferenceResponse>>, Json<ErrorResponse>> {
    info!(
        "🧠 Running inference with model handle: {}",
        request.model_handle_id
    );

    // Find the model handle by ID
    let model_handle = state.registry.get_model_handle(request.model_handle_id).await;

    match model_handle {
        Some(handle) => {
            let inference_request = InferenceRequest {
                request_id: Uuid::new_v4(),
                model_handle: handle,
                input: InferenceInput::Structured(request.input),
                parameters: request.parameters.map(|p| ModelParameters {
                    max_tokens: None,
                    temperature: None,
                    top_p: None,
                    frequency_penalty: None,
                    presence_penalty: None,
                    stop_sequences: None,
                    system_prompt: None,
                    custom_parameters: if let serde_json::Value::Object(map) = p {
                        map.into_iter().collect()
                    } else {
                        std::collections::HashMap::new()
                    },
                }),
                output_format: OutputFormat::Json,
                streaming: request.streaming.unwrap_or(false),
            };

            match state.registry.inference(inference_request).await {
                Ok(response) => {
                    info!("✅ Inference completed successfully");
                    Ok(Json(Response {
                        data: response,
                        metadata: Some(serde_json::json!({
                            "timestamp": chrono::Utc::now()
                        })),
                    }))
                }
                Err(e) => {
                    warn!("❌ Inference failed: {}", e);
                    Err(Json(ErrorResponse {
                        message: format!("Inference failed: {}", e),
                        code: Some("INFERENCE_FAILED".to_string()),
                        details: None,
                    }))
                }
            }
        }
        None => {
            warn!("❌ Model handle {} not found", request.model_handle_id);
            Err(Json(ErrorResponse {
                message: format!("Model handle {} not found", request.model_handle_id),
                code: Some("MODEL_NOT_FOUND".to_string()),
                details: None,
            }))
        }
    }
}

/// Stream inference on a model
pub async fn stream_inference(
    State(state): State<UniversalModelApiState>,
    Json(request): Json<InferenceApiRequest>,
) -> Result<Json<Response<String>>, Json<ErrorResponse>> {
    info!(
        "🌊 Streaming inference with model handle: {}",
        request.model_handle_id
    );

    // Find the model handle by ID
    let model_handle = state.registry.get_model_handle(request.model_handle_id).await;

    match model_handle {
        Some(handle) => {
            let inference_request = InferenceRequest {
                request_id: Uuid::new_v4(),
                model_handle: handle,
                input: InferenceInput::Structured(request.input),
                parameters: request.parameters.map(|p| ModelParameters {
                    max_tokens: None,
                    temperature: None,
                    top_p: None,
                    frequency_penalty: None,
                    presence_penalty: None,
                    stop_sequences: None,
                    system_prompt: None,
                    custom_parameters: if let serde_json::Value::Object(map) = p {
                        map.into_iter().collect()
                    } else {
                        std::collections::HashMap::new()
                    },
                }),
                output_format: OutputFormat::Stream,
                streaming: true, // Force streaming mode
            };

            // For now, use regular inference but indicate streaming intent
            // TODO: Implement actual streaming with SSE/WebSocket
            match state.registry.inference(inference_request).await {
                Ok(response) => {
                    info!("✅ Streaming inference completed successfully");
                    Ok(Json(Response {
                        data: format!("Streaming response: {}", serde_json::to_string(&response).unwrap_or_default()),
                        metadata: Some(serde_json::json!({
                            "timestamp": chrono::Utc::now(),
                            "streaming": true,
                            "note": "Full streaming support with SSE/WebSocket to be implemented"
                        })),
                    }))
                }
                Err(e) => {
                    warn!("❌ Streaming inference failed: {}", e);
                    Err(Json(ErrorResponse {
                        message: format!("Streaming inference failed: {}", e),
                        code: Some("STREAMING_FAILED".to_string()),
                        details: None,
                    }))
                }
            }
        }
        None => {
            warn!("❌ Model handle {} not found", request.model_handle_id);
            Err(Json(ErrorResponse {
                message: format!("Model handle {} not found", request.model_handle_id),
                code: Some("MODEL_NOT_FOUND".to_string()),
                details: None,
            }))
        }
    }
}

/// Get registry health
pub async fn get_registry_health(
    State(state): State<UniversalModelApiState>,
) -> Result<Json<Response<RegistryHealth>>, Json<ErrorResponse>> {
    info!("🏥 Getting registry health");

    match state.registry.get_health().await {
        Ok(health) => Ok(Json(Response {
            data: health,
            metadata: Some(serde_json::json!({
                "timestamp": chrono::Utc::now()
            })),
        })),
        Err(e) => {
            warn!("❌ Failed to get registry health: {}", e);
            Err(Json(ErrorResponse {
                message: format!("Failed to get registry health: {}", e),
                code: Some("REGISTRY_HEALTH_FAILED".to_string()),
                details: None,
            }))
        }
    }
}

/// Get registry metrics
pub async fn get_registry_metrics(
    State(state): State<UniversalModelApiState>,
) -> Json<Response<serde_json::Value>> {
    info!("📊 Getting registry metrics");

    // Get registry health for provider and model counts
    let health = state.registry.get_health().await.unwrap_or_else(|_| {
        // Fallback health data if health check fails
        RegistryHealth {
            total_providers: 0,
            healthy_providers: 0,
            total_models: 0,
            provider_healths: Vec::new(),
            is_healthy: false,
            config_version: "unknown".to_string(),
        }
    });

    // Collect comprehensive metrics
    let metrics = serde_json::json!({
        "registry": {
            "total_providers": health.total_providers,
            "healthy_providers": health.healthy_providers,
            "total_loaded_models": health.total_models,
            "is_healthy": health.is_healthy,
            "config_version": health.config_version
        },
        "providers": health.provider_healths,
        "system": {
            "uptime_seconds": 0, // TODO: Track actual uptime
            "timestamp": chrono::Utc::now()
        },
        "performance": {
            "total_requests": 0,      // TODO: Track actual request count
            "successful_requests": 0, // TODO: Track successful requests
            "failed_requests": 0,     // TODO: Track failed requests
            "average_latency_ms": 0.0 // TODO: Track actual latency
        }
    });

    Json(Response {
        data: metrics,
        metadata: Some(serde_json::json!({
            "timestamp": chrono::Utc::now(),
            "metrics_version": "1.0"
        })),
    })
}

/// List all available capabilities
pub async fn list_capabilities(
    State(state): State<UniversalModelApiState>,
) -> Result<Json<Response<Vec<ModelCapability>>>, Json<ErrorResponse>> {
    info!("🔍 Listing all model capabilities");

    match state.registry.list_all_models().await {
        Ok(models) => {
            let mut capabilities = Vec::new();
            for model in models {
                capabilities.extend(model.capabilities);
            }

            // Deduplicate capabilities
            capabilities.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
            capabilities.dedup();

            let total_capabilities = capabilities.len();

            Ok(Json(Response {
                data: capabilities,
                metadata: Some(serde_json::json!({
                    "total_capabilities": total_capabilities,
                    "timestamp": chrono::Utc::now()
                })),
            }))
        }
        Err(e) => {
            warn!("❌ Failed to list capabilities: {}", e);
            Err(Json(ErrorResponse {
                message: format!("Failed to list capabilities: {}", e),
                code: Some("CAPABILITIES_LIST_FAILED".to_string()),
                details: None,
            }))
        }
    }
}

/// Find models by capability
pub async fn find_models_by_capability(
    State(state): State<UniversalModelApiState>,
    Path(capability): Path<String>,
) -> Result<Json<Response<Vec<ModelInfo>>>, Json<ErrorResponse>> {
    info!("🔍 Finding models by capability: {}", capability);

    // Parse capability string into ModelCapability enum
    let model_capability = match capability.to_lowercase().as_str() {
        "text_generation" | "text-generation" => ModelCapability::TextGeneration {
            max_tokens: None,
            supports_streaming: true,
            supports_system_prompts: true,
            supports_function_calling: false,
        },
        "text_understanding" | "text-understanding" => ModelCapability::TextUnderstanding {
            supports_classification: true,
            supports_sentiment: true,
            supports_named_entities: true,
            supports_summarization: true,
        },
        "code_generation" | "code-generation" => ModelCapability::CodeGeneration {
            supported_languages: vec!["python".to_string(), "javascript".to_string(), "rust".to_string()],
            supports_completion: true,
            supports_explanation: true,
            supports_debugging: true,
        },
        "image_understanding" | "image-understanding" => ModelCapability::ImageUnderstanding {
            supported_formats: vec!["jpeg".to_string(), "png".to_string()],
            max_resolution: Some((1024, 1024)),
            supports_ocr: true,
            supports_description: true,
        },
        "image_generation" | "image-generation" => ModelCapability::ImageGeneration {
            supported_formats: vec!["jpeg".to_string(), "png".to_string()],
            max_resolution: Some((1024, 1024)),
            supports_editing: false,
            supports_variations: true,
        },
        "embeddings" => ModelCapability::Embeddings {
            dimensions: 1536,
            supports_similarity: true,
            supports_clustering: true,
        },
        _ => {
            warn!("❌ Unknown capability: {}", capability);
            return Err(Json(ErrorResponse {
                message: format!("Unknown capability: {}", capability),
                code: Some("UNKNOWN_CAPABILITY".to_string()),
                details: Some(serde_json::json!({
                    "supported_capabilities": [
                        "text_generation", "text_understanding", "code_generation",
                        "image_understanding", "image_generation", "embeddings"
                    ]
                })),
            }));
        }
    };

    // Find models with this capability
    match state.registry.find_models_by_capability(&model_capability).await {
        Ok(models) => {
            let total_models = models.len();
            info!("✅ Found {} models with capability: {}", total_models, capability);
            Ok(Json(Response {
                data: models,
                metadata: Some(serde_json::json!({
                    "capability": capability,
                    "total_models": total_models,
                    "timestamp": chrono::Utc::now()
                })),
            }))
        }
        Err(e) => {
            warn!("❌ Failed to find models by capability: {}", e);
            Err(Json(ErrorResponse {
                message: format!("Failed to find models by capability: {}", e),
                code: Some("CAPABILITY_SEARCH_FAILED".to_string()),
                details: None,
            }))
        }
    }
}

// Default implementations moved to nestgate-core crate
