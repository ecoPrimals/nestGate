//! # 🚀 RPC Module for NestGate
//!
//! **HIGH-PERFORMANCE PRIMAL-TO-PRIMAL COMMUNICATION** (v0.3.0)
//!
//! Provides tarpc, JSON-RPC, and **isomorphic IPC** interfaces for NestGate storage operations.
//!
//! ## Protocol Priority (Ecosystem Standard)
//! 1. **Isomorphic IPC** (NEW, OPTIMAL) - Unix socket OR TCP, auto-adaptive (~5-10μs)
//! 2. **tarpc** (PRIMARY) - High-performance binary RPC for primal-to-primal (~10-20μs)
//! 3. **JSON-RPC** (SECONDARY) - Universal, human-friendly (~50-100μs)
//! 4. **HTTP** (FALLBACK) - Enableable for network scenarios (~500-1000μs)
//!
//! ## NEW: Isomorphic IPC (v0.3.0)
//!
//! **Try→Detect→Adapt→Succeed** pattern for universal IPC:
//! - Tries Unix sockets first (optimal)
//! - Detects platform constraints (SELinux, lack of support)
//! - Adapts to TCP fallback (automatic)
//! - Works on ALL platforms (Linux, Android, etc.)
//!
//! See `isomorphic_ipc` module for details.
//!
//! ## Philosophy (Primal Sovereignty)
//! - **Self-knowledge**: NestGate exposes only storage capabilities
//! - **Runtime discovery**: Other primals discovered via capability
//! - **Zero hardcoding**: No primal names, ports, or endpoints
//! - **Zero unsafe blocks**: Memory-safe throughout
//! - **Modern async**: Native async/await patterns
//! - **Platform-agnostic**: Automatic adaptation to constraints
//!
//! ## Usage
//!
//! ### Server
//! ```no_run
//! use nestgate_core::rpc::{NestGateRpcService, serve_tarpc};
//! use std::net::SocketAddr;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let service = NestGateRpcService::new().await.expect("Failed to create service");
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
pub mod semantic_router;
pub mod socket_config;
// songbird_registration removed in v2.3.0
/// Model cache and discovery JSON-RPC handlers (smart refactoring extract)
pub(crate) mod model_cache_handlers;
pub mod tarpc_client;
pub mod tarpc_server;
pub mod tarpc_types;
pub mod template_storage;
pub mod unix_socket_server;

// NEW: Isomorphic IPC (v0.3.0) - Universal, adaptive IPC (Try→Detect→Adapt→Succeed)
pub mod isomorphic_ipc;

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
pub use semantic_router::SemanticRouter;
pub use socket_config::{SocketConfig, SocketConfigSource};
// pub use songbird_registration::SongbirdRegistration; // REMOVED: Deprecated module removed
pub use tarpc_client::NestGateRpcClient;
pub use tarpc_server::{serve_tarpc, NestGateRpcService};
pub use template_storage::{GraphTemplate, TemplateMetadata, TemplateStorage};
pub use unix_socket_server::JsonRpcUnixServer;

// NEW: Isomorphic IPC exports (v0.3.0)
pub use isomorphic_ipc::{
    connect_endpoint, discover_ipc_endpoint, is_platform_constraint, IpcEndpoint, IpcStream,
    IsomorphicIpcServer, RpcHandler, TcpFallbackServer, UnixSocketRpcHandler,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify key types are exported
        let _ = DatasetParams::default();
        // Test removed - new() is now async
        let _ = JsonRpcConfig::default();
    }
}
