//! # 🔌 JSON-RPC Unix Socket Server
//!
//! **⚠️ DEPRECATED**: This module is deprecated as of v2.3.0
//!
//! ## Migration to Universal IPC Architecture
//!
//! **Connection logic has moved to Songbird** (Universal IPC Layer)
//!
//! ### Why This Change?
//!
//! - **Separation of Concerns**: NestGate = Storage, Songbird = Communication
//! - **True Universality**: Songbird abstracts platform differences (Unix/Windows/etc.)
//! - **Single Responsibility**: Each primal owns its domain
//!
//! ### Migration Path
//!
//! **Before (NestGate Unix sockets)**:
//! ```rust,ignore
//! use nestgate_core::rpc::JsonRpcUnixServer;
//!
//! let server = JsonRpcUnixServer::new("myservice").await?;
//! server.serve().await?;
//! ```
//!
//! **After (Songbird Universal IPC)**:
//! ```rust,ignore
//! use songbird::ipc;
//!
//! // Register with Songbird (works on ALL platforms!)
//! let endpoint = ipc::register("myservice").await?;
//! ipc::listen(endpoint).await?;
//!
//! // Songbird stores metadata in NestGate automatically
//! ```
//!
//! ### What NestGate Still Provides
//!
//! - ✅ Service metadata storage (`service_metadata` module)
//! - ✅ Capability-based discovery
//! - ✅ Persistent service registry
//!
//! ### References
//!
//! - `UNIVERSAL_IPC_ARCHITECTURE_HANDOFF_JAN_19_2026.md`
//! - `UNIVERSAL_IPC_EVOLUTION_PLAN_JAN_19_2026.md`
//! - `code/crates/nestgate-core/src/service_metadata/mod.rs`
//!
//! ---
//!
//! ## Legacy Documentation (Deprecated)
//!
//! **biomeOS IPC Integration** - Native Unix socket communication
//!
//! Implements JSON-RPC 2.0 server over Unix sockets for efficient
//! primal-to-primal communication within the biomeOS ecosystem.
//!
//! ## Philosophy
//! - **Self-Knowledge**: Socket path from own environment ($NESTGATE_FAMILY_ID)
//! - **Runtime Discovery**: Discover Songbird via capability system
//! - **Zero Hardcoding**: All configuration from environment
//! - **Memory Safe**: Zero unsafe blocks
//! - **Modern Async**: Native async/await with tokio
//!
//! ## Socket Path Pattern
//! ```text
//! /run/user/{uid}/nestgate-{family_id}.sock
//! ```
//!
//! ## Environment Variables
//! - `NESTGATE_FAMILY_ID` (required): Family identifier for socket path
//! - `SONGBIRD_FAMILY_ID` (optional): For auto-registration
//!
//! ## Usage (Deprecated)
//! ```no_run
//! use nestgate_core::rpc::unix_socket_server::JsonRpcUnixServer;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let family_id = std::env::var("NESTGATE_FAMILY_ID")?;
//! let server = JsonRpcUnixServer::new(&family_id).await?;
//! server.serve().await?;
//! # Ok(())
//! # }
//! ```

mod audit_handlers;
mod storage_handlers;
mod template_handlers;

use crate::error::{NestGateError, Result};
use crate::rpc::model_cache_handlers;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tracing::{debug, error, info, warn};

/// JSON-RPC 2.0 Request
#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<Value>,
    id: Option<Value>,
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
    id: Option<Value>,
}

/// JSON-RPC 2.0 Error
#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

/// Storage service state
#[derive(Clone)]
pub(crate) struct StorageState {
    /// Persistent storage manager (filesystem-backed)
    /// **PRODUCTION**: Uses StorageManagerService for persistent storage
    storage_manager: Arc<crate::services::storage::StorageManagerService>,
    /// Template storage for collaborative intelligence
    templates: crate::rpc::template_storage::TemplateStorage,
    /// Audit storage for execution tracking
    audits: crate::rpc::audit_storage::AuditStorage,
}

impl StorageState {
    /// Create new storage state with persistent backend
    pub(crate) async fn new() -> Result<Self> {
        let storage_manager = Arc::new(
            crate::services::storage::StorageManagerService::new()
                .await
                .map_err(|e| {
                    warn!("Failed to initialize storage manager: {}", e);
                    e
                })?,
        );

        Ok(Self {
            storage_manager,
            templates: crate::rpc::template_storage::TemplateStorage::new(),
            audits: crate::rpc::audit_storage::AuditStorage::new(),
        })
    }
}

/// JSON-RPC Unix socket server for biomeOS integration
///
/// **⚠️ DEPRECATED**: Use `songbird::ipc` instead (Universal IPC Architecture)
///
/// Connection logic has moved to Songbird for true platform universality.
/// See `UNIVERSAL_IPC_EVOLUTION_PLAN_JAN_19_2026.md` for migration guide.
#[deprecated(
    since = "2.3.0",
    note = "Connection logic moved to Songbird IPC SERVICE. \
            Call /primal/songbird via JSON-RPC - DO NOT import songbird code! \
            See UNIVERSAL_IPC_EVOLUTION_PLAN_JAN_19_2026.md for service-based integration."
)]
pub struct JsonRpcUnixServer {
    socket_path: PathBuf,
    /// Family ID for primal identification (used in future multi-primal features)
    family_id: String,
    state: StorageState,
}

impl JsonRpcUnixServer {
    /// Create new Unix socket server with standardized configuration
    ///
    /// # Configuration Priority (3-tier fallback)
    ///
    /// 1. `NESTGATE_SOCKET` env var (explicit override)
    /// 2. XDG runtime: `/run/user/{uid}/nestgate-{family}.sock` (recommended)
    /// 3. Temp fallback: `/tmp/nestgate-{family}-{node}.sock` (least secure)
    ///
    /// # Self-Knowledge Principle
    /// - Socket path derived from environment and own identity
    /// - Agnostic to deployment environment
    /// - Buildable (creates directories, cleans old sockets)
    /// - No hardcoding or assumptions
    ///
    /// # Arguments
    /// - `family_id`: Family identifier from $NESTGATE_FAMILY_ID
    ///
    /// # Errors
    /// - Returns error if socket path cannot be prepared
    /// - Returns error if socket binding fails
    pub async fn new(family_id: &str) -> Result<Self> {
        // Use standardized socket configuration
        let socket_config = crate::rpc::socket_config::SocketConfig::from_environment()?;

        // Log configuration before preparing
        info!("═══════════════════════════════════════════════════════════");
        info!("🏰 NestGate JSON-RPC Unix Socket Server");
        info!("═══════════════════════════════════════════════════════════");
        socket_config.log_summary();

        // Prepare socket path (create dirs, remove old socket)
        socket_config.prepare_socket_path()?;

        let socket_path = socket_config.socket_path;

        // Initialize persistent storage backend
        info!("📦 Initializing persistent storage backend...");
        let state = StorageState::new().await?;
        info!("✅ Storage backend initialized");

        Ok(Self {
            socket_path,
            family_id: family_id.to_string(),
            state,
        })
    }

    /// Start serving requests
    ///
    /// Binds to Unix socket and processes JSON-RPC 2.0 requests
    /// indefinitely. Each connection is handled concurrently.
    pub async fn serve(&self) -> Result<()> {
        let listener = UnixListener::bind(&self.socket_path).map_err(|e| {
            NestGateError::configuration_error(
                "socket_bind",
                &format!("Failed to bind Unix socket: {}", e),
            )
        })?;

        info!("═══════════════════════════════════════════════════════════");
        info!("✅ NestGate ready!");
        info!("   Socket: {}", self.socket_path.display());
        info!("   Family: {}", self.family_id);
        info!("   Protocol: JSON-RPC 2.0 over Unix socket");
        info!("═══════════════════════════════════════════════════════════");
        info!("💡 Test with: echo '{{\"jsonrpc\":\"2.0\",\"method\":\"storage.list\",\"id\":1}}' | nc -U {}", self.socket_path.display());

        let state = Arc::new(self.state.clone());

        loop {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    let state = Arc::clone(&state);
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream, state).await {
                            error!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    /// Get socket path (for testing)
    pub fn socket_path(&self) -> &PathBuf {
        &self.socket_path
    }
}

impl Drop for JsonRpcUnixServer {
    fn drop(&mut self) {
        // Clean up socket file
        if self.socket_path.exists() {
            if let Err(e) = std::fs::remove_file(&self.socket_path) {
                warn!("Failed to remove socket file: {}", e);
            }
        }
    }
}

/// Handle a single Unix socket connection
async fn handle_connection(stream: UnixStream, state: Arc<StorageState>) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader
            .read_line(&mut line)
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to read request: {}", e)))?;

        if bytes_read == 0 {
            // Connection closed
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        debug!("Received request: {}", trimmed);

        // Parse and handle request
        let response = match serde_json::from_str::<JsonRpcRequest>(trimmed) {
            Ok(request) => handle_request(request, &state).await,
            Err(e) => {
                error!("Failed to parse JSON-RPC request: {}", e);
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: "Parse error".to_string(),
                        data: Some(json!({"error": e.to_string()})),
                    }),
                    id: None,
                }
            }
        };

        // Send response
        let response_json = serde_json::to_string(&response)
            .map_err(|e| NestGateError::api(format!("Failed to serialize response: {}", e)))?;

        writer
            .write_all(response_json.as_bytes())
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to write response: {}", e)))?;
        writer
            .write_all(b"\n")
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to write newline: {}", e)))?;

        debug!("Sent response: {}", response_json);
    }

    Ok(())
}

/// Handle JSON-RPC request
async fn handle_request(request: JsonRpcRequest, state: &StorageState) -> JsonRpcResponse {
    if request.jsonrpc != "2.0" {
        return JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32600,
                message: "Invalid Request".to_string(),
                data: Some(json!({"error": "Only JSON-RPC 2.0 is supported"})),
            }),
            id: request.id,
        };
    }

    let result = match request.method.as_str() {
        // Health & Discovery
        "health" => Ok(json!({"status": "healthy", "version": env!("CARGO_PKG_VERSION")})),
        "discover_capabilities" => model_cache_handlers::discover_capabilities().await,
        // Storage operations
        "storage.store" => storage_handlers::storage_store(&request.params, state).await,
        "storage.retrieve" => storage_handlers::storage_retrieve(&request.params, state).await,
        "storage.exists" => storage_handlers::storage_exists(&request.params, state).await,
        "storage.delete" => storage_handlers::storage_delete(&request.params, state).await,
        "storage.list" => storage_handlers::storage_list(&request.params, state).await,
        "storage.stats" => storage_handlers::storage_stats(&request.params, state).await,
        "storage.store_blob" => storage_handlers::storage_store_blob(&request.params, state).await,
        "storage.retrieve_blob" => {
            storage_handlers::storage_retrieve_blob(&request.params, state).await
        }
        // Model cache operations (extracted to model_cache_handlers.rs)
        "model.register" => {
            model_cache_handlers::model_register(&request.params, &state.storage_manager).await
        }
        "model.exists" => {
            model_cache_handlers::model_exists(&request.params, &state.storage_manager).await
        }
        "model.locate" => {
            model_cache_handlers::model_locate(&request.params, &state.storage_manager).await
        }
        "model.metadata" => {
            model_cache_handlers::model_metadata(&request.params, &state.storage_manager).await
        }
        // Template operations
        "templates.store" => template_handlers::templates_store(&request.params, state).await,
        "templates.retrieve" => template_handlers::templates_retrieve(&request.params, state).await,
        "templates.list" => template_handlers::templates_list(&request.params, state).await,
        "templates.community_top" => {
            template_handlers::templates_community_top(&request.params, state).await
        }
        // Audit operations
        "audit.store_execution" => {
            audit_handlers::audit_store_execution(&request.params, state).await
        }
        _ => {
            return JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: Some(json!({"method": request.method})),
                }),
                id: request.id,
            };
        }
    };

    match result {
        Ok(value) => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(value),
            error: None,
            id: request.id,
        },
        Err(e) => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32603,
                message: "Internal error".to_string(),
                data: Some(json!({"error": e.to_string()})),
            }),
            id: request.id,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Requires write permissions to /var/lib/nestgate/storage - run as integration test"]
    async fn test_storage_store_retrieve() {
        let state = StorageState::new()
            .await
            .expect("Failed to create test storage state");

        // Store
        let store_params = json!({
            "key": "test_key",
            "data": {"value": "test_data"},
            "family_id": "test_family"
        });
        let result = storage_handlers::storage_store(&Some(store_params), &state)
            .await
            .unwrap();
        assert_eq!(result["success"], true);

        // Retrieve
        let retrieve_params = json!({
            "key": "test_key",
            "family_id": "test_family"
        });
        let result = storage_handlers::storage_retrieve(&Some(retrieve_params), &state)
            .await
            .unwrap();
        assert_eq!(result["data"]["value"], "test_data");
    }

    #[tokio::test]
    #[ignore = "Requires write permissions to /var/lib/nestgate/storage - run as integration test"]
    async fn test_storage_delete() {
        let state = StorageState::new()
            .await
            .expect("Failed to create test storage state");

        // Store
        let store_params = json!({
            "key": "delete_key",
            "data": {"value": "delete_me"},
            "family_id": "test_family"
        });
        storage_handlers::storage_store(&Some(store_params), &state)
            .await
            .unwrap();

        // Delete
        let delete_params = json!({
            "key": "delete_key",
            "family_id": "test_family"
        });
        let result = storage_handlers::storage_delete(&Some(delete_params), &state)
            .await
            .unwrap();
        assert_eq!(result["success"], true);

        // Verify deleted
        let retrieve_params = json!({
            "key": "delete_key",
            "family_id": "test_family"
        });
        let result = storage_handlers::storage_retrieve(&Some(retrieve_params), &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore = "Requires write permissions to /var/lib/nestgate/storage - run as integration test"]
    async fn test_storage_list() {
        let state = StorageState::new()
            .await
            .expect("Failed to create test storage state");

        // Store multiple keys
        for i in 0..5 {
            let params = json!({
                "key": format!("key_{}", i),
                "data": {"index": i},
                "family_id": "test_family"
            });
            storage_handlers::storage_store(&Some(params), &state)
                .await
                .unwrap();
        }

        // List all
        let list_params = json!({"family_id": "test_family"});
        let result = storage_handlers::storage_list(&Some(list_params), &state)
            .await
            .unwrap();
        assert_eq!(result["keys"].as_array().unwrap().len(), 5);
    }

    #[tokio::test]
    #[ignore = "Requires write permissions to /var/lib/nestgate/storage - run as integration test"]
    async fn test_storage_stats() {
        let state = StorageState::new()
            .await
            .expect("Failed to create test storage state");

        // Store some data
        let store_params = json!({
            "key": "stats_key",
            "data": {"value": "stats"},
            "family_id": "test_family"
        });
        storage_handlers::storage_store(&Some(store_params), &state)
            .await
            .unwrap();

        // Get stats
        let stats_params = json!({"family_id": "test_family"});
        let result = storage_handlers::storage_stats(&Some(stats_params), &state)
            .await
            .unwrap();
        assert_eq!(result["key_count"], 1);
        assert_eq!(result["blob_count"], 0);
    }

    #[tokio::test]
    #[ignore = "Requires write permissions to /var/lib/nestgate/storage - run as integration test"]
    async fn test_blob_storage() {
        let state = StorageState::new()
            .await
            .expect("Failed to create test storage state");

        // Store blob
        let test_data = b"Hello, World!";
        use base64::Engine;
        let blob_base64 = base64::engine::general_purpose::STANDARD.encode(test_data);

        let store_params = json!({
            "key": "test_blob",
            "blob": blob_base64,
            "family_id": "test_family"
        });
        let result = storage_handlers::storage_store_blob(&Some(store_params), &state)
            .await
            .unwrap();
        assert_eq!(result["success"], true);
        assert_eq!(result["size"], test_data.len());

        // Retrieve blob
        let retrieve_params = json!({
            "key": "test_blob",
            "family_id": "test_family"
        });
        let result = storage_handlers::storage_retrieve_blob(&Some(retrieve_params), &state)
            .await
            .unwrap();
        let retrieved_blob = base64::engine::general_purpose::STANDARD
            .decode(result["blob"].as_str().unwrap())
            .unwrap();
        assert_eq!(retrieved_blob, test_data);
    }
}
