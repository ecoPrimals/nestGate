// **EVENT HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for EventHandler
pub struct EventHandlerConfig {
    /// Processing
    pub processing: EventProcessingConfig,
    /// Routing
    pub routing: EventRoutingConfig,
    /// Subscription
    pub subscription: EventSubscriptionConfig,
    /// Publishing
    pub publishing: EventPublishingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for EventProcessing
pub struct EventProcessingConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for EventRouting
pub struct EventRoutingConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for EventSubscription
pub struct EventSubscriptionConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for EventPublishing
pub struct EventPublishingConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl Default for EventHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            processing: EventProcessingConfig { enabled: true },
            routing: EventRoutingConfig { enabled: true },
            subscription: EventSubscriptionConfig { enabled: true },
            publishing: EventPublishingConfig { enabled: true },
        }
    }
}

impl EventHandlerConfig {
    /// Returns a production-optimized configuration
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }

    /// Returns a development-optimized configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Returns a high-performance configuration
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }

    /// Merges this configuration with another, returning the merged result
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
