// SPDX-License-Identifier: AGPL-3.0-only
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
//! - **Neural API Ready**: biomeOS can route by capability
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
//! let router = SemanticRouter::new(service);
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
//! - `data.ncbi_search` → NCBI database search
//! - `data.ncbi_fetch` → NCBI record fetch
//! - `data.noaa_ghcnd` → NOAA weather station data
//! - `data.iris_stations` → IRIS seismic station listing
//! - `data.iris_events` → IRIS seismic event listing
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
//! ## References
//!
//! - `wateringHole/SEMANTIC_METHOD_NAMING_STANDARD.md` v2.0
//! - `wateringHole/PRIMAL_IPC_PROTOCOL.md` v1.0
//! - `CAPABILITY_MAPPINGS.md`

use crate::rpc::NestGateRpcClient;
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
pub mod storage;

#[cfg(test)]
mod tests;

/// Semantic method router for TRUE PRIMAL compliance
///
/// Routes semantic method names (e.g., `storage.put`) to internal
/// implementations, enabling Neural API integration and capability-based
/// discovery.
pub struct SemanticRouter {
    /// Internal RPC client for delegation
    pub(crate) client: Arc<NestGateRpcClient>,
}

impl SemanticRouter {
    /// Create new semantic router
    ///
    /// # Arguments
    /// * `client` - Internal RPC client for method delegation
    ///
    /// # Example
    /// ```rust,ignore
    /// use nestgate_core::rpc::{SemanticRouter, NestGateRpcClient};
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = NestGateRpcClient::new("tarpc://localhost:8091")?;
    /// let router = SemanticRouter::new(Arc::new(client));
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(client: Arc<NestGateRpcClient>) -> Self {
        debug!("🌐 Creating semantic method router (TRUE PRIMAL compliance)");
        Self { client }
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

            // ==================== DATA DOMAIN (live feeds, not storage) ====================
            "data.ncbi_search" => data::data_ncbi_search(self, params),
            "data.ncbi_fetch" => data::data_ncbi_fetch(self, params),
            "data.noaa_ghcnd" => data::data_noaa_ghcnd(self, params),
            "data.iris_stations" => data::data_iris_stations(self, params),
            "data.iris_events" => data::data_iris_events(self, params),

            // ==================== DISCOVERY DOMAIN ====================
            "discovery.announce" => discovery::discovery_announce(self, params),
            "discovery.query" => discovery::discovery_query(self, params),
            "discovery.list" => discovery::discovery_list(self, params),
            "discovery.capabilities" => discovery::discovery_capabilities(self, params),

            // ==================== HEALTH DOMAIN ====================
            "health.check" => health::health_check(self, params).await,
            "health.liveness" => health::health_liveness(self, params).await,
            "health.readiness" => health::health_ready(self, params).await,
            "health.metrics" => health::health_metrics(self, params).await,
            "health.info" => health::health_info(self, params).await,

            // ==================== CAPABILITIES DOMAIN ====================
            "capabilities.list" => capabilities::capabilities_list(self, params),

            // ==================== METADATA DOMAIN ====================
            "metadata.store" => metadata::metadata_store(self, params),
            "metadata.retrieve" => metadata::metadata_retrieve(self, params),
            "metadata.search" => metadata::metadata_search(self, params),

            // ==================== CRYPTO DOMAIN ====================
            "crypto.encrypt" => crypto::crypto_encrypt(self, params),
            "crypto.decrypt" => crypto::crypto_decrypt(self, params),
            "crypto.generate_key" => crypto::crypto_generate_key(self, params),
            "crypto.generate_nonce" => crypto::crypto_generate_nonce(self, params),
            "crypto.hash" => crypto::crypto_hash(self, params),
            "crypto.verify_hash" => crypto::crypto_verify_hash(self, params),

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
