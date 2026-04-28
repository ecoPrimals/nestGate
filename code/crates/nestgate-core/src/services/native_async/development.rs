// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

/// Development Service Implementations
/// Contains development-focused implementations with enhanced debugging capabilities
/// Extracted for file size compliance and development workflow optimization
use crate::error::CanonicalResult as Result;
// Import missing ServiceRequest type
use crate::universal_traits::ServiceRequest;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use super::traits::NativeAsyncLoadBalancer;
use super::types::{LoadBalancerStats, ServiceResponse, ServiceStats};
use crate::service_discovery::types::ServiceInfo;
use nestgate_config::constants::system::DEFAULT_SERVICE_NAME;
use tracing::debug;
use uuid::Uuid;

/// Development load balancer for testing
pub struct DevelopmentLoadBalancer {
    service_count: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}
impl Default for DevelopmentLoadBalancer {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            service_count: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }
}

impl NativeAsyncLoadBalancer<100, 1000, 3600, 60> for DevelopmentLoadBalancer {
    /// Type alias for ServiceInfo
    type ServiceInfo = ServiceInfo;
    /// Type alias for ServiceRequest
    type ServiceRequest = ServiceRequest;
    /// Type alias for ServiceResponse
    type ServiceResponse = ServiceResponse;
    /// Type alias for LoadBalancerStats
    type LoadBalancerStats = LoadBalancerStats;
    /// Type alias for ServiceStats
    type ServiceStats = ServiceStats;

    /// Add Service
    async fn add_service(&self, service: Self::ServiceInfo) -> Result<()> {
        // Development service addition - always succeed
        self.service_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        debug!("DEV: Added service {service:?}");
        Ok(())
    }

    /// Remove Service
    async fn remove_service(&self, service_id: &str) -> Result<()> {
        self.service_count
            .fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        debug!("DEV: Removed service {service_id}");
        Ok(())
    }

    /// Route Request
    async fn route_request(&self, _request: Self::ServiceRequest) -> Result<Self::ServiceResponse> {
        Ok(ServiceResponse {
            success: true,
            data: b"Development response".to_vec(),
            request_id: Some("dev-request-123".to_string()),
            status: crate::canonical_types::ResponseStatus::Success,
            headers: HashMap::new(),
            payload: serde_json::json!({"status": "dev_success"}),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            duration: Duration::from_millis(1),
            processing_time: 1,
            tags: HashMap::new(),
            error_details: None,
            correlation_id: Some("dev-correlation".to_string()),
            trace_id: Some("dev-trace".to_string()),
        })
    }

    /// Gets Stats
    async fn get_stats(&self) -> Result<Self::LoadBalancerStats> {
        Ok(LoadBalancerStats {
            total_requests: 100,
            successful_requests: 95,
            failed_requests: 5,
            average_response_time: 1.5,
            service_stats: HashMap::new(),
            algorithm: "dev_round_robin".to_string(),
            health_aware: false,
            uptime_seconds: 300,
        })
    }

    /// Gets Service Stats
    async fn get_service_stats(&self, _service_id: &str) -> Result<Self::ServiceStats> {
        Ok(ServiceStats {
            requests: 10,
            successful_requests: 9,
            failed_requests: 1,
            average_response_time: 1.2,
            current_load: 0.1,
            health_score: 0.9,
            last_request_time: Some(SystemTime::now()),
        })
    }

    /// Health Check All
    async fn health_check_all(&self) -> Result<Vec<(String, bool)>> {
        Ok(vec![
            ("dev-service-1".to_string(), true),
            ("dev-service-2".to_string(), true),
        ])
    }

    /// Updates  Service Weight
    async fn update_service_weight(&self, service_id: &str, weight: f64) -> Result<()> {
        debug!("DEV: Updated service {service_id} weight to {weight}");
        Ok(())
    }

    /// List Services
    async fn list_services(&self) -> Result<Vec<Self::ServiceInfo>> {
        Ok(vec![])
    }

    /// Gets Service
    async fn get_service(&self, service_id: &str) -> Result<Option<Self::ServiceInfo>> {
        if service_id == "dev-service" {
            Ok(Some(ServiceInfo {
                service_id: Uuid::new_v4(),
                metadata: crate::service_discovery::types::ServiceMetadata {
                    name: "dev-service".to_string(),
                    category: crate::service_discovery::types::ServiceCategory::Development,
                    version: "1.0.0".to_string(),
                    description: "Development service".to_string(),
                    health_endpoint: Some("/health".to_string()),
                    metrics_endpoint: Some("/metrics".to_string()),
                },
                capabilities: vec![crate::service_discovery::types::ServiceCapability::Custom {
                    namespace: DEFAULT_SERVICE_NAME.to_string(),
                    capability: "development".to_string(),
                    version: "1.0.0".to_string(),
                }],
                endpoints: vec![crate::service_discovery::types::ServiceEndpoint {
                    url: {
                        // Now uses centralized runtime configuration
                        use crate::config::runtime::get_config;
                        let config = get_config();
                        config.network.api_base_url()
                    },
                    protocol: crate::service_discovery::types::CommunicationProtocol::Http,
                    health_check: Some("/health".to_string()),
                }],
                last_seen: SystemTime::now(),
            }))
        } else {
            Ok(None)
        }
    }

    /// Service Exists
    async fn service_exists(&self, _service_id: &str) -> Result<bool> {
        Ok(true) // Always exists in development
    }
}

/// Type aliases for development use
pub type DevelopmentServiceLoadBalancer = DevelopmentLoadBalancer;
