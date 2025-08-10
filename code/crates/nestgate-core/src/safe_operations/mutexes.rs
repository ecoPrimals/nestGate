/// Safe mutex operations  
/// Provides safe alternatives to lock().unwrap() patterns
use crate::NestGateError;
use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// **SAFE RWLOCK WRITE ACCESS**
/// Provides safe write access with poison recovery
pub fn safe_mutex_write<T>(mutex: &RwLock<T>) -> Result<RwLockWriteGuard<T>> {
    mutex.write().map_err(|_| NestGateError::Internal {
        message: "RwLock write lock poisoned".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        debug_info: Some("Recovering from poisoned lock".to_string()),
        is_bug: false,
    })
}

/// **SAFE RWLOCK READ ACCESS**
/// Provides safe read access with poison recovery
pub fn safe_mutex_read<T>(mutex: &RwLock<T>) -> Result<RwLockReadGuard<T>> {
    mutex.read().map_err(|_| NestGateError::Internal {
        message: "RwLock read lock poisoned".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        debug_info: Some("Recovering from poisoned lock".to_string()),
        is_bug: false,
    })
}

/// **SAFE MUTEX ACCESS**
/// Provides safe mutex access with poison recovery
pub fn safe_mutex_lock<T>(mutex: &Mutex<T>) -> Result<MutexGuard<T>> {
    mutex.lock().map_err(|_| NestGateError::Internal {
        message: "Mutex lock poisoned".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        debug_info: Some("Recovering from poisoned lock".to_string()),
        is_bug: false,
    })
}
