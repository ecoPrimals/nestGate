//! Zero-cost provider traits
//!
//! This module defines the core traits that enable zero-cost abstractions
//! by eliminating runtime dispatch overhead through compile-time specialization.

/// Zero-cost cache provider trait (replaces `async_trait` patterns)
pub trait ZeroCostCacheProvider<K, V> {
    /// Get value by key - native async, no boxing
    fn get(&self, key: &K) -> Option<V>;
    /// Set key-value pair - direct method dispatch
    fn set(&self, key: K, value: V) -> Result<(), super::types::ZeroCostError>;
    /// Remove key - zero overhead
    fn remove(&self, key: &K) -> bool;
}

/// Zero-cost security provider trait
pub trait ZeroCostSecurityProvider<Token, Credentials> {
    /// Authenticate - compile-time specialization
    fn authenticate(&self, credentials: &Credentials)
        -> Result<Token, super::types::ZeroCostError>;
    /// Validate token - direct dispatch
    fn validate(&self, token: &Token) -> bool;
    /// Refresh token - zero allocation
    fn refresh(&self, token: &Token) -> Result<Token, super::types::ZeroCostError>;
}

/// Zero-cost storage provider trait
pub trait ZeroCostStorageProvider<Key, Value> {
    /// Store value - no runtime overhead
    fn store(&self, key: Key, value: Value) -> Result<(), super::types::ZeroCostError>;
    /// Retrieve value - direct access
    fn retrieve(&self, key: &Key) -> Option<Value>;
    /// Delete value - zero cost
    fn delete(&self, key: &Key) -> bool;
}
