//! VLAN (Virtual LAN) management
//!
//! This module provides VLAN configuration and management functionality

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

// Use nestgate_core for error handling
use nestgate_core::{NestGateError, Result};

/// VLAN configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct VlanManager {
    vlans: Arc<RwLock<HashMap<u16, VlanConfig>>>,
}

impl VlanManager {
    /// Create a new VLAN manager
    pub fn new() -> Self {
        Self {
            vlans: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add a VLAN configuration
    pub async fn add_vlan(&self, vlan: VlanConfig) -> Result<()> {
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

        Ok(())
    }

    /// Remove a VLAN
    pub async fn remove_vlan(&self, vlan_id: u16) -> Result<()> {
        let mut vlans = self.vlans.write().await;
        if vlans.remove(&vlan_id).is_none() {
            return Err(NestGateError::NotFound(format!("VLAN {vlan_id} not found")));
        }

        tracing::info!("Removed VLAN {}", vlan_id);
        Ok(())
    }

    /// Get a VLAN configuration
    pub async fn get_vlan(&self, vlan_id: u16) -> Result<VlanConfig> {
        let vlans = self.vlans.read().await;
        vlans
            .get(&vlan_id)
            .cloned()
            .ok_or_else(|| NestGateError::NotFound(format!("VLAN {vlan_id} not found")))
    }

    /// List all VLANs
    pub async fn list_vlans(&self) -> Result<Vec<VlanConfig>> {
        let vlans = self.vlans.read().await;
        Ok(vlans.values().cloned().collect())
    }

    /// Update a VLAN configuration
    pub async fn update_vlan(&self, vlan_id: u16, updated_vlan: VlanConfig) -> Result<()> {
        if updated_vlan.vlan_id != vlan_id {
            return Err(NestGateError::InvalidInput(
                "Cannot change VLAN ID in update operation".to_string(),
            ));
        }

        let mut vlans = self.vlans.write().await;
        if !vlans.contains_key(&vlan_id) {
            return Err(NestGateError::NotFound(format!("VLAN {vlan_id} not found")));
        }

        tracing::info!("Updating VLAN {}: {}", vlan_id, updated_vlan.name);
        vlans.insert(vlan_id, updated_vlan);

        Ok(())
    }

    /// Enable a VLAN
    pub async fn enable_vlan(&self, vlan_id: u16) -> Result<()> {
        let mut vlans = self.vlans.write().await;
        if let Some(vlan) = vlans.get_mut(&vlan_id) {
            vlan.enabled = true;
            tracing::info!("Enabled VLAN {}", vlan_id);
            Ok(())
        } else {
            Err(NestGateError::NotFound(format!("VLAN {vlan_id} not found")))
        }
    }

    /// Disable a VLAN
    pub async fn disable_vlan(&self, vlan_id: u16) -> Result<()> {
        let mut vlans = self.vlans.write().await;
        if let Some(vlan) = vlans.get_mut(&vlan_id) {
            vlan.enabled = false;
            tracing::info!("Disabled VLAN {}", vlan_id);
            Ok(())
        } else {
            Err(NestGateError::NotFound(format!("VLAN {vlan_id} not found")))
        }
    }

    /// Get enabled VLANs only
    pub async fn get_enabled_vlans(&self) -> Result<Vec<VlanConfig>> {
        let vlans = self.vlans.read().await;
        Ok(vlans
            .values()
            .filter(|vlan| vlan.enabled)
            .cloned()
            .collect())
    }
}

impl Default for VlanManager {
    fn default() -> Self {
        Self::new()
    }
}
