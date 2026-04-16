// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Service discovery entry points and health checks.

use super::types::{DiscoveredService, DiscoveryConfig, DiscoveryMethod, DiscoveryResult};
use crate::Result;
use crate::canonical_types::service::{ServiceState, ServiceType};
use std::collections::HashMap;
use std::time::SystemTime;

/// Discover available services using canonical discovery
pub fn discover_services(config: &DiscoveryConfig) -> Result<DiscoveryResult> {
    let start_time = std::time::Instant::now();

    // For now, return a basic result - this would be expanded with real discovery logic
    let services = vec![DiscoveredService {
        id: "nestgate-core".to_string(),
        name: "NestGate Core".to_string(),
        service_type: ServiceType::Storage,
        state: ServiceState::Running,
        endpoint: config.endpoint.clone(),
        capabilities: vec!["storage".to_string(), "zfs".to_string()],
        metadata: HashMap::new(),
        discovered_at: SystemTime::now(),
        last_health_check: Some(SystemTime::now()),
    }];

    Ok(DiscoveryResult {
        services,
        method: DiscoveryMethod::Environment,
        duration: start_time.elapsed(),
        success: true,
        error: None,
    })
}
/// Discover services by capability
pub fn discover_by_capability(
    config: &DiscoveryConfig,
    capability: &str,
) -> Result<Vec<DiscoveredService>> {
    let result = discover_services(config)?;

    Ok(result
        .services
        .into_iter()
        .filter(|service| service.capabilities.contains(&capability.to_string()))
        .collect())
}
/// Health check a discovered service
pub const fn health_check_service(service: &DiscoveredService) -> bool {
    // Basic health check implementation - would be expanded with real health check logic
    matches!(service.state, ServiceState::Running)
}
