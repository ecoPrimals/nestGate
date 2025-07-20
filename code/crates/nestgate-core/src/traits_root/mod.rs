//! Core traits for the Songbird Orchestrator
//!
//! This module defines the fundamental traits that enable universal service orchestration
//! across different project types and deployment environments.

pub mod communication;
pub mod config;
pub mod discovery;
pub mod health;
pub mod load_balancer;
pub mod service;

// Re-export all trait types
pub use communication::CommunicationLayer;
pub use config::ConfigProvider;
pub use discovery::{ServiceDiscovery, ServiceEvent, ServiceQuery};
pub use health::{HealthCheck, HealthMonitor, HealthState, HealthStatus};
pub use service::*;
