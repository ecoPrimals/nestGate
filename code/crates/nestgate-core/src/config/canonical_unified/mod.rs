//! Canonical Unified Configuration System
//! 
//! This module provides the unified configuration system that consolidates
//! all configuration variants across the NestGate ecosystem.

pub mod builders;
pub mod network_security;
pub mod services_monitoring;
pub mod storage_api;
pub mod system_config;

// Re-export commonly used types
pub use storage_api::{CacheStorageConfig, StorageConfig, StorageTiersConfig, TierConfig};
pub use system_config::SystemConfig;
pub use network_security::NetworkSecurityConfig;
pub use services_monitoring::ServicesMonitoringConfig;
pub use builders::ConfigBuilder;
