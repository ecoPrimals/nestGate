// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Pool operations with circuit breaker and retry logic.

//! Pool Operations module

use std::sync::Arc;

use crate::handlers::zfs::universal_zfs::service_enum::UniversalZfsServiceEnum;
use crate::handlers::zfs::universal_zfs::traits::UniversalZfsService;
use crate::handlers::zfs::universal_zfs_types::{
    PoolConfig, PoolInfo, UniversalZfsError, UniversalZfsResult,
};

use super::core::FailSafeZfsService;

/// List Pools
pub async fn list_pools(service: &FailSafeZfsService) -> UniversalZfsResult<Vec<PoolInfo>> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_list_pools(fallback).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    match dispatch_list_pools(&service.primary).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_list_pools(fallback).await
            } else {
                Err(e)
            }
        }
    }
}

async fn dispatch_list_pools(
    e: &Arc<UniversalZfsServiceEnum>,
) -> UniversalZfsResult<Vec<PoolInfo>> {
    let mut current = e.as_ref();
    loop {
        match current {
            UniversalZfsServiceEnum::Native(n) => return n.list_pools().await,
            UniversalZfsServiceEnum::FailSafe(f) => {
                current = f.primary.as_ref();
            }
        }
    }
}

/// Gets Pool
pub async fn get_pool(
    service: &FailSafeZfsService,
    name: &str,
) -> UniversalZfsResult<Option<PoolInfo>> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_get_pool(fallback, name).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    match dispatch_get_pool(&service.primary, name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_get_pool(fallback, name).await
            } else {
                Err(e)
            }
        }
    }
}

async fn dispatch_get_pool(
    e: &Arc<UniversalZfsServiceEnum>,
    name: &str,
) -> UniversalZfsResult<Option<PoolInfo>> {
    let mut current = e.as_ref();
    loop {
        match current {
            UniversalZfsServiceEnum::Native(n) => return n.get_pool(name).await,
            UniversalZfsServiceEnum::FailSafe(f) => {
                current = f.primary.as_ref();
            }
        }
    }
}

/// Creates  Pool
pub async fn create_pool(
    service: &FailSafeZfsService,
    config: &PoolConfig,
) -> UniversalZfsResult<PoolInfo> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_create_pool(fallback, config).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    match dispatch_create_pool(&service.primary, config).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_create_pool(fallback, config).await
            } else {
                Err(e)
            }
        }
    }
}

async fn dispatch_create_pool(
    e: &Arc<UniversalZfsServiceEnum>,
    config: &PoolConfig,
) -> UniversalZfsResult<PoolInfo> {
    let mut current = e.as_ref();
    loop {
        match current {
            UniversalZfsServiceEnum::Native(n) => return n.create_pool(config).await,
            UniversalZfsServiceEnum::FailSafe(f) => {
                current = f.primary.as_ref();
            }
        }
    }
}

/// Destroy Pool
pub async fn destroy_pool(service: &FailSafeZfsService, name: &str) -> UniversalZfsResult<()> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_destroy_pool(fallback, name).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    match dispatch_destroy_pool(&service.primary, name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_destroy_pool(fallback, name).await
            } else {
                Err(e)
            }
        }
    }
}

async fn dispatch_destroy_pool(
    e: &Arc<UniversalZfsServiceEnum>,
    name: &str,
) -> UniversalZfsResult<()> {
    let mut current = e.as_ref();
    loop {
        match current {
            UniversalZfsServiceEnum::Native(n) => return n.destroy_pool(name).await,
            UniversalZfsServiceEnum::FailSafe(f) => {
                current = f.primary.as_ref();
            }
        }
    }
}

/// Scrub Pool
pub async fn scrub_pool(service: &FailSafeZfsService, name: &str) -> UniversalZfsResult<()> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_scrub_pool(fallback, name).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    match dispatch_scrub_pool(&service.primary, name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_scrub_pool(fallback, name).await
            } else {
                Err(e)
            }
        }
    }
}

async fn dispatch_scrub_pool(
    e: &Arc<UniversalZfsServiceEnum>,
    name: &str,
) -> UniversalZfsResult<()> {
    let mut current = e.as_ref();
    loop {
        match current {
            UniversalZfsServiceEnum::Native(n) => return n.scrub_pool(name).await,
            UniversalZfsServiceEnum::FailSafe(f) => {
                current = f.primary.as_ref();
            }
        }
    }
}

/// Gets Pool Status
pub async fn get_pool_status(
    service: &FailSafeZfsService,
    name: &str,
) -> UniversalZfsResult<String> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_get_pool_status(fallback, name).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    match dispatch_get_pool_status(&service.primary, name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_get_pool_status(fallback, name).await
            } else {
                Err(e)
            }
        }
    }
}

async fn dispatch_get_pool_status(
    e: &Arc<UniversalZfsServiceEnum>,
    name: &str,
) -> UniversalZfsResult<String> {
    let mut current = e.as_ref();
    loop {
        match current {
            UniversalZfsServiceEnum::Native(n) => return n.get_pool_status(name).await,
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
    async fn list_pools_circuit_open_without_fallback_errors() {
        let svc = service_with_open_circuit_no_fallback().await;
        let r = list_pools(&svc).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn get_pool_circuit_open_without_fallback_errors() {
        let svc = service_with_open_circuit_no_fallback().await;
        let r = get_pool(&svc, "tank").await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn create_pool_circuit_open_without_fallback_errors() {
        let svc = service_with_open_circuit_no_fallback().await;
        let cfg = PoolConfig {
            name: "test_pool".to_string(),
            devices: vec![],
            mountpoint: None,
            compression: false,
            deduplication: false,
            properties: HashMap::new(),
        };
        let r = create_pool(&svc, &cfg).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn destroy_pool_circuit_open_without_fallback_errors() {
        let svc = service_with_open_circuit_no_fallback().await;
        let r = destroy_pool(&svc, "tank").await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn scrub_pool_circuit_open_without_fallback_errors() {
        let svc = service_with_open_circuit_no_fallback().await;
        let r = scrub_pool(&svc, "tank").await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn get_pool_status_circuit_open_without_fallback_errors() {
        let svc = service_with_open_circuit_no_fallback().await;
        let r = get_pool_status(&svc, "tank").await;
        assert!(r.is_err());
    }
}
