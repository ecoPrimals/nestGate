// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Async ZFS adapter behind the `dev-stubs` feature (`ProductionZfsService` uses real
//! [`nestgate_zfs::command::ZfsOperations`] when ZFS is present).

pub mod implementations;
pub mod traits;

pub use implementations::{DevelopmentZfsService, ProductionZfsService};
pub use traits::*;

/// Builds a production async ZFS service (queries real pools when ZFS is available).
#[must_use]
pub fn create_production_zfs_service() -> ProductionZfsService {
    ProductionZfsService::new()
}

/// Builds a fast mock ZFS service for local development.
#[must_use]
pub fn create_development_zfs_service() -> DevelopmentZfsService {
    DevelopmentZfsService::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::zfs::universal_zfs_types::{ServiceStatus, SnapshotConfig};

    #[tokio::test]
    async fn production_service_meta() {
        let service = create_production_zfs_service();
        assert_eq!(service.service_name(), "ProductionZfsService");
        assert_eq!(service.service_version(), "1.0.0");
    }

    #[tokio::test]
    async fn production_health_reports_status() {
        let service = create_production_zfs_service();
        let health = service.health_check().await.expect("health");
        assert!(
            matches!(
                health.status,
                ServiceStatus::Healthy | ServiceStatus::Unhealthy
            ),
            "expected concrete health status"
        );
    }

    #[tokio::test]
    async fn development_pool_roundtrip() {
        let service = create_development_zfs_service();
        let pools = service.list_pools().await.expect("pools");
        assert_eq!(pools.len(), 1);
        assert_eq!(pools[0].name, "dev-pool");
        let p = service.get_pool("dev-pool").await.expect("get");
        assert!(p.is_some());
    }

    #[tokio::test]
    async fn development_snapshot_config_roundtrip() {
        let service = create_development_zfs_service();
        let cfg = SnapshotConfig {
            name: "snap1".to_string(),
            dataset: "dev-pool/test".to_string(),
            properties: std::collections::HashMap::new(),
        };
        let s = service.create_snapshot(&cfg).await.expect("snap");
        assert_eq!(s.name, "dev-pool/test@snap1");
    }

    #[tokio::test]
    async fn production_metrics_follow_zfs_or_unavailable() {
        let service = create_production_zfs_service();
        let m = service.get_metrics().await;
        match m {
            Ok(metrics) => {
                assert!(metrics.custom_metrics.contains_key("pool_count"));
            }
            Err(e) => {
                let msg = e.to_string();
                assert!(
                    msg.contains("not available") || msg.contains("ZFS"),
                    "unexpected error: {msg}"
                );
            }
        }
    }
}
