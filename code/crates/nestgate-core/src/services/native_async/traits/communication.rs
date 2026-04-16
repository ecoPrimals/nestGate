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
