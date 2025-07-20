//! Core implementation of NestGateEcoPrimal
//!
//! This module contains the main implementation of the EcoPrimal trait for NestGate.

use super::config::*;
use super::errors::*;
use super::traits::*;
use super::types::*;
use async_trait::async_trait;
use std::collections::HashMap;

/// NestGate EcoPrimal implementation
#[derive(Debug)]
pub struct NestGateEcoPrimal {
    /// Primal metadata
    pub metadata: PrimalMetadata,
    /// Supported capabilities
    pub capabilities: Vec<PrimalCapability>,
    /// Current configuration
    pub config: Option<PrimalConfig>,
    /// Performance metrics
    pub metrics: PrimalMetrics,
}

impl Default for NestGateEcoPrimal {
    fn default() -> Self {
        Self {
            metadata: PrimalMetadata {
                name: "NestGate".to_string(),
                version: "0.1.0".to_string(),
                primal_type: PrimalType::Infrastructure,
                description: "Universal storage and compute primal".to_string(),
                maintainer: "NestGate Team".to_string(),
                repository: Some("https://github.com/nestgate/nestgate".to_string()),
                documentation: Some("https://docs.nestgate.org".to_string()),
                license: "Apache-2.0".to_string(),
                supported_platforms: vec!["linux".to_string(), "darwin".to_string()],
                min_biomeos_version: "1.0.0".to_string(),
                tags: vec![
                    "storage".to_string(),
                    "compute".to_string(),
                    "zfs".to_string(),
                ],
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            capabilities: vec![
                PrimalCapability::FileSystem,
                PrimalCapability::Network,
                PrimalCapability::Monitoring,
            ],
            config: None,
            metrics: PrimalMetrics {
                request_count: 0,
                error_count: 0,
                avg_response_time_ms: 0.0,
                memory_usage_bytes: 0,
                cpu_usage_percent: 0.0,
                uptime_seconds: 0,
                custom_metrics: HashMap::new(),
            },
        }
    }
}

#[async_trait]
impl EcoPrimal for NestGateEcoPrimal {
    fn metadata(&self) -> &PrimalMetadata {
        &self.metadata
    }

    fn capabilities(&self) -> &[PrimalCapability] {
        &self.capabilities
    }

    async fn initialize(&self, config: &PrimalConfig) -> Result<(), PrimalError> {
        // Stub implementation
        let _ = config;
        Ok(())
    }

    async fn handle_request(&self, request: PrimalRequest) -> Result<PrimalResponse, PrimalError> {
        // Stub implementation
        Ok(PrimalResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: Some(format!("Handled request: {}", request.path).into_bytes()),
            timestamp: chrono::Utc::now(),
            duration_ms: 10,
        })
    }

    async fn health_check(&self) -> PrimalHealth {
        PrimalHealth::Healthy
    }

    async fn shutdown(&self) -> Result<(), PrimalError> {
        Ok(())
    }

    async fn get_metrics(&self) -> Result<PrimalMetrics, PrimalError> {
        Ok(self.metrics.clone())
    }

    async fn update_config(&self, config: &PrimalConfig) -> Result<(), PrimalError> {
        let _ = config;
        Ok(())
    }

    fn supported_api_versions(&self) -> Vec<String> {
        vec!["1.0".to_string()]
    }

    fn supports_capability(&self, capability: &PrimalCapability) -> bool {
        self.capabilities.contains(capability)
    }
}
