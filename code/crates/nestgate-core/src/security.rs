/*!
 * Security module for the Port Manager
 */

use axum::http::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::errors::{Error, Result};

/// Authentication context for API requests
#[derive(Debug, Clone)]
pub struct AuthContext {
    /// User ID
    pub user_id: String,

    /// Username for display
    pub username: String,

    /// User role
    pub role: Role,

    /// API key used for authentication
    pub api_key: String,

    /// Authentication timestamp
    pub auth_time: Instant,

    /// Permissions
    pub permissions: Vec<Permission>,
}

/// User roles
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    /// Administrator with full access
    Admin,

    /// Operator with service management access
    Operator,

    /// Read-only access
    ReadOnly,

    /// Service account for automated systems
    Service,
}

/// Permissions for various operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Permission {
    /// Read service information
    ServiceRead,

    /// Start/stop services
    ServiceWrite,

    /// Register/unregister services
    ServiceAdmin,

    /// Read port information
    PortRead,

    /// Allocate/deallocate ports
    PortWrite,

    /// Read process information
    ProcessRead,

    /// Manage processes
    ProcessWrite,

    /// System administration
    SystemAdmin,

    /// Health monitoring access
    HealthRead,

    /// Metrics collection access
    MetricsRead,
}

/// Rate limiting information
#[derive(Debug, Clone)]
pub struct RateLimit {
    /// Maximum requests per window
    pub max_requests: u32,

    /// Time window in seconds
    pub window_seconds: u64,

    /// Current request count
    pub current_count: u32,

    /// Window start time
    pub window_start: Instant,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub api_keys: Vec<ApiKey>,
    pub rate_limiting: RateLimitConfig,
    pub ssl: SslConfig,
    pub ip_whitelist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
    pub permissions: Vec<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub enabled: bool,
    pub requests_per_minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslConfig {
    pub enabled: bool,
    pub cert_file: String,
    pub key_file: String,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            api_keys: vec![ApiKey {
                key: "admin-port-manager-key".to_string(),
                permissions: vec!["admin".to_string(), "read".to_string(), "write".to_string()],
                description: "Admin API key".to_string(),
            }],
            rate_limiting: RateLimitConfig {
                enabled: false,
                requests_per_minute: 100,
            },
            ssl: SslConfig {
                enabled: false,
                cert_file: String::new(),
                key_file: String::new(),
            },
            ip_whitelist: vec![],
        }
    }
}

/// Security manager for the port manager
#[derive(Clone)]
pub struct SecurityManager {
    /// API key storage
    api_keys: Arc<RwLock<HashMap<String, AuthContext>>>,

    /// Rate limiting storage
    rate_limits: Arc<Mutex<HashMap<String, RateLimit>>>,

    /// Security configuration
    config: SecurityConfig,
}

impl SecurityManager {
    /// Create a new security manager
    pub fn new(config: SecurityConfig) -> Self {
        let mut api_keys = HashMap::new();

        // Add default admin API key for development
        api_keys.insert(
            "admin-port-manager-key".to_string(),
            AuthContext {
                user_id: "admin".to_string(),
                username: "Administrator".to_string(),
                role: Role::Admin,
                api_key: "admin-port-manager-key".to_string(),
                auth_time: Instant::now(),
                permissions: vec![
                    Permission::ServiceRead,
                    Permission::ServiceWrite,
                    Permission::ServiceAdmin,
                    Permission::PortRead,
                    Permission::PortWrite,
                    Permission::ProcessRead,
                    Permission::ProcessWrite,
                    Permission::SystemAdmin,
                    Permission::HealthRead,
                    Permission::MetricsRead,
                ],
            },
        );

        // Add read-only API key
        api_keys.insert(
            "readonly-port-manager-key".to_string(),
            AuthContext {
                user_id: "readonly".to_string(),
                username: "Read-Only User".to_string(),
                role: Role::ReadOnly,
                api_key: "readonly-port-manager-key".to_string(),
                auth_time: Instant::now(),
                permissions: vec![
                    Permission::ServiceRead,
                    Permission::PortRead,
                    Permission::ProcessRead,
                    Permission::HealthRead,
                    Permission::MetricsRead,
                ],
            },
        );

        Self {
            api_keys: Arc::new(RwLock::new(api_keys)),
            rate_limits: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    /// Initialize the security manager
    pub async fn initialize(&self) -> Result<()> {
        tracing::info!("Initializing security manager");
        tracing::info!("Authentication enabled: {}", self.config.ssl.enabled);
        tracing::info!(
            "Rate limiting enabled: {}",
            self.config.rate_limiting.enabled
        );
        tracing::info!("TLS enabled: {}", self.config.ssl.enabled);
        Ok(())
    }

    /// Validate API key and return authentication context
    pub async fn authenticate(&self, api_key: &str) -> Result<AuthContext> {
        if !self.config.ssl.enabled {
            // If SSL is disabled, return admin context
            return Ok(AuthContext {
                user_id: "system".to_string(),
                username: "System".to_string(),
                role: Role::Admin,
                api_key: api_key.to_string(),
                auth_time: Instant::now(),
                permissions: vec![
                    Permission::ServiceRead,
                    Permission::ServiceWrite,
                    Permission::ServiceAdmin,
                    Permission::PortRead,
                    Permission::PortWrite,
                    Permission::ProcessRead,
                    Permission::ProcessWrite,
                    Permission::SystemAdmin,
                    Permission::HealthRead,
                    Permission::MetricsRead,
                ],
            });
        }

        let api_keys = self.api_keys.read().await;

        match api_keys.get(api_key) {
            Some(context) => {
                // Check if session is still valid
                if context.auth_time.elapsed().as_secs() > 3600 {
                    return Err(Error::Api("Session expired".to_string()));
                }

                Ok(context.clone())
            }
            None => Err(Error::Api("Invalid API key".to_string())),
        }
    }

    /// Check if user has required permission
    pub fn has_permission(&self, context: &AuthContext, permission: &Permission) -> bool {
        // Admin role has all permissions
        if context.role == Role::Admin {
            return true;
        }

        context.permissions.contains(permission)
    }

    /// Check rate limiting for a client IP
    pub fn check_rate_limit(&self, client_ip: &str) -> Result<()> {
        if !self.config.rate_limiting.enabled {
            return Ok(());
        }

        let mut rate_limits = self.rate_limits.lock().unwrap();
        let now = Instant::now();

        let rate_limit = rate_limits
            .entry(client_ip.to_string())
            .or_insert(RateLimit {
                max_requests: self.config.rate_limiting.requests_per_minute,
                window_seconds: 60,
                current_count: 0,
                window_start: now,
            });

        // Reset window if needed
        if now.duration_since(rate_limit.window_start).as_secs() >= rate_limit.window_seconds {
            rate_limit.current_count = 0;
            rate_limit.window_start = now;
        }

        // Check if limit exceeded
        if rate_limit.current_count >= rate_limit.max_requests {
            return Err(Error::Api("Rate limit exceeded".to_string()));
        }

        rate_limit.current_count += 1;
        Ok(())
    }

    /// Extract API key from headers
    pub fn extract_api_key(&self, headers: &HeaderMap) -> Option<String> {
        headers
            .get("x-api-key")
            .and_then(|value| value.to_str().ok())
            .map(|s| s.to_string())
    }

    /// Extract client IP from headers
    pub fn extract_client_ip(&self, headers: &HeaderMap) -> String {
        // Try various headers for client IP
        for &header_name in &["x-forwarded-for", "x-real-ip", "x-client-ip"] {
            if let Some(value) = headers.get(header_name) {
                if let Ok(ip) = value.to_str() {
                    return ip.split(',').next().unwrap_or("unknown").trim().to_string();
                }
            }
        }

        "unknown".to_string()
    }

    /// Add a new API key
    pub async fn add_api_key(&self, api_key: String, context: AuthContext) -> Result<()> {
        let mut api_keys = self.api_keys.write().await;
        api_keys.insert(api_key, context);
        Ok(())
    }

    /// Remove an API key
    pub async fn remove_api_key(&self, api_key: &str) -> Result<()> {
        let mut api_keys = self.api_keys.write().await;
        api_keys.remove(api_key);
        Ok(())
    }

    /// List all API keys (for admin use)
    pub async fn list_api_keys(&self) -> Vec<String> {
        let api_keys = self.api_keys.read().await;
        api_keys.keys().cloned().collect()
    }

    /// Get configuration
    pub fn config(&self) -> &SecurityConfig {
        &self.config
    }

    pub fn validate_api_key(&self, key: &str) -> bool {
        self.config
            .api_keys
            .iter()
            .any(|api_key| api_key.key == key)
    }

    pub fn get_permissions(&self, key: &str) -> Vec<String> {
        self.config
            .api_keys
            .iter()
            .find(|api_key| api_key.key == key)
            .map(|api_key| api_key.permissions.clone())
            .unwrap_or_default()
    }

    /// Check if rate limit is exceeded for a given key
    pub fn is_rate_limited(&self, key: &str) -> bool {
        let mut rate_limits = match self.rate_limits.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                // If the mutex is poisoned, we can still access the data
                // but we should log this as it indicates a panic occurred while holding the lock
                tracing::warn!("Rate limiter mutex was poisoned, recovering");
                poisoned.into_inner()
            }
        };

        let now = Instant::now();
        let window_duration =
            Duration::from_secs(self.config.rate_limiting.requests_per_minute as u64 * 60);

        let entry = rate_limits
            .entry(key.to_string())
            .or_insert_with(|| RateLimit {
                max_requests: self.config.rate_limiting.requests_per_minute,
                window_seconds: 60,
                current_count: 0,
                window_start: now,
            });

        // Reset window if expired
        if now.duration_since(entry.window_start) >= window_duration {
            entry.current_count = 0;
            entry.window_start = now;
        }

        entry.current_count += 1;
        entry.current_count > entry.max_requests
    }
}

/// Middleware function to validate authentication and authorization
pub async fn validate_request(
    security_manager: &SecurityManager,
    headers: &HeaderMap,
    required_permission: Permission,
) -> std::result::Result<AuthContext, (StatusCode, String)> {
    // Check IP allowlist if configured
    if !security_manager.config.ip_whitelist.is_empty() {
        let client_ip = security_manager.extract_client_ip(headers);
        if !security_manager.config.ip_whitelist.contains(&client_ip) && client_ip != "unknown" {
            return Err((StatusCode::FORBIDDEN, "IP not allowed".to_string()));
        }
    }

    // Check rate limiting
    let client_ip = security_manager.extract_client_ip(headers);
    if let Err(e) = security_manager.check_rate_limit(&client_ip) {
        return Err((StatusCode::TOO_MANY_REQUESTS, e.to_string()));
    }

    // Extract and validate API key
    let api_key = match security_manager.extract_api_key(headers) {
        Some(key) => key,
        None => return Err((StatusCode::UNAUTHORIZED, "Missing API key".to_string())),
    };

    // Authenticate
    let auth_context = match security_manager.authenticate(&api_key).await {
        Ok(context) => context,
        Err(e) => return Err((StatusCode::UNAUTHORIZED, e.to_string())),
    };

    // Check permission
    if !security_manager.has_permission(&auth_context, &required_permission) {
        return Err((
            StatusCode::FORBIDDEN,
            "Insufficient permissions".to_string(),
        ));
    }

    Ok(auth_context)
}
