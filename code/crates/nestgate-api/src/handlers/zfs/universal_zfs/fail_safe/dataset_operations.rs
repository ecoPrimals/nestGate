// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Dataset operations with circuit breaker and retry logic.

//! Dataset Operations module

use std::collections::HashMap;
use std::sync::Arc;

use crate::handlers::zfs::universal_zfs::service_enum::UniversalZfsServiceEnum;
use crate::handlers::zfs::universal_zfs::traits::UniversalZfsService;
use crate::handlers::zfs::universal_zfs_types::{
    DatasetConfig, DatasetInfo, UniversalZfsError, UniversalZfsResult,
};
use async_recursion::async_recursion;

use super::core::FailSafeZfsService;

/// List Datasets
#[async_recursion]
pub async fn list_datasets(service: &FailSafeZfsService) -> UniversalZfsResult<Vec<DatasetInfo>> {
    if !service.circuit_breaker.can_execute().await {
        return if let Some(fallback) = &service.fallback {
            dispatch_list_datasets(fallback).await
        } else {
            match service.execute_fallback_operation("list_datasets", &service.primary) {
                Ok(()) => Ok(Vec::new()),
                Err(_) => Err(UniversalZfsError::CircuitBreakerOpen {
                    backend: service.service_name.clone(),
                }),
            }
        };
    }

    let primary = service.primary.clone();
    let timeout_duration = service.timeout_config.operation_timeout;
    let result = service
        .retry_executor
        .execute(|| {
            let primary = primary.clone();
            Box::pin(async move {
                tokio::time::timeout(timeout_duration, dispatch_list_datasets(&primary))
                    .await
                    .map_err(|_| UniversalZfsError::timeout("list_datasets", timeout_duration))?
            })
        })
        .await;

    match &result {
        Ok(_) => service.circuit_breaker.record_success().await,
        Err(_) => service.circuit_breaker.record_failure().await,
    }

    service
        .update_metrics("list_datasets", result.is_ok())
        .await;

    result
}

#[async_recursion]
async fn dispatch_list_datasets(
    e: &Arc<UniversalZfsServiceEnum>,
) -> UniversalZfsResult<Vec<DatasetInfo>> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.list_datasets().await,
        UniversalZfsServiceEnum::FailSafe(f) => list_datasets(f).await,
    }
}

/// Gets Dataset
#[async_recursion]
pub async fn get_dataset(
    service: &FailSafeZfsService,
    name: &str,
) -> UniversalZfsResult<Option<DatasetInfo>> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_get_dataset(fallback, name).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    match dispatch_get_dataset(&service.primary, name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_get_dataset(fallback, name).await
            } else {
                Err(e)
            }
        }
    }
}

#[async_recursion]
async fn dispatch_get_dataset(
    e: &Arc<UniversalZfsServiceEnum>,
    name: &str,
) -> UniversalZfsResult<Option<DatasetInfo>> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.get_dataset(name).await,
        UniversalZfsServiceEnum::FailSafe(f) => get_dataset(f, name).await,
    }
}

/// Creates  Dataset
#[async_recursion]
pub async fn create_dataset(
    service: &FailSafeZfsService,
    config: &DatasetConfig,
) -> UniversalZfsResult<DatasetInfo> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_create_dataset(fallback, config).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    match dispatch_create_dataset(&service.primary, config).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_create_dataset(fallback, config).await
            } else {
                Err(e)
            }
        }
    }
}

#[async_recursion]
async fn dispatch_create_dataset(
    e: &Arc<UniversalZfsServiceEnum>,
    config: &DatasetConfig,
) -> UniversalZfsResult<DatasetInfo> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.create_dataset(config).await,
        UniversalZfsServiceEnum::FailSafe(f) => create_dataset(f, config).await,
    }
}

/// Destroy Dataset
#[async_recursion]
pub async fn destroy_dataset(service: &FailSafeZfsService, name: &str) -> UniversalZfsResult<()> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_destroy_dataset(fallback, name).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    match dispatch_destroy_dataset(&service.primary, name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_destroy_dataset(fallback, name).await
            } else {
                Err(e)
            }
        }
    }
}

#[async_recursion]
async fn dispatch_destroy_dataset(
    e: &Arc<UniversalZfsServiceEnum>,
    name: &str,
) -> UniversalZfsResult<()> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.destroy_dataset(name).await,
        UniversalZfsServiceEnum::FailSafe(f) => destroy_dataset(f, name).await,
    }
}

/// Gets Dataset Properties
#[async_recursion]
pub async fn get_dataset_properties(
    service: &FailSafeZfsService,
    name: &str,
) -> UniversalZfsResult<HashMap<String, String>> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_get_dataset_properties(fallback, name).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    match dispatch_get_dataset_properties(&service.primary, name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_get_dataset_properties(fallback, name).await
            } else {
                Err(e)
            }
        }
    }
}

#[async_recursion]
async fn dispatch_get_dataset_properties(
    e: &Arc<UniversalZfsServiceEnum>,
    name: &str,
) -> UniversalZfsResult<HashMap<String, String>> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.get_dataset_properties(name).await,
        UniversalZfsServiceEnum::FailSafe(f) => get_dataset_properties(f, name).await,
    }
}

/// Sets Dataset Properties
#[async_recursion]
pub async fn set_dataset_properties(
    service: &FailSafeZfsService,
    name: &str,
    properties: &HashMap<String, String>,
) -> UniversalZfsResult<()> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_set_dataset_properties(fallback, name, properties).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    match dispatch_set_dataset_properties(&service.primary, name, properties).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_set_dataset_properties(fallback, name, properties).await
            } else {
                Err(e)
            }
        }
    }
}

#[async_recursion]
async fn dispatch_set_dataset_properties(
    e: &Arc<UniversalZfsServiceEnum>,
    name: &str,
    properties: &HashMap<String, String>,
) -> UniversalZfsResult<()> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.set_dataset_properties(name, properties).await,
        UniversalZfsServiceEnum::FailSafe(f) => set_dataset_properties(f, name, properties).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::zfs::universal_zfs::fail_safe::core::FailSafeZfsService;
    use crate::handlers::zfs::universal_zfs::service_enum::UniversalZfsServiceEnum;
    use nestgate_core::config::canonical_primary::handler_config::ZfsFailSafeConfig;
    use std::sync::Arc;

    fn svc() -> FailSafeZfsService {
        let mut c = ZfsFailSafeConfig::default();
        c.circuit_breaker.enabled = false;
        FailSafeZfsService::new(Arc::new(UniversalZfsServiceEnum::new_native()), c)
    }

    #[tokio::test]
    async fn list_datasets_runs_against_primary() {
        let s = svc();
        let _ = list_datasets(&s).await;
    }

    #[tokio::test]
    async fn get_dataset_runs_against_primary() {
        let s = svc();
        let _ = get_dataset(&s, "nonexistent-dataset-xyz").await;
    }

    #[tokio::test]
    async fn get_dataset_properties_runs() {
        let s = svc();
        let _ = get_dataset_properties(&s, "tank/foo").await;
    }
}
