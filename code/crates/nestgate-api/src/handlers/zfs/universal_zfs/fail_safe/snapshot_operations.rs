//
// Snapshot operations with circuit breaker and retry logic.

use crate::handlers::zfs::universal_zfs::traits::UniversalZfsService;
use crate::handlers::zfs::universal_zfs::types::{
    SnapshotConfig, SnapshotInfo, UniversalZfsError, UniversalZfsResult,
};

use super::core::FailSafeZfsService;

pub async fn list_snapshots(service: &FailSafeZfsService) -> UniversalZfsResult<Vec<SnapshotInfo>> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        // Try fallback service if available
        if let Some(fallback) = &service.fallback {
            return fallback.list_snapshots().await;
        } else {
            return Err(UniversalZfsError::internal(
                "Circuit breaker open and no fallback available",
            ));
        }
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

pub async fn list_dataset_snapshots(
    service: &FailSafeZfsService,
    dataset: &str,
) -> UniversalZfsResult<Vec<SnapshotInfo>> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.list_dataset_snapshots(dataset).await;
        } else {
            return Err(UniversalZfsError::internal(
                "Circuit breaker open and no fallback available",
            ));
        }
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

pub async fn create_snapshot(
    service: &FailSafeZfsService,
    config: &SnapshotConfig,
) -> UniversalZfsResult<SnapshotInfo> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        // Try fallback service if available
        if let Some(fallback) = &service.fallback {
            return fallback.create_snapshot(config).await;
        } else {
            return Err(UniversalZfsError::internal(
                "Circuit breaker open and no fallback available",
            ));
        }
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

pub async fn destroy_snapshot(service: &FailSafeZfsService, name: &str) -> UniversalZfsResult<()> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.destroy_snapshot(name).await;
        } else {
            return Err(UniversalZfsError::internal(
                "Circuit breaker open and no fallback available",
            ));
        }
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
