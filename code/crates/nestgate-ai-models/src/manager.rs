//! AI Model Manager
//!
//! Manages AI models for storage optimization

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use chrono::Utc;

use crate::{
    ModelConfig, ModelDeployment, ModelType, DeploymentStatus, OptimizationLevel,
    TierOptimization, Priority, Result,
    ModelRegistry, GPUMemoryManager, ModelOptimizer, InferenceService,
};

/// AI Model Manager for NestGate
pub struct ModelManager {
    /// GPU memory manager
    gpu_memory_manager: Arc<GPUMemoryManager>,
    /// Model registry
    model_registry: Arc<ModelRegistry>,
    /// Model optimizer
    model_optimizer: Arc<ModelOptimizer>,
    /// Inference service
    inference_service: Arc<InferenceService>,
    /// Maximum model size (bytes)
    max_model_size: usize,
    /// GPU compute capability
    gpu_compute_capability: f32,
}

impl ModelManager {
    /// Create a new model manager
    pub async fn new(
        gpu_memory_size: usize,
        compute_capability: f32,
        cache_dir: PathBuf,
    ) -> Result<Self> {
        let gpu_memory_manager = Arc::new(GPUMemoryManager::new(gpu_memory_size, compute_capability));
        let model_registry = Arc::new(ModelRegistry::new());
        let model_optimizer = Arc::new(ModelOptimizer::new(cache_dir, compute_capability));
        let inference_service = Arc::new(InferenceService::new(Arc::clone(&model_registry)));

        Ok(Self {
            gpu_memory_manager,
            model_registry,
            model_optimizer,
            inference_service,
            max_model_size: gpu_memory_size / 4, // Reserve 75% for other uses
            gpu_compute_capability: compute_capability,
        })
    }

    /// Start the model manager
    pub async fn start(&self) -> Result<()> {
        info!("Starting AI Model Manager");
        
        // Initialize GPU memory
        info!("GPU compute capability: {}", self.gpu_compute_capability);
        info!("Maximum model size: {} MB", self.max_model_size / (1024 * 1024));
        
        Ok(())
    }

    /// Stop the model manager
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping AI Model Manager");
        
        // Unload all models
        let models = self.model_registry.get_all_models().await?;
        for model in models {
            if let Err(e) = self.unload_model(&model.id).await {
                warn!("Failed to unload model {}: {}", model.id, e);
            }
        }
        
        Ok(())
    }

    /// Validate model compatibility
    async fn validate_model_compatibility(&self, config: &ModelConfig) -> Result<()> {
        // Check compute capability
        if config.min_compute_capability > self.gpu_compute_capability {
            return Err(format!(
                "Model requires compute capability {}, but GPU only supports {}",
                config.min_compute_capability, self.gpu_compute_capability
            ).into());
        }

        // Check model size
        if config.size > self.max_model_size {
            return Err(format!(
                "Model size {}MB exceeds maximum allowed size {}MB",
                config.size / (1024 * 1024),
                self.max_model_size / (1024 * 1024)
            ).into());
        }

        // Check if model file exists
        if !config.path.exists() {
            return Err(format!("Model file not found: {:?}", config.path).into());
        }

        Ok(())
    }

    /// Deploy a model
    pub async fn deploy_model(&self, config: ModelConfig) -> Result<ModelDeployment> {
        info!("Deploying model: {} ({})", config.id, config.model_type as u8);
        
        // Validate compatibility
        self.validate_model_compatibility(&config).await?;
        
        // Check if model is already deployed
        if let Ok(_) = self.model_registry.get_model(&config.id).await {
            return Err(format!("Model {} is already deployed", config.id).into());
        }
        
        // Allocate GPU memory
        let memory = self.gpu_memory_manager
            .allocate_memory(&config.id, config.size, config.priority)
            .await?;
        
        // Create deployment info
        let deployment = ModelDeployment {
            id: config.id.clone(),
            model_type: config.model_type,
            deployed_at: Utc::now(),
            status: DeploymentStatus::Loading,
            memory_usage: config.size,
            optimization_level: OptimizationLevel::Medium,
        };
        
        // Register the model
        self.model_registry.register_model(config.id.clone(), deployment.clone()).await?;
        
        // Update status to active
        let mut active_deployment = deployment.clone();
        active_deployment.status = DeploymentStatus::Active;
        self.model_registry.register_model(config.id.clone(), active_deployment.clone()).await?;
        
        info!("Successfully deployed model: {}", config.id);
        Ok(active_deployment)
    }

    /// Unload a model
    pub async fn unload_model(&self, id: &str) -> Result<()> {
        info!("Unloading model: {}", id);
        
        // Update status to unloading
        if let Ok(mut deployment) = self.model_registry.get_model(id).await {
            deployment.status = DeploymentStatus::Unloading;
            self.model_registry.register_model(id.to_string(), deployment).await?;
        }
        
        // Free GPU memory
        self.gpu_memory_manager.free_memory(id).await?;
        
        // Unregister the model
        self.model_registry.unregister_model(id).await?;
        
        info!("Successfully unloaded model: {}", id);
        Ok(())
    }

    /// Get model information
    pub async fn get_model(&self, id: &str) -> Result<ModelDeployment> {
        self.model_registry.get_model(id).await
    }

    /// Get models by type
    pub async fn get_models_by_type(&self, model_type: ModelType) -> Result<Vec<ModelDeployment>> {
        self.model_registry.get_models_by_type(model_type).await
    }

    /// Get all models
    pub async fn get_all_models(&self) -> Result<Vec<ModelDeployment>> {
        self.model_registry.get_all_models().await
    }

    /// Run inference on a model
    pub async fn run_inference(&self, model_id: &str, input: Vec<f32>) -> Result<Vec<f32>> {
        self.inference_service.run_inference(model_id, input).await
    }

    /// Optimize tier placement using AI
    pub async fn optimize_tier_placement(&self) -> Result<TierOptimization> {
        info!("Running AI-powered tier optimization");
        
        // Find storage optimizer models
        let optimizer_models = self.get_models_by_type(ModelType::StorageOptimizer).await?;
        
        if optimizer_models.is_empty() {
            return Err("No storage optimizer models available".into());
        }
        
        let model = &optimizer_models[0];
        
        // Run optimization inference with real system metrics
        // TODO: Integrate with nestgate-core system metrics collection
        let input = vec![0.25, 0.60, 0.05]; // Placeholder for real CPU, memory, IO metrics
        let output = self.run_inference(&model.id, input).await?;
        
        // Interpret results (simplified)
        let to_warm_count = (output.get(0).unwrap_or(&0.0) * 10.0) as usize;
        let to_cold_count = (output.get(1).unwrap_or(&0.0) * 5.0) as usize;
        let performance_improvement = output.get(2).unwrap_or(&0.0) * 100.0;
        
        let optimization = TierOptimization {
            optimized_at: Utc::now(),
            to_warm_count,
            to_cold_count,
            estimated_performance_improvement: performance_improvement as f64,
        };
        
        info!("Tier optimization completed: {} to warm, {} to cold, {:.1}% improvement",
              optimization.to_warm_count,
              optimization.to_cold_count,
              optimization.estimated_performance_improvement);
        
        Ok(optimization)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_model_manager_creation() {
        let temp_dir = tempdir().expect("Failed to create temporary directory for model manager test");
        let manager = ModelManager::new(
            1024 * 1024 * 1024, // 1GB
            7.5, // RTX 2070 compute capability
            temp_dir.path().to_path_buf(),
        ).await;
        
        assert!(manager.is_ok());
    }
} 