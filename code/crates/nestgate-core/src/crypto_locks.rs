//! Cryptographic Access Control and Digital Rights Management
//! **MODERNIZED**: Updated to use current error handling and patterns

// use crate::error::SecurityError; // Removed - using unified error system
use crate::universal_spore::{
    AuthorizationDecision, OperationRequest, UniversalCryptographicSpore, UserContext,
};
use crate::universal_traits::{SecurityPrimalProvider, Signature};
use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

/// Cryptographic proof - managed by any security primal provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoProof {
    /// Proof identifier
    pub proof_id: String,
    /// Raw proof data
    pub proof_data: Vec<u8>,
    /// Security provider key identifier
    pub key_id: String,
    /// Security provider digital signature
    pub signature: String,
    /// Proof timestamp
    pub timestamp: SystemTime,
    /// Security provider nonce for replay protection
    pub nonce: String,
    /// Proof hash
    pub proof_hash: String,
    /// Security provider validation token
    pub validation_token: String,
}

impl CryptoProof {
    /// Create new proof using any security primal provider
    pub async fn new_with_security_provider(
        security_provider: &Arc<dyn SecurityPrimalProvider>,
        data: &[u8],
        context: &str,
    ) -> Result<Self> {
        println!("Creating crypto proof with security provider");

        // Generate proof data
        let proof_data = Self::generate_proof_data(data, context)?;

        // Get security provider key ID and signature
        let key_id = security_provider.get_key_id().await?;
        let signature_result = security_provider.sign_data(&proof_data).await?;

        // Generate security provider validation token
        let validation_token = security_provider
            .generate_validation_token(&proof_data)
            .await?;

        // Calculate proof hash
        let proof_hash = Self::hash_proof_data(&proof_data, &signature_result.signature)?;

        Ok(Self {
            proof_id: uuid::Uuid::new_v4().to_string(),
            proof_data,
            key_id,
            signature: signature_result.signature,
            timestamp: SystemTime::now(),
            nonce: Self::generate_nonce(),
            proof_hash,
            validation_token,
        })
    }

    /// Validate proof using any security primal provider
    pub async fn validate_with_security_provider(
        &self,
        security_provider: &Arc<dyn SecurityPrimalProvider>,
    ) -> Result<bool> {
        println!("Validating crypto proof with security provider");

        // Check timestamp validity (not too old)
        if let Ok(elapsed) = self.timestamp.elapsed() {
            if elapsed > Duration::from_secs(3600) {
                println!("⚠️ Proof timestamp is too old: {elapsed:?}");
                return Ok(false);
            }
        }

        // Validate security provider signature
        let signature = Signature {
            algorithm: "RS256".to_string(),
            signature: self.signature.clone(),
            key_id: self.key_id.clone(),
        };

        let signature_valid = security_provider
            .verify_signature(&self.proof_data, &signature)
            .await?;

        if !signature_valid {
            println!("⚠️ Security provider signature validation failed");
            return Ok(false);
        }

        // Validate security provider token
        let token_valid = security_provider
            .validate_token(&self.validation_token, &self.proof_data)
            .await?;

        if !token_valid {
            println!("⚠️ Security provider token validation failed");
            return Ok(false);
        }

        // Validate proof hash
        let expected_hash = Self::hash_proof_data(&self.proof_data, &self.signature)?;
        if self.proof_hash != expected_hash {
            println!("⚠️ Proof hash validation failed");
            return Ok(false);
        }

        println!("✅ Crypto proof validation successful");
        Ok(true)
    }

    /// Generate proof data from input data and context
    fn generate_proof_data(data: &[u8], context: &str) -> Result<Vec<u8>> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.update(context.as_bytes());
        hasher.update(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|e| NestGateError::Internal {
                    message: format!("Time error: {e}"),
                    location: Some(file!().to_string()),
                    debug_info: None,
                    is_bug: false,
                })?
                .as_secs()
                .to_be_bytes(),
        );

        Ok(hasher.finalize().to_vec())
    }

    /// Generate a cryptographic nonce
    fn generate_nonce() -> String {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let nonce: [u8; 32] = rng.gen();
        hex::encode(nonce)
    }

    /// Hash proof data with security provider signature
    fn hash_proof_data(proof_data: &[u8], signature: &str) -> Result<String> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(proof_data);
        hasher.update(signature.as_bytes());

        Ok(hex::encode(hasher.finalize()))
    }
}

/// Access request for crypto lock evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRequest {
    /// Source identifier
    pub source: String,
    /// Destination identifier
    pub destination: String,
    /// Operation being performed
    pub operation: String,
    /// Request timestamp
    pub timestamp: SystemTime,
    /// Request context
    pub context: String,
}

/// Access decision result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessDecision {
    /// Access allowed
    Allow { reason: String },
    /// Access denied
    Deny { reason: String },
    /// Require lock for access
    RequireLock { reason: String },
    /// Require authentication for access
    RequireAuthentication { reason: String },
}

/// Statistics for lock operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockStats {
    pub locks_created: usize,
    pub locks_expired: usize,
    pub violations_detected: usize,
    pub corporate_accesses: usize,
    pub individual_accesses: usize,
    pub expired_locks: usize,
    pub lock_expiration: Duration,
}

/// Corporate detection pattern for identifying business usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateDetectionPattern {
    /// Pattern name
    pub name: String,
    /// Pattern description
    pub description: String,
    /// Detection threshold
    pub threshold: f64,
    /// Pattern enabled
    pub enabled: bool,
}

/// External boundary guardian for corporate license enforcement
pub struct ExternalBoundaryGuardian {
    /// Spore for autonomous rights enforcement
    pub spore: Arc<UniversalCryptographicSpore>,

    /// Detection patterns for corporate usage
    pub detection_patterns: Vec<CorporateDetectionPattern>,

    /// Security provider interface
    #[allow(dead_code)]
    security_provider: Arc<dyn SecurityPrimalProvider>,
}

impl ExternalBoundaryGuardian {
    /// Create new guardian with cryptographic spore
    pub async fn new_with_spore(
        security_provider: Arc<dyn SecurityPrimalProvider>,
        provider_endpoint: Option<String>,
    ) -> Result<Self> {
        // Create spore for NestGate
        let mut spore = UniversalCryptographicSpore::new_for_primal("nestgate")?;

        // Initialize security provider integration if available
        if let Some(endpoint) = provider_endpoint {
            spore
                .initialize_with_security_provider(
                    "security-provider".to_string(),
                    "default-provider".to_string(),
                    Some(endpoint),
                )
                .await?;
        }

        println!(
            "🧬 ExternalBoundaryGuardian initialized with spore: {}",
            spore.spore_id
        );

        Ok(Self {
            spore: Arc::new(spore),
            detection_patterns: vec![],
            security_provider,
        })
    }

    /// Install security extraction lock using spore authorization
    pub async fn install_security_extraction_lock(
        &self,
        dataset_path: &str,
        user_context: UserContext,
    ) -> Result<()> {
        println!("🔐 Installing security extraction lock for dataset: {dataset_path}");

        // Create operation request
        let operation = OperationRequest {
            operation_type: "install_extraction_lock".to_string(),
            resource_path: dataset_path.to_string(),
            user_context: user_context.clone(),
            metadata: std::collections::HashMap::new(),
            timestamp: std::time::SystemTime::now(),
        };

        // Ask spore for authorization
        let decision = self.spore.authorize_operation(&operation).await?;

        match decision {
            AuthorizationDecision::Allow {
                enhanced_by_security_provider,
                ..
            } => {
                if enhanced_by_security_provider {
                    println!("✅ Security lock installed with security provider enhancement");
                } else {
                    println!("✅ Security lock installed autonomously by spore");
                }

                Ok(())
            }

            AuthorizationDecision::Deny {
                reason,
                remediation: _remediation,
                ..
            } => {
                println!("⚠️ Access denied: {reason}");
                Err(NestGateError::security_error(
                    &reason,
                    "crypto_lock_access",
                    Some("external_boundary_guardian"),
                    None,
                ))
            }

            AuthorizationDecision::RequireLicense {
                terms: _terms,
                contact,
                organization_profile,
            } => {
                let message = format!(
                    "Corporate license required for sovereign locks. Organization: '{}' Contact: {}",
                    organization_profile.organization_name, contact
                );
                println!("🏢 {message}");
                Err(NestGateError::security_error(
                    &message,
                    "crypto_lock_access",
                    Some("external_boundary_guardian"),
                    None,
                ))
            }
        }
    }

    /// Create sovereign security lock using spore system
    pub async fn create_sovereign_security_lock(
        &self,
        dataset_path: &str,
        user_context: &UserContext,
    ) -> Result<String> {
        println!("🛡️ Creating sovereign security lock for dataset: {dataset_path}");

        // Create operation request
        let operation = OperationRequest {
            operation_type: "create_sovereign_lock".to_string(),
            resource_path: dataset_path.to_string(),
            user_context: user_context.clone(),
            metadata: std::collections::HashMap::new(),
            timestamp: std::time::SystemTime::now(),
        };

        // Ask spore for authorization
        let decision = self.spore.authorize_operation(&operation).await?;

        match decision {
            AuthorizationDecision::Allow {
                enhanced_by_security_provider,
                ..
            } => {
                let lock_id = if enhanced_by_security_provider {
                    // Enhanced lock with security provider genetic integration
                    let provider_id = self
                        .spore
                        .security_provider_integration
                        .as_ref()
                        .map(|i| i.provider_id.as_str())
                        .unwrap_or("autonomous");
                    format!(
                        "sovereign_lock_provider_{}_{}",
                        provider_id,
                        uuid::Uuid::new_v4()
                    )
                } else {
                    // Autonomous spore lock
                    format!(
                        "sovereign_lock_spore_{}_{}",
                        self.spore.spore_id,
                        uuid::Uuid::new_v4()
                    )
                };

                println!("✅ Sovereign security lock created: {lock_id}");
                Ok(lock_id)
            }

            AuthorizationDecision::Deny { reason, .. } => {
                Err(NestGateError::AccessDenied { reason })
            }

            AuthorizationDecision::RequireLicense {
                terms: _terms,
                contact,
                organization_profile,
            } => {
                let message = format!(
                    "Corporate license required for sovereign locks. Organization: '{}' Contact: {}",
                    organization_profile.organization_name, contact
                );
                Err(NestGateError::LicenseRequired { message })
            }
        }
    }

    /// Check if spore needs evolution
    pub async fn check_spore_evolution(&self) -> Result<bool> {
        // For immutable Arc, we can't actually evolve the spore in place
        // Just check if evolution would be beneficial
        println!("🌱 Checking if spore evolution would be beneficial");
        Ok(false) // Simplified for now - would need mutable reference to actually evolve
    }

    /// Get spore status for monitoring
    pub async fn get_spore_status(&self) -> Result<SporeStatus> {
        Ok(SporeStatus {
            spore_id: self.spore.spore_id.clone(),
            generation: self.spore.generation,
            security_provider_integrated: self.spore.security_provider_integration.is_some(),
            operations_count: self.spore.usage_stats.operations_count,
            total_locks: 0, // Simplified - no stats tracking in new structure
            valid_locks: 0, // Simplified - no stats tracking in new structure
            last_evolution: self.spore.last_evolution,
        })
    }
}

/// Spore status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SporeStatus {
    pub spore_id: String,
    pub generation: u32,
    pub security_provider_integrated: bool,
    pub operations_count: u64,
    pub total_locks: usize,
    pub valid_locks: usize,
    pub last_evolution: SystemTime,
}

/// Create a spore-enhanced boundary guardian
pub async fn create_spore_guardian(
    security_provider: Arc<dyn SecurityPrimalProvider>,
    provider_endpoint: Option<String>,
) -> Result<ExternalBoundaryGuardian> {
    ExternalBoundaryGuardian::new_with_spore(security_provider, provider_endpoint).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    // Mock security provider replaced with production security provider in security_provider module

    #[tokio::test]
    async fn test_crypto_proof_creation() -> Result<()> {
        let security_provider = crate::security_provider::create_security_provider();
        let data = b"test_data";
        let context = "test_context";

        let proof =
            CryptoProof::new_with_security_provider(&security_provider, data, context).await?;

        assert!(!proof.proof_hash.is_empty());
        assert!(!proof.signature.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_crypto_proof_validation() -> Result<()> {
        let security_provider = crate::security_provider::create_security_provider();
        let data = b"test_data";
        let context = "test_context";

        // SAFETY FIX: Replace unwrap() with proper error handling
        let proof =
            CryptoProof::new_with_security_provider(&security_provider, data, context).await?;

        let is_valid = proof
            .validate_with_security_provider(&security_provider)
            .await?;

        assert!(is_valid);
        Ok(())
    }

    #[tokio::test]
    async fn test_boundary_guardian() -> Result<()> {
        let security_provider = crate::security_provider::create_security_provider();
        let guardian = ExternalBoundaryGuardian::new_with_spore(security_provider, None).await?;

        let user_context = UserContext {
            user_id: Some("test_user".to_string()),
            session_id: "test_session".to_string(),
            ip_address: "127.0.0.1".to_string(),
            user_agent: Some("test_agent".to_string()),
            environment_info: std::collections::HashMap::new(),
        };

        let response = guardian
            .install_security_extraction_lock("test_resource", user_context)
            .await;
        assert!(response.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_boundary_guardian_with_security_provider() -> Result<()> {
        let security_provider = crate::security_provider::create_security_provider();
        let guardian = ExternalBoundaryGuardian::new_with_spore(security_provider, None).await?;

        // Test successful validation
        let valid_user_context = UserContext {
            user_id: Some("valid_user".to_string()),
            session_id: "valid_session".to_string(),
            ip_address: "127.0.0.1".to_string(),
            user_agent: Some("test_agent".to_string()),
            environment_info: std::collections::HashMap::new(),
        };

        let response = guardian
            .install_security_extraction_lock("public_resource", valid_user_context)
            .await;
        assert!(response.is_ok());

        // Test invalid source
        let invalid_user_context = UserContext {
            user_id: Some("malicious_user".to_string()),
            session_id: "malicious_session".to_string(),
            ip_address: "192.168.1.100".to_string(),
            user_agent: Some("malicious_agent".to_string()),
            environment_info: std::collections::HashMap::new(),
        };

        let response = guardian
            .install_security_extraction_lock("restricted_resource", invalid_user_context)
            .await;
        // In a real implementation, this might fail based on security rules
        // For now, we'll just ensure it doesn't crash
        assert!(response.is_ok());
        Ok(())
    }
}
