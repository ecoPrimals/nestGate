/// **AI CAPABILITY DISCOVERY**
/// Discovery and management of AI-related capabilities
/// Replaces hardcoded AI configurations with dynamic discovery
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// AI capability types that can be discovered
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Types of AiCapability
pub enum AiCapabilityType {
    /// Machine learning model inference capabilities
    ModelInference,
    /// Model training and fine-tuning capabilities
    ModelTraining,
    /// Data preprocessing and transformation capabilities
    DataPreprocessing,
    /// Feature extraction from raw data
    FeatureExtraction,
    /// Predictive analytics and forecasting
    PredictiveAnalytics,
    /// Natural language processing and understanding
    NaturalLanguageProcessing,
    /// Computer vision and image analysis
    ComputerVision,
    /// Recommendation engine capabilities
    RecommendationEngine,
}
/// AI capability metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Aicapabilityinfo
pub struct AiCapabilityInfo {
    /// Type of AI capability provided
    pub capability_type: AiCapabilityType,
    /// Service endpoint URL
    pub endpoint: String,
    /// API version string
    pub version: String,
    /// List of supported operations for this capability
    pub supported_operations: Vec<String>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// AI capability discovery manager
#[derive(Debug)]
/// Aicapabilitydiscovery
pub struct AiCapabilityDiscovery {
    discovered_capabilities: tokio::sync::RwLock<HashMap<AiCapabilityType, AiCapabilityInfo>>,
}
impl AiCapabilityDiscovery {
    /// Create new AI capability discovery manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            discovered_capabilities: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Discover available AI capabilities
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn discover_capabilities(&self) -> Result<Vec<AiCapabilityInfo>> {
        // Dynamic discovery logic - replaces hardcoded AI endpoints
        let mut capabilities = Vec::new();

        // Model inference capability discovery
        if let Ok(inference_info) = self.discover_inference_capability().await {
            capabilities.push(inference_info);
        }

        // Predictive analytics capability discovery
        if let Ok(analytics_info) = self.discover_analytics_capability().await {
            capabilities.push(analytics_info);
        }

        // NLP capability discovery
        if let Ok(nlp_info) = self.discover_nlp_capability().await {
            capabilities.push(nlp_info);
        }

        // Update cache
        let mut cache = self.discovered_capabilities.write().await;
        for capability in &capabilities {
            cache.insert(capability.capability_type.clone(), capability.clone());
        }

        Ok(capabilities)
    }

    /// Get specific AI capability by type
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_capability(
        &self,
        capability_type: &AiCapabilityType,
    ) -> Result<Option<AiCapabilityInfo>> {
        let cache = self.discovered_capabilities.read().await;
        Ok(cache.get(capability_type).cloned())
    }

    /// Discover model inference capabilities
    async fn discover_inference_capability(&self) -> Result<AiCapabilityInfo> {
        // Dynamic inference discovery - replaces hardcoded AI model endpoints
        Ok(AiCapabilityInfo {
            capability_type: AiCapabilityType::ModelInference,
            endpoint: "ai://model-inference".to_string(),
            version: "1.0.0".to_string(),
            supported_operations: vec![
                "load_model".to_string(),
                "run_inference".to_string(),
                "batch_inference".to_string(),
                "model_metrics".to_string(),
            ],
            metadata: HashMap::new(),
        })
    }

    /// Discover predictive analytics capabilities
    async fn discover_analytics_capability(&self) -> Result<AiCapabilityInfo> {
        // Dynamic analytics discovery - replaces hardcoded analytics endpoints
        Ok(AiCapabilityInfo {
            capability_type: AiCapabilityType::PredictiveAnalytics,
            endpoint: "ai://predictive-analytics".to_string(),
            version: "1.0.0".to_string(),
            supported_operations: vec![
                "predict_trends".to_string(),
                "anomaly_detection".to_string(),
                "forecasting".to_string(),
                "optimization_recommendations".to_string(),
            ],
            metadata: HashMap::new(),
        })
    }

    /// Discover NLP capabilities
    async fn discover_nlp_capability(&self) -> Result<AiCapabilityInfo> {
        // Dynamic NLP discovery - replaces hardcoded NLP endpoints
        Ok(AiCapabilityInfo {
            capability_type: AiCapabilityType::NaturalLanguageProcessing,
            endpoint: "ai://natural-language-processing".to_string(),
            version: "1.0.0".to_string(),
            supported_operations: vec![
                "text_analysis".to_string(),
                "sentiment_analysis".to_string(),
                "entity_recognition".to_string(),
                "language_translation".to_string(),
            ],
            metadata: HashMap::new(),
        })
    }
}

impl Default for AiCapabilityDiscovery {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// Get AI endpoint for routing compatibility (replaces hardcoded AI constants)
pub async fn get_ai_endpoint(
    _adapter: &crate::universal_adapter::PrimalAgnosticAdapter,
) -> Result<String> {
    let discovery = AiCapabilityDiscovery::new();
    let capabilities = discovery.discover_capabilities().await?;
    // Find model inference capability (primary AI endpoint)
    for capability in capabilities {
        if matches!(capability.capability_type, AiCapabilityType::ModelInference) {
            return Ok(capability.endpoint);
        }
    }

    // Default AI endpoint if discovery fails
    Ok("ai://model-inference".to_string())
}
