#[allow(deprecated)] // PEDANTIC: Allow deprecated ServiceRegistration during migration
//! **NESTGATE NETWORK CRATE**
//!
//! This module provides network functionality for NestGate,
//! organized into focused modules for maintainability.

// ==================== SECTION ====================

//! **SERVICE**: Main network service implementation
pub mod service;
//! **TYPES**: Data structures and configuration types
pub mod types;
//! **HANDLERS**: Protocol handlers and network management
pub mod handlers;
// ==================== SECTION ====================

//! Legacy modules for backward compatibility
pub mod api;
pub mod connection_manager;
pub mod orchestration_adapter;
pub mod universal_orchestration;
// ==================== SECTION ====================

//! Main network service types
pub use service::RealNetworkService;
pub use types::{
    NetworkConfig, NetworkConfigBuilder, ConnectionInfo, ServiceInfo,
    NetworkStatistics, ServiceStatus, ConnectionDetails, ServiceDetails,
    ConnectionStatus, HealthStatus
};
pub use handlers::{
    NetworkService, NetworkServiceManager, HttpProtocolHandler, TcpProtocolHandler,
    LoadBalancer, LoadBalancingStrategy, HttpRequest, HttpResponse
};
//! Legacy exports for backward compatibility
pub use orchestration_adapter::{
    OrchestrationAdapter, OrchestrationServiceRegistration,
    ServiceDiscoveryRequest, ServiceDiscoveryResponse,
    DiscoveredService,
};
//! Use canonical Result type from nestgate-core::error
pub use nestgate_core::error::Result; 