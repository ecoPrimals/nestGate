// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use std::collections::HashMap;
use std::time::SystemTime;

use super::{
    AuthToken, ComputeResources, Credentials, SecurityDecision, ServiceStatus, Signature,
    ZeroCostSecurityProvider, ZeroCostUniversalSecurityWrapper,
};

// Mock provider for testing
struct MockSecurityProvider;

impl ZeroCostSecurityProvider for MockSecurityProvider {
    type Error = String;

    async fn authenticate(
        &self,
        _credentials: &Credentials,
    ) -> std::result::Result<AuthToken, Self::Error> {
        Ok(AuthToken {
            token: "test_token".to_string(),
            expires_at: SystemTime::now(),
            permissions: vec!["read".to_string()],
        })
    }

    async fn encrypt(
        &self,
        data: &[u8],
        _algorithm: &str,
    ) -> std::result::Result<Vec<u8>, Self::Error> {
        Ok(data.to_vec())
    }

    async fn decrypt(
        &self,
        encrypted: &[u8],
        _algorithm: &str,
    ) -> std::result::Result<Vec<u8>, Self::Error> {
        Ok(encrypted.to_vec())
    }

    async fn sign_data(&self, _data: &[u8]) -> std::result::Result<Signature, Self::Error> {
        Ok(Signature {
            algorithm: "test".to_string(),
            signature: vec![1, 2, 3],
            public_key: None,
        })
    }

    async fn verify_signature(
        &self,
        _data: &[u8],
        _signature: &Signature,
    ) -> std::result::Result<bool, Self::Error> {
        Ok(true)
    }

    async fn health_check(&self) -> std::result::Result<bool, Self::Error> {
        Ok(true)
    }
}

#[test]
fn test_zero_cost_security_wrapper_creation() {
    let provider = MockSecurityProvider;
    let wrapper: ZeroCostUniversalSecurityWrapper<MockSecurityProvider, 1000> =
        ZeroCostUniversalSecurityWrapper::new(
            "test_provider".to_string(),
            "http://localhost:8080".to_string(),
            vec!["encryption".to_string()],
            provider,
        );

    assert_eq!(wrapper.provider_name(), "test_provider");
    assert_eq!(wrapper.endpoint(), "http://localhost:8080");
    assert_eq!(wrapper.capabilities().len(), 1);
}

#[tokio::test]
async fn test_zero_cost_wrapper_authenticate() {
    let provider = MockSecurityProvider;
    let wrapper: ZeroCostUniversalSecurityWrapper<MockSecurityProvider, 1000> =
        ZeroCostUniversalSecurityWrapper::new(
            "test".to_string(),
            "http://test".to_string(),
            vec![],
            provider,
        );

    let creds = Credentials {
        username: "user".to_string(),
        password: "pass".to_string(),
        additional_data: HashMap::new(),
    };

    let result = wrapper.authenticate(&creds).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().token, "test_token");
}

#[tokio::test]
async fn test_zero_cost_wrapper_encrypt() {
    let provider = MockSecurityProvider;
    let wrapper: ZeroCostUniversalSecurityWrapper<MockSecurityProvider, 1000> =
        ZeroCostUniversalSecurityWrapper::new(
            "test".to_string(),
            "http://test".to_string(),
            vec![],
            provider,
        );

    let data = b"test data";
    let result = wrapper.encrypt(data, "aes256").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_zero_cost_wrapper_decrypt() {
    let provider = MockSecurityProvider;
    let wrapper: ZeroCostUniversalSecurityWrapper<MockSecurityProvider, 1000> =
        ZeroCostUniversalSecurityWrapper::new(
            "test".to_string(),
            "http://test".to_string(),
            vec![],
            provider,
        );

    let encrypted = b"encrypted data";
    let result = wrapper.decrypt(encrypted, "aes256").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_zero_cost_wrapper_batch_authenticate() {
    let provider = MockSecurityProvider;
    let wrapper: ZeroCostUniversalSecurityWrapper<MockSecurityProvider, 1000> =
        ZeroCostUniversalSecurityWrapper::new(
            "test".to_string(),
            "http://test".to_string(),
            vec![],
            provider,
        );

    let creds_list = vec![
        Credentials {
            username: "user1".to_string(),
            password: "pass1".to_string(),
            additional_data: HashMap::new(),
        },
        Credentials {
            username: "user2".to_string(),
            password: "pass2".to_string(),
            additional_data: HashMap::new(),
        },
    ];

    let result = wrapper.batch_authenticate(&creds_list).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_service_status_default() {
    let status = ServiceStatus::default();
    assert!(!status.running);
    assert_eq!(status.replicas, 0);
    assert_eq!(status.health, "unknown");
}

#[test]
fn test_auth_token_creation() {
    let token = AuthToken {
        token: "abc123".to_string(),
        expires_at: SystemTime::now(),
        permissions: vec!["read".to_string(), "write".to_string()],
    };

    assert_eq!(token.token, "abc123");
    assert_eq!(token.permissions.len(), 2);
}

#[test]
fn test_credentials_creation() {
    let creds = Credentials {
        username: "testuser".to_string(),
        password: "testpass".to_string(),
        additional_data: HashMap::new(),
    };

    assert_eq!(creds.username, "testuser");
    assert_eq!(creds.password, "testpass");
}

#[test]
fn test_signature_creation() {
    let sig = Signature {
        algorithm: "SHA256".to_string(),
        signature: vec![1, 2, 3, 4],
        public_key: Some(vec![5, 6, 7, 8]),
    };

    assert_eq!(sig.algorithm, "SHA256");
    assert_eq!(sig.signature.len(), 4);
    assert!(sig.public_key.is_some());
}

#[test]
fn test_security_decision_allow() {
    let decision = SecurityDecision::Allow {
        reason: "Valid credentials".to_string(),
        enhanced_by_security_provider: true,
    };

    match decision {
        SecurityDecision::Allow {
            enhanced_by_security_provider,
            ..
        } => {
            assert!(enhanced_by_security_provider);
        }
        _ => panic!("Expected Allow variant"),
    }
}

#[test]
fn test_security_decision_deny() {
    let decision = SecurityDecision::Deny {
        reason: "Invalid credentials".to_string(),
        remediation: Some("Contact admin".to_string()),
    };

    match decision {
        SecurityDecision::Deny { remediation, .. } => {
            assert!(remediation.is_some());
        }
        _ => panic!("Expected Deny variant"),
    }
}

#[test]
fn test_compute_resources_creation() {
    let resources = ComputeResources {
        available_cpu: 4.0,
        available_memory_gb: 16.0,
        active_tasks: 10,
        max_tasks: 100,
    };

    assert_eq!(resources.available_cpu, 4.0);
    assert_eq!(resources.available_memory_gb, 16.0);
}
