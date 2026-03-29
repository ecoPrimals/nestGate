#![allow(
    unused,
    dead_code,
    deprecated,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

//! Ecosystem Integration Examples
//!
//! This file demonstrates how different ecoPrimals can discover and integrate
//! with each other using NestGate's unified capability system.
//!
//! Run tests: `cargo test --example ecosystem_integration_examples`

fn main() {
    println!("Ecosystem Integration Examples");
    println!("==============================");
    println!();
    println!("This demonstrates capability-based primal discovery.");
    println!("Run with: cargo test --example ecosystem_integration_examples");
}

use nestgate_core::{
    Result,
    capability_resolver::{CapabilityResolver, ResolvedService},
    unified_capabilities::UnifiedCapability,
};

// ============================================================================
// Example 1: SongBird Discovering NestGate Storage
// ============================================================================

/// SongBird orchestrator that discovers storage capabilities
pub struct SongBirdOrchestrator<R: CapabilityResolver> {
    resolver: R,
}

impl<R: CapabilityResolver> SongBirdOrchestrator<R> {
    pub fn new(resolver: R) -> Self {
        Self { resolver }
    }

    /// Discover NestGate storage by capability (not by name!)
    pub async fn discover_storage(&self) -> Result<ResolvedService> {
        self.resolver
            .resolve_capability(&UnifiedCapability::FileStorage)
            .await
    }

    /// Discover multiple storage backends for redundancy
    pub async fn discover_all_storage(&self) -> Result<Vec<ResolvedService>> {
        self.resolver
            .resolve_capability_all(&UnifiedCapability::ObjectStorage)
            .await
    }
}

// ============================================================================
// Example 2: ToadStool Discovering Compute Orchestration
// ============================================================================

/// ToadStool compute executor that discovers orchestration
pub struct ToadStoolExecutor<R: CapabilityResolver> {
    resolver: R,
}

impl<R: CapabilityResolver> ToadStoolExecutor<R> {
    pub fn new(resolver: R) -> Self {
        Self { resolver }
    }

    /// Discover orchestration service
    pub async fn discover_orchestrator(&self) -> Result<ResolvedService> {
        self.resolver
            .resolve_capability(&UnifiedCapability::Orchestration)
            .await
    }

    /// Discover storage for job inputs/outputs
    pub async fn discover_storage(&self) -> Result<ResolvedService> {
        self.resolver
            .resolve_capability(&UnifiedCapability::ObjectStorage)
            .await
    }
}

// ============================================================================
// Example 3: Multi-Primal Workflow Coordination
// ============================================================================

/// Workflow that spans multiple primals
pub struct EcosystemWorkflow<R: CapabilityResolver> {
    resolver: R,
}

impl<R: CapabilityResolver> EcosystemWorkflow<R> {
    pub fn new(resolver: R) -> Self {
        Self { resolver }
    }

    /// Orchestrate a complete workflow across primals
    ///
    /// Flow: Storage (NestGate) → Orchestration (SongBird) → Compute (ToadStool)
    pub async fn execute_multi_primal_workflow(&self) -> Result<WorkflowResult> {
        // Step 1: Discover storage for input data
        let storage = self
            .resolver
            .resolve_capability(&UnifiedCapability::FileStorage)
            .await?;

        // Step 2: Discover orchestration for workflow management
        let orchestrator = self
            .resolver
            .resolve_capability(&UnifiedCapability::Orchestration)
            .await?;

        // Step 3: Discover compute for processing
        let compute = self
            .resolver
            .resolve_capability(&UnifiedCapability::TaskExecution)
            .await?;

        Ok(WorkflowResult {
            input_storage: storage.url(),
            orchestrator: orchestrator.url(),
            compute_engine: compute.url(),
        })
    }

    /// Discover with fallback pattern
    pub async fn discover_with_fallback(&self) -> Result<ResolvedService> {
        // Try object storage first
        if let Ok(service) = self
            .resolver
            .resolve_capability(&UnifiedCapability::ObjectStorage)
            .await
        {
            return Ok(service);
        }

        // Fallback to file storage
        self.resolver
            .resolve_capability(&UnifiedCapability::FileStorage)
            .await
    }
}

#[derive(Debug)]
pub struct WorkflowResult {
    pub input_storage: String,
    pub orchestrator: String,
    pub compute_engine: String,
}

// ============================================================================
// Example 4: Graceful Degradation Pattern
// ============================================================================

/// Service that works with or without ecosystem integration
pub struct ResilientService<R: CapabilityResolver> {
    resolver: R,
}

impl<R: CapabilityResolver> ResilientService<R> {
    pub fn new(resolver: R) -> Self {
        Self { resolver }
    }

    /// Process data with optional external storage
    pub async fn process_with_optional_storage(&self, _data: Vec<u8>) -> Result<String> {
        // Try to discover external storage
        let storage = self
            .resolver
            .resolve_capability(&UnifiedCapability::FileStorage)
            .await
            .ok();

        if let Some(storage_service) = storage {
            // Use external storage if available
            Ok(format!(
                "Processed with external storage: {}",
                storage_service.url()
            ))
        } else {
            // Gracefully degrade to local storage
            Ok("Processed with local storage".to_string())
        }
    }

    /// Discover multiple capabilities, use what's available
    pub async fn adaptive_processing(&self) -> AdaptiveConfig {
        let mut config = AdaptiveConfig::default();

        // Try to discover each capability
        if let Ok(storage) = self
            .resolver
            .resolve_capability(&UnifiedCapability::FileStorage)
            .await
        {
            config.storage_url = Some(storage.url());
        }

        if let Ok(orchestrator) = self
            .resolver
            .resolve_capability(&UnifiedCapability::Orchestration)
            .await
        {
            config.orchestrator_url = Some(orchestrator.url());
        }

        if let Ok(api) = self
            .resolver
            .resolve_capability(&UnifiedCapability::HttpApi)
            .await
        {
            config.api_url = Some(api.url());
        }

        config
    }
}

#[derive(Debug, Default)]
pub struct AdaptiveConfig {
    pub storage_url: Option<String>,
    pub orchestrator_url: Option<String>,
    pub api_url: Option<String>,
}

// ============================================================================
// Tests demonstrating usage
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn example_songbird_discovers_nestgate() {
        // Set up environment-based discovery for testing
        // Uses NESTGATE_CAPABILITY_FILE_STORAGE_ENDPOINT (from Display impl)
        nestgate_core::env_process::set_var(
            "NESTGATE_CAPABILITY_FILE_STORAGE_ENDPOINT",
            "http://nestgate.local:8080",
        );

        let resolver = EnvironmentResolver::new();
        let orchestrator = SongBirdOrchestrator::new(resolver);

        // Discover storage
        let storage = orchestrator.discover_storage().await;
        assert!(storage.is_ok());

        let service = storage.unwrap();
        assert_eq!(service.host, "nestgate.local");
        assert_eq!(service.port, 8080);

        nestgate_core::env_process::remove_var("NESTGATE_CAPABILITY_FILE_STORAGE_ENDPOINT");
    }

    #[tokio::test]
    async fn example_toadstool_discovers_orchestration() {
        nestgate_core::env_process::set_var(
            "NESTGATE_CAPABILITY_ORCHESTRATION_ENDPOINT",
            "http://songbird.local:9090",
        );

        let resolver = EnvironmentResolver::new();
        let executor = ToadStoolExecutor::new(resolver);

        let orchestrator = executor.discover_orchestrator().await;
        assert!(orchestrator.is_ok());

        let service = orchestrator.unwrap();
        assert_eq!(service.host, "songbird.local");
        assert_eq!(service.port, 9090);

        nestgate_core::env_process::remove_var("NESTGATE_CAPABILITY_ORCHESTRATION_ENDPOINT");
    }

    #[tokio::test]
    async fn example_graceful_degradation() {
        // No storage available - should gracefully degrade
        let resolver = EnvironmentResolver::new();
        let service = ResilientService::new(resolver);

        let result = service.process_with_optional_storage(vec![1, 2, 3]).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Processed with local storage");
    }

    #[tokio::test]
    async fn example_multi_primal_workflow() {
        // Set up multiple services
        nestgate_core::env_process::set_var(
            "NESTGATE_CAPABILITY_FILE_STORAGE_ENDPOINT",
            "http://nestgate.local:8080",
        );
        nestgate_core::env_process::set_var(
            "NESTGATE_CAPABILITY_ORCHESTRATION_ENDPOINT",
            "http://songbird.local:9090",
        );
        nestgate_core::env_process::set_var(
            "NESTGATE_CAPABILITY_TASK_EXECUTION_ENDPOINT",
            "http://toadstool.local:7070",
        );

        let resolver = EnvironmentResolver::new();
        let workflow = EcosystemWorkflow::new(resolver);

        let result = workflow.execute_multi_primal_workflow().await;
        assert!(result.is_ok());

        let workflow_result = result.unwrap();
        assert!(workflow_result.input_storage.contains("nestgate.local"));
        assert!(workflow_result.orchestrator.contains("songbird.local"));
        assert!(workflow_result.compute_engine.contains("toadstool.local"));

        // Cleanup
        nestgate_core::env_process::remove_var("NESTGATE_CAPABILITY_FILE_STORAGE_ENDPOINT");
        nestgate_core::env_process::remove_var("NESTGATE_CAPABILITY_ORCHESTRATION_ENDPOINT");
        nestgate_core::env_process::remove_var("NESTGATE_CAPABILITY_TASK_EXECUTION_ENDPOINT");
    }
}
