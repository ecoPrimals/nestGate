//! Universal Security Client for Capability-Based Decentralized Authentication
//!
//! This client works with any service that provides the required security capabilities,
//! not hardcoded to any specific service implementation (like BearDog).

use crate::config::security::DecentralizedSecurityConfig;
use crate::types::{AccessGrant, CryptographicProof, SecurityServiceNode, ServiceNodeStatus};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tokio::time::timeout;

/// Universal security client for capability-based authentication
pub struct UniversalSecurityClient {
    /// Configuration for decentralized security
    config: DecentralizedSecurityConfig,
    /// Discovered security service nodes
    available_nodes: Vec<SecurityServiceNode>,
    /// Service discovery client
    service_discovery: Box<dyn ServiceDiscovery>,
    /// HTTP client for API calls
    http_client: reqwest::Client,
}

impl std::fmt::Debug for UniversalSecurityClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UniversalSecurityClient")
            .field("config", &self.config)
            .field("available_nodes", &self.available_nodes)
            .field("service_discovery", &"<service_discovery>")
            .field("http_client", &"<reqwest::Client>")
            .finish()
    }
}

/// Service discovery trait for finding security services
#[async_trait]
pub trait ServiceDiscovery: Send + Sync {
    /// Discover services that provide specific capabilities
    async fn discover_by_capabilities(
        &self,
        capabilities: &[String],
    ) -> Result<Vec<ServiceDiscoveryResult>, ServiceDiscoveryError>;

    /// Refresh service registry
    async fn refresh(&self) -> Result<(), ServiceDiscoveryError>;
}

/// Service discovery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryResult {
    /// Service identifier
    pub service_id: String,
    /// Service name
    pub service_name: String,
    /// Service endpoints
    pub endpoints: Vec<String>,
    /// Provided capabilities
    pub capabilities: Vec<String>,
    /// Service metadata
    pub metadata: HashMap<String, String>,
    /// Service priority (higher = preferred)
    pub priority: u8,
}

/// Service discovery errors
#[derive(Debug, thiserror::Error)]
pub enum ServiceDiscoveryError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Timeout: {0}")]
    Timeout(String),
    #[error("No services found with required capabilities")]
    NoServicesFound,
    #[error("Service registry error: {0}")]
    RegistryError(String),
}

/// Universal security client errors
#[derive(Debug, thiserror::Error)]
pub enum UniversalSecurityError {
    #[error("Service discovery error: {0}")]
    ServiceDiscovery(#[from] ServiceDiscoveryError),
    #[error("Consensus not achieved: {0}% (required: {1}%)")]
    ConsensusNotAchieved(f64, f64),
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Timeout error: {0}")]
    Timeout(String),
    #[error("Configuration error: {0}")]
    Configuration(String),
}

impl UniversalSecurityClient {
    /// Create a new universal security client
    pub async fn new(
        config: DecentralizedSecurityConfig,
        service_discovery: Box<dyn ServiceDiscovery>,
    ) -> Result<Self, UniversalSecurityError> {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.operation_timeout))
            .build()
            .map_err(|e| UniversalSecurityError::Network(e.to_string()))?;

        let mut client = Self {
            config,
            available_nodes: Vec::new(),
            service_discovery,
            http_client,
        };

        // Initial service discovery
        client.refresh_services().await?;

        Ok(client)
    }

    /// Refresh available security services
    pub async fn refresh_services(&mut self) -> Result<(), UniversalSecurityError> {
        // Discover services with required capabilities
        let discovery_results = self
            .service_discovery
            .discover_by_capabilities(&self.config.required_capabilities)
            .await?;

        // Convert to security service nodes
        self.available_nodes = discovery_results
            .into_iter()
            .map(|result| SecurityServiceNode {
                service_id: result.service_id,
                endpoint: result.endpoints.first().unwrap_or(&"".to_string()).clone(),
                capabilities: result.capabilities,
                public_key: result
                    .metadata
                    .get("public_key")
                    .unwrap_or(&"".to_string())
                    .clone(),
                status: ServiceNodeStatus::Active,
                last_seen: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
                priority: result.priority,
            })
            .collect();

        Ok(())
    }

    /// Authenticate with decentralized consensus
    pub async fn authenticate_with_consensus(
        &self,
        proof: &CryptographicProof,
    ) -> Result<AccessGrant, UniversalSecurityError> {
        // Verify we have sufficient nodes for consensus
        let active_nodes: Vec<_> = self
            .available_nodes
            .iter()
            .filter(|node| matches!(node.status, ServiceNodeStatus::Active))
            .collect();

        if active_nodes.is_empty() {
            return Err(UniversalSecurityError::ServiceDiscovery(
                ServiceDiscoveryError::NoServicesFound,
            ));
        }

        let required_consensus_count =
            ((active_nodes.len() as f64) * self.config.min_consensus).ceil() as usize;

        if required_consensus_count == 0 {
            return Err(UniversalSecurityError::Configuration(
                "Cannot achieve consensus with available nodes".to_string(),
            ));
        }

        // Send verification requests to all active nodes
        let verification_futures: Vec<_> = active_nodes
            .iter()
            .map(|node| self.verify_proof_with_node(node, proof))
            .collect();

        // Wait for responses with timeout
        let timeout_duration = Duration::from_secs(self.config.operation_timeout);
        let verification_results = timeout(
            timeout_duration,
            futures::future::join_all(verification_futures),
        )
        .await
        .map_err(|_| {
            UniversalSecurityError::Timeout("Consensus verification timeout".to_string())
        })?;

        // Process results and calculate consensus
        let mut successful_verifications = Vec::new();
        let mut participating_nodes = Vec::new();

        for (node, result) in active_nodes.iter().zip(verification_results.into_iter()) {
            if let Ok(verification) = result {
                successful_verifications.push(verification);
                participating_nodes.push(node.service_id.clone());
            }
        }

        let consensus_percentage =
            (successful_verifications.len() as f64) / (active_nodes.len() as f64);

        // Check if consensus achieved
        if consensus_percentage < self.config.min_consensus {
            return Err(UniversalSecurityError::ConsensusNotAchieved(
                consensus_percentage * 100.0,
                self.config.min_consensus * 100.0,
            ));
        }

        // Aggregate permissions from successful verifications
        let mut all_permissions = Vec::new();
        for verification in &successful_verifications {
            all_permissions.extend(verification.permissions.clone());
        }
        all_permissions.sort();
        all_permissions.dedup();

        // Calculate consensus expiry (use minimum from all responses)
        let consensus_expiry = successful_verifications
            .iter()
            .map(|v| v.valid_until)
            .min()
            .unwrap_or(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64
                    + 3600,
            );

        Ok(AccessGrant {
            permissions: all_permissions,
            valid_until: consensus_expiry,
            proof_hash: format!("{:x}", md5::compute(serde_json::to_string(proof).unwrap())),
            consensus_nodes: participating_nodes,
            consensus_percentage,
        })
    }

    /// Verify proof with a single security service node
    async fn verify_proof_with_node(
        &self,
        node: &SecurityServiceNode,
        proof: &CryptographicProof,
    ) -> Result<ProofVerificationResult, UniversalSecurityError> {
        let endpoint = format!("{}/api/v1/security/verify", node.endpoint);

        let verification_request = ProofVerificationRequest {
            proof: proof.clone(),
            required_capabilities: self.config.required_capabilities.clone(),
        };

        let response = self
            .http_client
            .post(&endpoint)
            .json(&verification_request)
            .send()
            .await
            .map_err(|e| UniversalSecurityError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(UniversalSecurityError::AuthenticationFailed(format!(
                "Node {} returned status: {}",
                node.service_id,
                response.status()
            )));
        }

        let verification_result: ProofVerificationResult = response
            .json()
            .await
            .map_err(|e| UniversalSecurityError::Network(e.to_string()))?;

        Ok(verification_result)
    }

    /// Check if sufficient services are available for consensus
    pub fn is_consensus_possible(&self) -> bool {
        let active_nodes = self
            .available_nodes
            .iter()
            .filter(|node| matches!(node.status, ServiceNodeStatus::Active))
            .count();

        let required_nodes = ((active_nodes as f64) * self.config.min_consensus).ceil() as usize;
        required_nodes > 0 && required_nodes <= active_nodes
    }

    /// Get information about available security services
    pub fn get_available_services(&self) -> &[SecurityServiceNode] {
        &self.available_nodes
    }
}

/// Proof verification request
#[derive(Debug, Serialize, Deserialize)]
struct ProofVerificationRequest {
    proof: CryptographicProof,
    required_capabilities: Vec<String>,
}

/// Proof verification result from a single node
#[derive(Debug, Serialize, Deserialize)]
struct ProofVerificationResult {
    /// Whether the proof is valid
    valid: bool,
    /// Granted permissions
    permissions: Vec<String>,
    /// Valid until timestamp
    valid_until: i64,
    /// Node-specific metadata
    metadata: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockServiceDiscovery {
        results: Vec<ServiceDiscoveryResult>,
    }

    #[async_trait]
    impl ServiceDiscovery for MockServiceDiscovery {
        async fn discover_by_capabilities(
            &self,
            _capabilities: &[String],
        ) -> Result<Vec<ServiceDiscoveryResult>, ServiceDiscoveryError> {
            Ok(self.results.clone())
        }

        async fn refresh(&self) -> Result<(), ServiceDiscoveryError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_universal_security_client_creation() {
        let config = DecentralizedSecurityConfig::default();
        let discovery = Box::new(MockServiceDiscovery { results: vec![] });

        let client = UniversalSecurityClient::new(config, discovery).await;
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_consensus_possible() {
        let config = DecentralizedSecurityConfig {
            required_capabilities: vec!["security.auth".to_string()],
            min_consensus: 0.66,
            operation_timeout: 30,
            max_retries: 3,
            service_discovery: crate::config::security::ServiceDiscoveryConfig::default(),
        };

        let discovery = Box::new(MockServiceDiscovery {
            results: vec![
                ServiceDiscoveryResult {
                    service_id: "service1".to_string(),
                    service_name: "Security Service 1".to_string(),
                    endpoints: vec!["http://localhost:8001".to_string()],
                    capabilities: vec!["security.auth".to_string()],
                    metadata: HashMap::new(),
                    priority: 100,
                },
                ServiceDiscoveryResult {
                    service_id: "service2".to_string(),
                    service_name: "Security Service 2".to_string(),
                    endpoints: vec!["http://localhost:8002".to_string()],
                    capabilities: vec!["security.auth".to_string()],
                    metadata: HashMap::new(),
                    priority: 100,
                },
            ],
        });

        let client = UniversalSecurityClient::new(config, discovery)
            .await
            .unwrap();
        assert!(client.is_consensus_possible());
    }
}
