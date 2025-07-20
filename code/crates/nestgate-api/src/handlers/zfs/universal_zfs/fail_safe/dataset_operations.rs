//! Dataset Operations with Fail-Safe
//!
//! Dataset operations with circuit breaker and retry logic.

use std::collections::HashMap;

use crate::handlers::zfs::universal_zfs::types::{
    DatasetConfig, DatasetInfo, UniversalZfsError, UniversalZfsResult,
};

use super::core::FailSafeZfsService;

pub async fn list_datasets(service: &FailSafeZfsService) -> UniversalZfsResult<Vec<DatasetInfo>> {
    // Check circuit breaker
    if !service.circuit_breaker.can_execute().await {
        return if let Some(fallback) = &service.fallback {
            fallback.list_datasets().await
        } else {
            // Try fallback operation
            match service.execute_fallback_operation("list_datasets", &service.primary).await {
                Ok(_) => Ok(Vec::new()), // Return empty list as fallback
                Err(_) => Err(crate::handlers::zfs::universal_zfs::types::UniversalZfsError::CircuitBreakerOpen {
                    service: service.service_name.clone(),
                })
            }
        };
    }

    // Execute with retry logic
    let primary = service.primary.clone();
    let timeout_duration = service.timeout_config.operation_timeout;
    let result = service
        .retry_executor
        .execute(|| {
            let primary = primary.clone();
            Box::pin(async move {
                // Apply timeout
                tokio::time::timeout(timeout_duration, primary.list_datasets())
                    .await
                    .map_err(|_| {
                        crate::handlers::zfs::universal_zfs::types::UniversalZfsError::timeout(
                            "list_datasets",
                            timeout_duration,
                        )
                    })?
            })
        })
        .await;

    // Update circuit breaker state
    match &result {
        Ok(_) => service.circuit_breaker.record_success().await,
        Err(_) => service.circuit_breaker.record_failure().await,
    }

    // Update metrics
    service
        .update_metrics("list_datasets", result.is_ok())
        .await;

    result
}

pub async fn get_dataset(
    service: &FailSafeZfsService,
    name: &str,
) -> UniversalZfsResult<Option<DatasetInfo>> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.get_dataset(name).await;
        } else {
            return Err(UniversalZfsError::internal(
                "Circuit breaker open and no fallback available",
            ));
        }
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.get_dataset(name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.get_dataset(name).await
            } else {
                Err(e)
            }
        }
    }
}

pub async fn create_dataset(
    service: &FailSafeZfsService,
    config: &DatasetConfig,
) -> UniversalZfsResult<DatasetInfo> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.create_dataset(config).await;
        } else {
            return Err(UniversalZfsError::internal(
                "Circuit breaker open and no fallback available",
            ));
        }
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.create_dataset(config).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.create_dataset(config).await
            } else {
                Err(e)
            }
        }
    }
}

pub async fn destroy_dataset(service: &FailSafeZfsService, name: &str) -> UniversalZfsResult<()> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.destroy_dataset(name).await;
        } else {
            return Err(UniversalZfsError::internal(
                "Circuit breaker open and no fallback available",
            ));
        }
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.destroy_dataset(name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.destroy_dataset(name).await
            } else {
                Err(e)
            }
        }
    }
}

pub async fn get_dataset_properties(
    service: &FailSafeZfsService,
    name: &str,
) -> UniversalZfsResult<HashMap<String, String>> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.get_dataset_properties(name).await;
        } else {
            return Err(UniversalZfsError::internal(
                "Circuit breaker open and no fallback available",
            ));
        }
    }

    // Execute primary service with circuit breaker tracking
    match service.primary.get_dataset_properties(name).await {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.get_dataset_properties(name).await
            } else {
                Err(e)
            }
        }
    }
}

pub async fn set_dataset_properties(
    service: &FailSafeZfsService,
    name: &str,
    properties: &HashMap<String, String>,
) -> UniversalZfsResult<()> {
    // Check if circuit breaker allows execution
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.set_dataset_properties(name, properties).await;
        } else {
            return Err(UniversalZfsError::internal(
                "Circuit breaker open and no fallback available",
            ));
        }
    }

    // Execute primary service with circuit breaker tracking
    match service
        .primary
        .set_dataset_properties(name, properties)
        .await
    {
        Ok(result) => {
            service.circuit_breaker.record_success().await;
            Ok(result)
        }
        Err(e) => {
            service.circuit_breaker.record_failure().await;
            if let Some(fallback) = &service.fallback {
                fallback.set_dataset_properties(name, properties).await
            } else {
                Err(e)
            }
        }
    }
}
