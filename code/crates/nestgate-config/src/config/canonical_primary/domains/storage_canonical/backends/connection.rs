// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Storage connection, pooling, retry, and TLS configuration.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// Connection configuration for a storage backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConnectionConfig {
    /// Connection timeout
    pub timeout: Duration,
    /// Maximum concurrent connections
    pub max_connections: u32,
    /// Connection retry settings
    pub retry: ConnectionRetryConfig,
    /// Connection pooling settings
    pub pooling: ConnectionPoolConfig,
    /// TLS/SSL settings
    pub tls: Option<ConnectionTlsConfig>,
}

impl Default for StorageConnectionConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_connections: 100,
            retry: ConnectionRetryConfig::default(),
            pooling: ConnectionPoolConfig::default(),
            tls: None,
        }
    }
}

/// Retry configuration for storage connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionRetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Base delay between retries
    pub base_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Retry strategy
    pub strategy: RetryStrategy,
}

impl Default for ConnectionRetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            strategy: RetryStrategy::Exponential,
        }
    }
}

/// Strategy for spacing retries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetryStrategy {
    /// Fixed delay between retries
    Fixed,
    /// Linearly increasing delay
    Linear,
    /// Exponentially increasing delay
    Exponential,
    /// Exponential with randomized jitter
    Jitter,
}

/// Connection pool sizing and lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolConfig {
    /// Minimum pool size
    pub min_size: u32,
    /// Maximum pool size
    pub max_size: u32,
    /// Connection idle timeout
    pub idle_timeout: Duration,
    /// Connection maximum lifetime
    pub max_lifetime: Duration,
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            min_size: 1,
            max_size: 10,
            idle_timeout: Duration::from_secs(600),
            max_lifetime: Duration::from_secs(3600),
        }
    }
}

/// TLS/SSL configuration for storage connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionTlsConfig {
    /// Enable TLS
    pub enabled: bool,
    /// Verify certificates
    pub verify_certificates: bool,
    /// CA certificate path
    pub ca_cert_path: Option<PathBuf>,
    /// Client certificate path
    pub client_cert_path: Option<PathBuf>,
    /// Client key path
    pub client_key_path: Option<PathBuf>,
}
