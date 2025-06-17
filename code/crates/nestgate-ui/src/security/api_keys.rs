use crates::ui::nestgate_ui::config::{Config, ApiKey};
use std::sync::Arc;
use std::path::PathBuf;
use std::env;

/// Simplified interface for working with API keys
pub struct ApiKeyManager {
    config: Arc<Config>,
}

impl ApiKeyManager {
    /// Create a new API key manager using environment variables
    pub fn from_env() -> Result<Self, String> {
        let mut config = Config::from_env();
        config.init_key_store()?;
        
        Ok(Self {
            config: Arc::new(config),
        })
    }
    
    /// Create a new API key manager with custom configuration
    pub fn with_config(master_key: &str, keys_path: PathBuf, expiration_days: u32) -> Result<Self, String> {
        // Set environment variables
        env::set_var("NESTGATE_MASTER_KEY", master_key);
        env::set_var("NESTGATE_KEYS_PATH", keys_path.to_str().unwrap_or("./keys"));
        env::set_var("NESTGATE_API_KEY_EXPIRATION_DAYS", expiration_days.to_string());
        
        Self::from_env()
    }
    
    /// Generate a new API key
    pub fn generate_key(&self, name: &str, services: Vec<String>) -> Result<ApiKey, String> {
        self.config.generate_api_key(name, services)
    }
    
    /// List all API keys
    pub fn list_keys(&self) -> Result<Vec<ApiKey>, String> {
        self.config.list_api_keys()
    }
    
    /// Revoke an API key
    pub fn revoke_key(&self, key_id: &str) -> Result<bool, String> {
        self.config.revoke_api_key(key_id)
    }
    
    /// Validate an API key for a specific service
    pub fn validate_key(&self, key_id: &str, key_value: &str, service: &str) -> Result<bool, String> {
        self.config.validate_api_key(key_id, key_value, service)
    }
    
    /// Get the underlying config object
    pub fn config(&self) -> Arc<Config> {
        self.config.clone()
    }
}

/// Helper function to validate API keys from HTTP headers
pub fn validate_api_key_from_headers(
    config: &Config,
    key_id: Option<&str>,
    key_value: Option<&str>,
    service: &str,
) -> bool {
    match (key_id, key_value) {
        (Some(id), Some(value)) => {
            match config.validate_api_key(id, value, service) {
                Ok(valid) => valid,
                Err(_) => false,
            }
        },
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use base64::{Engine as _, engine::general_purpose};
    use rand::Rng;
    
    fn generate_test_key() -> String {
        let mut key = [0u8; 32];
        rand::thread_rng().fill(&mut key);
        general_purpose::STANDARD.encode(key)
    }
    
    #[tokio::test]
    async fn test_api_key_manager() {
        // Setup test environment
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let keys_path = temp_dir.path().join("api_keys");
        let master_key = generate_test_key();
        
        // Create API key manager
        let key_manager = ApiKeyManager::with_config(
            &master_key,
            keys_path.clone(),
            30
        ).expect("Failed to create API key manager");
        
        // Generate a key
        let services = vec!["test-service".to_string()];
        let key = key_manager.generate_key("test-key", services.clone())
            .expect("Failed to generate key");
        
        assert_eq!(key.name, "test-key");
        assert_eq!(key.allowed_services, services);
        assert!(key.value.is_some());
        
        // List keys
        let keys = key_manager.list_keys().expect("Failed to list keys");
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].id, key.id);
        
        // Validate key
        let key_value = key.value.as_ref().unwrap();
        let valid = key_manager.validate_key(&key.id, key_value, "test-service")
            .expect("Failed to validate key");
        assert!(valid);
        
        // Validate with helper function
        let valid = validate_api_key_from_headers(
            &key_manager.config,
            Some(&key.id),
            key.value.as_deref(),
            "test-service"
        );
        assert!(valid);
        
        // Revoke key
        let revoked = key_manager.revoke_key(&key.id).expect("Failed to revoke key");
        assert!(revoked);
        
        // Check key is revoked
        let keys = key_manager.list_keys().expect("Failed to list keys");
        assert_eq!(keys.len(), 0);
    }
} 