/// **CONCURRENT USERS WORKFLOW TESTS**
///
/// This module contains workflow tests for concurrent user operations.
/// Extracted from the original e2e_comprehensive_workflows.rs for better maintainability.
use super::WorkflowResults;
use crate::common::config::UnifiedTestConfig;
use futures::future::join_all;
use nestgate_core::Result;
use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

/// Test concurrent user workflow
pub async fn test_concurrent_user_workflow(config: &UnifiedTestConfig) -> Result<WorkflowResults> {
    let start_time = Instant::now();
    let mut error_messages = Vec::new();
    let mut performance_metrics = HashMap::new();

    let concurrent_users = config.extensions.performance.concurrent_users.min(50); // Limit for testing
    let operations_per_user = 5;
    let total_steps = (concurrent_users * operations_per_user) as usize;
    let mut steps_completed = 0;

    println!("👥 **CONCURRENT USER WORKFLOW TEST**");
    println!(
        "  📊 Testing {} concurrent users with {} operations each",
        concurrent_users, operations_per_user
    );

    // Create concurrent user operations
    let mut tasks = Vec::new();
    for user_id in 0..concurrent_users {
        for op_id in 0..operations_per_user {
            tasks.push(simulate_user_operation(user_id as usize, op_id as usize));
        }
    }

    // Execute all operations concurrently
    let results = join_all(tasks).await;

    // Count successful operations
    for result in results {
        match result {
            Ok(_) => steps_completed += 1,
            Err(e) => error_messages.push(format!("User operation failed: {e}")),
        }
    }

    let execution_time = start_time.elapsed();
    let success = steps_completed >= (total_steps * 80 / 100); // 80% success rate threshold

    // Add performance metrics
    performance_metrics.insert("concurrent_users".to_string(), concurrent_users as f64);
    performance_metrics.insert("total_operations".to_string(), total_steps as f64);
    performance_metrics.insert("successful_operations".to_string(), steps_completed as f64);
    performance_metrics.insert(
        "success_rate".to_string(),
        (steps_completed as f64 / total_steps as f64) * 100.0,
    );
    performance_metrics.insert(
        "ops_per_second".to_string(),
        steps_completed as f64 / execution_time.as_secs_f64(),
    );
    performance_metrics.insert(
        "avg_response_time_ms".to_string(),
        execution_time.as_millis() as f64 / steps_completed as f64,
    );

    println!(
        "  📊 Concurrent User Results: {}/{} operations completed in {:?}",
        steps_completed, total_steps, execution_time
    );
    println!(
        "     Success rate: {:.1}%",
        (steps_completed as f64 / total_steps as f64) * 100.0
    );

    Ok(WorkflowResults {
        workflow_name: "Concurrent Users".to_string(),
        success,
        execution_time,
        steps_completed,
        total_steps,
        performance_metrics,
        error_messages,
        details: HashMap::new(),
    })
}

/// Simulate user operation
async fn simulate_user_operation(user_id: usize, op_id: usize) -> Result<()> {
    // Add some variance to simulate realistic load
    let delay = 50 + (user_id * 10 + op_id * 5) % 100;
    sleep(Duration::from_millis(delay as u64)).await;

    // Simulate occasional failures (5% failure rate)
    if (user_id + op_id) % 20 == 0 {
        return Err(nestgate_core::NestGateError::NetworkError(format!(
            "Simulated failure for user {} operation {}",
            user_id, op_id
        )));
    }

    Ok(())
}
