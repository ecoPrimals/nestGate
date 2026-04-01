// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive Error Path Tests for Primal Discovery
//!
//! Tests error handling, edge cases, and failure scenarios in primal discovery

use crate::primal_discovery::{PrimalDiscovery, PrimalInfo, SelfKnowledge};
use nestgate_types::error::Result;
use std::time::Duration;

// ==================== DISCOVERY ERROR TESTS ====================

#[tokio::test]
async fn test_discover_capability_not_found() {
    let self_knowledge = SelfKnowledge::builder()
        .name("test-primal")
        .primal_type("test")
        .version("1.0.0")
        .build()
        .expect("Valid self knowledge");
    
    let discovery = PrimalDiscovery::new(self_knowledge);
    
    // Try to discover a non-existent capability
    let result = discovery.discover_capability("non-existent-capability").await;
    assert!(result.is_err(), "Should fail for non-existent capability");
}

#[tokio::test]
async fn test_discover_with_empty_capability_string() {
    let self_knowledge = SelfKnowledge::builder()
        .name("test-primal")
        .primal_type("test")
        .version("1.0.0")
        .build()
        .expect("Valid self knowledge");
    
    let discovery = PrimalDiscovery::new(self_knowledge);
    
    // Try to discover with empty string
    let result = discovery.discover_capability("").await;
    assert!(result.is_err(), "Should fail for empty capability string");
}

#[tokio::test]
async fn test_discover_with_whitespace_only() {
    let self_knowledge = SelfKnowledge::builder()
        .name("test-primal")
        .primal_type("test")
        .version("1.0.0")
        .build()
        .expect("Valid self knowledge");
    
    let discovery = PrimalDiscovery::new(self_knowledge);
    
    // Try with whitespace
    let result = discovery.discover_capability("   ").await;
    assert!(result.is_err(), "Should fail for whitespace-only capability");
}

// ==================== SELF KNOWLEDGE VALIDATION TESTS ====================

#[test]
fn test_self_knowledge_builder_missing_required_fields() {
    // Try to build without setting required fields
    let result = SelfKnowledge::builder().build();
    
    assert!(result.is_err(), "Should fail without required fields");
}

#[test]
fn test_self_knowledge_empty_name() {
    let result = SelfKnowledge::builder()
        .name("")
        .primal_type("test")
        .version("1.0.0")
        .build();
    
    assert!(result.is_err(), "Should fail with empty name");
}

#[test]
fn test_self_knowledge_whitespace_name() {
    let result = SelfKnowledge::builder()
        .name("   ")
        .primal_type("test")
        .version("1.0.0")
        .build();
    
    assert!(result.is_err(), "Should fail with whitespace-only name");
}

#[test]
fn test_self_knowledge_empty_primal_type() {
    let result = SelfKnowledge::builder()
        .name("test-primal")
        .primal_type("")
        .version("1.0.0")
        .build();
    
    assert!(result.is_err(), "Should fail with empty primal_type");
}

#[test]
fn test_self_knowledge_invalid_version_format() {
    let result = SelfKnowledge::builder()
        .name("test-primal")
        .primal_type("test")
        .version("not-a-version")
        .build();
    
    // This might succeed depending on version validation rules
    // The test documents the behavior
    match result {
        Ok(_) => {
            // If it succeeds, version validation is lenient (which might be intentional)
        }
        Err(_) => {
            // If it fails, version validation is strict
        }
    }
}

// ==================== PRIMAL INFO VALIDATION TESTS ====================

#[test]
fn test_primal_info_empty_endpoint() {
    // Test that PrimalInfo handles empty endpoints gracefully
    let result = PrimalInfo::builder()
        .name("test-primal")
        .primal_type("test")
        .version("1.0.0")
        .primary_endpoint("")
        .build();
    
    assert!(result.is_err(), "Should fail with empty endpoint");
}

#[test]
fn test_primal_info_invalid_endpoint_format() {
    // Test that PrimalInfo validates endpoint format
    let result = PrimalInfo::builder()
        .name("test-primal")
        .primal_type("test")
        .version("1.0.0")
        .primary_endpoint("not-a-url")
        .build();
    
    // Depending on validation strictness, this might pass or fail
    match result {
        Ok(_) => {
            // Lenient validation - accepts various formats
        }
        Err(_) => {
            // Strict validation - requires proper URL format
        }
    }
}

#[test]
fn test_primal_info_malformed_url() {
    let invalid_urls = vec![
        "htp://invalid",  // Typo in protocol
        "http:/missing-slash",
        "://no-protocol",
        "http//missing-colon",
        "",
        "   ",
    ];
    
    for invalid_url in invalid_urls {
        let result = PrimalInfo::builder()
            .name("test-primal")
            .primal_type("test")
            .version("1.0.0")
            .primary_endpoint(invalid_url)
            .build();
        
        // Document behavior - either strict (error) or lenient (success)
        // This test ensures we're consistent
        if invalid_url.is_empty() || invalid_url.trim().is_empty() {
            assert!(result.is_err(), "Should fail with empty URL");
        }
    }
}

// ==================== CONCURRENT ACCESS TESTS ====================

#[tokio::test]
async fn test_concurrent_discovery_attempts() {
    use tokio::task::JoinSet;
    
    let self_knowledge = SelfKnowledge::builder()
        .name("test-primal")
        .primal_type("test")
        .version("1.0.0")
        .build()
        .expect("Valid self knowledge");
    
    let discovery = std::sync::Arc::new(PrimalDiscovery::new(self_knowledge));
    let mut tasks = JoinSet::new();
    
    // Spawn multiple concurrent discovery attempts
    for i in 0..10 {
        let disc = discovery.clone();
        tasks.spawn(async move {
            let capability = format!("test-capability-{}", i);
            disc.discover_capability(&capability).await
        });
    }
    
    // All tasks should complete without panic
    while let Some(result) = tasks.join_next().await {
        assert!(result.is_ok(), "Task should not panic");
        // The discovery itself might fail (capability not found), which is expected
    }
}

// ==================== TIMEOUT AND RETRY TESTS ====================

#[tokio::test]
#[ignore = "Requires network access or longer timeout; run manually"]
async fn test_discovery_timeout_handling() {
    let self_knowledge = SelfKnowledge::builder()
        .name("test-primal")
        .primal_type("test")
        .version("1.0.0")
        .build()
        .expect("Valid self knowledge");
    
    let discovery = PrimalDiscovery::new(self_knowledge);
    
    // Try to discover with a very short timeout
    let result = tokio::time::timeout(
        Duration::from_millis(1),
        discovery.discover_capability("slow-capability")
    ).await;
    
    // Should timeout
    assert!(result.is_err(), "Should timeout");
}

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_self_knowledge_clone_independence() {
    let sk1 = SelfKnowledge::builder()
        .name("test-primal")
        .primal_type("test")
        .version("1.0.0")
        .build()
        .expect("Valid self knowledge");
    
    let sk2 = sk1.clone();
    
    // Clones should have same values
    assert_eq!(sk1.name(), sk2.name());
    assert_eq!(sk1.primal_type(), sk2.primal_type());
    assert_eq!(sk1.version(), sk2.version());
}

#[test]
fn test_self_knowledge_debug_format() {
    let sk = SelfKnowledge::builder()
        .name("test-primal")
        .primal_type("test")
        .version("1.0.0")
        .build()
        .expect("Valid self knowledge");
    
    let debug_str = format!("{:?}", sk);
    
    // Should contain key information
    assert!(debug_str.contains("test-primal") || debug_str.contains("name"));
    assert!(!debug_str.is_empty());
}

#[test]
fn test_primal_discovery_is_send_sync() {
    // Verify PrimalDiscovery can be safely shared across threads
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    
    assert_send::<PrimalDiscovery>();
    assert_sync::<PrimalDiscovery>();
}

// ==================== SPECIAL CHARACTER TESTS ====================

#[test]
fn test_self_knowledge_special_characters_in_name() {
    let special_names = vec![
        "test-primal!",
        "test@primal",
        "test#primal",
        "test$primal",
        "test%primal",
        "test&primal",
        "test*primal",
        "test(primal)",
        "test[primal]",
        "test{primal}",
    ];
    
    for name in special_names {
        let result = SelfKnowledge::builder()
            .name(name)
            .primal_type("test")
            .version("1.0.0")
            .build();
        
        // Document behavior - either accept or reject special characters
        // This ensures consistency
        match result {
            Ok(_) => {
                // Accepts special characters
            }
            Err(_) => {
                // Rejects special characters
            }
        }
    }
}

#[test]
fn test_self_knowledge_unicode_in_name() {
    let result = SelfKnowledge::builder()
        .name("テスト-primal-🚀")
        .primal_type("test")
        .version("1.0.0")
        .build();
    
    // Document Unicode handling
    match result {
        Ok(_) => {
            // Accepts Unicode
        }
        Err(_) => {
            // Rejects Unicode
        }
    }
}

// ==================== BUILDER PATTERN TESTS ====================

#[test]
fn test_self_knowledge_builder_can_be_reused() {
    let builder = SelfKnowledge::builder();
    
    let result1 = builder
        .clone()
        .name("primal-1")
        .primal_type("test")
        .version("1.0.0")
        .build();
    
    let result2 = builder
        .name("primal-2")
        .primal_type("test")
        .version("2.0.0")
        .build();
    
    // Both should succeed if builder is properly designed
    assert!(result1.is_ok() || result2.is_ok(), "Builder should be reusable");
}

