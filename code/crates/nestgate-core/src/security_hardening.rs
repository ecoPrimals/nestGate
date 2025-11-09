//! # Security Hardening
//!
//! Comprehensive security hardening measures for production deployment,
//! including input validation, rate limiting, encryption, and security monitoring.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::time::{Duration, Instant, SystemTime};
use std::net::IpAddr;
use serde::{Serialize, Deserialize};

/// **SECURITY VALIDATOR**
///
/// Comprehensive input validation and sanitization
pub struct SecurityValidator {
    rules: HashMap<String, ValidationRule>,
    stats: ValidationStats,
}

#[derive(Debug, Clone)]
pub struct ValidationRule {
    pub name: String,
    pub validators: Vec<ValidationType>,
    pub max_length: Option<usize>,
    pub required: bool,
    pub sanitize: bool,
}

#[derive(Debug, Clone)]
pub enum ValidationType {
    Alphanumeric,
    Email,
    Url,
    IpAddress,
    JsonPath,
    SqlSafe,
    NoScripts,
    Numeric,
    Uuid,
    Base64,
}

#[derive(Debug, Default)]
struct ValidationStats {
    validations_performed: AtomicU64,
    validations_failed: AtomicU64,
    sanitizations_performed: AtomicU64,
    blocked_attempts: AtomicU64,
}

impl SecurityValidator {
    #[must_use]
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            stats: ValidationStats::default(),
        }
    }
    
    /// Add validation rule
    pub fn add_rule(&mut self, rule: ValidationRule) {
        self.rules.insert(rule.name.clone(), rule);
    }
    
    /// Validate input against rules
    pub fn validate(&self, field_name: &str, value: &str) -> ValidationResult {
        self.stats.validations_performed.fetch_add(1, Ordering::Relaxed);
        
        if let Some(rule) = self.rules.get(field_name) {
            // Check required
            if rule.required && value.is_empty() {
                self.stats.validations_failed.fetch_add(1, Ordering::Relaxed);
                return ValidationResult::Invalid(format!("Field '{field_name}' is required"));
            }
            
            // Check length
            if let Some(max_len) = rule.max_length {
                if value.len() > max_len {
                    self.stats.validations_failed.fetch_add(1, Ordering::Relaxed);
                    return ValidationResult::Invalid(format!("Field '{}' exceeds maximum length of {}", field_name, max_len);
                }
            }
            
            // Apply validators
            for validator in &rule.validators {
                if !self.apply_validator(validator, value) {
                    self.stats.validations_failed.fetch_add(1, Ordering::Relaxed);
                    return ValidationResult::Invalid(format!("Field '{}' failed {:?} validation", field_name, validator);
                }
            }
            
            // Sanitize if needed
            let sanitized_value = if rule.sanitize {
                self.stats.sanitizations_performed.fetch_add(1, Ordering::Relaxed);
                self.sanitize_input(value)
            } else {
                value.to_string()
            };
            
            ValidationResult::Valid(sanitized_value)
        } else {
            // No rule defined, basic sanitization
            ValidationResult::Valid(self.sanitize_input(value))
        }
    }
    
    fn apply_validator(&self, validator: &ValidationType, value: &str) -> bool {
        match validator {
            ValidationType::Alphanumeric => value.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-'),
            ValidationType::Email => self.validate_email(value),
            ValidationType::Url => self.validate_url(value),
            ValidationType::IpAddress => value.parse::<IpAddr>().is_ok(),
            ValidationType::JsonPath => self.validate_json_path(value),
            ValidationType::SqlSafe => self.validate_sql_safe(value),
            ValidationType::NoScripts => !self.contains_script_tags(value),
            ValidationType::Numeric => value.chars().all(|c| c.is_numeric() || c == '.' || c == '-'),
            ValidationType::Uuid => self.validate_uuid(value),
            ValidationType::Base64 => self.validate_base64(value),
        }
    }
    
    fn validate_email(&self, value: &str) -> bool {
        value.contains('@') && value.contains('.') && value.len() > 5 && value.len() < 255
    }
    
    fn validate_url(&self, value: &str) -> bool {
        value.starts_with("http://") || value.starts_with("https://")
    }
    
    fn validate_json_path(&self, value: &str) -> bool {
        !value.contains("..") && !value.contains("//")
    }
    
    fn validate_sql_safe(&self, value: &str) -> bool {
        let dangerous_keywords = ["SELECT", "INSERT", "UPDATE", "DELETE", "DROP", "CREATE", "ALTER", "EXEC", "UNION", "OR", "AND", "--", "/*", "*/"];
        let upper_value = value.to_uppercase();
        !dangerous_keywords.iter().any(|&keyword| upper_value.contains(keyword))
    }
    
    fn contains_script_tags(&self, value: &str) -> bool {
        let lower_value = value.to_lowercase();
        lower_value.contains("<script") || lower_value.contains("javascript:") || lower_value.contains("onclick") || lower_value.contains("onerror")
    }
    
    fn validate_uuid(&self, value: &str) -> bool {
        value.len() == 36 && value.chars().enumerate().all(|(i, c)| {
            match i {
                8 | 13 | 18 | 23 => c == '-',
                _ => c.is_ascii_hexdigit(),
            }
        })
    }
    
    fn validate_base64(&self, value: &str) -> bool {
        value.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '/' || c == '=')
    }
    
    fn sanitize_input(&self, value: &str) -> String {
        value
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
            .replace('&', "&amp;")
            .trim()
            .to_string()
    }
    
    /// Get validation statistics
    pub fn stats(&self) -> (u64, u64, u64, f64) {
        let performed = self.stats.validations_performed.load(Ordering::Relaxed);
        let failed = self.stats.validations_failed.load(Ordering::Relaxed);
        let sanitized = self.stats.sanitizations_performed.load(Ordering::Relaxed);
        let success_rate = if performed > 0 { (performed - failed) as f64 / performed as f64 } else { 1.0 };
        (performed, failed, sanitized, success_rate)
    }
}

#[derive(Debug)]
pub enum ValidationResult {
    Valid(String),
    Invalid(String),
}

/// **RATE LIMITER**
///
/// Advanced rate limiting with multiple strategies
pub struct RateLimiter {
    buckets: Mutex<HashMap<String, TokenBucket>>,
    global_limit: Option<RateLimit>,
    per_ip_limit: Option<RateLimit>,
    per_user_limit: Option<RateLimit>,
    stats: RateLimitStats,
}

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub requests_per_window: u64,
    pub window_duration: Duration,
}

#[derive(Debug)]
struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
    capacity: f64,
    refill_rate: f64, // tokens per second
}

#[derive(Debug, Default)]
struct RateLimitStats {
    requests_processed: AtomicU64,
    requests_blocked: AtomicU64,
    buckets_created: AtomicU64,
}

impl RateLimiter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            buckets: Mutex::new(HashMap::new()),
            global_limit: None,
            per_ip_limit: None,
            per_user_limit: None,
            stats: RateLimitStats::default(),
        }
    }
    
    /// Set global rate limit
    #[must_use]
    pub fn with_global_limit(mut self, limit: RateLimit) -> Self {
        self.global_limit = Some(limit);
        self
    }
    
    /// Set per-IP rate limit
    #[must_use]
    pub fn with_per_ip_limit(mut self, limit: RateLimit) -> Self {
        self.per_ip_limit = Some(limit);
        self
    }
    
    /// Set per-user rate limit
    #[must_use]
    pub fn with_per_user_limit(mut self, limit: RateLimit) -> Self {
        self.per_user_limit = Some(limit);
        self
    }
    
    /// Check if request is allowed
    pub fn is_allowed(&self, ip: &str, user_id: Option<&str>) -> RateLimitResult {
        self.stats.requests_processed.fetch_add(1, Ordering::Relaxed);
        
        // Check global limit
        if let Some(ref limit) = self.global_limit {
            if !self.check_bucket("global", limit) {
                self.stats.requests_blocked.fetch_add(1, Ordering::Relaxed);
                return RateLimitResult::Blocked("Global rate limit exceeded".to_string());
            }
        }
        
        // Check per-IP limit
        if let Some(ref limit) = self.per_ip_limit {
            let key = format!("ip:{ip}");
            if !self.check_bucket(&key, limit) {
                self.stats.requests_blocked.fetch_add(1, Ordering::Relaxed);
                return RateLimitResult::Blocked("IP rate limit exceeded".to_string());
            }
        }
        
        // Check per-user limit
        if let (Some(user_id), Some(ref limit)) = (user_id, &self.per_user_limit) {
            let key = format!("user:{user_id}");
            if !self.check_bucket(&key, limit) {
                self.stats.requests_blocked.fetch_add(1, Ordering::Relaxed);
                return RateLimitResult::Blocked("User rate limit exceeded".to_string());
            }
        }
        
        RateLimitResult::Allowed
    }
    
    fn check_bucket(&self, key: &str, limit: &RateLimit) -> bool {
        // ✅ SAFE: Mutex lock - handle poisoned mutex gracefully
        // If another thread panicked while holding this lock, we recover by creating a new state
        let mut buckets = self.buckets.lock().unwrap_or_else(|poisoned| {
            // Mutex was poisoned, but we can recover by accessing the underlying data
            poisoned.into_inner()
        });
        
        let bucket = buckets.entry(key.to_string()).or_insert_with(|| {
            self.stats.buckets_created.fetch_add(1, Ordering::Relaxed);
            TokenBucket {
                tokens: limit.requests_per_window as f64,
                last_refill: Instant::now(),
                capacity: limit.requests_per_window as f64,
                refill_rate: limit.requests_per_window as f64 / limit.window_duration.as_secs_f64(),
            }
        });
        
        // Refill tokens
        let now = Instant::now();
        let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
        bucket.tokens = (bucket.tokens + elapsed * bucket.refill_rate).min(bucket.capacity);
        bucket.last_refill = now;
        
        // Check if request is allowed
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }
    
    /// Get rate limiting statistics
    pub fn stats(&self) -> (u64, u64, u64, f64) {
        let processed = self.stats.requests_processed.load(Ordering::Relaxed);
        let blocked = self.stats.requests_blocked.load(Ordering::Relaxed);
        let buckets = self.stats.buckets_created.load(Ordering::Relaxed);
        let block_rate = if processed > 0 { blocked as f64 / processed as f64 } else { 0.0 };
        (processed, blocked, buckets, block_rate)
    }
}

#[derive(Debug)]
pub enum RateLimitResult {
    Allowed,
    Blocked(String),
}

/// **SECURITY MONITOR**
///
/// Security event monitoring and threat detection
pub struct SecurityMonitor {
    events: Mutex<Vec<SecurityEvent>>,
    threat_patterns: Vec<ThreatPattern>,
    stats: SecurityStats,
    max_events: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub event_type: SecurityEventType,
    pub timestamp: SystemTime,
    pub source_ip: String,
    pub user_id: Option<String>,
    pub description: String,
    pub severity: SecuritySeverity,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEventType {
    AuthenticationFailure,
    AuthorizationFailure,
    InputValidationFailure,
    RateLimitExceeded,
    SuspiciousActivity,
    DataAccessViolation,
    ConfigurationChange,
    SystemIntrusion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct ThreatPattern {
    pub name: String,
    pub pattern: String,
    pub severity: SecuritySeverity,
    pub action: ThreatAction,
}

#[derive(Debug, Clone)]
pub enum ThreatAction {
    Log,
    Block,
    Alert,
    Quarantine,
}

#[derive(Debug, Default)]
struct SecurityStats {
    events_recorded: AtomicU64,
    threats_detected: AtomicU64,
    actions_taken: AtomicU64,
}

impl SecurityMonitor {
    #[must_use]
    pub fn new(max_events: usize) -> Self {
        let mut monitor = Self {
            events: Mutex::new(Vec::new()),
            threat_patterns: Vec::new(),
            stats: SecurityStats::default(),
            max_events,
        };
        
        // Add default threat patterns
        monitor.add_default_patterns();
        monitor
    }
    
    fn add_default_patterns(&mut self) {
        self.threat_patterns.extend(vec![
            ThreatPattern {
                name: "SQL Injection".to_string(),
                pattern: "(?i)(union|select|insert|update|delete|drop|create|alter)".to_string(),
                severity: SecuritySeverity::High,
                action: ThreatAction::Block,
            },
            ThreatPattern {
                name: "XSS Attempt".to_string(),
                pattern: "(?i)(<script|javascript:|onclick|onerror)".to_string(),
                severity: SecuritySeverity::High,
                action: ThreatAction::Block,
            },
            ThreatPattern {
                name: "Path Traversal".to_string(),
                pattern: r"(\.\./|\.\.\\|%2e%2e%2f)".to_string(),
                severity: SecuritySeverity::Medium,
                action: ThreatAction::Block,
            },
            ThreatPattern {
                name: "Command Injection".to_string(),
                pattern: r"(;|\||&|`|\$\(|\${)".to_string(),
                severity: SecuritySeverity::High,
                action: ThreatAction::Block,
            },
        ]);
    }
    
    /// Record security event
    pub fn record_event(&self, event: SecurityEvent) {
        self.stats.events_recorded.fetch_add(1, Ordering::Relaxed);
        
        // ✅ SAFE: Mutex lock - handle poisoned mutex gracefully
        // If poisoned, we can still record events by recovering the underlying data
        let mut events = self.events.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        events.push(event);
        
        // Trim if over max capacity
        if events.len() > self.max_events {
            events.remove(0);
        }
    }
    
    /// Analyze input for threats
    pub fn analyze_input(&self, input: &str, source_ip: &str) -> ThreatAnalysisResult {
        for pattern in &self.threat_patterns {
            if let Ok(regex) = regex::Regex::new(&pattern.pattern) {
                if regex.is_match(input) {
                    self.stats.threats_detected.fetch_add(1, Ordering::Relaxed);
                    
                    let event = SecurityEvent {
                        event_type: SecurityEventType::SuspiciousActivity,
                        timestamp: SystemTime::now(),
                        source_ip: source_ip.to_string(),
                        user_id: None,
                        description: format!("Threat pattern '{pattern.name}' detected in input"),
                        severity: pattern.severity.clone(),
                        metadata: {
                            let mut meta = HashMap::new();
                            meta.insert("pattern".to_string(), pattern.name.clone());
                            meta.insert("input_sample".to_string(), input.chars().take(100).collect());
                            meta
                        },
                    };
                    
                    self.record_event(event);
                    
                    match pattern.action {
                        ThreatAction::Block => {
                            self.stats.actions_taken.fetch_add(1, Ordering::Relaxed);
                            return ThreatAnalysisResult::Blocked(pattern.name.clone());
                        }
                        ThreatAction::Alert => {
                            self.stats.actions_taken.fetch_add(1, Ordering::Relaxed);
                            return ThreatAnalysisResult::Alert(pattern.name.clone());
                        }
                        _ => {}
                    }
                }
            }
        }
        
        ThreatAnalysisResult::Safe
    }
    
    /// Get security events
    pub fn get_events(&self, severity_filter: Option<SecuritySeverity>) -> Vec<SecurityEvent> {
        // ✅ SAFE: Mutex lock - handle poisoned mutex gracefully
        // If poisoned, we can still read events by recovering the underlying data
        let events = self.events.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        
        match severity_filter {
            Some(severity) => events.iter()
                .filter(|e| std::mem::discriminant(&e.severity) == std::mem::discriminant(&severity))
                .cloned()
                .collect(),
            None => events.clone(),
        }
    }
    
    /// Get security statistics
    pub fn stats(&self) -> (u64, u64, u64) {
        (
            self.stats.events_recorded.load(Ordering::Relaxed),
            self.stats.threats_detected.load(Ordering::Relaxed),
            self.stats.actions_taken.load(Ordering::Relaxed),
        )
    }
}

#[derive(Debug)]
pub enum ThreatAnalysisResult {
    Safe,
    Alert(String),
    Blocked(String),
}

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
    #[must_use]
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            default_key_id: None,
            stats: EncryptionStats::default(),
        }
    }
    
    /// Add encryption key
    pub fn add_key(&mut self, key_id: String, key: Vec<u8>) {
        if key.len() != 32 {
            panic!("Key must be 32 bytes for AES-256");
        }
        
        self.keys.insert(key_id.clone(), key);
        
        if self.default_key_id.is_none() {
            self.default_key_id = Some(key_id);
        }
    }
    
    /// Encrypt data
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn encrypt(&self, data: &[u8], key_id: Option<&str>) -> Result<EncryptedData, String>  {
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
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn decrypt(&self, encrypted_data: &EncryptedData) -> Result<Vec<u8>, String>  {
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
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn rotate_key(&mut self, old_key_id: &str, new_key_id: String, new_key: Vec<u8>) -> Result<(), String>  {
        if !self.keys.contains_key(old_key_id) {
            return Err("Old key not found".to_string());
        }
        
        self.add_key(new_key_id.clone(), new_key);
        
        if self.default_key_id.as_ref() == Some(old_key_id) {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub key_id: String,
    pub data: Vec<u8>,
    pub timestamp: SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_security_validator() {
        let mut validator = SecurityValidator::new();
        
        let rule = ValidationRule {
            name: "username".to_string(),
            validators: vec![ValidationType::Alphanumeric],
            max_length: Some(50),
            required: true,
            sanitize: true,
        };
        
        validator.add_rule(rule);
        
        // Valid input
        match validator.validate("username", "valid_user123") {
            ValidationResult::Valid(_) => {},
            ValidationResult::Invalid(_) => panic!("Should be valid"),
        }
        
        // Invalid input (contains special characters)
        match validator.validate("username", "invalid<script>") {
            ValidationResult::Valid(_) => panic!("Should be invalid"),
            ValidationResult::Invalid(_) => {},
        }
    }
    
    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new()
            .with_per_ip_limit(RateLimit {
                requests_per_window: 5,
                window_duration: Duration::from_secs(60),
            });
        
        // First 5 requests should be allowed
        for _ in 0..5 {
            match limiter.is_allowed("192.168.1.1", None) {
                RateLimitResult::Allowed => {},
                RateLimitResult::Blocked(_) => panic!("Should be allowed"),
            }
        }
        
        // 6th request should be blocked
        match limiter.is_allowed("192.168.1.1", None) {
            RateLimitResult::Allowed => panic!("Should be blocked"),
            RateLimitResult::Blocked(_) => {},
        }
    }
    
    #[test]
    fn test_security_monitor() {
        let monitor = SecurityMonitor::new(100);
        
        // Test SQL injection detection
        match monitor.analyze_input("SELECT * FROM users", "192.168.1.1") {
            ThreatAnalysisResult::Blocked(_) => {},
            _ => panic!("Should detect SQL injection"),
        }
        
        // Test safe input
        match monitor.analyze_input("Hello world", "192.168.1.1") {
            ThreatAnalysisResult::Safe => {},
            _ => panic!("Should be safe"),
        }
    }
    
    #[test]
    fn test_encryption_manager() {
        let mut manager = EncryptionManager::new();
        let key = vec![0u8; 32]; // 32-byte key for AES-256
        manager.add_key("test_key".to_string(), key);
        
        let data = b"Hello, World!";
        let encrypted = manager.encrypt(data, None).expect("Security operation failed");
        let decrypted = manager.decrypt(&encrypted).expect("Security operation failed");
        
        assert_eq!(data.to_vec(), decrypted);
    }
    
    // **COMPREHENSIVE ENCRYPTION MANAGER TESTS** (Added Nov 3, 2025)
    
    #[test]
    fn test_encryption_manager_new_empty() {
        let manager = EncryptionManager::new();
        assert!(manager.keys.is_empty());
        assert!(manager.default_key_id.is_none());
    }

    #[test]
    fn test_add_valid_32_byte_key() {
        let mut manager = EncryptionManager::new();
        manager.add_key("key1".to_string(), vec![42u8; 32]);
        
        assert_eq!(manager.keys.len(), 1);
        assert_eq!(manager.default_key_id, Some("key1".to_string()));
    }

    #[test]
    #[should_panic(expected = "Key must be 32 bytes")]
    fn test_add_key_too_short() {
        let mut manager = EncryptionManager::new();
        manager.add_key("short".to_string(), vec![0u8; 16]);
    }

    #[test]
    #[should_panic(expected = "Key must be 32 bytes")]
    fn test_add_key_too_long() {
        let mut manager = EncryptionManager::new();
        manager.add_key("long".to_string(), vec![0u8; 64]);
    }

    #[test]
    fn test_multiple_keys_first_is_default() {
        let mut manager = EncryptionManager::new();
        manager.add_key("first".to_string(), vec![1u8; 32]);
        manager.add_key("second".to_string(), vec![2u8; 32]);
        
        assert_eq!(manager.keys.len(), 2);
        assert_eq!(manager.default_key_id, Some("first".to_string()));
    }

    #[test]
    fn test_encrypt_with_specific_key() {
        let mut manager = EncryptionManager::new();
        manager.add_key("key1".to_string(), vec![1u8; 32]);
        manager.add_key("key2".to_string(), vec![2u8; 32]);
        
        let encrypted = manager.encrypt(b"test", Some("key2"))
            .expect("Encryption with specific key should succeed");
        assert_eq!(encrypted.key_id, "key2");
    }

    #[test]
    fn test_encrypt_no_key_error() {
        let manager = EncryptionManager::new();
        let result = manager.encrypt(b"data", None);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No encryption key available");
    }

    #[test]
    fn test_encrypt_nonexistent_key_error() {
        let mut manager = EncryptionManager::new();
        manager.add_key("exists".to_string(), vec![1u8; 32]);
        
        let result = manager.encrypt(b"data", Some("missing"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Encryption key not found");
    }

    #[test]
    fn test_decrypt_roundtrip() {
        let mut manager = EncryptionManager::new();
        manager.add_key("key1".to_string(), vec![99u8; 32]);
        
        let original = b"Secret message to encrypt and decrypt!";
        let encrypted = manager.encrypt(original, None)
            .expect("Encryption should succeed for roundtrip test");
        let decrypted = manager.decrypt(&encrypted)
            .expect("Decryption should succeed for roundtrip test");
        
        assert_eq!(decrypted.as_slice(), original);
    }

    #[test]
    fn test_encrypt_empty_data() {
        let mut manager = EncryptionManager::new();
        manager.add_key("key1".to_string(), vec![42u8; 32]);
        
        let encrypted = manager.encrypt(b"", None)
            .expect("Encrypting empty data should succeed");
        let decrypted = manager.decrypt(&encrypted)
            .expect("Decrypting empty data should succeed");
        
        assert_eq!(decrypted, b"");
    }

    #[test]
    fn test_encrypt_large_data() {
        let mut manager = EncryptionManager::new();
        manager.add_key("key1".to_string(), vec![77u8; 32]);
        
        let large = vec![42u8; 50_000];
        let encrypted = manager.encrypt(&large, None)
            .expect("Encrypting large data should succeed");
        let decrypted = manager.decrypt(&encrypted)
            .expect("Decrypting large data should succeed");
        
        assert_eq!(decrypted, large);
    }

    #[test]
    fn test_decrypt_key_not_found() {
        let manager = EncryptionManager::new();
        
        let fake = EncryptedData {
            key_id: "nonexistent".to_string(),
            data: vec![1, 2, 3],
            timestamp: SystemTime::now(),
        };
        
        let result = manager.decrypt(&fake);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Decryption key not found");
    }

    #[test]
    fn test_rotate_key_success() {
        let mut manager = EncryptionManager::new();
        manager.add_key("old".to_string(), vec![1u8; 32]);
        
        let result = manager.rotate_key("old", "new".to_string(), vec![2u8; 32]);
        assert!(result.is_ok());
        
        assert_eq!(manager.keys.len(), 2);
        assert_eq!(manager.default_key_id, Some("new".to_string()));
    }

    #[test]
    fn test_rotate_nonexistent_key() {
        let manager = EncryptionManager::new();
        
        let result = manager.rotate_key("missing", "new".to_string(), vec![1u8; 32]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Old key not found");
    }

    #[test]
    fn test_stats_tracking() {
        let mut manager = EncryptionManager::new();
        manager.add_key("key1".to_string(), vec![42u8; 32]);
        
        let enc = manager.encrypt(b"test", None)
            .expect("Encryption should succeed before rotation");
        manager.decrypt(&enc)
            .expect("Decryption should succeed before rotation");
        manager.rotate_key("key1", "key2".to_string(), vec![43u8; 32])
            .expect("Key rotation should succeed");
        
        let (encryptions, decryptions, rotations) = manager.stats();
        assert_eq!(encryptions, 1);
        assert_eq!(decryptions, 1);
        assert_eq!(rotations, 1);
    }

    #[test]
    fn test_xor_is_reversible() {
        let mut manager = EncryptionManager::new();
        let key = vec![123u8; 32];
        manager.add_key("key".to_string(), key.clone());
        
        let original = b"Test XOR property";
        let encrypted = manager.encrypt(original, None)
            .expect("Encryption should succeed for XOR test");
        
        // Manual XOR should recover original
        let mut manual = Vec::new();
        for (i, &byte) in encrypted.data.iter().enumerate() {
            manual.push(byte ^ key[i % key.len()]);
        }
        
        assert_eq!(manual.as_slice(), original);
    }

    #[test]
    fn test_different_keys_different_ciphertext() {
        let mut manager = EncryptionManager::new();
        manager.add_key("key1".to_string(), vec![1u8; 32]);
        manager.add_key("key2".to_string(), vec![2u8; 32]);
        
        let plaintext = b"Same plaintext";
        let enc1 = manager.encrypt(plaintext, Some("key1"))
            .expect("Encryption with key1 should succeed");
        let enc2 = manager.encrypt(plaintext, Some("key2"))
            .expect("Encryption with key2 should succeed");
        
        assert_ne!(enc1.data, enc2.data);
    }

    #[test]
    fn test_timestamp_recorded() {
        let mut manager = EncryptionManager::new();
        manager.add_key("key1".to_string(), vec![42u8; 32]);
        
        let before = SystemTime::now();
        let encrypted = manager.encrypt(b"test", None)
            .expect("Encryption should succeed for timestamp test");
        let after = SystemTime::now();
        
        assert!(encrypted.timestamp >= before);
        assert!(encrypted.timestamp <= after);
    }

    #[test]
    fn test_all_byte_values() {
        let mut manager = EncryptionManager::new();
        manager.add_key("key1".to_string(), vec![128u8; 32]);
        
        let all_bytes: Vec<u8> = (0..=255).collect();
        let encrypted = manager.encrypt(&all_bytes, None)
            .expect("Encrypting all byte values should succeed");
        let decrypted = manager.decrypt(&encrypted)
            .expect("Decrypting all byte values should succeed");
        
        assert_eq!(decrypted, all_bytes);
    }
} 