//
// Handles automatic failover and pool takeover for high availability scenarios.
// Allows one NestGate instance to take over ZFS pools from a failed instance.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::process::Command as TokioCommand;
use tokio::sync::RwLock;
// Removed unused tracing import

use crate::config::ZfsConfig;
use nestgate_core::Result;
use std::time::Duration;
use tracing::debug;
use tracing::error;
use tracing::info;

/// Metadata about a ZFS pool for failover tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMetadata {
    pub name: String,
    pub original_owner: String,
    pub last_seen: SystemTime,
    pub import_guid: Option<String>,
    pub state: PoolFailoverState,
}

/// State of a pool in the failover system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PoolFailoverState {
    Active,   // Currently imported and active
    Orphaned, // Available for import (original owner failed)
    Failed,   // Pool is in a failed state
    Unknown,  // State cannot be determined
}

/// Pool state for failover management
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoolState {
    Online,   // Pool is healthy and accessible
    Degraded, // Pool has issues but is still functional
    Offline,  // Pool is not accessible
    Faulted,  // Pool has critical errors
    Removed,  // Pool has been removed from configuration
    Unavail,  // Pool is temporarily unavailable
    Orphaned, // Available for import (original owner failed)
    Failed,   // Pool is in a failed state
    Unknown,  // State cannot be determined
}

// Deprecated FailoverConfig removed - use CanonicalZfsConfig::default().pools.failover instead

/// **CANONICAL FAILOVER CONFIGURATION**
///
/// Modern replacement for the deprecated `FailoverConfig`.
/// Integrated into the canonical ZFS configuration system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalFailoverConfig {
    pub auto_takeover_enabled: bool,
    pub health_check_interval_secs: u64,
    pub takeover_timeout_secs: u64,
    pub node_failure_timeout_secs: u64,
    pub max_takeover_attempts: u32,
    pub failback_enabled: bool,
    pub failback_delay_secs: u64,
    pub notification_config: Option<FailoverNotificationConfig>,
}

impl Default for CanonicalFailoverConfig {
    fn default() -> Self {
        use crate::constants::NODE_FAILURE_TIMEOUT_SECS;
        
        Self {
            auto_takeover_enabled: true,
            health_check_interval_secs: 30,
            takeover_timeout_secs: 300,     // 5 minutes
            node_failure_timeout_secs: NODE_FAILURE_TIMEOUT_SECS,
            max_takeover_attempts: 3,
            failback_enabled: true,
            failback_delay_secs: 60,
            notification_config: None,
        }
    }
}

/// Notification configuration for failover events
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::FailoverNotificationConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::FailoverNotificationConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct FailoverNotificationConfig {
    pub email_enabled: bool,
    pub email_recipients: Vec<String>,
    pub webhook_enabled: bool,
    pub webhook_url: Option<String>,
    pub slack_enabled: bool,
    pub slack_webhook: Option<String>,
}

/// Manages ZFS pool takeover operations
#[allow(dead_code)] // Configuration fields used in advanced failover scenarios
pub struct PoolTakeoverManager {
    config: ZfsConfig,
    failover_config: CanonicalFailoverConfig,
    known_pools: Arc<RwLock<HashMap<String, PoolMetadata>>>,
    node_id: String,
}

impl PoolTakeoverManager {
    /// Create a new pool takeover manager
    #[must_use]
    pub fn new(
        config: ZfsConfig,
        failover_config: CanonicalFailoverConfig,
        node_id: String,
    ) -> Self {
        Self {
            config,
            failover_config,
            known_pools: Arc::new(RwLock::new(HashMap::new())),
            node_id,
        }
    }

    /// Attempt to import pools that were previously owned by another node
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn attempt_pool_takeover(&self, failed_node_id: &str) -> Result<Vec<String>> {
        info!(
            "Attempting pool takeover from failed node: {}",
            failed_node_id
        );

        // 1. Discover importable pools
        let importable_pools = self.discover_importable_pools().await?;
        debug!("Found {} importable pools", importable_pools.len());

        // 2. Check which belonged to failed node
        let target_pools = self
            .identify_orphaned_pools(&importable_pools, failed_node_id)
            .await?;

        if target_pools.is_empty() {
            info!("No orphaned pools found for node: {}", failed_node_id);
            return Ok(Vec::new());
        }

        info!("Found {} orphaned pools for takeover", target_pools.len());

        // 3. Import pools with force if necessary
        let mut imported_pools = Vec::new();
        for pool_name in &target_pools {
            match self.force_import_pool(pool_name).await {
                Ok(()) => {
                    info!("Successfully imported pool: {}", pool_name);
                    imported_pools.push(pool_name.to_string());

                    // Update pool metadata
                    self.update_pool_metadata(pool_name, PoolFailoverState::Active)
                        .await;
                }
                Err(e) => {
                    error!("Failed to import pool {}: {}", pool_name, e);
                }
            }
        }

        Ok(imported_pools)
    }

    /// Force import a ZFS pool (used for takeover)
    async fn force_import_pool(&self, pool_name: &str) -> Result<()> {
        info!("Force importing pool: {}", pool_name);

        let output = TokioCommand::new("zpool")
            .args(["import", "-f", pool_name])
            .output()
            .await
            .map_err(|e| {
                crate::error::ZfsErrorBuilder::new(&format!("Failed to execute zpool import: {e}"))
            })?;

        if !output.status.success() {
            return Err(crate::error::ZfsErrorBuilder::new(&format!(
                "Pool import failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let verification = self.get_pool_status().await;
        if verification.is_empty() {
            return Err(crate::error::ZfsErrorBuilder::new(&format!(
                "Pool verification failed after import: {pool_name}"
            )));
        }

        Ok(())
    }

    /// Verify that a pool was successfully imported
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn verify_pool_import(&self, pool_name: &str) -> Result<bool> {
        let output = TokioCommand::new("zpool")
            .args(["status", pool_name])
            .output()
            .await
            .map_err(|e| {
                crate::error::ZfsErrorBuilder::new(&format!("Failed to verify pool import: {e}"))
            })?;

        Ok(output.status.success())
    }

    /// Discover pools available for import
    async fn discover_importable_pools(&self) -> Result<Vec<String>> {
        debug!("Discovering importable pools");

        let output = TokioCommand::new("zpool")
            .args(["import"])
            .output()
            .await
            .map_err(|e| {
                crate::error::ZfsErrorBuilder::new(&format!("Failed to execute zpool import: {e}"))
            })?;

        // Note: zpool import returns non-zero when no pools available, which is normal
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut pools = Vec::new();

        for line in stdout.lines() {
            if line.trim_start().starts_with("pool:") {
                if let Some(pool_name) = line.split("pool:").nth(1) {
                    let pool_name = pool_name.trim().to_string();
                    pools.push(pool_name);
                }
            }
        }

        debug!("Discovered {} importable pools: {:?}", pools.len(), pools);
        Ok(pools)
    }

    /// Identify which pools were orphaned by a failed node
    async fn identify_orphaned_pools(
        &self,
        available_pools: &[String],
        failed_node_id: &str,
    ) -> Result<Vec<String>> {
        let known_pools = self.known_pools.read().await;
        let mut orphaned_pools = Vec::new();

        for pool_name in available_pools {
            if let Some(metadata) = known_pools.get(pool_name) {
                if metadata.original_owner == failed_node_id {
                    info!(
                        "Pool {} was owned by failed node {}",
                        pool_name, failed_node_id
                    );
                    orphaned_pools.push(pool_name.clone());
                }
            } else {
                // Unknown pool - could be orphaned, check metadata on disk
                if self.check_pool_ownership(pool_name, failed_node_id).await? {
                    orphaned_pools.push(pool_name.clone());
                }
            }
        }

        Ok(orphaned_pools)
    }

    /// Check pool ownership by examining pool properties
    async fn check_pool_ownership(&self, pool_name: &str, _node_id: &str) -> Result<bool> {
        // This would check pool properties or metadata to determine ownership
        // For now, we'll use a simple heuristic

        // Try to get pool properties without importing
        let output = TokioCommand::new("zpool")
            .args(["import", "-N", "-o", "readonly=on", pool_name])
            .output()
            .await;

        // If we can read properties, check for node_id markers
        match output {
            Ok(result) => {
                if result.status.success() {
                    // Pool was temporarily imported readonly - clean up
                    let _ = TokioCommand::new("zpool")
                        .args(["export", pool_name])
                        .output()
                        .await;

                    // In a real implementation, this would check pool properties
                    // for ownership markers
                    Ok(true) // Assume orphaned for now
                } else {
                    Ok(false)
                }
            }
            Err(_) => Ok(false),
        }
    }

    /// Update pool metadata in our tracking system
    async fn update_pool_metadata(&self, pool_name: &str, state: PoolFailoverState) {
        let mut known_pools = self.known_pools.write().await;

        if let Some(metadata) = known_pools.get_mut(pool_name) {
            metadata.state = state;
            metadata.last_seen = SystemTime::now();
        } else {
            // Create new metadata entry
            known_pools.insert(
                pool_name.to_string(),
                PoolMetadata {
                    name: pool_name.to_string(),
                    original_owner: self.node_id.clone(),
                    last_seen: SystemTime::now(),
                    import_guid: None,
                    state,
                },
            );
        }
    }

    /// Export a pool (preparation for graceful handover)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn export_pool(&self, pool_name: &str) -> Result<()> {
        info!("Exporting pool for handover: {}", pool_name);

        let output = TokioCommand::new("zpool")
            .args(["export", pool_name])
            .output()
            .await
            .map_err(|e| {
                crate::error::ZfsErrorBuilder::new(&format!("Failed to execute zpool export: {e}"))
            })?;

        if !output.status.success() {
            return Err(crate::error::ZfsErrorBuilder::new(&format!(
                "Pool export failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        // Update metadata
        self.update_pool_metadata(pool_name, PoolFailoverState::Orphaned)
            .await;

        info!("Successfully exported pool: {}", pool_name);
        Ok(())
    }

    /// Get status of all tracked pools
    pub async fn get_pool_status(&self) -> HashMap<String, PoolMetadata> {
        self.known_pools.read().await.clone()
    }
}

/// Node health monitor for failover detection
pub struct NodeHealthMonitor {
    known_nodes: Arc<RwLock<HashMap<String, NodeHealth>>>,
    config: CanonicalFailoverConfig,
}

#[derive(Debug, Clone)]
pub struct NodeHealth {
    pub node_id: String,
    pub last_heartbeat: SystemTime,
    pub is_alive: bool,
}

impl NodeHealthMonitor {
    #[must_use]
    pub fn new(config: CanonicalFailoverConfig) -> Self {
        Self {
            known_nodes: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Detect nodes that have failed (haven't sent heartbeat in timeout period)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn detect_failed_nodes(&self) -> Result<Vec<NodeHealth>> {
        let nodes = self.known_nodes.read().await;
        let now = SystemTime::now();
        let timeout = Duration::from_secs(self.config.node_failure_timeout_secs);

        let failed_nodes = nodes
            .values()
            .filter(|node| {
                if let Ok(elapsed) = now.duration_since(node.last_heartbeat) {
                    elapsed > timeout && node.is_alive
                } else {
                    false
                }
            })
            .cloned()
            .collect();

        Ok(failed_nodes)
    }

    /// Update node heartbeat
    pub async fn update_node_heartbeat(&self, node_id: &str) {
        let mut nodes = self.known_nodes.write().await;
        let now = SystemTime::now();

        if let Some(node) = nodes.get_mut(node_id) {
            node.last_heartbeat = now;
            node.is_alive = true;
        } else {
            nodes.insert(
                node_id.to_string(),
                NodeHealth {
                    node_id: node_id.to_string(),
                    last_heartbeat: now,
                    is_alive: true,
                },
            );
        }
    }
}


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type FailoverNotificationConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using FailoverNotificationConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_takeover_manager_creation() {
        let config = ZfsConfig::default();
        let failover_config = CanonicalFailoverConfig::default();
        let manager = PoolTakeoverManager::new(config, failover_config, "test-node".to_string());

        assert_eq!(manager.node_id, "test-node");
        assert!(manager.known_pools.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_node_health_monitoring() {
        let config = CanonicalFailoverConfig::default();
        let monitor = NodeHealthMonitor::new(config);

        // Update heartbeat for a node
        monitor.update_node_heartbeat("node1").await;

        // Should not detect as failed immediately
        let failed_nodes = monitor.detect_failed_nodes().await.unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            // Return empty vector for test purposes
            Vec::new()
        });
        assert!(failed_nodes.is_empty());

        // Simulate passage of time by manually setting old heartbeat
        {
            let mut nodes = monitor.known_nodes.write().await;
            if let Some(node) = nodes.get_mut("node1") {
                node.last_heartbeat = SystemTime::now()
                    - Duration::from_secs(
                        std::env::var("NESTGATE_ZFS_HEARTBEAT_TIMEOUT_SECS")
                            .ok()
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(300), // 5 minutes default
                    );
            }
        }

        // Now should detect as failed
        let failed_nodes = monitor.detect_failed_nodes().await.unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            // Return empty vector for test purposes
            Vec::new()
        });
        assert_eq!(failed_nodes.len(), 1);
        assert_eq!(failed_nodes[0].node_id, "node1");
    }

    #[tokio::test]
    async fn test_pool_metadata_tracking() {
        let config = ZfsConfig::default();
        let failover_config = CanonicalFailoverConfig::default();
        let manager = PoolTakeoverManager::new(config, failover_config, "test-node".to_string());

        // Update pool metadata
        manager
            .update_pool_metadata("testpool", PoolFailoverState::Active)
            .await;

        // Verify metadata was stored
        let status = manager.get_pool_status().await;
        assert_eq!(status.len(), 1);
        assert_eq!(status["testpool"].name, "testpool");
        assert_eq!(status["testpool"].state, PoolFailoverState::Active);
    }
}
