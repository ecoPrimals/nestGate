/// Safe mutex operations  
/// Provides safe alternatives to lock().unwrap() patterns
use crate::error::NestGateError;
use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// **SAFE RWLOCK WRITE ACCESS**
/// Provides safe write access with poison recovery
pub fn safe_mutex_write<T>(mutex: &RwLock<T>) -> Result<RwLockWriteGuard<T>> {
    mutex.write().map_err(|_| NestGateError::Internal {
        message: "RwLock write lock poisoned".to_string(),
        component: "safe_operations_mutexes".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        is_bug: false,
        context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
            operation: "safe_mutex_write".to_string(),
            component: "safe_mutexes".to_string(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                map.insert("details".to_string(), "Recovering from poisoned lock".to_string());
                map
            },
            timestamp: std::time::SystemTime::now(),
            retry_info: None,
            recovery_suggestions: vec!["Mutex was poisoned by panic, attempting recovery".to_string()],
                    performance_metrics: None,
                    environment: None,
        }),
    })
}

/// **SAFE RWLOCK READ ACCESS**
/// Provides safe read access with poison recovery
pub fn safe_mutex_read<T>(mutex: &RwLock<T>) -> Result<RwLockReadGuard<T>> {
    mutex.read().map_err(|_| NestGateError::Internal {
        message: "RwLock read lock poisoned".to_string(),
        component: "safe_operations_mutexes".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        is_bug: false,
        context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
            operation: "safe_mutex_read".to_string(),
            component: "safe_mutexes".to_string(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                map.insert("details".to_string(), "Recovering from poisoned lock".to_string());
                map
            },
            timestamp: std::time::SystemTime::now(),
            retry_info: None,
            recovery_suggestions: vec!["Mutex was poisoned by panic, attempting recovery".to_string()],
                    performance_metrics: None,
                    environment: None,
        }),
    })
}

/// **SAFE MUTEX ACCESS**
/// Provides safe mutex access with poison recovery
pub fn safe_mutex_lock<T>(mutex: &Mutex<T>) -> Result<MutexGuard<T>> {
    mutex.lock().map_err(|_| NestGateError::Internal {
        message: "Mutex lock poisoned".to_string(),
        component: "safe_operations_mutexes".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        is_bug: false,
        context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
            operation: "safe_mutex_lock".to_string(),
            component: "safe_mutexes".to_string(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                map.insert("details".to_string(), "Recovering from poisoned lock".to_string());
                map
            },
            timestamp: std::time::SystemTime::now(),
            retry_info: None,
            recovery_suggestions: vec!["Mutex was poisoned by panic, attempting recovery".to_string()],
                    performance_metrics: None,
                    environment: None,
        }),
    })
}
