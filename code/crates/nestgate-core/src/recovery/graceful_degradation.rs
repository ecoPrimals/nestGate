//! **GRACEFUL DEGRADATION**
//!
//! Graceful degradation patterns for handling service failures.

use crate::error::NestGateError;
use std::collections::HashMap;
use tracing::{info, warn};

/// Degradation levels
#[derive(Debug, Clone, Copy, PartialEq)]
/// Degradationlevel
pub enum DegradationLevel {
    /// Full functionality
    Normal,
    /// Minor features disabled
    Minor,
    /// Significant features disabled
    Major,
    /// Critical functionality only
    Critical,
    /// Emergency mode
    Emergency,
}

/// Fallback strategy
#[derive(Debug, Clone)]
/// Fallbackstrategy
pub enum FallbackStrategy {
    /// Return cached data
    Cache,
    /// Use default values
    Default,
    /// Disable feature
    Disable,
    /// Use alternative service
    Alternative { endpoint: String },
}

/// Graceful degradation manager
#[derive(Debug)]
/// Gracefuldegradation
pub struct GracefulDegradation {
    /// Current degradation level
    level: DegradationLevel,
    /// Fallback strategies by capability
    strategies: HashMap<String, FallbackStrategy>,
}

impl GracefulDegradation {
    /// Create new graceful degradation manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            level: DegradationLevel::Normal,
            strategies: HashMap::new(),
        }
    }

    /// Set degradation level
    pub fn set_level(&mut self, level: DegradationLevel) {
        if self.level != level {
            info!("Degradation level changed: {:?} -> {:?}", self.level, level);
            self.level = level;
        }
    }

    /// Get current degradation level
    #[must_use]
    pub fn level(&self) -> DegradationLevel {
        self.level
    }

    /// Add fallback strategy for capability
    pub fn add_strategy(&mut self, capability: String, strategy: FallbackStrategy) {
        self.strategies.insert(capability, strategy);
    }

    /// Handle capability failure
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn handle_failure(&mut self, capability: &str) -> Result<(), NestGateError> {
        warn!("Handling failure for capability: {}", capability);

        if let Some(strategy) = self.strategies.get(capability) {
            match strategy {
                FallbackStrategy::Cache => {
                    info!("Using cached data for {}", capability);
                }
                FallbackStrategy::Default => {
                    info!("Using default values for {}", capability);
                }
                FallbackStrategy::Disable => {
                    info!("Disabling capability: {}", capability);
                }
                FallbackStrategy::Alternative { endpoint } => {
                    info!(
                        "Using alternative endpoint for {}: {}",
                        capability, endpoint
                    );
                }
            }
        }

        Ok(())
    }
}

impl Default for GracefulDegradation {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}
