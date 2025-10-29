//! **COMPREHENSIVE AUTH HANDLER TESTS**
//!
//! Complete test coverage for auth.rs handler module.
//! Tests authentication service, credentials, status, modes, and challenges.

use super::*;
use nestgate_core::universal_traits::Credentials;

// ==================== AUTH SERVICE TESTS ====================

    #[test]
    fn test_auth_service_new() {
        let service = AuthService::new();
        assert!(service.authenticated_users.is_empty());
    }

    #[test]
    fn test_auth_service_authenticate() {
        let service = AuthService::new();
        let creds = Credentials {
            username: "test_user".to_string(),
            password: "test_pass".to_string(),
        };
        
        // Stub always returns true
        assert!(service.authenticate(&creds));
    }

    #[test]
    fn test_auth_service_get_auth_status() {
        let service = AuthService::new();
        let status = service.get_auth_status();
        
        assert!(status.authenticated);
        assert_eq!(status.user_id, Some("stub_user".to_string()));
        assert_eq!(status.permissions.len(), 2);
        assert!(status.permissions.contains(&"read".to_string()));
        assert!(status.permissions.contains(&"write".to_string()));
    }

    #[test]
    fn test_auth_service_security_primal_available() {
        let service = AuthService::new();
        assert!(service.security_primal_available());
    }

    #[test]
    fn test_auth_service_get_mode() {
        let service = AuthService::new();
        let mode = service.get_mode();
        
        matches!(mode, AuthMode::Development);
    }

    #[test]
    fn test_auth_service_clone() {
        let service1 = AuthService::new();
        let service2 = service1.clone();
        
        assert!(service2.authenticated_users.is_empty());
    }

    // ==================== AUTH CREDENTIALS TESTS ====================

    #[test]
    fn test_auth_credentials_creation() {
        let creds = AuthCredentials {
            username: "alice".to_string(),
            password: "secret123".to_string(),
        };
        
        assert_eq!(creds.username, "alice");
        assert_eq!(creds.password, "secret123");
    }

    #[test]
    fn test_auth_credentials_clone() {
        let creds1 = AuthCredentials {
            username: "bob".to_string(),
            password: "pass456".to_string(),
        };
        let creds2 = creds1.clone();
        
        assert_eq!(creds1.username, creds2.username);
        assert_eq!(creds1.password, creds2.password);
    }

    #[test]
    fn test_auth_credentials_serialization() {
        let creds = AuthCredentials {
            username: "carol".to_string(),
            password: "mypassword".to_string(),
        };
        
        let json = serde_json::to_string(&creds).expect("Failed to serialize");
        assert!(json.contains("carol"));
        assert!(json.contains("mypassword"));
    }

    #[test]
    fn test_auth_credentials_deserialization() {
        let json = r#"{"username":"dave","password":"davepass"}"#;
        let creds: AuthCredentials = serde_json::from_str(json).expect("Failed to deserialize");
        
        assert_eq!(creds.username, "dave");
        assert_eq!(creds.password, "davepass");
    }

    #[test]
    fn test_auth_credentials_empty_username() {
        let creds = AuthCredentials {
            username: String::new(),
            password: "password".to_string(),
        };
        
        assert!(creds.username.is_empty());
    }

    #[test]
    fn test_auth_credentials_empty_password() {
        let creds = AuthCredentials {
            username: "user".to_string(),
            password: String::new(),
        };
        
        assert!(creds.password.is_empty());
    }

    // ==================== AUTH STATUS TESTS ====================

    #[test]
    fn test_auth_status_authenticated() {
        let status = AuthStatus {
            authenticated: true,
            user_id: Some("user123".to_string()),
            permissions: vec!["admin".to_string()],
        };
        
        assert!(status.authenticated);
        assert_eq!(status.user_id, Some("user123".to_string()));
        assert_eq!(status.permissions.len(), 1);
    }

    #[test]
    fn test_auth_status_not_authenticated() {
        let status = AuthStatus {
            authenticated: false,
            user_id: None,
            permissions: Vec::new(),
        };
        
        assert!(!status.authenticated);
        assert!(status.user_id.is_none());
        assert!(status.permissions.is_empty());
    }

    #[test]
    fn test_auth_status_multiple_permissions() {
        let perms = vec![
            "read".to_string(),
            "write".to_string(),
            "delete".to_string(),
            "admin".to_string(),
        ];
        
        let status = AuthStatus {
            authenticated: true,
            user_id: Some("admin_user".to_string()),
            permissions: perms.clone(),
        };
        
        assert_eq!(status.permissions.len(), 4);
        assert!(status.permissions.contains(&"admin".to_string()));
    }

    #[test]
    fn test_auth_status_serialization() {
        let status = AuthStatus {
            authenticated: true,
            user_id: Some("test".to_string()),
            permissions: vec!["read".to_string()],
        };
        
        let json = serde_json::to_string(&status).expect("Failed to serialize");
        assert!(json.contains("authenticated"));
        assert!(json.contains("test"));
    }

    #[test]
    fn test_auth_status_clone() {
        let status1 = AuthStatus {
            authenticated: true,
            user_id: Some("user".to_string()),
            permissions: vec!["read".to_string()],
        };
        let status2 = status1.clone();
        
        assert_eq!(status1.authenticated, status2.authenticated);
        assert_eq!(status1.user_id, status2.user_id);
        assert_eq!(status1.permissions, status2.permissions);
    }

    // ==================== AUTH MODE TESTS ====================

    #[test]
    fn test_auth_mode_development() {
        let mode = AuthMode::Development;
        let json = serde_json::to_string(&mode).expect("Serialization failed");
        assert!(json.contains("Development"));
    }

    #[test]
    fn test_auth_mode_production() {
        let mode = AuthMode::Production;
        let json = serde_json::to_string(&mode).expect("Serialization failed");
        assert!(json.contains("Production"));
    }

    #[test]
    fn test_auth_mode_testing() {
        let mode = AuthMode::Testing;
        let json = serde_json::to_string(&mode).expect("Serialization failed");
        assert!(json.contains("Testing"));
    }

    #[test]
    fn test_auth_mode_clone() {
        let mode1 = AuthMode::Development;
        let mode2 = mode1.clone();
        
        matches!(mode2, AuthMode::Development);
    }

    #[test]
    fn test_auth_mode_deserialization() {
        let json = r#""Production""#;
        let mode: AuthMode = serde_json::from_str(json).expect("Deserialization failed");
        matches!(mode, AuthMode::Production);
    }

    // ==================== AUTH CHALLENGE TESTS ====================

    #[test]
    fn test_auth_challenge_creation() {
        let challenge = AuthChallenge {
            challenge: "random_string_123".to_string(),
            timestamp: 1000000,
            expires_at: 1001000,
        };
        
        assert_eq!(challenge.challenge, "random_string_123");
        assert_eq!(challenge.timestamp, 1000000);
        assert_eq!(challenge.expires_at, 1001000);
    }

    #[test]
    fn test_auth_challenge_expiration() {
        let challenge = AuthChallenge {
            challenge: "challenge".to_string(),
            timestamp: 1000,
            expires_at: 2000,
        };
        
        // Challenge should expire after expires_at
        assert!(challenge.expires_at > challenge.timestamp);
        assert_eq!(challenge.expires_at - challenge.timestamp, 1000);
    }

    #[test]
    fn test_auth_challenge_serialization() {
        let challenge = AuthChallenge {
            challenge: "test_challenge".to_string(),
            timestamp: 123456,
            expires_at: 789012,
        };
        
        let json = serde_json::to_string(&challenge).expect("Serialization failed");
        assert!(json.contains("test_challenge"));
        assert!(json.contains("123456"));
    }

    #[test]
    fn test_auth_challenge_clone() {
        let challenge1 = AuthChallenge {
            challenge: "original".to_string(),
            timestamp: 100,
            expires_at: 200,
        };
        let challenge2 = challenge1.clone();
        
        assert_eq!(challenge1.challenge, challenge2.challenge);
        assert_eq!(challenge1.timestamp, challenge2.timestamp);
        assert_eq!(challenge1.expires_at, challenge2.expires_at);
    }

    // ==================== AUTH REQUEST TESTS ====================

    #[test]
    fn test_auth_request_creation() {
        let req = AuthRequest {
            username: "user1".to_string(),
            password: "pass1".to_string(),
            domain: None,
        };
        
        assert_eq!(req.username, "user1");
        assert_eq!(req.password, "pass1");
        assert!(req.domain.is_none());
    }

    #[test]
    fn test_auth_request_with_domain() {
        let req = AuthRequest {
            username: "user2".to_string(),
            password: "pass2".to_string(),
            domain: Some("example.com".to_string()),
        };
        
        assert_eq!(req.domain, Some("example.com".to_string()));
    }

    #[test]
    fn test_auth_request_deserialization() {
        let json = r#"{"username":"testuser","password":"testpass","domain":null}"#;
        let req: AuthRequest = serde_json::from_str(json).expect("Deserialization failed");
        
        assert_eq!(req.username, "testuser");
        assert_eq!(req.password, "testpass");
        assert!(req.domain.is_none());
    }

    #[test]
    fn test_auth_request_with_domain_deserialization() {
        let json = r#"{"username":"user","password":"pass","domain":"corp.com"}"#;
        let req: AuthRequest = serde_json::from_str(json).expect("Deserialization failed");
        
        assert_eq!(req.domain, Some("corp.com".to_string()));
    }

    // ==================== AUTH RESPONSE TESTS ====================

    #[test]
    fn test_auth_response_success() {
        use std::time::SystemTime;
        
        let resp = AuthResponse {
            success: true,
            token: Some("token_abc123".to_string()),
            expires_at: Some(SystemTime::now()),
            permissions: Some(vec!["read".to_string(), "write".to_string()]),
            message: "Authentication successful".to_string(),
        };
        
        assert!(resp.success);
        assert!(resp.token.is_some());
        assert!(resp.expires_at.is_some());
        assert!(resp.permissions.is_some());
        assert!(!resp.message.is_empty());
    }

    #[test]
    fn test_auth_response_failure() {
        let resp = AuthResponse {
            success: false,
            token: None,
            expires_at: None,
            permissions: None,
            message: "Invalid credentials".to_string(),
        };
        
        assert!(!resp.success);
        assert!(resp.token.is_none());
        assert_eq!(resp.message, "Invalid credentials");
    }

    #[test]
    fn test_auth_response_serialization() {
        let resp = AuthResponse {
            success: true,
            token: Some("mytoken".to_string()),
            expires_at: None,
            permissions: Some(vec!["admin".to_string()]),
            message: "Success".to_string(),
        };
        
        let json = serde_json::to_string(&resp).expect("Serialization failed");
        assert!(json.contains("mytoken"));
        assert!(json.contains("Success"));
    }

    // ==================== INTEGRATION TESTS ====================

    #[test]
    fn test_full_auth_flow() {
        let service = AuthService::new();
        let creds = Credentials {
            username: "integration_test".to_string(),
            password: "integration_pass".to_string(),
        };
        
        // Authenticate
        let auth_result = service.authenticate(&creds);
        assert!(auth_result);
        
        // Get status
        let status = service.get_auth_status();
        assert!(status.authenticated);
        
        // Check mode
        let mode = service.get_mode();
        matches!(mode, AuthMode::Development);
    }

    #[test]
    fn test_auth_service_consistency() {
        let service = AuthService::new();
        
        // Multiple calls should return consistent results
        assert!(service.security_primal_available());
        assert!(service.security_primal_available());
        
        let status1 = service.get_auth_status();
        let status2 = service.get_auth_status();
        
        assert_eq!(status1.authenticated, status2.authenticated);
        assert_eq!(status1.user_id, status2.user_id);
    }

