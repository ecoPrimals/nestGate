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
