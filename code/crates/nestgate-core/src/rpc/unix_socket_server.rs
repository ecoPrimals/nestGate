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
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
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
#[derive(Debug, Clone)]
struct StorageState {
    /// In-memory storage (family_id -> key -> value)
    /// **LOCK-FREE**: Uses DashMap for concurrent storage access
    storage: Arc<DashMap<String, DashMap<String, Value>>>, // ✅ Nested DashMaps!
    /// Blob storage (family_id -> key -> bytes)
    blobs: Arc<DashMap<String, DashMap<String, Vec<u8>>>>, // ✅ Nested DashMaps!
    /// Template storage for collaborative intelligence
    templates: crate::rpc::template_storage::TemplateStorage,
    /// Audit storage for execution tracking
    audits: crate::rpc::audit_storage::AuditStorage,
}

impl Default for StorageState {
    fn default() -> Self {
        Self {
            storage: Arc::new(DashMap::new()),
            blobs: Arc::new(DashMap::new()),
            templates: crate::rpc::template_storage::TemplateStorage::new(),
            audits: crate::rpc::audit_storage::AuditStorage::new(),
        }
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
    note = "Connection logic moved to Songbird (Universal IPC). Use songbird::ipc::register() instead. See UNIVERSAL_IPC_EVOLUTION_PLAN_JAN_19_2026.md"
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

        // Log configuration for debugging
        socket_config.log_summary();

        // Prepare socket path (create dirs, remove old socket)
        socket_config.prepare_socket_path()?;

        let socket_path = socket_config.socket_path;

        info!("Initializing JSON-RPC Unix socket server");
        info!("  Socket path: {:?}", socket_path);
        info!("  Family ID: {}", family_id);

        Ok(Self {
            socket_path,
            family_id: family_id.to_string(),
            state: StorageState::default(),
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

        info!("🔌 JSON-RPC Unix socket server listening");
        info!("   Ready for biomeOS IPC connections");

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
    let data = &params["data"];
    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // ✅ Lock-free: Get or create family storage, then insert
    let family_storage = state
        .storage
        .entry(family_id.to_string())
        .or_insert_with(DashMap::new);
    family_storage.insert(key.to_string(), data.clone());

    debug!("Stored key '{}' for family '{}'", key, family_id);

    Ok(json!({
        "success": true,
        "key": key
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

    // ✅ Lock-free: Nested get from DashMap
    let data = state
        .storage
        .get(family_id)
        .and_then(|family_storage| family_storage.get(key).map(|v| v.clone()))
        .ok_or_else(|| NestGateError::not_found(format!("Key '{}' not found", key)))?;

    debug!("Retrieved key '{}' for family '{}'", key, family_id);

    Ok(json!({
        "data": data
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

    // ✅ Lock-free: Remove from nested DashMap
    let deleted = state
        .storage
        .get(family_id)
        .and_then(|family_storage| family_storage.remove(key))
        .is_some();

    if deleted {
        debug!("Deleted key '{}' for family '{}'", key, family_id);
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
async fn storage_list(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;
    let prefix = params["prefix"].as_str();

    // ✅ Lock-free: Iterate keys from nested DashMap
    let keys: Vec<String> = state
        .storage
        .get(family_id)
        .map(|family_storage| {
            family_storage
                .iter()
                .map(|entry| entry.key().clone())
                .filter(|k| prefix.is_none_or(|p| k.starts_with(p)))
                .collect()
        })
        .unwrap_or_default();

    debug!(
        "Listed {} keys for family '{}' (prefix: {:?})",
        keys.len(),
        family_id,
        prefix
    );

    Ok(json!({
        "keys": keys
    }))
}

/// storage.stats - Get storage statistics
async fn storage_stats(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // ✅ Lock-free: Get counts from DashMaps (no read locks needed!)
    let key_count = state.storage.get(family_id).map(|s| s.len()).unwrap_or(0);
    let blob_count = state.blobs.get(family_id).map(|b| b.len()).unwrap_or(0);

    debug!(
        "Stats for family '{}': {} keys, {} blobs",
        family_id, key_count, blob_count
    );

    Ok(json!({
        "key_count": key_count,
        "blob_count": blob_count,
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

    // ✅ Lock-free: Store blob in nested DashMap
    let family_blobs = state
        .blobs
        .entry(family_id.to_string())
        .or_insert_with(DashMap::new);
    family_blobs.insert(key.to_string(), blob_data.clone());

    debug!(
        "Stored blob '{}' ({} bytes) for family '{}'",
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

    // ✅ Lock-free: Get blob from nested DashMap
    let blob_data = state
        .blobs
        .get(family_id)
        .and_then(|family_blobs| family_blobs.get(key).map(|v| v.clone()))
        .ok_or_else(|| NestGateError::not_found(format!("Blob '{}' not found", key)))?;

    // Encode as base64
    use base64::Engine;
    let blob_base64 = base64::engine::general_purpose::STANDARD.encode(&blob_data);

    debug!(
        "Retrieved blob '{}' ({} bytes) for family '{}'",
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
    async fn test_storage_store_retrieve() {
        let state = StorageState::default();

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
    async fn test_storage_delete() {
        let state = StorageState::default();

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
    async fn test_storage_list() {
        let state = StorageState::default();

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
    async fn test_storage_stats() {
        let state = StorageState::default();

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
    async fn test_blob_storage() {
        let state = StorageState::default();

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
