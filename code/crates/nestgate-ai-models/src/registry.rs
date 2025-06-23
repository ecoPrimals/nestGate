//! Model Registry
//!
//! Manages deployed model information

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{ModelDeployment, ModelType, Result};

/// Registry for deployed models
pub struct ModelRegistry {
    /// Deployed models
    models: Arc<RwLock<HashMap<String, ModelDeployment>>>,
}

impl ModelRegistry {
    /// Create a new model registry
    pub fn new() -> Self {
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a model
    pub async fn register_model(&self, id: String, deployment: ModelDeployment) -> Result<()> {
        let mut models = self.models.write().await;
        models.insert(id, deployment);
        Ok(())
    }

    /// Unregister a model
    pub async fn unregister_model(&self, id: &str) -> Result<()> {
        let mut models = self.models.write().await;
        if models.remove(id).is_none() {
            return Err(format!("Model {} not found", id).into());
        }
        Ok(())
    }

    /// Get a model
    pub async fn get_model(&self, id: &str) -> Result<ModelDeployment> {
        let models = self.models.read().await;
        models.get(id)
            .cloned()
            .ok_or_else(|| format!("Model {} not found", id).into())
    }

    /// Get models by type
    pub async fn get_models_by_type(&self, model_type: ModelType) -> Result<Vec<ModelDeployment>> {
        let models = self.models.read().await;
        let filtered: Vec<ModelDeployment> = models
            .values()
            .filter(|deployment| deployment.model_type == model_type)
            .cloned()
            .collect();
        Ok(filtered)
    }

    /// Get all models
    pub async fn get_all_models(&self) -> Result<Vec<ModelDeployment>> {
        let models = self.models.read().await;
        Ok(models.values().cloned().collect())
    }
} 