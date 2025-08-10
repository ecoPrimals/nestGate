//! Intelligence Adapter
//!
//! This module provides the adapter-based implementation for AI and intelligence operations,
//! replacing hardcoded HuggingFace/Squirrel integrations with the universal adapter pattern.

use crate::ecosystem_integration::universal_adapter::CapabilityRequest;
use crate::ecosystem_integration::{
    DataAnalysisRequest, DataAnalysisResponse, ModelInferenceRequest, ModelInferenceResponse,
    OptimizationRequest, OptimizationResponse, UniversalAdapter,
};
use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info, warn};

/// Model metadata for AI operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub model_id: String,
    pub model_name: String,
    pub model_type: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// AI inference request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIInferenceRequest {
    pub model_id: String,
    pub input_data: serde_json::Value,
    pub parameters: HashMap<String, serde_json::Value>,
    pub timeout_seconds: Option<u64>,
}

/// AI inference response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIInferenceResponse {
    pub prediction: serde_json::Value,
    pub confidence: Option<f64>,
    pub model_version: String,
    pub processing_time_ms: u64,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Data analysis task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisTask {
    pub task_id: String,
    pub analysis_type: String,
    pub dataset: serde_json::Value,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResults {
    pub task_id: String,
    pub results: serde_json::Value,
    pub insights: Vec<String>,
    pub statistics: HashMap<String, f64>,
    pub visualizations: Vec<String>,
}

/// Intelligence adapter using universal adapter pattern
#[derive(Debug, Clone)]
pub struct IntelligenceAdapter {
    /// Universal adapter for external primal communication
    adapter: Arc<UniversalAdapter>,
    /// Service name for AI operations
    service_name: String,
}

impl IntelligenceAdapter {
    /// Create new intelligence adapter
    pub fn new(adapter: Arc<UniversalAdapter>, service_name: String) -> Self {
        info!("🧠 Creating Intelligence Adapter via Universal Adapter");
        info!("🧠 Service: {}", service_name);

        Self {
            adapter,
            service_name,
        }
    }

    /// Create adapter with mock capabilities for testing
    pub fn new_with_mock() -> Result<Self> {
        // For now, return an error since we need the actual adapter infrastructure
        // This will be implemented when the universal adapter is fully available
        Err(NestGateError::Internal {
            message: "Mock intelligence adapter not yet implemented - use real adapter".to_string(),
            location: Some(format!("{}:{}", file!(), line!())),
            debug_info: None,
            is_bug: false,
        })
    }

    /// Run model inference via intelligence adapter
    pub async fn model_inference(
        &self,
        request: &AIInferenceRequest,
    ) -> Result<AIInferenceResponse> {
        info!(
            "🧠 Running model inference via intelligence adapter: {}",
            request.model_id
        );

        // Convert to intelligence capability request
        let inference_request = ModelInferenceRequest {
            model_name: request.model_id.clone(),
            input_data: request.input_data.clone(),
            parameters: request.parameters.clone(),
            timeout_seconds: request.timeout_seconds,
        };

        let payload =
            serde_json::to_vec(&inference_request).map_err(|e| NestGateError::Internal {
                message: format!("Failed to serialize inference request: {e}"),
                location: Some(format!("{}:{}", file!(), line!())),
                debug_info: None,
                is_bug: false,
            })?;

        let capability_request = CapabilityRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            capability_id: "ai.model_inference".to_string(),
            payload,
            metadata: {
                let mut metadata = std::collections::HashMap::new();
                metadata.insert("service".to_string(), self.service_name.clone());
                metadata.insert("model_id".to_string(), request.model_id.clone());
                metadata
            },
            performance_requirements: None,
            timeout: Some(std::time::Duration::from_secs(
                request.timeout_seconds.unwrap_or(300),
            )),
            priority: 7,                // High priority for inference
            requires_encryption: false, // Model inference typically doesn't need encryption
        };

        // Execute via universal adapter
        match self.adapter.execute_capability(capability_request).await {
            Ok(response) => {
                if response.success {
                    let inference_response: ModelInferenceResponse =
                        serde_json::from_slice(&response.payload).map_err(|e| {
                            NestGateError::Internal {
                                message: format!("Failed to deserialize inference response: {e}"),
                                location: Some(format!("{}:{}", file!(), line!())),
                                debug_info: None,
                                is_bug: false,
                            }
                        })?;

                    let ai_response = AIInferenceResponse {
                        prediction: inference_response.prediction,
                        confidence: inference_response.confidence,
                        model_version: inference_response.model_version,
                        processing_time_ms: inference_response.processing_time_ms,
                        metadata: HashMap::new(),
                    };

                    info!(
                        "✅ Model inference completed via intelligence adapter: {} ({}ms)",
                        request.model_id, ai_response.processing_time_ms
                    );
                    Ok(ai_response)
                } else {
                    let error_msg = response
                        .error
                        .map(|e| format!("{e:?}"))
                        .unwrap_or_else(|| "Unknown error".to_string());
                    error!("❌ Model inference failed via adapter: {}", error_msg);
                    Err(NestGateError::Internal {
                        message: format!("Model inference failed: {error_msg}"),
                        location: Some(format!("{}:{}", file!(), line!())),
                        debug_info: None,
                        is_bug: false,
                    })
                }
            }
            Err(e) => {
                error!("❌ Intelligence adapter communication failed: {}", e);
                Err(NestGateError::Internal {
                    message: format!("Intelligence adapter communication failed: {e}"),
                    location: Some(format!("{}:{}", file!(), line!())),
                    debug_info: None,
                    is_bug: false,
                })
            }
        }
    }

    /// Perform data analysis via intelligence adapter
    pub async fn analyze_data(&self, task: &AnalysisTask) -> Result<AnalysisResults> {
        info!(
            "🧠 Analyzing data via intelligence adapter: {} ({})",
            task.task_id, task.analysis_type
        );

        let analysis_request = DataAnalysisRequest {
            dataset: task.dataset.clone(),
            analysis_type: task.analysis_type.clone(),
            parameters: task.parameters.clone(),
        };

        let payload =
            serde_json::to_vec(&analysis_request).map_err(|e| NestGateError::Internal {
                message: format!("Failed to serialize analysis request: {e}"),
                location: Some(format!("{}:{}", file!(), line!())),
                debug_info: None,
                is_bug: false,
            })?;

        let capability_request = CapabilityRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            capability_id: "ai.data_analysis".to_string(),
            payload,
            metadata: {
                let mut metadata = std::collections::HashMap::new();
                metadata.insert("service".to_string(), self.service_name.clone());
                metadata.insert("task_id".to_string(), task.task_id.clone());
                metadata.insert("analysis_type".to_string(), task.analysis_type.clone());
                metadata
            },
            performance_requirements: None,
            timeout: Some(std::time::Duration::from_secs(600)), // 10 minutes for analysis
            priority: 6,                                        // Medium-high priority for analysis
            requires_encryption: false,
        };

        match self.adapter.execute_capability(capability_request).await {
            Ok(response) => {
                if response.success {
                    let analysis_response: DataAnalysisResponse =
                        serde_json::from_slice(&response.payload).map_err(|e| {
                            NestGateError::Internal {
                                message: format!("Failed to deserialize analysis response: {e}"),
                                location: Some(format!("{}:{}", file!(), line!())),
                                debug_info: None,
                                is_bug: false,
                            }
                        })?;

                    let results = AnalysisResults {
                        task_id: task.task_id.clone(),
                        results: analysis_response.results,
                        insights: analysis_response.insights,
                        statistics: analysis_response.statistics,
                        visualizations: vec![], // Add visualization support later
                    };

                    info!(
                        "✅ Data analysis completed via intelligence adapter: {} ({} insights)",
                        task.task_id,
                        results.insights.len()
                    );
                    Ok(results)
                } else {
                    let error_msg = response
                        .error
                        .map(|e| format!("{e:?}"))
                        .unwrap_or_else(|| "Unknown error".to_string());
                    error!("❌ Data analysis failed via adapter: {}", error_msg);
                    Err(NestGateError::Internal {
                        message: format!("Data analysis failed: {error_msg}"),
                        location: Some(format!("{}:{}", file!(), line!())),
                        debug_info: None,
                        is_bug: false,
                    })
                }
            }
            Err(e) => {
                error!("❌ Intelligence adapter communication failed: {}", e);
                Err(NestGateError::Internal {
                    message: format!("Intelligence adapter communication failed: {e}"),
                    location: Some(format!("{}:{}", file!(), line!())),
                    debug_info: None,
                    is_bug: false,
                })
            }
        }
    }

    /// Get optimization suggestions via intelligence adapter
    pub async fn get_optimization_suggestions(
        &self,
        target_system: &str,
        current_metrics: HashMap<String, f64>,
        constraints: Vec<String>,
    ) -> Result<OptimizationResponse> {
        info!(
            "🧠 Getting optimization suggestions via intelligence adapter: {}",
            target_system
        );

        let optimization_request = OptimizationRequest {
            target_system: target_system.to_string(),
            current_metrics,
            constraints,
            optimization_goals: vec![
                "performance".to_string(),
                "efficiency".to_string(),
                "reliability".to_string(),
            ],
        };

        let payload =
            serde_json::to_vec(&optimization_request).map_err(|e| NestGateError::Internal {
                message: format!("Failed to serialize optimization request: {e}"),
                location: Some(format!("{}:{}", file!(), line!())),
                debug_info: None,
                is_bug: false,
            })?;

        let capability_request = CapabilityRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            capability_id: "ai.optimization_suggestions".to_string(),
            payload,
            metadata: {
                let mut metadata = std::collections::HashMap::new();
                metadata.insert("service".to_string(), self.service_name.clone());
                metadata.insert("target_system".to_string(), target_system.to_string());
                metadata
            },
            performance_requirements: None,
            timeout: Some(std::time::Duration::from_secs(120)), // 2 minutes for optimization
            priority: 5,                                        // Medium priority for optimization
            requires_encryption: false,
        };

        match self.adapter.execute_capability(capability_request).await {
            Ok(response) => {
                if response.success {
                    let optimization_response: OptimizationResponse =
                        serde_json::from_slice(&response.payload).map_err(|e| {
                            NestGateError::Internal {
                                message: format!(
                                    "Failed to deserialize optimization response: {e}"
                                ),
                                location: Some(format!("{}:{}", file!(), line!())),
                                debug_info: None,
                                is_bug: false,
                            }
                        })?;

                    info!("✅ Optimization suggestions received via intelligence adapter: {} ({} suggestions, confidence: {:.2})", 
                          target_system, optimization_response.suggestions.len(), optimization_response.confidence_score);
                    Ok(optimization_response)
                } else {
                    let error_msg = response
                        .error
                        .map(|e| format!("{e:?}"))
                        .unwrap_or_else(|| "Unknown error".to_string());
                    error!(
                        "❌ Optimization suggestions failed via adapter: {}",
                        error_msg
                    );
                    Err(NestGateError::Internal {
                        message: format!("Optimization suggestions failed: {error_msg}"),
                        location: Some(format!("{}:{}", file!(), line!())),
                        debug_info: None,
                        is_bug: false,
                    })
                }
            }
            Err(e) => {
                error!("❌ Intelligence adapter communication failed: {}", e);
                Err(NestGateError::Internal {
                    message: format!("Intelligence adapter communication failed: {e}"),
                    location: Some(format!("{}:{}", file!(), line!())),
                    debug_info: None,
                    is_bug: false,
                })
            }
        }
    }

    /// Discover available AI models via intelligence adapter
    pub async fn discover_models(&self, model_type: Option<String>) -> Result<Vec<ModelMetadata>> {
        info!("🧠 Discovering AI models via intelligence adapter");

        let discovery_request = serde_json::json!({
            "model_type": model_type,
            "service": self.service_name,
            "capabilities": ["inference", "analysis"]
        });

        let payload =
            serde_json::to_vec(&discovery_request).map_err(|e| NestGateError::Internal {
                message: format!("Failed to serialize model discovery request: {e}"),
                location: Some(format!("{}:{}", file!(), line!())),
                debug_info: None,
                is_bug: false,
            })?;

        let capability_request = CapabilityRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            capability_id: "ai.model_discovery".to_string(),
            payload,
            metadata: std::collections::HashMap::new(),
            performance_requirements: None,
            timeout: Some(std::time::Duration::from_secs(30)),
            priority: 4, // Medium priority for discovery
            requires_encryption: false,
        };

        match self.adapter.execute_capability(capability_request).await {
            Ok(response) => {
                if response.success {
                    let models_data: serde_json::Value = serde_json::from_slice(&response.payload)
                        .map_err(|e| NestGateError::Internal {
                            message: format!("Failed to deserialize model discovery response: {e}"),
                            location: Some(format!("{}:{}", file!(), line!())),
                            debug_info: None,
                            is_bug: false,
                        })?;

                    let models: Vec<ModelMetadata> = models_data
                        .get("models")
                        .and_then(|m| serde_json::from_value(m.clone()).ok())
                        .unwrap_or_default();

                    info!(
                        "✅ Discovered {} AI models via intelligence adapter",
                        models.len()
                    );
                    Ok(models)
                } else {
                    let error_msg = response
                        .error
                        .map(|e| format!("{e:?}"))
                        .unwrap_or_else(|| "Unknown error".to_string());
                    warn!("⚠️ Model discovery failed via adapter: {}", error_msg);
                    // Return empty list rather than error for discovery
                    Ok(vec![])
                }
            }
            Err(e) => {
                warn!(
                    "⚠️ Intelligence adapter communication failed during discovery: {}",
                    e
                );
                // Return empty list rather than error for discovery
                Ok(vec![])
            }
        }
    }

    /// Health check for intelligence adapter
    pub async fn health_check(&self) -> Result<bool> {
        info!("🧠 Performing intelligence adapter health check");

        let health_request = serde_json::json!({
            "service": self.service_name,
            "check_type": "ai_connectivity"
        });

        let payload = serde_json::to_vec(&health_request).map_err(|e| NestGateError::Internal {
            message: format!("Failed to serialize health check request: {e}"),
            location: Some(format!("{}:{}", file!(), line!())),
            debug_info: None,
            is_bug: false,
        })?;

        let capability_request = CapabilityRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            capability_id: "ai.health_check".to_string(),
            payload,
            metadata: std::collections::HashMap::new(),
            performance_requirements: None,
            timeout: Some(std::time::Duration::from_secs(15)),
            priority: 4, // Medium priority for health checks
            requires_encryption: false,
        };

        match self.adapter.execute_capability(capability_request).await {
            Ok(response) => {
                let healthy = response.success;
                if healthy {
                    info!("✅ Intelligence adapter health check passed");
                } else {
                    warn!("⚠️ Intelligence adapter health check failed");
                }
                Ok(healthy)
            }
            Err(e) => {
                warn!(
                    "⚠️ Intelligence adapter health check communication failed: {}",
                    e
                );
                Ok(false) // Return false rather than error for health checks
            }
        }
    }
}
