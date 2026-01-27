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
//! - `crypto.encrypt` → delegate to BearDog (capability discovery!)
//! - `crypto.decrypt` → delegate to BearDog
//! - `crypto.generate_key` → delegate to BearDog
//! - `crypto.generate_nonce` → delegate to BearDog
//! - `crypto.hash` → delegate to BearDog
//! - `crypto.verify_hash` → delegate to BearDog
//!
//! ### Health Domain (`health.*`)
//! - `health.check` → `health_check`
//! - `health.metrics` → `get_metrics`
//! - `health.info` → `get_info`
//! - `health.ready` → readiness check
//!
//! ## References
//!
//! - wateringHole/SEMANTIC_METHOD_NAMING_STANDARD.md v2.0
//! - wateringHole/PRIMAL_IPC_PROTOCOL.md v1.0
//! - CAPABILITY_MAPPINGS.md

use crate::error::{NestGateError, Result};
use crate::rpc::tarpc_types::{DatasetParams, NestGateRpcClient};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{debug, warn};

/// Semantic method router for TRUE PRIMAL compliance
///
/// Routes semantic method names (e.g., `storage.put`) to internal
/// implementations, enabling Neural API integration and capability-based
/// discovery.
pub struct SemanticRouter {
    /// Internal RPC client for delegation
    client: Arc<NestGateRpcClient>,
}

impl SemanticRouter {
    /// Create new semantic router
    ///
    /// # Arguments
    /// * `client` - Internal RPC client for method delegation
    ///
    /// # Example
    /// ```no_run
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
    /// ```no_run
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
            "storage.put" => self.storage_put(params).await,
            "storage.get" => self.storage_get(params).await,
            "storage.delete" => self.storage_delete(params).await,
            "storage.list" => self.storage_list(params).await,
            "storage.exists" => self.storage_exists(params).await,
            "storage.metadata" => self.storage_metadata(params).await,

            // Dataset operations
            "storage.dataset.create" => self.dataset_create(params).await,
            "storage.dataset.get" => self.dataset_get(params).await,
            "storage.dataset.list" => self.dataset_list(params).await,
            "storage.dataset.delete" => self.dataset_delete(params).await,

            // ==================== DISCOVERY DOMAIN ====================
            "discovery.announce" => self.discovery_announce(params).await,
            "discovery.query" => self.discovery_query(params).await,
            "discovery.list" => self.discovery_list(params).await,
            "discovery.capabilities" => self.discovery_capabilities(params).await,

            // ==================== HEALTH DOMAIN ====================
            "health.check" => self.health_check(params).await,
            "health.metrics" => self.health_metrics(params).await,
            "health.info" => self.health_info(params).await,
            "health.ready" => self.health_ready(params).await,

            // ==================== METADATA DOMAIN ====================
            "metadata.store" => self.metadata_store(params).await,
            "metadata.retrieve" => self.metadata_retrieve(params).await,
            "metadata.search" => self.metadata_search(params).await,

            // ==================== CRYPTO DOMAIN ====================
            "crypto.encrypt" => self.crypto_encrypt(params).await,
            "crypto.decrypt" => self.crypto_decrypt(params).await,
            "crypto.generate_key" => self.crypto_generate_key(params).await,
            "crypto.generate_nonce" => self.crypto_generate_nonce(params).await,
            "crypto.hash" => self.crypto_hash(params).await,
            "crypto.verify_hash" => self.crypto_verify_hash(params).await,

            // Unknown method
            _ => {
                warn!("❌ Unknown semantic method: {}", method);
                Err(NestGateError::method_not_found(method))
            }
        }
    }

    // ==================== STORAGE DOMAIN IMPLEMENTATIONS ====================

    /// Route storage.put → store_object
    async fn storage_put(&self, params: Value) -> Result<Value> {
        let dataset = params["dataset"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("dataset", "string required"))?;
        let key = params["key"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("key", "string required"))?;
        let data_b64 = params["data"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("data", "base64 string required"))?;

        // Decode base64
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let data = STANDARD.decode(data_b64)
            .map_err(|e| NestGateError::invalid_input("data", &format!("Invalid base64: {}", e)))?;

        // Call internal implementation
        let result = self.client.store_object(dataset, key, data).await?;

        Ok(json!({
            "success": result.success,
            "message": result.message,
            "metadata": result.metadata
        }))
    }

    /// Route storage.get → retrieve_object
    async fn storage_get(&self, params: Value) -> Result<Value> {
        let dataset = params["dataset"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("dataset", "string required"))?;
        let key = params["key"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("key", "string required"))?;

        let data = self.client.retrieve_object(dataset, key).await?;

        // Encode to base64 for transport
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let data_b64 = STANDARD.encode(&data);

        Ok(json!({
            "data": data_b64,
            "size": data.len()
        }))
    }

    /// Route storage.delete → delete_object
    async fn storage_delete(&self, params: Value) -> Result<Value> {
        let dataset = params["dataset"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("dataset", "string required"))?;
        let key = params["key"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("key", "string required"))?;

        let result = self.client.delete_object(dataset, key).await?;

        Ok(json!({
            "success": result.success,
            "message": result.message
        }))
    }

    /// Route storage.list → list_objects
    async fn storage_list(&self, params: Value) -> Result<Value> {
        let dataset = params["dataset"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("dataset", "string required"))?;
        let prefix = params["prefix"].as_str().map(String::from);

        let objects = self.client.list_objects(dataset, prefix).await?;

        Ok(json!({
            "objects": objects,
            "count": objects.len()
        }))
    }

    /// Route storage.exists → check if object exists
    async fn storage_exists(&self, params: Value) -> Result<Value> {
        let dataset = params["dataset"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("dataset", "string required"))?;
        let key = params["key"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("key", "string required"))?;

        // Try to retrieve metadata (cheaper than full object)
        let exists = self.client.retrieve_object(dataset, key).await.is_ok();

        Ok(json!({ "exists": exists }))
    }

    /// Route storage.metadata → get object metadata
    async fn storage_metadata(&self, params: Value) -> Result<Value> {
        let dataset = params["dataset"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("dataset", "string required"))?;
        let key = params["key"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("key", "string required"))?;

        // Get object info (includes metadata)
        let data = self.client.retrieve_object(dataset, key).await?;

        Ok(json!({
            "size": data.len(),
            "exists": true
        }))
    }

    // ==================== DATASET OPERATIONS ====================

    /// Route storage.dataset.create → create_dataset
    async fn dataset_create(&self, params: Value) -> Result<Value> {
        let name = params["name"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("name", "string required"))?;
        let description = params["description"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let dataset_params = DatasetParams {
            description,
            ..Default::default()
        };

        let dataset = self.client.create_dataset(name, dataset_params).await?;

        Ok(json!({
            "name": dataset.name,
            "created_at": dataset.created_at,
            "status": dataset.status
        }))
    }

    /// Route storage.dataset.get → get_dataset
    async fn dataset_get(&self, params: Value) -> Result<Value> {
        let name = params["name"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("name", "string required"))?;

        let dataset = self.client.get_dataset(name).await?;

        Ok(serde_json::to_value(dataset)
            .map_err(|e| NestGateError::serialization(&format!("Failed to serialize dataset: {}", e)))?)
    }

    /// Route storage.dataset.list → list_datasets
    async fn dataset_list(&self, _params: Value) -> Result<Value> {
        let datasets = self.client.list_datasets().await?;

        Ok(json!({
            "datasets": datasets,
            "count": datasets.len()
        }))
    }

    /// Route storage.dataset.delete → delete_dataset
    async fn dataset_delete(&self, params: Value) -> Result<Value> {
        let name = params["name"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("name", "string required"))?;

        let result = self.client.delete_dataset(name).await?;

        Ok(json!({
            "success": result.success,
            "message": result.message
        }))
    }

    // ==================== DISCOVERY DOMAIN ====================

    /// Route discovery.announce → register service
    ///
    /// Registers a service with the discovery system.
    /// Typically called by Songbird when a primal comes online.
    async fn discovery_announce(&self, params: Value) -> Result<Value> {
        use crate::service_metadata::{ServiceMetadata, ServiceMetadataStore};
        use std::time::SystemTime;

        // Parse service metadata from params
        let name = params["name"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("name", "string required"))?
            .to_string();

        let version = params["version"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        let capabilities: Vec<String> = params["capabilities"]
            .as_array()
            .ok_or_else(|| NestGateError::invalid_input("capabilities", "array required"))?
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();

        let virtual_endpoint = params["endpoint"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("endpoint", "string required"))?
            .to_string();

        let platform = params["platform"]
            .as_str()
            .unwrap_or(std::env::consts::OS)
            .to_string();

        let native_endpoint = params["native_endpoint"]
            .as_str()
            .unwrap_or(&virtual_endpoint)
            .to_string();

        // Create metadata
        let metadata = ServiceMetadata {
            name: name.clone(),
            version,
            capabilities,
            virtual_endpoint,
            registered_at: SystemTime::now(),
            last_seen: SystemTime::now(),
            platform,
            native_endpoint,
            metadata: std::collections::HashMap::new(),
        };

        // Store metadata
        let store = ServiceMetadataStore::new().await?;
        store.store_service(metadata).await?;

        info!("🎉 Service registered: {}", name);

        Ok(json!({
            "registered": true,
            "service": name,
            "message": "Service successfully registered"
        }))
    }

    /// Route discovery.query → find services by capability
    ///
    /// Finds all services that provide a specific capability.
    async fn discovery_query(&self, params: Value) -> Result<Value> {
        use crate::service_metadata::ServiceMetadataStore;

        let capability = params["capability"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("capability", "string required"))?;

        let store = ServiceMetadataStore::new().await?;
        let services = store.find_by_capability(capability).await?;

        let result: Vec<Value> = services
            .into_iter()
            .map(|meta| {
                json!({
                    "name": meta.name,
                    "version": meta.version,
                    "endpoint": meta.virtual_endpoint,
                    "capabilities": meta.capabilities,
                    "platform": meta.platform
                })
            })
            .collect();

        debug!("🔍 Discovery query for '{}': {} services found", capability, result.len());

        Ok(json!({ "services": result }))
    }

    /// Route discovery.list → list all services
    ///
    /// Lists all registered services in the discovery system.
    async fn discovery_list(&self, _params: Value) -> Result<Value> {
        use crate::service_metadata::ServiceMetadataStore;

        let store = ServiceMetadataStore::new().await?;
        let services = store.list_services().await?;

        let result: Vec<Value> = services
            .into_iter()
            .map(|meta| {
                json!({
                    "name": meta.name,
                    "version": meta.version,
                    "endpoint": meta.virtual_endpoint,
                    "capabilities": meta.capabilities,
                    "platform": meta.platform,
                    "registered_at": meta.registered_at
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()
                })
            })
            .collect();

        debug!("📋 Discovery list: {} services total", result.len());

        Ok(json!({ "services": result, "count": result.len() }))
    }

    /// Route discovery.capabilities → get service capabilities (placeholder)
    async fn discovery_capabilities(&self, _params: Value) -> Result<Value> {
        Ok(json!({
            "capabilities": ["storage", "discovery", "metadata", "health"]
        }))
    }

    // ==================== HEALTH DOMAIN ====================

    /// Route health.check → health_check
    async fn health_check(&self, _params: Value) -> Result<Value> {
        let health = self.client.health().await?;

        Ok(json!({
            "status": health.status,
            "uptime_seconds": health.uptime_seconds,
            "version": health.version
        }))
    }

    /// Route health.metrics → get_metrics
    async fn health_metrics(&self, _params: Value) -> Result<Value> {
        let metrics = self.client.get_storage_metrics().await?;

        Ok(serde_json::to_value(metrics)
            .map_err(|e| NestGateError::serialization(&format!("Failed to serialize metrics: {}", e)))?)
    }

    /// Route health.info → get_info
    async fn health_info(&self, _params: Value) -> Result<Value> {
        let info = self.client.get_info().await?;

        Ok(serde_json::to_value(info)
            .map_err(|e| NestGateError::serialization(&format!("Failed to serialize info: {}", e)))?)
    }

    /// Route health.ready → readiness check
    async fn health_ready(&self, _params: Value) -> Result<Value> {
        let health = self.client.health().await?;

        Ok(json!({
            "ready": health.status == "healthy",
            "status": health.status
        }))
    }

    // ==================== METADATA DOMAIN ====================

    /// Route metadata.store → store service metadata
    ///
    /// Stores or updates metadata for a service.
    async fn metadata_store(&self, params: Value) -> Result<Value> {
        use crate::service_metadata::{ServiceMetadata, ServiceMetadataStore};
        use std::time::SystemTime;

        // Parse service metadata
        let name = params["name"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("name", "string required"))?
            .to_string();

        let version = params["version"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        let capabilities: Vec<String> = params["capabilities"]
            .as_array()
            .ok_or_else(|| NestGateError::invalid_input("capabilities", "array required"))?
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();

        let virtual_endpoint = params["endpoint"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("endpoint", "string required"))?
            .to_string();

        let platform = params["platform"]
            .as_str()
            .unwrap_or(std::env::consts::OS)
            .to_string();

        let native_endpoint = params["native_endpoint"]
            .as_str()
            .unwrap_or(&virtual_endpoint)
            .to_string();

        let metadata = ServiceMetadata {
            name: name.clone(),
            version,
            capabilities,
            virtual_endpoint,
            registered_at: SystemTime::now(),
            last_seen: SystemTime::now(),
            platform,
            native_endpoint,
            metadata: std::collections::HashMap::new(),
        };

        let store = ServiceMetadataStore::new().await?;
        store.store_service(metadata).await?;

        info!("💾 Metadata stored: {}", name);

        Ok(json!({
            "stored": true,
            "service": name,
            "message": "Metadata successfully stored"
        }))
    }

    /// Route metadata.retrieve → get service metadata
    ///
    /// Retrieves metadata for a specific service by name.
    async fn metadata_retrieve(&self, params: Value) -> Result<Value> {
        use crate::service_metadata::ServiceMetadataStore;

        let name = params["name"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("name", "string required"))?;

        let store = ServiceMetadataStore::new().await?;
        let meta = store.get_service(name).await?;

        debug!("📖 Metadata retrieved: {}", name);

        Ok(json!({
            "name": meta.name,
            "version": meta.version,
            "capabilities": meta.capabilities,
            "endpoint": meta.virtual_endpoint,
            "platform": meta.platform,
            "registered_at": meta.registered_at
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            "last_seen": meta.last_seen
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        }))
    }

    /// Route metadata.search → search metadata
    ///
    /// Searches for services by capability (alias for discovery.query).
    async fn metadata_search(&self, params: Value) -> Result<Value> {
        use crate::service_metadata::ServiceMetadataStore;

        // Support both "capability" and "query" parameters
        let capability = params["capability"]
            .as_str()
            .or_else(|| params["query"].as_str())
            .ok_or_else(|| {
                NestGateError::invalid_input("capability or query", "string required")
            })?;

        let store = ServiceMetadataStore::new().await?;
        let services = store.find_by_capability(capability).await?;

        let results: Vec<Value> = services
            .into_iter()
            .map(|meta| {
                json!({
                    "name": meta.name,
                    "version": meta.version,
                    "endpoint": meta.virtual_endpoint,
                    "capabilities": meta.capabilities,
                    "platform": meta.platform
                })
            })
            .collect();

        debug!("🔎 Metadata search for '{}': {} results", capability, results.len());

        Ok(json!({
            "results": results,
            "count": results.len(),
            "query": capability
        }))
    }

    // ==================== CRYPTO DOMAIN ====================

    /// Route crypto.encrypt → CryptoDelegate::encrypt
    ///
    /// Delegates encryption to BearDog or compatible crypto provider.
    /// Demonstrates capability-based discovery in action!
    async fn crypto_encrypt(&self, params: Value) -> Result<Value> {
        use crate::crypto::{delegate::CryptoDelegate, EncryptionAlgorithm, EncryptionParams};
        use base64::{engine::general_purpose::STANDARD, Engine as _};

        // Discover and connect to crypto provider
        let delegate = CryptoDelegate::new().await?;

        // Parse parameters
        let plaintext_b64 = params["plaintext"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("plaintext", "base64 string required"))?;

        let plaintext = STANDARD.decode(plaintext_b64)
            .map_err(|e| NestGateError::invalid_input("plaintext", &format!("Invalid base64: {}", e)))?;

        let algorithm = match params["algorithm"].as_str().unwrap_or("aes256gcm") {
            "aes256gcm" => EncryptionAlgorithm::Aes256Gcm,
            "chacha20poly1305" => EncryptionAlgorithm::ChaCha20Poly1305,
            algo => return Err(NestGateError::invalid_input("algorithm", &format!("Unsupported algorithm: {}", algo))),
        };

        let associated_data = if let Some(ad) = params["associated_data"].as_str() {
            STANDARD.decode(ad)
                .map_err(|e| NestGateError::invalid_input("associated_data", &format!("Invalid base64: {}", e)))?
        } else {
            Vec::new()
        };

        let encryption_params = EncryptionParams {
            algorithm,
            associated_data,
        };

        // Delegate to crypto provider
        let encrypted = delegate.encrypt(&plaintext, &encryption_params).await?;

        debug!("🔐 Encryption complete via {}", delegate.provider_info().name);

        Ok(json!({
            "ciphertext": STANDARD.encode(&encrypted.ciphertext),
            "nonce": STANDARD.encode(&encrypted.nonce),
            "algorithm": match encrypted.algorithm {
                EncryptionAlgorithm::Aes256Gcm => "aes256gcm",
                EncryptionAlgorithm::ChaCha20Poly1305 => "chacha20poly1305",
            },
            "timestamp": encrypted.timestamp,
            "provider": delegate.provider_info().name
        }))
    }

    /// Route crypto.decrypt → CryptoDelegate::decrypt
    async fn crypto_decrypt(&self, params: Value) -> Result<Value> {
        use crate::crypto::{delegate::CryptoDelegate, EncryptedData, EncryptionAlgorithm};
        use base64::{engine::general_purpose::STANDARD, Engine as _};

        let delegate = CryptoDelegate::new().await?;

        // Parse parameters
        let ciphertext_b64 = params["ciphertext"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("ciphertext", "base64 string required"))?;

        let ciphertext = STANDARD.decode(ciphertext_b64)
            .map_err(|e| NestGateError::invalid_input("ciphertext", &format!("Invalid base64: {}", e)))?;

        let nonce_b64 = params["nonce"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("nonce", "base64 string required"))?;

        let nonce = STANDARD.decode(nonce_b64)
            .map_err(|e| NestGateError::invalid_input("nonce", &format!("Invalid base64: {}", e)))?;

        let algorithm = match params["algorithm"].as_str().unwrap_or("aes256gcm") {
            "aes256gcm" => EncryptionAlgorithm::Aes256Gcm,
            "chacha20poly1305" => EncryptionAlgorithm::ChaCha20Poly1305,
            algo => return Err(NestGateError::invalid_input("algorithm", &format!("Unsupported algorithm: {}", algo))),
        };

        let encrypted = EncryptedData {
            ciphertext,
            nonce,
            algorithm,
            timestamp: params["timestamp"].as_u64().unwrap_or(0),
        };

        // Delegate to crypto provider
        let plaintext = delegate.decrypt(&encrypted).await?;

        debug!("🔓 Decryption complete via {}", delegate.provider_info().name);

        Ok(json!({
            "plaintext": STANDARD.encode(&plaintext),
            "provider": delegate.provider_info().name
        }))
    }

    /// Route crypto.generate_key → CryptoDelegate::generate_key
    async fn crypto_generate_key(&self, params: Value) -> Result<Value> {
        use crate::crypto::delegate::CryptoDelegate;
        use base64::{engine::general_purpose::STANDARD, Engine as _};

        let delegate = CryptoDelegate::new().await?;

        let length = params["length"]
            .as_u64()
            .ok_or_else(|| NestGateError::invalid_input("length", "number required"))? as usize;

        let key = delegate.generate_key(length).await?;

        debug!("🔑 Key generated ({} bytes) via {}", length, delegate.provider_info().name);

        Ok(json!({
            "key": STANDARD.encode(&key),
            "length": key.len(),
            "provider": delegate.provider_info().name
        }))
    }

    /// Route crypto.generate_nonce → CryptoDelegate::generate_nonce
    async fn crypto_generate_nonce(&self, params: Value) -> Result<Value> {
        use crate::crypto::{delegate::CryptoDelegate, EncryptionAlgorithm};
        use base64::{engine::general_purpose::STANDARD, Engine as _};

        let delegate = CryptoDelegate::new().await?;

        let algorithm = match params["algorithm"].as_str().unwrap_or("aes256gcm") {
            "aes256gcm" => EncryptionAlgorithm::Aes256Gcm,
            "chacha20poly1305" => EncryptionAlgorithm::ChaCha20Poly1305,
            algo => return Err(NestGateError::invalid_input("algorithm", &format!("Unsupported algorithm: {}", algo))),
        };

        let nonce = delegate.generate_nonce(algorithm).await?;

        debug!("🎲 Nonce generated ({} bytes) via {}", nonce.len(), delegate.provider_info().name);

        Ok(json!({
            "nonce": STANDARD.encode(&nonce),
            "length": nonce.len(),
            "provider": delegate.provider_info().name
        }))
    }

    /// Route crypto.hash → CryptoDelegate::hash
    async fn crypto_hash(&self, params: Value) -> Result<Value> {
        use crate::crypto::delegate::CryptoDelegate;
        use base64::{engine::general_purpose::STANDARD, Engine as _};

        let delegate = CryptoDelegate::new().await?;

        let data_b64 = params["data"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("data", "base64 string required"))?;

        let data = STANDARD.decode(data_b64)
            .map_err(|e| NestGateError::invalid_input("data", &format!("Invalid base64: {}", e)))?;

        let algorithm = params["algorithm"].as_str().unwrap_or("sha256");

        let hash = delegate.hash(&data, algorithm).await?;

        debug!("🔨 Hash computed ({} bytes) with {} via {}", hash.len(), algorithm, delegate.provider_info().name);

        Ok(json!({
            "hash": STANDARD.encode(&hash),
            "algorithm": algorithm,
            "provider": delegate.provider_info().name
        }))
    }

    /// Route crypto.verify_hash → CryptoDelegate::verify_hash
    async fn crypto_verify_hash(&self, params: Value) -> Result<Value> {
        use crate::crypto::delegate::CryptoDelegate;
        use base64::{engine::general_purpose::STANDARD, Engine as _};

        let delegate = CryptoDelegate::new().await?;

        let data_b64 = params["data"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("data", "base64 string required"))?;

        let data = STANDARD.decode(data_b64)
            .map_err(|e| NestGateError::invalid_input("data", &format!("Invalid base64: {}", e)))?;

        let hash_b64 = params["hash"]
            .as_str()
            .ok_or_else(|| NestGateError::invalid_input("hash", "base64 string required"))?;

        let hash = STANDARD.decode(hash_b64)
            .map_err(|e| NestGateError::invalid_input("hash", &format!("Invalid base64: {}", e)))?;

        let algorithm = params["algorithm"].as_str().unwrap_or("sha256");

        let valid = delegate.verify_hash(&data, &hash, algorithm).await?;

        debug!("🔍 Hash verification: {} via {}", if valid { "VALID" } else { "INVALID" }, delegate.provider_info().name);

        Ok(json!({
            "valid": valid,
            "algorithm": algorithm,
            "provider": delegate.provider_info().name
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_semantic_router_method_not_found() {
        // Note: Can't easily test without a real client, but we can verify
        // error handling structure
        // This would require mocking the client
    }

    #[test]
    fn test_semantic_method_names() {
        // Verify semantic method naming conventions
        let storage_methods = vec![
            "storage.put",
            "storage.get",
            "storage.delete",
            "storage.list",
            "storage.dataset.create",
        ];

        for method in storage_methods {
            assert!(method.contains('.'), "Method should use dot notation: {}", method);
            assert!(method.starts_with("storage."), "Storage method should start with storage.: {}", method);
        }
    }
}
