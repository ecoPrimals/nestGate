pub mod administration;
pub mod backup_recovery;
pub mod concurrent_users;
pub mod file_management;
/// **E2E COMPREHENSIVE WORKFLOW ORCHESTRATOR**
///
/// This module orchestrates end-to-end workflow testing using the unified configuration system.
/// Split from the original 1027-line e2e_comprehensive_workflows.rs for better maintainability.
///
/// **Architecture**:
/// - mod.rs: Main orchestrator and shared types
/// - nas_setup.rs: NAS setup workflow tests
/// - file_management.rs: File management workflow tests
/// - tier_management.rs: Tier management workflow tests
/// - concurrent_users.rs: Concurrent user workflow tests
/// - administration.rs: System administration workflow tests
/// - backup_recovery.rs: Backup and recovery workflow tests
/// - performance_optimization.rs: Performance optimization workflow tests
/// - network_protocols.rs: Network protocol workflow tests
pub mod nas_setup;
pub mod network_protocols;
pub mod performance_optimization;
pub mod tier_management;

use crate::common::config::UnifiedTestConfig;
use crate::common::{
    inject_chaos_events, setup_test_environment, simulate_concurrent_users,
    validate_performance_metrics,
};
use nestgate_core::{NestGateError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::sleep;

/// Test results structure
#[derive(Debug, Clone)]
pub struct TestResults {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub execution_time: Duration,
    pub performance_metrics: HashMap<String, f64>,
    pub error_messages: Vec<String>,
}

/// Workflow results structure for E2E tests
#[derive(Debug, Clone)]
pub struct WorkflowResults {
    pub workflow_name: String,
    pub success: bool,
    pub execution_time: Duration,
    pub steps_completed: usize,
    pub total_steps: usize,
    pub performance_metrics: HashMap<String, f64>,
    pub error_messages: Vec<String>,
    pub details: HashMap<String, serde_json::Value>,
}

/// Main E2E workflow orchestrator
pub struct E2EWorkflowOrchestrator {
    config: UnifiedTestConfig,
    start_time: Instant,
}

impl E2EWorkflowOrchestrator {
    /// Create new orchestrator with configuration
    pub fn new(config: UnifiedTestConfig) -> Self {
        Self {
            config,
            start_time: Instant::now(),
        }
    }

    /// Run comprehensive workflow tests
    pub async fn run_comprehensive_workflow_tests(&self) -> Result<Vec<WorkflowResults>> {
        let mut results = Vec::new();

        // Run all workflow tests
        results.push(nas_setup::test_nas_setup_workflow(&self.config).await?);
        results.push(file_management::test_file_management_workflow(&self.config).await?);
        results.push(tier_management::test_tier_management_workflow(&self.config).await?);
        results.push(concurrent_users::test_concurrent_user_workflow(&self.config).await?);
        results.push(administration::test_system_administration_workflow(&self.config).await?);
        results.push(backup_recovery::test_backup_recovery_workflow(&self.config).await?);
        results.push(
            performance_optimization::test_performance_optimization_workflow(&self.config).await?,
        );
        results.push(network_protocols::test_network_protocol_workflow(&self.config).await?);

        // Print comprehensive results
        self.print_comprehensive_results(&results).await;

        Ok(results)
    }

    /// Print comprehensive results summary
    async fn print_comprehensive_results(&self, results: &[WorkflowResults]) {
        let total_workflows = results.len();
        let successful_workflows = results.iter().filter(|r| r.success).count();
        let total_execution_time: Duration = results.iter().map(|r| r.execution_time).sum();

        println!("\n🎯 **COMPREHENSIVE E2E WORKFLOW RESULTS**");
        println!("=".repeat(60));
        println!("📊 **SUMMARY**:");
        println!(
            "  ✅ Successful workflows: {}/{}",
            successful_workflows, total_workflows
        );
        println!("  ⏱️ Total execution time: {:?}", total_execution_time);
        println!(
            "  📈 Success rate: {:.1}%",
            (successful_workflows as f64 / total_workflows as f64) * 100.0
        );

        println!("\n📋 **WORKFLOW DETAILS**:");
        for result in results {
            let status_icon = if result.success { "✅" } else { "❌" };
            println!(
                "  {} {}: {:?} ({}/{} steps)",
                status_icon,
                result.workflow_name,
                result.execution_time,
                result.steps_completed,
                result.total_steps
            );

            if !result.error_messages.is_empty() {
                for error in &result.error_messages {
                    println!("    🚨 {}", error);
                }
            }
        }

        if successful_workflows == total_workflows {
            println!("\n🎉 **ALL WORKFLOWS COMPLETED SUCCESSFULLY!**");
        } else {
            println!("\n⚠️ **SOME WORKFLOWS FAILED - REVIEW ERRORS ABOVE**");
        }
    }
}

/// E2E workflow test runner using unified configuration
pub async fn run_comprehensive_workflow_test(config: &UnifiedTestConfig) -> Result<TestResults> {
    let start_time = Instant::now();

    // Extract settings from unified config
    let performance = &config.extensions.performance;
    let chaos = &config.extensions.chaos;

    let test_duration = performance.duration;
    let concurrent_users = performance.concurrent_users;
    let enable_chaos_injection = chaos.enabled;

    println!("🚀 **STARTING COMPREHENSIVE E2E WORKFLOW TESTS**");
    println!("⚙️ Configuration:");
    println!("  📏 Test duration: {:?}", test_duration);
    println!("  👥 Concurrent users: {}", concurrent_users);
    println!(
        "  🌪️ Chaos injection: {}",
        if enable_chaos_injection {
            "enabled"
        } else {
            "disabled"
        }
    );

    // Initialize test environment
    setup_test_environment(config).await?;

    // Run orchestrated workflow tests
    let orchestrator = E2EWorkflowOrchestrator::new(config.clone());
    let workflow_results = orchestrator.run_comprehensive_workflow_tests().await?;

    // Aggregate results
    let total_tests = workflow_results.len();
    let passed_tests = workflow_results.iter().filter(|r| r.success).count();
    let failed_tests = total_tests - passed_tests;
    let execution_time = start_time.elapsed();

    let mut performance_metrics = HashMap::new();
    let mut error_messages = Vec::new();

    for result in &workflow_results {
        for (key, value) in &result.performance_metrics {
            performance_metrics.insert(format!("{}_{}", result.workflow_name, key), *value);
        }
        error_messages.extend(result.error_messages.clone());
    }

    Ok(TestResults {
        total_tests,
        passed_tests,
        failed_tests,
        execution_time,
        performance_metrics,
        error_messages,
    })
}
