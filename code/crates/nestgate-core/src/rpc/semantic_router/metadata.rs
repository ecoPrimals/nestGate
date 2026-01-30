//! Metadata domain semantic methods
//!
//! Handles metadata.* semantic method routing for service metadata management.

use super::SemanticRouter;
use crate::error::{NestGateError, Result};
use serde_json::{json, Value};
use tracing::{debug, info};

/// Route metadata.store → store service metadata
///
/// Stores or updates metadata for a service.
pub(super) async fn metadata_store(_router: &SemanticRouter, params: Value) -> Result<Value> {
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
pub(super) async fn metadata_retrieve(_router: &SemanticRouter, params: Value) -> Result<Value> {
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
pub(super) async fn metadata_search(_router: &SemanticRouter, params: Value) -> Result<Value> {
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
