// Core Interface Definitions
//! Core Interfaces functionality and utilities.
// This module provides the fundamental interface traits and types that form
//! the foundation of the NestGate interface system.

use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ==================== SECTION ====================

/// Universal conversion trait for interface compatibility
pub trait ToUnified<T> {
    /// Convert to unified interface type
    fn to_unified(self) -> T;
}
/// Universal configuration interface
pub trait UniversalConfigInterface {
    type Config: Clone + Send + Sync;
    /// Get current configuration
    fn get_config(&self) -> impl std::future::Future<Output = Result<Self::Config>> + Send;

    /// Update configuration
    fn update_config(&mut self, config: Self::Config) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Validate configuration
    fn validate_config(&self, config: &Self::Config) -> Result<()>;
}

/// Universal event interface
pub trait UniversalEventInterface {
    type Event: Clone + Send + Sync;
    /// Emit an event
    fn emit_event(&self, event: Self::Event) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Subscribe to events
    async fn subscribe_events(
        &self,
    ) -> Result<Box<dyn futures::Stream<Item = Self::Event> + Send + Unpin>>;
}

/// Universal provider interface
pub trait UniversalProviderInterface {
    type Request: Send + Sync;
    type Response: Send + Sync;
    /// Process a request
    fn process_request(&self, request: Self::Request) -> impl std::future::Future<Output = Result<Self::Response>> + Send;

    /// Check provider capabilities
    fn get_capabilities(&self) -> Vec<String>;

    /// Get provider metadata
    fn get_metadata(&self) -> HashMap<String, String>;
}

// ==================== SECTION ====================

/// Universal interface metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceMetadata {
    pub interface_id: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub properties: HashMap<String, String>,
}
/// Universal interface status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterfaceStatus {
    Active,
    Inactive,
    Error(String),
    Maintenance,
}
impl Default for InterfaceStatus {
    fn default() -> Self {
        Self::Active
    }
}

// ==================== SECTION ====================

/// Interface builder for creating standardized interfaces
pub struct InterfaceBuilder {
    metadata: InterfaceMetadata,
}
impl InterfaceBuilder {
    /// Create a new interface builder
    #[must_use]
    pub fn new(interface_id: String, version: String) -> Self {
        Self {
            metadata: InterfaceMetadata {
                interface_id,
                version,
                capabilities: Vec::new(),
                properties: HashMap::new(),
            },
        }
    }

    /// Add capability to interface
    #[must_use]
    pub fn with_capability(mut self, capability: String) -> Self {
        self.metadata.capabilities.push(capability);
        self
    }

    /// Add property to interface
    #[must_use]
    pub fn with_property(mut self, key: String, value: String) -> Self {
        self.metadata.properties.insert(key, value);
        self
    }

    /// Build the interface metadata
    pub const fn build(self) -> InterfaceMetadata {
        self.metadata
    }
}

// ==================== SECTION ====================

/// Registry for managing interface implementations
pub struct InterfaceRegistry {
    interfaces: HashMap<String, InterfaceMetadata>,
}
impl InterfaceRegistry {
    /// Create a new interface registry
    #[must_use]
    pub fn new() -> Self {
        Self {
            interfaces: HashMap::new(),
        }
    }

    /// Register an interface
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn register(&mut self, metadata: InterfaceMetadata) -> Result<()>  {
        let interface_id = metadata.interface_id.clone();
        self.interfaces.insert(interface_id, metadata);
        Ok(())
    }

    /// Get interface metadata
    pub const fn get_interface(&self, interface_id: &str) -> Option<&InterfaceMetadata> {
        self.interfaces.get(interface_id)
    }

    /// List all registered interfaces
    pub const fn list_interfaces(&self) -> Vec<&InterfaceMetadata> {
        self.interfaces.values().collect()
    }
}

impl Default for InterfaceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
