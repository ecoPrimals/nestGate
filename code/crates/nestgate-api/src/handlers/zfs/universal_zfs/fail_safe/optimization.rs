//
// Optimization operations with circuit breaker and retry logic.

use crate::handlers::zfs::universal_zfs::types::{UniversalZfsError, UniversalZfsResult};

use super::core::FailSafeZfsService;

pub async fn optimize(service: &FailSafeZfsService) -> UniversalZfsResult<String> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.optimize().await;
        } else {
            return Err(UniversalZfsError::CircuitBreakerOpen {
                service: "zfs_optimization".to_string(),
            });
        }
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

pub async fn get_optimization_analytics(
    service: &FailSafeZfsService,
) -> UniversalZfsResult<serde_json::Value> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.get_optimization_analytics().await;
        } else {
            return Err(UniversalZfsError::CircuitBreakerOpen {
                service: "zfs_optimization".to_string(),
            });
        }
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

pub async fn predict_tier(
    service: &FailSafeZfsService,
    file_path: &str,
) -> UniversalZfsResult<String> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.predict_tier(file_path).await;
        } else {
            return Err(UniversalZfsError::CircuitBreakerOpen {
                service: "zfs_optimization".to_string(),
            });
        }
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

pub async fn get_configuration(
    service: &FailSafeZfsService,
) -> UniversalZfsResult<serde_json::Value> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.get_configuration().await;
        } else {
            return Err(UniversalZfsError::CircuitBreakerOpen {
                service: "zfs_optimization".to_string(),
            });
        }
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

pub async fn update_configuration(
    service: &FailSafeZfsService,
    config: serde_json::Value,
) -> UniversalZfsResult<()> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.update_configuration(config).await;
        } else {
            return Err(UniversalZfsError::CircuitBreakerOpen {
                service: "zfs_optimization".to_string(),
            });
        }
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

pub async fn shutdown(service: &FailSafeZfsService) -> UniversalZfsResult<()> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.shutdown().await;
        } else {
            return Err(UniversalZfsError::CircuitBreakerOpen {
                service: "zfs_optimization".to_string(),
            });
        }
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
