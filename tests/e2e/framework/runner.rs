//! # E2E Test Runner
//!
//! Main test execution engine for end-to-end testing

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use uuid::Uuid;
use reqwest::Client;

use super::types::{E2EConfig, E2EEndpoints, E2EScenario, E2ETestResult, E2EMetrics, TestStepResult};
use super::scenarios::{
    UserLifecycleRunner, ApiValidationRunner, DataFlowRunner, ServiceIntegrationRunner,
    LoadTestingRunner, SecurityValidationRunner, ConfigValidationRunner,
};

/// **E2E TESTING FRAMEWORK**
///
/// Main framework for orchestrating end-to-end tests
pub struct E2ETestingFramework {
    /// Configuration for E2E tests
    config: E2EConfig,
    /// HTTP client for API testing
    http_client: Client,
    /// Active test sessions
    active_tests: Arc<RwLock<Vec<E2ETestResult>>>,
    /// System endpoints
    endpoints: E2EEndpoints,
}

impl E2ETestingFramework {
    /// Create new E2E testing framework
    pub fn new(config: E2EConfig, endpoints: E2EEndpoints) -> Self {
        let http_client = Client::builder()
            .timeout(config.timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config,
            http_client,
            active_tests: Arc::new(RwLock::new(Vec::new())),
            endpoints,
        }
    }

    /// **RUN E2E TEST SCENARIO**
    ///
    /// Execute a comprehensive end-to-end test scenario
    pub async fn run_e2e_test(
        &self,
        scenario: E2EScenario,
    ) -> Result<E2ETestResult, Box<dyn std::error::Error + Send + Sync>> {
        let test_id = Uuid::new_v4();
        let start_time = std::time::SystemTime::now();

        let mut result = E2ETestResult {
            scenario: scenario.clone(),
            test_id,
            start_time,
            end_time: None,
            success: false,
            error_message: None,
            metrics: E2EMetrics::default(),
            step_results: Vec::new(),
        };

        // Add to active tests
        {
            let mut active_tests = self.active_tests.write().await;
            active_tests.push(result.clone());
        }

        // Execute E2E test
        let test_result = self.execute_e2e_scenario(&scenario, &mut result).await;

        // Update result
        result.end_time = Some(std::time::SystemTime::now());
        result.success = test_result.is_ok() && result.step_results.iter().all(|s| s.success);
        
        if let Err(e) = test_result {
            result.error_message = Some(e.to_string());
        }

        // Calculate overall metrics
        result.metrics = self.calculate_overall_metrics(&result.step_results);

        // Remove from active tests
        {
            let mut active_tests = self.active_tests.write().await;
            active_tests.retain(|t| t.test_id != test_id);
        }

        Ok(result)
    }

    /// **RUN E2E TEST SUITE**
    ///
    /// Execute multiple E2E scenarios
    pub async fn run_e2e_suite(
        &self,
        scenarios: Vec<E2EScenario>,
        parallel: bool,
    ) -> Result<Vec<E2ETestResult>, Box<dyn std::error::Error + Send + Sync>> {
        if parallel {
            // Run scenarios in parallel
            let semaphore = Arc::new(tokio::sync::Semaphore::new(self.config.concurrent_limit));
            let mut handles = Vec::new();

            for scenario in scenarios {
                let semaphore = semaphore.clone();
                let framework = self;
                
                let handle = tokio::spawn(async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    framework.run_e2e_test(scenario).await
                });
                
                handles.push(handle);
            }

            let mut results = Vec::new();
            for handle in handles {
                match handle.await? {
                    Ok(result) => results.push(result),
                    Err(e) => {
                        if self.config.fail_fast {
                            return Err(e);
                        }
                        // Continue with other tests if fail_fast is disabled
                    }
                }
            }

            Ok(results)
        } else {
            // Run scenarios sequentially
            let mut results = Vec::new();
            for scenario in scenarios {
                let result = self.run_e2e_test(scenario).await?;
                let success = result.success;
                results.push(result);

                if !success && self.config.fail_fast {
                    break;
                }

                // Brief pause between tests
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
            Ok(results)
        }
    }

    /// Execute individual E2E scenario
    async fn execute_e2e_scenario(
        &self,
        scenario: &E2EScenario,
        result: &mut E2ETestResult,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match scenario {
            E2EScenario::UserLifecycle { user_count, operations_per_user } => {
                let runner = UserLifecycleRunner::new(&self.config, &self.http_client);
                runner.run(*user_count, *operations_per_user, result).await
            }
            E2EScenario::ApiValidation { endpoints, concurrent_requests } => {
                let runner = ApiValidationRunner::new(&self.config, &self.http_client);
                runner.run(endpoints.clone(), *concurrent_requests, result).await
            }
            E2EScenario::DataFlowValidation { data_size_mb, concurrent_streams } => {
                let runner = DataFlowRunner::new(&self.config, &self.http_client);
                runner.run(*data_size_mb, *concurrent_streams, result).await
            }
            E2EScenario::ServiceIntegration { services, integration_depth } => {
                let runner = ServiceIntegrationRunner::new(&self.config, &self.http_client);
                runner.run(services.clone(), integration_depth.clone(), result).await
            }
            E2EScenario::LoadTesting { concurrent_users, duration, ramp_up_time } => {
                let runner = LoadTestingRunner::new(&self.config, &self.http_client);
                runner.run(*concurrent_users, *duration, *ramp_up_time, result).await
            }
            E2EScenario::SecurityValidation { attack_scenarios } => {
                let runner = SecurityValidationRunner::new(&self.config, &self.http_client);
                runner.run(attack_scenarios.clone(), result).await
            }
            E2EScenario::ConfigValidation { config_variations } => {
                let runner = ConfigValidationRunner::new(&self.config, &self.http_client);
                runner.run(config_variations.clone(), result).await
            }
        }
    }

    /// Calculate overall metrics from step results
    pub fn calculate_overall_metrics(&self, step_results: &[TestStepResult]) -> E2EMetrics {
        let total_steps = step_results.len();
        let successful_steps = step_results.iter().filter(|s| s.success).count();
        
        let total_assertions_passed: usize = step_results.iter()
            .map(|s| s.assertions_passed)
            .sum();
        let total_assertions: usize = step_results.iter()
            .map(|s| s.assertions_total)
            .sum();
        
        let total_duration: Duration = step_results.iter()
            .map(|s| s.duration)
            .sum();
        
        let average_duration_ms = if total_steps > 0 {
            total_duration.as_millis() as f64 / total_steps as f64
        } else {
            0.0
        };
        
        let error_rate = if total_steps > 0 {
            ((total_steps - successful_steps) as f64 / total_steps as f64) * 100.0
        } else {
            0.0
        };

        E2EMetrics {
            total_requests: total_steps as u64,
            successful_requests: successful_steps as u64,
            failed_requests: (total_steps - successful_steps) as u64,
            average_response_time_ms: average_duration_ms,
            p95_response_time_ms: average_duration_ms * 1.2, // Approximation
            p99_response_time_ms: average_duration_ms * 1.5, // Approximation
            throughput_rps: if total_duration.as_secs() > 0 {
                total_steps as f64 / total_duration.as_secs() as f64
            } else {
                0.0
            },
            error_rate_percentage: error_rate,
            data_processed_mb: 0.0, // Would be calculated by specific runners
            assertions_passed: total_assertions_passed,
            assertions_total: total_assertions,
        }
    }

    /// Get active tests
    pub async fn get_active_tests(&self) -> Vec<E2ETestResult> {
        let active_tests = self.active_tests.read().await;
        active_tests.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_framework_creation() -> Result<(), Box<dyn std::error::Error>> {
        let config = E2EConfig::default();
        let endpoints = E2EEndpoints::default();
        let framework = E2ETestingFramework::new(config, endpoints);
        
        let active_tests = framework.get_active_tests().await;
        assert!(active_tests.is_empty());
    Ok(())
}
} 