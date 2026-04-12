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
//! ### Data Domain (`data.*` — live feeds, NOT storage)
//! - `data.*` → delegate to whichever primal advertises the `"data"` capability
//!   (resolved at runtime via capability discovery, not by name). `NestGate` routes
//!   these method names but does not fetch data itself.
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
pub mod data;
pub mod discovery;
pub mod health;
pub mod metadata;
pub mod session;
pub mod storage;

#[cfg(test)]
mod tests;

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
    /// # async fn example(router: SemanticRouter) -> Result<(), Box<dyn std::error::Error>> {
    /// let result = router.call_method("storage.put", json!({
    ///     "dataset": "my-dataset",
    ///     "key": "my-key",
    ///     "data": "aGVsbG8gd29ybGQ=" // base64("hello world")
    /// })).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn call_method(&self, method: &str, params: Value) -> Result<Value> {
        debug!("🔀 Routing semantic method: {}", method);

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

            // ==================== DATA DOMAIN (live feeds, not storage — wildcard delegation) ====================
            m if m.starts_with("data.") => data::data_delegation(self, m, params),

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
                warn!("❌ Unknown semantic method: {}", method);
                Err(NestGateError::not_found(format!(
                    "semantic method `{method}`"
                )))
            }
        }
    }
}
