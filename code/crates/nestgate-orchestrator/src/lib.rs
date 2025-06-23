//! Sapsucker Orchestrator
//! 
//! Service-centric orchestrator with WebSocket support
//! Central coordination hub for all Sapsucker services

pub mod orchestrator;
pub mod services;
pub mod communication;
pub mod scalability;
pub mod robustness;
pub mod security;
pub mod errors;

// Export the main types
pub use orchestrator::{
    Orchestrator, OrchestratorConfig, ServiceInfo, ManagedService, ServiceHealth, ServiceMetrics, 
    ServiceInstance, CircuitBreaker, RateLimiter, ConnectionPool, WebSocketConnection,
    ClientInfo, FederationStatus, WebSocketClient, OrchestratorMetrics, LoadBalancer,
    LoadBalancingAlgorithm, PortAllocator, OrchestratorEvent, ExternalSystem,
    ExternalTool, ExternalSystemStatus, ToolStatus, TowerInfo, TowerStatus,
    HealthConfig, WebSocketConfig, StressTestConfig, TestingConfig, McpConfig,
    InterTowerCommunicationConfig
};

pub use services::{
    ZfsService, ApiService, NetworkService, McpService, TowerFederationService, MockTowerService
};
pub use communication::*;
pub use scalability::*;
pub use robustness::*;
pub use security::*;
pub use errors::*; 