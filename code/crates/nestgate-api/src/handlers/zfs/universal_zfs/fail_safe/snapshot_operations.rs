// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Snapshot operations with circuit breaker and retry logic.

//! Snapshot Operations module

use crate::handlers::zfs::universal_zfs::traits::UniversalZfsService;
use crate::handlers::zfs::universal_zfs_types::{
    SnapshotConfig, SnapshotInfo, UniversalZfsError, UniversalZfsResult,
};

use super::core::FailSafeZfsService;

/// List Snapshots
pub async fn list_snapshots(service: &FailSafeZfsService) -> UniversalZfsResult<Vec<SnapshotInfo>> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        // Try fallback service if available
        if let Some(fallback) = &service.fallback {
            return fallback.list_snapshots().await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.list_snapshots().await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            // Try fallback if available
            if let Some(fallback) = &service.fallback {
                fallback.list_snapshots().await
            } else {
                Err(e)
            }
        }
    }
}

/// List Dataset Snapshots
pub async fn list_dataset_snapshots(
    service: &FailSafeZfsService,
    dataset: &str,
) -> UniversalZfsResult<Vec<SnapshotInfo>> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.list_dataset_snapshots(dataset).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.list_dataset_snapshots(dataset).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.list_dataset_snapshots(dataset).await
            } else {
                Err(e)
            }
        }
    }
}

/// Creates  Snapshot
pub async fn create_snapshot(
    service: &FailSafeZfsService,
    config: &SnapshotConfig,
) -> UniversalZfsResult<SnapshotInfo> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        // Try fallback service if available
        if let Some(fallback) = &service.fallback {
            return fallback.create_snapshot(config).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.create_snapshot(config).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            // Try fallback if available
            if let Some(fallback) = &service.fallback {
                fallback.create_snapshot(config).await
            } else {
                Err(e)
            }
        }
    }
}

/// Destroy Snapshot
pub async fn destroy_snapshot(service: &FailSafeZfsService, name: &str) -> UniversalZfsResult<()> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.destroy_snapshot(name).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.destroy_snapshot(name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.destroy_snapshot(name).await
            } else {
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::zfs::universal_zfs::fail_safe::core::FailSafeZfsService;
    use crate::handlers::zfs::universal_zfs::service_enum::UniversalZfsServiceEnum;
    use nestgate_core::config::canonical_primary::handler_config::ZfsFailSafeConfig;
    use std::collections::HashMap;
    use std::sync::Arc;

    async fn service_with_open_circuit_no_fallback() -> FailSafeZfsService {
        let mut c = ZfsFailSafeConfig::default();
        c.circuit_breaker.enabled = true;
        c.failure_threshold = 1;
        let primary = Arc::new(UniversalZfsServiceEnum::new_native());
        let svc = FailSafeZfsService::new(primary, c);
        svc.circuit_breaker.record_failure().await;
        svc
    }

    #[tokio::test]
    async fn list_snapshots_circuit_open_without_fallback_errors() {
        let svc = service_with_open_circuit_no_fallback().await;
        let r = list_snapshots(&svc).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn list_dataset_snapshots_circuit_open_without_fallback_errors() {
        let svc = service_with_open_circuit_no_fallback().await;
        let r = list_dataset_snapshots(&svc, "tank/fs").await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn create_snapshot_circuit_open_without_fallback_errors() {
        let svc = service_with_open_circuit_no_fallback().await;
        let cfg = SnapshotConfig {
            name: "snap".to_string(),
            dataset: "tank/fs".to_string(),
            properties: HashMap::new(),
        };
        let r = create_snapshot(&svc, &cfg).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn destroy_snapshot_circuit_open_without_fallback_errors() {
        let svc = service_with_open_circuit_no_fallback().await;
        let r = destroy_snapshot(&svc, "tank/fs@snap").await;
        assert!(r.is_err());
    }
}
