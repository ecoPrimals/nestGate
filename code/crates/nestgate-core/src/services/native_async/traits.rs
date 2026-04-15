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

    struct MockCommunication;

    impl NativeAsyncCommunicationProvider<50, 512, 10, 2> for MockCommunication {
        type Message = String;
        type Address = String;
        type ConnectionInfo = String;

        fn send_message(
            &self,
            _endpoint: Self::Address,
            _message: Self::Message,
        ) -> impl std::future::Future<Output = Result<()>> + Send {
            std::future::ready(Ok(()))
        }
        fn receive_message(
            &self,
        ) -> impl std::future::Future<Output = Result<Self::Message>> + Send {
            std::future::ready(Ok("m".into()))
        }
        fn connect(
            &self,
            _endpoint: Self::Address,
        ) -> impl std::future::Future<Output = Result<Self::ConnectionInfo>> + Send {
            std::future::ready(Ok("conn".into()))
        }
        fn disconnect(
            &self,
            _connection: &Self::ConnectionInfo,
        ) -> impl std::future::Future<Output = Result<()>> + Send {
            std::future::ready(Ok(()))
        }
        fn connection_status(
            &self,
            _connection: &Self::ConnectionInfo,
        ) -> impl std::future::Future<Output = Result<String>> + Send {
            std::future::ready(Ok("open".into()))
        }
        fn broadcast(
            &self,
            _message: Self::Message,
        ) -> impl std::future::Future<Output = Result<u32>> + Send {
            std::future::ready(Ok(1))
        }
        fn list_connections(
            &self,
        ) -> impl std::future::Future<Output = Result<Vec<Self::ConnectionInfo>>> + Send {
            std::future::ready(Ok(vec![]))
        }
        fn ping(
            &self,
            _connection: &Self::ConnectionInfo,
        ) -> impl std::future::Future<Output = Result<std::time::Duration>> + Send {
            std::future::ready(Ok(std::time::Duration::from_millis(1)))
        }
    }

    #[test]
    fn communication_provider_constants() {
        assert_eq!(MockCommunication::max_connections(), 50);
        assert_eq!(MockCommunication::max_message_size(), 512);
        assert_eq!(MockCommunication::connection_timeout_seconds(), 10);
        assert_eq!(MockCommunication::message_retry_attempts(), 2);
    }

    #[tokio::test]
    async fn communication_provider_async_methods() {
        let c = MockCommunication;
        c.send_message("a".into(), "hi".into()).await.expect("send");
        c.receive_message().await.expect("recv");
        c.connect("ep".into()).await.expect("connect");
        c.disconnect(&"conn".into()).await.expect("disc");
        c.connection_status(&"conn".into()).await.expect("st");
        assert_eq!(c.broadcast("x".into()).await.expect("bc"), 1);
        c.list_connections().await.expect("list");
        c.ping(&"conn".into()).await.expect("ping");
    }

    struct MockMcpProtocol;

    impl NativeAsyncMCPProtocolHandler<10, 60, 2048, 2> for MockMcpProtocol {
        type SessionInfo = String;
        type Message = String;
        type Response = String;
        type Error = String;

        fn create_session(
            &self,
            _client_id: String,
        ) -> impl std::future::Future<Output = Result<Self::SessionInfo>> + Send {
            std::future::ready(Ok("sess".into()))
        }
        fn close_session(
            &self,
            _session_id: &str,
        ) -> impl std::future::Future<Output = Result<()>> + Send {
            std::future::ready(Ok(()))
        }
        fn send_mcp_message(
            &self,
            _session_id: &str,
            _message: Self::Message,
        ) -> impl std::future::Future<Output = Result<Self::Response>> + Send {
            std::future::ready(Ok("r".into()))
        }
        fn handle_message(
            &self,
            _session_id: &str,
            _message: Self::Message,
        ) -> impl std::future::Future<Output = Result<Self::Response>> + Send {
            std::future::ready(Ok("r2".into()))
        }
        fn get_session(
            &self,
            _session_id: &str,
        ) -> impl std::future::Future<Output = Result<Option<Self::SessionInfo>>> + Send {
            std::future::ready(Ok(None))
        }
        fn list_sessions(
            &self,
        ) -> impl std::future::Future<Output = Result<Vec<Self::SessionInfo>>> + Send {
            std::future::ready(Ok(vec![]))
        }
        fn ping_session(
            &self,
            _session_id: &str,
        ) -> impl std::future::Future<Output = Result<std::time::Duration>> + Send {
            std::future::ready(Ok(std::time::Duration::from_millis(2)))
        }
    }

    #[test]
    fn mcp_protocol_handler_constants() {
        assert_eq!(MockMcpProtocol::max_sessions(), 10);
        assert_eq!(MockMcpProtocol::session_timeout_seconds(), 60);
        assert_eq!(MockMcpProtocol::max_message_size(), 2048);
        assert_eq!(MockMcpProtocol::protocol_version(), 2);
    }

    #[tokio::test]
    async fn mcp_protocol_handler_async_methods() {
        let h = MockMcpProtocol;
        h.create_session("c".into()).await.expect("create");
        h.close_session("s").await.expect("close");
        h.send_mcp_message("s", "m".into()).await.expect("send");
        h.handle_message("s", "m".into()).await.expect("handle");
        h.get_session("s").await.expect("get");
        h.list_sessions().await.expect("list");
        h.ping_session("s").await.expect("ping");
    }

    struct MockAutomation;

    impl NativeAsyncAutomationService<20, 5, 120, 50> for MockAutomation {
        type WorkflowDefinition = String;
        type WorkflowExecution = String;
        type ExecutionResult = String;

        fn create_workflow(
            &self,
            _definition: Self::WorkflowDefinition,
        ) -> impl std::future::Future<Output = Result<String>> + Send {
            std::future::ready(Ok("wf".into()))
        }
        fn execute_workflow(
            &self,
            _workflow_id: &str,
            _parameters: std::collections::HashMap<String, serde_json::Value>,
        ) -> impl std::future::Future<Output = Result<Self::WorkflowExecution>> + Send {
            std::future::ready(Ok("ex".into()))
        }
        fn stop_execution(
            &self,
            _execution_id: &str,
        ) -> impl std::future::Future<Output = Result<()>> + Send {
            std::future::ready(Ok(()))
        }
        fn get_execution_status(
            &self,
            _execution_id: &str,
        ) -> impl std::future::Future<Output = Result<String>> + Send {
            std::future::ready(Ok("running".into()))
        }
        fn list_executions(
            &self,
        ) -> impl std::future::Future<Output = Result<Vec<Self::WorkflowExecution>>> + Send
        {
            std::future::ready(Ok(vec![]))
        }
        fn get_execution_result(
            &self,
            _execution_id: &str,
        ) -> impl std::future::Future<Output = Result<Self::ExecutionResult>> + Send {
            std::future::ready(Ok("done".into()))
        }
    }

    #[test]
    fn automation_service_constants() {
        assert_eq!(MockAutomation::max_workflows(), 20);
        assert_eq!(MockAutomation::max_concurrent_executions(), 5);
        assert_eq!(MockAutomation::execution_timeout_seconds(), 120);
        assert_eq!(MockAutomation::max_workflow_steps(), 50);
    }

    #[tokio::test]
    async fn automation_service_async_methods() {
        let a = MockAutomation;
        a.create_workflow("d".into()).await.expect("create");
        a.execute_workflow("w", std::collections::HashMap::new())
            .await
            .expect("exec");
        a.stop_execution("e").await.expect("stop");
        a.get_execution_status("e").await.expect("st");
        a.list_executions().await.expect("list");
        a.get_execution_result("e").await.expect("res");
    }

    struct MockSecurity;

    #[expect(
        deprecated,
        reason = "Exercises deprecated NativeAsyncSecurityService for coverage"
    )]
    impl NativeAsyncSecurityService<100, 200> for MockSecurity {
        type AuthRequest = String;
        type AuthResponse = String;

        fn authenticate(
            &self,
            _request: Self::AuthRequest,
        ) -> impl std::future::Future<Output = Result<Self::AuthResponse>> + Send {
            std::future::ready(Ok("tok".into()))
        }
        fn validate_token(
            &self,
            _token: &str,
        ) -> impl std::future::Future<Output = Result<bool>> + Send {
            std::future::ready(Ok(true))
        }
    }

    #[test]
    #[expect(
        deprecated,
        reason = "Exercises deprecated NativeAsyncSecurityService for coverage"
    )]
    fn security_service_constants() {
        assert_eq!(MockSecurity::max_sessions(), 100);
        assert_eq!(MockSecurity::session_duration_seconds(), 200);
    }

    #[tokio::test]
    #[expect(
        deprecated,
        reason = "Exercises deprecated NativeAsyncSecurityService for coverage"
    )]
    async fn security_service_async_methods() {
        let s = MockSecurity;
        s.authenticate("req".into()).await.expect("auth");
        s.validate_token("t").await.expect("val");
    }

    #[tokio::test]
    async fn load_balancer_async_surface() {
        let lb = MockLoadBalancer;
        lb.add_service("svc".into()).await.expect("add");
        lb.remove_service("svc").await.expect("rm");
        lb.route_request("r".into()).await.expect("route");
        lb.get_stats().await.expect("stats");
        lb.get_service_stats("svc").await.expect("svc_stats");
        lb.health_check_all().await.expect("health");
        lb.update_service_weight("svc", 1.0).await.expect("w");
        lb.list_services().await.expect("list");
        lb.get_service("svc").await.expect("get");
        lb.service_exists("svc").await.expect("ex");
    }
}
