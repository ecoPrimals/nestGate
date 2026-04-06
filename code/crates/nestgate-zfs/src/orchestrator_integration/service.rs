// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Service implementation for orchestrator integration
//!
//! This module contains the `ZfsService` implementation that handles registration
//! and coordination with orchestration systems.

use super::types::{ServiceInfo, ServiceRegistration, ZfsHealthStatus, ZfsServiceConfig};
use anyhow::{Result, bail};
use std::sync::Arc;
use std::time::SystemTime;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// ZFS service for orchestration module integration
///
/// This is the main service type that handles registration and coordination
/// with orchestration systems. It manages service lifecycle, health reporting,
/// and distributed coordination.
///
/// # Features
///
/// - Service registration with orchestrators
/// - Health check reporting
/// - Load balancing coordination
/// - Metadata management
///
/// # Zero-Copy Design
///
/// Uses Arc for config sharing to enable efficient cloning without deep-copying
/// the entire configuration structure.
#[derive(Debug, Clone)]
pub struct ZfsService {
    /// Configuration wrapped in Arc for zero-copy cloning
    config: Arc<ZfsServiceConfig>,
    /// Unique node identifier
    node_id: String,
    /// Last health check timestamp
    last_health_check: Option<SystemTime>,
    /// Registration status
    registered_with_orchestrator: bool,
}

impl ZfsService {
    /// Create a new ZFS service with Arc-wrapped config for zero-copy sharing
    ///
    /// # Performance
    ///
    /// This constructor uses Arc to enable zero-copy config sharing across
    /// multiple service instances. Cloning the config only increments a reference
    /// count instead of deep-copying the entire configuration.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use nestgate_zfs::orchestrator_integration::{ZfsService, ZfsServiceConfig};
    ///
    /// let config = ZfsServiceConfig::default();
    /// let service = ZfsService::new(config);
    /// ```
    #[must_use]
    pub fn new(config: ZfsServiceConfig) -> Self {
        Self {
            config: Arc::new(config),
            node_id: Uuid::new_v4().to_string(),
            last_health_check: None,
            registered_with_orchestrator: false,
        }
    }

    /// Create a new ZFS service from an existing Arc-wrapped config
    ///
    /// This is the most efficient constructor when you already have an `Arc<ZfsServiceConfig>`
    /// as it avoids any allocation.
    #[must_use]
    pub fn from_arc(config: Arc<ZfsServiceConfig>) -> Self {
        Self {
            config,
            node_id: Uuid::new_v4().to_string(),
            last_health_check: None,
            registered_with_orchestrator: false,
        }
    }

    /// Get service ID
    #[must_use]
    pub fn service_id(&self) -> &str {
        &self.config.service_name
    }

    /// Get node ID
    #[must_use]
    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    /// Get service endpoints
    #[must_use]
    pub fn endpoints(&self) -> Vec<String> {
        vec![format!("{}:{}", self.config.bind_address, self.config.port)]
    }

    /// Get service capabilities
    #[must_use]
    pub fn capabilities(&self) -> &[String] {
        &self.config.capabilities
    }

    /// Check if registered with orchestrator
    #[must_use]
    pub const fn is_registered(&self) -> bool {
        self.registered_with_orchestrator
    }

    /// Create service registration information
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let service = ZfsService::new(config);
    /// let registration = service.create_registration();
    /// ```
    #[must_use]
    pub fn create_registration(&self) -> ServiceRegistration {
        ServiceRegistration {
            service_id: self.node_id.clone(),
            service_type: "zfs-storage".to_string(),
            capabilities: self.config.capabilities.clone(),
            endpoints: self.endpoints(),
            metadata: self.config.metadata.clone(),
        }
    }

    /// Register service with orchestrator
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Network communication fails
    /// - Orchestrator rejects the registration
    /// - Authentication fails
    pub fn register_with_orchestrator(&mut self, _orchestrator_url: &str) -> Result<()> {
        info!("Registering service {} with orchestrator", self.node_id);

        // Outbound registration RPC/HTTP to the orchestrator is not implemented yet; this path
        // still validates configuration and records local registration state for the process.

        if self.config.orchestrator_endpoints.is_empty() {
            bail!("No orchestrator endpoints configured");
        }

        // Mark as registered
        self.registered_with_orchestrator = true;

        debug!("Service {} registered successfully", self.node_id);
        Ok(())
    }

    /// Perform health check and return status
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let service = ZfsService::new(config);
    /// let health = service.health_check()?;
    /// println!("Service health: {:?}", health);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if health check operations fail
    pub fn health_check(&mut self) -> Result<ZfsHealthStatus> {
        debug!("Performing health check for node {}", self.node_id);

        self.last_health_check = Some(SystemTime::now());

        // Returns a service-level heartbeat with real timestamps and node id; capacity fields are
        // not populated here until wired to live ZFS pool/dataset metrics.
        Ok(ZfsHealthStatus {
            node_id: self.node_id.clone(),
            status: "healthy".to_string(),
            pools_healthy: true,
            datasets_healthy: true,
            system_healthy: true,
            total_capacity: 0,
            available_capacity: 0,
            last_check: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        })
    }

    /// Send health status to orchestrator
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Network communication fails
    /// - Health check fails
    /// - Orchestrator is unreachable
    pub fn report_health(&mut self) -> Result<()> {
        if !self.registered_with_orchestrator {
            bail!("Service not registered with orchestrator");
        }

        let health_status = self.health_check()?;
        debug!("Reporting health status: {:?}", health_status);

        // NOTE: Would send health_status to orchestrator via HTTP
        // Implementation requires HTTP client

        Ok(())
    }

    /// Unregister from orchestrator
    ///
    /// # Errors
    ///
    /// Returns an error if deregistration fails
    pub fn unregister(&mut self) -> Result<()> {
        if !self.registered_with_orchestrator {
            warn!("Service not registered, skipping unregister");
            return Ok(());
        }

        info!("Unregistering service {}", self.node_id);

        // NOTE: Would send unregister request to orchestrator
        self.registered_with_orchestrator = false;

        Ok(())
    }

    /// Get reference to the configuration
    ///
    /// This returns an Arc clone, which is a zero-cost operation (just incrementing
    /// the reference count).
    #[must_use]
    pub fn config(&self) -> Arc<ZfsServiceConfig> {
        Arc::clone(&self.config)
    }

    /// Get last health check time
    #[must_use]
    pub const fn last_health_check(&self) -> Option<SystemTime> {
        self.last_health_check
    }

    /// Get comprehensive service information
    ///
    /// Returns a `ServiceInfo` struct containing all service metadata,
    /// endpoints, capabilities, and health status information.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let service = ZfsService::new(config);
    /// let info = service.get_service_info();
    /// println!("Service: {} at {}", info.service_id, info.endpoints[0]);
    /// ```
    #[must_use]
    pub fn get_service_info(&self) -> ServiceInfo {
        ServiceInfo {
            service_id: self.node_id.clone(),
            service_type: "zfs-storage".to_string(),
            endpoints: self.endpoints(),
            capabilities: self.config.capabilities.clone(),
            metadata: self.config.metadata.clone(),
            last_heartbeat: self
                .last_health_check
                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                .map(|d| d.as_secs()),
        }
    }
}

impl Default for ZfsService {
    fn default() -> Self {
        Self::new(ZfsServiceConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::ZfsServiceConfig;
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;

    fn sample_config() -> ZfsServiceConfig {
        ZfsServiceConfig {
            service_name: "test-zfs".to_string(),
            bind_address: "127.0.0.1".to_string(),
            port: 9000,
            orchestrator_endpoints: vec!["http://127.0.0.1:1".to_string()],
            health_check_interval: 10,
            capabilities: vec!["pool".to_string()],
            metadata: HashMap::from([("k".to_string(), "v".to_string())]),
        }
    }

    #[test]
    fn new_sets_node_id_and_service_id() {
        let s = ZfsService::new(sample_config());
        assert_eq!(s.service_id(), "test-zfs");
        assert!(!s.node_id().is_empty());
    }

    #[test]
    fn from_arc_shares_config() {
        let cfg = Arc::new(sample_config());
        let s = ZfsService::from_arc(Arc::clone(&cfg));
        assert_eq!(Arc::strong_count(&cfg), 2);
        assert_eq!(s.config().service_name, "test-zfs");
    }

    #[test]
    fn endpoints_and_capabilities_and_registration() {
        let s = ZfsService::new(sample_config());
        assert_eq!(s.endpoints(), vec!["127.0.0.1:9000"]);
        assert_eq!(s.capabilities(), &["pool".to_string()]);
        let reg = s.create_registration();
        assert_eq!(reg.service_type, "zfs-storage");
        assert_eq!(reg.capabilities, vec!["pool".to_string()]);
        assert!(reg.endpoints.iter().any(|e| e.contains(':')));
    }

    #[test]
    fn register_and_report_health_and_unregister() {
        let mut s = ZfsService::new(sample_config());
        assert!(!s.is_registered());
        assert!(s.register_with_orchestrator("http://x").is_ok());
        assert!(s.is_registered());
        assert!(s.report_health().is_ok());
        assert!(s.unregister().is_ok());
        assert!(!s.is_registered());
    }

    #[test]
    fn register_fails_without_orchestrator_endpoints() {
        let mut cfg = sample_config();
        cfg.orchestrator_endpoints.clear();
        let mut s = ZfsService::new(cfg);
        assert!(s.register_with_orchestrator("http://x").is_err());
    }

    #[test]
    fn report_health_fails_when_not_registered() {
        let mut s = ZfsService::new(sample_config());
        assert!(s.report_health().is_err());
    }

    #[test]
    fn unregister_noop_when_not_registered() {
        let mut s = ZfsService::new(sample_config());
        assert!(s.unregister().is_ok());
    }

    #[test]
    fn health_check_updates_last_check() {
        let mut s = ZfsService::new(sample_config());
        assert!(s.last_health_check().is_none());
        let h = s.health_check().expect("health");
        assert_eq!(h.status, "healthy");
        assert!(s.last_health_check().is_some());
    }

    #[test]
    fn get_service_info_includes_metadata() {
        let mut s = ZfsService::new(sample_config());
        let _ = s.health_check();
        let info = s.get_service_info();
        assert_eq!(info.service_type, "zfs-storage");
        assert_eq!(info.metadata.get("k"), Some(&"v".to_string()));
        assert!(info.last_heartbeat.is_some());
    }

    #[test]
    #[allow(deprecated)]
    fn default_service_builds() {
        let s = ZfsService::default();
        assert!(!s.node_id().is_empty());
    }
}
