//! # E2E Testing Framework Types
//!
//! Core types and data structures for end-to-end testing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

/// **E2E TEST SCENARIO**
///
/// Comprehensive end-to-end test scenarios covering all major workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum E2EScenario {
    /// Complete user workflow from registration to data access
    UserLifecycle {
        user_count: usize,
        operations_per_user: usize,
    },
    /// API endpoint validation across all services
    ApiValidation {
        endpoints: Vec<String>,
        concurrent_requests: usize,
    },
    /// Data flow validation from ingestion to retrieval
    DataFlowValidation {
        data_size_mb: u64,
        concurrent_streams: usize,
    },
    /// Service integration validation
    ServiceIntegration {
        services: Vec<String>,
        integration_depth: IntegrationDepth,
    },
    /// Performance validation under load
    LoadTesting {
        concurrent_users: usize,
        duration: Duration,
        ramp_up_time: Duration,
    },
    /// Security validation
    SecurityValidation {
        attack_scenarios: Vec<AttackScenario>,
    },
    /// Configuration validation
    ConfigValidation {
        config_variations: Vec<ConfigVariation>,
    },
}

/// **INTEGRATION DEPTH**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationDepth {
    /// Basic connectivity tests
    Shallow,
    /// Functional integration tests
    Medium,
    /// Complete workflow tests
    Deep,
}

/// **ATTACK SCENARIOS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttackScenario {
    /// SQL injection attempts
    SqlInjection,
    /// Cross-site scripting attempts
    XssAttempts,
    /// Authentication bypass attempts
    AuthBypass,
    /// Rate limiting validation
    RateLimit,
    /// Input validation bypass
    InputValidation,
}

/// **CONFIG VARIATIONS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigVariation {
    /// Minimal configuration
    Minimal,
    /// Development configuration
    Development,
    /// Production configuration
    Production,
    /// High-availability configuration
    HighAvailability,
    /// Custom configuration
    Custom(HashMap<String, String>),
}

/// **E2E TEST RESULT**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct E2ETestResult {
    pub scenario: E2EScenario,
    pub test_id: Uuid,
    pub start_time: std::time::SystemTime,
    pub end_time: Option<std::time::SystemTime>,
    pub success: bool,
    pub error_message: Option<String>,
    pub metrics: E2EMetrics,
    pub step_results: Vec<TestStepResult>,
}

/// **TEST STEP RESULT**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStepResult {
    pub step_name: String,
    pub success: bool,
    pub duration: Duration,
    pub error_message: Option<String>,
    pub assertions_passed: usize,
    pub assertions_total: usize,
}

/// **E2E METRICS**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct E2EMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub p95_response_time_ms: f64,
    pub p99_response_time_ms: f64,
    pub throughput_rps: f64,
    pub error_rate_percentage: f64,
    pub data_processed_mb: f64,
    pub assertions_passed: usize,
    pub assertions_total: usize,
}

/// **E2E CONFIGURATION**
#[derive(Debug, Clone)]
pub struct E2EConfig {
    pub base_url: String,
    pub timeout: Duration,
    pub retry_attempts: usize,
    pub concurrent_limit: usize,
    pub metrics_collection_interval: Duration,
    pub verbose_logging: bool,
    pub fail_fast: bool,
}

/// **SYSTEM ENDPOINTS**
#[derive(Debug, Clone)]
pub struct E2EEndpoints {
    pub api_base: String,
    pub auth_endpoint: String,
    pub health_endpoint: String,
    pub metrics_endpoint: String,
    pub admin_endpoint: String,
    pub websocket_endpoint: String,
}

impl Default for E2EConfig {
    fn default() -> Self {
        Self {
            base_url: std::env::var("NESTGATE_API_ENDPOINT")
                .unwrap_or_else(|_| format!("http://{}:{}", 
                    std::env::var("NESTGATE_HOSTNAME").unwrap_or_else(|_| nestgate_core::constants::TEST_HOSTNAME.to_string()),
                    std::env::var("NESTGATE_API_PORT").unwrap_or_else(|_| "8080".to_string())
                )),
            timeout: Duration::from_secs(30),
            retry_attempts: 3,
            concurrent_limit: 10,
            metrics_collection_interval: Duration::from_secs(1),
            verbose_logging: false,
            fail_fast: false,
        }
    }
}

impl Default for E2EEndpoints {
    fn default() -> Self {
        Self {
            api_base: "/api".to_string(),
            auth_endpoint: "/api/auth".to_string(),
            health_endpoint: "/api/health".to_string(),
            metrics_endpoint: "/api/metrics".to_string(),
            admin_endpoint: "/api/admin".to_string(),
            websocket_endpoint: "/ws".to_string(),
        }
    }
} 