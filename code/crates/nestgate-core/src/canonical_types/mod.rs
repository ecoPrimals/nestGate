// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Canonical Types module

pub mod api;
pub mod conversion;
pub mod events;
pub mod handlers;
pub mod health;
pub mod network;
pub mod security;
pub mod service;
pub mod storage;
pub mod system;

use serde::{Deserialize, Serialize};

// Universal request/response types removed - use domain-specific types instead
// Migration: UniversalRequest → Domain-specific request types
// Migration: UniversalResponse → Domain-specific response types

/// Response status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Status values for Response
pub enum ResponseStatus {
    /// Operation completed successfully
    Success,
    /// Operation failed with an error
    Error,
    /// Operation completed partially
    Partial,
    /// Operation timed out
    Timeout,
    /// Requested resource was not found
    NotFound,
    /// Unauthorized access attempted
    Unauthorized,
    /// Access forbidden
    Forbidden,
}

/// Common types re-exported for easy access
pub use api::{ApiError, Request, Response};
pub use events::{Event, EventCategory, EventSeverity};
pub use health::{HealthCheck, HealthStatus, SystemHealth};
pub use network::{ConnectionStatus, Endpoint, Protocol};
pub use security::{AccessLevel, AuthMethod, SecurityContext};
pub use service::{ServiceConfig, ServiceId, ServiceMetrics, ServiceState, ServiceType};
pub use storage::{StorageMetadata, StorageOperation, StorageResource, StorageTier};
pub use system::AllocationStatus;

#[cfg(test)]
mod tests {
    use super::*;

    /// Parses  Service State
    fn parse_service_state(state: &str) -> ServiceState {
        match state {
            "running" => ServiceState::Running,
            "stopped" => ServiceState::Stopped,
            "starting" => ServiceState::Starting,
            "stopping" => ServiceState::Stopping,
            _ => ServiceState::Failed,
        }
    }

    /// Parses  Service Type
    fn parse_service_type(service_type: &str) -> ServiceType {
        match service_type {
            "storage" => ServiceType::Storage,
            "network" => ServiceType::Network,
            "security" => ServiceType::Security,
            _ => ServiceType::Storage, // Default fallback
        }
    }

    #[test]
    fn test_service_types() {
        let storage = ServiceType::Storage;
        let network = ServiceType::Network;
        assert_ne!(storage, network);
    }

    #[test]
    fn test_conversion_utilities() {
        assert_eq!(parse_service_state("running"), ServiceState::Running);
        assert_eq!(parse_service_type("storage"), ServiceType::Storage);
    }
}
