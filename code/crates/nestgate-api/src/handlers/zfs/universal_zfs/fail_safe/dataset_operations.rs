// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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

use super::core::FailSafeZfsService;

/// List Datasets
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

async fn dispatch_list_datasets(
    e: &Arc<UniversalZfsServiceEnum>,
) -> UniversalZfsResult<Vec<DatasetInfo>> {
    let mut current = e.as_ref();
    loop {
        match current {
            UniversalZfsServiceEnum::Native(n) => return n.list_datasets().await,
            UniversalZfsServiceEnum::FailSafe(f) => {
                current = f.primary.as_ref();
            }
        }
    }
}

/// Gets Dataset
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

async fn dispatch_get_dataset(
    e: &Arc<UniversalZfsServiceEnum>,
    name: &str,
) -> UniversalZfsResult<Option<DatasetInfo>> {
    let mut current = e.as_ref();
    loop {
        match current {
            UniversalZfsServiceEnum::Native(n) => return n.get_dataset(name).await,
            UniversalZfsServiceEnum::FailSafe(f) => {
                current = f.primary.as_ref();
            }
        }
    }
}

/// Creates  Dataset
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

async fn dispatch_create_dataset(
    e: &Arc<UniversalZfsServiceEnum>,
    config: &DatasetConfig,
) -> UniversalZfsResult<DatasetInfo> {
    let mut current = e.as_ref();
    loop {
        match current {
            UniversalZfsServiceEnum::Native(n) => return n.create_dataset(config).await,
            UniversalZfsServiceEnum::FailSafe(f) => {
                current = f.primary.as_ref();
            }
        }
    }
}

/// Destroy Dataset
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

async fn dispatch_destroy_dataset(
    e: &Arc<UniversalZfsServiceEnum>,
    name: &str,
) -> UniversalZfsResult<()> {
    let mut current = e.as_ref();
    loop {
        match current {
            UniversalZfsServiceEnum::Native(n) => return n.destroy_dataset(name).await,
            UniversalZfsServiceEnum::FailSafe(f) => {
                current = f.primary.as_ref();
            }
        }
    }
}

/// Gets Dataset Properties
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

async fn dispatch_get_dataset_properties(
    e: &Arc<UniversalZfsServiceEnum>,
    name: &str,
) -> UniversalZfsResult<HashMap<String, String>> {
    let mut current = e.as_ref();
    loop {
        match current {
            UniversalZfsServiceEnum::Native(n) => return n.get_dataset_properties(name).await,
            UniversalZfsServiceEnum::FailSafe(f) => {
                current = f.primary.as_ref();
            }
        }
    }
}

/// Sets Dataset Properties
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

async fn dispatch_set_dataset_properties(
    e: &Arc<UniversalZfsServiceEnum>,
    name: &str,
    properties: &HashMap<String, String>,
) -> UniversalZfsResult<()> {
    let mut current = e.as_ref();
    loop {
        match current {
            UniversalZfsServiceEnum::Native(n) => {
                return n.set_dataset_properties(name, properties).await;
            }
            UniversalZfsServiceEnum::FailSafe(f) => {
                current = f.primary.as_ref();
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

    fn svc_cb(threshold: u32) -> FailSafeZfsService {
        let mut c = ZfsFailSafeConfig::default();
        c.circuit_breaker.enabled = true;
        c.failure_threshold = threshold;
        FailSafeZfsService::new(Arc::new(UniversalZfsServiceEnum::new_native()), c)
    }

    async fn open_circuit(s: &FailSafeZfsService, times: u32) {
        for _ in 0..times {
            s.circuit_breaker.record_failure().await;
        }
    }

    #[tokio::test]
    async fn list_datasets_circuit_open_no_fallback_returns_empty() {
        let s = svc_cb(1);
        open_circuit(&s, 1).await;
        let r = list_datasets(&s)
            .await
            .expect("test: empty list when breaker open");
        assert!(r.is_empty());
    }

    #[tokio::test]
    async fn get_dataset_circuit_open_no_fallback_errors() {
        let s = svc_cb(1);
        open_circuit(&s, 1).await;
        let e = get_dataset(&s, "any")
            .await
            .expect_err("test: breaker blocks get");
        assert!(
            format!("{e:?}").contains("Circuit") || format!("{e:?}").contains("fallback"),
            "{e:?}"
        );
    }

    #[tokio::test]
    async fn destroy_dataset_circuit_open_no_fallback_errors() {
        let s = svc_cb(1);
        open_circuit(&s, 1).await;
        assert!(destroy_dataset(&s, "tank/x").await.is_err());
    }

    #[tokio::test]
    async fn create_dataset_circuit_open_no_fallback_errors() {
        let s = svc_cb(1);
        open_circuit(&s, 1).await;
        let cfg = DatasetConfig {
            name: "tank/new_ds_cov".into(),
            mountpoint: None,
            compression: false,
            quota: None,
            reservation: None,
            properties: std::collections::HashMap::new(),
        };
        assert!(create_dataset(&s, &cfg).await.is_err());
    }

    #[tokio::test]
    async fn set_dataset_properties_circuit_open_no_fallback_errors() {
        let s = svc_cb(1);
        open_circuit(&s, 1).await;
        let mut m = std::collections::HashMap::new();
        m.insert("atime".into(), "off".into());
        assert!(set_dataset_properties(&s, "tank/x", &m).await.is_err());
    }

    #[tokio::test]
    async fn get_dataset_circuit_open_with_fallback_dispatches_fallback() {
        let mut c = ZfsFailSafeConfig::default();
        c.circuit_breaker.enabled = true;
        c.failure_threshold = 1;
        let primary = Arc::new(UniversalZfsServiceEnum::new_native());
        let fallback = Arc::new(UniversalZfsServiceEnum::new_native());
        let s = FailSafeZfsService::new(primary, c).with_fallback(fallback);
        open_circuit(&s, 1).await;
        let _ = get_dataset(&s, "nonexistent-dataset-fallback-cov").await;
    }

    #[tokio::test]
    async fn dispatch_unwraps_nested_fail_safe_to_native() {
        let native = Arc::new(UniversalZfsServiceEnum::new_native());
        let mut inner_cfg = ZfsFailSafeConfig::default();
        inner_cfg.circuit_breaker.enabled = false;
        let inner = FailSafeZfsService::new(native, inner_cfg);
        let wrapped = Arc::new(UniversalZfsServiceEnum::FailSafe(inner));
        let mut outer_cfg = ZfsFailSafeConfig::default();
        outer_cfg.circuit_breaker.enabled = false;
        let s = FailSafeZfsService::new(wrapped, outer_cfg);
        let _ = list_datasets(&s).await;
        let _ = get_dataset(&s, "z").await;
    }
}
