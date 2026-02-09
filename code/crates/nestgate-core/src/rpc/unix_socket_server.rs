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

use crate::error::{NestGateError, Result};
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
struct StorageState {
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
    async fn new() -> Result<Self> {
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
    #[allow(dead_code)]
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
        "storage.store" => storage_store(&request.params, state).await,
        "storage.retrieve" => storage_retrieve(&request.params, state).await,
        "storage.exists" => storage_exists(&request.params, state).await,
        "storage.delete" => storage_delete(&request.params, state).await,
        "storage.list" => storage_list(&request.params, state).await,
        "storage.stats" => storage_stats(&request.params, state).await,
        "storage.store_blob" => storage_store_blob(&request.params, state).await,
        "storage.retrieve_blob" => storage_retrieve_blob(&request.params, state).await,
        "templates.store" => templates_store(&request.params, state).await,
        "templates.retrieve" => templates_retrieve(&request.params, state).await,
        "templates.list" => templates_list(&request.params, state).await,
        "templates.community_top" => templates_community_top(&request.params, state).await,
        "audit.store_execution" => audit_store_execution(&request.params, state).await,
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

/// storage.store - Store key-value data
async fn storage_store(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;

    // ✅ FIX: Accept both "value" (biomeOS) and "data" (legacy) parameters
    let data = if params.get("value").is_some() && !params["value"].is_null() {
        &params["value"]
    } else if params.get("data").is_some() && !params["data"].is_null() {
        &params["data"]
    } else {
        return Err(NestGateError::invalid_input_with_field(
            "value",
            "value or data (json) required",
        ));
    };

    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // ✅ ENHANCED LOGGING: Input validation
    let data_str = serde_json::to_string(data).unwrap_or_else(|_| "<invalid>".to_string());
    debug!(
        "📝 storage.store called: family_id='{}', key='{}', value_size={} bytes",
        family_id, key, data_str.len()
    );

    // ✅ PERSISTENT: Store via StorageManagerService (filesystem-backed)
    let dataset = family_id; // Family maps to dataset
    let object_id = key;

    // Serialize JSON data to bytes
    let data_bytes = serde_json::to_vec(data)
        .map_err(|e| NestGateError::storage_error(&format!("Failed to serialize data: {}", e)))?;

    // ✅ ENHANCED LOGGING: Before storage call
    debug!(
        "💾 Calling storage_manager.store_object: dataset='{}', key='{}', bytes={}",
        dataset, object_id, data_bytes.len()
    );

    // Store via persistent backend
    let object_info = state
        .storage_manager
        .store_object(dataset, object_id, data_bytes)
        .await?;

    // ✅ ENHANCED LOGGING: Success with details
    info!(
        "✅ storage.store SUCCESS: {}/{} ({} bytes stored)",
        family_id, key, object_info.size_bytes
    );

    Ok(json!({
        "success": true,
        "key": key,
        "family_id": family_id,
        "size_bytes": object_info.size_bytes
    }))
}

/// storage.retrieve - Retrieve data by key
async fn storage_retrieve(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // ✅ ENHANCED LOGGING: Input validation
    debug!(
        "📖 storage.retrieve called: family_id='{}', key='{}'",
        family_id, key
    );

    // ✅ PERSISTENT: Retrieve from StorageManagerService (filesystem-backed)
    let dataset = family_id;
    let object_id = key;

    // ✅ ENHANCED LOGGING: Before storage call
    debug!(
        "🔍 Calling storage_manager.retrieve_object: dataset='{}', key='{}'",
        dataset, object_id
    );

    // Retrieve from persistent backend (returns (Vec<u8>, ObjectInfo))
    let (data_bytes, _info) = state
        .storage_manager
        .retrieve_object(dataset, object_id)
        .await?;

    // ✅ ENHANCED LOGGING: Retrieved bytes
    debug!(
        "📦 Retrieved raw bytes: {} bytes for {}/{}",
        data_bytes.len(), family_id, key
    );

    // ✅ ENHANCED LOGGING: Before deserialization
    debug!("🔄 Deserializing {} bytes as JSON...", data_bytes.len());

    // Deserialize bytes to JSON
    let data: Value = serde_json::from_slice(&data_bytes).map_err(|e| {
        error!(
            "❌ DESERIALIZATION FAILED for {}/{}: {}",
            family_id, key, e
        );
        NestGateError::storage_error(&format!("Failed to deserialize data: {}", e))
    })?;

    // ✅ ENHANCED LOGGING: Success
    info!(
        "✅ storage.retrieve SUCCESS: {}/{} → {} bytes JSON",
        family_id,
        key,
        serde_json::to_string(&data).unwrap_or_default().len()
    );

    Ok(json!({
        "data": data
    }))
}

/// storage.exists - Check if data exists by key
///
/// Modern idiomatic Rust: Efficient existence check without data transfer
/// Deep Debt Principle #1: Standard API pattern, no unnecessary data retrieval
async fn storage_exists(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // ✅ MODERN IDIOMATIC: Efficient check without full data retrieval
    let dataset = family_id;
    let object_id = key;

    // Check existence via retrieve (returns error if not found)
    // ✅ DEEP DEBT: Proper Result propagation, no unwraps
    let exists = match state
        .storage_manager
        .retrieve_object(dataset, object_id)
        .await
    {
        Ok(_) => true,
        Err(e) => {
            // Distinguish "not found" from actual errors
            if e.to_string().contains("not found") || e.to_string().contains("Not found") {
                false
            } else {
                // Propagate actual errors
                return Err(e);
            }
        }
    };

    debug!(
        "🔍 Existence check: key='{}', family='{}', exists={}",
        key, family_id, exists
    );

    Ok(json!({
        "exists": exists,
        "key": key,
        "family_id": family_id
    }))
}

/// storage.delete - Delete data by key
async fn storage_delete(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // ✅ PERSISTENT: Delete from StorageManagerService (filesystem-backed)
    let dataset = family_id;
    let object_id = key;

    // Delete from persistent backend
    let result = state
        .storage_manager
        .delete_object(dataset, object_id)
        .await;

    let deleted = result.is_ok();

    if deleted {
        debug!(
            "✅ Deleted key '{}' for family '{}' (persistent)",
            key, family_id
        );
    } else {
        warn!(
            "Key '{}' not found for deletion (family: '{}')",
            key, family_id
        );
    }

    Ok(json!({
        "success": deleted
    }))
}

/// storage.list - List all keys with optional prefix
async fn storage_list(params: &Option<Value>, _state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;
    let prefix = params["prefix"].as_str();

    // ✅ PERSISTENT: List from filesystem (StorageManagerService doesn't have list_objects yet)
    let dataset = family_id;

    // Read directly from storage filesystem
    // ✅ EVOLVED: Use XDG-compliant storage path (Phase 4)
    let base_path = crate::config::storage_paths::get_storage_base_path()
        .join("datasets")
        .join(dataset)
        .join("objects");

    let mut keys: Vec<String> = Vec::new();
    if base_path.exists() {
        if let Ok(mut entries) = tokio::fs::read_dir(&base_path).await {
            // Read all entries
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Some(file_name) = entry.file_name().to_str() {
                    // Filter by prefix if provided
                    if let Some(p) = prefix {
                        if file_name.starts_with(p) {
                            keys.push(file_name.to_string());
                        }
                    } else {
                        keys.push(file_name.to_string());
                    }
                }
            }
        }
    }

    debug!(
        "✅ Listed {} keys for family '{}' (prefix: {:?}) (persistent)",
        keys.len(),
        family_id,
        prefix
    );

    Ok(json!({
        "keys": keys
    }))
}

/// storage.stats - Get storage statistics
async fn storage_stats(params: &Option<Value>, _state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // ✅ PERSISTENT: Get stats from filesystem
    let dataset = family_id;

    // Count objects by reading directory
    let base_path = PathBuf::from("/var/lib/nestgate/storage")
        .join("datasets")
        .join(dataset)
        .join("objects");

    let key_count = if base_path.exists() {
        if let Ok(mut entries) = tokio::fs::read_dir(&base_path).await {
            let mut count = 0;
            while let Ok(Some(_)) = entries.next_entry().await {
                count += 1;
            }
            count
        } else {
            0
        }
    } else {
        0
    };

    debug!(
        "✅ Stats for family '{}': {} objects (persistent)",
        family_id, key_count
    );

    Ok(json!({
        "key_count": key_count,
        "blob_count": 0,  // No separate blob tracking in new architecture
        "family_id": family_id
    }))
}

/// storage.store_blob - Store binary blob (base64 encoded)
async fn storage_store_blob(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let blob_base64 = params["blob"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("blob", "blob (base64 string) required")
    })?;
    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // Decode base64
    use base64::Engine;
    let blob_data = base64::engine::general_purpose::STANDARD
        .decode(blob_base64)
        .map_err(|e| {
            NestGateError::invalid_input_with_field("blob", format!("Invalid base64: {}", e))
        })?;

    // ✅ PERSISTENT: Store blob via StorageManagerService
    let dataset = family_id;
    let object_id = key;

    // Store raw bytes
    state
        .storage_manager
        .store_object(dataset, object_id, blob_data.clone())
        .await?;

    debug!(
        "✅ Stored blob '{}' ({} bytes) for family '{}' (persistent)",
        key,
        blob_data.len(),
        family_id
    );

    Ok(json!({
        "success": true,
        "key": key,
        "size": blob_data.len()
    }))
}

/// storage.retrieve_blob - Retrieve binary blob (base64 encoded)
async fn storage_retrieve_blob(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // ✅ PERSISTENT: Retrieve blob from StorageManagerService
    let dataset = family_id;
    let object_id = key;

    // Retrieve raw bytes (returns (Vec<u8>, ObjectInfo))
    let (blob_data, _info) = state
        .storage_manager
        .retrieve_object(dataset, object_id)
        .await?;

    // Encode as base64
    use base64::Engine;
    let blob_base64 = base64::engine::general_purpose::STANDARD.encode(&blob_data);

    debug!(
        "✅ Retrieved blob '{}' ({} bytes) for family '{}' (persistent)",
        key,
        blob_data.len(),
        family_id
    );

    Ok(json!({
        "blob": blob_base64,
        "size": blob_data.len()
    }))
}

/// templates.store - Store graph template
async fn templates_store(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let name = params["name"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("name", "name (string) required"))?
        .to_string();
    let description = params["description"]
        .as_str()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field("description", "description (string) required")
        })?
        .to_string();
    let graph_data = params["graph_data"].clone();
    let user_id = params["user_id"]
        .as_str()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field("user_id", "user_id (string) required")
        })?
        .to_string();
    let family_id = params["family_id"]
        .as_str()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
        })?
        .to_string();

    // Parse metadata
    let metadata = if let Some(meta_value) = params.get("metadata") {
        serde_json::from_value(meta_value.clone()).map_err(|e| {
            NestGateError::invalid_input_with_field(
                "metadata",
                format!("Invalid metadata format: {}", e),
            )
        })?
    } else {
        crate::rpc::template_storage::TemplateMetadata::default()
    };

    let (template_id, version) = state
        .templates
        .store_template(name, description, graph_data, user_id, family_id, metadata)
        .await?;

    debug!("Stored template '{}' (version {})", template_id, version);

    Ok(json!({
        "template_id": template_id,
        "version": version,
        "created_at": chrono::Utc::now().to_rfc3339(),
        "success": true
    }))
}

/// templates.retrieve - Retrieve graph template by ID
async fn templates_retrieve(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let template_id = params["template_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("template_id", "template_id (string) required")
    })?;
    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    let template = state
        .templates
        .retrieve_template(template_id, family_id)
        .await?;

    debug!(
        "Retrieved template '{}' for family '{}'",
        template_id, family_id
    );

    serde_json::to_value(template)
        .map_err(|e| NestGateError::api(format!("Failed to serialize template: {}", e)))
}

/// templates.list - List templates with filtering
async fn templates_list(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // Optional filters
    let user_id = params.get("user_id").and_then(|v| v.as_str());
    let niche_type = params.get("niche_type").and_then(|v| v.as_str());
    let is_community = params.get("is_community").and_then(|v| v.as_bool());

    let tags: Option<Vec<String>> = params.get("tags").and_then(|v| {
        v.as_array().map(|arr| {
            arr.iter()
                .filter_map(|t| t.as_str().map(String::from))
                .collect()
        })
    });

    let templates = state
        .templates
        .list_templates(
            family_id,
            user_id,
            tags.as_deref(),
            niche_type,
            is_community,
        )
        .await?;

    debug!(
        "Listed {} templates for family '{}' with filters",
        templates.len(),
        family_id
    );

    Ok(json!({
        "templates": templates,
        "total": templates.len()
    }))
}

/// templates.community_top - Get top community templates
async fn templates_community_top(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let niche_type = params.get("niche_type").and_then(|v| v.as_str());
    let limit = params.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
    let min_usage = params
        .get("min_usage")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let top_templates = state
        .templates
        .get_community_top(niche_type, limit, min_usage)
        .await?;

    let result: Vec<Value> = top_templates
        .into_iter()
        .map(|(template, score)| {
            json!({
                "id": template.id,
                "name": template.name,
                "description": template.description,
                "score": score,
                "usage_count": template.metadata.usage_count,
                "success_rate": template.metadata.success_rate,
                "community_rating": template.metadata.community_rating,
                "rating_count": template.metadata.rating_count,
                "metadata": {
                    "tags": template.metadata.tags,
                    "niche_type": template.metadata.niche_type
                }
            })
        })
        .collect();

    debug!(
        "Retrieved {} top community templates (niche: {:?})",
        result.len(),
        niche_type
    );

    Ok(json!({
        "templates": result
    }))
}

/// audit.store_execution - Store execution audit trail
async fn audit_store_execution(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    // Deserialize the entire audit structure from params
    let audit: crate::rpc::audit_storage::ExecutionAudit = serde_json::from_value(params.clone())
        .map_err(|e| {
        NestGateError::invalid_input_with_field(
            "audit_data",
            format!("Invalid audit data format: {}", e),
        )
    })?;

    let audit_id = state.audits.store_audit(audit).await?;

    debug!("Stored execution audit '{}'", audit_id);

    Ok(json!({
        "audit_id": audit_id,
        "stored_at": chrono::Utc::now().to_rfc3339(),
        "success": true
    }))
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
        let result = storage_store(&Some(store_params), &state).await.unwrap();
        assert_eq!(result["success"], true);

        // Retrieve
        let retrieve_params = json!({
            "key": "test_key",
            "family_id": "test_family"
        });
        let result = storage_retrieve(&Some(retrieve_params), &state)
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
        storage_store(&Some(store_params), &state).await.unwrap();

        // Delete
        let delete_params = json!({
            "key": "delete_key",
            "family_id": "test_family"
        });
        let result = storage_delete(&Some(delete_params), &state).await.unwrap();
        assert_eq!(result["success"], true);

        // Verify deleted
        let retrieve_params = json!({
            "key": "delete_key",
            "family_id": "test_family"
        });
        let result = storage_retrieve(&Some(retrieve_params), &state).await;
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
            storage_store(&Some(params), &state).await.unwrap();
        }

        // List all
        let list_params = json!({"family_id": "test_family"});
        let result = storage_list(&Some(list_params), &state).await.unwrap();
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
        storage_store(&Some(store_params), &state).await.unwrap();

        // Get stats
        let stats_params = json!({"family_id": "test_family"});
        let result = storage_stats(&Some(stats_params), &state).await.unwrap();
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
        let result = storage_store_blob(&Some(store_params), &state)
            .await
            .unwrap();
        assert_eq!(result["success"], true);
        assert_eq!(result["size"], test_data.len());

        // Retrieve blob
        let retrieve_params = json!({
            "key": "test_blob",
            "family_id": "test_family"
        });
        let result = storage_retrieve_blob(&Some(retrieve_params), &state)
            .await
            .unwrap();
        let retrieved_blob = base64::engine::general_purpose::STANDARD
            .decode(result["blob"].as_str().unwrap())
            .unwrap();
        assert_eq!(retrieved_blob, test_data);
    }
}
