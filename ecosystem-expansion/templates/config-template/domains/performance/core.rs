//! **CORE PERFORMANCE CONFIGURATION**
//!
//! The main performance configuration structure that unifies all performance settings.

use serde::{Deserialize, Serialize};

use crate::{NestGateError, Result};
use super::{
    cpu::CpuPerformanceConfig,
    memory::MemoryPerformanceConfig,
    io::IoPerformanceConfig,
    network::NetworkPerformanceConfig,
    caching::CachePerformanceConfig,
    concurrency::ConcurrencyConfig,
    monitoring::PerformanceMonitoringConfig,
    profiles::OptimizationProfiles,
    environment::PerformanceEnvironmentConfig,
};

// ==================== CANONICAL PERFORMANCE CONFIGURATION ====================

/// **THE** canonical performance configuration for the entire NestGate ecosystem
/// This replaces ALL other PerformanceConfig variants
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct CanonicalPerformanceConfig {
    /// CPU optimization settings
    pub cpu: CpuPerformanceConfig,
    
    /// Memory optimization settings
    pub memory: MemoryPerformanceConfig,
    
    /// I/O optimization settings
    pub io: IoPerformanceConfig,
    
    /// Network performance settings
    pub network: NetworkPerformanceConfig,
    
    /// Caching performance settings
    pub caching: CachePerformanceConfig,
    
    /// Threading and concurrency settings
    pub concurrency: ConcurrencyConfig,
    
    /// Monitoring and metrics
    pub monitoring: PerformanceMonitoringConfig,
    
    /// Optimization profiles
    pub profiles: OptimizationProfiles,
    
    /// Environment-specific settings
    pub environment: PerformanceEnvironmentConfig,
}


impl CanonicalPerformanceConfig {
    /// Create a new performance configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the performance configuration
    pub fn validate(&self) -> Result<()> {
        // Validate CPU configuration
        self.cpu.validate()?;
        
        // Validate memory configuration
        self.memory.validate()?;
        
        // Validate I/O configuration
        self.io.validate()?;
        
        // Validate network configuration
        self.network.validate()?;
        
        // Validate caching configuration
        self.caching.validate()?;
        
        // Validate concurrency configuration
        if self.concurrency.max_concurrent == 0 {
            return Err(NestGateError::Configuration {
                field: "concurrency.max_concurrent".to_string(),
                message: "Maximum concurrent operations cannot be zero".to_string(),
                current_value: Some("0".to_string()),
                expected: Some(">0".to_string()),
                user_error: true,
            });
        }

        Ok(())
    }

    /// Load configuration from environment variables
    pub fn from_environment() -> Result<Self> {
        let config = Self::default();
        config.validate()?;
        Ok(config)
    }

    /// Merge with another configuration, with other taking precedence
    pub fn merge(self, other: Self) -> Self {
        // For now, other completely replaces self
        // In the future, we could implement field-level merging
        other
    }
} 