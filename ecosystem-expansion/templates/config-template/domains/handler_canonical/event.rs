//! **EVENT HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventHandlerConfig {
    pub processing: EventProcessingConfig,
    pub routing: EventRoutingConfig,
    pub subscription: EventSubscriptionConfig,
    pub publishing: EventPublishingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventProcessingConfig { pub enabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRoutingConfig { pub enabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscriptionConfig { pub enabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPublishingConfig { pub enabled: bool }

impl Default for EventHandlerConfig {
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
    pub fn production_optimized() -> Self { Self::default() }
    pub fn development_optimized() -> Self { Self::default() }
    pub fn high_performance() -> Self { Self::default() }
    pub fn merge(self, _other: Self) -> Self { self }
    pub fn validate(&self) -> crate::Result<()> { Ok(()) }
} 