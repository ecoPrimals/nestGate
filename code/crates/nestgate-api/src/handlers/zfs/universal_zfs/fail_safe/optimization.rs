//
// Optimization operations with circuit breaker and retry logic.

use crate::handlers::zfs::universal_zfs::traits::UniversalZfsService;
use crate::handlers::zfs::universal_zfs_types::{UniversalZfsError, UniversalZfsResult};

use super::core::FailSafeZfsService;

/// Optimize
pub async fn optimize(service: &FailSafeZfsService) -> UniversalZfsResult<String> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.optimize().await;
        }
        return Err(UniversalZfsError::CircuitBreakerOpen {
            backend: "zfs_optimization".to_string(),
        });
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.optimize().await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.optimize().await
            } else {
                Err(e)
            }
        }
    }
}

/// Gets Optimization Analytics
pub async fn get_optimization_analytics(
    service: &FailSafeZfsService,
) -> UniversalZfsResult<serde_json::Value> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.get_optimization_analytics().await;
        }
        return Err(UniversalZfsError::CircuitBreakerOpen {
            backend: "zfs_optimization".to_string(),
        });
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.get_optimization_analytics().await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.get_optimization_analytics().await
            } else {
                Err(e)
            }
        }
    }
}

/// Predict Tier
pub async fn predict_tier(
    service: &FailSafeZfsService,
    file_path: &str,
) -> UniversalZfsResult<String> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.predict_tier(file_path).await;
        }
        return Err(UniversalZfsError::CircuitBreakerOpen {
            backend: "zfs_optimization".to_string(),
        });
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.predict_tier(file_path).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.predict_tier(file_path).await
            } else {
                Err(e)
            }
        }
    }
}

/// Gets Configuration
pub async fn get_configuration(
    service: &FailSafeZfsService,
) -> UniversalZfsResult<serde_json::Value> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.get_configuration().await;
        }
        return Err(UniversalZfsError::CircuitBreakerOpen {
            backend: "zfs_optimization".to_string(),
        });
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.get_configuration().await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.get_configuration().await
            } else {
                Err(e)
            }
        }
    }
}

/// Updates  Configuration
pub async fn update_configuration(
    service: &FailSafeZfsService,
    config: serde_json::Value,
) -> UniversalZfsResult<()> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.update_configuration(config).await;
        }
        return Err(UniversalZfsError::CircuitBreakerOpen {
            backend: "zfs_optimization".to_string(),
        });
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.update_configuration(config.clone()).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.update_configuration(config).await
            } else {
                Err(e)
            }
        }
    }
}

/// Shutdown
pub async fn shutdown(service: &FailSafeZfsService) -> UniversalZfsResult<()> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.shutdown().await;
        }
        return Err(UniversalZfsError::CircuitBreakerOpen {
            backend: "zfs_optimization".to_string(),
        });
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.shutdown().await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.shutdown().await
            } else {
                Err(e)
            }
        }
    }
}
