//! Encryption and Key Management
//!
//! Data encryption utilities with key rotation support

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;
use serde::{Serialize, Deserialize};

/// **ENCRYPTION MANAGER**
///
/// Encryption and decryption utilities for sensitive data
pub struct EncryptionManager {
    keys: HashMap<String, Vec<u8>>,
    default_key_id: Option<String>,
    stats: EncryptionStats,
}

#[derive(Debug, Default)]
struct EncryptionStats {
    encryptions_performed: AtomicU64,
    decryptions_performed: AtomicU64,
    key_rotations: AtomicU64,
}

impl EncryptionManager {
    /// Create a new encryption manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            default_key_id: None,
            stats: EncryptionStats::default(),
        }
    }
    
    /// Add an encryption key
    ///
    /// # Errors
    ///
    /// Returns an error if key is not exactly 32 bytes (AES-256 requirement)
    pub fn add_key(&mut self, key_id: String, key: Vec<u8>) -> Result<(), String> {
        if key.len() != 32 {
            return Err("Key must be 32 bytes for AES-256".to_string());
        }
        
        self.keys.insert(key_id.clone(), key);
        
        if self.default_key_id.is_none() {
            self.default_key_id = Some(key_id);
        }
        Ok(())
    }
    
    /// Encrypt data
    ///
    /// # Errors
    ///
    /// Returns error if no encryption key is available or key is not found
    pub fn encrypt(&self, data: &[u8], key_id: Option<&str>) -> Result<EncryptedData, String> {
        let key_id = key_id.or(self.default_key_id.as_deref())
            .ok_or("No encryption key available")?;
        
        let key = self.keys.get(key_id)
            .ok_or("Encryption key not found")?;
        
        // Simplified encryption (in production, use proper AES-GCM)
        let mut encrypted = Vec::new();
        for (i, &byte) in data.iter().enumerate() {
            encrypted.push(byte ^ key[i % key.len()]);
        }
        
        self.stats.encryptions_performed.fetch_add(1, Ordering::Relaxed);
        
        Ok(EncryptedData {
            key_id: key_id.to_string(),
            data: encrypted,
            timestamp: SystemTime::now(),
        })
    }
    
    /// Decrypt data
    ///
    /// # Errors
    ///
    /// Returns error if decryption key is not found
    pub fn decrypt(&self, encrypted_data: &EncryptedData) -> Result<Vec<u8>, String> {
        let key = self.keys.get(&encrypted_data.key_id)
            .ok_or("Decryption key not found")?;
        
        // Simplified decryption (XOR is its own inverse)
        let mut decrypted = Vec::new();
        for (i, &byte) in encrypted_data.data.iter().enumerate() {
            decrypted.push(byte ^ key[i % key.len()]);
        }
        
        self.stats.decryptions_performed.fetch_add(1, Ordering::Relaxed);
        
        Ok(decrypted)
    }
    
    /// Rotate encryption key
    ///
    /// # Errors
    ///
    /// Returns error if old key is not found
    pub fn rotate_key(&mut self, old_key_id: &str, new_key_id: String, new_key: Vec<u8>) -> Result<(), String> {
        if !self.keys.contains_key(old_key_id) {
            return Err("Old key not found".to_string());
        }
        
        self.add_key(new_key_id.clone(), new_key)?;
        
        if self.default_key_id.as_ref() == Some(&old_key_id.to_string()) {
            self.default_key_id = Some(new_key_id);
        }
        
        self.stats.key_rotations.fetch_add(1, Ordering::Relaxed);
        
        Ok(())
    }
    
    /// Get encryption statistics
    pub fn stats(&self) -> (u64, u64, u64) {
        (
            self.stats.encryptions_performed.load(Ordering::Relaxed),
            self.stats.decryptions_performed.load(Ordering::Relaxed),
            self.stats.key_rotations.load(Ordering::Relaxed),
        )
    }
}

impl Default for EncryptionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Encrypted data container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    /// Key identifier used for encryption
    pub key_id: String,
    /// Encrypted data
    pub data: Vec<u8>,
    /// Encryption timestamp
    pub timestamp: SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encryption_manager() {
        let mut manager = EncryptionManager::new();
        manager.add_key("key1".to_string(), vec![42u8; 32]).unwrap();
        
        let data = b"sensitive data";
        let encrypted = manager.encrypt(data, Some("key1")).unwrap();
        let decrypted = manager.decrypt(&encrypted).unwrap();
        
        assert_eq!(data.to_vec(), decrypted);
    }
    
    #[test]
    fn test_default_key() {
        let mut manager = EncryptionManager::new();
        manager.add_key("default".to_string(), vec![42u8; 32]).unwrap();
        
        // Should use default key when not specified
        let data = b"test";
        let encrypted = manager.encrypt(data, None).unwrap();
        assert_eq!(encrypted.key_id, "default");
    }
    
    #[test]
    fn test_key_rotation() {
        let mut manager = EncryptionManager::new();
        manager.add_key("old".to_string(), vec![1u8; 32]).unwrap();
        
        manager.rotate_key("old", "new".to_string(), vec![2u8; 32]).unwrap();
        
        let (_, _, rotations) = manager.stats();
        assert_eq!(rotations, 1);
    }
    
    #[test]
    fn test_invalid_key_length() {
        let mut manager = EncryptionManager::new();
        let result = manager.add_key("bad".to_string(), vec![1u8; 16]); // Wrong size
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("32 bytes"));
    }
}

