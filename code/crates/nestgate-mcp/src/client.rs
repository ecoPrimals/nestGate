//! MCP Client Implementations
//!
//! This module contains client implementations for communicating with MCP orchestrators.

use crate::protocol;
use async_trait::async_trait;
use nestgate_core::diagnostics::SystemMetrics;
use nestgate_core::error::{NestGateError, Result};

/// Orchestrator client trait for v2 integration
#[async_trait]
pub trait OrchestratorClient: Send + Sync {
    async fn register_service(&self, service_info: protocol::ServiceInfo) -> Result<()>;
    async fn send_metrics(&self, metrics: &SystemMetrics) -> Result<()>;
    async fn route_message(&self, message: protocol::Message) -> Result<protocol::Response>;
}

/// HTTP-based orchestrator client implementation
pub struct HttpOrchestratorClient {
    base_url: String,
    client: reqwest::Client,
}

impl HttpOrchestratorClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl OrchestratorClient for HttpOrchestratorClient {
    async fn register_service(&self, service_info: protocol::ServiceInfo) -> Result<()> {
        let url = format!("{}/services/register", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&service_info)
            .send()
            .await
            .map_err(|e| {
                NestGateError::mcp_error(
                    &format!("HTTP request failed: {e}"),
                    "register_service",
                    None,
                )
            })?;

        if response.status().is_success() {
            tracing::info!("Successfully registered service with orchestrator");
            Ok(())
        } else {
            Err(NestGateError::mcp_error(
                &format!(
                    "Service registration failed with status: {}",
                    response.status()
                ),
                "register_service",
                None,
            ))
        }
    }

    async fn send_metrics(&self, metrics: &SystemMetrics) -> Result<()> {
        let url = format!("{}/metrics", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&metrics)
            .send()
            .await
            .map_err(|e| {
                NestGateError::mcp_error(&format!("HTTP request failed: {e}"), "send_metrics", None)
            })?;

        if response.status().is_success() {
            tracing::debug!("Successfully sent metrics to orchestrator");
            Ok(())
        } else {
            Err(NestGateError::mcp_error(
                &format!("Metrics sending failed with status: {}", response.status()),
                "send_metrics",
                None,
            ))
        }
    }

    async fn route_message(&self, message: protocol::Message) -> Result<protocol::Response> {
        let url = format!("{}/messages/route", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&message)
            .send()
            .await
            .map_err(|e| {
                NestGateError::mcp_error(
                    &format!("HTTP request failed: {e}"),
                    "route_message",
                    Some(&message.id),
                )
            })?;

        if response.status().is_success() {
            let response_data = response.json().await.map_err(|e| {
                NestGateError::mcp_error(
                    &format!("Failed to parse response: {e}"),
                    "route_message",
                    Some(&message.id),
                )
            })?;
            Ok(response_data)
        } else {
            Err(NestGateError::mcp_error(
                &format!("Message routing failed with status: {}", response.status()),
                "route_message",
                Some(&message.id),
            ))
        }
    }
}
