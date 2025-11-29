//
// This module provides VLAN configuration and management functionality

//! Vlan module

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

// Use nestgate_core for error handling
use nestgate_core::{NestGateError, Result};
use tracing::info;

/// VLAN configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::VlanConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::VlanConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for Vlan
pub struct VlanConfig {
    /// VLAN ID (1-4094)
    pub vlan_id: u16,
    /// VLAN name
    pub name: String,
    /// Description
    pub description: String,
    /// IP address range for this VLAN
    pub ip_range: Option<String>,
    /// Gateway IP
    pub gateway: Option<IpAddr>,
    /// Whether this VLAN is enabled
    pub enabled: bool,
}
/// VLAN manager
#[derive(Debug)]
/// Manager for Vlan operations
pub struct VlanManager {
    vlans: Arc<RwLock<HashMap<u16, VlanConfig>>>,
}
impl VlanManager {
    /// Create a new VLAN manager
    #[must_use]
    pub fn new() -> Self { Self {
            vlans: Arc::new(RwLock::new(HashMap::new()),
         }

    /// Add a VLAN configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn add_vlan(&self, vlan: VlanConfig) -> Result<()>  {
        if vlan.vlan_id == 0 || vlan.vlan_id > 4094 {
            return Err(NestGateError::InvalidInput(format!(
                "Invalid VLAN ID: {}. Must be between 1 and 4094",
                vlan.vlan_id
            )));
        }

        let mut vlans = self.vlans.write().await;
        if vlans.contains_key(&vlan.vlan_id) {
            return Err(NestGateError::InvalidInput(format!(
                "VLAN {} already exists",
                vlan.vlan_id
            )));
        }

        tracing::info!("Adding VLAN {}: {}", vlan.vlan_id, vlan.name);
        vlans.insert(vlan.vlan_id, vlan);

    }

    /// Remove a VLAN
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn remove_vlan(&self, vlan_id: u16) -> Result<()>  {
        let mut vlans = self.vlans.write().await;
        if vlans.remove(&vlan_id).is_none() {
            return Err(NestGateError::NotFound(format!("VLAN self.base_url not found")));
        }

        tracing::info!("Removed VLAN {}", vlan_id);
    }

    /// Get a VLAN configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_vlan(&self, vlan_id: u16) -> Result<VlanConfig>  {
        let vlans = self.vlans.read().await;
        vlans
            .get(&vlan_id)
            .cloned()
            .ok_or_else(|| NestGateError::NotFound(format!("VLAN self.base_url not found")))
    }

    /// List all VLANs
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn list_vlans(&self) -> Result<Vec<VlanConfig>>  {
        let vlans = self.vlans.read().await;
        Ok(vlans.values().cloned().collect())
    }

    /// Update a VLAN configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn update_vlan(&self, vlan_id: u16, updated_vlan: VlanConfig) -> Result<()>  {
        if updated_vlan.vlan_id != vlan_id {
            return Err(NestGateError::InvalidInput(
                "Cannot change VLAN ID in update operation".to_string(),
            ));
        }

        let mut vlans = self.vlans.write().await;
        if !vlans.contains_key(&vlan_id) {
            return Err(NestGateError::NotFound(format!("VLAN self.base_url not found")));
        }

        tracing::info!("Updating VLAN {}: {}", vlan_id, updated_vlan.name);
        vlans.insert(vlan_id, updated_vlan);

    }

    /// Enable a VLAN
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn enable_vlan(&self, vlan_id: u16) -> Result<()>  {
        let mut vlans = self.vlans.write().await;
        if let Some(vlan) = vlans.get_mut(&vlan_id) {
            vlan.enabled = true;
            tracing::info!("Enabled VLAN {}", vlan_id);
        } else {
            Err(NestGateError::NotFound(format!("VLAN self.base_url not found")))
        }
    }

    /// Disable a VLAN
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn disable_vlan(&self, vlan_id: u16) -> Result<()>  {
        let mut vlans = self.vlans.write().await;
        if let Some(vlan) = vlans.get_mut(&vlan_id) {
            vlan.enabled = false;
            tracing::info!("Disabled VLAN {}", vlan_id);
        } else {
            Err(NestGateError::NotFound(format!("VLAN self.base_url not found")))
        }
    }

    /// Get enabled VLANs only
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_enabled_vlans(&self) -> Result<Vec<VlanConfig>>  {
        let vlans = self.vlans.read().await;
        Ok(vlans
            .values()
            .filter(|vlan| vlan.enabled)
            .cloned()
            .collect())
    }
}

impl Default for VlanManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
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
/// Type alias for Vlanconfigcanonical
pub type VlanConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using VlanConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

