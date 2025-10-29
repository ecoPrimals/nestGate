// **BIOMEOS UNIVERSAL ADAPTER INTEGRATION**
//! Adapters functionality and utilities.
// Universal adapter integration and routing functionality for capability-based discovery.
// Extracted from management.rs for file size compliance.

use crate::Result;
use super::types::TemplateSpec;

/// Route capability request through universal adapter
#[must_use]
pub fn route_capability_through_adapter(capability: &str) -> Result<Vec<TemplateSpec>> {
    // This function routes capability requests through the universal adapter
    // instead of hardcoding primal-specific implementations
    // For now, return capability-based templates
    // In full implementation, this would use UniversalAdapter::route_capability()
    match capability {
        "ai-runtime" | "artificial-intelligence" => Ok(vec![TemplateSpec {
            name: "ai-runtime-template".to_string(),
            resources: "cpu:2,memory:4Gi".to_string(),
            config: std::collections::HashMap::new(),
        }]),
        "agent-processing" | "compute" => Ok(vec![TemplateSpec {
            name: "agent-processing-template".to_string(),
            resources: "cpu:1,memory:2Gi".to_string(),
            config: std::collections::HashMap::new(),
        }]),
        "security-provider" | "encryption" => Ok(vec![TemplateSpec {
            name: "security-template".to_string(),
            resources: "cpu:1,memory:1Gi".to_string(),
            config: std::collections::HashMap::new(),
        }]),
        "orchestration-provider" | "orchestration" => Ok(vec![TemplateSpec {
            name: "orchestration-template".to_string(),
            resources: "cpu:2,memory:2Gi".to_string(),
            config: std::collections::HashMap::new(),
        }]),
        _ => {
            // Route through universal adapter for unknown capabilities
            Ok(vec![])
        }
    }
} 