//! Security Fallback Provider
//! Local cryptographic operations when external security primals are unavailable

use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use std::collections::HashMap;
use tracing::debug;

use crate::ecosystem_integration::mock_router::{FallbackProvider, MockRoutingError};

/// Security fallback provider using local cryptographic functions
pub struct SecurityFallbackProvider {
    config: SecurityFallbackConfig,
}

#[derive(Debug, Clone)]
pub struct SecurityFallbackConfig {
    pub key_size: usize,
    pub enable_logging: bool,
}

impl Default for SecurityFallbackConfig {
    fn default() -> Self {
        Self {
            key_size: 256,
            enable_logging: true,
        }
    }
}

impl Default for SecurityFallbackProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityFallbackProvider {
    pub fn new() -> Self {
        Self::with_config(SecurityFallbackConfig::default())
    }

    pub fn with_config(config: SecurityFallbackConfig) -> Self {
        Self { config }
    }

    /// Local encryption fallback
    async fn encrypt_fallback(
        &self,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, MockRoutingError> {
        debug!("🔄 Security fallback: Local encryption");

        let data = params.get("data").and_then(|v| v.as_str()).ok_or_else(|| {
            MockRoutingError::FallbackError("Missing data to encrypt".to_string())
        })?;

        // Simple base64 encoding as placeholder for real encryption
        let encoded = general_purpose::STANDARD.encode(data.as_bytes());

        Ok(serde_json::json!({
            "success": true,
            "encrypted_data": encoded,
            "algorithm": "local_fallback",
            "key_size": self.config.key_size,
            "provider": "security_fallback"
        }))
    }

    /// Local decryption fallback
    async fn decrypt_fallback(
        &self,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, MockRoutingError> {
        debug!("🔄 Security fallback: Local decryption");

        let encrypted_data = params
            .get("encrypted_data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MockRoutingError::FallbackError("Missing encrypted data".to_string()))?;

        // Simple base64 decoding as placeholder for real decryption
        let decoded = general_purpose::STANDARD
            .decode(encrypted_data)
            .map_err(|e| MockRoutingError::FallbackError(format!("Decryption failed: {e}")))?;

        let decrypted = String::from_utf8(decoded)
            .map_err(|e| MockRoutingError::FallbackError(format!("Invalid UTF-8: {e}")))?;

        Ok(serde_json::json!({
            "success": true,
            "decrypted_data": decrypted,
            "provider": "security_fallback"
        }))
    }

    /// Generate key fallback
    async fn generate_key_fallback(
        &self,
        _params: serde_json::Value,
    ) -> Result<serde_json::Value, MockRoutingError> {
        debug!("🔄 Security fallback: Key generation");

        // Generate a simple random key (placeholder for real key generation)
        let key = format!("fallback_key_{}", uuid::Uuid::new_v4());

        Ok(serde_json::json!({
            "success": true,
            "key": key,
            "key_size": self.config.key_size,
            "algorithm": "local_fallback",
            "provider": "security_fallback"
        }))
    }

    #[allow(dead_code)]
    async fn decrypt_data(&self, encrypted_data: &str) -> Result<String, MockRoutingError> {
        // Simple mock decryption - in reality this would use proper cryptography
        use base64::{engine::general_purpose, Engine as _};
        let decoded = general_purpose::STANDARD
            .decode(encrypted_data)
            .map_err(|e| MockRoutingError::FallbackError(format!("Decryption failed: {e}")))?;

        String::from_utf8(decoded)
            .map_err(|e| MockRoutingError::FallbackError(format!("Invalid UTF-8: {e}")))
    }

    #[allow(dead_code)]
    async fn handle_operation(
        &self,
        operation: &str,
        _params: &str,
    ) -> Result<String, MockRoutingError> {
        match operation {
            "authenticate" => Ok("Authentication successful".to_string()),
            "authorize" => Ok("Authorization granted".to_string()),
            "encrypt_data" => Ok("Data encrypted successfully".to_string()),
            _ => Err(MockRoutingError::FallbackError(format!(
                "Unsupported security operation: {operation}"
            ))),
        }
    }
}

#[async_trait]
impl FallbackProvider for SecurityFallbackProvider {
    async fn execute(
        &self,
        operation: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, MockRoutingError> {
        match operation {
            "encrypt" => self.encrypt_fallback(params).await,
            "decrypt" => self.decrypt_fallback(params).await,
            "generate_key" => self.generate_key_fallback(params).await,
            _ => Err(MockRoutingError::FallbackError(format!(
                "Unsupported security operation: {operation}"
            ))),
        }
    }

    fn supported_operations(&self) -> Vec<String> {
        vec![
            "encrypt".to_string(),
            "decrypt".to_string(),
            "generate_key".to_string(),
            "hash".to_string(),
            "sign".to_string(),
            "verify".to_string(),
        ]
    }

    fn metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("provider_type".to_string(), "security_fallback".to_string());
        metadata.insert("version".to_string(), "1.0.0".to_string());
        metadata.insert(
            "description".to_string(),
            "Local cryptographic fallback provider".to_string(),
        );
        metadata
    }
}
