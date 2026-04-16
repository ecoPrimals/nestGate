// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::collections::HashMap;
use std::time::SystemTime;

use super::{AuthToken, ComputeResources, Credentials, SecurityDecision, ServiceStatus, Signature};

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
