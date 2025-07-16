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
use tracing::{info, warn};
use uuid::Uuid;

use nestgate_core::{NestGateError, Result};

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
            .map_err(|e| NestGateError::Network(format!("Failed to create HTTP client: {e}")))?;

        let nestgate_identity = NestGateIdentity {
            instance_id: format!("nestgate-{}", Uuid::new_v4().simple()),
            capabilities: vec![
                "storage".to_string(),
                "data".to_string(),
                "orchestration".to_string(),
                "ai".to_string(),
                "security".to_string(),
                "tiered_storage".to_string(),
                "replication".to_string(),
                "snapshots".to_string(),
                "encryption".to_string(),
                "monitoring".to_string(),
                "automation".to_string(),
                "performance".to_string(),
                "federation".to_string(),
                "ecosystem".to_string(),
                "universal_coordination".to_string(),
                "storage_provider".to_string(),
                "data_processor".to_string(),
                "orchestration_participant".to_string(),
                "ai_compute_provider".to_string(),
                "security_coordinator".to_string(),
                "ecosystem_member".to_string(),
            ],
            endpoints: HashMap::new(),
            storage_info: StorageCapabilities {
                total_capacity_bytes: 0,
                available_capacity_bytes: 0,
                supported_protocols: vec![
                    "NFS".to_string(),
                    "SMB".to_string(),
                    "HTTP".to_string(),
                    "ZFS".to_string(),
                ],
                storage_tiers: vec!["Hot".to_string(), "Warm".to_string(), "Cold".to_string()],
                zfs_features: vec![
                    "pooled_storage".to_string(),
                    "tiered_storage".to_string(),
                    "snapshots".to_string(),
                    "replication".to_string(),
                    "compression".to_string(),
                    "deduplication".to_string(),
                    "encryption".to_string(),
                    "performance_monitoring".to_string(),
                    "automated_management".to_string(),
                    "federation".to_string(),
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
    pub async fn coordinate_with_primal(&self, primal_name: &str) -> Result<CoordinationResult> {
        let primal_config = self.primal_configs.get(primal_name).ok_or_else(|| {
            NestGateError::Configuration(format!("Primal {primal_name} not configured"))
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
            .map_err(|e| NestGateError::Network(format!("Request failed: {e}")))?;

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
                "compute" | "execution" => return "/api/v1/provision-storage".to_string(),
                "orchestration" | "coordination" => {
                    return "/api/v1/coordinate-storage".to_string()
                }
                "security" | "authentication" => return "/api/v1/secure-storage".to_string(),
                "ai" | "ml" | "agents" => return "/api/v1/optimize-storage".to_string(),
                "custom" => return "/api/v1/coordinate".to_string(),
                _ => continue,
            }
        }

        // Fallback to standard coordination endpoint
        "/api/v1/coordinate".to_string()
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
            primal_name: primal_name.to_string(),
            status: CoordinationStatus::Success,
            message: Some("Coordination successful".to_string()),
            response_data,
            timestamp: Utc::now(),
        }
    }

    pub fn failed(primal_name: &str, error: String) -> Self {
        Self {
            primal_name: primal_name.to_string(),
            status: CoordinationStatus::Failed,
            message: Some(error),
            response_data: None,
            timestamp: Utc::now(),
        }
    }

    pub fn skipped(primal_name: &str) -> Self {
        Self {
            primal_name: primal_name.to_string(),
            status: CoordinationStatus::Skipped,
            message: Some("Coordination disabled".to_string()),
            response_data: None,
            timestamp: Utc::now(),
        }
    }

    pub fn unavailable(primal_name: &str) -> Self {
        Self {
            primal_name: primal_name.to_string(),
            status: CoordinationStatus::Unavailable,
            message: Some("Endpoint not available".to_string()),
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

    #[test]
    fn test_universal_adapter_creation() {
        let mut primal_configs = HashMap::new();
        primal_configs.insert(
            "example-compute".to_string(),
            PrimalCoordination {
                enabled: true,
                endpoint: Some("http://localhost:8082".to_string()),
                capabilities: vec!["compute".to_string(), "execution".to_string()],
            },
        );

        let adapter = NestGateUniversalAdapter::new(primal_configs).unwrap();
        assert_eq!(adapter.primal_configs.len(), 1);
        assert!(adapter
            .nestgate_identity
            .capabilities
            .contains(&"storage".to_string()));
    }

    #[test]
    fn test_api_path_determination() {
        let adapter = NestGateUniversalAdapter::new(HashMap::new()).unwrap();

        let compute_path = adapter.determine_api_path("example-compute", &["compute".to_string()]);
        assert_eq!(compute_path, "/api/v1/provision-storage");

        let custom_path = adapter.determine_api_path("custom-primal", &["custom".to_string()]);
        assert_eq!(custom_path, "/api/v1/coordinate");
    }

    #[tokio::test]
    async fn test_coordination_result_types() {
        let success = CoordinationResult::success("test-primal", None);
        assert!(matches!(success.status, CoordinationStatus::Success));

        let failed = CoordinationResult::failed("test-primal", "error".to_string());
        assert!(matches!(failed.status, CoordinationStatus::Failed));
    }
}
