// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Optimization operations with circuit breaker and retry logic.

use std::sync::Arc;

use crate::handlers::zfs::universal_zfs::service_enum::UniversalZfsServiceEnum;
use crate::handlers::zfs::universal_zfs::traits::UniversalZfsService;
use crate::handlers::zfs::universal_zfs_types::{UniversalZfsError, UniversalZfsResult};
use async_recursion::async_recursion;

use super::core::FailSafeZfsService;

/// Optimize
#[async_recursion]
pub async fn optimize(service: &FailSafeZfsService) -> UniversalZfsResult<String> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_optimize(fallback).await;
        }
        return Err(UniversalZfsError::CircuitBreakerOpen {
            backend: "zfs_optimization".to_string(),
        });
    }

    match dispatch_optimize(&service.primary).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_optimize(fallback).await
            } else {
                Err(e)
            }
        }
    }
}

#[async_recursion]
async fn dispatch_optimize(e: &Arc<UniversalZfsServiceEnum>) -> UniversalZfsResult<String> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.optimize().await,
        UniversalZfsServiceEnum::FailSafe(f) => optimize(f).await,
    }
}

/// Gets Optimization Analytics
#[async_recursion]
pub async fn get_optimization_analytics(
    service: &FailSafeZfsService,
) -> UniversalZfsResult<serde_json::Value> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_get_optimization_analytics(fallback).await;
        }
        return Err(UniversalZfsError::CircuitBreakerOpen {
            backend: "zfs_optimization".to_string(),
        });
    }

    match dispatch_get_optimization_analytics(&service.primary).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_get_optimization_analytics(fallback).await
            } else {
                Err(e)
            }
        }
    }
}

#[async_recursion]
async fn dispatch_get_optimization_analytics(
    e: &Arc<UniversalZfsServiceEnum>,
) -> UniversalZfsResult<serde_json::Value> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.get_optimization_analytics().await,
        UniversalZfsServiceEnum::FailSafe(f) => get_optimization_analytics(f).await,
    }
}

/// Predict Tier
#[async_recursion]
pub async fn predict_tier(
    service: &FailSafeZfsService,
    file_path: &str,
) -> UniversalZfsResult<String> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_predict_tier(fallback, file_path).await;
        }
        return Err(UniversalZfsError::CircuitBreakerOpen {
            backend: "zfs_optimization".to_string(),
        });
    }

    match dispatch_predict_tier(&service.primary, file_path).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_predict_tier(fallback, file_path).await
            } else {
                Err(e)
            }
        }
    }
}

#[async_recursion]
async fn dispatch_predict_tier(
    e: &Arc<UniversalZfsServiceEnum>,
    file_path: &str,
) -> UniversalZfsResult<String> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.predict_tier(file_path).await,
        UniversalZfsServiceEnum::FailSafe(f) => predict_tier(f, file_path).await,
    }
}

/// Gets Configuration
#[async_recursion]
pub async fn get_configuration(
    service: &FailSafeZfsService,
) -> UniversalZfsResult<serde_json::Value> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_get_configuration(fallback).await;
        }
        return Err(UniversalZfsError::CircuitBreakerOpen {
            backend: "zfs_optimization".to_string(),
        });
    }

    match dispatch_get_configuration(&service.primary).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_get_configuration(fallback).await
            } else {
                Err(e)
            }
        }
    }
}

#[async_recursion]
async fn dispatch_get_configuration(
    e: &Arc<UniversalZfsServiceEnum>,
) -> UniversalZfsResult<serde_json::Value> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.get_configuration().await,
        UniversalZfsServiceEnum::FailSafe(f) => get_configuration(f).await,
    }
}

/// Updates  Configuration
#[async_recursion]
pub async fn update_configuration(
    service: &FailSafeZfsService,
    config: serde_json::Value,
) -> UniversalZfsResult<()> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_update_configuration(fallback, config).await;
        }
        return Err(UniversalZfsError::CircuitBreakerOpen {
            backend: "zfs_optimization".to_string(),
        });
    }

    match dispatch_update_configuration(&service.primary, config.clone()).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_update_configuration(fallback, config).await
            } else {
                Err(e)
            }
        }
    }
}

#[async_recursion]
async fn dispatch_update_configuration(
    e: &Arc<UniversalZfsServiceEnum>,
    config: serde_json::Value,
) -> UniversalZfsResult<()> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.update_configuration(config.clone()).await,
        UniversalZfsServiceEnum::FailSafe(f) => update_configuration(f, config).await,
    }
}

/// Shutdown
#[async_recursion]
pub async fn shutdown(service: &FailSafeZfsService) -> UniversalZfsResult<()> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return dispatch_shutdown(fallback).await;
        }
        return Err(UniversalZfsError::CircuitBreakerOpen {
            backend: "zfs_optimization".to_string(),
        });
    }

    match dispatch_shutdown(&service.primary).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                dispatch_shutdown(fallback).await
            } else {
                Err(e)
            }
        }
    }
}

#[async_recursion]
async fn dispatch_shutdown(e: &Arc<UniversalZfsServiceEnum>) -> UniversalZfsResult<()> {
    match e.as_ref() {
        UniversalZfsServiceEnum::Native(n) => n.shutdown().await,
        UniversalZfsServiceEnum::FailSafe(f) => shutdown(f).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::zfs::universal_zfs::fail_safe::core::FailSafeZfsService;
    use crate::handlers::zfs::universal_zfs::service_enum::UniversalZfsServiceEnum;
    use crate::handlers::zfs::universal_zfs_types::UniversalZfsError;
    use nestgate_core::config::canonical_primary::handler_config::ZfsFailSafeConfig;
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
    async fn optimize_circuit_open_returns_breaker_error() {
        let svc = service_with_open_circuit_no_fallback().await;
        let r = optimize(&svc).await;
        assert!(matches!(
            r,
            Err(UniversalZfsError::CircuitBreakerOpen { .. })
        ));
    }

    #[tokio::test]
    async fn get_optimization_analytics_circuit_open_returns_breaker_error() {
        let svc = service_with_open_circuit_no_fallback().await;
        let r = get_optimization_analytics(&svc).await;
        assert!(matches!(
            r,
            Err(UniversalZfsError::CircuitBreakerOpen { .. })
        ));
    }

    #[tokio::test]
    async fn predict_tier_circuit_open_returns_breaker_error() {
        let svc = service_with_open_circuit_no_fallback().await;
        let r = predict_tier(&svc, "/tmp/x").await;
        assert!(matches!(
            r,
            Err(UniversalZfsError::CircuitBreakerOpen { .. })
        ));
    }

    #[tokio::test]
    async fn get_configuration_circuit_open_returns_breaker_error() {
        let svc = service_with_open_circuit_no_fallback().await;
        let r = get_configuration(&svc).await;
        assert!(matches!(
            r,
            Err(UniversalZfsError::CircuitBreakerOpen { .. })
        ));
    }

    #[tokio::test]
    async fn update_configuration_circuit_open_returns_breaker_error() {
        let svc = service_with_open_circuit_no_fallback().await;
        let r = update_configuration(&svc, serde_json::json!({})).await;
        assert!(matches!(
            r,
            Err(UniversalZfsError::CircuitBreakerOpen { .. })
        ));
    }

    #[tokio::test]
    async fn shutdown_circuit_open_returns_breaker_error() {
        let svc = service_with_open_circuit_no_fallback().await;
        let r = shutdown(&svc).await;
        assert!(matches!(
            r,
            Err(UniversalZfsError::CircuitBreakerOpen { .. })
        ));
    }
}
