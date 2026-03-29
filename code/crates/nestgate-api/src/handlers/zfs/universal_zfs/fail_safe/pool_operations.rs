// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Pool operations with circuit breaker and retry logic.

//! Pool Operations module

use crate::handlers::zfs::universal_zfs::traits::UniversalZfsService;
use crate::handlers::zfs::universal_zfs_types::{
    PoolConfig, PoolInfo, UniversalZfsError, UniversalZfsResult,
};

use super::core::FailSafeZfsService;

/// List Pools
pub async fn list_pools(service: &FailSafeZfsService) -> UniversalZfsResult<Vec<PoolInfo>> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        // Try fallback service if available
        if let Some(fallback) = &service.fallback {
            return fallback.list_pools().await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.list_pools().await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            // Try fallback if available
            if let Some(fallback) = &service.fallback {
                fallback.list_pools().await
            } else {
                Err(e)
            }
        }
    }
}

/// Gets Pool
pub async fn get_pool(
    service: &FailSafeZfsService,
    name: &str,
) -> UniversalZfsResult<Option<PoolInfo>> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.get_pool(name).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.get_pool(name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.get_pool(name).await
            } else {
                Err(e)
            }
        }
    }
}

/// Creates  Pool
pub async fn create_pool(
    service: &FailSafeZfsService,
    config: &PoolConfig,
) -> UniversalZfsResult<PoolInfo> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.create_pool(config).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.create_pool(config).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.create_pool(config).await
            } else {
                Err(e)
            }
        }
    }
}

/// Destroy Pool
pub async fn destroy_pool(service: &FailSafeZfsService, name: &str) -> UniversalZfsResult<()> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.destroy_pool(name).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.destroy_pool(name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.destroy_pool(name).await
            } else {
                Err(e)
            }
        }
    }
}

/// Scrub Pool
pub async fn scrub_pool(service: &FailSafeZfsService, name: &str) -> UniversalZfsResult<()> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.scrub_pool(name).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.scrub_pool(name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.scrub_pool(name).await
            } else {
                Err(e)
            }
        }
    }
}

/// Gets Pool Status
pub async fn get_pool_status(
    service: &FailSafeZfsService,
    name: &str,
) -> UniversalZfsResult<String> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.get_pool_status(name).await;
        }
        return Err(UniversalZfsError::internal(
            "Circuit breaker open and no fallback available",
        ));
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.get_pool_status(name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.get_pool_status(name).await
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
