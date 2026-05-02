// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! In-memory registry and capability index for discovered services.

use super::types::{CapabilityCategory, DiscoveredService, ServiceCapability};
use std::collections::HashMap;
use uuid::Uuid;

/// Registry of all discovered capabilities in the ecosystem
#[derive(Debug, Default)]
/// Capabilityregistry
pub struct CapabilityRegistry {
    /// All discovered services
    services: HashMap<Uuid, DiscoveredService>,
    /// Index by capability category
    capability_index: HashMap<CapabilityCategory, Vec<Uuid>>,
    /// Our own capabilities that we advertise
    our_capabilities: Vec<ServiceCapability>,
    /// Our service information
    our_service: Option<DiscoveredService>,
}

impl CapabilityRegistry {
    /// Create a new capability registry
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register our own service and capabilities (NestGate only knows itself)
    pub fn register_self(&mut self, service: DiscoveredService) {
        // Index our capabilities
        for capability in &service.capabilities {
            self.capability_index
                .entry(capability.category.clone())
                .or_default()
                .push(service.service_id);
        }

        self.our_capabilities.clone_from(&service.capabilities);
        self.services.insert(service.service_id, service.clone());
        self.our_service = Some(service);
    }

    /// Register a discovered service
    pub fn register_service(&mut self, service: DiscoveredService) {
        // Index capabilities
        for capability in &service.capabilities {
            self.capability_index
                .entry(capability.category.clone())
                .or_default()
                .push(service.service_id);
        }

        self.services.insert(service.service_id, service);
    }

    /// Find services that provide a specific capability
    #[must_use]
    pub fn find_providers(
        &self,
        category: &CapabilityCategory,
        operation: &str,
    ) -> Vec<&DiscoveredService> {
        let mut providers = Vec::new();

        if let Some(service_ids) = self.capability_index.get(category) {
            for service_id in service_ids {
                if let Some(service) = self.services.get(service_id)
                    && service.provides_capability(category, operation)
                    && service.healthy
                {
                    providers.push(service);
                }
            }
        }

        providers
    }

    /// Get our own advertised capabilities
    #[must_use]
    pub fn our_capabilities(&self) -> &[ServiceCapability] {
        &self.our_capabilities
    }

    /// Get all discovered services
    #[must_use]
    pub fn all_services(&self) -> Vec<&DiscoveredService> {
        self.services.values().collect()
    }

    /// Remove unhealthy services
    pub fn cleanup_unhealthy(&mut self) {
        let unhealthy_ids: Vec<Uuid> = self
            .services
            .iter()
            .filter(|(_, service)| !service.healthy)
            .map(|(id, _)| *id)
            .collect();

        for id in unhealthy_ids {
            self.remove_service(&id);
        }
    }

    /// Remove a service from the registry
    pub fn remove_service(&mut self, service_id: &Uuid) {
        if let Some(service) = self.services.remove(service_id) {
            // Remove from capability index
            for capability in &service.capabilities {
                if let Some(service_ids) = self.capability_index.get_mut(&capability.category) {
                    service_ids.retain(|id| id != service_id);
                }
            }
        }
    }
}
