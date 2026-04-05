// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use crate::Result;

/// Native async load balancer trait - replaces #\[async_trait\] `LoadBalancer`
pub trait NativeAsyncLoadBalancer<
    const MAX_SERVICES: usize = 1000,
    const MAX_CONCURRENT_REQUESTS: usize = 10000,
    const STATS_RETENTION_SECS: u64 = 86400, // 24 hours
    const HEALTH_CHECK_INTERVAL_SECS: u64 = 30,
>: Send + Sync {
    /// Type alias for ServiceInfo
    type ServiceInfo: Clone + Send + Sync + 'static;
    /// Type alias for ServiceRequest
    type ServiceRequest: Clone + Send + Sync + 'static;
    /// Type alias for ServiceResponse
    type ServiceResponse: Clone + Send + Sync + 'static;
    /// Type alias for LoadBalancerStats
    type LoadBalancerStats: Clone + Send + Sync + 'static;
    /// Type alias for ServiceStats
    type ServiceStats: Clone + Send + Sync + 'static;
    /// Add service - native async, no Future boxing
    fn add_service(&self, service: Self::ServiceInfo) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Remove service - direct async method
    fn remove_service(&self, service_id: &str) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Route request - native async with zero allocation
    fn route_request(
        &self,
        request: Self::ServiceRequest,
    ) -> impl std::future::Future<Output = Result<Self::ServiceResponse>> + Send;

    /// Get load balancer statistics - compile-time optimization
    fn get_stats(&self) -> impl std::future::Future<Output = Result<Self::LoadBalancerStats>> + Send;

    /// Get service statistics - direct async method
    fn get_service_stats(&self, service_id: &str) -> impl std::future::Future<Output = Result<Self::ServiceStats>> + Send;

    /// Performs health checks on all services and returns their status
    fn health_check_all(&self) -> impl std::future::Future<Output = Result<Vec<(String, bool)>>> + Send;

    /// Update service weight - no Future boxing
    fn update_service_weight(&self, service_id: &str, weight: f64) -> impl std::future::Future<Output = Result<()>> + Send;

    /// List active services - compile-time optimization
    fn list_services(&self) -> impl std::future::Future<Output = Result<Vec<Self::ServiceInfo>>> + Send;

    /// Get service by ID - direct async method
    fn get_service(&self, service_id: &str) -> impl std::future::Future<Output = Result<Option<Self::ServiceInfo>>> + Send;

    /// Check if service exists - native async
    fn service_exists(&self, service_id: &str) -> impl std::future::Future<Output = Result<bool>> + Send;

    /// Compile-time constants
    /// Returns the maximum number of services.
    #[must_use] fn max_services() -> usize { MAX_SERVICES }
    /// Returns the maximum number of concurrent requests.
    #[must_use] fn max_concurrent_requests() -> usize { MAX_CONCURRENT_REQUESTS }
    /// Returns the statistics retention duration in seconds.
    #[must_use] fn stats_retention_seconds() -> u64 { STATS_RETENTION_SECS }
    /// Returns the health check interval in seconds.
    #[must_use] fn health_check_interval_seconds() -> u64 { HEALTH_CHECK_INTERVAL_SECS }
    }

/// Native async communication provider trait - replaces #\[async_trait\] `CommunicationProvider`  
pub trait NativeAsyncCommunicationProvider<
    const MAX_CONNECTIONS: usize = 1000,
    const MAX_MESSAGE_SIZE: usize = 1024,
    const CONNECTION_TIMEOUT_SECS: u64 = 30,
    const MESSAGE_RETRY_ATTEMPTS: u32 = 3,
>: Send + Sync
{
    /// Type alias for Message
    type Message: Clone + Send + Sync + 'static;
    /// Type alias for Address
    type Address: Clone + Send + Sync + 'static;
    /// Type alias for ConnectionInfo
    type ConnectionInfo: Clone + Send + Sync + 'static;
    /// Send message - native async, no Future boxing
    fn send_message(
        &self,
        endpoint: Self::Address,
        message: Self::Message,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Receive message - direct async method
    fn receive_message(&self) -> impl std::future::Future<Output = Result<Self::Message>> + Send;

    /// Establish connection - native async
    fn connect(
        &self,
        endpoint: Self::Address,
    ) -> impl std::future::Future<Output = Result<Self::ConnectionInfo>> + Send;

    /// Close connection - no Future boxing
    fn disconnect(
        &self,
        connection: &Self::ConnectionInfo,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Get connection status - compile-time optimization
    fn connection_status(
        &self,
        connection: &Self::ConnectionInfo,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Broadcast message - direct async method
    fn broadcast(
        &self,
        message: Self::Message,
    ) -> impl std::future::Future<Output = Result<u32>> + Send;

    /// List active connections - native async
    fn list_connections(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Self::ConnectionInfo>>> + Send;

    /// Check connection health - no Future boxing
    fn ping(
        &self,
        connection: &Self::ConnectionInfo,
    ) -> impl std::future::Future<Output = Result<std::time::Duration>> + Send;

    /// Compile-time constants
    /// Returns the maximum number of connections.
    #[must_use]
    fn max_connections() -> usize {
        MAX_CONNECTIONS
    }
    /// Returns the maximum message size.
    #[must_use]
    fn max_message_size() -> usize {
        MAX_MESSAGE_SIZE
    }
    /// Returns the connection timeout in seconds.
    #[must_use]
    fn connection_timeout_seconds() -> u64 {
        CONNECTION_TIMEOUT_SECS
    }
    /// Returns the number of message retry attempts.
    #[must_use]
    fn message_retry_attempts() -> u32 {
        MESSAGE_RETRY_ATTEMPTS
    }
}

/// Native async MCP protocol handler trait - replaces #\[async_trait\] `MCPProtocolHandler`
pub trait NativeAsyncMCPProtocolHandler<
    const MAX_SESSIONS: usize = 1000,
    const SESSION_TIMEOUT_SECS: u64 = 300,
    const MAX_MESSAGE_SIZE: usize = 1024,
    const PROTOCOL_VERSION: u32 = 1,
>: Send + Sync
{
    /// Type alias for SessionInfo
    type SessionInfo: Clone + Send + Sync + 'static;
    /// Type alias for Message
    type Message: Clone + Send + Sync + 'static;
    /// Type alias for Response
    type Response: Clone + Send + Sync + 'static;
    /// Type alias for Error
    type Error: Clone + Send + Sync + 'static;
    /// Create session - native async, no Future boxing
    fn create_session(
        &self,
        client_id: String,
    ) -> impl std::future::Future<Output = Result<Self::SessionInfo>> + Send;

    /// Close session - direct async method
    fn close_session(
        &self,
        session_id: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Send MCP message - native async
    fn send_mcp_message(
        &self,
        session_id: &str,
        message: Self::Message,
    ) -> impl std::future::Future<Output = Result<Self::Response>> + Send;

    /// Handle incoming message - no Future boxing
    fn handle_message(
        &self,
        session_id: &str,
        message: Self::Message,
    ) -> impl std::future::Future<Output = Result<Self::Response>> + Send;

    /// Get session info - compile-time optimization
    fn get_session(
        &self,
        session_id: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self::SessionInfo>>> + Send;

    /// List active sessions - direct async method
    fn list_sessions(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Self::SessionInfo>>> + Send;

    /// Ping session - native async
    fn ping_session(
        &self,
        session_id: &str,
    ) -> impl std::future::Future<Output = Result<std::time::Duration>> + Send;

    /// Compile-time constants
    /// Returns the maximum number of sessions.
    #[must_use]
    fn max_sessions() -> usize {
        MAX_SESSIONS
    }
    /// Returns the session timeout in seconds.
    #[must_use]
    fn session_timeout_seconds() -> u64 {
        SESSION_TIMEOUT_SECS
    }
    /// Returns the maximum message size.
    #[must_use]
    fn max_message_size() -> usize {
        MAX_MESSAGE_SIZE
    }
    /// Returns the protocol version.
    #[must_use]
    fn protocol_version() -> u32 {
        PROTOCOL_VERSION
    }
}

/// Native async automation service trait - replaces #\[async_trait\] `AutomationService`
pub trait NativeAsyncAutomationService<
    const MAX_WORKFLOWS: usize = 1000,
    const MAX_CONCURRENT_EXECUTIONS: usize = 100,
    const EXECUTION_TIMEOUT_SECS: u64 = 300,
    const MAX_WORKFLOW_STEPS: usize = 100,
>: Send + Sync
{
    /// Type alias for WorkflowDefinition
    type WorkflowDefinition: Clone + Send + Sync + 'static;
    /// Type alias for WorkflowExecution
    type WorkflowExecution: Clone + Send + Sync + 'static;
    /// Type alias for ExecutionResult
    type ExecutionResult: Clone + Send + Sync + 'static;
    /// Create workflow - native async, no Future boxing
    fn create_workflow(
        &self,
        definition: Self::WorkflowDefinition,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Execute workflow - direct async method
    fn execute_workflow(
        &self,
        workflow_id: &str,
        parameters: std::collections::HashMap<String, serde_json::Value>,
    ) -> impl std::future::Future<Output = Result<Self::WorkflowExecution>> + Send;

    /// Stop execution - native async
    fn stop_execution(
        &self,
        execution_id: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Get execution status - no Future boxing
    fn get_execution_status(
        &self,
        execution_id: &str,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    /// List executions - compile-time optimization
    fn list_executions(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Self::WorkflowExecution>>> + Send;

    /// Get execution result - direct async method
    fn get_execution_result(
        &self,
        execution_id: &str,
    ) -> impl std::future::Future<Output = Result<Self::ExecutionResult>> + Send;

    /// Compile-time constants
    /// Returns the maximum number of workflows.
    #[must_use]
    fn max_workflows() -> usize {
        MAX_WORKFLOWS
    }
    /// Returns the maximum number of concurrent executions.
    #[must_use]
    fn max_concurrent_executions() -> usize {
        MAX_CONCURRENT_EXECUTIONS
    }
    /// Returns the execution timeout in seconds.
    #[must_use]
    fn execution_timeout_seconds() -> u64 {
        EXECUTION_TIMEOUT_SECS
    }
    /// Returns the maximum number of workflow steps.
    #[must_use]
    fn max_workflow_steps() -> usize {
        MAX_WORKFLOW_STEPS
    }
}

/// Native async universal service provider trait - replaces #\[async_trait\] `UniversalServiceProvider`
pub trait NativeAsyncUniversalServiceProvider<
    const MAX_SERVICES: usize = 1000,
    const SERVICE_TIMEOUT_SECS: u64 = 300,
>: Send + Sync
{
    /// Type alias for ServiceDefinition
    type ServiceDefinition: Clone + Send + Sync + 'static;
    /// Type alias for ServiceInstance
    type ServiceInstance: Clone + Send + Sync + 'static;
    /// Register service - native async, no Future boxing
    fn register_service(
        &self,
        definition: Self::ServiceDefinition,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Unregister service - direct async method
    fn unregister_service(
        &self,
        service_id: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Get service instance - native async
    fn get_service_instance(
        &self,
        service_id: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self::ServiceInstance>>> + Send;

    /// List services - no Future boxing
    fn list_services(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Self::ServiceDefinition>>> + Send;

    /// Compile-time constants
    /// Returns the maximum number of services.
    #[must_use]
    fn max_services() -> usize {
        MAX_SERVICES
    }
    /// Returns the service timeout in seconds.
    #[must_use]
    fn service_timeout_seconds() -> u64 {
        SERVICE_TIMEOUT_SECS
    }
}

/// Native async security service trait - replaces #\[async_trait\] `SecurityService`
/// **DEPRECATED**: Service pattern consolidated into canonical security
#[deprecated(
    since = "0.9.0",
    note = "Use crate::traits::canonical::CanonicalSecurity for security services"
)]
/// NativeAsyncSecurityService trait
pub trait NativeAsyncSecurityService<
    const MAX_SESSIONS: usize = 1000,
    const SESSION_DURATION_SECS: u64 = 300,
>: Send + Sync
{
    /// Type alias for AuthRequest
    type AuthRequest: Clone + Send + Sync + 'static;
    /// Type alias for AuthResponse
    type AuthResponse: Clone + Send + Sync + 'static;
    /// Authenticate - native async, no Future boxing
    fn authenticate(
        &self,
        request: Self::AuthRequest,
    ) -> impl std::future::Future<Output = Result<Self::AuthResponse>> + Send;

    /// Validate token - direct async method
    fn validate_token(&self, token: &str)
    -> impl std::future::Future<Output = Result<bool>> + Send;

    /// Compile-time constants
    /// Returns the maximum number of sessions.
    #[must_use]
    fn max_sessions() -> usize {
        MAX_SESSIONS
    }
    /// Returns the session duration in seconds.
    #[must_use]
    fn session_duration_seconds() -> u64 {
        SESSION_DURATION_SECS
    }
}

/// Native async MCP service trait - replaces #\[async_trait\] `McpService`
pub trait NativeAsyncMcpService<
    const MAX_CONNECTIONS: usize = 1000,
    const REQUEST_TIMEOUT_SECS: u64 = 300,
>: Send + Sync
{
    /// Type alias for Request
    type Request: Clone + Send + Sync + 'static;
    /// Type alias for Response
    type Response: Clone + Send + Sync + 'static;
    /// Process request - native async, no Future boxing
    fn process_request(
        &self,
        request: Self::Request,
    ) -> impl std::future::Future<Output = Result<Self::Response>> + Send;

    /// Performs a health check for the MCP service.
    ///
    /// # Returns
    /// A future that resolves to `Ok(true)` if healthy, `Ok(false)` otherwise, or an error.
    fn health_check(&self) -> impl std::future::Future<Output = Result<bool>> + Send;

    /// Compile-time constants
    /// Returns the maximum number of connections.
    #[must_use]
    fn max_connections() -> usize {
        MAX_CONNECTIONS
    }
    /// Returns the request timeout in seconds.
    #[must_use]
    fn request_timeout_seconds() -> u64 {
        REQUEST_TIMEOUT_SECS
    }
}

/// Native async workflow service trait - replaces #\[async_trait\] `WorkflowService`
pub trait NativeAsyncWorkflowService<
    const MAX_WORKFLOWS: usize = 1000,
    const EXECUTION_TIMEOUT_SECS: u64 = 300,
>: Send + Sync
{
    /// Type alias for Workflow
    type Workflow: Clone + Send + Sync + 'static;
    /// Type alias for ExecutionContext
    type ExecutionContext: Clone + Send + Sync + 'static;
    /// Execute workflow - native async, no Future boxing
    fn execute(
        &self,
        workflow: Self::Workflow,
    ) -> impl std::future::Future<Output = Result<Self::ExecutionContext>> + Send;

    /// Get execution status - direct async method
    fn get_status(
        &self,
        execution_id: &str,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Compile-time constants
    /// Returns the maximum number of workflows.
    #[must_use]
    fn max_workflows() -> usize {
        MAX_WORKFLOWS
    }
    /// Returns the execution timeout in seconds.
    #[must_use]
    fn execution_timeout_seconds() -> u64 {
        EXECUTION_TIMEOUT_SECS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockLoadBalancer;

    impl NativeAsyncLoadBalancer<100, 1000, 3600, 30> for MockLoadBalancer {
        type ServiceInfo = String;
        type ServiceRequest = String;
        type ServiceResponse = String;
        type LoadBalancerStats = String;
        type ServiceStats = String;

        fn add_service(
            &self,
            _service: Self::ServiceInfo,
        ) -> impl std::future::Future<Output = Result<()>> + Send {
            std::future::ready(Ok(()))
        }
        fn remove_service(
            &self,
            _service_id: &str,
        ) -> impl std::future::Future<Output = Result<()>> + Send {
            std::future::ready(Ok(()))
        }
        fn route_request(
            &self,
            _request: Self::ServiceRequest,
        ) -> impl std::future::Future<Output = Result<Self::ServiceResponse>> + Send {
            std::future::ready(Ok("response".to_string()))
        }
        fn get_stats(
            &self,
        ) -> impl std::future::Future<Output = Result<Self::LoadBalancerStats>> + Send {
            std::future::ready(Ok("stats".to_string()))
        }
        fn get_service_stats(
            &self,
            _service_id: &str,
        ) -> impl std::future::Future<Output = Result<Self::ServiceStats>> + Send {
            std::future::ready(Ok("svc_stats".to_string()))
        }
        fn health_check_all(
            &self,
        ) -> impl std::future::Future<Output = Result<Vec<(String, bool)>>> + Send {
            std::future::ready(Ok(vec![]))
        }
        fn update_service_weight(
            &self,
            _service_id: &str,
            _weight: f64,
        ) -> impl std::future::Future<Output = Result<()>> + Send {
            std::future::ready(Ok(()))
        }
        fn list_services(
            &self,
        ) -> impl std::future::Future<Output = Result<Vec<Self::ServiceInfo>>> + Send {
            std::future::ready(Ok(vec![]))
        }
        fn get_service(
            &self,
            _service_id: &str,
        ) -> impl std::future::Future<Output = Result<Option<Self::ServiceInfo>>> + Send {
            std::future::ready(Ok(None))
        }
        fn service_exists(
            &self,
            _service_id: &str,
        ) -> impl std::future::Future<Output = Result<bool>> + Send {
            std::future::ready(Ok(false))
        }
    }

    #[test]
    fn test_load_balancer_max_services() {
        assert_eq!(MockLoadBalancer::max_services(), 100);
    }

    #[test]
    fn test_load_balancer_max_concurrent_requests() {
        assert_eq!(MockLoadBalancer::max_concurrent_requests(), 1000);
    }

    #[test]
    fn test_load_balancer_stats_retention() {
        assert_eq!(MockLoadBalancer::stats_retention_seconds(), 3600);
    }

    #[test]
    fn test_load_balancer_health_check_interval() {
        assert_eq!(MockLoadBalancer::health_check_interval_seconds(), 30);
    }

    struct MockMcpService;

    impl NativeAsyncMcpService<500, 120> for MockMcpService {
        type Request = String;
        type Response = String;

        fn process_request(
            &self,
            _request: Self::Request,
        ) -> impl std::future::Future<Output = Result<Self::Response>> + Send {
            std::future::ready(Ok("ok".to_string()))
        }
        fn health_check(&self) -> impl std::future::Future<Output = Result<bool>> + Send {
            std::future::ready(Ok(true))
        }
    }

    #[test]
    fn test_mcp_service_max_connections() {
        assert_eq!(MockMcpService::max_connections(), 500);
    }

    #[test]
    fn test_mcp_service_request_timeout() {
        assert_eq!(MockMcpService::request_timeout_seconds(), 120);
    }

    #[test]
    fn test_workflow_service_constants() {
        struct MockWorkflow;
        impl NativeAsyncWorkflowService<50, 60> for MockWorkflow {
            type Workflow = String;
            type ExecutionContext = String;

            fn execute(
                &self,
                _workflow: Self::Workflow,
            ) -> impl std::future::Future<Output = Result<Self::ExecutionContext>> + Send
            {
                std::future::ready(Ok("ctx".to_string()))
            }
            fn get_status(
                &self,
                _execution_id: &str,
            ) -> impl std::future::Future<Output = Result<String>> + Send {
                std::future::ready(Ok("running".to_string()))
            }
        }
        assert_eq!(MockWorkflow::max_workflows(), 50);
        assert_eq!(MockWorkflow::execution_timeout_seconds(), 60);
    }

    #[test]
    fn test_universal_service_provider_constants() {
        struct MockProvider;
        impl NativeAsyncUniversalServiceProvider<200, 600> for MockProvider {
            type ServiceDefinition = String;
            type ServiceInstance = String;

            fn register_service(
                &self,
                _def: Self::ServiceDefinition,
            ) -> impl std::future::Future<Output = Result<String>> + Send {
                std::future::ready(Ok("id".to_string()))
            }
            fn unregister_service(
                &self,
                _id: &str,
            ) -> impl std::future::Future<Output = Result<()>> + Send {
                std::future::ready(Ok(()))
            }
            fn get_service_instance(
                &self,
                _id: &str,
            ) -> impl std::future::Future<Output = Result<Option<Self::ServiceInstance>>> + Send
            {
                std::future::ready(Ok(None))
            }
            fn list_services(
                &self,
            ) -> impl std::future::Future<Output = Result<Vec<Self::ServiceDefinition>>> + Send
            {
                std::future::ready(Ok(vec![]))
            }
        }
        assert_eq!(MockProvider::max_services(), 200);
        assert_eq!(MockProvider::service_timeout_seconds(), 600);
    }
}
