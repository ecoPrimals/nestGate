// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Type conversion utilities for canonical types

use super::service::{ServiceState, ServiceType};

/// Convert legacy service state string to canonical type
#[must_use]
pub fn parse_service_state(state: &str) -> ServiceState {
    match state.to_lowercase().as_str() {
        "starting" => ServiceState::Starting,
        "running" => ServiceState::Running,
        "stopping" => ServiceState::Stopping,
        "stopped" => ServiceState::Stopped,
        "failed" => ServiceState::Failed,
        _ => ServiceState::Unknown,
    }
}

/// Convert legacy service type string to canonical type
#[must_use]
pub fn parse_service_type(service_type: &str) -> ServiceType {
    match service_type.to_lowercase().as_str() {
        "storage" => ServiceType::Storage,
        "network" => ServiceType::Network,
        "security" => ServiceType::Security,
        "monitoring" => ServiceType::Monitoring,
        "compute" => ServiceType::Compute,
        "api" => ServiceType::Api,
        _ => ServiceType::Generic,
    }
}
