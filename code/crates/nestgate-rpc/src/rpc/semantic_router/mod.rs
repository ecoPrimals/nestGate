// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Semantic Method Router - TRUE PRIMAL Compliance
//!
//! **Routes semantic method names to internal implementations**
//!
//! ## Philosophy
//!
//! - **TRUE PRIMAL**: External primals use semantic names (`storage.put`)
//! - **Internal Flexibility**: Internal code uses descriptive names (`store_object`)
//! - **Zero Breaking Changes**: Existing code continues to work
//! - **Neural API Ready**: the ecosystem orchestration layer can route by capability
//!
//! ## Architecture
//!
//! ```text
//! External Primal
//!   ↓
//! "storage.put" (semantic)
//!   ↓
//! SemanticRouter::route()
//!   ↓
//! NestGateRpcService::store_object() (internal)
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! use nestgate_core::rpc::SemanticRouter;
//! use serde_json::json;
//!
//! let router = SemanticRouter::new(service)?;
//!
//! // External primal calls with semantic name
//! let result = router.call_method("storage.put", json!({
//!     "dataset": "my-dataset",
//!     "key": "my-key",
//!     "data": "base64-encoded-data"
//! })).await?;
//! ```
//!
//! ## Semantic Methods Supported
//!
//! ### Storage Domain (`storage.*`)
//! - `storage.put` → `store_object`
//! - `storage.get` → `retrieve_object`
//! - `storage.delete` → `delete_object`
//! - `storage.list` → `list_objects`
//! - `storage.dataset.create` → `create_dataset`
//! - `storage.dataset.get` → `get_dataset`
//! - `storage.dataset.list` → `list_datasets`
//! - `storage.dataset.delete` → `delete_dataset`
//!
//!
//! ### Discovery Domain (`discovery.*`)
//! - `discovery.announce` → register service metadata
//! - `discovery.query` → find services by capability
//! - `discovery.list` → list all services
//! - `discovery.capabilities` → get own capabilities
//!
//! ### Metadata Domain (`metadata.*`)
//! - `metadata.store` → store service metadata
//! - `metadata.retrieve` → get service metadata by name
//! - `metadata.search` → search services by capability
//!
//! ### Crypto Domain (`crypto.*`)
//! - `crypto.*` → delegate to whichever primal advertises `crypto` / security capabilities
//!   (resolved at runtime via capability discovery, not by name)
//!
//! ### Health Domain (`health.*`)
//! - `health.check` → `health_check`
//! - `health.liveness` → minimal process-alive signal
//! - `health.readiness` → readiness to serve
//! - `health.metrics` → `get_metrics`
//! - `health.info` → `get_info`
//!
//! ### Capabilities Domain (`capabilities.*`)
//! - `capabilities.list` → supported semantic method names
//!
//! ### Session Domain (`session.*`)
//! - `session.save` → persist session state (storage + metadata index)
//! - `session.load` → load session JSON by id
//! - `session.list` → list session index entries (`session/` prefix in metadata)
//! - `session.delete` → remove session blob and index entry
//!
//! ## References
//!
//! - `wateringHole/SEMANTIC_METHOD_NAMING_STANDARD.md` v2.0
//! - `wateringHole/PRIMAL_IPC_PROTOCOL.md` v1.0
//! - `CAPABILITY_MAPPINGS.md`

use crate::rpc::NestGateRpcClient;
use crate::rpc::metadata_backend::{
    DefaultMetadataBackend, FileMetadataBackend, InMemoryMetadataBackend, MetadataBackend,
    default_metadata_base_dir,
};
use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;
use std::sync::Arc;
use tracing::{debug, warn};

// Domain modules
pub mod capabilities;
pub mod crypto;
pub mod discovery;
pub mod health;
pub mod metadata;
pub mod session;
pub mod storage;

/// Semantic method router for TRUE PRIMAL compliance
///
/// Routes semantic method names (e.g., `storage.put`) to internal
/// implementations, enabling Neural API integration and capability-based
/// discovery. Generic over the [`MetadataBackend`] implementation.
pub struct SemanticRouter<M: MetadataBackend = DefaultMetadataBackend> {
    /// Internal RPC client for delegation (storage, health, etc.)
    pub(crate) client: Arc<NestGateRpcClient>,
    /// Metadata backend for service registration / lookup
    pub(crate) metadata: Arc<M>,
}

impl SemanticRouter {
    /// Create new semantic router with the default file-backed metadata backend
    /// ([`FileMetadataBackend`] under [`default_metadata_base_dir`]).
    ///
    /// **NG-01 compliance**: `FileMetadataBackend` is the production default.
    /// When `FAMILY_ID` is set (production mode), the file backend is mandatory
    /// and this constructor returns an error if the directory cannot be created.
    /// In development mode (no `FAMILY_ID` / `BIOMEOS_INSECURE=1`), an in-memory
    /// fallback is used with a warning.
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`] in production mode when `FileMetadataBackend`
    /// cannot be initialized (e.g. permissions, disk full).
    pub fn new(
        client: Arc<NestGateRpcClient>,
    ) -> std::result::Result<Self, nestgate_types::error::NestGateError> {
        debug!("Creating semantic method router");
        let metadata: DefaultMetadataBackend = match FileMetadataBackend::new(
            default_metadata_base_dir(),
        ) {
            Ok(b) => DefaultMetadataBackend::File(b),
            Err(e) => {
                let is_production = std::env::var("FAMILY_ID")
                    .or_else(|_| std::env::var("BIOMEOS_FAMILY_ID"))
                    .or_else(|_| std::env::var("NESTGATE_FAMILY_ID"))
                    .is_ok_and(|fid| !matches!(fid.as_str(), "" | "default" | "standalone"));

                if is_production {
                    return Err(nestgate_types::error::NestGateError::storage_error(
                        format!(
                            "NG-01: file metadata backend required in production (FAMILY_ID set) \
                             but initialization failed at {}: {e}",
                            default_metadata_base_dir().display()
                        ),
                    ));
                }

                warn!(
                    error = %e,
                    "file metadata backend unavailable; using in-memory metadata (lost on restart)"
                );
                DefaultMetadataBackend::InMemory(InMemoryMetadataBackend::new())
            }
        };
        Ok(Self {
            client,
            metadata: Arc::new(metadata),
        })
    }
}

impl<M: MetadataBackend> SemanticRouter<M> {
    /// Create a semantic router with a custom metadata backend.
    ///
    /// Use this at daemon startup to inject `nestgate-core`'s
    /// `ServiceMetadataStore`-backed implementation.
    pub fn with_metadata_backend(client: Arc<NestGateRpcClient>, metadata: Arc<M>) -> Self {
        debug!("Creating semantic method router (custom metadata backend)");
        Self { client, metadata }
    }

    /// Route semantic method call to internal implementation
    ///
    /// # Arguments
    /// * `method` - Semantic method name (e.g., "storage.put")
    /// * `params` - Method parameters as JSON value
    ///
    /// # Returns
    /// Result value from the internal method
    ///
    /// # Errors
    /// Returns error if:
    /// - Method not found
    /// - Invalid parameters
    /// - Internal method fails
    ///
    /// # Example
    /// ```rust,ignore
    /// use nestgate_core::rpc::SemanticRouter;
    /// use serde_json::json;
    ///
    /// # async fn example(router: SemanticRouter) -> std::result::Result<(), nestgate_types::NestGateError> {
    /// let result = router.call_method("storage.put", json!({
    ///     "dataset": "my-dataset",
    ///     "key": "my-key",
    ///     "data": "aGVsbG8gd29ybGQ=" // base64("hello world")
    /// })).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn call_method(&self, method: &str, params: Value) -> Result<Value> {
        debug!("Routing semantic method: {}", method);

        match method {
            // ==================== STORAGE DOMAIN ====================
            "storage.put" => storage::storage_put(self, params).await,
            "storage.get" => storage::storage_get(self, params).await,
            "storage.delete" => storage::storage_delete(self, params).await,
            "storage.list" => storage::storage_list(self, params).await,
            "storage.exists" => storage::storage_exists(self, params).await,
            "storage.metadata" => storage::storage_metadata(self, params).await,

            // Dataset operations
            "storage.dataset.create" => storage::dataset_create(self, params).await,
            "storage.dataset.get" => storage::dataset_get(self, params).await,
            "storage.dataset.list" => storage::dataset_list(self, params).await,
            "storage.dataset.delete" => storage::dataset_delete(self, params).await,

            // Blob / streaming operations (filesystem-backed, not tarpc-delegated)
            "storage.store_blob" => storage::storage_store_blob(self, params).await,
            "storage.retrieve_blob" => storage::storage_retrieve_blob(self, params).await,
            "storage.retrieve_range" => storage::storage_retrieve_range(self, params).await,
            "storage.store_stream" => storage::storage_store_stream(self, params).await,
            "storage.store_stream_chunk" => storage::storage_store_stream_chunk(self, params).await,
            "storage.retrieve_stream" => storage::storage_retrieve_stream(self, params).await,
            "storage.retrieve_stream_chunk" => {
                storage::storage_retrieve_stream_chunk(self, params).await
            }
            "storage.object.size" => storage::storage_object_size(self, params).await,
            "storage.namespaces.list" => storage::storage_namespaces_list(self, params).await,

            // ==================== DISCOVERY DOMAIN ====================
            "discovery.announce" => discovery::discovery_announce(self, &params),
            "discovery.query" => discovery::discovery_query(self, &params),
            "discovery.list" => discovery::discovery_list(self, &params),
            "discovery.capabilities" => discovery::discovery_capabilities(self, &params),

            // ==================== HEALTH DOMAIN ====================
            "health.check" => health::health_check(self, params).await,
            "health.liveness" => health::health_liveness(self, params).await,
            "health.readiness" => health::health_readiness(self, params).await,
            "health.metrics" => health::health_metrics(self, params).await,
            "health.info" => health::health_info(self, params).await,

            // ==================== CAPABILITIES DOMAIN ====================
            "capabilities.list" => capabilities::capabilities_list(self, params),

            // ==================== METADATA DOMAIN ====================
            "metadata.store" => metadata::metadata_store(self, params).await,
            "metadata.retrieve" => metadata::metadata_retrieve(self, params).await,
            "metadata.search" => metadata::metadata_search(self, params).await,

            // ==================== CRYPTO DOMAIN ====================
            "crypto.encrypt" => crypto::crypto_encrypt(self, params),
            "crypto.decrypt" => crypto::crypto_decrypt(self, params),
            "crypto.generate_key" => crypto::crypto_generate_key(self, params),
            "crypto.generate_nonce" => crypto::crypto_generate_nonce(self, params),
            "crypto.hash" => crypto::crypto_hash(self, params),
            "crypto.verify_hash" => crypto::crypto_verify_hash(self, params),

            // ==================== SESSION DOMAIN ====================
            "session.save" => session::session_save(self, params).await,
            "session.load" => session::session_load(self, params).await,
            "session.list" => session::session_list(self, params).await,
            "session.delete" => session::session_delete(self, params).await,

            // Unknown method
            _ => {
                warn!("Unknown semantic method: {}", method);
                Err(NestGateError::not_found(format!(
                    "semantic method `{method}`"
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for semantic router (`call_method` dispatch table and helpers).

    use super::SemanticRouter;
    use crate::rpc::metadata_backend::{DefaultMetadataBackend, InMemoryMetadataBackend};
    use crate::rpc::{NestGateRpcClient, NestGateRpcService, serve_tarpc};
    use nestgate_types::error::NestGateError;
    use serde_json::json;
    use std::net::SocketAddr;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_semantic_method_names() {
        let storage_methods = vec![
            "storage.put",
            "storage.get",
            "storage.delete",
            "storage.list",
            "storage.dataset.create",
        ];

        for method in storage_methods {
            assert!(
                method.contains('.'),
                "Method should use dot notation: {}",
                method
            );
            assert!(
                method.starts_with("storage."),
                "Storage method should start with storage.: {}",
                method
            );
        }
    }

    fn test_router() -> SemanticRouter {
        let client = match NestGateRpcClient::new("tarpc://127.0.0.1:65534") {
            Ok(c) => Arc::new(c),
            Err(e) => panic!("client: {e}"),
        };
        SemanticRouter::with_metadata_backend(
            client,
            Arc::new(DefaultMetadataBackend::InMemory(
                InMemoryMetadataBackend::new(),
            )),
        )
    }

    #[tokio::test]
    async fn capabilities_list_includes_self_and_storage_methods() {
        let router = test_router();
        let v = match router.call_method("capabilities.list", json!({})).await {
            Ok(x) => x,
            Err(e) => panic!("capabilities.list: {e}"),
        };
        let methods = match v["methods"].as_array() {
            Some(a) => a,
            None => panic!("methods array"),
        };
        assert!(methods.iter().any(|m| m == "capabilities.list"));
        assert!(methods.iter().any(|m| m == "storage.put"));
        assert!(
            methods
                .iter()
                .filter_map(|m| m.as_str())
                .all(|s| !s.starts_with("data.")),
            "NestGate must not advertise data.* in capabilities.list (storage primal; data is delegated)"
        );
    }

    #[tokio::test]
    async fn crypto_methods_return_not_implemented() {
        let router = test_router();
        for method in [
            "crypto.encrypt",
            "crypto.decrypt",
            "crypto.generate_key",
            "crypto.generate_nonce",
            "crypto.hash",
            "crypto.verify_hash",
        ] {
            let err = match router.call_method(method, json!({})).await {
                Ok(_) => panic!("expected error for {method}"),
                Err(e) => e,
            };
            match err {
                NestGateError::NotImplemented(_) => {}
                other => panic!("expected NotImplemented for {method}, got {other:?}"),
            }
        }
    }

    #[tokio::test]
    async fn data_prefixed_methods_are_unknown_semantic_methods() {
        let router = test_router();
        for method in [
            "data.ncbi_search",
            "data.ncbi_fetch",
            "data.noaa_ghcnd",
            "data.iris_stations",
            "data.iris_events",
            "data.anything_future",
            "data.unknown_provider",
        ] {
            let err = match router.call_method(method, json!({})).await {
                Ok(_) => panic!("expected error for {method}"),
                Err(e) => e,
            };
            let NestGateError::Api(details) = err else {
                panic!("expected Api error for unknown {method}, got {err:?}");
            };
            assert_eq!(details.status_code, Some(404));
            assert!(
                details.message.contains("semantic method"),
                "unexpected message for {method}: {}",
                details.message
            );
        }
    }

    #[tokio::test]
    async fn storage_put_get_delete_error_without_server() {
        let router = test_router();
        assert!(
            router
                .call_method(
                    "storage.put",
                    json!({"dataset":"d","key":"k","data":"YQ=="}),
                )
                .await
                .is_err()
        );
        assert!(
            router
                .call_method("storage.get", json!({"dataset":"d","key":"k"}))
                .await
                .is_err()
        );
        assert!(
            router
                .call_method("storage.delete", json!({"dataset":"d","key":"k"}))
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn storage_routes_hit_dispatch_table_without_server() {
        let router = test_router();
        let ex = match router
            .call_method("storage.exists", json!({"dataset":"d","key":"k"}))
            .await
        {
            Ok(x) => x,
            Err(e) => panic!("exists: {e}"),
        };
        assert_eq!(ex["exists"], false);

        for (method, params) in [
            ("storage.metadata", json!({"dataset":"d","key":"k"})),
            ("storage.dataset.list", json!({})),
            ("storage.dataset.get", json!({"name":"n"})),
            ("storage.dataset.delete", json!({"name":"n"})),
        ] {
            assert!(
                router.call_method(method, params).await.is_err(),
                "expected connection error for {method}"
            );
        }
    }

    #[tokio::test]
    async fn unknown_semantic_method_is_not_found() {
        let router = test_router();
        let err = match router.call_method("storage.nonexistent", json!({})).await {
            Ok(_) => panic!("expected unknown method error"),
            Err(e) => e,
        };
        match err {
            NestGateError::Api(details) => {
                assert!(
                    details.message.contains("semantic method"),
                    "unexpected message: {}",
                    details.message
                );
                assert_eq!(details.status_code, Some(404));
            }
            other => panic!("expected Api(404), got {other:?}"),
        }
    }

    /// Reserve a free localhost TCP port, release it, then start [`serve_tarpc`] on that address.
    pub(crate) async fn spawn_local_tarpc_server() -> (SocketAddr, tokio::task::JoinHandle<()>) {
        let addr = {
            let l = match std::net::TcpListener::bind("127.0.0.1:0") {
                Ok(x) => x,
                Err(e) => panic!("bind: {e}"),
            };
            match l.local_addr() {
                Ok(a) => a,
                Err(e) => panic!("local_addr: {e}"),
            }
        };
        let service = match NestGateRpcService::new() {
            Ok(s) => s,
            Err(e) => panic!("service: {e}"),
        };
        let handle = tokio::spawn(async move {
            let _ = serve_tarpc(addr, service).await;
        });
        for _ in 0..80 {
            if std::net::TcpStream::connect(addr).is_ok() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        (addr, handle)
    }

    #[tokio::test]
    async fn discovery_methods_return_self_knowledge() {
        let router = test_router();

        let announce = match router.call_method("discovery.announce", json!({})).await {
            Ok(x) => x,
            Err(e) => panic!("discovery.announce: {e}"),
        };
        assert_eq!(announce["status"], "registered_locally");

        let list = match router.call_method("discovery.list", json!({})).await {
            Ok(x) => x,
            Err(e) => panic!("discovery.list: {e}"),
        };
        assert!(list["services"].as_array().is_some());

        let cap = match router
            .call_method("discovery.capabilities", json!({}))
            .await
        {
            Ok(x) => x,
            Err(e) => panic!("discovery.capabilities: {e}"),
        };
        assert!(cap.get("capabilities").is_some());

        match router.call_method("discovery.query", json!({})).await {
            Ok(_) => panic!("expected error for missing capability"),
            Err(_) => {}
        }
        let query = match router
            .call_method("discovery.query", json!({"capability": "storage"}))
            .await
        {
            Ok(x) => x,
            Err(e) => panic!("discovery.query: {e}"),
        };
        assert!(query["providers"].as_array().is_some());
    }

    #[tokio::test]
    async fn metadata_methods_wire_through_backend() {
        let router = test_router();

        let stored = match router
            .call_method(
                "metadata.store",
                json!({"name": "test-svc", "capabilities": ["storage"]}),
            )
            .await
        {
            Ok(x) => x,
            Err(e) => panic!("metadata.store: {e}"),
        };
        assert_eq!(stored["status"], "stored");

        let retrieved = match router
            .call_method("metadata.retrieve", json!({"name": "test-svc"}))
            .await
        {
            Ok(x) => x,
            Err(e) => panic!("metadata.retrieve: {e}"),
        };
        assert_eq!(retrieved["name"], "test-svc");

        let searched = match router
            .call_method("metadata.search", json!({"capability": "storage"}))
            .await
        {
            Ok(x) => x,
            Err(e) => panic!("metadata.search: {e}"),
        };
        assert_eq!(searched["count"], 1);

        match router.call_method("metadata.retrieve", json!({})).await {
            Ok(_) => panic!("expected missing name error"),
            Err(_) => {}
        }
        match router.call_method("metadata.search", json!({})).await {
            Ok(_) => panic!("expected missing capability error"),
            Err(_) => {}
        }
    }

    #[tokio::test]
    async fn health_semantic_methods_with_live_tarpc_server() {
        let (addr, server_handle) = spawn_local_tarpc_server().await;
        let endpoint = format!("tarpc://{}", addr);
        let client = match NestGateRpcClient::new(&endpoint) {
            Ok(c) => Arc::new(c),
            Err(e) => panic!("client: {e}"),
        };
        let router = match SemanticRouter::new(client) {
            Ok(r) => r,
            Err(e) => panic!("router: {e}"),
        };

        let check = match router.call_method("health.check", json!({})).await {
            Ok(x) => x,
            Err(e) => panic!("health.check: {e}"),
        };
        assert_eq!(check["status"], "healthy");

        let live = match router.call_method("health.liveness", json!({})).await {
            Ok(x) => x,
            Err(e) => panic!("liveness: {e}"),
        };
        assert_eq!(live["alive"], true);
        assert_eq!(live["status"], "ok");

        let ready = match router.call_method("health.readiness", json!({})).await {
            Ok(x) => x,
            Err(e) => panic!("readiness: {e}"),
        };
        assert_eq!(ready["ready"], true);
        assert_eq!(ready["backends"]["storage"], "ready");

        let metrics = match router.call_method("health.metrics", json!({})).await {
            Ok(x) => x,
            Err(e) => panic!("metrics: {e}"),
        };
        assert!(metrics.get("used_space_bytes").is_some() || metrics.is_object());

        let info = match router.call_method("health.info", json!({})).await {
            Ok(x) => x,
            Err(e) => panic!("info: {e}"),
        };
        assert!(info.get("version").is_some());

        server_handle.abort();
    }

    #[tokio::test]
    async fn storage_put_success_through_router_with_server() {
        let (addr, server_handle) = spawn_local_tarpc_server().await;
        let endpoint = format!("tarpc://{}", addr);
        let client = match NestGateRpcClient::new(&endpoint) {
            Ok(c) => Arc::new(c),
            Err(e) => panic!("client: {e}"),
        };
        let router = match SemanticRouter::new(client) {
            Ok(r) => r,
            Err(e) => panic!("router: {e}"),
        };

        match router
            .call_method(
                "storage.dataset.create",
                json!({"name": "ds-semantic", "description": "t"}),
            )
            .await
        {
            Ok(_) => {}
            Err(e) => panic!("create dataset: {e}"),
        }

        let put = match router
            .call_method(
                "storage.put",
                json!({
                    "dataset": "ds-semantic",
                    "key": "k1",
                    "data": "aGVsbG8="
                }),
            )
            .await
        {
            Ok(x) => x,
            Err(e) => panic!("put: {e}"),
        };
        assert!(put.get("key").is_some() || put.get("size").is_some());

        let get = match router
            .call_method(
                "storage.get",
                json!({"dataset": "ds-semantic", "key": "k1"}),
            )
            .await
        {
            Ok(x) => x,
            Err(e) => panic!("get: {e}"),
        };
        assert_eq!(get["data"], "aGVsbG8=");

        server_handle.abort();
    }

    #[tokio::test]
    async fn unknown_method_with_whitespace_not_matched() {
        let router = test_router();
        let err = match router.call_method(" storage.put", json!({})).await {
            Ok(_) => panic!("expected not found"),
            Err(e) => e,
        };
        match err {
            NestGateError::Api(details) => {
                assert_eq!(details.status_code, Some(404));
            }
            other => panic!("expected Api 404, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn storage_list_route_errors_without_server() {
        let router = test_router();
        assert!(
            router
                .call_method("storage.list", json!({"dataset": "d"}))
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn blob_and_stream_routes_dispatch_without_server() {
        let router = test_router();
        for method in [
            "storage.store_blob",
            "storage.retrieve_blob",
            "storage.retrieve_range",
            "storage.store_stream",
            "storage.retrieve_stream",
        ] {
            assert!(
                router.call_method(method, json!({})).await.is_err(),
                "expected error for offline {method}"
            );
        }
    }

    #[tokio::test]
    async fn session_routes_are_reachable_offline() {
        let router = test_router();
        let listed = match router.call_method("session.list", json!({})).await {
            Ok(x) => x,
            Err(e) => panic!("session.list: {e}"),
        };
        assert!(listed.get("sessions").is_some());
    }
}
