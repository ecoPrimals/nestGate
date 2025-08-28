//! Universal Data Adapter
//!
//! Routes data requests to any available data capability provider.
//! NestGate doesn't care if data comes from NCBI, HuggingFace, or any other source.
//! It only cares about the capability to provide the requested data.

use crate::data_sources::data_capabilities::*;
use crate::{NestGateError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Universal data adapter that can work with any data provider
pub struct UniversalDataAdapter {
    /// Registered data capability providers
    providers: HashMap<String, Arc<dyn DataCapability>>,
    /// Fallback providers for when primary providers fail
    fallback_providers: HashMap<String, Vec<Arc<dyn DataCapability>>>,
}

impl UniversalDataAdapter {
    /// Create a new universal data adapter
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            fallback_providers: HashMap::new(),
        }
    }
    
    /// Register a data capability provider
    pub fn register_provider(&mut self, provider: Arc<dyn DataCapability>) {
        let capability_type = provider.capability_type().to_string();
        info!("📊 Registering data capability provider: {}", capability_type);
        self.providers.insert(capability_type, provider);
    }
    
    /// Register a fallback provider
    pub fn register_fallback_provider(&mut self, capability_type: String, provider: Arc<dyn DataCapability>) {
        self.fallback_providers
            .entry(capability_type)
            .or_insert_with(Vec::new)
            .push(provider);
    }
    
    /// Execute a data request using any available provider
    pub async fn execute_request(&self, request: &DataRequest) -> Result<DataResponse> {
        debug!("🔍 Executing data request for capability: {}", request.capability_type);
        
        // Try primary provider first
        if let Some(provider) = self.providers.get(&request.capability_type) {
            match provider.can_handle(request).await {
                Ok(true) => {
                    match provider.execute_request(request).await {
                        Ok(response) => return Ok(response),
                        Err(e) => {
                            warn!("Primary provider failed: {}", e);
                            // Continue to try fallbacks
                        }
                    }
                }
                Ok(false) => {
                    debug!("Primary provider cannot handle this request");
                }
                Err(e) => {
                    warn!("Error checking if provider can handle request: {}", e);
                }
            }
        }
        
        // Try fallback providers
        if let Some(fallback_providers) = self.fallback_providers.get(&request.capability_type) {
            for provider in fallback_providers {
                match provider.can_handle(request).await {
                    Ok(true) => {
                        match provider.execute_request(request).await {
                            Ok(response) => return Ok(response),
                            Err(e) => {
                                warn!("Fallback provider failed: {}", e);
                                continue;
                            }
                        }
                    }
                    Ok(false) => continue,
                    Err(e) => {
                        warn!("Error checking fallback provider: {}", e);
                        continue;
                    }
                }
            }
        }
        
        Err(NestGateError::Internal {
            message: format!("No available provider for capability: {}", request.capability_type),
            location: Some("UniversalDataAdapter::execute_request".to_string()),
            context: None,
            is_bug: false,
        })
    }
    
    /// Get available capabilities
    pub fn get_available_capabilities(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }
    
    /// Get provider metadata for a capability
    pub fn get_provider_metadata(&self, capability_type: &str) -> Option<HashMap<String, String>> {
        self.providers.get(capability_type).map(|p| p.get_metadata())
    }
}

impl Default for UniversalDataAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating configured data adapters
pub struct UniversalDataAdapterBuilder {
    adapter: UniversalDataAdapter,
}

impl UniversalDataAdapterBuilder {
    pub fn new() -> Self {
        Self {
            adapter: UniversalDataAdapter::new(),
        }
    }
    
    /// Add a data capability provider
    pub fn with_provider(mut self, provider: Arc<dyn DataCapability>) -> Self {
        self.adapter.register_provider(provider);
        self
    }
    
    /// Add a fallback provider
    pub fn with_fallback_provider(mut self, capability_type: String, provider: Arc<dyn DataCapability>) -> Self {
        self.adapter.register_fallback_provider(capability_type, provider);
        self
    }
    
    /// Build the configured adapter
    pub fn build(self) -> UniversalDataAdapter {
        self.adapter
    }
}

impl Default for UniversalDataAdapterBuilder {
    fn default() -> Self {
        Self::new()
    }
} 