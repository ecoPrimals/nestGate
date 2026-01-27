//! # 🚀 RPC Module for NestGate
//!
//! **HIGH-PERFORMANCE PRIMAL-TO-PRIMAL COMMUNICATION** (v0.2.0)
//!
//! Provides tarpc and JSON-RPC interfaces for NestGate storage operations.
//!
//! ## Protocol Priority (Ecosystem Standard)
//! 1. **tarpc** (PRIMARY) - High-performance binary RPC for primal-to-primal (~10-20μs)
//! 2. **JSON-RPC** (SECONDARY) - Universal, human-friendly (~50-100μs)
//! 3. **HTTP** (FALLBACK) - Enableable for network scenarios (~500-1000μs)
//!
//! ## Philosophy (Primal Sovereignty)
//! - **Self-knowledge**: NestGate exposes only storage capabilities
//! - **Runtime discovery**: Other primals discovered via capability
//! - **Zero hardcoding**: No primal names, ports, or endpoints
//! - **Zero unsafe blocks**: Memory-safe throughout
//! - **Modern async**: Native async/await patterns
//!
//! ## Usage
//!
//! ### Server
//! ```no_run
//! use nestgate_core::rpc::{NestGateRpcService, serve_tarpc};
//! use std::net::SocketAddr;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let service = NestGateRpcService::new();
//! // Environment-driven: $NESTGATE_RPC_HOST and $NESTGATE_RPC_PORT
//! let addr: SocketAddr = nestgate_core::constants::ports::get_rpc_server_addr().parse()?;
//! serve_tarpc(addr, service).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Client
//! ```no_run
//! use nestgate_core::rpc::NestGateRpcClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Environment-driven: $NESTGATE_RPC_ADDR or default
//! let rpc_addr = std::env::var("NESTGATE_RPC_ADDR")
//!     .unwrap_or_else(|_| "tarpc://localhost:8091".to_string());
//! let client = NestGateRpcClient::new(&rpc_addr)?;
//! let health = client.health().await?;
//! println!("Service status: {}", health.status);
//! # Ok(())
//! # }
//! ```

pub mod audit_storage;
pub mod jsonrpc_client;
pub mod jsonrpc_server;
pub mod orchestrator_registration;
pub mod socket_config;
// pub mod songbird_registration; // REMOVED: Deprecated since v2.3.0, zero production usage
pub mod tarpc_client;
pub mod tarpc_server;
pub mod tarpc_types;
pub mod template_storage;
pub mod unix_socket_server;

// Re-export key types
pub use jsonrpc_client::{JsonRpcClient, JsonRpcError, JsonRpcRequest, JsonRpcResponse};
pub use tarpc_types::{
    CapabilityRegistration, DatasetInfo, DatasetParams, HealthStatus, NestGateRpc,
    NestGateRpcError, ObjectInfo, OperationResult, ProtocolInfo, RegistrationResult, ServiceInfo,
    StorageMetrics, VersionInfo,
};

pub use audit_storage::{
    AuditStorage, ExecutionAudit, ExecutionStatus, GraphModification, ModificationType,
    NodeOutcome, NodeStatus,
};
pub use jsonrpc_server::{JsonRpcConfig, JsonRpcServer};
pub use orchestrator_registration::OrchestratorRegistration;
pub use socket_config::{SocketConfig, SocketConfigSource};
// pub use songbird_registration::SongbirdRegistration; // REMOVED: Deprecated module removed
pub use tarpc_client::NestGateRpcClient;
pub use tarpc_server::{serve_tarpc, NestGateRpcService};
pub use template_storage::{GraphTemplate, TemplateMetadata, TemplateStorage};
pub use unix_socket_server::JsonRpcUnixServer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify key types are exported
        let _ = DatasetParams::default();
        let _ = NestGateRpcService::new();
        let _ = JsonRpcConfig::default();
    }
}
