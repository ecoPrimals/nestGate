// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use crate::Result;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
