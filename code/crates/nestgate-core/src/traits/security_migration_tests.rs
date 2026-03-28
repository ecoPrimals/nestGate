// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![allow(deprecated)]

use super::*;
use crate::traits::canonical_provider_unification::{AuthToken, SecurityProvider};
use crate::universal_traits::{Credentials, SecurityPrimalProvider, Signature};
use crate::zero_cost_security_provider::{
    traits::ZeroCostSecurityProvider,
    types::{ZeroCostAuthToken, ZeroCostCredentials, ZeroCostSignature},
};
use std::time::SystemTime;

// Mock SecurityPrimalProvider for adapter tests
struct MockSecurityPrimalProvider {
    key_id: String,
}

#[allow(deprecated)]
impl SecurityPrimalProvider for MockSecurityPrimalProvider {
    async fn authenticate(
        &self,
        credentials: &Credentials,
    ) -> Result<crate::universal_traits::AuthToken> {
        if credentials.username == "test" {
            Ok(crate::universal_traits::AuthToken {
                token: "token-123".to_string(),
                expires_at: SystemTime::now(),
                permissions: vec!["read".to_string()],
            })
        } else {
            Err(crate::NestGateError::security("Invalid credentials"))
        }
    }

    async fn encrypt(&self, data: &[u8], _alg: &str) -> Result<Vec<u8>> {
        Ok(data.to_vec())
    }

    async fn decrypt(&self, data: &[u8], _alg: &str) -> Result<Vec<u8>> {
        Ok(data.to_vec())
    }

    async fn sign_data(&self, data: &[u8]) -> Result<Signature> {
        Ok(Signature {
            algorithm: "default".to_string(),
            signature: data.to_vec(),
            key_id: Some(self.key_id.clone()),
        })
    }

    async fn verify_signature(&self, data: &[u8], sig: &Signature) -> Result<bool> {
        Ok(sig.signature == data)
    }

    async fn get_key_id(&self) -> Result<String> {
        Ok(self.key_id.clone())
    }

    async fn hash_data(&self, data: &[u8], _alg: &str) -> Result<Vec<u8>> {
        Ok(data.to_vec())
    }

    async fn generate_random(&self, length: usize) -> Result<Vec<u8>> {
        Ok(vec![0u8; length])
    }

    async fn derive_key(&self, _pwd: &str, _salt: &[u8], _iter: u32) -> Result<Vec<u8>> {
        Ok(vec![0u8; 32])
    }

    async fn create_session(&self, _uid: &str, _perms: Vec<String>) -> Result<String> {
        Ok("session-1".to_string())
    }

    async fn validate_session(
        &self,
        _token: &str,
    ) -> Result<crate::universal_traits::SecurityDecision> {
        Ok(crate::universal_traits::SecurityDecision::Allow)
    }

    async fn evaluate_boundary_access(
        &self,
        _src: &str,
        _dst: &str,
    ) -> Result<crate::universal_traits::SecurityDecision> {
        Ok(crate::universal_traits::SecurityDecision::Allow)
    }
}

struct MockZeroCostProvider {
    config: String,
}

#[allow(deprecated)]
impl ZeroCostSecurityProvider for MockZeroCostProvider {
    type Config = String;
    type Health = bool;
    type Metrics = u64;

    async fn authenticate(&self, _creds: &ZeroCostCredentials) -> Result<ZeroCostAuthToken> {
        Ok(ZeroCostAuthToken::new(
            "zc-token".to_string(),
            "user".to_string(),
            vec!["read".to_string(), "write".to_string()],
            std::time::Duration::from_secs(3600),
        ))
    }

    async fn validate_token(&self, token: &str) -> Result<bool> {
        Ok(!token.is_empty())
    }

    async fn refresh_token(&self, _token: &str) -> Result<ZeroCostAuthToken> {
        Ok(ZeroCostAuthToken::new(
            "refreshed".to_string(),
            "user".to_string(),
            vec!["read".to_string()],
            std::time::Duration::from_secs(3600),
        ))
    }

    async fn revoke_token(&self, _token: &str) -> Result<()> {
        Ok(())
    }

    async fn encrypt(&self, data: &[u8], _alg: &str) -> Result<Vec<u8>> {
        Ok(data.to_vec())
    }

    async fn decrypt(&self, data: &[u8], _alg: &str) -> Result<Vec<u8>> {
        Ok(data.to_vec())
    }

    async fn sign_data(&self, _data: &[u8]) -> Result<ZeroCostSignature> {
        Ok(ZeroCostSignature::new(
            "ECDSA".to_string(),
            "sig".to_string(),
            "key-1".to_string(),
        ))
    }

    async fn verify_signature(&self, _data: &[u8], _sig: &ZeroCostSignature) -> Result<bool> {
        Ok(true)
    }

    fn get_key_id(&self) -> String {
        "zc-key".to_string()
    }

    fn supported_algorithms(&self) -> Vec<String> {
        vec!["AES-256".to_string(), "ECDSA".to_string()]
    }

    fn supports_algorithm(&self, alg: &str) -> bool {
        self.supported_algorithms().contains(&alg.to_string())
    }

    fn health_check(&self) -> impl std::future::Future<Output = bool> + Send {
        async { true }
    }

    fn get_metrics(&self) -> impl std::future::Future<Output = u64> + Send {
        async { 0 }
    }

    fn current_config(&self) -> &String {
        &self.config
    }

    async fn update_config(&mut self, config: String) -> Result<()> {
        self.config = config;
        Ok(())
    }
}

#[tokio::test]
async fn test_security_primal_adapter_authenticate() {
    let provider = MockSecurityPrimalProvider {
        key_id: "key-1".to_string(),
    };
    let adapter = SecurityPrimalAdapter(provider);

    let creds = b"test";
    let token = adapter
        .authenticate(creds)
        .await
        .expect("test: primal adapter authenticate");
    assert_eq!(token.token, "token-123");
}

#[tokio::test]
async fn test_security_primal_adapter_validate_token() {
    let provider = MockSecurityPrimalProvider {
        key_id: "key-1".to_string(),
    };
    let adapter = SecurityPrimalAdapter(provider);
    let token = AuthToken {
        token: "t".to_string(),
        expires_at: SystemTime::now(),
        permissions: vec![],
    };
    let ok = adapter
        .validate_token(&token)
        .await
        .expect("test: primal adapter validate_token");
    assert!(ok);
}

#[tokio::test]
async fn test_security_primal_adapter_refresh_token() {
    let provider = MockSecurityPrimalProvider {
        key_id: "key-1".to_string(),
    };
    let adapter = SecurityPrimalAdapter(provider);
    let token = AuthToken {
        token: "t".to_string(),
        expires_at: SystemTime::now(),
        permissions: vec![],
    };
    let refreshed = adapter
        .refresh_token(&token)
        .await
        .expect("test: primal adapter refresh_token");
    assert_eq!(refreshed.token, "t");
}

#[tokio::test]
async fn test_security_primal_adapter_encrypt_decrypt() {
    let provider = MockSecurityPrimalProvider {
        key_id: "key-1".to_string(),
    };
    let adapter = SecurityPrimalAdapter(provider);
    let enc = adapter
        .encrypt(b"data", "AES256")
        .await
        .expect("test: primal adapter encrypt");
    assert_eq!(enc, b"data");
    let dec = adapter
        .decrypt(b"data")
        .await
        .expect("test: primal adapter decrypt");
    assert_eq!(dec, Some(b"data".to_vec()));
}

#[tokio::test]
async fn test_security_primal_adapter_get_key_id() {
    let provider = MockSecurityPrimalProvider {
        key_id: "my-key".to_string(),
    };
    let adapter = SecurityPrimalAdapter(provider);
    let id = adapter
        .get_key_id()
        .await
        .expect("test: primal adapter get_key_id");
    assert_eq!(id, "my-key");
}

#[tokio::test]
async fn test_security_primal_adapter_supported_algorithms() {
    let provider = MockSecurityPrimalProvider {
        key_id: "k".to_string(),
    };
    let adapter = SecurityPrimalAdapter(provider);
    let algs = adapter
        .supported_algorithms()
        .await
        .expect("test: primal adapter supported_algorithms");
    assert!(!algs.is_empty());
}

#[tokio::test]
async fn test_security_primal_adapter_canonical_provider_provide() {
    let provider = MockSecurityPrimalProvider {
        key_id: "k".to_string(),
    };
    let adapter: SecurityPrimalAdapter<_> = SecurityPrimalAdapter(provider);
    let result = adapter.provide().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_zero_cost_adapter_authenticate() {
    let provider = MockZeroCostProvider {
        config: "cfg".to_string(),
    };
    let adapter = ZeroCostSecurityAdapter(provider);
    let creds = b"user:pass";
    let token = adapter
        .authenticate(creds)
        .await
        .expect("test: zero-cost adapter authenticate");
    assert_eq!(token.token, "zc-token");
}

#[tokio::test]
async fn test_zero_cost_adapter_validate_token() {
    let provider = MockZeroCostProvider {
        config: "cfg".to_string(),
    };
    let adapter = ZeroCostSecurityAdapter(provider);
    let token = AuthToken {
        token: "valid".to_string(),
        expires_at: SystemTime::now(),
        permissions: vec![],
    };
    let ok = adapter
        .validate_token(&token)
        .await
        .expect("test: zero-cost adapter validate_token");
    assert!(ok);
}

#[tokio::test]
async fn test_zero_cost_adapter_encrypt_decrypt() {
    let provider = MockZeroCostProvider {
        config: "cfg".to_string(),
    };
    let adapter = ZeroCostSecurityAdapter(provider);
    let enc = adapter
        .encrypt(b"secret", "AES")
        .await
        .expect("test: zero-cost adapter encrypt");
    assert_eq!(enc, b"secret");
    let dec = adapter
        .decrypt(b"secret")
        .await
        .expect("test: zero-cost adapter decrypt");
    assert_eq!(dec, Some(b"secret".to_vec()));
}

#[tokio::test]
async fn test_zero_cost_adapter_get_key_id() {
    let provider = MockZeroCostProvider {
        config: "cfg".to_string(),
    };
    let adapter = ZeroCostSecurityAdapter(provider);
    let id = adapter
        .get_key_id()
        .await
        .expect("test: zero-cost adapter get_key_id");
    assert_eq!(id, "zc-key");
}

#[tokio::test]
async fn test_zero_cost_adapter_supported_algorithms() {
    let provider = MockZeroCostProvider {
        config: "cfg".to_string(),
    };
    let adapter = ZeroCostSecurityAdapter(provider);
    let algs = adapter
        .supported_algorithms()
        .await
        .expect("test: zero-cost adapter supported_algorithms");
    assert_eq!(algs.len(), 2);
}

#[tokio::test]
async fn test_zero_cost_adapter_hash_data() {
    let provider = MockZeroCostProvider {
        config: "cfg".to_string(),
    };
    let adapter = ZeroCostSecurityAdapter(provider);
    let hash = adapter
        .hash_data(b"data", "SHA256")
        .await
        .expect("test: zero-cost adapter hash_data");
    assert!(!hash.is_empty());
}

#[tokio::test]
async fn test_zero_cost_adapter_generate_random() {
    let provider = MockZeroCostProvider {
        config: "cfg".to_string(),
    };
    let adapter = ZeroCostSecurityAdapter(provider);
    let bytes = adapter
        .generate_random(32)
        .await
        .expect("test: zero-cost adapter generate_random");
    assert_eq!(bytes.len(), 32);
}

#[tokio::test]
async fn test_security_primal_adapter_authenticate_rejects_invalid_user() {
    let provider = MockSecurityPrimalProvider {
        key_id: "key-1".to_string(),
    };
    let adapter = SecurityPrimalAdapter(provider);
    let err = adapter
        .authenticate(b"not-test")
        .await
        .expect_err("test: invalid user should fail");
    assert!(err.to_string().contains("Invalid") || err.to_string().contains("credentials"));
}

#[tokio::test]
async fn test_security_primal_adapter_verify_signature_mismatch_returns_none() {
    let provider = MockSecurityPrimalProvider {
        key_id: "key-1".to_string(),
    };
    let adapter = SecurityPrimalAdapter(provider);
    let out = adapter
        .verify(b"data", b"other")
        .await
        .expect("test: primal adapter verify");
    assert!(out.is_none());
}

#[tokio::test]
async fn test_security_primal_adapter_hash_and_random_delegate() {
    let provider = MockSecurityPrimalProvider {
        key_id: "key-1".to_string(),
    };
    let adapter = SecurityPrimalAdapter(provider);
    let h = adapter
        .hash_data(b"abc", "x")
        .await
        .expect("test: primal adapter hash_data");
    assert_eq!(h, b"abc");
    let r = adapter
        .generate_random(4)
        .await
        .expect("test: primal adapter generate_random");
    assert_eq!(r, vec![0u8; 4]);
}

#[tokio::test]
async fn test_zero_cost_adapter_credentials_without_colon_uses_whole_string_as_username() {
    let provider = MockZeroCostProvider {
        config: "cfg".to_string(),
    };
    let adapter = ZeroCostSecurityAdapter(provider);
    let token = adapter
        .authenticate(b"onlyuser")
        .await
        .expect("test: zero-cost authenticate single segment");
    assert_eq!(token.token, "zc-token");
}

#[tokio::test]
async fn test_zero_cost_adapter_revoke_token_delegates() {
    let provider = MockZeroCostProvider {
        config: "cfg".to_string(),
    };
    let adapter = ZeroCostSecurityAdapter(provider);
    let token = AuthToken {
        token: "t".to_string(),
        expires_at: SystemTime::now(),
        permissions: vec![],
    };
    adapter
        .revoke_token(&token)
        .await
        .expect("test: zero-cost revoke_token");
}
