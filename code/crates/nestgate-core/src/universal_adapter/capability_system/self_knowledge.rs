// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! NestGate's advertised capabilities (the only primal identity this adapter hardcodes).

use super::types::{CapabilityCategory, ServiceCapability};
use nestgate_config::constants::system::DEFAULT_SERVICE_NAME;
use std::collections::HashMap;
use uuid::Uuid;

/// NestGate's knowledge about itself (the only primal it knows)
#[derive(Debug, Clone)]
/// Nestgateselfknowledge
pub struct NestGateSelfKnowledge {
    /// Our service identity
    pub service_id: Uuid,
    /// Capabilities we provide
    pub our_capabilities: Vec<ServiceCapability>,
    /// Our service metadata
    pub metadata: HashMap<String, String>,
}

impl NestGateSelfKnowledge {
    /// Create NestGate self-knowledge
    #[must_use]
    pub fn new() -> Self {
        // Storage capabilities (our primary domain) + Management capabilities
        let capabilities = vec![
            ServiceCapability::storage("create_dataset", "Create ZFS dataset"),
            ServiceCapability::storage("list_datasets", "List all datasets"),
            ServiceCapability::storage("snapshot_dataset", "Create dataset snapshot"),
            ServiceCapability::storage("clone_dataset", "Clone dataset"),
            ServiceCapability::storage("destroy_dataset", "Destroy dataset"),
            ServiceCapability::new(
                CapabilityCategory::Management,
                "health_check",
                "Service health monitoring",
            ),
        ];

        let mut metadata = HashMap::new();
        metadata.insert("service_name".to_string(), DEFAULT_SERVICE_NAME.to_string());
        metadata.insert("version".to_string(), "1.0.0".to_string());
        metadata.insert("primary_capability".to_string(), "storage".to_string());

        Self {
            service_id: Uuid::new_v4(),
            our_capabilities: capabilities,
            metadata,
        }
    }

    /// Check if we can handle a capability locally
    #[must_use]
    pub fn can_handle_capability(&self, category: &CapabilityCategory, operation: &str) -> bool {
        self.our_capabilities
            .iter()
            .any(|cap| cap.category == *category && cap.operation == operation)
    }

    /// Get our advertised capabilities (for discovery by other primals)
    #[must_use]
    pub fn get_advertised_capabilities(&self) -> &[ServiceCapability] {
        &self.our_capabilities
    }
}

impl Default for NestGateSelfKnowledge {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}
