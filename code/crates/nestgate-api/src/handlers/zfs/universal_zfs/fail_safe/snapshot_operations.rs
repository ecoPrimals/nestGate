// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Snapshot operations with circuit breaker and retry logic.

//! Snapshot Operations module

use std::sync::Arc;

use crate::handlers::zfs::universal_zfs::service_enum::UniversalZfsServiceEnum;
use crate::handlers::zfs::universal_zfs::traits::UniversalZfsService;
use crate::handlers::zfs::universal_zfs_types::{
    SnapshotConfig, SnapshotInfo, UniversalZfsError, UniversalZfsResult,
};
use async_recursion::async_recursion;

use super::core::FailSafeZfsService;

/// List Snapshots
#[async_recursion]
pub async fn list_snapshots(service: &FailSafeZfsService) -> UniversalZfsResult<Vec<SnapshotInfo>> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_list_snapshots(fallback).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    match dispatch_list_snapshots(&service.primary).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_list_snapshots(fallback).await
            } else {
                Err(e)
            }
        }
    }
}

#[async_recursion]
async fn dispatch_list_snapshots(
    e: &Arc<UniversalZfsServiceEnum>,
) -> UniversalZfsResult<Vec<SnapshotInfo>> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.list_snapshots().await,
        UniversalZfsServiceEnum::FailSafe(f) => list_snapshots(f).await,
    }
}

/// List Dataset Snapshots
#[async_recursion]
pub async fn list_dataset_snapshots(
    service: &FailSafeZfsService,
    dataset: &str,
) -> UniversalZfsResult<Vec<SnapshotInfo>> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_list_dataset_snapshots(fallback, dataset).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    match dispatch_list_dataset_snapshots(&service.primary, dataset).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_list_dataset_snapshots(fallback, dataset).await
            } else {
                Err(e)
            }
        }
    }
}

#[async_recursion]
async fn dispatch_list_dataset_snapshots(
    e: &Arc<UniversalZfsServiceEnum>,
    dataset: &str,
) -> UniversalZfsResult<Vec<SnapshotInfo>> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.list_dataset_snapshots(dataset).await,
        UniversalZfsServiceEnum::FailSafe(f) => list_dataset_snapshots(f, dataset).await,
    }
}

/// Creates  Snapshot
#[async_recursion]
pub async fn create_snapshot(
    service: &FailSafeZfsService,
    config: &SnapshotConfig,
) -> UniversalZfsResult<SnapshotInfo> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_create_snapshot(fallback, config).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    match dispatch_create_snapshot(&service.primary, config).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_create_snapshot(fallback, config).await
            } else {
                Err(e)
            }
        }
    }
}

#[async_recursion]
async fn dispatch_create_snapshot(
    e: &Arc<UniversalZfsServiceEnum>,
    config: &SnapshotConfig,
) -> UniversalZfsResult<SnapshotInfo> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.create_snapshot(config).await,
        UniversalZfsServiceEnum::FailSafe(f) => create_snapshot(f, config).await,
    }
}

/// Destroy Snapshot
#[async_recursion]
pub async fn destroy_snapshot(service: &FailSafeZfsService, name: &str) -> UniversalZfsResult<()> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_destroy_snapshot(fallback, name).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    match dispatch_destroy_snapshot(&service.primary, name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_destroy_snapshot(fallback, name).await
            } else {
                Err(e)
            }
        }
    }
}

#[async_recursion]
async fn dispatch_destroy_snapshot(
    e: &Arc<UniversalZfsServiceEnum>,
    name: &str,
) -> UniversalZfsResult<()> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.destroy_snapshot(name).await,
        UniversalZfsServiceEnum::FailSafe(f) => destroy_snapshot(f, name).await,
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
