//! Universal Cryptographic Spore Integration Demo
//!
//! This demo shows how the Universal Cryptographic Spore system works
//! and how it can be easily integrated into any primal in the ecosystem.
//!
//! Key Features Demonstrated:
//! - Autonomous spore operation without BearDog
//! - BearDog integration for enhanced capabilities
//! - Individual vs corporate user detection
//! - Frictionless access for individuals
//! - License negotiation for corporations
//! - Autonomous spore evolution

use crate::universal_spore::{
    UniversalCryptographicSpore, OperationRequest, UserContext, AuthorizationDecision
};
use crate::crypto_locks::{create_spore_guardian, SporeStatus};
use crate::{Result, NestGateError};
use std::collections::HashMap;
use std::time::SystemTime;
use tracing::{info, warn};

/// Demo: Individual User Experience (Frictionless)
pub async fn demo_individual_user_experience() -> Result<()> {
    println!("\n🧑‍💻 DEMO: Individual User Experience");
    println!("=====================================");
    
    // Create spore for any ecosystem participant (e.g., "orchestration-service", "compute-service", "ai-service")
    let mut spore = UniversalCryptographicSpore::new_for_primal("demo_primal")?;
    
    // Individual user context
    let user_context = UserContext {
        user_id: Some("alice_individual".to_string()),
        session_id: "session_123".to_string(),
        ip_address: "192.168.1.100".to_string(),
        user_agent: Some("Individual Developer Environment".to_string()),
        environment_info: {
            let mut env = HashMap::new();
            env.insert("user_type".to_string(), "individual".to_string());
            env.insert("entropy_source".to_string(), "human_webcam".to_string());
            env
        },
    };
    
    // Create operation request
    let operation = OperationRequest {
        operation_type: "data_access".to_string(),
        resource_path: "/sensitive/research_data".to_string(),
        user_context,
        metadata: HashMap::new(),
        timestamp: SystemTime::now(),
    };
    
    // Request authorization
    let decision = spore.authorize_operation(&operation).await?;
    
    match decision {
        AuthorizationDecision::Allow { enhanced_by_beardog, .. } => {
            println!("✅ Individual user Alice gets FULL ACCESS instantly");
            println!("   - No restrictions");
            println!("   - No licensing required"); 
            println!("   - BearDog enhanced: {}", enhanced_by_beardog);
            println!("   - Zero friction experience");
        },
        _ => {
            println!("❌ Unexpected result - individuals should always get access");
        }
    }
    
    Ok(())
}

/// Demo: Corporate User Experience (License Required)
pub async fn demo_corporate_user_experience() -> Result<()> {
    println!("\n🏢 DEMO: Corporate User Experience");
    println!("==================================");
    
    // Create spore for any primal
    let mut spore = UniversalCryptographicSpore::new_for_primal("demo_primal")?;
    
    // Corporate user context (detected patterns)
    let user_context = UserContext {
        user_id: Some("corp_user_123".to_string()),
        session_id: "corp_session_456".to_string(),
        ip_address: "10.0.0.50".to_string(),  // Corporate IP range
        user_agent: Some("Corporate Automation System v2.1".to_string()),
        environment_info: {
            let mut env = HashMap::new();
            env.insert("user_type".to_string(), "corporate".to_string());
            env.insert("automation_level".to_string(), "high".to_string());
            env.insert("organization".to_string(), "ACME Corp".to_string());
            env
        },
    };
    
    // Create operation request
    let operation = OperationRequest {
        operation_type: "bulk_data_extraction".to_string(),
        resource_path: "/datasets/customer_data".to_string(),
        user_context,
        metadata: HashMap::new(),
        timestamp: SystemTime::now(),
    };
    
    // Request authorization
    let decision = spore.authorize_operation(&operation).await?;
    
    match decision {
        AuthorizationDecision::RequireLicense { terms, contact, organization_profile } => {
            println!("🚫 Corporate user requires license negotiation:");
            println!("   - Organization: {}", organization_profile.organization_name);
            println!("   - Base monthly rate: ${}", terms.base_monthly_rate);
            println!("   - Automation tax: {}x multiplier", terms.automation_tax_multiplier);
            println!("   - Contact for negotiation: {}", contact);
            println!("   - Entropy requirements: {:?}", terms.entropy_requirements);
        },
        _ => {
            println!("❌ Unexpected result - corporations should require license");
        }
    }
    
    Ok(())
}

/// Demo: Spore Evolution (Autonomous Security Improvement)
pub async fn demo_spore_evolution() -> Result<()> {
    println!("\n🌱 DEMO: Autonomous Spore Evolution");
    println!("===================================");
    
    // Create parent spore
    let mut parent_spore = UniversalCryptographicSpore::new_for_primal("demo_primal")?;
    println!("📍 Parent spore created: {}", parent_spore.spore_id);
    println!("   - Generation: {}", parent_spore.generation);
    
    // Simulate usage to trigger evolution
    // (In real implementation, this would be based on actual usage patterns)
    parent_spore.usage_stats.operations_count = 1000;
    parent_spore.usage_stats.performance_score = 0.95;
    
    // Force evolution for demo (normally automatic)
    match parent_spore.spawn_child().await {
        Ok(child_spore) => {
            println!("🌿 Child spore spawned autonomously:");
            println!("   - Child ID: {}", child_spore.spore_id);
            println!("   - Generation: {}", child_spore.generation);
            println!("   - Parent lineage: {:?}", child_spore.parent_lineage);
            println!("   - Evolved genetics with improved security");
        },
        Err(e) => {
            println!("ℹ️ Evolution not needed: {}", e);
        }
    }
    
    Ok(())
}

/// Demo: Security Service Integration (Enhanced Capabilities)
pub async fn demo_security_integration() -> Result<()> {
    println!("\n🧬 DEMO: BearDog Integration");
    println!("============================");
    
    // Create spore without BearDog
    let mut spore_standalone = UniversalCryptographicSpore::new_for_primal("demo_primal")?;
    println!("📍 Standalone spore: {}", spore_standalone.spore_id);
    println!("   - BearDog integrated: {}", spore_standalone.beardog_integration.is_some());
    
    // Initialize with BearDog integration
    let security_endpoint = Some("https://security.local:8443".to_string());
    spore_standalone.initialize_with_beardog(security_endpoint).await?;
    
    println!("🔗 After security integration:");
    println!("   - BearDog integrated: {}", spore_standalone.beardog_integration.is_some());
    
    if let Some(integration) = &spore_standalone.beardog_integration {
        println!("   - Genetics ID: {}", integration.genetics_id);
        println!("   - Extended capabilities: {:?}", integration.extended_capabilities);
        println!("   - Status: {:?}", integration.status);
    }
    
    Ok(())
}

/// Demo: Copy-Paste Integration Pattern for Any Primal
pub async fn demo_universal_integration_pattern() -> Result<()> {
    println!("\n📋 DEMO: Universal Integration Pattern");
    println!("======================================");
    println!("This pattern works identically for ALL primals:");
    
    // Demonstrate the same pattern works for different ecosystem participants
    let ecosystem_participants = vec!["orchestration-service", "compute-service", "ai-service", "storage-service", "security-service"];
    
    for participant_name in ecosystem_participants {
        let spore = UniversalCryptographicSpore::new_for_primal(participant_name)?;
        println!("✅ {} spore created: {}", participant_name, spore.spore_id);
        println!("   - Same API, same behavior, same sovereignty");
    }
    
    println!("\n💡 Integration steps for any primal:");
    println!("1. Copy universal_spore.rs to your primal");
    println!("2. Add spore to your main struct"); 
    println!("3. Call spore.authorize_operation() before sensitive operations");
    println!("4. Optionally initialize BearDog integration");
    println!("5. Your primal now has autonomous security with your terms!");
    
    Ok(())
}

/// Run all demos
pub async fn run_all_spore_demos() -> Result<()> {
    println!("🧬 UNIVERSAL CRYPTOGRAPHIC SPORE SYSTEM DEMO");
    println!("===========================================");
    println!("Demonstrating self-contained security for all primals");
    
    demo_individual_user_experience().await?;
    demo_corporate_user_experience().await?;
    demo_spore_evolution().await?;
    demo_beardog_integration().await?;
    demo_universal_integration_pattern().await?;
    
    println!("\n🎉 ALL DEMOS COMPLETED");
    println!("The Universal Spore system is ready for deployment to all primals!");
    
    Ok(())
} 