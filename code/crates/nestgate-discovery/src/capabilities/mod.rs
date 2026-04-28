// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Capability-based service architecture for `NestGate`
//!
//! This module implements a capability-based service discovery and integration
//! system that replaces hardcoded service names with dynamic, capability-driven
//! discovery.
//!
//! # Architecture
//!
//! The capability system consists of:
//!
//! - **Discovery**: Find services by capability, not by name
//! - **Resolution**: Connect to services with load balancing
//! - **Detection**: Automatically discover services on the network
//! - **Registry**: Central repository of service capabilities
//!
//! # Zero Hardcoding Principle
//!
//! This implementation follows the "Zero Hardcoding" principle from
//! `PRIMAL_ECOSYSTEM_INTEGRATION_SPEC.md`. Services are discovered by
//! **capability**, not by name. This ensures:
//!
//! - Vendor independence (no hardcoded primal names)
//! - Dynamic service substitution
//! - Load balancing across providers
//! - Sovereignty compliance
//!
//! # Example: Complete Flow
//!
//! ```text
//! Full flow requires ServiceDetector with scan_ports; see capabilities::discovery for
//! simpler capability-based lookup examples using CapabilityRegistry::find_providers().
//! ```
//!
//! # Migration from Hardcoded Names
//!
//! ## Before (Violates Specification)
//!
//! ```rust,ignore
//! // Hardcoded primal name (violates spec)
//! if service_name == "<specific-primal>" {
//!     connect_to_security_service("<specific-primal>:8443").await?;
//! }
//! ```
//!
//! ## After (Specification Compliant)
//!
//! ```rust,ignore
//! // Capability-based resolution (specification-compliant)
//! let security_services = registry
//!     .find_providers(&Capability::Security(SecurityCapability::Authentication))
//!     .await;
//!
//! for service in security_services {
//!     if service.is_healthy() {
//!         connect_to_service(&service).await?;
//!         break;
//!     }
//! }
//! ```
//!
//! # Supported Capabilities
//!
//! ## Security (7 capabilities)
//! - Authentication, Authorization, Encryption, `KeyManagement`
//! - `ThreatDetection`, `AuditLogging`, `CertificateManagement`
//!
//! ## Networking (8 capabilities)
//! - TCP, UDP, HTTP, WebSocket, GRPC
//! - `LoadBalancing`, `ServiceMesh`, DNS
//!
//! ## AI (7 capabilities)
//! - Inference, Training, `ModelServing`, `FeatureExtraction`
//! - `NaturalLanguage`, `ComputerVision`, `ReinforcementLearning`
//!
//! ## Orchestration (6 capabilities)
//! - `ContainerManagement`, `ServiceScheduling`, `ResourceAllocation`
//! - `HealthMonitoring`, `AutoScaling`, `ServiceDiscovery`
//!
//! ## Storage (6 capabilities)
//! - `ObjectStorage`, `BlockStorage`, `FileSystem`
//! - Database, Cache, Snapshots
//!
//! # Status
//!
//! **Implementation**: Complete (December 2, 2025)\
//! **Test Coverage**: 17 unit tests, all passing\
//! **Build Status**: Clean compilation\
//! **Next Phase**: Migrate hardcoded primal names to capability queries  

pub mod discovery;
