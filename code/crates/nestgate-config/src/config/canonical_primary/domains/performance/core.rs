// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Performance core configuration module
//! Provides unified performance configuration settings.

use serde::{Deserialize, Serialize};

use super::{
    caching::CachePerformanceConfig, concurrency::ConcurrencyConfig, cpu::CpuPerformanceConfig,
    environment::PerformanceEnvironmentConfig, io::IoPerformanceConfig,
    memory::MemoryPerformanceConfig, monitoring::PerformanceMonitoringConfig,
    network::NetworkPerformanceConfig, profiles::OptimizationProfiles,
};
use nestgate_types::error::{NestGateError, Result};

// ==================== CANONICAL PERFORMANCE CONFIGURATION ====================

/// **THE** canonical performance configuration for the entire `NestGate` ecosystem
/// This replaces ALL other `PerformanceConfig` variants
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for `CanonicalPerformance`
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
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the performance configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
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
            return Err(NestGateError::configuration_error_detailed(
                "concurrency.max_concurrent".to_string(),
                "Maximum concurrent operations cannot be zero".to_string(),
                Some("0".into()),
                Some(">0".into()),
                true,
            ));
        }

        Ok(())
    }

    /// Load configuration from environment variables
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn from_environment() -> Result<Self> {
        let config = Self::default();
        config.validate()?;
        Ok(config)
    }

    /// Merge with another configuration, with other taking precedence
    #[must_use]
    pub fn merge(self, other: Self) -> Self {
        // For now, other completely replaces self
        // In the future, we could implement field-level merging
        other
    }
}
