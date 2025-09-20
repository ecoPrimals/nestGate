//
// Pool operations with circuit breaker and retry logic.

use crate::handlers::zfs::universal_zfs::traits::UniversalZfsService;
use crate::handlers::zfs::universal_zfs::types::{
    PoolConfig, PoolInfo, UniversalZfsError, UniversalZfsResult,
};

use super::core::FailSafeZfsService;

pub async fn list_pools(service: &FailSafeZfsService) -> UniversalZfsResult<Vec<PoolInfo>> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        // Try fallback service if available
        if let Some(fallback) = &service.fallback {
            return fallback.list_pools().await;
        } else {
            return Err(UniversalZfsError::internal(
                "Circuit breaker open and no fallback available",
            ));
        }
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

pub async fn get_pool(
    service: &FailSafeZfsService,
    name: &str,
) -> UniversalZfsResult<Option<PoolInfo>> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.get_pool(name).await;
        } else {
            return Err(UniversalZfsError::internal(
                "Circuit breaker open and no fallback available",
            ));
        }
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

pub async fn create_pool(
    service: &FailSafeZfsService,
    config: &PoolConfig,
) -> UniversalZfsResult<PoolInfo> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.create_pool(config).await;
        } else {
            return Err(UniversalZfsError::internal(
                "Circuit breaker open and no fallback available",
            ));
        }
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

pub async fn destroy_pool(service: &FailSafeZfsService, name: &str) -> UniversalZfsResult<()> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.destroy_pool(name).await;
        } else {
            return Err(UniversalZfsError::internal(
                "Circuit breaker open and no fallback available",
            ));
        }
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

pub async fn scrub_pool(service: &FailSafeZfsService, name: &str) -> UniversalZfsResult<()> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.scrub_pool(name).await;
        } else {
            return Err(UniversalZfsError::internal(
                "Circuit breaker open and no fallback available",
            ));
        }
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

pub async fn get_pool_status(
    service: &FailSafeZfsService,
    name: &str,
) -> UniversalZfsResult<String> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.get_pool_status(name).await;
        } else {
            return Err(UniversalZfsError::internal(
                "Circuit breaker open and no fallback available",
            ));
        }
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
