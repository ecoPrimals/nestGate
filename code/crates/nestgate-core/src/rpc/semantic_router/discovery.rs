//! Discovery domain semantic methods
//!
//! Handles discovery.* semantic method routing for service registration and discovery.

use super::SemanticRouter;
use crate::error::Result;
use serde_json::{json, Value};
use tracing::{debug, info};

/// Route discovery.announce → register service
///
/// Registers a service with the discovery system.
/// Typically called by Songbird when a primal comes online.
pub(super) async fn discovery_announce(router: &SemanticRouter, params: Value) -> Result<Value> {
    use crate::error::NestGateError;
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
pub(super) async fn discovery_query(router: &SemanticRouter, params: Value) -> Result<Value> {
    use crate::error::NestGateError;
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
pub(super) async fn discovery_list(_router: &SemanticRouter, _params: Value) -> Result<Value> {
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
pub(super) async fn discovery_capabilities(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Ok(json!({
        "capabilities": ["storage", "discovery", "metadata", "health"]
    }))
}
