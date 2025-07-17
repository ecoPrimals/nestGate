//! Crypto Locks System
//!
//! Universal crypto locks that work with any security primal provider,
//! eliminating hardcoded dependencies on specific security implementations.

use std::sync::Arc;
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::universal_traits::{SecurityPrimalProvider, Signature};
use crate::{NestGateError, Result};

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
        debug!("Creating crypto proof with security provider");
        
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
        debug!("Validating crypto proof with security provider");
        
        // Check timestamp validity (not too old)
        if let Ok(elapsed) = self.timestamp.elapsed() {
            if elapsed > Duration::from_secs(3600) {
                warn!("Proof timestamp is too old: {:?}", elapsed);
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
            warn!("Security provider signature validation failed");
            return Ok(false);
        }
        
        // Validate security provider token
        let token_valid = security_provider
            .validate_token(&self.validation_token, &self.proof_data)
            .await?;
        
        if !token_valid {
            warn!("Security provider token validation failed");
            return Ok(false);
        }
        
        // Validate proof hash
        let expected_hash = Self::hash_proof_data(&self.proof_data, &self.signature)?;
        if self.proof_hash != expected_hash {
            warn!("Proof hash validation failed");
            return Ok(false);
        }
        
        info!("Crypto proof validation successful");
        Ok(true)
    }

    /// Generate proof data from input data and context
    fn generate_proof_data(data: &[u8], context: &str) -> Result<Vec<u8>> {
        use sha2::{Digest, Sha256};
        
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.update(context.as_bytes());
        hasher.update(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| NestGateError::Internal(format!("Time error: {e}")))?
            .as_secs()
            .to_be_bytes());
        
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

/// External boundary guardian - uses any security primal provider
pub struct ExternalBoundaryGuardian {
    /// Active extraction locks (all locked by security provider)
    active_locks: tokio::sync::RwLock<std::collections::HashMap<String, CryptoProof>>,
    /// Lock expiration time
    lock_expiration: Duration,
    /// Security provider (any security primal)
    security_provider: Arc<dyn SecurityPrimalProvider>,
}

impl ExternalBoundaryGuardian {
    /// Create new guardian with any security primal provider
    pub fn new(security_provider: Arc<dyn SecurityPrimalProvider>) -> Self {
        info!("Creating external boundary guardian with security provider");
        
        Self {
            active_locks: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            lock_expiration: Duration::from_secs(3600),
            security_provider,
        }
    }

    /// Check if access crosses external boundary (security provider validated)
    pub async fn check_boundary_access(&self, request: &AccessRequest) -> Result<AccessDecision> {
        debug!("Checking boundary access: {} -> {}", request.source, request.destination);
        
        // Internal communication is always allowed (no security provider needed)
        if self.is_internal_communication(&request.source, &request.destination) {
            return Ok(AccessDecision::Allow {
                reason: "Internal primal communication - no security provider key required".to_string(),
            });
        }
        
        // External access requires security provider validation
        let decision = self
            .security_provider
            .evaluate_boundary_access(&request.source, &request.destination, &request.operation)
            .await?;
        
        match decision {
            crate::universal_traits::SecurityDecision::Allow => {
                Ok(AccessDecision::Allow { 
                    reason: "Access granted by security provider".to_string(),
                })
            }
            crate::universal_traits::SecurityDecision::Deny => {
                Ok(AccessDecision::Deny { 
                    reason: "Access denied by security provider".to_string(),
                })
            }
            crate::universal_traits::SecurityDecision::RequireAuth => {
                Ok(AccessDecision::RequireAuthentication { 
                    reason: "Authentication required for boundary access".to_string(),
                })
            }
        }
    }

    /// Check if communication is internal (primal to primal)
    fn is_internal_communication(&self, source: &str, destination: &str) -> bool {
        let internal_prefixes = ["nestgate", "primal", "internal"];
        
        internal_prefixes.iter().any(|prefix| {
            source.starts_with(prefix) && destination.starts_with(prefix)
        })
    }



    /// Create an extraction lock for external access
    pub async fn create_extraction_lock(
        &self,
        destination: &str,
        context: &str,
    ) -> Result<String> {
        info!("Creating extraction lock for: {}", destination);
        
        // Create crypto proof with security provider
        let proof = CryptoProof::new_with_security_provider(
            &self.security_provider,
            destination.as_bytes(),
            context,
        ).await?;
        
        let lock_id = proof.proof_id.clone();
        
        // Store the lock
        let mut locks = self.active_locks.write().await;
        locks.insert(destination.to_string(), proof);
        
        info!("Extraction lock created: {}", lock_id);
        Ok(lock_id)
    }

    /// Remove an extraction lock
    pub async fn remove_extraction_lock(&self, destination: &str) -> Result<bool> {
        let mut locks = self.active_locks.write().await;
        let removed = locks.remove(destination).is_some();
        
        if removed {
            info!("Extraction lock removed for: {}", destination);
        }
        
        Ok(removed)
    }

    /// Get statistics about active locks
    pub async fn get_lock_stats(&self) -> LockStats {
        let locks = self.active_locks.read().await;
        let mut expired_count = 0;
        let mut valid_count = 0;
        
        for lock in locks.values() {
            if let Ok(elapsed) = lock.timestamp.elapsed() {
                if elapsed >= self.lock_expiration {
                    expired_count += 1;
                } else {
                    valid_count += 1;
                }
            }
        }
        
        LockStats {
            total_locks: locks.len(),
            valid_locks: valid_count,
            expired_locks: expired_count,
            lock_expiration: self.lock_expiration,
        }
    }

    /// Cleanup expired locks
    pub async fn cleanup_expired_locks(&self) -> Result<usize> {
        let mut locks = self.active_locks.write().await;
        let initial_count = locks.len();
        
        locks.retain(|_, lock| {
            if let Ok(elapsed) = lock.timestamp.elapsed() {
                elapsed < self.lock_expiration
            } else {
                false
            }
        });
        
        let removed_count = initial_count - locks.len();
        if removed_count > 0 {
            info!("Cleaned up {} expired locks", removed_count);
        }
        
        Ok(removed_count)
    }

    /// Check external boundary access
    pub async fn check_external_boundary(
        &self,
        source: &str,
        destination: &str,
        operation: &str,
    ) -> Result<AccessDecision> {
        debug!("Checking external boundary access: {} -> {} ({})", source, destination, operation);
        
        // Use security provider to evaluate boundary access
        let decision = self.security_provider
            .evaluate_boundary_access(source, destination, operation)
            .await?;
        
        match decision {
            crate::universal_traits::SecurityDecision::Allow => Ok(AccessDecision::Allow {
                reason: "Access granted by security provider".to_string(),
            }),
            crate::universal_traits::SecurityDecision::Deny => Ok(AccessDecision::Deny {
                reason: "Access denied by security provider".to_string(),
            }),
            crate::universal_traits::SecurityDecision::RequireAuth => Ok(AccessDecision::RequireAuthentication {
                reason: "Authentication required for boundary access".to_string(),
            }),
        }
    }

    /// Install BearDog extraction lock (universal adapter pattern)
    pub async fn install_beardog_extraction_lock(
        &self,
        _lock_type: crate::hardware_tuning::ExternalLockType,
        _source: &str,
        destination: &str,
        operation: &str,
    ) -> Result<String> {
        debug!("Installing BearDog extraction lock: {} -> {} ({})", _source, destination, operation);
        
        // Use the create_extraction_lock method which works with any security provider
        let lock_id = self.create_extraction_lock(destination, operation).await?;
        
        info!("BearDog extraction lock installed: {}", lock_id);
        Ok(lock_id)
    }

    /// Create sovereign BearDog lock (universal adapter pattern)
    pub async fn create_sovereign_beardog_lock(
        &self,
        destination: &str,
        operation: &str,
        _lock_type: crate::hardware_tuning::ExternalLockType,
    ) -> Result<String> {
        debug!("Creating sovereign BearDog lock for: {} ({})", destination, operation);
        
        // Use the create_extraction_lock method which works with any security provider
        let lock_id = self.create_extraction_lock(destination, operation).await?;
        
        info!("Sovereign BearDog lock created: {}", lock_id);
        Ok(lock_id)
    }
}

/// Statistics about active locks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockStats {
    pub total_locks: usize,
    pub valid_locks: usize,
    pub expired_locks: usize,
    pub lock_expiration: Duration,
}

/// Create a default external boundary guardian
pub fn create_default_guardian(
    security_provider: Arc<dyn SecurityPrimalProvider>,
) -> ExternalBoundaryGuardian {
    ExternalBoundaryGuardian::new(security_provider)
}

#[cfg(test)]
mod tests {
    use super::*;
    // Mock security provider replaced with production security provider in security_provider module

    #[tokio::test]
    async fn test_crypto_proof_creation() {
        let security_provider = crate::security_provider::create_security_provider().await.unwrap();
        let data = b"test_data";
        let context = "test_context";
        
        let proof = CryptoProof::new_with_security_provider(
            &security_provider,
            data,
            context,
        ).await;
        
        assert!(proof.is_ok());
        let proof = proof.unwrap();
        assert!(!proof.proof_id.is_empty());
        assert!(!proof.key_id.is_empty());
        assert!(!proof.signature.is_empty());
    }

    #[tokio::test]
    async fn test_crypto_proof_validation() {
        let security_provider = crate::security_provider::create_security_provider().await.unwrap();
        let data = b"test_data";
        let context = "test_context";
        
        let proof = CryptoProof::new_with_security_provider(
            &security_provider,
            data,
            context,
        ).await.unwrap();
        
        let validation_result = proof.validate_with_security_provider(&security_provider).await;
        assert!(validation_result.is_ok());
        assert!(validation_result.unwrap());
    }

    #[tokio::test]
    async fn test_boundary_guardian() {
        let security_provider = crate::security_provider::create_security_provider().await.unwrap();
        let guardian = ExternalBoundaryGuardian::new(security_provider);
        
        let request = AccessRequest {
            source: "127.0.0.1".to_string(),
            destination: "internal_destination".to_string(),
            operation: "read".to_string(),
            timestamp: SystemTime::now(),
            context: "test_context".to_string(),
        };
        
        let decision = guardian.check_boundary_access(&request).await;
        assert!(decision.is_ok());
        
        match decision.unwrap() {
            AccessDecision::Allow { reason: _ } => {
                // Test passed
            }
            AccessDecision::Deny { reason } => {
                panic!("Expected allow, got deny: {}", reason);
            }
            AccessDecision::RequireLock { reason } => {
                panic!("Expected allow, got require lock: {}", reason);
            }
            AccessDecision::RequireAuthentication { reason } => {
                panic!("Expected allow, got require authentication: {}", reason);
            }
        }
    }

    #[tokio::test]
    async fn test_internal_communication() {
        let security_provider = crate::security_provider::create_security_provider().await.unwrap();
        let guardian = ExternalBoundaryGuardian::new(security_provider);
        
        let request = AccessRequest {
            source: "nestgate_source".to_string(),
            destination: "nestgate_destination".to_string(),
            operation: "read".to_string(),
            timestamp: SystemTime::now(),
            context: "test_context".to_string(),
        };
        
        let decision = guardian.check_boundary_access(&request).await;
        assert!(decision.is_ok());
        
        match decision.unwrap() {
            AccessDecision::Allow { reason: _ } => {
                // Test passed - access was granted
            }
            AccessDecision::Deny { reason } => {
                panic!("Expected allow for internal communication, got deny: {}", reason);
            }
            AccessDecision::RequireLock { reason } => {
                panic!("Expected allow for internal communication, got require lock: {}", reason);
            }
            AccessDecision::RequireAuthentication { reason } => {
                panic!("Expected allow for internal communication, got require authentication: {}", reason);
            }
        }
    }
}
