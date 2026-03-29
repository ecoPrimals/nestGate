// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Orchestrator Integration
//!
//! This module provides integration with orchestration systems for distributed
//! ZFS storage management and coordination across the ecoPrimals ecosystem.
//!
//! # Overview
//!
//! The orchestrator integration enables:
//! - **Service Registration**: Register ZFS services with orchestrators (capability-based, K8s)
//! - **Health Reporting**: Real-time health status and metrics reporting
//! - **Load Balancing**: Coordinate distributed ZFS operations
//! - **Service Discovery**: Dynamic discovery of ZFS nodes in the cluster
//! - **Distributed Management**: Coordinate storage across multiple nodes
//!
//! # Integration Points
//!
//! - **Orchestration Service**: Native ecoPrimals orchestrator integration (capability-based)
//! - **Kubernetes**: Service mesh and discovery integration
//! - **Custom Orchestrators**: Generic registration interface
//!
//! # Module Organization
//!
//! This module is organized into logical sub-modules:
//! - `types` - Type definitions for service registration and health status
//! - `service` - `ZfsService` implementation for orchestrator coordination
//!
//! # Examples
//!
//! ```rust,ignore
//! use nestgate_zfs::orchestrator_integration::{ZfsService, ZfsServiceConfig};
//!
//! // Create and configure service
//! let config = ZfsServiceConfig::default();
//! let mut service = ZfsService::new(config);
//!
//! // Register with orchestrator
//! // Port is now configurable via environment variable NESTGATE_ORCHESTRATION_PORT (default: 9091)
//! // use nestgate_core::config::port_config::get_port_config;
//! // service.register_with_orchestrator(
//! //     &format!("http://orchestrator:{}", get_port_config().orchestration_port)
//! // ).await?;
//!
//! // Perform health check
//! let health = service.health_check()?;
//! println!("Service health: {:?}", health);
//! ```
//!
//! # Architecture
//!
//! This module implements the **Infant Discovery Architecture**, allowing
//! ZFS services to dynamically discover and coordinate with each other in
//! a distributed environment.
//!
//! # Safety
//!
//! All network operations use safe async HTTP clients. No unsafe code.
//! Zero-copy patterns with Arc are used for efficient config sharing.

pub mod service;
pub mod types;

#[cfg(test)]
mod tests;

// Re-export commonly used types
pub use service::ZfsService;
pub use types::{ServiceInfo, ServiceRegistration, ZfsHealthStatus, ZfsServiceConfig};
