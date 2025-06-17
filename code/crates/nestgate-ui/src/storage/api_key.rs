use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tracing::{debug, info};

/// API key information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyInfo {
    /// The key ID
    pub id: String,
    /// The user ID associated with this key
    pub user_id: String,
    /// The key description
    pub description: String,
    /// Creation date
    pub created_at: DateTime<Utc>,
    /// Expiration date (if any)
    pub expires_at: Option<DateTime<Utc>>,
    /// Last used date
    pub last_used: Option<DateTime<Utc>>,
    /// Is the key active
    pub active: bool,
}

/// Store for API keys
pub struct ApiKeyStore {
    /// In-memory storage of API keys
    keys: Arc<RwLock<HashMap<String, ApiKeyInfo>>>,
}

impl ApiKeyStore {
    /// Create a new in-memory API key store
    pub fn new_in_memory() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Add a new API key
    pub async fn add_key(&self, key: &str, user_id: &str, description: &str) -> ApiKeyInfo {
        let key_info = ApiKeyInfo {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            description: description.to_string(),
            created_at: Utc::now(),
            expires_at: None,
            last_used: None,
            active: true,
        };
        
        let mut keys = self.keys.write().await;
        keys.insert(key.to_string(), key_info.clone());
        info!("Added new API key for user {}", user_id);
        
        key_info
    }
    
    /// Get API key info
    pub async fn get_key(&self, key: &str) -> Option<ApiKeyInfo> {
        let mut keys = self.keys.write().await;
        
        if let Some(key_info) = keys.get_mut(key) {
            // Update last used time
            key_info.last_used = Some(Utc::now());
            return Some(key_info.clone());
        }
        
        None
    }
    
    /// Revoke an API key
    pub async fn revoke_key(&self, key: &str) -> bool {
        let mut keys = self.keys.write().await;
        
        if let Some(key_info) = keys.get_mut(key) {
            key_info.active = false;
            info!("Revoked API key {}", key_info.id);
            return true;
        }
        
        false
    }
    
    /// List all API keys for a user
    pub async fn list_keys_for_user(&self, user_id: &str) -> Vec<ApiKeyInfo> {
        let keys = self.keys.read().await;
        
        keys.values()
            .filter(|k| k.user_id == user_id)
            .cloned()
            .collect()
    }
} 