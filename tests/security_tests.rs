//! Comprehensive Security Test Suite for Songbird Orchestrator
//!
//! Enterprise-grade security testing covering:
//! - Authentication providers and flows
//! - Authorization and permission systems
//! - Audit logging and security events
//! - Credential handling and validation
//! - Security context management
//! - Role-based access control (RBAC)

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

use songbird_orchestrator::{
    security::{
        Action, AuditEvent, AuthenticationProvider, AuthenticationResult, AuthorizationProvider,
        Condition, Credentials, Permission, Resource, SecurityProvider, Subject,
    },
    traits::service::{AuthInfo, ClientInfo, ServiceRequest, ServiceResponse, UniversalService},
    Result, SongbirdError,
};

mod common;
use common::{MockConfig, MockHealth, MockService};

/// Mock Security Provider for testing
#[derive(Debug)]
pub struct MockSecurityProvider {
    users: Arc<RwLock<HashMap<String, MockUser>>>,
    audit_events: Arc<RwLock<Vec<AuditEvent>>>,
    permissions: Arc<RwLock<HashMap<String, Vec<Permission>>>>,
    encryption_key: Vec<u8>,
}

#[derive(Debug, Clone)]
struct MockUser {
    id: String,
    username: String,
    password_hash: String,
    roles: Vec<String>,
    permissions: Vec<String>,
    attributes: HashMap<String, String>,
    active: bool,
}

impl MockSecurityProvider {
    pub fn new() -> Self {
        let mut provider = Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            audit_events: Arc::new(RwLock::new(Vec::new())),
            permissions: Arc::new(RwLock::new(HashMap::new())),
            encryption_key: b"test-encryption-key-32-bytes!!".to_vec(),
        };

        // Add test users
        provider.add_test_users();
        provider
    }

    fn add_test_users(&mut self) {
        // This would normally be async, but for setup we'll use a blocking approach
        let users = vec![
            MockUser {
                id: "admin-001".to_string(),
                username: "admin".to_string(),
                password_hash: "hashed_admin_password".to_string(),
                roles: vec!["admin".to_string(), "user".to_string()],
                permissions: vec!["read".to_string(), "write".to_string(), "delete".to_string()],
                attributes: HashMap::from([
                    ("department".to_string(), "security".to_string()),
                    ("clearance".to_string(), "high".to_string()),
                ]),
                active: true,
            },
            MockUser {
                id: "user-001".to_string(),
                username: "testuser".to_string(),
                password_hash: "hashed_user_password".to_string(),
                roles: vec!["user".to_string()],
                permissions: vec!["read".to_string()],
                attributes: HashMap::from([
                    ("department".to_string(), "engineering".to_string()),
                    ("clearance".to_string(), "standard".to_string()),
                ]),
                active: true,
            },
            MockUser {
                id: "guest-001".to_string(),
                username: "guest".to_string(),
                password_hash: "hashed_guest_password".to_string(),
                roles: vec!["guest".to_string()],
                permissions: vec![],
                attributes: HashMap::new(),
                active: true,
            },
            MockUser {
                id: "disabled-001".to_string(),
                username: "disabled".to_string(),
                password_hash: "hashed_disabled_password".to_string(),
                roles: vec!["user".to_string()],
                permissions: vec!["read".to_string()],
                attributes: HashMap::new(),
                active: false,
            },
        ];

        // In a real implementation, this would be async
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Handle::current();
            rt.block_on(async {
                let mut user_map = self.users.write().await;
                for user in users {
                    user_map.insert(user.username.clone(), user);
                }
            });
        });
    }

    pub async fn get_audit_events(&self) -> Vec<AuditEvent> {
        self.audit_events.read().await.clone()
    }

    pub async fn clear_audit_events(&self) {
        self.audit_events.write().await.clear();
    }
}

#[async_trait]
impl SecurityProvider for MockSecurityProvider {
    async fn authenticate(&self, credentials: &Credentials) -> Result<AuthenticationResult> {
        match credentials {
            Credentials::Basic { username, password } => {
                let users = self.users.read().await;
                if let Some(user) = users.get(username) {
                    if !user.active {
                        return Ok(AuthenticationResult {
                            success: false,
                            subject: None,
                            token: None,
                            expires_at: None,
                            permissions: vec![],
                        });
                    }

                    // In real implementation, verify password hash
                    let password_valid = password == "correct_password" || username == "admin";

                    if password_valid {
                        let subject = Subject {
                            id: user.id.clone(),
                            name: user.username.clone(),
                            roles: user.roles.clone(),
                            attributes: user.attributes.clone(),
                        };

                        let permissions = user
                            .permissions
                            .iter()
                            .map(|p| Permission {
                                resource_pattern: "/*".to_string(),
                                actions: vec![p.clone()],
                                conditions: vec![],
                            })
                            .collect();

                        Ok(AuthenticationResult {
                            success: true,
                            subject: Some(subject),
                            token: Some(format!("token_{}", user.id)),
                            expires_at: Some(Utc::now() + chrono::Duration::hours(1)),
                            permissions,
                        })
                    } else {
                        Ok(AuthenticationResult {
                            success: false,
                            subject: None,
                            token: None,
                            expires_at: None,
                            permissions: vec![],
                        })
                    }
                } else {
                    Ok(AuthenticationResult {
                        success: false,
                        subject: None,
                        token: None,
                        expires_at: None,
                        permissions: vec![],
                    })
                }
            }
            Credentials::Bearer { token } => {
                // Validate bearer token
                if token.starts_with("token_") {
                    let user_id = token.strip_prefix("token_").unwrap();
                    let users = self.users.read().await;
                    
                    if let Some((_, user)) = users.iter().find(|(_, u)| u.id == user_id) {
                        let subject = Subject {
                            id: user.id.clone(),
                            name: user.username.clone(),
                            roles: user.roles.clone(),
                            attributes: user.attributes.clone(),
                        };

                        let permissions = user
                            .permissions
                            .iter()
                            .map(|p| Permission {
                                resource_pattern: "/*".to_string(),
                                actions: vec![p.clone()],
                                conditions: vec![],
                            })
                            .collect();

                        Ok(AuthenticationResult {
                            success: true,
                            subject: Some(subject),
                            token: Some(token.clone()),
                            expires_at: Some(Utc::now() + chrono::Duration::hours(1)),
                            permissions,
                        })
                    } else {
                        Ok(AuthenticationResult {
                            success: false,
                            subject: None,
                            token: None,
                            expires_at: None,
                            permissions: vec![],
                        })
                    }
                } else {
                    Ok(AuthenticationResult {
                        success: false,
                        subject: None,
                        token: None,
                        expires_at: None,
                        permissions: vec![],
                    })
                }
            }
            Credentials::ApiKey { key } => {
                // Simple API key validation
                if key == "valid-api-key" {
                    let subject = Subject {
                        id: "api-client".to_string(),
                        name: "API Client".to_string(),
                        roles: vec!["api".to_string()],
                        attributes: HashMap::from([("type".to_string(), "api".to_string())]),
                    };

                    Ok(AuthenticationResult {
                        success: true,
                        subject: Some(subject),
                        token: Some(key.clone()),
                        expires_at: None,
                        permissions: vec![Permission {
                            resource_pattern: "/api/*".to_string(),
                            actions: vec!["read".to_string(), "write".to_string()],
                            conditions: vec![],
                        }],
                    })
                } else {
                    Ok(AuthenticationResult {
                        success: false,
                        subject: None,
                        token: None,
                        expires_at: None,
                        permissions: vec![],
                    })
                }
            }
            _ => Ok(AuthenticationResult {
                success: false,
                subject: None,
                token: None,
                expires_at: None,
                permissions: vec![],
            }),
        }
    }

    async fn authorize(
        &self,
        subject: &Subject,
        resource: &Resource,
        action: &Action,
    ) -> Result<bool> {
        // Check if user has admin role (full access)
        if subject.roles.contains(&"admin".to_string()) {
            return Ok(true);
        }

        // Check specific permissions
        let users = self.users.read().await;
        if let Some((_, user)) = users.iter().find(|(_, u)| u.id == subject.id) {
            // Simple permission check
            let has_permission = match action.name.as_str() {
                "read" => user.permissions.contains(&"read".to_string()),
                "write" => user.permissions.contains(&"write".to_string()),
                "delete" => user.permissions.contains(&"delete".to_string()),
                _ => false,
            };

            // Additional resource-specific checks
            let resource_allowed = match resource.type_.as_str() {
                "service" => true, // All authenticated users can access services
                "admin" => subject.roles.contains(&"admin".to_string()),
                "user_data" => {
                    // Users can only access their own data
                    resource.id == subject.id || subject.roles.contains(&"admin".to_string())
                }
                _ => has_permission,
            };

            Ok(has_permission && resource_allowed)
        } else {
            Ok(false)
        }
    }

    async fn audit_log(&self, event: AuditEvent) -> Result<()> {
        self.audit_events.write().await.push(event);
        Ok(())
    }

    async fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Simple XOR encryption for testing
        let mut encrypted = Vec::with_capacity(data.len());
        for (i, byte) in data.iter().enumerate() {
            let key_byte = self.encryption_key[i % self.encryption_key.len()];
            encrypted.push(byte ^ key_byte);
        }
        Ok(encrypted)
    }

    async fn decrypt_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        // XOR decryption (same as encryption for XOR)
        self.encrypt_data(data).await
    }
}

#[async_trait]
impl AuthenticationProvider for MockSecurityProvider {
    async fn authenticate(&self, credentials: &Credentials) -> Result<AuthenticationResult> {
        SecurityProvider::authenticate(self, credentials).await
    }
}

#[async_trait]
impl AuthorizationProvider for MockSecurityProvider {
    async fn authorize(
        &self,
        subject: &Subject,
        resource: &Resource,
        action: &Action,
    ) -> Result<bool> {
        SecurityProvider::authorize(self, subject, resource, action).await
    }
}

// SECURITY TEST SUITE

#[tokio::test]
async fn test_authentication_basic_valid_credentials() {
    let provider = MockSecurityProvider::new();
    
    let credentials = Credentials::Basic {
        username: "admin".to_string(),
        password: "correct_password".to_string(),
    };

    let result = provider.authenticate(&credentials).await.unwrap();
    
    assert!(result.success);
    assert!(result.subject.is_some());
    assert!(result.token.is_some());
    assert!(result.expires_at.is_some());
    assert!(!result.permissions.is_empty());
    
    let subject = result.subject.unwrap();
    assert_eq!(subject.id, "admin-001");
    assert_eq!(subject.name, "admin");
    assert!(subject.roles.contains(&"admin".to_string()));
}

#[tokio::test]
async fn test_authentication_basic_invalid_credentials() {
    let provider = MockSecurityProvider::new();
    
    let credentials = Credentials::Basic {
        username: "admin".to_string(),
        password: "wrong_password".to_string(),
    };

    let result = provider.authenticate(&credentials).await.unwrap();
    
    assert!(!result.success);
    assert!(result.subject.is_none());
    assert!(result.token.is_none());
    assert!(result.expires_at.is_none());
    assert!(result.permissions.is_empty());
}

#[tokio::test]
async fn test_authentication_nonexistent_user() {
    let provider = MockSecurityProvider::new();
    
    let credentials = Credentials::Basic {
        username: "nonexistent".to_string(),
        password: "any_password".to_string(),
    };

    let result = provider.authenticate(&credentials).await.unwrap();
    
    assert!(!result.success);
    assert!(result.subject.is_none());
}

#[tokio::test]
async fn test_authentication_disabled_user() {
    let provider = MockSecurityProvider::new();
    
    let credentials = Credentials::Basic {
        username: "disabled".to_string(),
        password: "correct_password".to_string(),
    };

    let result = provider.authenticate(&credentials).await.unwrap();
    
    assert!(!result.success);
    assert!(result.subject.is_none());
}

#[tokio::test]
async fn test_authentication_bearer_token_valid() {
    let provider = MockSecurityProvider::new();
    
    let credentials = Credentials::Bearer {
        token: "token_admin-001".to_string(),
    };

    let result = provider.authenticate(&credentials).await.unwrap();
    
    assert!(result.success);
    assert!(result.subject.is_some());
    assert_eq!(result.token.unwrap(), "token_admin-001");
}

#[tokio::test]
async fn test_authentication_bearer_token_invalid() {
    let provider = MockSecurityProvider::new();
    
    let credentials = Credentials::Bearer {
        token: "invalid_token".to_string(),
    };

    let result = provider.authenticate(&credentials).await.unwrap();
    
    assert!(!result.success);
    assert!(result.subject.is_none());
}

#[tokio::test]
async fn test_authentication_api_key_valid() {
    let provider = MockSecurityProvider::new();
    
    let credentials = Credentials::ApiKey {
        key: "valid-api-key".to_string(),
    };

    let result = provider.authenticate(&credentials).await.unwrap();
    
    assert!(result.success);
    assert!(result.subject.is_some());
    
    let subject = result.subject.unwrap();
    assert_eq!(subject.id, "api-client");
    assert!(subject.roles.contains(&"api".to_string()));
}

#[tokio::test]
async fn test_authentication_api_key_invalid() {
    let provider = MockSecurityProvider::new();
    
    let credentials = Credentials::ApiKey {
        key: "invalid-api-key".to_string(),
    };

    let result = provider.authenticate(&credentials).await.unwrap();
    
    assert!(!result.success);
    assert!(result.subject.is_none());
}

#[tokio::test]
async fn test_authorization_admin_full_access() {
    let provider = MockSecurityProvider::new();
    
    let subject = Subject {
        id: "admin-001".to_string(),
        name: "admin".to_string(),
        roles: vec!["admin".to_string()],
        attributes: HashMap::new(),
    };

    let resource = Resource {
        type_: "service".to_string(),
        id: "test-service".to_string(),
        attributes: HashMap::new(),
    };

    let action = Action {
        name: "delete".to_string(),
        parameters: HashMap::new(),
    };

    let authorized = provider.authorize(&subject, &resource, &action).await.unwrap();
    assert!(authorized);
}

#[tokio::test]
async fn test_authorization_user_read_access() {
    let provider = MockSecurityProvider::new();
    
    let subject = Subject {
        id: "user-001".to_string(),
        name: "testuser".to_string(),
        roles: vec!["user".to_string()],
        attributes: HashMap::new(),
    };

    let resource = Resource {
        type_: "service".to_string(),
        id: "test-service".to_string(),
        attributes: HashMap::new(),
    };

    let action = Action {
        name: "read".to_string(),
        parameters: HashMap::new(),
    };

    let authorized = provider.authorize(&subject, &resource, &action).await.unwrap();
    assert!(authorized);
}

#[tokio::test]
async fn test_authorization_user_denied_write_access() {
    let provider = MockSecurityProvider::new();
    
    let subject = Subject {
        id: "user-001".to_string(),
        name: "testuser".to_string(),
        roles: vec!["user".to_string()],
        attributes: HashMap::new(),
    };

    let resource = Resource {
        type_: "service".to_string(),
        id: "test-service".to_string(),
        attributes: HashMap::new(),
    };

    let action = Action {
        name: "write".to_string(),
        parameters: HashMap::new(),
    };

    let authorized = provider.authorize(&subject, &resource, &action).await.unwrap();
    assert!(!authorized);
}

#[tokio::test]
async fn test_authorization_user_own_data_access() {
    let provider = MockSecurityProvider::new();
    
    let subject = Subject {
        id: "user-001".to_string(),
        name: "testuser".to_string(),
        roles: vec!["user".to_string()],
        attributes: HashMap::new(),
    };

    let resource = Resource {
        type_: "user_data".to_string(),
        id: "user-001".to_string(), // Same as subject ID
        attributes: HashMap::new(),
    };

    let action = Action {
        name: "read".to_string(),
        parameters: HashMap::new(),
    };

    let authorized = provider.authorize(&subject, &resource, &action).await.unwrap();
    assert!(authorized);
}

#[tokio::test]
async fn test_authorization_user_denied_other_user_data() {
    let provider = MockSecurityProvider::new();
    
    let subject = Subject {
        id: "user-001".to_string(),
        name: "testuser".to_string(),
        roles: vec!["user".to_string()],
        attributes: HashMap::new(),
    };

    let resource = Resource {
        type_: "user_data".to_string(),
        id: "user-002".to_string(), // Different user
        attributes: HashMap::new(),
    };

    let action = Action {
        name: "read".to_string(),
        parameters: HashMap::new(),
    };

    let authorized = provider.authorize(&subject, &resource, &action).await.unwrap();
    assert!(!authorized);
}

#[tokio::test]
async fn test_authorization_guest_denied_access() {
    let provider = MockSecurityProvider::new();
    
    let subject = Subject {
        id: "guest-001".to_string(),
        name: "guest".to_string(),
        roles: vec!["guest".to_string()],
        attributes: HashMap::new(),
    };

    let resource = Resource {
        type_: "service".to_string(),
        id: "test-service".to_string(),
        attributes: HashMap::new(),
    };

    let action = Action {
        name: "read".to_string(),
        parameters: HashMap::new(),
    };

    let authorized = provider.authorize(&subject, &resource, &action).await.unwrap();
    assert!(!authorized);
}

#[tokio::test]
async fn test_audit_logging() {
    let provider = MockSecurityProvider::new();
    
    let event = AuditEvent {
        timestamp: Utc::now(),
        subject: "admin-001".to_string(),
        resource: "test-service".to_string(),
        action: "delete".to_string(),
        result: "success".to_string(),
        metadata: HashMap::from([
            ("ip_address".to_string(), serde_json::json!("192.168.1.100")),
            ("user_agent".to_string(), serde_json::json!("test-client/1.0")),
        ]),
    };

    provider.audit_log(event.clone()).await.unwrap();
    
    let events = provider.get_audit_events().await;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].subject, "admin-001");
    assert_eq!(events[0].action, "delete");
    assert_eq!(events[0].result, "success");
}

#[tokio::test]
async fn test_encryption_decryption() {
    let provider = MockSecurityProvider::new();
    
    let original_data = b"sensitive data that needs encryption";
    
    let encrypted = provider.encrypt_data(original_data).await.unwrap();
    assert_ne!(encrypted, original_data);
    
    let decrypted = provider.decrypt_data(&encrypted).await.unwrap();
    assert_eq!(decrypted, original_data);
}

#[tokio::test]
async fn test_service_request_with_security_context() {
    let mut service = MockService::new("secure-service");
    let config = MockConfig::default();
    
    service.initialize(config).await.unwrap();
    service.start().await.unwrap();

    // Create request with authentication context
    let auth_info = AuthInfo {
        user_id: "user-001".to_string(),
        roles: vec!["user".to_string()],
        attributes: HashMap::from([
            ("department".to_string(), "engineering".to_string()),
            ("clearance".to_string(), "standard".to_string()),
        ]),
    };

    let client_info = ClientInfo {
        ip_address: Some("192.168.1.100".to_string()),
        user_agent: Some("test-client/1.0".to_string()),
        session_id: Some("session-123".to_string()),
        auth_info: Some(auth_info),
    };

    let mut request = ServiceRequest::new("GET", "/secure/data");
    request.client_info = Some(client_info);

    let response = service.handle_request(request).await.unwrap();
    
    // Verify response contains request ID
    assert!(!response.request_id.is_empty());
}

#[tokio::test]
async fn test_role_based_access_control() {
    let provider = MockSecurityProvider::new();
    
    // Test different roles have different access levels
    let roles_and_permissions = vec![
        ("admin", vec!["read", "write", "delete"]),
        ("user", vec!["read"]),
        ("guest", vec![]),
    ];

    for (role, expected_permissions) in roles_and_permissions {
        let subject = Subject {
            id: format!("{}-001", role),
            name: role.to_string(),
            roles: vec![role.to_string()],
            attributes: HashMap::new(),
        };

        let resource = Resource {
            type_: "service".to_string(),
            id: "test-service".to_string(),
            attributes: HashMap::new(),
        };

        for permission in &["read", "write", "delete"] {
            let action = Action {
                name: permission.to_string(),
                parameters: HashMap::new(),
            };

            let authorized = provider.authorize(&subject, &resource, &action).await.unwrap();
            
            if role == "admin" {
                // Admin should have all permissions
                assert!(authorized, "Admin should have {} permission", permission);
            } else {
                let should_have_permission = expected_permissions.contains(permission);
                assert_eq!(
                    authorized, 
                    should_have_permission,
                    "Role {} should {} have {} permission",
                    role,
                    if should_have_permission { "" } else { "not" },
                    permission
                );
            }
        }
    }
}

#[tokio::test]
async fn test_concurrent_authentication() {
    let provider = Arc::new(MockSecurityProvider::new());
    
    let mut handles = vec![];
    
    // Test concurrent authentication requests
    for i in 0..10 {
        let provider_clone = provider.clone();
        let handle = tokio::spawn(async move {
            let credentials = if i % 2 == 0 {
                Credentials::Basic {
                    username: "admin".to_string(),
                    password: "correct_password".to_string(),
                }
            } else {
                Credentials::Bearer {
                    token: "token_user-001".to_string(),
                }
            };
            
            provider_clone.authenticate(&credentials).await
        });
        handles.push(handle);
    }
    
    // Wait for all authentication attempts
    for handle in handles {
        let result = handle.await.unwrap().unwrap();
        assert!(result.success);
    }
}

#[tokio::test]
async fn test_security_audit_trail() {
    let provider = MockSecurityProvider::new();
    
    // Simulate a series of security events
    let events = vec![
        ("user-001", "login", "success"),
        ("user-001", "read_data", "success"),
        ("user-001", "write_data", "denied"),
        ("admin-001", "login", "success"),
        ("admin-001", "delete_service", "success"),
        ("guest-001", "login", "failed"),
    ];

    for (subject, action, result) in events {
        let event = AuditEvent {
            timestamp: Utc::now(),
            subject: subject.to_string(),
            resource: "system".to_string(),
            action: action.to_string(),
            result: result.to_string(),
            metadata: HashMap::new(),
        };
        
        provider.audit_log(event).await.unwrap();
    }

    let audit_events = provider.get_audit_events().await;
    assert_eq!(audit_events.len(), 6);
    
    // Verify audit trail integrity
    let failed_logins: Vec<_> = audit_events
        .iter()
        .filter(|e| e.action == "login" && e.result == "failed")
        .collect();
    assert_eq!(failed_logins.len(), 1);
    
    let successful_admin_actions: Vec<_> = audit_events
        .iter()
        .filter(|e| e.subject == "admin-001" && e.result == "success")
        .collect();
    assert_eq!(successful_admin_actions.len(), 2);
}

#[tokio::test]
async fn test_permission_conditions() {
    // Test time-based permission conditions
    let now = Utc::now();
    let permission = Permission {
        resource_pattern: "/api/*".to_string(),
        actions: vec!["read".to_string()],
        conditions: vec![
            Condition::TimeRange {
                start: now - chrono::Duration::hours(1),
                end: now + chrono::Duration::hours(1),
            },
            Condition::IpRange {
                cidr: "192.168.1.0/24".to_string(),
            },
        ],
    };

    // Verify permission structure
    assert_eq!(permission.actions.len(), 1);
    assert_eq!(permission.conditions.len(), 2);
    
    // Test condition types
    match &permission.conditions[0] {
        Condition::TimeRange { start, end } => {
            assert!(start < &now && end > &now);
        }
        _ => panic!("Expected TimeRange condition"),
    }
    
    match &permission.conditions[1] {
        Condition::IpRange { cidr } => {
            assert_eq!(cidr, "192.168.1.0/24");
        }
        _ => panic!("Expected IpRange condition"),
    }
} 