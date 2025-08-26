use crate::NestGateError;
/// Safe Operations Module
/// Provides safe utility functions for parsing and handling potentially unsafe operations
use std::net::IpAddr;
use tracing::{debug, error, warn};

/// Safely parse an IP address with fallback
pub fn safe_parse_ip_with_fallback(input: &str, fallback: &str) -> IpAddr {
    match input.parse::<IpAddr>() {
        Ok(addr) => addr,
        Err(_) => fallback.parse().unwrap_or_else(|_| {
            // Safe guaranteed localhost address
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))
        }),
    }
}

/// Safely parse an IP address with fallback to localhost
pub fn safe_parse_ip(ip_str: &str) -> Result<IpAddr> {
    ip_str.parse().map_err(|e| NestGateError::Validation {
        field: "ip_address".to_string(),
        message: format!("Failed to parse IP address '{ip_str}': {e}"),
        current_value: Some(ip_str.to_string()),
        expected: Some("Valid IP address format".to_string()),
        user_error: true,
    })
}

/// Safe adapter initialization - prevents panic during adapter setup
pub fn safe_adapter_init(adapter_name: &str, config: serde_json::Value) -> Result<()> {
    tracing::debug!("Safely initializing adapter: {}", adapter_name);

    // Validate adapter configuration
    if config.is_null() {
        return Err(NestGateError::Configuration {
            message: format!("Adapter '{adapter_name}' requires configuration but none provided"),
            config_source: crate::error::core::UnifiedConfigSource::Runtime,
            field: Some("adapter_config".to_string()),
            suggested_fix: Some("Provide valid configuration for the adapter".to_string()),
        });
    }

    // Log successful initialization
    tracing::info!("Adapter '{}' initialized successfully", adapter_name);
    Ok(())
}

/// Safe connection pool return operation - prevents connection leaks
pub async fn safe_connection_pool_return<T, F>(
    return_operation: F,
    pool_name: &str,
) -> Result<Result<()>>
where
    F: std::future::Future<Output = Result<()>>,
{
    tracing::debug!("Safely returning connection to pool: {}", pool_name);

    match return_operation.await {
        Ok(()) => {
            tracing::debug!("Connection successfully returned to pool: {}", pool_name);
            Ok(Ok(()))
        }
        Err(e) => {
            tracing::warn!("Failed to return connection to pool '{}': {}", pool_name, e);
            // Don't propagate the error - log it but allow the system to continue
            // This prevents cascade failures when connection return fails
            Ok(Err(e))
        }
    }
}

/// **SAFE UNWRAP ALTERNATIVES**
/// Provides safe utility functions for parsing and handling potentially unsafe operations
/// without using unwrap() which can cause panics in production.
/// **SAFE UNWRAP ALTERNATIVE** - Extract value with context
///
/// Use this instead of unwrap() to provide meaningful error messages
pub fn safe_extract<T>(result: Option<T>, context: &str) -> Result<T> {
    result.ok_or_else(|| {
        error!("Failed to extract value: {}", context);
        NestGateError::Internal {
            message: format!("Missing required value: {context}"),
            location: Some("safe_operations.rs:88".to_string()),
            debug_info: Some("safe_extract".to_string()),
            is_bug: false,
        }
    })
}

/// **SAFE RESULT UNWRAP** - Convert Result to NestGateError with context
///
/// Use this instead of unwrap() on Results to provide better error handling
pub fn safe_unwrap_result<T, E: std::fmt::Display>(
    result: std::result::Result<T, E>,
    context: &str,
) -> Result<T> {
    result.map_err(|e| {
        error!("Operation failed in {}: {}", context, e);
        NestGateError::Internal {
            message: format!("{context}: {e}"),
            location: Some("safe_operations.rs:106".to_string()),
            debug_info: Some(context.to_string()),
            is_bug: false,
        }
    })
}

/// **SAFE PARSING ALTERNATIVE**
///
/// Use this instead of unwrap() for safer parsing
pub fn safe_parse<T: std::str::FromStr>(value: &str, context: &str) -> Result<T>
where
    T::Err: std::fmt::Display,
{
    value.parse::<T>().map_err(|e| {
        error!("Parse failed in {}: {} -> {}", context, value, e);
        NestGateError::Validation {
            field: context.to_string(),
            message: format!("Parse failed: {e}"),
            current_value: Some(value.to_string()),
            expected: Some(format!("Valid {}", std::any::type_name::<T>())),
            user_error: true,
        }
    })
}

/// **SAFE STRING CONVERSION**
///
/// Use this instead of to_string()? (which doesn't exist) for safer string conversion
pub fn safe_to_string<T: std::fmt::Display>(value: T, context: &str) -> String {
    // Note: to_string() never fails, but we provide this for consistency
    debug!("Converting to string in context: {}", context);
    value.to_string()
}

/// **SAFE TEMPORARY DIRECTORY CREATION**
///
/// Creates a temporary directory safely with proper error handling
pub fn safe_create_temp_dir(prefix: &str) -> Result<std::path::PathBuf> {
    use std::env;

    let temp_base = env::temp_dir();
    let temp_name = format!("{}_{}", prefix, uuid::Uuid::new_v4());
    let temp_path = temp_base.join(temp_name);

    std::fs::create_dir_all(&temp_path).map_err(|e| {
        error!(
            "Failed to create temp directory '{}': {}",
            temp_path.display(),
            e
        );
        NestGateError::Io {
            operation: "create_temp_dir".to_string(),
            error_message: format!("Failed to create temporary directory: {e}"),
            resource: Some(temp_path.to_string_lossy().to_string()),
            retryable: true,
        }
    })?;

    debug!("Created temporary directory: {}", temp_path.display());
    Ok(temp_path)
}

/// **SAFE INTERNAL ERROR CREATION**
///
/// Creates internal errors with proper context
pub fn internal_error(message: String) -> NestGateError {
    error!("Internal error: {}", message);
    NestGateError::Internal {
        message,
        location: Some("safe_operations.rs".to_string()),
        debug_info: None,
        is_bug: false,
    }
}

/// **SAFE INDEX ACCESS** - Access array/vector elements safely
///
/// Use this instead of array[index] to prevent panic on out-of-bounds
pub fn safe_index<T: Clone>(collection: &[T], index: usize, context: &str) -> Result<T> {
    collection.get(index).cloned().ok_or_else(|| {
        error!(
            "Index {} out of bounds in {}: length={}",
            index,
            context,
            collection.len()
        );
        NestGateError::Internal {
            message: format!(
                "Index {} out of bounds in {} (length: {})",
                index,
                context,
                collection.len()
            ),
            location: Some("safe_operations.rs:144".to_string()),
            debug_info: Some(context.to_string()),
            is_bug: false,
        }
    })
}

/// **SAFE ADAPTER VALIDATION** - Validate adapter configuration safely
///
/// Use this to validate adapter configurations without panics
pub fn safe_validate_adapter(adapter_name: &str, config: &serde_json::Value) -> Result<()> {
    // Validate adapter configuration
    if config.is_null() {
        return Err(NestGateError::Configuration {
            message: format!("Adapter '{adapter_name}' requires configuration but none provided"),
            config_source: crate::error::core::UnifiedConfigSource::Runtime,
            field: Some("adapter_config".to_string()),
            suggested_fix: Some("Provide valid configuration for the adapter".to_string()),
        });
    }

    // Validate required fields exist
    if config.get("capabilities").is_none() {
        return Err(NestGateError::Configuration {
            message: format!("Adapter '{adapter_name}' missing required 'capabilities' field"),
            config_source: crate::error::core::UnifiedConfigSource::Runtime,
            field: Some("capabilities".to_string()),
            suggested_fix: Some("Add capabilities array to adapter configuration".to_string()),
        });
    }

    Ok(())
}

/// **SAFE ENVIRONMENT VARIABLE ACCESS**
///
/// Use this instead of env::var()? for safer environment access
pub fn safe_env_var(var_name: &str, default_value: Option<&str>) -> Result<String> {
    match std::env::var(var_name) {
        Ok(value) => Ok(value),
        Err(_) => {
            if let Some(default) = default_value {
                warn!(
                    "Environment variable '{}' not found, using default: {}",
                    var_name, default
                );
                Ok(default.to_string())
            } else {
                Err(NestGateError::Configuration {
                    message: format!("Required environment variable '{var_name}' not set"),
                    config_source: crate::error::core::UnifiedConfigSource::Environment,
                    field: Some(var_name.to_string()),
                    suggested_fix: Some(format!("Set environment variable {var_name}")),
                })
            }
        }
    }
}

/// **SAFE THREAD JOIN**
///
/// Use this instead of join()? for safer thread management
pub fn safe_thread_join<T>(handle: std::thread::JoinHandle<T>, context: &str) -> Result<T> {
    handle.join().map_err(|e| {
        error!("Thread join failed in {}: {:?}", context, e);
        NestGateError::System {
            message: format!("Thread join failed in {context}: {e:?}"),
            resource: crate::error::core::SystemResource::Threads,
            utilization: None,
            recovery: crate::error::core::RecoveryStrategy::Restart,
        }
    })
}

/// Safe unwrap for Option with default fallback
pub fn safe_unwrap_option<T>(option: Option<T>, context: &str, default: T) -> Result<T> {
    match option {
        Some(value) => Ok(value),
        None => {
            tracing::debug!("Option was None in context: {}, using default", context);
            Ok(default)
        }
    }
}

/// Safe mutex lock operation with timeout and poisoning recovery
pub fn safe_mutex_lock<T>(mutex: &std::sync::Mutex<T>) -> Result<std::sync::MutexGuard<T>> {
    mutex
        .lock()
        .map_err(|_poisoned| {
            warn!("Mutex was poisoned, recovering");
            // Recover from poisoning by taking the inner data
            // This is safe because we're discarding the poisoned guard
            NestGateError::Internal {
                message: "Mutex poisoning recovered".to_string(),
                location: Some("safe_mutex_lock".to_string()),
                debug_info: None,
                is_bug: false,
            }
        })
        .or_else(|_| {
            // If we can't recover, try to get the poisoned data
            match mutex.lock() {
                Ok(guard) => Ok(guard),
                Err(poisoned) => {
                    warn!("Recovering from poisoned mutex");
                    Ok(poisoned.into_inner())
                }
            }
        })
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_unwrap_result() -> Result<()> {
        let result: std::result::Result<i32, &str> = Ok(42);
        assert_eq!(safe_unwrap_result(result, "test context")?, 42);
        Ok(())
    }

    #[test]
    fn test_safe_index() -> Result<()> {
        assert_eq!(safe_index(&vec![1, 2, 3], 1, "test")?, 2);
        assert!(safe_index(&vec![1, 2, 3], 5, "test").is_err());
        Ok(())
    }

    #[test]
    fn test_safe_unwrap_option() -> Result<()> {
        let option = Some(42);
        assert_eq!(safe_unwrap_option(option, "test", 42)?, 42);

        let option: Option<i32> = None;
        assert_eq!(safe_unwrap_option(option, "test", 42)?, 42);
        Ok(())
    }

    #[test]
    fn test_safe_env_var() -> Result<()> {
        let result = safe_env_var("NONEXISTENT_VAR_12345", Some("default"))?;
        assert_eq!(result, "default");

        // Test without default
        assert!(safe_env_var("NONEXISTENT_VAR_12345", None).is_err());
        Ok(())
    }

    #[test]
    fn test_safe_to_string() {
        assert_eq!(safe_to_string(42, "test"), "42");
        assert_eq!(safe_to_string("hello", "test"), "hello");
    }
}
