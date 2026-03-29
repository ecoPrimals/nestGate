// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;
/// Safe mutex operations
/// Provides safe alternatives to `lock().unwrap()` patterns
use crate::error::NestGateError;
use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
/// **SAFE RWLOCK WRITE ACCESS**
/// Provides safe write access with poison recovery
pub fn safe_mutex_write<T>(mutex: &RwLock<T>) -> Result<RwLockWriteGuard<'_, T>> {
    mutex.write().map_err(|_| {
        NestGateError::internal_error("RwLock write lock poisoned", "safe_operations_mutexes")
    })
}
/// **SAFE RWLOCK READ ACCESS**
/// Provides safe read access with poison recovery
pub fn safe_mutex_read<T>(mutex: &RwLock<T>) -> Result<RwLockReadGuard<'_, T>> {
    mutex.read().map_err(|_| {
        NestGateError::internal_error("RwLock read lock poisoned", "safe_operations_mutexes")
    })
}
/// **SAFE MUTEX ACCESS**
/// Provides safe mutex access with poison recovery
pub fn safe_mutex_lock<T>(mutex: &Mutex<T>) -> Result<MutexGuard<'_, T>> {
    mutex.lock().map_err(|_| {
        NestGateError::internal_error("Mutex lock poisoned", "safe_operations_mutexes")
    })
}
