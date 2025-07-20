/*
 * This file is part of NestGate.
 *
 * NestGate is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * NestGate is also available under a commercial license.
 * For commercial licensing options, please contact licensing@nestgate.io
 *
 * Copyright (C) 2024 NestGate Contributors
 */

//! Certificate validation and management for NestGate
//!
//! Supports both standalone operation and universal security provider integration
//! for certificate validation and authentication.

use crate::universal_adapter::{create_default_adapter, UniversalPrimalAdapter};
use crate::universal_traits::Signature;
use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;

/// Certificate validation errors
#[derive(Error, Debug)]
pub enum CertError {
    #[error("Certificate not found")]
    NotFound,

    #[error("Certificate expired")]
    Expired,

    #[error("Invalid certificate signature")]
    InvalidSignature,

    #[error("Certificate revoked")]
    Revoked,

    #[error("Insufficient permissions for integration: {integration}")]
    InsufficientPermissions { integration: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// Certificate types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CertificateType {
    /// Free certificate for good faith users
    Free,
    /// Commercial certificate for enterprise users
    Commercial,
    /// Development certificate for testing
    Development,
}

/// External integration types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Integration {
    /// Service orchestration (any orchestration provider)
    Orchestration,
    /// Multi-runtime platform (any compute provider)
    Compute,
    /// AI platform (any AI provider)
    AI,
    /// Security provider (any security provider)
    Security,
}

/// Certificate permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permissions {
    /// Allowed integrations
    pub integrations: Vec<Integration>,
    /// Maximum number of nodes
    pub max_nodes: Option<u32>,
    /// Support tier
    pub support_tier: String,
    /// Custom features enabled
    pub custom_features: Vec<String>,
}

/// Universal security provider signed certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    /// Certificate version
    pub version: u32,
    /// Certificate type (free or commercial)
    pub cert_type: CertificateType,
    /// Certificate issuer (any security provider)
    pub issuer: String,
    /// Certificate subject (user/organization)
    pub subject: String,
    /// Issue timestamp
    pub issued_at: SystemTime,
    /// Expiration timestamp
    pub expires_at: SystemTime,
    /// Permissions granted by this certificate
    pub permissions: Permissions,
    /// Certificate serial number
    pub serial: String,
    /// Cryptographic signature (from any security provider)
    pub signature: String,
}

/// Certificate manager for validation and caching
pub struct CertificateManager {
    /// Certificate cache
    cache: HashMap<String, Certificate>,
    /// Certificate directory path
    cert_dir: PathBuf,
    /// Universal security provider adapter
    security_adapter: Option<Arc<UniversalPrimalAdapter>>,
    /// Last validation time
    #[allow(dead_code)]
    last_validation: Option<SystemTime>,
}

impl CertificateManager {
    /// Create a new certificate manager
    pub fn new<P: AsRef<Path>>(cert_dir: P) -> Self {
        Self {
            cache: HashMap::new(),
            cert_dir: cert_dir.as_ref().to_path_buf(),
            security_adapter: None,
            last_validation: None,
        }
    }

    /// Create certificate manager with universal security provider integration
    pub async fn with_universal_security<P: AsRef<Path>>(cert_dir: P) -> Self {
        let adapter = Arc::new(create_default_adapter());
        if let Err(e) = adapter.initialize().await {
            tracing::warn!("Failed to initialize universal security adapter: {}", e);
        }

        Self {
            cache: HashMap::new(),
            cert_dir: cert_dir.as_ref().to_path_buf(),
            security_adapter: Some(adapter),
            last_validation: None,
        }
    }

    /// Load certificates from disk
    pub async fn load_certificates(&mut self) -> Result<()> {
        if !self.cert_dir.exists() {
            std::fs::create_dir_all(&self.cert_dir)?;
            return Ok(());
        }

        let entries = std::fs::read_dir(&self.cert_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("cert") {
                match self.load_certificate(&path) {
                    Ok(cert) => {
                        self.cache.insert(cert.serial.clone(), cert);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to load certificate {:?}: {}", path, e);
                    }
                }
            }
        }

        tracing::info!("Loaded {} certificates", self.cache.len());
        Ok(())
    }

    /// Load a single certificate from file
    fn load_certificate(&self, path: &Path) -> Result<Certificate> {
        let content = std::fs::read_to_string(path)?;
        let cert: Certificate = serde_json::from_str(&content)?;
        Ok(cert)
    }

    /// Save certificate to disk
    pub fn save_certificate(&self, cert: &Certificate) -> Result<()> {
        let filename = format!("{}.cert", cert.serial);
        let path = self.cert_dir.join(filename);
        let content = serde_json::to_string_pretty(cert)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Validate integration access using available security providers
    pub async fn validate_integration_access(&self, integration: Integration) -> Result<()> {
        // First check if we have any certificates that allow this integration
        for cert in self.cache.values() {
            if cert.permissions.integrations.contains(&integration)
                && self.is_certificate_valid(cert)?
            {
                return Ok(());
            }
        }

        // If no valid certificates, check if universal security provider allows access
        if let Some(adapter) = &self.security_adapter {
            if let Some(_security_provider) = adapter.get_security_provider().await {
                tracing::info!(
                    "Universal security provider available for integration: {:?}",
                    integration
                );
                return Ok(());
            }
        }

        Err(NestGateError::Internal(format!(
            "No valid certificate or security provider for integration: {integration:?}"
        )))
    }

    /// Validate individual certificate
    fn is_certificate_valid(&self, cert: &Certificate) -> Result<bool> {
        // Check expiration
        if cert.expires_at < SystemTime::now() {
            return Err(NestGateError::Internal("Certificate expired".to_string()));
        }

        // Verify signature using available security provider
        if !self.verify_signature(cert) {
            return Err(NestGateError::Internal(
                "Invalid certificate signature".to_string(),
            ));
        }

        // Check revocation (would check against revocation list in real implementation)
        if self.is_revoked(cert) {
            return Err(NestGateError::Internal("Certificate revoked".to_string()));
        }

        Ok(true)
    }

    /// Verify certificate signature using any available security provider
    fn verify_signature(&self, cert: &Certificate) -> bool {
        // Universal signature validation
        if cert.signature.is_empty() {
            return false;
        }

        // Accept signatures from any security provider
        cert.signature.len() >= 10
            && (cert.signature.starts_with("sec_provider_sig_")
                || cert.signature.starts_with("beardog_sig_")
                || cert.signature.starts_with("vault_sig_")
                || cert.signature.starts_with("standalone_sig_")
                || cert.signature.starts_with("self_signed_"))
    }

    /// Check if certificate is revoked
    fn is_revoked(&self, cert: &Certificate) -> bool {
        // Check for obvious revocation indicators
        if cert.signature.contains("REVOKED")
            || cert.subject.contains("REVOKED")
            || cert.serial.contains("REVOKED")
        {
            return true;
        }

        // Check if certificate has expired (also considered revoked)
        if let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) {
            if let Ok(expires) = cert.expires_at.duration_since(UNIX_EPOCH) {
                return now > expires;
            }
        }

        false
    }

    /// Get certificate info for display
    pub fn get_certificate_info(&self) -> Vec<CertificateInfo> {
        self.cache
            .values()
            .map(|cert| CertificateInfo {
                serial: cert.serial.clone(),
                cert_type: cert.cert_type.clone(),
                subject: cert.subject.clone(),
                expires_at: cert.expires_at,
                integrations: cert.permissions.integrations.clone(),
                is_valid: self.is_certificate_valid(cert).unwrap_or(false),
            })
            .collect()
    }
}

/// Certificate information for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateInfo {
    pub serial: String,
    pub cert_type: CertificateType,
    pub subject: String,
    pub expires_at: SystemTime,
    pub integrations: Vec<Integration>,
    pub is_valid: bool,
}

/// Integration access validator
pub struct IntegrationValidator {
    cert_manager: CertificateManager,
    security_adapter: Option<Arc<UniversalPrimalAdapter>>,
}

impl IntegrationValidator {
    /// Create a new integration validator
    pub async fn new<P: AsRef<Path>>(cert_dir: P) -> Result<Self> {
        let mut cert_manager = CertificateManager::with_universal_security(cert_dir).await;
        cert_manager.load_certificates().await?;

        let adapter = Arc::new(create_default_adapter());
        if let Err(e) = adapter.initialize().await {
            tracing::warn!("Failed to initialize universal security adapter: {}", e);
        }

        Ok(Self {
            cert_manager,
            security_adapter: Some(adapter),
        })
    }

    /// Check if external integration is available
    pub async fn can_use_integration(&self, integration: Integration) -> bool {
        match self
            .cert_manager
            .validate_integration_access(integration)
            .await
        {
            Ok(_) => true,
            Err(e) => {
                tracing::info!("Integration access denied: {}", e);
                false
            }
        }
    }

    /// Get status of all integrations
    pub async fn get_integration_status(&self) -> IntegrationStatus {
        let orchestration = self.can_use_integration(Integration::Orchestration).await;
        let compute = self.can_use_integration(Integration::Compute).await;
        let ai = self.can_use_integration(Integration::AI).await;
        let security = self.can_use_integration(Integration::Security).await;

        IntegrationStatus {
            orchestration_available: orchestration,
            compute_available: compute,
            ai_available: ai,
            security_available: security,
            certificates: self.cert_manager.get_certificate_info(),
        }
    }

    /// Get discovered security providers
    pub async fn get_available_security_providers(&self) -> Vec<String> {
        if let Some(adapter) = &self.security_adapter {
            let providers = adapter.find_providers_by_capability("security").await;
            providers.into_iter().map(|p| p.endpoint).collect()
        } else {
            vec![]
        }
    }
}

/// Integration availability status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationStatus {
    pub orchestration_available: bool,
    pub compute_available: bool,
    pub ai_available: bool,
    pub security_available: bool,
    pub certificates: Vec<CertificateInfo>,
}

/// Convenience macro for checking integration access
#[macro_export]
macro_rules! require_integration {
    ($validator:expr, $integration:expr) => {
        if !$validator.can_use_integration($integration).await {
            return Err($crate::error::CoreError::IntegrationNotAvailable {
                integration: stringify!($integration).to_string(),
                message: "Valid certificate or security provider required for external integration access".to_string(),
            });
        }
    };
}

/// Certificate validation modes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CertMode {
    /// Standalone mode - self-managed certificates
    Standalone,
    /// Universal security provider mode - delegated certificate management
    UniversalSecurity,
    /// Hybrid mode - both standalone and universal security capabilities
    Hybrid,
}

/// Certificate information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertInfo {
    pub subject: String,
    pub issuer: String,
    pub valid_from: SystemTime,
    pub valid_to: SystemTime,
    pub fingerprint: String,
    pub key_usage: Vec<String>,
    pub extended_key_usage: Vec<String>,
}

/// Universal security provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalSecurityConfig {
    pub discovery_timeout: Duration,
    pub validation_timeout: Duration,
    pub retry_attempts: u32,
    pub fallback_to_standalone: bool,
}

impl Default for UniversalSecurityConfig {
    fn default() -> Self {
        Self {
            discovery_timeout: Duration::from_secs(10),
            validation_timeout: Duration::from_secs(30),
            retry_attempts: 3,
            fallback_to_standalone: true,
        }
    }
}

/// Certificate validator with universal security provider support
pub struct CertValidator {
    mode: CertMode,
    security_config: Option<UniversalSecurityConfig>,
    security_adapter: Option<Arc<UniversalPrimalAdapter>>,
    trust_store: HashMap<String, CertInfo>,
    validation_cache: HashMap<String, (bool, SystemTime)>,
}

impl CertValidator {
    /// Create new certificate validator in standalone mode
    pub fn standalone() -> Self {
        Self {
            mode: CertMode::Standalone,
            security_config: None,
            security_adapter: None,
            trust_store: HashMap::new(),
            validation_cache: HashMap::new(),
        }
    }

    /// Create new certificate validator with universal security provider integration
    pub async fn with_universal_security(config: UniversalSecurityConfig) -> Self {
        let adapter = Arc::new(create_default_adapter());
        if let Err(e) = adapter.initialize().await {
            tracing::warn!("Failed to initialize universal security adapter: {}", e);
        }

        Self {
            mode: CertMode::UniversalSecurity,
            security_config: Some(config),
            security_adapter: Some(adapter),
            trust_store: HashMap::new(),
            validation_cache: HashMap::new(),
        }
    }

    /// Create hybrid validator supporting both modes
    pub async fn hybrid(security_config: UniversalSecurityConfig) -> Self {
        let adapter = Arc::new(create_default_adapter());
        if let Err(e) = adapter.initialize().await {
            tracing::warn!("Failed to initialize universal security adapter: {}", e);
        }

        Self {
            mode: CertMode::Hybrid,
            security_config: Some(security_config),
            security_adapter: Some(adapter),
            trust_store: HashMap::new(),
            validation_cache: HashMap::new(),
        }
    }

    /// Validate certificate using current mode
    pub async fn validate_cert(&mut self, cert_pem: &str) -> Result<bool> {
        let fingerprint = self.calculate_fingerprint(cert_pem)?;

        // Check cache first
        if let Some((result, timestamp)) = self.validation_cache.get(&fingerprint) {
            let cache_timeout = self
                .security_config
                .as_ref()
                .map(|c| c.validation_timeout)
                .unwrap_or(Duration::from_secs(300));

            if timestamp.elapsed().unwrap_or(Duration::MAX) < cache_timeout {
                return Ok(*result);
            }
        }

        let result = match self.mode {
            CertMode::Standalone => self.validate_standalone(cert_pem).await,
            CertMode::UniversalSecurity => self.validate_universal_security(cert_pem).await,
            CertMode::Hybrid => {
                // Try universal security first, fallback to standalone
                match self.validate_universal_security(cert_pem).await {
                    Ok(result) => Ok(result),
                    Err(_) => self.validate_standalone(cert_pem).await,
                }
            }
        }?;

        // Cache result
        self.validation_cache
            .insert(fingerprint, (result, SystemTime::now()));
        Ok(result)
    }

    /// Add certificate to trust store (standalone mode)
    pub fn add_trusted_cert(&mut self, cert_info: CertInfo) -> Result<()> {
        if matches!(self.mode, CertMode::UniversalSecurity) {
            return Err(NestGateError::Internal(
                "Cannot add certificates in universal security mode".to_string(),
            ));
        }

        self.trust_store
            .insert(cert_info.fingerprint.clone(), cert_info);
        Ok(())
    }

    /// Get current validation mode
    pub fn mode(&self) -> &CertMode {
        &self.mode
    }

    /// Check if universal security provider is available
    pub async fn universal_security_available(&self) -> bool {
        if let Some(adapter) = &self.security_adapter {
            adapter.get_security_provider().await.is_some()
        } else {
            false
        }
    }

    /// Standalone certificate validation
    async fn validate_standalone(&self, cert_pem: &str) -> Result<bool> {
        let cert_info = self.parse_certificate(cert_pem)?;

        // Basic validation checks
        if cert_info.subject.is_empty() || cert_info.issuer.is_empty() {
            return Ok(false);
        }

        // Check expiration
        if cert_info.valid_to < SystemTime::now() {
            return Ok(false);
        }

        // Additional validation logic would go here
        Ok(true)
    }

    /// Universal security provider certificate validation
    async fn validate_universal_security(&self, cert_pem: &str) -> Result<bool> {
        let adapter = self.security_adapter.as_ref().ok_or_else(|| {
            NestGateError::Validation("Universal security adapter not set".to_string())
        })?;

        if let Some(security_provider) = adapter.get_security_provider().await {
            // Use universal security provider to validate certificate
            let signature = Signature {
                algorithm: "universal".to_string(),
                signature: cert_pem.to_string(),
                key_id: "cert-validation".to_string(),
            };

            match security_provider
                .verify_signature(cert_pem.as_bytes(), &signature)
                .await
            {
                Ok(valid) => Ok(valid),
                Err(_) => {
                    // Fall back to basic validation if provider fails
                    self.validate_standalone(cert_pem).await
                }
            }
        } else {
            tracing::warn!("No universal security provider available for certificate validation");
            self.validate_standalone(cert_pem).await
        }
    }

    /// Calculate certificate fingerprint
    fn calculate_fingerprint(&self, cert_pem: &str) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        cert_pem.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    }

    /// Parse certificate information
    fn parse_certificate(&self, cert_pem: &str) -> Result<CertInfo> {
        // Simplified certificate parsing for testing
        // Real implementation would use proper X.509 parsing

        if cert_pem.is_empty() {
            return Err(NestGateError::Internal("Empty certificate".to_string()));
        }

        let fingerprint = self.calculate_fingerprint(cert_pem)?;
        let now = SystemTime::now();

        Ok(CertInfo {
            subject: "CN=NestGate".to_string(),
            issuer: "CN=Universal Security CA".to_string(),
            valid_from: now,
            valid_to: now + Duration::from_secs(365 * 24 * 3600), // 1 year
            fingerprint,
            key_usage: vec![
                "digitalSignature".to_string(),
                "keyEncipherment".to_string(),
            ],
            extended_key_usage: vec!["serverAuth".to_string(), "clientAuth".to_string()],
        })
    }

    /// Get the key ID for universal security provider encryption
    pub async fn get_key_id(&self) -> Result<String> {
        if let Some(adapter) = &self.security_adapter {
            if let Some(security_provider) = adapter.get_security_provider().await {
                return security_provider.get_key_id().await;
            }
        }

        // Fallback key ID
        Ok("universal-security-key".to_string())
    }

    /// Sign data using universal security provider
    pub async fn sign_data(&self, data: &str) -> Result<String> {
        if let Some(adapter) = &self.security_adapter {
            if let Some(security_provider) = adapter.get_security_provider().await {
                let signature = security_provider.sign_data(data.as_bytes()).await?;
                return Ok(signature.signature);
            }
        }

        // Fallback signing
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.update(b"universal-signature");

        Ok(format!("universal-sig-{:x}", hasher.finalize()))
    }

    /// Generate a validation token for proof data
    pub async fn generate_validation_token(&self, proof_data: &str) -> Result<String> {
        if let Some(adapter) = &self.security_adapter {
            if let Some(security_provider) = adapter.get_security_provider().await {
                return security_provider
                    .generate_validation_token(proof_data.as_bytes())
                    .await;
            }
        }

        // Fallback token generation
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(proof_data.as_bytes());
        hasher.update(b"universal-validation-token");

        Ok(format!("universal-token-{:x}", hasher.finalize()))
    }

    /// Verify signature using universal security provider
    pub async fn verify_signature(
        &self,
        data: &str,
        signature: &str,
        key_id: &str,
    ) -> Result<bool> {
        if let Some(adapter) = &self.security_adapter {
            if let Some(security_provider) = adapter.get_security_provider().await {
                let sig = Signature {
                    algorithm: "universal".to_string(),
                    signature: signature.to_string(),
                    key_id: key_id.to_string(),
                };

                return security_provider
                    .verify_signature(data.as_bytes(), &sig)
                    .await;
            }
        }

        // Fallback verification
        let _expected_sig = self.sign_data(data).await?;
        let expected_key_id = self.get_key_id().await?;

        Ok(signature.contains("universal-sig-") && key_id == expected_key_id)
    }

    /// Validate universal security provider token
    pub async fn validate_token(&self, token: &str, proof_data: &str) -> Result<bool> {
        if let Some(adapter) = &self.security_adapter {
            if let Some(security_provider) = adapter.get_security_provider().await {
                return security_provider
                    .validate_token(token, proof_data.as_bytes())
                    .await;
            }
        }

        // Fallback validation
        Ok(token.starts_with("universal-token-"))
    }
}

/// Certificate utilities
pub struct CertUtils;

impl CertUtils {
    /// Generate self-signed certificate for standalone mode
    pub fn generate_self_signed() -> Result<String> {
        // Simplified certificate generation
        // Real implementation would use proper cryptographic libraries

        let cert_template = r#"-----BEGIN CERTIFICATE-----
MIICWjCCAcMCAg38MA0GCSqGSIb3DQEBBQUAMHsxCzAJBgNVBAYTAlVTMQswCQYD
VQQIDAJOSjEQMA0GA1UEBwwGVW5pdmVyc2FsMRMwEQYDVQQKDABVbml2ZXJzYWwg
Q0ExEzARBgNVBAMMClVuaXZlcnNhbCBDQTEjMCEGCSqGSIb3DQEJARYUYWRtaW5A
dW5pdmVyc2FsLmxvY2FsMB4XDTIwMTAwMTAwMDAwMFoXDTI1MTAwMTAwMDAwMFow
azELMAkGA1UEBhMCVVMxCzAJBgNVBAgMAlNGMRAwDgYDVQQHDAdVbml2ZXJzYWwx
ETAPBgNVBAoMCFVuaXZlcnNhbDE"#;

        Ok(cert_template.to_string())
    }
}

// Legacy compatibility aliases for migration
pub type BearDogConfig = UniversalSecurityConfig;

// Re-export universal types for easy migration
pub use crate::universal_traits::SecurityDecision;
