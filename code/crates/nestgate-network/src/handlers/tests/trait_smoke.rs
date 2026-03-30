// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use crate::handlers::NetworkService;
use crate::types::ServiceStatus;

#[tokio::test]
async fn test_network_service_noop_impl() {
    struct NoopNet;
    impl NetworkService for NoopNet {
        async fn start_service(&self) -> nestgate_core::Result<()> {
            Ok(())
        }
        async fn stop_service(&self) -> nestgate_core::Result<()> {
            Ok(())
        }
        async fn get_status(&self) -> nestgate_core::Result<ServiceStatus> {
            Ok(ServiceStatus::Running)
        }
        async fn allocate_port_for_service(
            &self,
            _service_name: &str,
        ) -> nestgate_core::Result<u16> {
            Ok(9000)
        }
        async fn release_service_port(&self, _port: u16) -> nestgate_core::Result<()> {
            Ok(())
        }
    }

    let n = NoopNet;
    assert!(n.start_service().await.is_ok());
    assert!(n.stop_service().await.is_ok());
    assert_eq!(
        n.get_status().await.expect("status"),
        ServiceStatus::Running
    );
    assert_eq!(
        n.allocate_port_for_service("svc").await.expect("port"),
        9000
    );
    assert!(n.release_service_port(9000).await.is_ok());
}
