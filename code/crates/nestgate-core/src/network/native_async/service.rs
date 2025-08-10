use crate::error::Result;
use crate::service_discovery::types::{CommunicationProtocol, ServiceEndpoint};
/// Native Async Network Service Implementation
/// Extracted from native_async_network.rs to maintain file size compliance
/// Contains the main NativeAsyncNetworkService and UniversalService implementation
use crate::traits::UniversalService;
use crate::unified_enums::service_types::{UnifiedServiceState, UnifiedServiceType};
use crate::unified_types::UnifiedNetworkConfig;

// NetworkServiceHealth has been consolidated into unified types
// use super::config::NetworkServiceHealth;

// Define NetworkServiceHealth locally using unified health status
use crate::interface::health_status::HealthStatus;

/// Network service health status (consolidated from config module)
pub type NetworkServiceHealth = HealthStatus;

/// Native async network service using canonical traits
pub struct NativeAsyncNetworkService {
    service_id: String,
    service_type: UnifiedServiceType,
    bind_address: String,
    port: u16,
    state: UnifiedServiceState,
    #[allow(dead_code)]
    endpoints: Vec<ServiceEndpoint>,
}

impl NativeAsyncNetworkService {
    pub fn new(service_id: String, bind_address: String, port: u16) -> Self {
        // Create service endpoints using modern structure
        let endpoints = vec![
            ServiceEndpoint {
                url: format!("http://{bind_address}:{port}/api/v1"),
                protocol: CommunicationProtocol::HTTP,
                health_check: Some("/health".to_string()),
            },
            ServiceEndpoint {
                url: format!("https://{}:{}/api/v1", bind_address, port + 443),
                protocol: CommunicationProtocol::HTTP,
                health_check: Some("/health".to_string()),
            },
        ];
        Self {
            service_id,
            service_type: UnifiedServiceType::Network,
            bind_address: bind_address.clone(),
            port,
            state: UnifiedServiceState::Stopped,
            endpoints,
        }
    }
}

/// **CANONICAL TRAIT IMPLEMENTATION** for network service
/// Demonstrates migration from deprecated traits to canonical UniversalService
#[async_trait::async_trait]
impl UniversalService for NativeAsyncNetworkService {
    type Config = UnifiedNetworkConfig;
    type Health = NetworkServiceHealth;

    async fn initialize(&mut self, config: Self::Config) -> Result<()> {
        self.bind_address = config.bind_address.to_string();
        self.port = config.port;
        self.state = UnifiedServiceState::Starting;
        Ok(())
    }

    async fn start(&mut self) -> Result<()> {
        // Simulate network service startup
        self.state = UnifiedServiceState::Running;
        println!(
            "Network service started on {}:{}",
            self.bind_address, self.port
        );
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        self.state = UnifiedServiceState::Stopped;
        println!("Network service stopped");
        Ok(())
    }

    async fn status(&self) -> UnifiedServiceState {
        self.state.clone()
    }

    async fn health(&self) -> Result<Self::Health> {
        // Return simple health status based on state
        Ok(match &self.state {
            UnifiedServiceState::Running => NetworkServiceHealth::Healthy,
            UnifiedServiceState::Starting => NetworkServiceHealth::Degraded,
            _ => NetworkServiceHealth::Unhealthy,
        })
    }

    fn service_id(&self) -> &str {
        &self.service_id
    }

    fn service_type(&self) -> UnifiedServiceType {
        self.service_type.clone()
    }

    fn name(&self) -> &str {
        "Native Async Network Service"
    }

    fn version(&self) -> &str {
        "2.1.0-canonical"
    }

    fn description(&self) -> &str {
        "High-performance async network service using canonical traits"
    }

    fn capabilities(&self) -> Vec<String> {
        vec![
            "http".to_string(),
            "https".to_string(),
            "async".to_string(),
            "high_performance".to_string(),
            "load_balancing".to_string(),
        ]
    }
}
