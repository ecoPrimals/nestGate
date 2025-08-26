// Core implementation of NestGateEcoPrimal
//
// This module contains the main implementation of the EcoPrimal trait for NestGate.

use super::config::*;
use super::errors::*;
use super::traits::*;
use super::types::*;
use serde_json::json;
use std::collections::HashMap;
use tracing::info;

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
    /// Unique identifier for the primal
    pub id: String,
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
            id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

impl EcoPrimal for NestGateEcoPrimal {
    fn metadata(&self) -> &PrimalMetadata {
        &self.metadata
    }

    fn capabilities(&self) -> &[PrimalCapability] {
        &self.capabilities
    }

    fn initialize(&self, config: &PrimalConfig) -> impl Future<Output = Result<(), PrimalError>> + Send {
        async move {
        // Initialize primal with proper configuration
        info!("🚀 Initializing primal: {}", self.id);

        // Validate configuration
        if config.network.additional_ports.is_empty() && config.network.port == 0 {
            return Err(PrimalError::Configuration(
                "No valid ports configured for primal".to_string(),
            ));
        }

        // Initialize capabilities
        for capability in &self.capabilities {
            info!("  ✅ Capability: {:?}", capability);
        }

            info!("✅ Primal initialized successfully: {}", self.id);
            Ok(())
        }
    }

    fn handle_request(&self, request: PrimalRequest) -> impl Future<Output = Result<PrimalResponse, PrimalError>> + Send {
        async move {
        // Handle request with proper routing and error handling
        let start_time = std::time::Instant::now();

        info!(
            "📨 Handling request: {} {}",
            request.method.as_deref().unwrap_or("UNKNOWN"),
            request.path
        );

        // Route based on path and method
        let response_body = match (
            request.method.as_deref().unwrap_or("GET"),
            request.path.as_str(),
        ) {
            ("GET", "/health") => json!({
                "status": "healthy",
                "primal_id": self.id,
                "capabilities": self.capabilities,
                "timestamp": chrono::Utc::now().to_rfc3339()
            })
            .to_string()
            .into_bytes(),
            ("GET", "/capabilities") => json!({
                "capabilities": self.capabilities,
                "primal_id": self.id,
                "version": env!("CARGO_PKG_VERSION")
            })
            .to_string()
            .into_bytes(),
            ("POST", path) if path.starts_with("/api/") => {
                // Handle API requests
                json!({
                    "result": "success",
                    "path": path,
                    "primal_id": self.id,
                    "processed_at": chrono::Utc::now().to_rfc3339()
                })
                .to_string()
                .into_bytes()
            }
            _ => {
                return Err(PrimalError::NotFound(request.path));
            }
        };

        let duration = start_time.elapsed();

        Ok(PrimalResponse {
            status_code: 200,
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers.insert("X-Primal-ID".to_string(), self.id.clone());
                headers
            },
            body: Some(response_body),
            timestamp: chrono::Utc::now(),
            duration_ms: duration.as_millis() as u64,
        })
        }
    }

    fn health_check(&self) -> impl Future<Output = PrimalHealth> + Send {
        async move {
            PrimalHealth::Healthy
        }
    }

    fn shutdown(&self) -> impl Future<Output = Result<(), PrimalError>> + Send {
        async move {
            Ok(())
        }
    }

    fn get_metrics(&self) -> impl Future<Output = Result<PrimalMetrics, PrimalError>> + Send {
        let metrics = self.metrics.clone();
        async move {
            Ok(metrics)
        }
    }

    fn update_config(&self, config: &PrimalConfig) -> impl Future<Output = Result<(), PrimalError>> + Send {
        async move {
            let _ = config;
            Ok(())
        }
    }

    fn supported_api_versions(&self) -> Vec<String> {
        vec!["1.0".to_string()]
    }

    fn supports_capability(&self, capability: &PrimalCapability) -> bool {
        self.capabilities.contains(capability)
    }
}
