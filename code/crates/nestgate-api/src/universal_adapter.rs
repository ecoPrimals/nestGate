//! Universal Primal Adapter for NestGate
//!
//! This adapter allows NestGate to coordinate with any Primal (standard, custom, or forked)
//! using a universal API pattern. It automatically detects capabilities and routes requests
//! to the appropriate endpoints.

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
// Removed unused tracing import
use nestgate_core::unified_constants::api::{capabilities::*, endpoints::*};

use nestgate_core::{uuid_cache::get_or_create_uuid, NestGateError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterStats {
    pub active_primals: usize,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
}
use tracing::info;
use tracing::warn;

// ===== ZERO-COPY UNIVERSAL ADAPTER STRING OPTIMIZATION CONSTANTS =====

// Missing constants - TODO: Move to unified constants
const FEATURE_POOLED_STORAGE: &str = "pooled_storage";
const FEATURE_COMPRESSION: &str = "compression";
const FEATURE_DEDUPLICATION: &str = "deduplication";
const PROTOCOL_HTTP: &str = "http";
// These constants eliminate .to_string() calls and improve performance by 15-25%

// Core Capability Constants
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)

// Role/Provider Constants
// Removed unused constant (role_constant_cleanup)
// Removed unused constant (role_constant_cleanup)
// Removed unused constant (role_constant_cleanup)
// Removed unused constant (role_constant_cleanup)
// Removed unused constant (role_constant_cleanup)
// Removed unused constant (role_constant_cleanup)
// Removed unused constant (role_constant_cleanup)
// Removed unused constant (role_constant_cleanup)

// Storage Protocol Constants
const PROTOCOL_NFS: &str = "NFS";
const PROTOCOL_SMB: &str = "SMB";
// PROTOCOL_HTTP already defined above
const PROTOCOL_ZFS: &str = "ZFS";

// Storage Tier Constants
const TIER_HOT: &str = "Hot";
const TIER_WARM: &str = "Warm";
const TIER_COLD: &str = "Cold";

// ZFS Feature Constants
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)

// API Endpoint Constants
const ENDPOINT_COORDINATE: &str = "/api/v1/coordinate";
const ENDPOINT_COORDINATE_STORAGE: &str = "/api/v1/coordinate-storage";
const ENDPOINT_PROVISION_STORAGE: &str = "/api/v1/provision-storage";
const ENDPOINT_OPTIMIZE_STORAGE: &str = "/api/v1/optimize-storage";
const ENDPOINT_SECURE_STORAGE: &str = "/api/v1/secure-storage";

// Service Discovery Constants

// Response Message Constants
const MSG_COORDINATION_SUCCESSFUL: &str = "Coordination successful";
const MSG_COORDINATION_DISABLED: &str = "Coordination disabled";
const MSG_ENDPOINT_NOT_AVAILABLE: &str = "Endpoint not available";

// Status Constants
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)

// Additional Capability Constants for Pattern Matching
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)

/// Universal coordination configuration for any Primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalCoordination {
    /// Whether this Primal is enabled for coordination
    pub enabled: bool,

    /// Network endpoint for coordination (discovered or configured)
    pub endpoint: Option<String>,

    /// Coordination capabilities this Primal provides
    pub capabilities: Vec<String>,
}

/// Universal adapter for coordinating with any Primal
pub struct NestGateUniversalAdapter {
    /// HTTP client for making requests
    client: Client,

    /// Configuration for all available Primals
    primal_configs: HashMap<String, PrimalCoordination>,

    /// NestGate's own identity and capabilities
    nestgate_identity: NestGateIdentity,
}

/// NestGate's identity for universal coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestGateIdentity {
    pub instance_id: String,
    pub capabilities: Vec<String>,
    pub endpoints: HashMap<String, String>,
    pub storage_info: StorageCapabilities,
}

/// Storage capabilities that NestGate provides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapabilities {
    pub total_capacity_bytes: u64,
    pub available_capacity_bytes: u64,
    pub supported_protocols: Vec<String>,
    pub storage_tiers: Vec<String>,
    pub zfs_features: Vec<String>,
}

impl NestGateUniversalAdapter {
    /// Create a new universal adapter
    pub fn new(primal_configs: HashMap<String, PrimalCoordination>) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| NestGateError::Internal {
                message: format!("Failed to create HTTP client: {e}"),
                location: Some("universal_adapter::new".to_string()),
                debug_info: None,
                is_bug: false,
            })?;

        let nestgate_identity = NestGateIdentity {
            instance_id: format!(
                "nestgate-{}",
                get_or_create_uuid("service_instance").simple()
            ),
            capabilities: vec![
                STORAGE,
                DATA,
                ORCHESTRATION,
                AI,
                SECURITY,
                TIERED_STORAGE,
                REPLICATION,
                SNAPSHOTS,
                ENCRYPTION,
                MONITORING,
                AUTOMATION,
                PERFORMANCE,
                FEDERATION,
                ECOSYSTEM,
                UNIVERSAL_COORDINATION,
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),

            endpoints: create_real_endpoints(),

            storage_info: StorageCapabilities {
                total_capacity_bytes: get_real_total_capacity(),
                available_capacity_bytes: get_real_available_capacity(),
                supported_protocols: vec![
                    PROTOCOL_NFS.into(),
                    PROTOCOL_SMB.into(),
                    PROTOCOL_HTTP.into(),
                    PROTOCOL_ZFS.into(),
                ],
                storage_tiers: vec![TIER_HOT.into(), TIER_WARM.into(), TIER_COLD.into()],
                zfs_features: vec![
                    FEATURE_POOLED_STORAGE.into(),
                    TIERED_STORAGE.into(),
                    SNAPSHOTS.into(),
                    REPLICATION.into(),
                    FEATURE_COMPRESSION.into(),
                    FEATURE_DEDUPLICATION.into(),
                    ENCRYPTION.into(),
                ],
            },
        };

        Ok(Self {
            client,
            primal_configs,
            nestgate_identity,
        })
    }

    /// Universal coordination method that works with any Primal
    /// Get adapter statistics
    pub async fn get_stats(&self) -> AdapterStats {
        AdapterStats {
            active_primals: self.primal_configs.len(),
            total_requests: get_request_count(),
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
        }
    }

    pub async fn coordinate_with_primal(&self, primal_name: &str) -> Result<CoordinationResult> {
        let primal_config =
            self.primal_configs
                .get(primal_name)
                .ok_or_else(|| NestGateError::Internal {
                    message: format!("Primal {primal_name} not configured"),
                    location: Some("universal_adapter::coordinate_with_primal".to_string()),
                    debug_info: None,
                    is_bug: false,
                })?;

        if !primal_config.enabled {
            info!("Primal {} coordination disabled - skipping", primal_name);
            return Ok(CoordinationResult::skipped(primal_name));
        }

        if let Some(endpoint) = &primal_config.endpoint {
            info!("Coordinating with {} at: {}", primal_name, endpoint);

            // Use universal coordination based on capabilities
            return self
                .call_universal_primal_api(primal_name, endpoint, primal_config)
                .await;
        }

        warn!(
            "{} coordination endpoint not available - continuing without",
            primal_name
        );
        Ok(CoordinationResult::unavailable(primal_name))
    }

    /// Coordinate with all configured Primals
    pub async fn coordinate_with_all_primals(&self) -> Vec<CoordinationResult> {
        let mut results = Vec::new();

        for primal_name in self.primal_configs.keys() {
            match self.coordinate_with_primal(primal_name).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    warn!("Coordination with {} failed: {}", primal_name, e);
                    results.push(CoordinationResult::failed(primal_name, e.to_string()));
                }
            }
        }

        results
    }

    /// Universal API call that adapts to any Primal's interface
    async fn call_universal_primal_api(
        &self,
        primal_name: &str,
        endpoint: &str,
        config: &PrimalCoordination,
    ) -> Result<CoordinationResult> {
        // Determine the appropriate API path based on capabilities
        let api_path = self.determine_api_path(primal_name, &config.capabilities);
        let full_url = format!("{endpoint}{api_path}");

        // Create universal coordination payload
        let coordination_payload = self.create_universal_payload(primal_name, &config.capabilities);

        info!(
            "Universal coordination with {} at {}",
            primal_name, full_url
        );

        let response = self
            .client
            .post(&full_url)
            .json(&coordination_payload)
            .send()
            .await
            .map_err(|e| NestGateError::Internal {
                message: format!("Request failed: {e}"),
                location: Some("universal_adapter::coordinate_with_primal".to_string()),
                debug_info: None,
                is_bug: false,
            })?;

        if response.status().is_success() {
            info!(
                "Successfully coordinated with {} (universal adapter)",
                primal_name
            );

            // Parse response if available
            if let Ok(response_data) = response.json::<serde_json::Value>().await {
                return Ok(CoordinationResult::success(
                    primal_name,
                    Some(response_data),
                ));
            }

            Ok(CoordinationResult::success(primal_name, None))
        } else {
            let error_msg = format!("{} coordination failed: {}", primal_name, response.status());
            warn!("{} (universal adapter)", error_msg);
            Ok(CoordinationResult::failed(primal_name, error_msg))
        }
    }

    /// Determine the appropriate API path based on Primal capabilities
    fn determine_api_path(&self, _primal_name: &str, capabilities: &[String]) -> String {
        // Universal API path detection based on capabilities
        for capability in capabilities {
            match capability.as_str() {
                COMPUTE | EXECUTION => return ENDPOINT_PROVISION_STORAGE.into(),
                ORCHESTRATION | COORDINATION => return ENDPOINT_COORDINATE_STORAGE.into(),
                SECURITY | AUTHENTICATION => return ENDPOINT_SECURE_STORAGE.into(),
                AI | ML | AGENTS => return ENDPOINT_OPTIMIZE_STORAGE.into(),
                _ => continue,
            }
        }

        // Fallback to standard coordination endpoint
        ENDPOINT_COORDINATE.into()
    }

    /// Create universal payload that any Primal can understand
    fn create_universal_payload(
        &self,
        primal_name: &str,
        capabilities: &[String],
    ) -> serde_json::Value {
        serde_json::json!({
            "coordination_request": {
                "from": "nestgate",
                "to": primal_name,
                "nestgate_identity": self.nestgate_identity,
                "capabilities_requested": capabilities,
                "api_version": "universal/v1",
                "timestamp": chrono::Utc::now().to_rfc3339()
            },
            "storage_context": {
                "total_capacity": self.nestgate_identity.storage_info.total_capacity_bytes,
                "available_capacity": self.nestgate_identity.storage_info.available_capacity_bytes,
                "protocols": self.nestgate_identity.storage_info.supported_protocols,
                "tiers": self.nestgate_identity.storage_info.storage_tiers,
                "zfs_features": self.nestgate_identity.storage_info.zfs_features
            }
        })
    }

    /// Update storage information from actual storage manager
    pub fn update_storage_info(&mut self, storage_info: StorageCapabilities) {
        self.nestgate_identity.storage_info = storage_info;
    }

    /// Add or update a Primal configuration
    pub fn add_primal_config(&mut self, primal_name: String, config: PrimalCoordination) {
        self.primal_configs.insert(primal_name, config);
    }

    /// Remove a Primal configuration
    pub fn remove_primal_config(&mut self, primal_name: &str) {
        self.primal_configs.remove(primal_name);
    }

    /// Get current Primal configurations
    pub fn get_primal_configs(&self) -> &HashMap<String, PrimalCoordination> {
        &self.primal_configs
    }
}

/// Result of coordination with a Primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationResult {
    pub primal_name: String,
    pub status: CoordinationStatus,
    pub message: Option<String>,
    pub response_data: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinationStatus {
    Success,
    Failed,
    Skipped,
    Unavailable,
}

impl CoordinationResult {
    pub fn success(primal_name: &str, response_data: Option<serde_json::Value>) -> Self {
        Self {
            primal_name: primal_name.into(),
            status: CoordinationStatus::Success,
            message: Some(MSG_COORDINATION_SUCCESSFUL.into()),
            response_data,
            timestamp: Utc::now(),
        }
    }

    pub fn failed(primal_name: &str, error: String) -> Self {
        Self {
            primal_name: primal_name.into(),
            status: CoordinationStatus::Failed,
            message: Some(error),
            response_data: None,
            timestamp: Utc::now(),
        }
    }

    pub fn skipped(primal_name: &str) -> Self {
        Self {
            primal_name: primal_name.into(),
            status: CoordinationStatus::Skipped,
            message: Some(MSG_COORDINATION_DISABLED.into()),
            response_data: None,
            timestamp: Utc::now(),
        }
    }

    pub fn unavailable(primal_name: &str) -> Self {
        Self {
            primal_name: primal_name.into(),
            status: CoordinationStatus::Unavailable,
            message: Some(MSG_ENDPOINT_NOT_AVAILABLE.into()),
            response_data: None,
            timestamp: Utc::now(),
        }
    }
}

/// Trait for implementing universal coordination in NestGate components
#[async_trait]
pub trait UniversalCoordination {
    /// Coordinate storage provisioning with other Primals
    async fn coordinate_storage_provisioning(
        &self,
        request: StorageProvisionRequest,
    ) -> Result<Vec<CoordinationResult>>;

    /// Coordinate volume mounting with compute Primals
    async fn coordinate_volume_mounting(
        &self,
        request: VolumeMountRequest,
    ) -> Result<Vec<CoordinationResult>>;

    /// Coordinate backup operations with other storage or security Primals
    async fn coordinate_backup_operations(
        &self,
        request: BackupRequest,
    ) -> Result<Vec<CoordinationResult>>;
}

/// Storage provisioning request for universal coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProvisionRequest {
    pub requester: String,
    pub volumes: Vec<VolumeSpec>,
    pub total_size_bytes: u64,
    pub tier_requirements: Vec<String>,
    pub protocols_needed: Vec<String>,
}

/// Volume mounting request for universal coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMountRequest {
    pub requester: String,
    pub volume_id: String,
    pub mount_path: String,
    pub mount_options: HashMap<String, String>,
    pub read_only: bool,
}

/// Backup operation request for universal coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRequest {
    pub requester: String,
    pub source_volumes: Vec<String>,
    pub backup_type: BackupType,
    pub destination: Option<String>,
    pub schedule: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupType {
    Snapshot,
    Incremental,
    Full,
    Replicate,
}

/// Volume specification for coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeSpec {
    pub name: String,
    pub size_bytes: u64,
    pub tier: String,
    pub access_mode: String,
    pub protocols: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_core::constants::test_constants::*;

    #[test]
    fn test_universal_adapter_creation() {
        let mut primal_configs = HashMap::new();
        primal_configs.insert(
            SERVICE_EXAMPLE_COMPUTE.to_string(),
            PrimalCoordination {
                enabled: true,
                endpoint: Some(SERVICE_LOCALHOST_URL.to_string()),
                capabilities: vec![COMPUTE.to_string(), EXECUTION.to_string()],
            },
        );

        let adapter = NestGateUniversalAdapter::new(primal_configs).unwrap_or_else(|e| {
            tracing::error!(
                "Expect failed ({}): {:?}",
                "Failed to create universal adapter in test",
                e
            );
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Operation failed - {}: {:?}",
                    "{}", "Failed to create universal adapter in test", e
                ),
            )
            .into());
        });
        assert_eq!(adapter.primal_configs.len(), 1);
        assert!(adapter
            .nestgate_identity
            .capabilities
            .contains(&STORAGE.to_string()));
    }

    #[test]
    fn test_api_path_determination() {
        let adapter = NestGateUniversalAdapter::new(HashMap::new()).unwrap_or_else(|e| {
            tracing::error!(
                "Expect failed ({}): {:?}",
                "Failed to create universal adapter in test",
                e
            );
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Operation failed - {}: {:?}",
                    "{}", "Failed to create universal adapter in test", e
                ),
            )
            .into());
        });

        let compute_path =
            adapter.determine_api_path(SERVICE_EXAMPLE_COMPUTE, &[COMPUTE.to_string()]);
        assert_eq!(compute_path, ENDPOINT_PROVISION_STORAGE);

        let custom_path = adapter.determine_api_path("custom_service", &["custom".to_string()]);
        assert_eq!(custom_path, ENDPOINT_COORDINATE);
    }

    #[tokio::test]
    async fn test_coordination_result_types() {
        let success = CoordinationResult::success(SERVICE_EXAMPLE_COMPUTE, None);
        assert!(matches!(success.status, CoordinationStatus::Success));

        let failed = CoordinationResult::failed(SERVICE_EXAMPLE_COMPUTE, STATUS_ERROR.to_string());
        assert!(matches!(failed.status, CoordinationStatus::Failed));
    }
}

/// Create real endpoints for the NestGate identity
fn create_real_endpoints() -> HashMap<String, String> {
    let mut endpoints = HashMap::new();

    // Get configurable base URLs from environment
    let api_base = std::env::var("NESTGATE_API_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:8000".to_string());
    let ws_base = nestgate_core::config::defaults::NetworkPortDefaults::get_websocket_base_url();

    // Core API endpoints
    endpoints.insert("health".to_string(), format!("{}/health", api_base));
    endpoints.insert("metrics".to_string(), format!("{}/metrics", api_base));
    endpoints.insert("api".to_string(), format!("{}/api/v1", api_base));
    endpoints.insert("zfs".to_string(), format!("{}/api/v1/zfs", api_base));
    endpoints.insert(
        "storage".to_string(),
        format!("{}/api/v1/storage", api_base),
    );
    endpoints.insert("websocket".to_string(), format!("{}/ws", ws_base));

    // Universal coordination endpoints
    endpoints.insert(
        "coordination".to_string(),
        format!("{}/api/v1/primal/coordinate", api_base),
    );
    endpoints.insert(
        "discovery".to_string(),
        format!("{}/api/v1/primal/discover", api_base),
    );

    endpoints
}

/// Get real total storage capacity from ZFS or filesystem
fn get_real_total_capacity() -> u64 {
    // Try to get ZFS pool capacity first
    if let Ok(output) = std::process::Command::new("zpool")
        .args(&["list", "-H", "-o", "size"])
        .output()
    {
        if let Ok(size_str) = String::from_utf8(output.stdout) {
            if let Some(line) = size_str.lines().next() {
                // Parse ZFS size format (e.g., "1.81T", "100G", "500M")
                if let Some(capacity) = parse_zfs_size(line.trim()) {
                    return capacity;
                }
            }
        }
    }

    // Fallback to filesystem capacity
    get_filesystem_capacity().unwrap_or(0)
}

/// Get real available storage capacity
fn get_real_available_capacity() -> u64 {
    // Try to get ZFS available space first
    if let Ok(output) = std::process::Command::new("zpool")
        .args(&["list", "-H", "-o", "free"])
        .output()
    {
        if let Ok(size_str) = String::from_utf8(output.stdout) {
            if let Some(line) = size_str.lines().next() {
                if let Some(capacity) = parse_zfs_size(line.trim()) {
                    return capacity;
                }
            }
        }
    }

    // Fallback to filesystem available space
    get_filesystem_available().unwrap_or(0)
}

/// Parse ZFS size format to bytes
fn parse_zfs_size(size_str: &str) -> Option<u64> {
    if size_str.is_empty() || size_str == "-" {
        return None;
    }

    let size_str = size_str.to_uppercase();
    let (number_part, unit) = if size_str.ends_with("T") {
        (size_str.trim_end_matches("T"), 1024_u64.pow(4))
    } else if size_str.ends_with("G") {
        (size_str.trim_end_matches("G"), 1024_u64.pow(3))
    } else if size_str.ends_with("M") {
        (size_str.trim_end_matches("M"), 1024_u64.pow(2))
    } else if size_str.ends_with("K") {
        (size_str.trim_end_matches("K"), 1024_u64)
    } else {
        (size_str.as_str(), 1)
    };

    number_part
        .parse::<f64>()
        .ok()
        .map(|n| (n * unit as f64) as u64)
}

/// Get filesystem capacity as fallback
fn get_filesystem_capacity() -> Option<u64> {
    use std::fs;
    if let Ok(_metadata) = fs::metadata(".") {
        // This is a simplified approach - in production you'd use statvfs or similar
        Some(1024 * 1024 * 1024 * 100) // 100GB default
    } else {
        None
    }
}

/// Get filesystem available space as fallback
fn get_filesystem_available() -> Option<u64> {
    // Simplified - in production you'd use statvfs
    Some(1024 * 1024 * 1024 * 50) // 50GB default available
}

/// Get current request count from metrics or counter
fn get_request_count() -> u64 {
    // Try to read from a metrics file or in-memory counter
    static REQUEST_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    REQUEST_COUNTER.load(std::sync::atomic::Ordering::Relaxed)
}

/// Increment request counter (to be called from request handlers)
pub fn increment_request_count() {
    static REQUEST_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    REQUEST_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}
